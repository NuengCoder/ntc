//! Argument completion for curated programming tools, compilers, runtimes
//! (`g++`, `python`, `java`, `cargo`, `git`, …) and common shell utilities
//! (`grep`, `rm`, `curl`, …), enabled by `config code_tools on`.
//!
//! Only names in the registry below are recognised, and only names actually
//! installed on `PATH` are offered at level 1. Subcommands and flags come
//! from static tables; the genuinely dynamic bits (git branches, npm scripts,
//! containers, crate examples) are shelled out once and cached per directory.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use super::{nonempty_lines, run_capture, tokens_to_lines};

const TOOL_TTL: Duration = Duration::from_secs(5);

pub(crate) struct ToolSpec {
    name: &'static str,
    subcommands: &'static [&'static str],
    flags: &'static [&'static str],
}

const C_COMPILE_FLAGS: &[&str] = &[
    "-o",
    "-c",
    "-g",
    "-O0",
    "-O1",
    "-O2",
    "-O3",
    "-Os",
    "-Wall",
    "-Wextra",
    "-Werror",
    "-pedantic",
    "-std=c11",
    "-std=c17",
    "-std=c++14",
    "-std=c++17",
    "-std=c++20",
    "-std=c++23",
    "-I",
    "-L",
    "-l",
    "-D",
    "-U",
    "-fPIC",
    "-shared",
    "-static",
    "-pthread",
    "-M",
    "-MM",
    "-E",
    "-S",
    "-v",
    "--version",
    "--help",
    "-h",
];

const GIT_GLOBAL_FLAGS: &[&str] = &[
    "--version",
    "--help",
    "-h",
    "-C",
    "--exec-path",
    "--git-dir",
    "--work-tree",
    "--namespace",
    "--bare",
    "--no-pager",
    "--paginate",
    "-p",
    "--no-replace-objects",
    "--literal-pathspecs",
    "--glob-pathspecs",
    "--noglob-pathspecs",
    "--icase-pathspecs",
];

const CARGO_GLOBAL_FLAGS: &[&str] = &[
    "--version",
    "-V",
    "--help",
    "-h",
    "-v",
    "--verbose",
    "-q",
    "--quiet",
    "--color",
    "--config",
    "-Z",
    "--frozen",
    "--locked",
    "--offline",
];

const NPM_GLOBAL_FLAGS: &[&str] = &[
    "--version",
    "-v",
    "--help",
    "-h",
    "--global",
    "-g",
    "--prefix",
    "--registry",
    "--silent",
    "-s",
    "--verbose",
    "--no-fund",
    "--no-audit",
    "--production",
    "--omit",
    "--workspace",
    "-w",
];

const DOCKER_GLOBAL_FLAGS: &[&str] = &[
    "--version",
    "-v",
    "--help",
    "-h",
    "--config",
    "--context",
    "-c",
    "--debug",
    "-D",
    "--host",
    "-H",
    "--log-level",
    "-l",
    "--tls",
    "--tls-verify",
];

const PYTHON_FLAGS: &[&str] = &[
    "-m",
    "-c",
    "-i",
    "-O",
    "-OO",
    "-u",
    "-B",
    "-E",
    "-I",
    "-S",
    "-s",
    "-X",
    "-V",
    "--version",
    "-h",
    "--help",
];

const JAVA_FLAGS: &[&str] = &[
    "-jar",
    "-cp",
    "-classpath",
    "-version",
    "--version",
    "-help",
    "--help",
    "-X",
    "-Xmx",
    "-Xms",
    "-ea",
    "-da",
    "-D",
    "-module-path",
    "-p",
    "-m",
    "--module",
    "--add-modules",
    "--enable-preview",
    "--enable-native-access",
];

const NODE_FLAGS: &[&str] = &[
    "-e",
    "--eval",
    "-p",
    "--print",
    "-i",
    "--interactive",
    "--inspect",
    "--inspect-brk",
    "--experimental-modules",
    "--no-warnings",
    "--watch",
    "--watch-path",
    "--version",
    "-v",
    "--help",
    "-h",
];

const GO_FLAGS: &[&str] = &[
    "-o",
    "-v",
    "-x",
    "-n",
    "-race",
    "-msan",
    "-asan",
    "-cover",
    "-coverprofile",
    "-tags",
    "-ldflags",
    "-gcflags",
    "-mod",
    "-C",
    "-trimpath",
    "-buildmode",
    "-p",
    "-work",
    "-a",
    "-buildvcs",
    "--help",
];

const GO_SUBS: &[&str] = &[
    "build", "run", "test", "get", "install", "mod", "work", "env", "fmt", "vet", "doc",
    "generate", "clean", "list", "version", "tool", "debug",
];

const MAVEN_SUBS: &[&str] = &[
    "clean",
    "validate",
    "compile",
    "test",
    "package",
    "verify",
    "install",
    "deploy",
    "site",
    "dependency",
    "dependency:tree",
    "versions:display-dependency-updates",
    "archetype:generate",
    "spring-boot:run",
];

