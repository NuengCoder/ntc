mod download;
pub(crate) mod manager;
pub(crate) mod sources;
mod types;

use crate::output::{print_error, print_info, print_success};
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use anyhow::Result;

/// Dispatch `ntpkg` subcommand.
pub fn cmd_ntpkg(args: &str, _nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(3, ' ').collect();
    let subcmd = parts.first().unwrap_or(&"").to_lowercase();

    match subcmd.as_str() {
        "install" | "i" => {
            let name = parts.get(1).unwrap_or(&"");
            let version = parts.get(2).unwrap_or(&"");
            if name.is_empty() {
                print_error("Usage: ntpkg install <language> [version]");
                return Ok(false);
            }
            let ver = if version.is_empty() {
                match pick_version_interactive(name) {
                    Some(v) => v,
                    None => return Ok(false),
                }
            } else {
                version.to_string()
            };
            match manager::install_package(name, &ver) {
                Ok(()) => {
                    print_success(&format!("{} {} installed!", name, ver));
                }
                Err(e) => print_error(&format!("Install failed: {:#}", e)),
            }
            Ok(false)
        }
        "uninstall" | "rm" => {
            let name = parts.get(1).unwrap_or(&"");
            let version = parts.get(2).unwrap_or(&"");
            if name.is_empty() {
                print_error("Usage: ntpkg uninstall <language> [version]");
                return Ok(false);
            }
            match manager::uninstall_package(name, version) {
                Ok(()) => print_success(&format!("{} {} removed", name, version)),
                Err(e) => print_error(&format!("Uninstall failed: {:#}", e)),
            }
            Ok(false)
        }
        "list" | "ls" => {
            match manager::list_packages() {
                Ok(packages) => {
                    if packages.is_empty() {
                        print_info("No packages installed. Use 'ntpkg install <language>'");
                    } else {
                        println!();
                        for (name, version, active) in &packages {
                            let marker = if *active { " (active)" } else { "" };
                            println!("  {} {}{}", name, version, marker);
                        }
                        println!();
                    }
                }
                Err(e) => print_error(&format!("{:#}", e)),
            }
            Ok(false)
        }
        "use" => {
            let name = parts.get(1).unwrap_or(&"");
            let version = parts.get(2).unwrap_or(&"");
            if name.is_empty() || version.is_empty() {
                print_error("Usage: ntpkg use <language> <version>");
                return Ok(false);
            }
            match manager::use_package(name, version) {
                Ok(()) => print_success(&format!("Now using {} {}", name, version)),
                Err(e) => print_error(&format!("{:#}", e)),
            }
            Ok(false)
        }
        "which" => {
            let name = parts.get(1).unwrap_or(&"");
            if name.is_empty() {
                print_error("Usage: ntpkg which <language> [version]");
                return Ok(false);
            }
            let version = parts.get(2).copied();
            match manager::which_package(name, version) {
                Ok(path) => println!("{}", path.display()),
                Err(e) => print_error(&format!("{:#}", e)),
            }
            Ok(false)
        }
        "available" | "avail" => {
            let name = parts.get(1).unwrap_or(&"");
            if name.is_empty() {
                let langs = sources::list_supported_languages();
                println!("  Supported languages:");
                for lang in langs {
                    println!("    {}", lang);
                }
                println!("  Use 'ntpkg available <language>' to list versions.");
                return Ok(false);
            }
            let versions = sources::list_available_versions(name);
            if versions.is_empty() {
                print_info(&format!("No versions tracked for '{name}'"));
            } else {
                println!();
                for v in &versions {
                    println!("  {}", v);
                }
                println!();
            }
            Ok(false)
        }
        "help" | "" => {
            print_help();
            Ok(false)
        }
        _ => {
            print_error(&format!("Unknown ntpkg subcommand: '{}'. Use 'ntpkg help'", subcmd));
            Ok(false)
        }
    }
}

/// Show interactive menu to pick a version.
fn pick_version_interactive(name: &str) -> Option<String> {
    let versions = sources::list_available_versions(name);
    if versions.is_empty() {
        print_error(&format!("No versions available for '{name}'"));
        return None;
    }
    let latest = versions.last().map(|s| s.as_str()).unwrap_or("");
    let items: Vec<MenuItem> = versions
        .iter()
        .map(|v| {
            let detail = if v.as_str() == latest { "(latest)" } else { "" };
            MenuItem::with_detail(v.as_str(), detail)
        })
        .collect();
    let title = format!("Select {name} version");
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(format!("Enter number (1-{}) or 0: ", versions.len()));
    menu.run().ok().flatten().map(|i| versions[i].clone())
}

/// Build the PATH entries from active packages (called at shell startup).
pub fn build_package_path_entries() -> Vec<std::path::PathBuf> {
    manager::active_bin_dirs().unwrap_or_default()
}

fn print_help() {
    println!();
    println!("  ntpkg — ntc package manager for language runtimes");
    println!();
    println!("  Usage:");
    println!("    ntpkg install <language> [version]   Download & install a runtime");
    println!("    ntpkg uninstall <language> [version]  Remove an installed runtime");
    println!("    ntpkg list                           Show installed runtimes");
    println!("    ntpkg use <language> <version>        Activate a specific version");
    println!("    ntpkg which <language>                Show path to active runtime");
    println!("    ntpkg available [language]             List languages or downloadable versions");
    println!("    ntpkg help                            Show this help");
    println!();
    println!("  Examples:");
    println!("    ntpkg install python 3.13");
    println!("    ntpkg use python 3.13");
    println!("    ntpkg list");
    println!();
}
