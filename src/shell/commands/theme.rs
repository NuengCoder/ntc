use crate::navigator::Navigator;
use crate::output::{print_error, print_success, print_info};
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
            println!("║{:^65}║", "🎨 ntc Theme Manager".cyan().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");
            println!("║  Current theme: {:48} ║", current.name.green().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");

            for (i, name) in themes.iter().enumerate() {
                let marker = if name == &current.name { " ✓" } else { "  " };
                println!("║  {}. {}{:<53} ",
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
            print!("{} ", "theme>".green().bold());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "0" || input.is_empty() {
                println!("{}", "Exiting theme manager.".dimmed());
                break;
            }

            // Drop the read guard on current before set_theme, which needs a write lock
            drop(current);

            // Parse number
            if let Ok(num) = input.parse::<usize>() {
                if num >= 1 && num <= themes.len() {
                    let theme_name = &themes[num - 1];
                    if ThemeManager::set_theme(theme_name) {
                        print_success(&format!("Switched to theme: {}", theme_name));
                    } else {
                        print_error(&format!("Failed to switch to theme: {}", theme_name));
                    }
                } else {
                    print_error(&format!("Invalid number: {}. Use 1-{}", num, themes.len()));
                }
                continue;
            }

            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0].to_lowercase().as_str() {
                "a" | "add" => {
                    print!("{} ", "Theme name: ".cyan());
                    io::stdout().flush()?;
                    let mut name = String::new();
                    io::stdin().read_line(&mut name)?;
                    let name = name.trim();

                    if name.is_empty() {
                        print_error("Theme name cannot be empty");
                    } else if let Err(e) = ThemeManager::create_theme(name) {
                        print_error(&format!("Failed to create theme: {}", e));
                    } else {
                        print_success(&format!("Created theme '{}' (opened in editor)", name));
                        print_info("Edit the file, save, then run 'theme reload' to apply changes");
                    }
                }
                "r" | "rm" | "remove" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme rm <name>");
                    } else {
                        let name = parts[1];
                        if let Err(e) = ThemeManager::remove_theme(name) {
                            print_error(&format!("Failed to remove theme: {}", e));
                        } else {
                            print_success(&format!("Removed theme: {}", name));
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
                                let _ = std::process::Command::new("termux-open").arg(&dir).status();
                            }
                            print_info(&format!("Opened theme directory: {}", dir.display()));
                        } else {
                            print_error("Could not determine theme directory");
                        }
                    } else {
                        let name = parts[1];
                        if ThemeManager::theme_exists(name) {
                            let themes_dir = dirs::config_dir()
                                .map(|d| d.join("ntc").join("themes"));
                            if let Some(dir) = themes_dir {
                                let theme_path = dir.join(format!("{}.ntc_theme", name));
                                let _ = crate::editor::edit_file(&theme_path);
                                print_info(&format!("Edit '{}', save, then run 'theme reload'", name));
                            }
                        } else {
                            print_error(&format!("Theme '{}' not found", name));
                        }
                    }
                }
                "i" | "info" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme info <name>");
                    } else {
                        let name = parts[1];
                        if let Some(theme) = ThemeManager::get_theme_info(name) {
                            println!();
                            println!("{}", "════════════════════════════════════════════════════════════".cyan());
                            println!("{}", format!("🎨 Theme: {}", theme.name).cyan().bold());
                            println!("{}", "════════════════════════════════════════════════════════════".cyan());
                            if let Some(author) = theme.author {
                                println!("  {}: {}", "Author".yellow(), author);
                            }
                            if let Some(desc) = theme.description {
                                println!("  {}: {}", "Description".yellow(), desc);
                            }
                            println!("  {}: {}", "Syntax colors".yellow(), "✓ defined".green());
                            println!("  {}: {}", "Shell colors".yellow(), "✓ defined".green());
                            println!("  {}: {}", "Editor colors".yellow(), "✓ defined".green());
                            println!();
                        } else {
                            print_error(&format!("Theme '{}' not found", name));
                        }
                    }
                }
                "x" | "export" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme export <name> [output_file]");
                    } else {
                        let name = parts[1];
                        let output = if parts.len() >= 3 {
                            std::path::PathBuf::from(parts[2])
                        } else {
                            let current_dir = std::env::current_dir().unwrap_or_default();
                            current_dir.join(format!("{}.ntc_theme", name))
                        };

                        if let Err(e) = ThemeManager::export_theme(name, &output) {
                            print_error(&format!("Failed to export theme: {}", e));
                        } else {
                            print_success(&format!("Exported theme '{}' to: {}", name, output.display()));
                        }
                    }
                }
                "m" | "import" => {
                    if parts.len() < 2 {
                        print_error("Usage: theme import <file.ntc_theme>");
                    } else {
                        let path = std::path::Path::new(parts[1]);
                        if !path.exists() {
                            print_error(&format!("File not found: {}", path.display()));
                        } else if let Err(e) = ThemeManager::import_theme(path) {
                            print_error(&format!("Failed to import theme: {}", e));
                        } else {
                            print_success(&format!("Imported theme from: {}", path.display()));
                            print_info("Run 'theme list' to see available themes");
                        }
                    }
                }
                "rnm" | "rename" => {
                    if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                        print_error("Usage: theme rnm <old_name> to <new_name>");
                        println!("Example: theme rnm mytheme to my_new_theme");
                    } else {
                        let old_name = parts[1];
                        let new_name = parts[3];
                        if let Err(e) = ThemeManager::rename_theme(old_name, new_name) {
                            print_error(&format!("Failed to rename theme: {}", e));
                        } else {
                            print_success(&format!("Renamed theme '{}' -> '{}'", old_name, new_name));
                        }
                    }
                }
                "reload" => {
                    ThemeManager::reload_themes();
                    print_success("Themes reloaded from disk");
                }
                "where" | "wh" => {
                    if let Some(dir) = ThemeManager::themes_dir() {
                        println!();
                        println!("{}", "Theme directory:".cyan().bold());
                        println!("  {}", dir.display().to_string().green());
                        if dir.exists() {
                            println!();
                            println!("{}", "Theme files:".cyan().bold());
                            if let Ok(entries) = std::fs::read_dir(&dir) {
                                let mut found = false;
                                for entry in entries.flatten() {
                                    let path = entry.path();
                                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                        if ext == "ntc_theme" || ext == "toml" {
                                            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                                                println!("  - {}.{}", name.blue(), ext.dimmed());
                                                found = true;
                                            }
                                        }
                                    }
                                }
                                if !found {
                                    println!("  {}", "(no theme files)".dimmed());
                                }
                            }
                        } else {
                            println!();
                            println!("  {}", "(directory does not exist yet)".dimmed());
                            println!("  {}", "Run 'theme add <name>' to create your first theme".dimmed());
                        }
                        println!();
                    } else {
                        print_error("Could not determine theme directory");
                    }
                }
                "make" | "mk" => {
                    let name = if parts.len() >= 2 { parts[1] } else { "example" };
                    let path = std::env::current_dir().unwrap_or_default().join(format!("{}.ntc_theme", name));
                    if path.exists() {
                        print_error(&format!("File already exists: {}", path.display()));
                    } else {
                        let content = ThemeManager::get_example_theme_template();
                        match std::fs::write(&path, &content) {
                            Ok(()) => {
                                print_success(&format!("Created theme template: {}", path.display()));
                                print_info("This is a local template file. Use 'theme import' to add it to your theme list.");
                            }
                            Err(e) => print_error(&format!("Failed to create file: {}", e)),
                        }
                    }
                }
                _ => {
                    print_error(&format!("Unknown command: {}", parts[0]));
                    println!("Type 'theme' for interactive menu");
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
                println!("Current theme: {}", current.name.green().bold());
                if let Some(author) = &current.author {
                    println!("Author: {}", author);
                }
                if let Some(desc) = &current.description {
                    println!("Description: {}", desc);
                }
            }
            "reload" => {
                ThemeManager::reload_themes();
                print_success("Themes reloaded from disk");
            }
            "add" => {
                if parts.len() < 2 {
                    print_error("Usage: theme add <name>");
                } else if let Err(e) = ThemeManager::create_theme(parts[1]) {
                    print_error(&format!("Failed to create theme: {}", e));
                } else {
                    print_success(&format!("Created theme '{}' (opened in editor)", parts[1]));
                }
            }
            "rm" => {
                if parts.len() < 2 {
                    print_error("Usage: theme rm <name>");
                } else if let Err(e) = ThemeManager::remove_theme(parts[1]) {
                    print_error(&format!("Failed to remove theme: {}", e));
                } else {
                    print_success(&format!("Removed theme: {}", parts[1]));
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
                        print_info(&format!("Opened theme directory: {}", dir.display()));
                    } else {
                        print_error("Could not determine theme directory");
                    }
                } else if ThemeManager::theme_exists(parts[1]) {
                    let themes_dir = dirs::config_dir()
                        .map(|d| d.join("ntc").join("themes"));
                    if let Some(dir) = themes_dir {
                        let theme_path = dir.join(format!("{}.ntc_theme", parts[1]));
                        let _ = crate::editor::edit_file(&theme_path);
                    }
                } else {
                    print_error(&format!("Theme '{}' not found", parts[1]));
                }
            }
            "info" => {
                if parts.len() < 2 {
                    print_error("Usage: theme info <name>");
                } else if let Some(theme) = ThemeManager::get_theme_info(parts[1]) {
                    println!("Theme: {}", theme.name.cyan().bold());
                    if let Some(author) = theme.author {
                        println!("Author: {}", author);
                    }
                    if let Some(desc) = theme.description {
                        println!("Description: {}", desc);
                    }
                } else {
                    print_error(&format!("Theme '{}' not found", parts[1]));
                }
            }
            "export" => {
                if parts.len() < 2 {
                    print_error("Usage: theme export <name> [output_file]");
                } else {
                    let name = parts[1];
                    let output = if parts.len() >= 3 {
                        std::path::PathBuf::from(parts[2])
                    } else {
                        std::env::current_dir().unwrap_or_default().join(format!("{}.ntc_theme", name))
                    };
                    if let Err(e) = ThemeManager::export_theme(name, &output) {
                        print_error(&format!("Failed to export: {}", e));
                    } else {
                        print_success(&format!("Exported to: {}", output.display()));
                    }
                }
            }
            "import" => {
                if parts.len() < 2 {
                    print_error("Usage: theme import <file.ntc_theme>");
                } else {
                    let path = std::path::Path::new(parts[1]);
                    if !path.exists() {
                        print_error(&format!("File not found: {}", path.display()));
                    } else if let Err(e) = ThemeManager::import_theme(path) {
                        print_error(&format!("Failed to import: {}", e));
                    } else {
                        print_success(&format!("Imported theme from: {}", path.display()));
                    }
                }
            }
            "rename" | "rnm" => {
                if parts.len() < 4 || parts[2].to_lowercase() != "to" {
                    print_error("Usage: theme rename <old_name> to <new_name>");
                } else if let Err(e) = ThemeManager::rename_theme(parts[1], parts[3]) {
                    print_error(&format!("Failed to rename: {}", e));
                } else {
                    print_success(&format!("Renamed '{}' -> '{}'", parts[1], parts[3]));
                }
            }
            "make" | "mk" => {
                let name = if parts.len() >= 2 { parts[1] } else { "example" };
                let path = std::env::current_dir().unwrap_or_default().join(format!("{}.ntc_theme", name));
                if path.exists() {
                    print_error(&format!("File already exists: {}", path.display()));
                } else {
                    let content = ThemeManager::get_example_theme_template();
                    match std::fs::write(&path, &content) {
                        Ok(()) => {
                            print_success(&format!("Created theme template: {}", path.display()));
                            print_info("This is a local template file. Use 'theme import' to add it to your theme list.");
                        }
                        Err(e) => print_error(&format!("Failed to create file: {}", e)),
                    }
                }
            }
            "where" | "wh" => {
                if let Some(dir) = ThemeManager::themes_dir() {
                    println!();
                    println!("{}", "Theme directory:".cyan().bold());
                    println!("  {}", dir.display().to_string().green());
                    if dir.exists() {
                        println!();
                        println!("{}", "Theme files:".cyan().bold());
                        if let Ok(entries) = std::fs::read_dir(&dir) {
                            let mut found = false;
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                    if ext == "ntc_theme" || ext == "toml" {
                                        if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                                            println!("  - {}.{}", name.blue(), ext.dimmed());
                                            found = true;
                                        }
                                    }
                                }
                            }
                            if !found {
                                println!("  {}", "(no theme files)".dimmed());
                            }
                        }
                    } else {
                        println!();
                        println!("  {}", "(directory does not exist yet)".dimmed());
                        println!("  {}", "Run 'theme add <name>' to create your first theme".dimmed());
                    }
                    println!();
                } else {
                    print_error("Could not determine theme directory");
                }
            }
            _ => {
                // Try to switch to theme by name, or by index if it's a number
                let themes = ThemeManager::list_themes();
                if let Ok(num) = parts[0].parse::<usize>() {
                    if num >= 1 && num <= themes.len() {
                        let theme_name = &themes[num - 1];
                        if ThemeManager::set_theme(theme_name) {
                            print_success(&format!("Switched to theme: {}", theme_name));
                        } else {
                            print_error(&format!("Failed to switch to theme: {}", theme_name));
                        }
                    } else {
                        print_error(&format!("Invalid number: {}. Use 1-{}", num, themes.len()));
                    }
                } else if ThemeManager::set_theme(parts[0]) {
                    print_success(&format!("Switched to theme: {}", parts[0]));
                } else {
                    print_error(&format!("Theme not found: {}. Use 'theme list' to see available themes.", parts[0]));
                }
            }
        }
        Ok(false)
    }
}
