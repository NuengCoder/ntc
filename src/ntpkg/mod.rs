mod download;
mod installers;
pub(crate) mod manager;
pub(crate) mod sources;
mod types;

use colored::Colorize;

use crate::output::{print_error, print_info, print_success};
use crate::t;
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
                print_error(&t!("ntpkg.install_usage"));
                return Ok(false);
            }
            let ver = if version.is_empty()
                || (*name == "java" && !version.is_empty() && !version.contains('-'))
            {
                let initial = if version.is_empty() {
                    match pick_version_interactive(name) {
                        Some(v) => v,
                        None => return Ok(false),
                    }
                } else {
                    version.to_string()
                };
                if *name == "java" {
                    let minors = sources::list_java_minor_versions(&initial);
                    if minors.is_empty() {
                        initial
                    } else {
                        match pick_java_version_interactive(&initial, &minors) {
                            Some(m) => m,
                            None => return Ok(false),
                        }
                    }
                } else {
                    initial
                }
            } else {
                version.to_string()
            };
            match manager::install_package(name, &ver) {
                Ok(()) => {
                    print_success(&t!("ntpkg.installed_msg", name, ver));
                    let install_dir = types::version_dir(name, &ver);
                    println!();
                    print_info(&t!("ntpkg.install_loc", install_dir.display()));
                    print_info(&format!("cd {}", install_dir.display()));
                    #[cfg(windows)]
                    {
                        let bin_path = types::version_dir(name, &ver);
                        println!();
                        print_info(&t!("ntpkg.add_to_user_path"));
                        println!("  usrpath add {}", bin_path.display());
                        println!();
                        print_info(&t!("ntpkg.add_to_sys_path"));
                        println!("  mode admin");
                        println!("  syspath add {}", bin_path.display());
                    }
                    #[cfg(not(windows))]
                    {
                        let bin_path = types::bin_dir(name, &ver);
                        println!();
                        print_info(&t!("ntpkg.add_to_path"));
                        println!("  export PATH=\"{}$PATH\"", bin_path.display());
                    }
                }
                Err(e) => print_error(&t!("ntpkg.install_failed", e)),
            }
            Ok(false)
        }
        "uninstall" | "rm" => {
            let name = *parts.get(1).unwrap_or(&"");
            let mut version = parts.get(2).unwrap_or(&"").to_string();
            if name.is_empty() {
                print_error(&t!("ntpkg.uninstall_usage"));
                return Ok(false);
            }
            if version.is_empty() {
                let manifest = manager::load_manifest()?;
                if let Some(active) = manifest.active.get(name) {
                    version = active.clone();
                } else {
                    print_error(&t!("ntpkg.no_active_version", name));
                    return Ok(false);
                }
            }
            match manager::uninstall_package(name, &version) {
                Ok(()) => print_success(&t!("ntpkg.removed_msg", name, version)),
                Err(e) => print_error(&t!("ntpkg.uninstall_failed", e)),
            }
            Ok(false)
        }
        "list" | "ls" => {
            match manager::list_packages() {
                Ok(packages) => {
                    if packages.is_empty() {
                        print_info(&t!("ntpkg.no_packages"));
                    } else {
                        println!();
                        for (name, version, active) in &packages {
                            let marker = if *active { t!("ntpkg.active_marker") } else { String::new() };
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
                print_error(&t!("ntpkg.use_usage"));
                return Ok(false);
            }
            match manager::use_package(name, version) {
                Ok(()) => print_success(&t!("ntpkg.now_using", name, version)),
                Err(e) => print_error(&format!("{:#}", e)),
            }
            Ok(false)
        }
        "which" => {
            let name = parts.get(1).unwrap_or(&"");
            if name.is_empty() {
                print_error(&t!("ntpkg.which_usage"));
                return Ok(false);
            }
            let version = parts.get(2).copied();
            match manager::which_package(name, version) {
                Ok(path) => println!("{}", path.display()),
                Err(e) => print_error(&t!("ntpkg.install_failed", e)),
            }
            Ok(false)
        }
        "available" | "avail" => {
            let name = parts.get(1).unwrap_or(&"");
            if name.is_empty() {
                let langs = sources::list_supported_languages();
                println!("{}", t!("ntpkg.supported_langs"));
                for lang in langs {
                    println!("    {}", lang);
                }
                println!("{}", t!("ntpkg.available_hint"));
                return Ok(false);
            }
            let versions = sources::list_available_versions(name);
            if versions.is_empty() {
                print_info(&t!("ntpkg.no_versions", name));
            } else {
                println!();
                for v in &versions {
                    println!("  {}", v);
                }
                println!();
            }
            Ok(false)
        }
        "search" => {
            let keyword = parts.get(1).map(|s| s.trim()).filter(|s| !s.is_empty());
            let all = sources::list_packages_with_descriptions();
            let mut sorted = all.clone();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            let matched = if let Some(kw) = keyword {
                let kw_lower = kw.to_lowercase();
                sorted
                    .into_iter()
                    .filter(|(name, desc)| {
                        name.to_lowercase().contains(&kw_lower)
                            || desc.to_lowercase().contains(&kw_lower)
                    })
                    .collect::<Vec<_>>()
            } else {
                sorted
            };
            if matched.is_empty() {
                print_info(&t!("ntpkg.no_match", keyword.unwrap_or("")));
            } else {
                println!();
                println!("╔══════════════════════════════════════════════════════════════════╗");
                println!("║{:^60}", t!("ntpkg.available_title").cyan().bold());
                if let Some(kw) = keyword {
                    println!(
                        "║  {}                                  ",
                        t!("ntpkg.filter_label", kw).dimmed()
                    );
                }
                println!("╠══════════════════════════════════════════════════════════════════╣");
                for (name, desc) in &matched {
                    println!("║  {:<12} {:<45}", name.cyan(), desc);
                }
                println!("╚══════════════════════════════════════════════════════════════════╝");
                println!("  {}", t!("ntpkg.package_count", matched.len()));
                println!();
            }
            Ok(false)
        }
        "info" => {
            let name = parts.get(1).copied().unwrap_or("");
            if name.is_empty() {
                print_error(&t!("ntpkg.info_usage"));
                return Ok(false);
            }
            let manifest = manager::load_manifest()?;
            let active = manifest.active.get(name);
            let installed = manifest.installed.get(name);
            let vdir = types::version_dir(name, active.unwrap_or(&"".to_string()));
            let has_bin_dir = active.is_some() && vdir.exists();
            println!();
            println!("╔══════════════════════════════════════════════════════════════════╗");
            println!("║{:^60}", t!("ntpkg.package_info_title", name).cyan().bold());
            println!("╠══════════════════════════════════════════════════════════════════╣");
            if let Some(ver) = active {
                println!("║  {:<20} {:<37}", t!("ntpkg.active_label"), ver.to_string().green());
            } else {
                println!("║  {:<20} {:<37}", t!("ntpkg.active_label"), t!("ntpkg.not_set").dimmed());
            }
            if let Some(vers) = installed {
                let versions_str = vers.join(", ");
                println!("║  {:<20} {:<37}", t!("ntpkg.installed_label"), versions_str);
            } else {
                println!("║  {:<20} {:<37}", t!("ntpkg.installed_label"), t!("ntpkg.none").dimmed());
            }
            if active.is_some() {
                println!("║  {:<20} {:<37}", t!("ntpkg.location_label"), vdir.display().to_string());
                if has_bin_dir {
                    if let Ok(entries) = std::fs::read_dir(&vdir) {
                        let bins: Vec<String> = entries
                            .filter_map(|e| e.ok())
                            .filter(|e| e.path().is_file())
                            .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                            .collect();
                        if !bins.is_empty() {
                            let bins_str = bins.join(", ");
                            println!("║  {:<20} {:<37}", t!("ntpkg.binaries_label"), bins_str);
                        }
                    }
                }
            }
            println!("╚══════════════════════════════════════════════════════════════════╝");
            println!();
            Ok(false)
        }
        "help" | "" => {
            print_help();
            Ok(false)
        }
        _ => {
            print_error(&t!("ntpkg.unknown_subcmd", subcmd));
            Ok(false)
        }
    }
}

/// Dispatch `ntpx <pkg> [binary?] [args...]`
pub fn cmd_ntpx(args: &str, _nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.is_empty() || parts[0].is_empty() {
        print_error(&t!("ntpkg.ntpx_usage"));
        return Ok(false);
    }

    let pkg = parts[0];
    let rest = if parts.len() > 1 { parts[1] } else { "" };

    let (binary, exec_args) = resolve_ntpx_binary_and_args(pkg, rest);

    match manager::execute_package(pkg, &binary, &exec_args) {
        Ok(_code) => {}
        Err(e) => {
            print_error(&t!("ntpkg.install_failed", e));
        }
    }
    Ok(false)
}

fn resolve_ntpx_binary_and_args(pkg: &str, rest: &str) -> (String, Vec<String>) {
    if rest.is_empty() {
        return (pkg.to_string(), vec![]);
    }

    let tokens: Vec<&str> = rest.split(' ').collect();
    let first = tokens[0];

    if first.starts_with('-') {
        return (
            pkg.to_string(),
            tokens.iter().map(|s| s.to_string()).collect(),
        );
    }

    if tokens.len() == 1 {
        return (
            pkg.to_string(),
            tokens.iter().map(|s| s.to_string()).collect(),
        );
    }

    let binary = first.to_string();
    let exec_args = tokens[1..].iter().map(|s| s.to_string()).collect();
    (binary, exec_args)
}

/// Show interactive menu to pick a version.
fn pick_version_interactive(name: &str) -> Option<String> {
    let versions = sources::list_available_versions(name);
    if versions.is_empty() {
        print_error(&t!("ntpkg.no_versions_for", name));
        return None;
    }
    let latest = versions.last().map(|s| s.as_str()).unwrap_or("");
    let items: Vec<MenuItem> = versions
        .iter()
        .map(|v| {
            let label = if name == "java" {
                format!("JDK{}", v)
            } else {
                v.to_string()
            };
            let detail = if v.as_str() == latest { "(latest)" } else { "" };
            MenuItem::with_detail(&label, detail)
        })
        .collect();
    let title = t!("ntpkg.select_version", name);
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("ntpkg.enter_number", versions.len()));
    menu.run().ok().flatten().map(|i| versions[i].clone())
}

