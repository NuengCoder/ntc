//! `ntc init project` — scaffold a new project directory, or fill in the
//! current one. Existing files are never overwritten.

use crate::config::Config;
use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_warning};
use crate::shell::commands::builtin::ran::ranfile::parser;
use crate::t;
use anyhow::Result;
use chrono::Datelike;
use std::fs;
use std::io::IsTerminal;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::shell::commands::support::local_config::LOCAL_CONFIG_TEMPLATE;
use crate::shell::commands::support::opencode::scaffold_opencode;

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

const README_TEMPLATE: &str = include_str!("assets/project_template/README.md");
const CHANGELOG_TEMPLATE: &str = include_str!("assets/project_template/CHANGELOG.md");
const CONTEXT_TEMPLATE: &str = include_str!("assets/project_template/CONTEXT.md");
const AGENTS_TEMPLATE: &str = include_str!("assets/project_template/AGENTS.md");
const INDEX_TEMPLATE: &str = include_str!("assets/project_template/index.html");
const OPENCODE_TEMPLATE: &str = include_str!("assets/project_template/opencode.jsonc");
const GITIGNORE_TEMPLATE: &str = include_str!("assets/project_template/gitignore");
const ENV_EXAMPLE_TEMPLATE: &str = include_str!("assets/project_template/env.example");
const LICENSE_MIT: &str = include_str!("assets/project_template/LICENSE_MIT");
const LICENSE_APACHE: &str = include_str!("assets/project_template/LICENSE_APACHE");
const PY_MAIN: &str = include_str!("assets/project_template/python/main.py");
const PY_REQUIREMENTS: &str = include_str!("assets/project_template/python/requirements.txt");
const RUST_MAIN: &str = include_str!("assets/project_template/rust/main.rs");
const RUST_CARGO: &str = include_str!("assets/project_template/rust/Cargo.toml");
const GO_MAIN: &str = include_str!("assets/project_template/go/main.go");
const JS_PACKAGE: &str = include_str!("assets/project_template/js/package.json");
const JS_MAIN: &str = include_str!("assets/project_template/js/index.js");
const TS_PACKAGE: &str = include_str!("assets/project_template/ts/package.json");
const TS_CONFIG: &str = include_str!("assets/project_template/ts/tsconfig.json");
const TS_MAIN: &str = include_str!("assets/project_template/ts/index.ts");
const JAVA_MAIN: &str = include_str!("assets/project_template/java/Main.java");
const JAVA_POM: &str = include_str!("assets/project_template/java/pom.xml");
const C_MAIN: &str = include_str!("assets/project_template/c/main.c");
const C_MAKEFILE: &str = include_str!("assets/project_template/c/Makefile");
const CPP_MAIN: &str = include_str!("assets/project_template/cpp/main.cpp");
const CPP_MAKEFILE: &str = include_str!("assets/project_template/cpp/Makefile");
const DOCKERIGNORE_TEMPLATE: &str = include_str!("assets/project_template/dockerignore");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lang {
    Python,
    Rust,
    Go,
    Js,
    Ts,
    Java,
    C,
    Cpp,
    Other,
}

impl Lang {
    fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "python" | "py" => Some(Self::Python),
            "rust" | "rs" => Some(Self::Rust),
            "go" | "golang" => Some(Self::Go),
            "js" | "javascript" | "node" => Some(Self::Js),
            "ts" | "typescript" => Some(Self::Ts),
            "java" => Some(Self::Java),
            "c" => Some(Self::C),
            "c++" | "cpp" | "cxx" => Some(Self::Cpp),
            "other" | "none" => Some(Self::Other),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::Rust => "rust",
            Self::Go => "go",
            Self::Js => "js",
            Self::Ts => "ts",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "c++",
            Self::Other => "other",
        }
    }

    /// Command `ran` should run for this language.
    fn standalone(self) -> &'static str {
        match self {
            Self::Python => "python src/main.py",
            Self::Rust => "cargo run",
            Self::Go => "go run .",
            Self::Js => "node src/index.js",
            Self::Ts => "npx tsx src/index.ts",
            Self::Java => "javac -d target src/Main.java && java -cp target Main",
            Self::C | Self::Cpp => "make run",
            Self::Other => "echo 'Hello from ran!'",
        }
    }

    /// Suggested `ntpkg install` command when the toolchain is missing.
    fn tool_hint(self) -> Option<&'static str> {
        match self {
            Self::Python => Some("ntpkg install python"),
            Self::Rust => Some("ntpkg install rust"),
            Self::Go => Some("ntpkg install go"),
            Self::Js | Self::Ts => Some("ntpkg install node"),
            Self::Java => Some("ntpkg install java"),
            Self::C | Self::Cpp | Self::Other => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum License {
    Mit,
    Apache,
}

impl License {
    fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "mit" => Some(Self::Mit),
            "apache" | "apache-2.0" | "apache2" | "asl" => Some(Self::Apache),
            _ => None,
        }
    }
}

