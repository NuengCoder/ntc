use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub fn cmd_theme(args: &str, _nav: &mut Navigator) -> Result<bool> {
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
                let marker = if name == &current.name { " ✓" } else { "  " };
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
            println!("║    make [name]        - Create theme template in CWD             ║");
            println!("║    rm <name>          - Remove theme                             ║");
            println!("║    edit [name]        - Edit theme (or open theme dir)           ║");
            println!("║    where              - Show theme directory location            ║");
            println!("║    info <name>        - Show theme info                          ║");
            println!("║    export <name>      - Export theme to file                     ║");
            println!("║    import <file>      - Import theme from file                   ║");
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
                "r" | "rm" | "remove" => {
                    if parts.len() < 2 {
                        print_error(&t!("theme.usage_rm"));
                    } else {
                        let name = parts[1];
                        if let Err(e) = ThemeManager::remove_theme(name) {
                            print_error(&t!("theme.remove_failed", e));
                        } else {
                            print_success(&t!("theme.removed", name));
                        }
                    }
                }
                "e" | "edit" => {
                    if parts.len() < 2 {
                        // No name: open theme directory
                        if let Some(dir) = ThemeManager::themes_dir() {
                            if !dir.exists() {
                                let _ = std::fs::create_dir_all(&dir);
                            }
                            #[cfg(windows)]
                            {
                                let _ = std::process::Command::new("cmd")
                                    .args(["/C", "start", "", &dir.to_string_lossy()])
                                    .status();
                            }
                            #[cfg(target_os = "macos")]
                            {
                                let _ = std::process::Command::new("open").arg(&dir).status();
                            }
                            #[cfg(target_os = "linux")]
                            {
                                let _ = std::process::Command::new("xdg-open").arg(&dir).status();
                            }
                            #[cfg(target_os = "android")]
                            {
                                let _ =
                                    std::process::Command::new("termux-open").arg(&dir).status();
                            }
                            print_info(&t!("theme.opened_dir", dir.display()));
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
                                let _ = crate::editor::edit_file(&theme_path);
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
            "list" | "ls" => {
                let current = ThemeManager::current();
                for name in ThemeManager::list_themes() {
                    if name == current.name {
                        println!("  ✓ {}", name.green());
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
            "add" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_add"));
                } else if let Err(e) = ThemeManager::create_theme(parts[1]) {
                    print_error(&t!("theme.create_failed", e));
                } else {
                    print_success(&t!("theme.created", parts[1]));
                }
            }
            "rm" => {
                if parts.len() < 2 {
                    print_error(&t!("theme.usage_rm"));
                } else if let Err(e) = ThemeManager::remove_theme(parts[1]) {
                    print_error(&t!("theme.remove_failed", e));
                } else {
                    print_success(&t!("theme.removed", parts[1]));
                }
            }
            "edit" => {
                if parts.len() < 2 {
                    // No name: open theme directory
                    if let Some(dir) = ThemeManager::themes_dir() {
                        if !dir.exists() {
                            let _ = std::fs::create_dir_all(&dir);
                        }
                        #[cfg(windows)]
                        {
                            let _ = std::process::Command::new("cmd")
                                .args(["/C", "start", "", &dir.to_string_lossy()])
                                .status();
                        }
                        #[cfg(target_os = "macos")]
                        {
                            let _ = std::process::Command::new("open").arg(&dir).status();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            let _ = std::process::Command::new("xdg-open").arg(&dir).status();
                        }
                        #[cfg(target_os = "android")]
                        {
                            let _ = std::process::Command::new("termux-open").arg(&dir).status();
                        }
                        print_info(&t!("theme.opened_dir", dir.display()));
                    } else {
                        print_error(&t!("theme.no_dir"));
                    }
                } else if ThemeManager::theme_exists(parts[1]) {
                    let themes_dir = dirs::config_dir().map(|d| d.join("ntc").join("themes"));
                    if let Some(dir) = themes_dir {
                        let theme_path = dir.join(format!("{}.ntc_theme", parts[1]));
                        let _ = crate::editor::edit_file(&theme_path);
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
