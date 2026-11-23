pub(crate) mod completion;
pub(crate) mod help;
use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::shell::commands::support::remote_catalog;
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use crate::utils::theme::style::ThemeColorize;
use anyhow::{Context, Result};
use std::io::{self, Write};
use std::path::PathBuf;

/// Split `rm` arguments into theme names and a force flag.
fn parse_remove_args<'a>(args: &[&'a str]) -> (Vec<&'a str>, bool) {
    let mut force = false;
    let mut names = Vec::new();
    for arg in args {
        match *arg {
            "-f" | "--force" => force = true,
            other if other.starts_with('-') => {}
            other => names.push(other),
        }
    }
    (names, force)
}

/// Ask `[y/N]`; only `y`/`yes` (any case) confirms.
fn confirm_removal(names: &[&str]) -> bool {
    print!(
        "{} ",
        t!("theme.remove_confirm", names.len(), names.join(", "))
    );
    let _ = io::stdout().flush();
    let mut answer = String::new();
    if io::stdin().read_line(&mut answer).is_err() {
        return false;
    }
    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Remove one or more themes. More than one name asks for confirmation
/// unless `-f`/`--force` is given, and the active theme is never removable.
fn run_remove(args: &[&str]) {
    use crate::utils::theme::ThemeManager;

    let (names, force) = parse_remove_args(args);
    if names.is_empty() {
        print_error(&t!("theme.usage_rm"));
        return;
    }

    let current = ThemeManager::current().name.clone();
    if names.iter().any(|name| *name == current) {
        print_error(&t!("theme.remove_current", current));
        return;
    }
    if let Some(missing) = names.iter().find(|name| !ThemeManager::theme_exists(name)) {
        print_error(&t!("theme.not_found", missing));
        return;
    }
    if names.iter().any(|name| *name == "default") {
        print_error(&t!("theme.remove_builtin", "default"));
        return;
    }
    if names.len() > 1 && !force && !confirm_removal(&names) {
        print_info(&t!("theme.remove_cancelled"));
        return;
    }

    for name in names {
        match ThemeManager::remove_theme(name) {
            Ok(()) => print_success(&t!("theme.removed", name)),
            Err(e) => print_error(&t!("theme.remove_failed", e)),
        }
    }
}

/// Open the theme directory itself in ntceditor (welcome screen + sidebar).
fn open_theme_dir(dir: PathBuf) {
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    print_info(&t!("theme.opened_dir", dir.display()));
    super::ne::launch_ne_editor(dir, None);
}

fn install_theme_from_github() -> Result<()> {
    print_info(&t!("theme.install_fetching"));
    let entries = remote_catalog::list("ntc_theme", ".ntc_theme")?;
    if entries.is_empty() {
        print_info(&t!("theme.install_none"));
        return Ok(());
    }

    let items: Vec<MenuItem> = entries
        .iter()
        .map(|entry| MenuItem::with_detail(entry.name.clone(), entry.size_label()))
        .collect();
    let title = t!("theme.install_title");
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("theme.install_prompt", entries.len()));
    let Some(index) = menu.run()? else {
        return Ok(());
    };

    let entry = &entries[index];
    print_info(&t!("theme.install_downloading", &entry.name));
    let bytes = remote_catalog::download(&entry.url)?;
    let content = String::from_utf8(bytes).context("downloaded theme file is not UTF-8")?;
    let theme: crate::utils::theme::Theme =
        toml::from_str(&content).context("downloaded theme file is not a valid theme")?;
    if theme.name.trim().is_empty()
        || theme.name.contains('/')
        || theme.name.contains('\\')
        || theme.name.contains("..")
    {
        anyhow::bail!("downloaded theme has an invalid name: {}", theme.name);
    }

    let path = remote_catalog::temp_path("theme", "ntc_theme");
    std::fs::write(&path, &content).with_context(|| format!("write {}", path.display()))?;
    let installed = crate::utils::theme::ThemeManager::install_theme(&path)
        .map_err(|e| anyhow::anyhow!("{}", e));
    let _ = std::fs::remove_file(&path);
    installed?;

    if crate::utils::theme::ThemeManager::set_theme(&theme.name) {
        print_success(&t!("theme.installed_active", &theme.name));
    } else {
        print_success(&t!("theme.installed", &theme.name));
    }
    Ok(())
}