/// Scaffold groups that can be disabled with `--no`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Skip {
    docker: bool,
    git: bool,
    ai: bool,
    doc: bool,
    ntc: bool,
    oc: bool,
    lc: bool,
}

impl Skip {
    fn is_known(raw: &str) -> bool {
        matches!(
            raw.to_ascii_lowercase().as_str(),
            "docker" | "git" | "ai" | "doc" | "ntc" | "oc" | "lc"
        )
    }

    /// Apply one category. `ai` also disables the OpenCode scaffold.
    fn apply(&mut self, raw: &str) -> bool {
        match raw.to_ascii_lowercase().as_str() {
            "docker" => self.docker = true,
            "git" => self.git = true,
            "ai" => {
                self.ai = true;
                self.oc = true;
            }
            "doc" => self.doc = true,
            "ntc" => self.ntc = true,
            "oc" => self.oc = true,
            "lc" => self.lc = true,
            _ => return false,
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    Usage,
    LangUnknown,
    LicenseUnknown,
    NoUnknown(String),
}

impl ParseError {
    fn key(&self) -> &'static str {
        match self {
            Self::Usage => "cmd.config.init.project.usage",
            Self::LangUnknown => "cmd.config.init.project.lang_unknown",
            Self::LicenseUnknown => "cmd.config.init.project.license_unknown",
            Self::NoUnknown(_) => "cmd.config.init.project.no_unknown",
        }
    }

    fn arg(&self) -> Option<&str> {
        match self {
            Self::NoUnknown(value) => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Options {
    name: Option<String>,
    lang: Option<Lang>,
    license: License,
    /// Scaffold groups disabled with `--no`.
    skip: Skip,
    /// Run external tools (`git`, `cargo`, `npm`, ...). Tests turn this off.
    run_tools: bool,
}

fn default_options(name: Option<String>) -> Options {
    Options {
        name,
        lang: None,
        license: License::Mit,
        skip: Skip::default(),
        run_tools: true,
    }
}

fn parse_args(args: &str) -> std::result::Result<Options, ParseError> {
    let mut opts = default_options(None);
    let mut iter = args.split_whitespace().peekable();
    while let Some(token) = iter.next() {
        match token {
            // Accepted for symmetry; `init project` never overwrites anyway.
            "-f" | "--force" => {}
            "--lang" | "-l" => {
                let value = iter.next().ok_or(ParseError::Usage)?;
                opts.lang = Some(Lang::parse(value).ok_or(ParseError::LangUnknown)?);
            }
            "--license" => {
                let value = iter.next().ok_or(ParseError::Usage)?;
                opts.license = License::parse(value).ok_or(ParseError::LicenseUnknown)?;
            }
            "--no" => parse_no_list(&mut iter, &mut opts.skip)?,
            t if t.starts_with('-') => return Err(ParseError::Usage),
            t if opts.name.is_none() => opts.name = Some(t.to_string()),
            _ => return Err(ParseError::Usage),
        }
    }
    Ok(opts)
}

/// Consume the value list after `--no`.
///
/// Accepts `lc,oc`, `lc, oc`, `lc , oc`, repeated flags, and space-separated
/// categories. The first token that is not a category stops the list and is
/// left for the caller to read as the project name.
fn parse_no_list(
    iter: &mut std::iter::Peekable<std::str::SplitWhitespace<'_>>,
    skip: &mut Skip,
) -> std::result::Result<(), ParseError> {
    let mut consumed = false;
    let mut expect_more = false;
    loop {
        let Some(token) = iter.peek().copied() else {
            if expect_more {
                return Err(ParseError::Usage);
            }
            break;
        };
        if token.starts_with('-') {
            if expect_more {
                return Err(ParseError::Usage);
            }
            break;
        }
        if token == "," {
            if !consumed {
                return Err(ParseError::Usage);
            }
            iter.next();
            expect_more = true;
            continue;
        }
        let parts: Vec<&str> = token.split(',').filter(|p| !p.is_empty()).collect();
        let known = !parts.is_empty() && parts.iter().all(|p| Skip::is_known(p));
        if !known {
            if consumed && !expect_more {
                break;
            }
            let bad = parts
                .iter()
                .copied()
                .find(|p| !Skip::is_known(p))
                .unwrap_or(token);
            return Err(ParseError::NoUnknown(bad.to_string()));
        }
        iter.next();
        for part in &parts {
            skip.apply(part);
        }
        consumed = true;
        expect_more = token.ends_with(',');
        let next_is_comma = iter.peek().copied() == Some(",");
        let next_is_known = iter
            .peek()
            .copied()
            .is_some_and(|t| !t.starts_with('-') && Skip::is_known(t));
        if !(expect_more || next_is_comma || next_is_known) {
            break;
        }
    }
    if consumed {
        Ok(())
    } else {
        Err(ParseError::Usage)
    }
}

pub(crate) fn cmd_init_project(args: &str, nav: &mut Navigator) -> Result<bool> {
    let opts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            let key = err.key();
            let message = match err.arg() {
                Some(arg) => t!(key, arg),
                None => t!(key),
            };
            print_error(&message);
            if key != "cmd.config.init.project.usage" {
                println!("{}", t!("cmd.config.init.project.usage"));
            }
            return Ok(false);
        }
    };

    let cwd = nav.current_path().to_path_buf();
    let target = match &opts.name {
        Some(name) => {
            if Path::new(name).is_absolute() || name.split(['/', '\\']).any(|p| p == "..") {
                print_error(&t!("cmd.config.init.project.invalid_name", name));
                return Ok(false);
            }
            cwd.join(name)
        }
        None => cwd.clone(),
    };

    if let Err(e) = fs::create_dir_all(&target) {
        print_error(&t!("cmd.config.init.create_failed", target.display(), e));
        return Ok(false);
    }
    if opts.name.is_some() {
        print_success(&t!("cmd.config.init.project.creating", target.display()));
    }

    let created = scaffold_project(&target, &opts);
    if git_enabled(&opts) {
        init_git(&target);
    }
    setup_language(&target, &opts);
    print_success(&t!(
        "cmd.config.init.project.done",
        target.display(),
        created
    ));

    if opts.name.is_some() {
        nav.go_to(&target)?;
    }
    Ok(false)
}

/// Write every scaffold file that is missing. Returns how many were created.
fn scaffold_project(dir: &Path, opts: &Options) -> usize {
    let name = project_name(dir, opts);
    let pkg = package_name(&name);
    let crate_name = pkg.replace('-', "_");
    let mut created = 0usize;

    for rel in ["assets/.gitkeep", "tests/.gitkeep"] {
        write_file(dir, rel, "", &mut created);
    }
    if !opts.skip.oc {
        for rel in [".opencode/skills/.gitkeep", ".opencode/plan/.gitkeep"] {
            write_file(dir, rel, "", &mut created);
        }
    }

    let fmt = parser::preferred_format();
    let fill = |text: &str| {
        text.replace("{{name}}", &name)
            .replace("{{pkg}}", &pkg)
            .replace("{{crate_name}}", &crate_name)
    };

    if !opts.skip.lc {
        write_file(
            dir,
            "LICENSE",
            &license_text(opts.license, opts.run_tools),
            &mut created,
        );
    }
    if !opts.skip.git {
        write_file(dir, "VERSION", "0.1.0\n", &mut created);
        write_file(dir, ".gitignore", GITIGNORE_TEMPLATE, &mut created);
        write_file(
            dir,
            ".github/workflows/ci.yml",
            &ci_workflow(opts.lang),
            &mut created,
        );
    }
    if !opts.skip.docker {
        write_file(
            dir,
            ".env",
            &format!("# Local secrets for {}. Gitignored.\n", name),
            &mut created,
        );
        write_file(
            dir,
            ".env.example",
            &fill(ENV_EXAMPLE_TEMPLATE),
            &mut created,
        );
        write_file(
            dir,
            "Dockerfile",
            &dockerfile_contents(opts.lang, &crate_name),
            &mut created,
        );
        write_file(dir, ".dockerignore", DOCKERIGNORE_TEMPLATE, &mut created);
    }
    if !opts.skip.doc {
        write_file(
            dir,
            "README.md",
            &readme_contents(&name, opts, fmt.ext()),
            &mut created,
        );
        write_file(dir, "CHANGELOG.md", &fill(CHANGELOG_TEMPLATE), &mut created);
        write_file(dir, "index.html", &fill(INDEX_TEMPLATE), &mut created);
        write_file(dir, "docs/.gitkeep", "", &mut created);
    }
    if !opts.skip.ai {
        write_file(dir, "CONTEXT.md", &fill(CONTEXT_TEMPLATE), &mut created);
        write_file(dir, "AGENTS.md", &fill(AGENTS_TEMPLATE), &mut created);
    }
    if !opts.skip.oc {
        write_file(dir, "opencode.jsonc", OPENCODE_TEMPLATE, &mut created);
    }

    if !opts.skip.ntc {
        if parser::resolve(dir).is_none() {
            if let Ok(body) = parser::template_with_standalone(
                fmt,
                opts.lang
                    .map(|lang| lang.standalone())
                    .unwrap_or("echo 'Hello from ran!'"),
            ) {
                write_file(dir, fmt.file_name(), &body, &mut created);
            }
        }

        let todo_fmt = Config::global_get_todo_format();
        let todo_name = format!(
            "future_todo.{}",
            crate::shell::commands::support::todo::todo_extension(&todo_fmt)
        );
        write_file(dir, &todo_name, "", &mut created);

        write_file(dir, "ntconfig.toml", LOCAL_CONFIG_TEMPLATE, &mut created);
    }

    if !opts.skip.oc {
        scaffold_opencode(dir, false);
    }
    created
}

/// Language-aware workflow written to `.github/workflows/ci.yml`.
fn ci_workflow(lang: Option<Lang>) -> String {
    let steps = match lang.unwrap_or(Lang::Other) {
        Lang::Python => {
            r#"      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install dependencies
        run: pip install -r requirements.txt
      - name: Compile
        run: python -m compileall -q src
"#
        }
        Lang::Rust => {
            r#"      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --verbose
      - name: Test
        run: cargo test --verbose
"#
        }
        Lang::Go => {
            r#"      - uses: actions/setup-go@v5
        with:
          go-version: "1.22"
      - name: Build
        run: go build ./...
      - name: Test
        run: go test ./...
"#
        }
        Lang::Js => {
            r#"      - uses: actions/setup-node@v4
        with:
          node-version: "22"
      - name: Install dependencies
        run: npm install
      - name: Check
        run: node --check src/index.js
"#
        }
        Lang::Ts => {
            r#"      - uses: actions/setup-node@v4
        with:
          node-version: "22"
      - name: Install dependencies
        run: npm install
      - name: Type-check
        run: npx tsc --noEmit
"#
        }
        Lang::Java => {
            r#"      - uses: actions/setup-java@v4
        with:
          distribution: temurin
          java-version: "21"
      - name: Build
        run: mvn -B -q package
"#
        }
        Lang::C | Lang::Cpp => {
            r#"      - name: Build
        run: make
"#
        }
        Lang::Other => {
            r#"      - name: Build / test
        run: echo "Add your build and test commands here"
"#
        }
    };

    format!(
        "name: CI\n\non: [push, pull_request]\n\npermissions:\n  contents: read\n\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n{steps}"
    )
}

/// README with only the sections that actually exist in this scaffold.
fn readme_contents(name: &str, opts: &Options, ran_ext: &str) -> String {
    let getting_started = if opts.skip.ntc {
        "## Getting started\n\nAdd your source files under `src/` and your tests under `tests/`.\n"
    } else {
        "## Getting started\n\n```bash\nntc ran        # run the standalone task\nntc ran list   # list every task\nntc todo       # show open tasks\n```\n"
    };

    let mut layout =
        String::from("- `src/` — source code\n- `tests/` — tests\n- `assets/` — static assets\n");
    if !opts.skip.oc {
        layout.push_str("- `.opencode/` — agent, tool and skill configuration\n");
    }
    if !opts.skip.ntc {
        layout.push_str(&format!("- `NTCRANFILE.{}` — task runner\n", ran_ext));
        layout.push_str("- `ntconfig.toml` — project-local ntc settings\n");
    }
    if !opts.skip.git {
        layout.push_str("- `.github/workflows/ci.yml` — CI workflow\n");
    }
    if !opts.skip.doc {
        layout.push_str("- `docs/` — documentation\n");
    }
    if !opts.skip.docker {
        layout.push_str("- `Dockerfile` — container image\n");
    }

    let agent_note = if opts.skip.ai {
        ""
    } else {
        "See `CONTEXT.md` for the project brief and `AGENTS.md` for agent rules.\n"
    };

    README_TEMPLATE
        .replace("{{name}}", name)
        .replace("{{getting_started}}", getting_started)
        .replace("{{layout}}", &layout)
        .replace("{{agent_note}}", agent_note)
}

/// Starter Dockerfile for the scaffolded project. `lang` picks the base image
/// and run command; without one it is a minimal image to adapt by hand.
fn dockerfile_contents(lang: Option<Lang>, crate_name: &str) -> String {
    match lang.unwrap_or(Lang::Other) {
        Lang::Python => r#"FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
CMD ["python", "src/main.py"]
"#
        .to_string(),
        Lang::Rust => format!(
            r#"FROM rust:1.82-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/{crate_name} /usr/local/bin/app
CMD ["app"]
"#
        ),
        Lang::Go => r#"FROM golang:1.22-alpine AS build
WORKDIR /app
COPY . .
RUN go build -o /app/app .

FROM alpine:3.20
WORKDIR /app
COPY --from=build /app/app /usr/local/bin/app
CMD ["app"]
"#
        .to_string(),
        Lang::Js => r#"FROM node:22-alpine
WORKDIR /app
COPY package.json ./
RUN npm install --omit=dev
COPY . .
CMD ["node", "src/index.js"]
"#
        .to_string(),
        Lang::Ts => r#"FROM node:22-alpine AS build
WORKDIR /app
COPY package.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
COPY package.json ./
RUN npm install --omit=dev
COPY --from=build /app/dist ./dist
CMD ["node", "dist/index.js"]
"#
        .to_string(),
        Lang::Java => r#"FROM eclipse-temurin:21-jdk AS build
WORKDIR /app
COPY . .
RUN javac -d target src/Main.java

FROM eclipse-temurin:21-jre
WORKDIR /app
COPY --from=build /app/target ./target
CMD ["java", "-cp", "target", "Main"]
"#
        .to_string(),
        Lang::C => r#"FROM gcc:14 AS build
WORKDIR /app
COPY . .
RUN make

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/app ./app
CMD ["./app"]
"#
        .to_string(),
        Lang::Cpp => r#"FROM gcc:14 AS build
WORKDIR /app
COPY . .
RUN make

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/app ./app
CMD ["./app"]
"#
        .to_string(),
        Lang::Other => r#"# ntc project image — replace the base image and command
# with the runtime your project needs.
FROM debian:bookworm-slim
WORKDIR /app
COPY . .
CMD ["bash"]
"#
        .to_string(),
    }
}

fn setup_language(dir: &Path, opts: &Options) {
    let Some(lang) = opts.lang else {
        return;
    };
    let name = project_name(dir, opts);
    let pkg = package_name(&name);
    let mut created = 0usize;
    let fill = |text: &str| text.replace("{{name}}", &name).replace("{{pkg}}", &pkg);

    match lang {
        Lang::Python => {
            if opts.run_tools && std::io::stdin().is_terminal() {
                let wants_venv = crate::utils::interactive::prompt_yes_no(
                    &t!("cmd.config.init.project.venv_prompt"),
                    true,
                )
                .unwrap_or(false);
                if wants_venv {
                    create_python_venv(dir);
                }
            }
            write_file(dir, "requirements.txt", PY_REQUIREMENTS, &mut created);
            write_file(dir, "src/main.py", &fill(PY_MAIN), &mut created);
        }
        Lang::Rust => {
            let initialized = opts.run_tools
                && !dir.join("Cargo.toml").exists()
                && tool_exists("cargo")
                && run_tool(dir, "cargo", &["init", "--name", &pkg]);
            if !initialized {
                if opts.run_tools && !tool_exists("cargo") {
                    warn_missing_tool(lang);
                }
                write_file(dir, "Cargo.toml", &fill(RUST_CARGO), &mut created);
                write_file(dir, "src/main.rs", &fill(RUST_MAIN), &mut created);
            }
        }
        Lang::Go => {
            if opts.run_tools && !dir.join("go.mod").exists() {
                if tool_exists("go") {
                    run_tool(dir, "go", &["mod", "init", &pkg]);
                } else {
                    warn_missing_tool(lang);
                }
            }
            write_file(dir, "main.go", &fill(GO_MAIN), &mut created);
            if !dir.join("go.mod").exists() {
                write_file(
                    dir,
                    "go.mod",
                    &format!("module {}\n\ngo 1.22\n", pkg),
                    &mut created,
                );
            }
        }
        Lang::Js => {
            let initialized = opts.run_tools
                && !dir.join("package.json").exists()
                && tool_exists(NPM)
                && run_tool(dir, NPM, &["init", "-y"]);
            if !initialized {
                if opts.run_tools && !tool_exists(NPM) {
                    warn_missing_tool(lang);
                }
                write_file(dir, "package.json", &fill(JS_PACKAGE), &mut created);
            }
            write_file(dir, "src/index.js", &fill(JS_MAIN), &mut created);
        }
        Lang::Ts => {
            if !dir.join("package.json").exists() {
                write_file(dir, "package.json", &fill(TS_PACKAGE), &mut created);
            }
            write_file(dir, "tsconfig.json", TS_CONFIG, &mut created);
            write_file(dir, "src/index.ts", &fill(TS_MAIN), &mut created);
        }
        Lang::Java => {
            write_file(dir, "pom.xml", &fill(JAVA_POM), &mut created);
            write_file(dir, "src/Main.java", &fill(JAVA_MAIN), &mut created);
        }
        Lang::C => {
            write_file(dir, "Makefile", C_MAKEFILE, &mut created);
            write_file(dir, "src/main.c", &fill(C_MAIN), &mut created);
        }
        Lang::Cpp => {
            write_file(dir, "Makefile", CPP_MAKEFILE, &mut created);
            write_file(dir, "src/main.cpp", &fill(CPP_MAIN), &mut created);
        }
        Lang::Other => {}
    }

    print_success(&t!("cmd.config.init.project.lang_ready", lang.name()));
}

fn create_python_venv(dir: &Path) {
    let Some(python) = ["python", "python3"]
        .into_iter()
        .find(|bin| tool_exists(bin))
    else {
        print_warning(&t!(
            "cmd.config.init.project.tool_missing",
            "python",
            "ntpkg install python"
        ));
        return;
    };
    if run_tool(dir, python, &["-m", "venv", ".venv"]) {
        print_success(&t!(
            "cmd.config.init.project.venv_created",
            dir.join(".venv").display()
        ));
    } else {
        print_warning(&t!("cmd.config.init.project.venv_failed"));
    }
}

/// Whether `init project` should run `git init`.
fn git_enabled(opts: &Options) -> bool {
    opts.run_tools && !opts.skip.git
}

fn init_git(dir: &Path) {
    if dir.join(".git").exists() {
        println!("{}", t!("cmd.config.init.project.git_exists"));
        return;
    }
    if !tool_exists("git") {
        print_warning(&t!("cmd.config.init.project.git_missing"));
        return;
    }
    if run_tool(dir, "git", &["init", "-q"]) {
        print_success(&t!("cmd.config.init.project.git_created"));
    } else {
        print_warning(&t!("cmd.config.init.project.git_missing"));
    }
}

fn run_tool(dir: &Path, program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn tool_exists(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn warn_missing_tool(lang: Lang) {
    if let Some(hint) = lang.tool_hint() {
        print_warning(&t!(
            "cmd.config.init.project.tool_missing",
            lang.name(),
            hint
        ));
    }
}

fn write_file(dir: &Path, rel: &str, contents: &str, created: &mut usize) {
    let path = dir.join(rel);
    if path.exists() {
        return;
    }
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            print_error(&t!("cmd.config.init.create_failed", parent.display(), e));
            return;
        }
    }
    match fs::write(&path, contents) {
        Ok(_) => *created += 1,
        Err(e) => print_error(&t!("cmd.config.init.create_failed", path.display(), e)),
    }
}

fn license_text(license: License, run_tools: bool) -> String {
    let template = match license {
        License::Mit => LICENSE_MIT,
        License::Apache => LICENSE_APACHE,
    };
    template.replace("{{copyright}}", &copyright_line(run_tools))
}

fn copyright_line(run_tools: bool) -> String {
    let year = chrono::Local::now().year();
    let author = if run_tools {
        git_user_name().unwrap_or_default()
    } else {
        String::new()
    };
    if author.is_empty() {
        format!("Copyright (c) {}", year)
    } else {
        format!("Copyright (c) {} {}", year, author)
    }
}

fn git_user_name() -> Option<String> {
    let output = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn project_name(dir: &Path, opts: &Options) -> String {
    opts.name
        .as_deref()
        .and_then(|name| Path::new(name).file_name())
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .filter(|name| !name.is_empty())
        .or_else(|| {
            dir.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_string())
        })
        .unwrap_or_else(|| "project".to_string())
}

/// Lowercase, dash-separated identifier suitable for package/module names.
fn package_name(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "project".to_string()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::commands::builtin::ran::ranfile::types::RanFormat;

    fn options(lang: Option<Lang>) -> Options {
        Options {
            name: Some("demo".to_string()),
            lang,
            license: License::Mit,
            skip: Skip::default(),
            run_tools: false,
        }
    }

    /// Scaffold into a fresh temp dir with the supplied skip set.
    fn scaffold_skipping(skip: Skip) -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();
        let mut opts = options(None);
        opts.skip = skip;
        scaffold_project(&dir, &opts);
        (tmp, dir)
    }

    #[test]
    fn scaffold_writes_the_full_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();

        let created = scaffold_project(&dir, &options(None));
        assert!(created >= 15, "only {} files", created);

        for rel in [
            "LICENSE",
            "VERSION",
            "README.md",
            "CHANGELOG.md",
            "CONTEXT.md",
            "AGENTS.md",
            "index.html",
            ".gitignore",
            ".dockerignore",
            ".github/workflows/ci.yml",
            "Dockerfile",
            ".env",
            ".env.example",
            "ntconfig.toml",
            "opencode.jsonc",
            "assets",
            "tests",
            "docs/.gitkeep",
            ".opencode/agents/ntc-impl.md",
            ".opencode/agents/ntc-plan.md",
            ".opencode/tools/ntc.ts",
            ".opencode/opencode.jsonc",
            ".opencode/skills",
            ".opencode/plan",
        ] {
            assert!(dir.join(rel).exists(), "missing {}", rel);
        }

        let fmt = parser::preferred_format();
        assert!(dir.join(fmt.file_name()).is_file());
        let todo = format!(
            "future_todo.{}",
            crate::shell::commands::support::todo::todo_extension(&Config::global_get_todo_format())
        );
        assert!(dir.join(todo).is_file());

        let gitignore = fs::read_to_string(dir.join(".gitignore")).unwrap();
        assert!(gitignore.contains("ntc_smtp.json"), "got:\n{}", gitignore);
        assert!(fs::read_to_string(dir.join("LICENSE"))
            .unwrap()
            .contains("MIT License"));
    }