/// Show interactive menu for Java minor versions (e.g., jdk-21.0.5+9).
fn pick_java_version_interactive(feature: &str, versions: &[String]) -> Option<String> {
    if versions.is_empty() {
        return None;
    }
    let latest = versions.last().map(|s| s.as_str()).unwrap_or("");
    let items: Vec<MenuItem> = versions
        .iter()
        .map(|v| {
            let label = v
                .strip_prefix("jdk-")
                .or_else(|| v.strip_prefix("jdk"))
                .unwrap_or(v);
            let detail = if v.as_str() == latest { "(latest)" } else { "" };
            MenuItem::with_detail(label, detail)
        })
        .collect();
    let title = t!("ntpkg.select_jdk_version", feature);
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("ntpkg.enter_number", versions.len()));
    menu.run().ok().flatten().map(|i| versions[i].clone())
}

/// Build the PATH entries from active packages (called at shell startup).
pub fn build_package_path_entries() -> Vec<std::path::PathBuf> {
    manager::active_bin_dirs().unwrap_or_default()
}

fn print_help() {
    println!();
    println!("{}", t!("ntpkg.help_usage"));
    println!();
    println!("{}", t!("ntpkg.help_usage_title"));
    println!("{}", t!("ntpkg.help_install"));
    println!("{}", t!("ntpkg.help_uninstall"));
    println!("{}", t!("ntpkg.help_list"));
    println!("{}", t!("ntpkg.help_use"));
    println!("{}", t!("ntpkg.help_which"));
    println!("{}", t!("ntpkg.help_available"));
    println!("{}", t!("ntpkg.help_help"));
    println!();
    println!("{}", t!("ntpkg.ntpx_header"));
    println!("{}", t!("ntpkg.ntpx_usage_line"));
    println!();
    println!("{}", t!("ntpkg.help_examples_title"));
    println!("{}", t!("ntpkg.help_ex_ntpx_go"));
    println!("{}", t!("ntpkg.help_ex_ntpx_py"));
    println!("{}", t!("ntpkg.help_ex_ntpx_msys2"));
    println!("    ntpkg install python 3.13");
    println!("    ntpkg install git 2.45");
    println!("    ntpkg install gh 2.58");
    println!("    ntpkg use python 3.13");
    println!("    ntpkg list");
    println!();
}