/// Curated names. Anything not listed here is never hinted, even when it is
/// an executable on `PATH`.
pub(crate) const SPECS: &[ToolSpec] = &[
    // VCS / devops
    ToolSpec {
        name: "git",
        subcommands: &[],
        flags: GIT_GLOBAL_FLAGS,
    },
    ToolSpec {
        name: "svn",
        subcommands: &[
            "checkout", "co", "commit", "ci", "update", "up", "add", "delete", "rm", "status",
            "st", "log", "diff", "revert", "info", "export", "import", "move", "mv", "copy", "cp",
            "switch", "blame", "cleanup", "lock", "unlock", "merge", "propget", "propset",
        ],
        flags: &[
            "--help",
            "--version",
            "-q",
            "--quiet",
            "-v",
            "--verbose",
            "--username",
            "--password",
            "--non-interactive",
            "--trust-server-cert",
        ],
    },
    ToolSpec {
        name: "hg",
        subcommands: &[
            "clone", "commit", "status", "log", "diff", "pull", "push", "update", "add", "remove",
            "merge", "branch", "tag", "bookmark", "graft", "rebase", "histedit", "shelve",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "-R",
            "--repository",
            "-u",
            "--user",
            "--config",
        ],
    },
    ToolSpec {
        name: "gh",
        subcommands: &[
            "auth", "repo", "issue", "pr", "release", "workflow", "run", "gist", "alias", "config",
            "browse", "search", "status", "api",
        ],
        flags: &[
            "--help",
            "--version",
            "-R",
            "--repo",
            "--json",
            "--jq",
            "--limit",
            "-L",
        ],
    },
    ToolSpec {
        name: "glab",
        subcommands: &[
            "auth", "repo", "mr", "issue", "ci", "release", "pipeline", "variable", "config", "api",
        ],
        flags: &[
            "--help",
            "--version",
            "-R",
            "--repo",
            "--output",
            "-F",
            "--paginate",
        ],
    },
    ToolSpec {
        name: "docker",
        subcommands: &[],
        flags: DOCKER_GLOBAL_FLAGS,
    },
    ToolSpec {
        name: "podman",
        subcommands: &[
            "run",
            "ps",
            "images",
            "build",
            "pull",
            "push",
            "exec",
            "logs",
            "stop",
            "start",
            "restart",
            "rm",
            "rmi",
            "compose",
            "volume",
            "network",
            "inspect",
            "stats",
            "cp",
            "tag",
            "login",
            "logout",
            "search",
            "version",
            "info",
            "system",
            "image",
            "container",
        ],
        flags: DOCKER_GLOBAL_FLAGS,
    },
    ToolSpec {
        name: "kubectl",
        subcommands: &[
            "get",
            "describe",
            "create",
            "apply",
            "delete",
            "edit",
            "exec",
            "logs",
            "port-forward",
            "config",
            "rollout",
            "scale",
            "top",
            "explain",
            "api-resources",
            "version",
            "cluster-info",
            "cp",
            "auth",
            "diff",
            "patch",
            "replace",
            "wait",
            "annotate",
            "label",
        ],
        flags: &[
            "--help",
            "-n",
            "--namespace",
            "-A",
            "--all-namespaces",
            "-o",
            "--output",
            "--context",
            "--kubeconfig",
            "-f",
            "--filename",
            "-l",
            "--selector",
            "--dry-run",
            "--watch",
            "-w",
        ],
    },
    ToolSpec {
        name: "helm",
        subcommands: &[
            "install",
            "upgrade",
            "uninstall",
            "list",
            "ls",
            "repo",
            "search",
            "show",
            "template",
            "lint",
            "package",
            "dependency",
            "rollback",
            "status",
            "history",
            "get",
            "create",
            "version",
            "env",
            "plugin",
            "pull",
            "push",
            "registry",
            "test",
        ],
        flags: &[
            "--help",
            "-n",
            "--namespace",
            "--kube-context",
            "--kubeconfig",
            "--debug",
            "--dry-run",
            "-f",
            "--values",
            "--set",
            "--version",
        ],
    },
    ToolSpec {
        name: "terraform",
        subcommands: &[
            "init",
            "plan",
            "apply",
            "destroy",
            "validate",
            "fmt",
            "show",
            "output",
            "state",
            "import",
            "providers",
            "workspace",
            "taint",
            "untaint",
            "console",
            "graph",
            "version",
            "force-unlock",
        ],
        flags: &[
            "-help",
            "-version",
            "-chdir",
            "-auto-approve",
            "-var",
            "-var-file",
            "-target",
            "-out",
            "-input",
            "-lock",
            "-refresh",
            "-parallelism",
        ],
    },
    ToolSpec {
        name: "ansible",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-i",
            "--inventory",
            "-m",
            "--module-name",
            "-a",
            "--args",
            "-u",
            "--user",
            "-k",
            "--ask-pass",
            "-K",
            "--ask-become-pass",
            "-b",
            "--become",
            "-l",
            "--limit",
            "-e",
            "--extra-vars",
            "-v",
            "--verbose",
            "--syntax-check",
            "--check",
            "-C",
            "--diff",
        ],
    },
    ToolSpec {
        name: "ansible-playbook",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-i",
            "--inventory",
            "-l",
            "--limit",
            "-e",
            "--extra-vars",
            "-t",
            "--tags",
            "--skip-tags",
            "--start-at-task",
            "--list-tasks",
            "--list-hosts",
            "--list-tags",
            "--syntax-check",
            "--check",
            "-C",
            "--diff",
            "-v",
            "--verbose",
        ],
    },
    ToolSpec {
        name: "vagrant",
        subcommands: &[
            "init",
            "up",
            "halt",
            "destroy",
            "ssh",
            "status",
            "reload",
            "provision",
            "box",
            "snapshot",
            "plugin",
            "global-status",
            "port",
            "suspend",
            "resume",
            "validate",
        ],
        flags: &[
            "--help",
            "--version",
            "-h",
            "--provider",
            "--debug",
            "--force",
            "--provision",
        ],
    },
    // Rust
    ToolSpec {
        name: "cargo",
        subcommands: &[],
        flags: CARGO_GLOBAL_FLAGS,
    },
    ToolSpec {
        name: "rustc",
        subcommands: &[],
        flags: &[
            "-o",
            "-O",
            "-g",
            "--edition",
            "--crate-type",
            "--crate-name",
            "--emit",
            "-L",
            "-l",
            "--extern",
            "--out-dir",
            "--target",
            "-C",
            "-Z",
            "-W",
            "-A",
            "-D",
            "--cfg",
            "--test",
            "--bench",
            "--verbose",
            "-v",
            "-V",
            "--version",
            "--help",
            "-h",
        ],
    },
    ToolSpec {
        name: "rustup",
        subcommands: &[
            "install",
            "uninstall",
            "update",
            "default",
            "toolchain",
            "target",
            "component",
            "override",
            "run",
            "which",
            "doc",
            "self",
            "man",
            "completions",
            "show",
            "profile",
            "set",
        ],
        flags: &["--help", "--version", "-v", "--verbose", "-q", "--quiet"],
    },
    ToolSpec {
        name: "rustfmt",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "--check",
            "--emit",
            "--edition",
            "--config",
            "--config-path",
            "-v",
        ],
    },
    // JavaScript / TypeScript
    ToolSpec {
        name: "node",
        subcommands: &[],
        flags: NODE_FLAGS,
    },
    ToolSpec {
        name: "npm",
        subcommands: &[],
        flags: NPM_GLOBAL_FLAGS,
    },
    ToolSpec {
        name: "npx",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-y",
            "--yes",
            "-p",
            "--package",
            "-c",
            "--call",
            "--no-install",
            "--ignore-existing",
            "--offline",
            "--prefer-offline",
        ],
    },
    ToolSpec {
        name: "yarn",
        subcommands: &[
            "add", "remove", "install", "up", "upgrade", "list", "info", "run", "test", "build",
            "init", "publish", "cache", "config", "global",
        ],
        flags: &[
            "--help",
            "--version",
            "--production",
            "--frozen-lockfile",
            "--dev",
            "-D",
            "--global",
            "-g",
            "--silent",
            "--verbose",
        ],
    },
    ToolSpec {
        name: "pnpm",
        subcommands: &[
            "add", "remove", "install", "update", "run", "test", "build", "init", "publish",
            "store", "config",
        ],
        flags: &[
            "--help",
            "--version",
            "-D",
            "--save-dev",
            "-g",
            "--global",
            "-r",
            "--recursive",
            "--filter",
            "-F",
            "--frozen-lockfile",
        ],
    },
    ToolSpec {
        name: "bun",
        subcommands: &[
            "run", "install", "add", "remove", "update", "test", "build", "init", "create", "x",
            "publish", "pm",
        ],
        flags: &[
            "--help",
            "--version",
            "--watch",
            "--hot",
            "--bun",
            "--dev",
            "-d",
            "--production",
        ],
    },
    ToolSpec {
        name: "deno",
        subcommands: &[
            "run",
            "test",
            "bench",
            "fmt",
            "lint",
            "check",
            "cache",
            "info",
            "doc",
            "eval",
            "serve",
            "compile",
            "bundle",
            "install",
            "uninstall",
            "upgrade",
            "task",
        ],
        flags: &[
            "--help",
            "--version",
            "-A",
            "--allow-all",
            "--allow-read",
            "--allow-write",
            "--allow-net",
            "--allow-env",
            "--watch",
            "--no-check",
            "--unstable",
        ],
    },
    ToolSpec {
        name: "tsc",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-p",
            "--project",
            "--target",
            "--module",
            "--outDir",
            "--rootDir",
            "--strict",
            "--declaration",
            "-d",
            "--watch",
            "-w",
            "--noEmit",
            "--sourceMap",
            "--esModuleInterop",
            "--skipLibCheck",
            "--jsx",
            "--moduleResolution",
            "--lib",
        ],
    },
    // Python
    ToolSpec {
        name: "python",
        subcommands: &[],
        flags: PYTHON_FLAGS,
    },
    ToolSpec {
        name: "python3",
        subcommands: &[],
        flags: PYTHON_FLAGS,
    },
    ToolSpec {
        name: "py",
        subcommands: &[],
        flags: PYTHON_FLAGS,
    },
    ToolSpec {
        name: "pip",
        subcommands: &[
            "install",
            "uninstall",
            "download",
            "freeze",
            "list",
            "show",
            "check",
            "search",
            "wheel",
            "hash",
            "config",
            "cache",
            "index",
            "debug",
        ],
        flags: &[
            "--help",
            "--version",
            "-V",
            "-q",
            "--quiet",
            "-v",
            "--verbose",
            "--no-cache-dir",
            "--no-deps",
            "--user",
            "--upgrade",
            "-U",
            "--force-reinstall",
            "--pre",
            "-r",
            "--requirement",
            "-e",
            "--editable",
            "-t",
            "--target",
        ],
    },
    ToolSpec {
        name: "pip3",
        subcommands: &[
            "install",
            "uninstall",
            "download",
            "freeze",
            "list",
            "show",
            "check",
            "search",
            "wheel",
            "hash",
            "config",
            "cache",
            "index",
            "debug",
        ],
        flags: &[
            "--help",
            "--version",
            "-V",
            "-q",
            "--quiet",
            "-v",
            "--verbose",
            "--no-cache-dir",
            "--no-deps",
            "--user",
            "--upgrade",
            "-U",
            "--force-reinstall",
            "--pre",
            "-r",
            "--requirement",
            "-e",
            "--editable",
            "-t",
            "--target",
        ],
    },
    ToolSpec {
        name: "uv",
        subcommands: &[
            "add", "remove", "sync", "lock", "run", "pip", "venv", "init", "build", "publish",
            "tool", "python", "cache", "self",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "--no-cache",
            "--offline",
            "--frozen",
            "--locked",
            "--python",
            "-p",
        ],
    },
    ToolSpec {
        name: "poetry",
        subcommands: &[
            "add", "remove", "install", "update", "lock", "run", "build", "publish", "new", "init",
            "env", "shell", "show", "config", "export", "check", "search", "source", "version",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "--no-interaction",
            "-n",
            "--no-ansi",
            "--directory",
        ],
    },
    ToolSpec {
        name: "pipenv",
        subcommands: &[
            "install",
            "uninstall",
            "lock",
            "run",
            "shell",
            "graph",
            "check",
            "update",
            "sync",
            "clean",
            "scripts",
            "requirements",
        ],
        flags: &[
            "--help",
            "--version",
            "--dev",
            "-d",
            "--three",
            "--two",
            "--python",
            "--system",
            "--verbose",
            "--quiet",
        ],
    },
    // Go
    ToolSpec {
        name: "go",
        subcommands: GO_SUBS,
        flags: GO_FLAGS,
    },
    ToolSpec {
        name: "gofmt",
        subcommands: &[],
        flags: &[
            "-w",
            "-l",
            "-d",
            "-e",
            "-r",
            "-s",
            "-simplify",
            "--help",
            "--version",
        ],
    },
    // C / C++
    ToolSpec {
        name: "gcc",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "g++",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "clang",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "clang++",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "cc",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "c++",
        subcommands: &[],
        flags: C_COMPILE_FLAGS,
    },
    ToolSpec {
        name: "make",
        subcommands: &[],
        flags: &[
            "-f",
            "--file",
            "-j",
            "--jobs",
            "-k",
            "--keep-going",
            "-n",
            "--just-print",
            "-s",
            "--silent",
            "-B",
            "--always-make",
            "-C",
            "--directory",
            "-i",
            "--ignore-errors",
            "-e",
            "--environment-overrides",
            "-I",
            "--include-dir",
            "-r",
            "--no-builtin-rules",
            "-R",
            "--no-builtin-variables",
            "-t",
            "--touch",
            "-q",
            "--question",
            "-v",
            "--version",
            "-h",
            "--help",
        ],
    },
    ToolSpec {
        name: "cmake",
        subcommands: &[],
        flags: &[
            "-S",
            "-B",
            "-G",
            "-A",
            "-T",
            "-D",
            "-U",
            "-C",
            "--build",
            "--install",
            "--open",
            "--fresh",
            "--preset",
            "--list-presets",
            "-E",
            "-P",
            "--help",
            "--version",
            "-LA",
            "-LH",
        ],
    },
    ToolSpec {
        name: "ninja",
        subcommands: &[],
        flags: &[
            "-C",
            "-f",
            "-j",
            "-k",
            "-l",
            "-n",
            "-v",
            "-d",
            "-t",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "gdb",
        subcommands: &[],
        flags: &[
            "-q",
            "--quiet",
            "-batch",
            "-ex",
            "-x",
            "--command",
            "-p",
            "--pid",
            "-tui",
            "--args",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "lldb",
        subcommands: &[],
        flags: &[
            "-o",
            "-O",
            "-p",
            "--attach-pid",
            "-f",
            "--file",
            "--source",
            "-b",
            "--batch",
            "-v",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "valgrind",
        subcommands: &[],
        flags: &[
            "--tool",
            "--leak-check",
            "--track-origins",
            "--error-exitcode",
            "--log-file",
            "--suppressions",
            "--gen-suppressions",
            "--num-callers",
            "--show-leak-kinds",
            "--version",
            "--help",
            "-q",
        ],
    },
    // JVM
    ToolSpec {
        name: "java",
        subcommands: &[],
        flags: JAVA_FLAGS,
    },
    ToolSpec {
        name: "javac",
        subcommands: &[],
        flags: &[
            "-d",
            "-cp",
            "-classpath",
            "-sourcepath",
            "-source",
            "-target",
            "--release",
            "-encoding",
            "-g",
            "-nowarn",
            "-Werror",
            "-Xlint",
            "-verbose",
            "-version",
            "--version",
            "-help",
            "--help",
        ],
    },
    ToolSpec {
        name: "jar",
        subcommands: &[],
        flags: &[
            "--create",
            "-c",
            "--extract",
            "-x",
            "--list",
            "-t",
            "--update",
            "-u",
            "--file",
            "-f",
            "--verbose",
            "-v",
            "--main-class",
            "-e",
            "--manifest",
            "-m",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "jshell",
        subcommands: &[],
        flags: &[
            "--class-path",
            "-cp",
            "--module-path",
            "--add-modules",
            "--execution",
            "-s",
            "--startup",
            "--feedback",
            "-q",
            "-v",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "kotlin",
        subcommands: &[],
        flags: &[
            "-classpath",
            "-cp",
            "-d",
            "-include-runtime",
            "-jvm-target",
            "-version",
            "-help",
        ],
    },
    ToolSpec {
        name: "kotlinc",
        subcommands: &[],
        flags: &[
            "-classpath",
            "-cp",
            "-d",
            "-include-runtime",
            "-jvm-target",
            "-version",
            "-help",
        ],
    },
    ToolSpec {
        name: "gradle",
        subcommands: &[
            "build",
            "clean",
            "test",
            "run",
            "assemble",
            "check",
            "jar",
            "bootRun",
            "dependencies",
            "tasks",
            "projects",
            "wrapper",
            "init",
            "help",
        ],
        flags: &[
            "--help",
            "--version",
            "-q",
            "--quiet",
            "-i",
            "--info",
            "-d",
            "--debug",
            "--stacktrace",
            "--scan",
            "--offline",
            "--no-daemon",
            "--parallel",
            "--refresh-dependencies",
            "-x",
            "--exclude-task",
            "-P",
        ],
    },
    ToolSpec {
        name: "mvn",
        subcommands: MAVEN_SUBS,
        flags: &[
            "-h",
            "--help",
            "-v",
            "--version",
            "-q",
            "--quiet",
            "-X",
            "--debug",
            "-e",
            "--errors",
            "-B",
            "--batch-mode",
            "-U",
            "--update-snapshots",
            "-o",
            "--offline",
            "-D",
            "-P",
            "--activate-profiles",
            "-f",
            "--file",
            "-s",
            "--settings",
            "-T",
            "--threads",
        ],
    },
    ToolSpec {
        name: "scala",
        subcommands: &[],
        flags: &[
            "-classpath",
            "-cp",
            "-d",
            "-version",
            "-help",
            "-howtorun",
            "-i",
            "-e",
        ],
    },
    ToolSpec {
        name: "scalac",
        subcommands: &[],
        flags: &[
            "-classpath",
            "-cp",
            "-d",
            "-deprecation",
            "-feature",
            "-unchecked",
            "-Xlint",
            "-optimise",
            "-version",
            "-help",
        ],
    },
    // Swift / Apple
    ToolSpec {
        name: "swift",
        subcommands: &["build", "run", "test", "package", "repl", "format"],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-c",
            "--configuration",
            "--package-path",
            "--build-path",
            "--triple",
            "--sdk",
            "--toolchain",
            "--disable-sandbox",
        ],
    },
    ToolSpec {
        name: "swiftc",
        subcommands: &[],
        flags: &[
            "-o",
            "-O",
            "-Onone",
            "-g",
            "-emit-executable",
            "-emit-library",
            "-emit-object",
            "-c",
            "-module-name",
            "-I",
            "-L",
            "-l",
            "-framework",
            "-target",
            "-sdk",
            "-parse",
            "-typecheck",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "xcodebuild",
        subcommands: &[],
        flags: &[
            "-project",
            "-workspace",
            "-scheme",
            "-target",
            "-configuration",
            "-sdk",
            "-destination",
            "-archivePath",
            "-derivedDataPath",
            "-quiet",
            "-verbose",
            "-list",
            "-showBuildSettings",
            "-version",
            "-help",
            "-build",
            "-clean",
            "-test",
            "-archive",
        ],
    },
    // PHP / Ruby / Perl / Lua
    ToolSpec {
        name: "php",
        subcommands: &[],
        flags: &[
            "-f",
            "-r",
            "-a",
            "-i",
            "-v",
            "--version",
            "-m",
            "-l",
            "-c",
            "-d",
            "-e",
            "-H",
            "-S",
            "-t",
            "--help",
        ],
    },
    ToolSpec {
        name: "composer",
        subcommands: &[
            "install",
            "update",
            "require",
            "remove",
            "dump-autoload",
            "run",
            "exec",
            "create-project",
            "init",
            "config",
            "show",
            "outdated",
            "validate",
            "diagnose",
            "self",
            "global",
            "search",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "--no-interaction",
            "-n",
            "--no-dev",
            "--dev",
            "-d",
            "--working-dir",
            "--prefer-dist",
            "--prefer-source",
            "--optimize-autoloader",
            "-o",
        ],
    },
    ToolSpec {
        name: "ruby",
        subcommands: &[],
        flags: &[
            "-e",
            "-I",
            "-r",
            "-w",
            "-W",
            "-d",
            "--debug",
            "-v",
            "--version",
            "-h",
            "--help",
            "-c",
            "-n",
            "-p",
            "-l",
            "-a",
            "-0",
        ],
    },
    ToolSpec {
        name: "gem",
        subcommands: &[
            "install",
            "uninstall",
            "update",
            "list",
            "search",
            "info",
            "contents",
            "dependency",
            "build",
            "push",
            "fetch",
            "sources",
            "environment",
            "cleanup",
            "which",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "--local",
            "--remote",
            "--pre",
            "--no-document",
            "-i",
            "--install-dir",
            "-n",
            "--bindir",
        ],
    },
    ToolSpec {
        name: "bundle",
        subcommands: &[
            "install", "update", "exec", "add", "remove", "list", "show", "info", "config",
            "check", "clean", "lock", "package", "init", "gem", "viz", "doctor", "platform",
            "outdated",
        ],
        flags: &[
            "--help",
            "--version",
            "--verbose",
            "--quiet",
            "--gemfile",
            "--path",
            "--without",
            "--with",
            "--jobs",
            "-j",
            "--deployment",
            "--frozen",
        ],
    },
    ToolSpec {
        name: "rake",
        subcommands: &[],
        flags: &[
            "-T",
            "-t",
            "-P",
            "-p",
            "-n",
            "-f",
            "--rakefile",
            "--tasks",
            "--trace",
            "--dry-run",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "perl",
        subcommands: &[],
        flags: &[
            "-e",
            "-E",
            "-c",
            "-w",
            "-W",
            "-I",
            "-M",
            "-m",
            "-n",
            "-p",
            "-l",
            "-a",
            "-0",
            "-v",
            "--version",
            "-h",
            "--help",
        ],
    },
    ToolSpec {
        name: "lua",
        subcommands: &[],
        flags: &["-e", "-l", "-i", "-v", "-h", "--help", "--version"],
    },
    ToolSpec {
        name: "luajit",
        subcommands: &[],
        flags: &["-e", "-l", "-i", "-v", "-h", "--help", "--version"],
    },
    // Other languages
    ToolSpec {
        name: "Rscript",
        subcommands: &[],
        flags: &[
            "--vanilla",
            "--no-save",
            "--no-restore",
            "--no-site-file",
            "--no-init-file",
            "--slave",
            "-e",
            "--version",
            "--help",
            "-h",
        ],
    },
    ToolSpec {
        name: "julia",
        subcommands: &[],
        flags: &[
            "--project",
            "-p",
            "--threads",
            "-e",
            "--eval",
            "-i",
            "--interactive",
            "-q",
            "--quiet",
            "--startup-file",
            "--history-file",
            "--version",
            "-v",
            "--help",
            "-h",
        ],
    },
    ToolSpec {
        name: "dotnet",
        subcommands: &[
            "new", "build", "run", "test", "publish", "restore", "add", "remove", "list", "pack",
            "nuget", "tool", "sln", "watch", "format", "clean",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbosity",
            "-c",
            "--configuration",
            "-o",
            "--output",
            "--no-restore",
            "--no-build",
            "-f",
            "--framework",
            "-r",
            "--runtime",
            "--self-contained",
        ],
    },
    ToolSpec {
        name: "zig",
        subcommands: &[
            "build",
            "build-exe",
            "build-lib",
            "build-obj",
            "run",
            "test",
            "fmt",
            "init",
            "version",
            "env",
        ],
        flags: &[
            "--help",
            "--version",
            "-O",
            "-femit-bin",
            "-target",
            "-mcpu",
            "-I",
            "-L",
            "-l",
            "-D",
            "--release",
            "--release-fast",
            "--release-safe",
            "--release-small",
        ],
    },
    ToolSpec {
        name: "nim",
        subcommands: &[
            "c",
            "cc",
            "cpp",
            "js",
            "r",
            "run",
            "check",
            "doc",
            "build",
            "install",
            "uninstall",
        ],
        flags: &[
            "--help",
            "--version",
            "-o",
            "--out",
            "-d",
            "--define",
            "-p",
            "--path",
            "-I",
            "--include",
            "-L",
            "--linker",
            "--opt",
            "--debugger",
            "--release",
            "--threads",
            "--gc",
        ],
    },
    ToolSpec {
        name: "crystal",
        subcommands: &["build", "run", "spec", "init", "tool", "version"],
        flags: &[
            "--help",
            "--version",
            "-o",
            "--output",
            "--release",
            "--debug",
            "--no-debug",
            "-D",
            "--define",
            "-I",
            "--include",
            "-L",
            "--link-flags",
            "--verbose",
        ],
    },
    ToolSpec {
        name: "elixir",
        subcommands: &[],
        flags: &[
            "-e",
            "--eval",
            "-r",
            "--require",
            "-S",
            "--script",
            "-pa",
            "-pz",
            "--erl",
            "--elixir",
            "-v",
            "--version",
            "-h",
            "--help",
        ],
    },
    ToolSpec {
        name: "erl",
        subcommands: &[],
        flags: &[
            "-s",
            "-eval",
            "-pa",
            "-pz",
            "-noshell",
            "-noinput",
            "-sname",
            "-name",
            "-setcookie",
            "-boot",
            "-version",
            "-help",
        ],
    },
    ToolSpec {
        name: "ghc",
        subcommands: &[],
        flags: &[
            "-o",
            "-O",
            "-O0",
            "-O2",
            "-Wall",
            "-Werror",
            "-i",
            "-I",
            "-L",
            "-l",
            "-package",
            "-hide-package",
            "-X",
            "-cpp",
            "-threaded",
            "-rtsopts",
            "-dynamic",
            "--make",
            "-e",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "cabal",
        subcommands: &[
            "build", "run", "test", "repl", "install", "update", "init", "sdist", "haddock",
            "freeze", "new", "list", "info", "fetch", "get", "upload",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "--builddir",
            "--project-dir",
            "--offline",
            "--enable-tests",
            "--enable-benchmarks",
            "-f",
            "--flags",
            "--constraint",
        ],
    },
    ToolSpec {
        name: "stack",
        subcommands: &[
            "build", "run", "test", "exec", "ghci", "install", "setup", "update", "new", "ls",
            "path", "ide", "haddock", "sdist", "hoogle", "dot", "docker",
        ],
        flags: &[
            "--help",
            "--version",
            "-v",
            "--verbose",
            "--stack-yaml",
            "--resolver",
            "--system-ghc",
            "--no-system-ghc",
            "--local-bin-path",
            "--jobs",
            "-j",
            "--fast",
        ],
    },
    ToolSpec {
        name: "ocaml",
        subcommands: &[],
        flags: &[
            "-o",
            "-c",
            "-i",
            "-I",
            "-pp",
            "-ppx",
            "-w",
            "-warn-error",
            "-g",
            "-bin-annot",
            "-version",
            "-help",
            "--help",
        ],
    },
    ToolSpec {
        name: "opam",
        subcommands: &[
            "install",
            "remove",
            "update",
            "upgrade",
            "list",
            "show",
            "info",
            "search",
            "switch",
            "init",
            "pin",
            "source",
            "exec",
            "config",
            "repository",
        ],
        flags: &[
            "--help",
            "--version",
            "-y",
            "--yes",
            "-v",
            "--verbose",
            "-q",
            "--quiet",
            "--root",
            "--switch",
            "--jobs",
        ],
    },
    ToolSpec {
        name: "dune",
        subcommands: &[
            "build",
            "run",
            "test",
            "exec",
            "clean",
            "init",
            "install",
            "uninstall",
            "external-lib-deps",
            "utop",
            "format",
        ],
        flags: &[
            "--help",
            "--version",
            "-j",
            "--jobs",
            "--root",
            "--profile",
            "--release",
            "--dev",
            "--watch",
            "--display",
        ],
    },
    // Database CLIs
    ToolSpec {
        name: "sqlite3",
        subcommands: &[],
        flags: &[
            "-help",
            "-version",
            "-header",
            "-noheader",
            "-column",
            "-csv",
            "-json",
            "-line",
            "-list",
            "-html",
            "-separator",
            "-nullvalue",
            "-readonly",
            "-bail",
            "-echo",
            "-init",
            "-cmd",
            "-stats",
        ],
    },
    ToolSpec {
        name: "mysql",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-u",
            "--user",
            "-p",
            "--password",
            "-h",
            "--host",
            "-P",
            "--port",
            "-D",
            "--database",
            "-e",
            "--execute",
            "--ssl-mode",
            "--default-character-set",
        ],
    },
    ToolSpec {
        name: "psql",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "-U",
            "--username",
            "-d",
            "--dbname",
            "-h",
            "--host",
            "-p",
            "--port",
            "-c",
            "--command",
            "-f",
            "--file",
            "-l",
            "--list",
            "-v",
            "--set",
            "-q",
            "--quiet",
            "-A",
            "--no-align",
            "-t",
            "--tuples-only",
        ],
    },
    ToolSpec {
        name: "mongosh",
        subcommands: &[],
        flags: &[
            "--help",
            "--version",
            "--host",
            "--port",
            "--username",
            "--password",
            "--authenticationDatabase",
            "--eval",
            "--file",
            "--quiet",
            "--norc",
            "--shell",
            "--nodb",
        ],
    },
    ToolSpec {
        name: "redis-cli",
        subcommands: &[],
        flags: &[
            "-h",
            "--host",
            "-p",
            "--port",
            "-a",
            "--pass",
            "-u",
            "--uri",
            "-n",
            "--db",
            "--raw",
            "--no-raw",
            "--csv",
            "--json",
            "-c",
            "--cluster",
            "-s",
            "--tls",
            "--help",
            "--version",
        ],
    },
    // Common shell utilities: no subcommands, just the flags people reach for
    // (`grep -`, `rm --`). ntc's own builtins (`ls`, `cp`, `mv`, `head`, …)
    // stay out of this list so their completion is never overridden.
    ToolSpec {
        name: "grep",
        subcommands: &[],
        flags: &[
            "-i",
            "--ignore-case",
            "-v",
            "--invert-match",
            "-r",
            "-R",
            "--recursive",
            "-n",
            "--line-number",
            "-H",
            "--with-filename",
            "-h",
            "--no-filename",
            "-l",
            "--files-with-matches",
            "-L",
            "--files-without-match",
            "-c",
            "--count",
            "-o",
            "--only-matching",
            "-w",
            "--word-regexp",
            "-x",
            "--line-regexp",
            "-E",
            "--extended-regexp",
            "-F",
            "--fixed-strings",
            "-P",
            "--perl-regexp",
            "-e",
            "--regexp",
            "-f",
            "--file",
            "-A",
            "--after-context",
            "-B",
            "--before-context",
            "-C",
            "--context",
            "-m",
            "--max-count",
            "--include",
            "--exclude",
            "--exclude-dir",
            "--color",
            "--no-color",
            "--binary-files",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "egrep",
        subcommands: &[],
        flags: &[
            "-i",
            "-v",
            "-r",
            "-n",
            "-l",
            "-c",
            "-w",
            "-o",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "rm",
        subcommands: &[],
        flags: &[
            "-f",
            "--force",
            "-r",
            "-R",
            "--recursive",
            "-i",
            "--interactive",
            "-I",
            "-v",
            "--verbose",
            "-d",
            "--dir",
            "--no-preserve-root",
            "--preserve-root",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "cat",
        subcommands: &[],
        flags: &[
            "-n",
            "--number",
            "-b",
            "--number-nonblank",
            "-s",
            "--squeeze-blank",
            "-E",
            "--show-ends",
            "-T",
            "--show-tabs",
            "-A",
            "--show-all",
            "-v",
            "--version",
            "--help",
        ],
    },
    ToolSpec {
        name: "find",
        subcommands: &[],
        flags: &[
            "-name",
            "-iname",
            "-path",
            "-ipath",
            "-type",
            "-size",
            "-mtime",
            "-mmin",
            "-newer",
            "-user",
            "-group",
            "-perm",
            "-empty",
            "-maxdepth",
            "-mindepth",
            "-exec",
            "-execdir",
            "-delete",
            "-print",
            "-print0",
            "-not",
            "-o",
            "-a",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "sed",
        subcommands: &[],
        flags: &[
            "-i",
            "--in-place",
            "-e",
            "--expression",
            "-f",
            "--file",
            "-n",
            "--quiet",
            "-r",
            "-E",
            "--regexp-extended",
            "-s",
            "--separate",
            "-z",
            "--null-data",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "awk",
        subcommands: &[],
        flags: &[
            "-F",
            "--field-separator",
            "-f",
            "--file",
            "-v",
            "--assign",
            "-i",
            "--include",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "chmod",
        subcommands: &[],
        flags: &[
            "-R",
            "--recursive",
            "-v",
            "--verbose",
            "-c",
            "--changes",
            "-f",
            "--silent",
            "--reference",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "chown",
        subcommands: &[],
        flags: &[
            "-R",
            "--recursive",
            "-v",
            "--verbose",
            "-c",
            "--changes",
            "-f",
            "--silent",
            "--from",
            "--reference",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "mkdir",
        subcommands: &[],
        flags: &[
            "-p",
            "--parents",
            "-v",
            "--verbose",
            "-m",
            "--mode",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "rmdir",
        subcommands: &[],
        flags: &["-p", "--parents", "-v", "--verbose", "--help", "--version"],
    },
    ToolSpec {
        name: "touch",
        subcommands: &[],
        flags: &[
            "-a",
            "-m",
            "-c",
            "--no-create",
            "-d",
            "--date",
            "-t",
            "-r",
            "--reference",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "ln",
        subcommands: &[],
        flags: &[
            "-s",
            "--symbolic",
            "-f",
            "--force",
            "-n",
            "--no-dereference",
            "-r",
            "--relative",
            "-v",
            "--verbose",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "tar",
        subcommands: &[],
        flags: &[
            "-c",
            "--create",
            "-x",
            "--extract",
            "-t",
            "--list",
            "-f",
            "--file",
            "-v",
            "--verbose",
            "-z",
            "--gzip",
            "-j",
            "--bzip2",
            "-J",
            "--xz",
            "-C",
            "--directory",
            "--exclude",
            "--strip-components",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "gzip",
        subcommands: &[],
        flags: &[
            "-d",
            "--decompress",
            "-k",
            "--keep",
            "-f",
            "--force",
            "-v",
            "--verbose",
            "-c",
            "--stdout",
            "-r",
            "--recursive",
            "-9",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "unzip",
        subcommands: &[],
        flags: &[
            "-o",
            "-n",
            "-q",
            "-d",
            "-l",
            "-j",
            "-x",
            "-P",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "zip",
        subcommands: &[],
        flags: &[
            "-r",
            "-q",
            "-d",
            "-x",
            "-9",
            "-e",
            "-P",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "curl",
        subcommands: &[],
        flags: &[
            "-o",
            "--output",
            "-O",
            "--remote-name",
            "-L",
            "--location",
            "-i",
            "--include",
            "-I",
            "--head",
            "-s",
            "--silent",
            "-S",
            "--show-error",
            "-v",
            "--verbose",
            "-H",
            "--header",
            "-d",
            "--data",
            "-X",
            "--request",
            "-u",
            "--user",
            "-f",
            "--fail",
            "-k",
            "--insecure",
            "-A",
            "--user-agent",
            "-b",
            "--cookie",
            "-c",
            "--cookie-jar",
            "--compressed",
            "--url",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "wget",
        subcommands: &[],
        flags: &[
            "-O",
            "--output-document",
            "-P",
            "--directory-prefix",
            "-q",
            "--quiet",
            "-v",
            "--verbose",
            "-c",
            "--continue",
            "-r",
            "--recursive",
            "-np",
            "--no-parent",
            "-N",
            "--timestamping",
            "--header",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "ssh",
        subcommands: &[],
        flags: &[
            "-p",
            "-i",
            "-L",
            "-R",
            "-D",
            "-J",
            "-o",
            "-N",
            "-f",
            "-T",
            "-C",
            "-q",
            "-v",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "scp",
        subcommands: &[],
        flags: &[
            "-r",
            "-P",
            "-i",
            "-p",
            "-C",
            "-q",
            "-v",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "rsync",
        subcommands: &[],
        flags: &[
            "-a",
            "--archive",
            "-r",
            "--recursive",
            "-v",
            "--verbose",
            "-z",
            "--compress",
            "-h",
            "--human-readable",
            "-P",
            "--progress",
            "-n",
            "--dry-run",
            "-e",
            "--rsh",
            "-u",
            "--update",
            "--delete",
            "--exclude",
            "--include",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "kill",
        subcommands: &[],
        flags: &[
            "-l", "--list", "-s", "--signal", "-9", "-15", "-TERM", "-KILL", "-INT", "-HUP",
            "--help",
        ],
    },
    ToolSpec {
        name: "df",
        subcommands: &[],
        flags: &[
            "-h",
            "--human-readable",
            "-i",
            "--inodes",
            "-T",
            "--print-type",
            "-a",
            "--all",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "du",
        subcommands: &[],
        flags: &[
            "-h",
            "--human-readable",
            "-s",
            "--summarize",
            "-a",
            "--all",
            "-c",
            "--total",
            "-d",
            "--max-depth",
            "-x",
            "--one-file-system",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "uniq",
        subcommands: &[],
        flags: &[
            "-c",
            "--count",
            "-d",
            "--repeated",
            "-u",
            "--unique",
            "-i",
            "--ignore-case",
            "-f",
            "--skip-fields",
            "-s",
            "--skip-chars",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "wc",
        subcommands: &[],
        flags: &[
            "-l",
            "--lines",
            "-w",
            "--words",
            "-c",
            "--bytes",
            "-m",
            "--chars",
            "-L",
            "--max-line-length",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "diff",
        subcommands: &[],
        flags: &[
            "-u",
            "--unified",
            "-c",
            "--context",
            "-i",
            "--ignore-case",
            "-w",
            "--ignore-all-space",
            "-b",
            "--ignore-space-change",
            "-B",
            "--ignore-blank-lines",
            "-r",
            "--recursive",
            "-q",
            "--brief",
            "-y",
            "--side-by-side",
            "-N",
            "--new-file",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "which",
        subcommands: &[],
        flags: &["-a", "--all", "--help", "--version"],
    },
    ToolSpec {
        name: "whereis",
        subcommands: &[],
        flags: &["-b", "-m", "-s", "-u", "--help", "--version"],
    },
    ToolSpec {
        name: "env",
        subcommands: &[],
        flags: &[
            "-i",
            "--ignore-environment",
            "-u",
            "--unset",
            "-C",
            "--chdir",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "xargs",
        subcommands: &[],
        flags: &[
            "-0",
            "--null",
            "-n",
            "--max-args",
            "-I",
            "--replace",
            "-P",
            "--max-procs",
            "-L",
            "-r",
            "--no-run-if-empty",
            "-t",
            "--verbose",
            "-d",
            "--delimiter",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "less",
        subcommands: &[],
        flags: &[
            "-N",
            "--LINE-NUMBERS",
            "-S",
            "--chop-long-lines",
            "-R",
            "--RAW-CONTROL-CHARS",
            "-i",
            "--ignore-case",
            "-M",
            "-F",
            "--quit-if-one-screen",
            "-X",
            "--help",
            "--version",
        ],
    },
    ToolSpec {
        name: "tee",
        subcommands: &[],
        flags: &[
            "-a",
            "--append",
            "-i",
            "--ignore-interrupts",
            "--help",
            "--version",
        ],
    },
    // macOS package manager. Hinted with the zsh integration (`config
    // integration_shell on`) as well as with `code_tools`; no formulae lists,
    // so it stays instant and offline.
    ToolSpec {
        name: "brew",
        subcommands: &[
            "install",
            "uninstall",
            "remove",
            "reinstall",
            "upgrade",
            "update",
            "list",
            "ls",
            "info",
            "search",
            "outdated",
            "cleanup",
            "doctor",
            "deps",
            "uses",
            "leaves",
            "link",
            "unlink",
            "tap",
            "untap",
            "pin",
            "unpin",
            "services",
            "shellenv",
            "config",
            "analytics",
            "autoremove",
            "fetch",
            "home",
            "log",
            "cat",
            "edit",
            "bundle",
            "casks",
        ],
        flags: &[
            "-h",
            "--help",
            "-v",
            "--verbose",
            "--version",
            "--debug",
            "-q",
            "--quiet",
            "--cask",
            "--formula",
            "--force",
            "--overwrite",
            "--dry-run",
            "-n",
            "--json",
            "--global",
            "--user",
            "--greedy",
            "--auto-update",
            "--no-auto-update",
            "--zap",
            "--appdir",
            "--no-quarantine",
        ],
    },
];

fn find(name: &str) -> Option<&'static ToolSpec> {
    SPECS
        .iter()
        .find(|spec| spec.name.eq_ignore_ascii_case(name))
}

pub(crate) fn owns(cmd: &str) -> bool {
    find(cmd).is_some()
}

/// Curated tool names present on `PATH`, in canonical casing.
pub(crate) fn installed_names() -> Vec<String> {
    let mut names: Vec<String> = SPECS
        .iter()
        .filter(|spec| super::is_installed(spec.name))
        .map(|spec| spec.name.to_string())
        .collect();
    names.sort();
    names
}

pub(crate) fn invalidate() {
    if let Ok(mut c) = dynamic().lock() {
        c.clear();
    }
}

fn dynamic() -> &'static Mutex<HashMap<(PathBuf, String), (Instant, Vec<String>)>> {
    static CACHE: OnceLock<Mutex<HashMap<(PathBuf, String), (Instant, Vec<String>)>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached(key: &str, f: impl FnOnce() -> Vec<String>) -> Vec<String> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let k = (cwd, key.to_string());
    if let Ok(c) = dynamic().lock() {
        if let Some((at, vals)) = c.get(&k) {
            if at.elapsed() < TOOL_TTL {
                return vals.clone();
            }
        }
    }
    let vals = f();
    if let Ok(mut c) = dynamic().lock() {
        if c.len() > 128 {
            c.clear();
        }
        c.insert(k, (Instant::now(), vals.clone()));
    }
    vals
}

/// `before` is the whole active segment, e.g. `git checkout ma`.
pub(crate) fn complete(tool: &str, before: &str, _pos: usize) -> Option<Vec<String>> {
    let spec = find(tool)?;
    let parts: Vec<&str> = before.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }
    let trailing_space = before.ends_with(char::is_whitespace);
    let at = slot(&parts, trailing_space);
    let frag = if trailing_space {
        ""
    } else {
        parts.last().copied().unwrap_or("")
    };
    // Still typing the tool name itself: level-1 completion owns it.
    if parts.len() == 1 && !trailing_space && !frag.starts_with('-') {
        return None;
    }

    let is_flag = frag.starts_with('-');
    let mut tokens: Vec<String> = match tool {
        "git" => git_tokens(&parts, trailing_space),
        "cargo" => cargo_tokens(&parts, trailing_space),
        "npm" => npm_tokens(&parts, trailing_space),
        "docker" => docker_tokens(&parts, trailing_space),
        _ => Vec::new(),
    };

    if is_flag {
        // Basic flags work for every curated tool, even when the rich
        // provider only knows the subcommand it is on (`git commit -`).
        tokens.extend(spec.flags.iter().map(|s| s.to_string()));
    } else if at == 1 {
        tokens.extend(spec.subcommands.iter().map(|s| s.to_string()));
    }

    Some(tokens_to_lines(before, tokens))
}

fn slot(parts: &[&str], trailing_space: bool) -> usize {
    if trailing_space {
        parts.len()
    } else {
        parts.len().saturating_sub(1)
    }
}

fn git_tokens(parts: &[&str], trailing_space: bool) -> Vec<String> {
    const SUBCOMMANDS: &[&str] = &[
        "add",
        "am",
        "apply",
        "archive",
        "bisect",
        "blame",
        "branch",
        "bundle",
        "checkout",
        "cherry-pick",
        "clean",
        "clone",
        "commit",
        "config",
        "describe",
        "diff",
        "fetch",
        "format-patch",
        "fsck",
        "gc",
        "grep",
        "init",
        "log",
        "ls-files",
        "merge",
        "mv",
        "pull",
        "push",
        "rebase",
        "reflog",
        "remote",
        "reset",
        "restore",
        "revert",
        "rm",
        "shortlog",
        "show",
        "stash",
        "status",
        "submodule",
        "switch",
        "tag",
        "worktree",
    ];

    let at = slot(parts, trailing_space);
    if at == 1 {
        return SUBCOMMANDS.iter().map(|s| s.to_string()).collect();
    }
    let sub = parts.get(1).copied().unwrap_or("");
    let mut out: Vec<String> = match sub {
        "add" => vec!["-A", "-p", "-u", "-n", "-v", "--", "."],
        "commit" => vec![
            "-m",
            "-a",
            "-p",
            "--amend",
            "--no-edit",
            "--allow-empty",
            "--signoff",
        ],
        "push" => vec![
            "-u",
            "--force",
            "--force-with-lease",
            "--tags",
            "--all",
            "--set-upstream",
            "--dry-run",
        ],
        "pull" => vec!["--rebase", "--ff-only", "--no-ff", "--autostash", "--all"],
        "fetch" => vec!["--all", "--prune", "--tags", "--depth"],
        "branch" => vec![
            "-a", "-r", "-d", "-D", "-m", "-v", "-vv", "--list", "--merged",
        ],
        "checkout" => vec!["-b", "-B", "--", "--detach", "-f"],
        "switch" => vec!["-c", "-C", "--detach", "--discard-changes"],
        "log" => vec![
            "--oneline",
            "--graph",
            "--all",
            "--stat",
            "--name-only",
            "--decorate",
            "-n",
            "-p",
        ],
        "status" => vec!["-s", "--short", "--porcelain", "--branch", "--ignored"],
        "diff" => vec![
            "--stat",
            "--cached",
            "--staged",
            "--name-only",
            "--name-status",
        ],
        "stash" => vec![
            "list", "show", "pop", "apply", "drop", "push", "clear", "-u",
        ],
        "remote" => vec!["-v", "add", "remove", "rename", "set-url", "show", "prune"],
        "tag" => vec!["-l", "-d", "-a", "-m", "-f", "--list"],
        "merge" => vec!["--no-ff", "--ff-only", "--abort", "--continue", "--squash"],
        "rebase" => vec![
            "-i",
            "--abort",
            "--continue",
            "--skip",
            "--onto",
            "--autostash",
        ],
        "reset" => vec!["--soft", "--mixed", "--hard", "--keep", "--merge", "HEAD"],
        "restore" => vec!["--staged", "--worktree", "--source", "."],
        "revert" => vec!["--no-commit", "--abort", "--continue"],
        "clean" => vec!["-f", "-d", "-x", "-n", "-i"],
        "clone" => vec!["--depth", "--branch", "--recursive", "--bare"],
        "reflog" => vec!["show", "expire", "delete"],
        "submodule" => vec!["add", "init", "update", "status", "sync", "foreach"],
        "worktree" => vec!["add", "list", "remove", "prune", "lock", "unlock"],
        "bisect" => vec!["start", "good", "bad", "reset", "run", "skip"],
        _ => vec![],
    }
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    match sub {
        "add" | "rm" | "mv" | "restore" | "checkout" if at >= 2 => {
            out.extend(git_files());
        }
        "checkout" | "switch" | "merge" | "rebase" | "reset" | "diff" | "log" | "show"
        | "branch" | "tag" => {
            out.extend(git_refs());
        }
        "push" | "pull" | "fetch" => {
            out.extend(git_remotes());
        }
        _ => {}
    }
    out
}

fn cargo_tokens(parts: &[&str], trailing_space: bool) -> Vec<String> {
    const SUBCOMMANDS: &[&str] = &[
        "build",
        "run",
        "test",
        "check",
        "clippy",
        "fmt",
        "doc",
        "bench",
        "clean",
        "update",
        "add",
        "remove",
        "publish",
        "install",
        "uninstall",
        "new",
        "init",
        "search",
        "tree",
        "metadata",
        "watch",
        "fix",
        "generate-lockfile",
        "vendor",
        "package",
    ];
    const FLAGS: &[&str] = &[
        "--release",
        "--example",
        "--bin",
        "--features",
        "--all-features",
        "--no-default-features",
        "--target",
        "--package",
        "-p",
        "--verbose",
        "-v",
        "--quiet",
        "-q",
        "--offline",
        "--locked",
        "--workspace",
        "--all",
    ];

    let at = slot(parts, trailing_space);
    if at == 1 {
        return SUBCOMMANDS.iter().map(|s| s.to_string()).collect();
    }
    let mut out: Vec<String> = FLAGS.iter().map(|s| s.to_string()).collect();
    // `cargo run --example ` / `cargo test --bin `: complete actual targets.
    let prev = if trailing_space {
        parts.last().copied().unwrap_or("")
    } else {
        parts
            .get(parts.len().saturating_sub(2))
            .copied()
            .unwrap_or("")
    };
    match prev {
        "--example" => out.extend(file_stems("examples", "rs")),
        "--bin" => {
            out.extend(file_stems("src/bin", "rs"));
        }
        "--package" | "-p" => out.extend(workspace_packages()),
        _ => {}
    }
    out
}

fn npm_tokens(parts: &[&str], trailing_space: bool) -> Vec<String> {
    const SUBCOMMANDS: &[&str] = &[
        "install",
        "i",
        "uninstall",
        "un",
        "remove",
        "rm",
        "run",
        "run-script",
        "test",
        "start",
        "build",
        "publish",
        "update",
        "ls",
        "init",
        "ci",
        "audit",
        "outdated",
        "link",
        "pack",
        "version",
        "view",
        "add",
        "exec",
        "create",
        "why",
    ];
    let at = slot(parts, trailing_space);
    if at == 1 {
        return SUBCOMMANDS.iter().map(|s| s.to_string()).collect();
    }
    let mut out: Vec<String> = Vec::new();
    match parts.get(1).copied().unwrap_or("") {
        "run" | "run-script" if at == 2 => out.extend(npm_scripts()),
        "install" | "i" | "add" => out.extend(vec![
            "--save-dev".to_string(),
            "-D".to_string(),
            "--save".to_string(),
            "-S".to_string(),
            "--global".to_string(),
            "-g".to_string(),
        ]),
        _ => {}
    }
    out
}

fn docker_tokens(parts: &[&str], trailing_space: bool) -> Vec<String> {
    const SUBCOMMANDS: &[&str] = &[
        "run",
        "ps",
        "images",
        "build",
        "pull",
        "push",
        "exec",
        "logs",
        "stop",
        "start",
        "restart",
        "rm",
        "rmi",
        "compose",
        "volume",
        "network",
        "inspect",
        "stats",
        "cp",
        "tag",
        "login",
        "logout",
        "search",
        "version",
        "info",
        "system",
        "image",
        "container",
    ];
    const COMPOSE: &[&str] = &[
        "up", "down", "logs", "ps", "build", "restart", "exec", "config", "pull", "stop", "start",
        "run", "rm",
    ];
    let at = slot(parts, trailing_space);
    if at == 1 {
        return SUBCOMMANDS.iter().map(|s| s.to_string()).collect();
    }
    let mut out: Vec<String> = Vec::new();
    match parts.get(1).copied().unwrap_or("") {
        "compose" if at == 2 => out.extend(COMPOSE.iter().map(|s| s.to_string())),
        "exec" | "logs" | "stop" | "start" | "restart" | "rm" | "inspect" | "stats" => {
            out.extend(docker_names());
        }
        "rmi" => out.extend(docker_images()),
        _ => {}
    }
    out
}

fn git_files() -> Vec<String> {
    cached("git-files", || {
        run_capture("git", &["ls-files", "-co", "--exclude-standard"])
            .map(|s| nonempty_lines(&s))
            .unwrap_or_default()
    })
}

fn git_refs() -> Vec<String> {
    cached("git-refs", || {
        let mut out = Vec::new();
        if let Some(s) = run_capture(
            "git",
            &[
                "for-each-ref",
                "--format=%(refname:short)",
                "refs/heads",
                "refs/remotes",
                "refs/tags",
            ],
        ) {
            out.extend(nonempty_lines(&s));
        }
        out
    })
}

fn git_remotes() -> Vec<String> {
    cached("git-remotes", || {
        run_capture("git", &["remote"])
            .map(|s| nonempty_lines(&s))
            .unwrap_or_default()
    })
}

fn file_stems(dir: &str, ext: &str) -> Vec<String> {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(cwd.join(dir)) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(stem) = name.strip_suffix(&format!(".{}", ext)) {
                out.push(stem.to_string());
            }
        }
    }
    out.sort();
    out
}

fn npm_scripts() -> Vec<String> {
    cached("npm-scripts", || {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let Ok(text) = std::fs::read_to_string(cwd.join("package.json")) else {
            return Vec::new();
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
            return Vec::new();
        };
        let mut out: Vec<String> = json
            .get("scripts")
            .and_then(|s| s.as_object())
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        out.sort();
        out
    })
}

fn workspace_packages() -> Vec<String> {
    cached("cargo-packages", || {
        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let Ok(text) = std::fs::read_to_string(cwd.join("Cargo.toml")) else {
            return Vec::new();
        };
        let Ok(toml) = toml::from_str::<toml::Value>(&text) else {
            return Vec::new();
        };
        toml.get("workspace")
            .and_then(|w| w.get("members"))
            .and_then(|m| m.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str())
                    .filter_map(|p| {
                        PathBuf::from(p)
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

fn docker_names() -> Vec<String> {
    cached("docker-names", || {
        run_capture("docker", &["ps", "-a", "--format", "{{.Names}}"])
            .map(|s| nonempty_lines(&s))
            .unwrap_or_default()
    })
}

fn docker_images() -> Vec<String> {
    cached("docker-images", || {
        run_capture(
            "docker",
            &["images", "--format", "{{.Repository}}:{{.Tag}}"],
        )
        .map(|s| nonempty_lines(&s))
        .unwrap_or_default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_curated_names_are_owned() {
        for name in [
            "g++", "python", "java", "node", "Rscript", "cargo", "git", "grep", "rm", "curl",
            "brew",
        ] {
            assert!(owns(name), "{name}");
        }
        assert!(owns("RSCRIPT"));
        assert!(!owns("ntcfixture"));
        assert!(!owns("notacompiler"));
    }

    #[test]
    fn basic_flags_cover_the_documented_examples() {
        let flags =
            |tool: &str, before: &str| complete(tool, before, before.len()).expect("tool handled");
        let gpp = flags("g++", "g++ -");
        for flag in ["-o", "-Wall", "-std=c++17"] {
            assert!(gpp.contains(&format!("g++ {flag}")), "{gpp:?}");
        }
        let java = flags("java", "java -");
        for flag in ["-jar", "-cp"] {
            assert!(java.contains(&format!("java {flag}")), "{java:?}");
        }
        let python = flags("python", "python -");
        for flag in ["-m", "-c"] {
            assert!(python.contains(&format!("python {flag}")), "{python:?}");
        }
        let node = flags("node", "node -");
        for flag in ["-e", "--inspect"] {
            assert!(node.contains(&format!("node {flag}")), "{node:?}");
        }
    }

    #[test]
    fn static_subcommands_complete_at_the_first_slot() {
        let cands = complete("go", "go b", 4).expect("go handled");
        assert!(cands.contains(&"go build".to_string()), "{cands:?}");
        let cands = complete("pip", "pip in", 6).expect("pip handled");
        assert!(cands.contains(&"pip install".to_string()), "{cands:?}");
        // Dynamic providers still work.
        let cands = complete("git", "git ch", 6).expect("git handled");
        assert!(cands.contains(&"git checkout".to_string()), "{cands:?}");
    }

    #[test]
    fn common_shell_utilities_hint_their_flags() {
        let flags =
            |tool: &str, before: &str| complete(tool, before, before.len()).expect("tool handled");
        let grep = flags("grep", "grep -");
        for flag in ["-i", "--ignore-case", "-r"] {
            assert!(grep.contains(&format!("grep {flag}")), "{grep:?}");
        }
        let grep = flags("grep", "grep --");
        assert!(grep.contains(&"grep --recursive".to_string()), "{grep:?}");
        let rm = flags("rm", "rm -");
        for flag in ["-f", "-r", "--force"] {
            assert!(rm.contains(&format!("rm {flag}")), "{rm:?}");
        }
        let curl = flags("curl", "curl --");
        assert!(curl.contains(&"curl --location".to_string()), "{curl:?}");
    }

    #[test]
    fn brew_hints_its_subcommands_and_flags() {
        let cands = complete("brew", "brew ins", 8).expect("brew handled");
        assert!(cands.contains(&"brew install".to_string()), "{cands:?}");
        let cands = complete("brew", "brew up", 7).expect("brew handled");
        assert!(cands.contains(&"brew upgrade".to_string()), "{cands:?}");
        let flags = complete("brew", "brew --", 7).expect("brew handled");
        assert!(flags.contains(&"brew --cask".to_string()), "{flags:?}");
        assert!(flags.contains(&"brew --formula".to_string()), "{flags:?}");
    }

    #[test]
    fn fragments_never_get_suggested_for_the_tool_name_itself() {
        assert!(complete("g++", "g++", 3).is_none());
    }
}