pub fn cmd_theme(args: &str, nav: &mut Navigator) -> Result<bool> {
    use crate::utils::theme::ThemeManager;

    if args.is_empty() {
        // Interactive menu
        loop {
            let current = ThemeManager::current();
            let themes = ThemeManager::list_themes();

            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║{:^65}", t!("theme.ui.title").cyan().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");
            println!(
                "║  {} {:48} ",
                format!("{}:", t!("theme.ui.current")).yellow(),
                current.name.green().bold()
            );
            println!("╠══════════════════════════════════════════════════════════════════╣");

            for (i, name) in themes.iter().enumerate() {
                let marker = if name == &current.name { "* " } else { "  " };
                println!(
                    "║  {}. {}{:<53} ",
                    (i + 1).to_string().yellow(),
                    name,
                    marker
                );
            }
            println!("╠══════════════════════════════════════════════════════════════════╣");
            println!("║  Commands:                                                       ║");
            println!("║    n <number>         - Switch to theme by number                ║");
            println!("║    add                - Add new theme (opens editor)             ║");
            println!("║    use <name>         - Switch to theme by name                  ║");
            println!("║    update <name>      - Add fields missing from a theme file     ║");
            println!("║    make [name]        - Create theme template in CWD             ║");
            println!("║    rm <name...> [-f]  - Remove themes (confirm when several)     ║");
            println!("║    edit [name]        - Edit theme (or open theme dir)           ║");
            println!("║    where              - Show theme directory location            ║");
            println!("║    info <name>        - Show theme info                          ║");
            println!("║    export <name>      - Export theme to file                     ║");
            println!("║    import <file>      - Import theme from file                   ║");
            println!("║    install            - Install a theme from GitHub              ║");
            println!("║    rnm <old> to <new> - Rename theme                             ║");
            println!("║    0                  - Exit                                     ║");
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!();
            print!("{} ", t!("theme.ui.theme_prompt").green().bold());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "0" || input.is_empty() {
                println!("{}", t!("theme.ui.exit").dimmed());
                break;
            }

            // Drop the read guard on current before set_theme, which needs a write lock
            drop(current);

            // Parse number
            if let Ok(num) = input.parse::<usize>() {
                if num >= 1 && num <= themes.len() {
                    let theme_name = &themes[num - 1];
                    ThemeManager::reload_themes();
                    if ThemeManager::set_theme(theme_name) {
                        print_success(&t!("theme.switched", theme_name));
                    } else {
                        print_error(&t!("theme.switch_failed", theme_name));
                    }
                } else {
                    print_error(&t!("theme.invalid_number", num, themes.len()));
                }
                continue;
            }

            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0].to_lowercase().as_str() {
                "a" | "add" => {
                    print!("{} ", t!("theme.ui.name_prompt").cyan());
                    io::stdout().flush()?;
                    let mut name = String::new();
                    io::stdin().read_line(&mut name)?;
                    let name = name.trim();

                    if name.is_empty() {
                        print_error(&t!("theme.name_empty"));
                    } else if let Err(e) = ThemeManager::create_theme(name) {
                        print_error(&t!("theme.create_failed", e));
                    } else {
                        print_success(&t!("theme.created", name));
                        print_info(&t!("theme.created_hint"));
                    }
                }
                "u" | "update" => {
                    if parts.len() < 2 {
                        print_error(&t!("theme.usage_update"));
                    } else {
                        match ThemeManager::update_theme(parts[1]) {
                            Ok(0) => print_success(&t!("theme.up_to_date", parts[1])),
                            Ok(count) => print_success(&t!("theme.updated", parts[1], count)),
                            Err(e) => print_error(&t!("theme.update_failed", e)),
                        }
                    }
                }
                "r" | "rm" | "remove" => run_remove(&parts[1..]),
                "e" | "edit" => {
                    if parts.len() < 2 {
                        // No name: open the theme directory in ntceditor.
                        if let Some(dir) = ThemeManager::themes_dir() {
                            open_theme_dir(dir);
                        } else {
                            print_error(&t!("theme.no_dir"));
                        }
                    } else {
                        let name = parts[1];
                        if ThemeManager::theme_exists(name) {
                            let themes_dir =
                                dirs::config_dir().map(|d| d.join("ntc").join("themes"));
                            if let Some(dir) = themes_dir {
                                let theme_path = dir.join(format!("{}.ntc_theme", name));
                                let _ = crate::shell::commands::builtin::ne::editor::edit_file(
                                    &theme_path,
                                );
                                print_info(&t!("theme.edit_hint", name));
                            }
                        } else {
                            print_error(&t!("theme.not_found", name));
                        }
                    }
                }
                "i" | "info" => {
                    if parts.len() < 2 {
                        print_error(&t!("theme.usage_info"));
                    } else {
                        let name = parts[1];
                        if let Some(theme) = ThemeManager::get_theme_info(name) {
                            println!();
                            println!(
                                "{}",
                                "════════════════════════════════════════════════════════════"
                                    .cyan()
                            );
                            println!("{}", t!("theme.info_title", theme.name).cyan().bold());
                            println!(
                                "{}",
                                "════════════════════════════════════════════════════════════"
                                    .cyan()
                            );
                            if let Some(author) = theme.author {
                                println!("  {}: {}", t!("theme.info_author").yellow(), author);
                            }
                            if let Some(desc) = theme.description {
                                println!("  {}: {}", t!("theme.info_description").yellow(), desc);
                            }
                            println!(
                                "  {}: {}",
                                t!("theme.info_syntax").yellow(),
                                t!("theme.info_defined").green()
                            );
                            println!(
                                "  {}: {}",
                                t!("theme.info_shell").yellow(),
                                t!("theme.info_defined").green()
                            );
                            println!(
                                "  {}: {}",
                                t!("theme.info_editor").yellow(),
                                t!("theme.info_defined").green()
                            );
                            println!();
                        } else {
                            print_error(&t!("theme.not_found", name));
                        }
                    }
                }
                "x" | "export" => {
                    if parts.len() < 2 {
                        print_error(&t!("theme.usage_export"));
                    } else {
                        let name = parts[1];
                        let output = if parts.len() >= 3 {
                            std::path::PathBuf::from(parts[2])
                        } else {
                            let current_dir = std::env::current_dir().unwrap_or_default();
                            current_dir.join(format!("{}.ntc_theme", name))
                        };

                        if let Err(e) = ThemeManager::export_theme(name, &output) {
                            print_error(&t!("theme.export_failed", e));
                        } else {
                            print_success(&t!("theme.exported_to", name, output.display()));
                        }
                    }
                }
                "install" => {
                    if let Err(e) = install_theme_from_github() {
                        print_error(&t!("theme.install_failed", e));
                    }
                }
                "m" | "import" => {
                    if parts.len() < 2 {
                        print_error(&t!("theme.usage_import"));
                    } else {
                        let path = std::path::Path::new(parts[1]);
                        if !path.exists() {
                            print_error(&t!("theme.file_not_found", path.display()));
                        } else if let Err(e) = ThemeManager::import_theme(path) {
                            print_error(&t!("theme.import_failed", e));
                        } else {
                            print_success(&t!("theme.imported", path.display()));
                            print_info(&t!("theme.import_hint"));
                        }
                    }
                }
                "rnm" | "rename" => {
                    if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                        print_error(&t!("theme.usage_rnm"));
                    } else {
                        let old_name = parts[1];
                        let new_name = parts[3];
                        if let Err(e) = ThemeManager::rename_theme(old_name, new_name) {
                            print_error(&t!("theme.rename_failed", e));
                        } else {
                            print_success(&t!("theme.renamed", old_name, new_name));
                        }
                    }
                }
                "reload" => {
                    ThemeManager::reload_themes();
                    print_success(&t!("theme.reloaded"));
                }
                "where" | "wh" => {
                    if let Some(dir) = ThemeManager::themes_dir() {
                        println!();
                        println!("{}", t!("theme.dir_title").cyan().bold());
                        println!("  {}", dir.display().to_string().green());
                        if dir.exists() {
                            println!();
                            println!("{}", t!("theme.files_title").cyan().bold());
                            if let Ok(entries) = std::fs::read_dir(&dir) {
                                let mut found = false;
                                for entry in entries.flatten() {
                                    let path = entry.path();
                                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                        if ext == "ntc_theme" || ext == "toml" {
                                            if let Some(name) =
                                                path.file_stem().and_then(|n| n.to_str())
                                            {
                                                println!("  - {}.{}", name.blue(), ext.dimmed());
                                                found = true;
                                            }
                                        }
                                    }
                                }
                                if !found {
                                    println!("  {}", t!("theme.no_files").dimmed());
                                }
                            }
                        } else {
                            println!();
                            println!("  {}", t!("theme.no_dir_yet").dimmed());
                            println!("  {}", t!("theme.create_first").dimmed());
                        }
                        println!();
                    } else {
                        print_error(&t!("theme.no_dir"));
                    }
                }
                "make" | "mk" => {
                    let name = if parts.len() >= 2 {
                        parts[1]
                    } else {
                        "example"
                    };
                    let path = std::env::current_dir()
                        .unwrap_or_default()
                        .join(format!("{}.ntc_theme", name));
                    if path.exists() {
                        print_error(&t!("theme.file_exists", path.display()));
                    } else {
                        let content = ThemeManager::get_example_theme_template();
                        match std::fs::write(&path, &content) {
                            Ok(()) => {
                                print_success(&t!("theme.template_created", path.display()));
                                print_info(&t!("theme.template_info"));
                            }
                            Err(e) => print_error(&t!("theme.create_file_failed", e)),
                        }
                    }
                }
                _ => {
                    print_error(&t!("theme.unknown_cmd", parts[0]));
                    println!("{}", t!("theme.type_interactive"));
                }
            }
        }
        Ok(false)
    } else {
        // Direct command mode (non-interactive)
        let parts: Vec<&str> = args.split_whitespace().collect();
        match parts[0].to_lowercase().as_str() {
            // Handled before the "unknown name switches theme" fallback.
            "check" => {
                let cwd = nav.current_path().to_path_buf();
                let target = parts.get(1).copied().unwrap_or("");
                super::super::support::check::run(
                    "theme",
                    target,
                    &cwd,
                    ".ntc_theme",
                    super::super::support::check::formats::theme,
                );
            }
            "list" | "ls" => {
                let current = ThemeManager::current();
                for name in ThemeManager::list_themes() {
                    if name == current.name {
                        println!("  {}", name.green());
                    } else {
                        println!("    {}", name);
                    }
                }
            }
            "current" => {
                let current = ThemeManager::current();
                println!("{}", t!("theme.current_title", current.name.green().bold()));
                if let Some(author) = &current.author {
                    println!("{}: {}", t!("theme.info_author"), author);
                }
                if let Some(desc) = &current.description {
                    println!("{}: {}", t!("theme.info_description"), desc);
                }
            }
            "reload" => {
                ThemeManager::reload_themes();
                print_success(&t!("theme.reloaded"));
            }
            "update" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_update"));
                } else {
                    match ThemeManager::update_theme(parts[1]) {
                        Ok(0) => print_success(&t!("theme.up_to_date", parts[1])),
                        Ok(count) => print_success(&t!("theme.updated", parts[1], count)),
                        Err(e) => print_error(&t!("theme.update_failed", e)),
                    }
                }
            }
            "add" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_add"));
                } else if let Err(e) = ThemeManager::create_theme(parts[1]) {
                    print_error(&t!("theme.create_failed", e));
                } else {
                    print_success(&t!("theme.created", parts[1]));
                }
            }
            "use" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_use"));
                } else if {
                    ThemeManager::reload_themes();
                    ThemeManager::set_theme(parts[1])
                } {
                    print_success(&t!("theme.switched", parts[1]));
                } else {
                    print_error(&t!("theme.not_found_hint", parts[1]));
                }
            }
            "rm" | "remove" => run_remove(&parts[1..]),
            "edit" => {
                if parts.len() < 2 {
                    // No name: open the theme directory in ntceditor.
                    if let Some(dir) = ThemeManager::themes_dir() {
                        open_theme_dir(dir);
                    } else {
                        print_error(&t!("theme.no_dir"));
                    }
                } else if ThemeManager::theme_exists(parts[1]) {
                    let themes_dir = dirs::config_dir().map(|d| d.join("ntc").join("themes"));
                    if let Some(dir) = themes_dir {
                        let theme_path = dir.join(format!("{}.ntc_theme", parts[1]));
                        let _ = crate::shell::commands::builtin::ne::editor::edit_file(&theme_path);
                    }
                } else {
                    print_error(&t!("theme.not_found", parts[1]));
                }
            }
            "info" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_info"));
                } else if let Some(theme) = ThemeManager::get_theme_info(parts[1]) {
                    println!("{}", t!("theme.info_title", theme.name).cyan().bold());
                    if let Some(author) = theme.author {
                        println!("{}: {}", t!("theme.info_author"), author);
                    }
                    if let Some(desc) = theme.description {
                        println!("{}: {}", t!("theme.info_description"), desc);
                    }
                } else {
                    print_error(&t!("theme.not_found", parts[1]));
                }
            }
            "export" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_export"));
                } else {
                    let name = parts[1];
                    let output = if parts.len() >= 3 {
                        std::path::PathBuf::from(parts[2])
                    } else {
                        std::env::current_dir()
                            .unwrap_or_default()
                            .join(format!("{}.ntc_theme", name))
                    };
                    if let Err(e) = ThemeManager::export_theme(name, &output) {
                        print_error(&t!("theme.export_failed_simple", e));
                    } else {
                        print_success(&t!("theme.exported_simple", output.display()));
                    }
                }
            }
            "install" => {
                if let Err(e) = install_theme_from_github() {
                    print_error(&t!("theme.install_failed", e));
                }
            }
            "import" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_import"));
                } else {
                    let path = std::path::Path::new(parts[1]);
                    if !path.exists() {
                        print_error(&t!("theme.file_not_found", path.display()));
                    } else if let Err(e) = ThemeManager::import_theme(path) {
                        print_error(&t!("theme.import_failed_simple", e));
                    } else {
                        print_success(&t!("theme.imported", path.display()));
                    }
                }
            }
            "rename" | "rnm" => {
                if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                    print_error(&t!("theme.usage_rename"));
                } else if let Err(e) = ThemeManager::rename_theme(parts[1], parts[3]) {
                    print_error(&t!("theme.rename_failed_simple", e));
                } else {
                    print_success(&t!("theme.renamed_simple", parts[1], parts[3]));
                }
            }
            "make" | "mk" => {
                let name = if parts.len() >= 2 {
                    parts[1]
                } else {
                    "example"
                };
                let path = std::env::current_dir()
                    .unwrap_or_default()
                    .join(format!("{}.ntc_theme", name));
                if path.exists() {
                    print_error(&t!("theme.file_exists", path.display()));
                } else {
                    let content = ThemeManager::get_example_theme_template();
                    match std::fs::write(&path, &content) {
                        Ok(()) => {
                            print_success(&t!("theme.template_created", path.display()));
                            print_info(&t!("theme.template_info"));
                        }
                        Err(e) => print_error(&t!("theme.create_file_failed", e)),
                    }
                }
            }
            "where" | "wh" => {
                if let Some(dir) = ThemeManager::themes_dir() {
                    println!();
                    println!("{}", t!("theme.dir_title").cyan().bold());
                    println!("  {}", dir.display().to_string().green());
                    if dir.exists() {
                        println!();
                        println!("{}", t!("theme.files_title").cyan().bold());
                        if let Ok(entries) = std::fs::read_dir(&dir) {
                            let mut found = false;
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                    if ext == "ntc_theme" || ext == "toml" {
                                        if let Some(name) =
                                            path.file_stem().and_then(|n| n.to_str())
                                        {
                                            println!("  - {}.{}", name.blue(), ext.dimmed());
                                            found = true;
                                        }
                                    }
                                }
                            }
                            if !found {
                                println!("  {}", t!("theme.no_files").dimmed());
                            }
                        }
                    } else {
                        println!();
                        println!("  {}", t!("theme.no_dir_yet").dimmed());
                        println!("  {}", t!("theme.create_first").dimmed());
                    }
                    println!();
                } else {
                    print_error(&t!("theme.no_dir"));
                }
            }
            _ => {
                // Try to switch to theme by name, or by index if it's a number
                let themes = ThemeManager::list_themes();
                if let Ok(num) = parts[0].parse::<usize>() {
                    if num >= 1 && num <= themes.len() {
                        let theme_name = &themes[num - 1];
                        if ThemeManager::set_theme(theme_name) {
                            print_success(&t!("theme.switched", theme_name));
                        } else {
                            print_error(&t!("theme.switch_failed_simple", theme_name));
                        }
                    } else {
                        print_error(&t!("theme.invalid_number", num, themes.len()));
                    }
                } else if ThemeManager::set_theme(parts[0]) {
                    print_success(&t!("theme.switched", parts[0]));
                } else {
                    print_error(&t!("theme.not_found_hint", parts[0]));
                }
            }
        }
        Ok(false)
    }
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "theme",
    aliases: &[],
    help_topic: Some("theme"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: true,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_theme),
};
