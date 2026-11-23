//! Embedded OpenCode scaffold, shared by `init` and `init project`.

use crate::output::{print_error, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use std::io::{self, Write};
use std::path::Path;

/// Files written by `init opencode`, as `(path relative to .opencode/, contents)`.
const OPENCODE_TEMPLATES: &[(&str, &str)] = &[
    (
        "tools/ntc.ts",
        include_str!("../builtin/init/assets/opencode_template/ntc.ts"),
    ),
    (
        "agents/ntc-plan.md",
        include_str!("../builtin/init/assets/opencode_template/ntc-plan.md"),
    ),
    (
        "agents/ntc-impl.md",
        include_str!("../builtin/init/assets/opencode_template/ntc-impl.md"),
    ),
    (
        "opencode.jsonc",
        include_str!("../builtin/init/assets/opencode_template/opencode.jsonc"),
    ),
];

/// Prompt for destructive confirmation unless `force` is set. Returns `true`
/// when the caller may proceed.
pub(crate) fn confirm_delete(path: &Path, force: bool) -> Result<bool> {
    if force {
        return Ok(true);
    }
    print!(
        "{} {}",
        "[!]".red().bold(),
        t!("cmd.config.deinit.confirm", path.display()).yellow()
    );
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
}

/// Create `.opencode/` with `tools/`, `agents/` and the bundled templates.
/// Existing files are skipped unless `force` is set (which overwrites only the
/// template files, never the directory or any unrelated file).
pub(crate) fn scaffold_opencode(dir: &Path, force: bool) {
    let base = dir.join(".opencode");

    for (rel, _) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                print_error(&t!("cmd.config.init.create_failed", parent.display(), e));
            }
        }
    }

    for (rel, contents) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if path.exists() && !force {
            print_warning(&t!("cmd.config.init.exists_warn", path.display()));
            continue;
        }
        match std::fs::write(&path, contents) {
            Ok(_) => print_success(&t!("cmd.config.init.created", path.display())),
            Err(e) => print_error(&t!("cmd.config.init.create_failed", path.display(), e)),
        }
    }

    print_success(&t!("cmd.config.init.opencode_ready", base.display()));
}

/// Remove the bundled template files, then prune empty directories.
pub(crate) fn prune_opencode(base: &Path) {
    for (rel, _) in OPENCODE_TEMPLATES {
        let path = base.join(rel);
        if !path.exists() {
            continue;
        }
        match std::fs::remove_file(&path) {
            Ok(_) => print_success(&t!("cmd.config.deinit.removed", path.display())),
            Err(e) => print_error(&t!("cmd.config.deinit.remove_failed", path.display(), e)),
        }
    }

    // `remove_dir` only succeeds when the directory is empty, so user files
    // survive and keep their parent directory in place.
    for rel in ["tools", "agents", ""] {
        let dir = if rel.is_empty() {
            base.to_path_buf()
        } else {
            base.join(rel)
        };
        if dir.is_dir() {
            let _ = std::fs::remove_dir(&dir);
        }
    }
}