    #[test]
    fn scaffold_never_overwrites_existing_files() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("README.md"), "keep me").unwrap();
        fs::write(dir.join("VERSION"), "9.9.9").unwrap();

        scaffold_project(&dir, &options(None));

        assert_eq!(
            fs::read_to_string(dir.join("README.md")).unwrap(),
            "keep me"
        );
        assert_eq!(fs::read_to_string(dir.join("VERSION")).unwrap(), "9.9.9");
    }

    #[test]
    fn dockerfile_and_dockerignore_are_scaffolded() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();

        scaffold_project(&dir, &options(Some(Lang::Python)));

        let dockerfile = fs::read_to_string(dir.join("Dockerfile")).unwrap();
        assert!(dockerfile.contains("FROM python:"), "got:\n{}", dockerfile);
        assert!(dockerfile.contains("src/main.py"), "got:\n{}", dockerfile);
        assert!(dir.join(".dockerignore").is_file());

        let rust = dockerfile_contents(Some(Lang::Rust), "my_demo");
        assert!(rust.contains("/target/release/my_demo"), "got:\n{}", rust);

        let generic = dockerfile_contents(None, "demo");
        assert!(generic.contains("FROM debian:"), "got:\n{}", generic);
    }

    #[test]
    fn apache_license_option() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();
        let mut opts = options(None);
        opts.license = License::Apache;

        scaffold_project(&dir, &opts);

        let text = fs::read_to_string(dir.join("LICENSE")).unwrap();
        assert!(text.contains("Apache License"), "got:\n{}", text);
    }

    #[test]
    fn python_scaffold_writes_runtime_files() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();

        scaffold_project(&dir, &options(Some(Lang::Python)));
        setup_language(&dir, &options(Some(Lang::Python)));

        assert!(dir.join("requirements.txt").is_file());
        assert!(dir.join("src/main.py").is_file());
        let ranfile = fs::read_to_string(dir.join(parser::preferred_format().file_name())).unwrap();
        assert!(ranfile.contains("python src/main.py"), "got:\n{}", ranfile);
    }

    #[test]
    fn rust_scaffold_falls_back_without_cargo() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("demo");
        fs::create_dir_all(&dir).unwrap();

        scaffold_project(&dir, &options(Some(Lang::Rust)));
        setup_language(&dir, &options(Some(Lang::Rust)));

        assert!(dir.join("Cargo.toml").is_file());
        assert!(dir.join("src/main.rs").is_file());
        let ranfile = fs::read_to_string(dir.join(parser::preferred_format().file_name())).unwrap();
        assert!(ranfile.contains("cargo run"), "got:\n{}", ranfile);
    }

    #[test]
    fn parse_project_args() {
        let opts = parse_args("demo --lang python --license apache").unwrap();
        assert_eq!(opts.name.as_deref(), Some("demo"));
        assert_eq!(opts.lang, Some(Lang::Python));
        assert_eq!(opts.license, License::Apache);

        let opts = parse_args("--lang c++").unwrap();
        assert_eq!(opts.lang, Some(Lang::Cpp));
        assert!(opts.name.is_none());

        assert_eq!(parse_args("demo extra").unwrap_err(), ParseError::Usage);
        assert_eq!(
            parse_args("demo --lang brainfuck").unwrap_err(),
            ParseError::LangUnknown
        );
        assert_eq!(
            parse_args("demo --license WTFPL").unwrap_err(),
            ParseError::LicenseUnknown
        );
    }

    #[test]
    fn parse_no_flag_variants() {
        let opts = parse_args("demo --no lc,oc").unwrap();
        assert!(opts.skip.lc && opts.skip.oc);
        assert!(!opts.skip.git);

        // Spaces around commas are ignored.
        let opts = parse_args("--no lc , oc demo").unwrap();
        assert_eq!(opts.name.as_deref(), Some("demo"));
        assert!(opts.skip.lc && opts.skip.oc);

        // The flag is repeatable and cumulative.
        let opts = parse_args("demo --no lc --no git").unwrap();
        assert!(opts.skip.lc && opts.skip.git);

        // Space-separated categories work too.
        let opts = parse_args("demo --no lc oc").unwrap();
        assert!(opts.skip.lc && opts.skip.oc);

        // `ai` implies the OpenCode scaffold.
        let opts = parse_args("--no ai demo").unwrap();
        assert!(opts.skip.ai && opts.skip.oc);
        assert_eq!(opts.name.as_deref(), Some("demo"));

        assert_eq!(parse_args("demo --no").unwrap_err(), ParseError::Usage);
        assert_eq!(
            parse_args("demo --no wat").unwrap_err(),
            ParseError::NoUnknown("wat".to_string())
        );
        assert_eq!(
            parse_args("demo --no lc,wat").unwrap_err(),
            ParseError::NoUnknown("wat".to_string())
        );
    }

    #[test]
    fn no_docker_skips_container_files() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            docker: true,
            ..Skip::default()
        });
        for rel in ["Dockerfile", ".dockerignore", ".env", ".env.example"] {
            assert!(!dir.join(rel).exists(), "unexpected {}", rel);
        }
        assert!(dir.join("README.md").is_file());
        assert!(dir.join("LICENSE").is_file());
    }

    #[test]
    fn no_git_skips_repo_files() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            git: true,
            ..Skip::default()
        });
        for rel in [".gitignore", "VERSION", ".github/workflows/ci.yml"] {
            assert!(!dir.join(rel).exists(), "unexpected {}", rel);
        }
        assert!(!dir.join(".github").exists());
        assert!(dir.join("README.md").is_file());
    }

    #[test]
    fn no_ai_skips_agent_docs_and_opencode() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            ai: true,
            oc: true,
            ..Skip::default()
        });
        for rel in [
            "CONTEXT.md",
            "AGENTS.md",
            "opencode.jsonc",
            ".opencode/agents/ntc-plan.md",
        ] {
            assert!(!dir.join(rel).exists(), "unexpected {}", rel);
        }
        assert!(!dir.join(".opencode").exists());
        assert!(dir.join("README.md").is_file());
    }

    #[test]
    fn no_oc_keeps_agent_docs() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            oc: true,
            ..Skip::default()
        });
        assert!(!dir.join(".opencode").exists());
        assert!(!dir.join("opencode.jsonc").exists());
        assert!(dir.join("CONTEXT.md").is_file());
        assert!(dir.join("AGENTS.md").is_file());
    }

    #[test]
    fn no_doc_skips_documentation() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            doc: true,
            ..Skip::default()
        });
        for rel in ["README.md", "CHANGELOG.md", "index.html", "docs"] {
            assert!(!dir.join(rel).exists(), "unexpected {}", rel);
        }
        assert!(dir.join("LICENSE").is_file());
    }

    #[test]
    fn no_ntc_skips_ntc_files() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            ntc: true,
            ..Skip::default()
        });
        assert!(!dir.join(parser::preferred_format().file_name()).exists());
        let todo = format!(
            "future_todo.{}",
            crate::shell::commands::support::todo::todo_extension(&Config::global_get_todo_format())
        );
        assert!(!dir.join(todo).exists());
        assert!(!dir.join("ntconfig.toml").exists());
        assert!(dir.join("README.md").is_file());
    }

    #[test]
    fn no_lc_skips_license() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            lc: true,
            ..Skip::default()
        });
        assert!(!dir.join("LICENSE").exists());
        assert!(dir.join("README.md").is_file());
    }

    #[test]
    fn skipping_everything_leaves_only_common_dirs() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            docker: true,
            git: true,
            ai: true,
            doc: true,
            ntc: true,
            oc: true,
            lc: true,
        });
        let mut entries: Vec<String> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        entries.sort();
        assert_eq!(entries, vec!["assets", "tests"]);
    }

    #[test]
    fn readme_omits_skipped_sections() {
        let (_tmp, dir) = scaffold_skipping(Skip {
            git: true,
            ai: true,
            ntc: true,
            oc: true,
            ..Skip::default()
        });
        let readme = fs::read_to_string(dir.join("README.md")).unwrap();
        assert!(!readme.contains("ntc ran"), "got:\n{}", readme);
        assert!(!readme.contains("NTCRANFILE"), "got:\n{}", readme);
        assert!(!readme.contains(".opencode/"), "got:\n{}", readme);
        assert!(!readme.contains("CONTEXT.md"), "got:\n{}", readme);
        assert!(readme.contains("Add your source files"), "got:\n{}", readme);
    }

    #[test]
    fn ci_workflow_matches_language() {
        assert!(ci_workflow(Some(Lang::Python)).contains("setup-python"));
        assert!(ci_workflow(Some(Lang::Rust)).contains("cargo test"));
        assert!(ci_workflow(Some(Lang::Go)).contains("setup-go"));
        assert!(ci_workflow(Some(Lang::Java)).contains("setup-java"));
        assert!(ci_workflow(Some(Lang::C)).contains("make"));
        assert!(ci_workflow(None).contains("Add your build and test commands"));
    }

    #[test]
    fn git_is_skipped_by_flag_or_tests() {
        let mut opts = options(None);
        opts.run_tools = true;
        assert!(git_enabled(&opts));
        opts.skip.git = true;
        assert!(!git_enabled(&opts));
        opts.run_tools = false;
        opts.skip.git = false;
        assert!(!git_enabled(&opts));
    }

    #[test]
    fn package_names_are_sanitized() {
        assert_eq!(package_name("My Demo!"), "my-demo");
        assert_eq!(package_name("!!!"), "project");
    }

    #[test]
    fn standalone_template_replaces_echo() {
        let body = parser::template_with_standalone(RanFormat::Toml, "go run .").unwrap();
        assert!(body.contains("go run ."));
        assert!(!body.contains("Hello from ran!"));
    }
}
