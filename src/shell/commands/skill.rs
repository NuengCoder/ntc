//! `skill` command: argument grammar and terminal output.
//!
//! Network, storage and copy logic lives in `crate::skill`; this module only
//! parses the verbs (download / add / remove / delete / edit / avail / info /
//! sub) and prints localized results.

use anyhow::Result;
use std::io::IsTerminal;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::skill;
use crate::t;
use crate::utils::interactive::{InteractiveMenu, MenuItem};

#[derive(Debug, PartialEq, Eq)]
pub enum SkillCmd {
    List {
        all: bool,
    },
    Download {
        spec: String,
        name: Option<String>,
        force: bool,
    },
    Add {
        name: String,
        harness: Option<String>,
        global: bool,
        force: bool,
    },
    Remove {
        name: String,
        harness: Option<String>,
        global: bool,
        force: bool,
    },
    Delete {
        name: String,
        force: bool,
    },
    Edit {
        name: String,
    },
    Avail,
    Info {
        harness: String,
    },
    Sub {
        name: String,
    },
    Help,
}

/// Parse `skill <verb> [args]`. Flags may appear anywhere after the verb.
pub(crate) fn parse_args(args: &str) -> std::result::Result<SkillCmd, String> {
    let tokens: Vec<&str> = args.split_whitespace().collect();
    let Some((verb, rest)) = tokens.split_first() else {
        return Ok(SkillCmd::List { all: false });
    };
    let verb = verb.to_lowercase();
    let mut force = false;
    let mut global = false;
    let mut all = false;
    let mut name: Option<String> = None;
    let mut harness: Option<String> = None;
    let mut spec: Option<String> = None;
    let mut new_name: Option<String> = None;
    let download_verb = matches!(verb.as_str(), "download" | "dl" | "install" | "i");

    let mut i = 0;
    while i < rest.len() {
        let token = rest[i];
        if download_verb {
            // `--url <url>` / `--url=<url>` is the flag form of the same spec.
            if token == "--url" {
                let Some(next) = rest.get(i + 1) else {
                    return Err(t!("skill.missing_url"));
                };
                if spec.is_some() {
                    return Err(t!("skill.unexpected_arg", next));
                }
                spec = Some(next.trim_matches('"').to_string());
                i += 2;
                continue;
            }
            if let Some(value) = token.strip_prefix("--url=") {
                if spec.is_some() {
                    return Err(t!("skill.unexpected_arg", token));
                }
                if value.is_empty() {
                    return Err(t!("skill.missing_url"));
                }
                spec = Some(value.to_string());
                i += 1;
                continue;
            }
            // `--name <name>` renames the installed skill.
            if token == "--name" {
                let Some(next) = rest.get(i + 1) else {
                    return Err(t!("skill.missing_name"));
                };
                if new_name.is_some() {
                    return Err(t!("skill.unexpected_arg", next));
                }
                new_name = Some(next.trim_matches('"').to_string());
                i += 2;
                continue;
            }
            if let Some(value) = token.strip_prefix("--name=") {
                if new_name.is_some() {
                    return Err(t!("skill.unexpected_arg", token));
                }
                if value.is_empty() {
                    return Err(t!("skill.missing_name"));
                }
                new_name = Some(value.to_string());
                i += 1;
                continue;
            }
        }
        match token.to_lowercase().as_str() {
            "--force" | "-f" => force = true,
            "--global" | "-g" => global = true,
            "--all" | "-a" => all = true,
            "to" | "from" if matches!(verb.as_str(), "add" | "remove" | "rm") => {
                let Some(next) = rest.get(i + 1) else {
                    return Err(t!("skill.missing_harness"));
                };
                harness = Some(next.trim_matches('"').to_string());
                i += 1;
            }
            // `skill sub <name> ls` — the verb is optional filler.
            "ls" | "list" if verb == "sub" => {}
            _ => {
                if download_verb {
                    if spec.is_some() {
                        return Err(t!("skill.unexpected_arg", token));
                    }
                    spec = Some(token.to_string());
                } else if name.is_none() {
                    name = Some(token.to_string());
                } else {
                    return Err(t!("skill.unexpected_arg", token));
                }
            }
        }
        i += 1;
    }

    match verb.as_str() {
        "download" | "dl" | "install" | "i" => match spec {
            Some(spec) => Ok(SkillCmd::Download {
                spec,
                name: new_name,
                force,
            }),
            None => Err(t!("skill.usage_download")),
        },
        "add" => match name {
            Some(name) => Ok(SkillCmd::Add {
                name,
                harness,
                global,
                force,
            }),
            None => Err(t!("skill.usage_add")),
        },
        "remove" | "rm" => match name {
            Some(name) => Ok(SkillCmd::Remove {
                name,
                harness,
                global,
                force,
            }),
            None => Err(t!("skill.usage_remove")),
        },
        "delete" | "del" | "uninstall" | "u" => match name {
            Some(name) => Ok(SkillCmd::Delete { name, force }),
            None => Err(t!("skill.usage_delete")),
        },
        "edit" => match name {
            Some(name) => Ok(SkillCmd::Edit { name }),
            None => Err(t!("skill.usage_edit")),
        },
        "avail" | "available" => match name {
            Some(extra) => Err(t!("skill.unexpected_arg", extra)),
            None => Ok(SkillCmd::Avail),
        },
        "info" => match name {
            Some(harness) => Ok(SkillCmd::Info { harness }),
            None => Err(t!("skill.usage_info")),
        },
        "sub" => match name {
            Some(name) => Ok(SkillCmd::Sub { name }),
            None => Err(t!("skill.usage_sub")),
        },
        "help" | "--help" | "-h" => Ok(SkillCmd::Help),
        "list" | "ls" => Ok(SkillCmd::List { all }),
        other => Err(t!("skill.unknown_subcmd", other)),
    }
}

fn print_help() {
    println!();
    println!("{}", t!("skill.help_usage"));
    println!();
    println!("{}", t!("skill.help_download"));
    println!("{}", t!("skill.help_install"));
    println!("{}", t!("skill.help_avail"));
    println!("{}", t!("skill.help_add"));
    println!("{}", t!("skill.help_remove"));
    println!("{}", t!("skill.help_delete"));
    println!("{}", t!("skill.help_edit"));
    println!("{}", t!("skill.help_list"));
    println!("{}", t!("skill.help_list_all"));
    println!("{}", t!("skill.help_info"));
    println!("{}", t!("skill.help_sub"));
    println!("{}", t!("skill.help_help"));
    println!();
    println!("{}", t!("skill.help_harnesses"));
    println!("{}", t!("skill.help_scope"));
    println!();
    println!("{}", t!("help.examples_header"));
    println!("    {}", t!("skill.example1"));
    println!("    {}", t!("skill.example2"));
    println!("    {}", t!("skill.example3"));
    println!("    {}", t!("skill.example4"));
    println!("    {}", t!("skill.example5"));
    println!("    {}", t!("skill.example6"));
    println!("    {}", t!("skill.example7"));
    println!("    {}", t!("skill.example8"));
    println!("    {}", t!("skill.example9"));
    println!("    {}", t!("skill.example10"));
    println!();
}

fn pick_harness() -> Option<&'static str> {
    if !std::io::stdin().is_terminal() {
        return None;
    }
    let items: Vec<MenuItem> = skill::harness::HARNESSES
        .iter()
        .map(|h| MenuItem::with_detail(*h, ""))
        .collect();
    let title = t!("skill.harness_pick");
    let menu = InteractiveMenu::new(&title)
        .items(items)
        .prompt(t!("skill.enter_number", skill::harness::HARNESSES.len()));
    menu.run()
        .ok()
        .flatten()
        .map(|index| skill::harness::HARNESSES[index])
}

fn resolve_harness(harness: Option<String>, usage_key: &str) -> Option<&'static str> {
    match harness {
        Some(name) => match skill::harness::canonical_harness(&name) {
            Ok(harness) => Some(harness),
            Err(e) => {
                print_error(&format!("{:#}", e));
                None
            }
        },
        None => match pick_harness() {
            Some(harness) => Some(harness),
            None => {
                print_error(&t!(usage_key));
                None
            }
        },
    }
}

fn scope_label(scope: &str) -> String {
    if scope == "global" {
        t!("skill.scope_global")
    } else {
        t!("skill.scope_project")
    }
}

fn print_copies(added: &[crate::skill::registry::AddedRecord]) {
    for add in added {
        println!(
            "    {} ({}): {}",
            add.harness,
            scope_label(&add.scope),
            add.path
        );
    }
}

fn list_skills(all: bool) {
    if !all {
        list_bundles();
        return;
    }
    match skill::list() {
        Ok(entries) => {
            if entries.is_empty() {
                print_info(&t!("skill.empty"));
                return;
            }
            println!();
            println!("{}", t!("skill.installed_header"));
            for entry in entries {
                println!(
                    "  {:<24} {} {}",
                    entry.name,
                    t!("skill.bundle_tag", entry.bundle),
                    entry.description
                );
                print_copies(&entry.added);
            }
            println!();
        }
        Err(e) => print_error(&format!("{:#}", e)),
    }
}

/// `skill ls` without `--all`: one line per downloaded bundle, not per skill.
fn list_bundles() {
    match skill::list_bundles() {
        Ok(bundles) => {
            if bundles.is_empty() {
                print_info(&t!("skill.empty"));
                return;
            }
            println!();
            println!("{}", t!("skill.bundles_header"));
            for bundle in bundles {
                if bundle.skills.len() > 1 {
                    println!(
                        "  {:<24} {}",
                        bundle.name,
                        t!("skill.bundle_count", bundle.skills.len())
                    );
                } else {
                    println!("  {}", bundle.name);
                }
                print_copies(&bundle.added);
            }
            println!();
            println!("{}", t!("skill.list_all_hint"));
            println!();
        }
        Err(e) => print_error(&format!("{:#}", e)),
    }
}

fn print_available() {
    let entries = skill::catalog::available();
    println!();
    println!("{}", t!("skill.avail_header"));
    for entry in entries {
        let source = match entry.skill {
            Some(skill) => format!("{}/{}", entry.repo, skill),
            None => t!("skill.avail_all", entry.repo),
        };
        println!("  {:<24} {}", entry.alias, source);
    }
    println!();
    println!("{}", t!("skill.avail_hint"));
    println!();
}

fn info_skills(harness: &str) {
    match skill::info(harness) {
        Ok(entries) => {
            if entries.is_empty() {
                print_info(&t!("skill.info_empty", harness));
                return;
            }
            println!();
            println!("{}", t!("skill.info_header", harness));
            for entry in entries {
                let tag = if entry.managed {
                    t!("skill.info_ntc")
                } else {
                    t!("skill.info_manual")
                };
                println!(
                    "  {:<24} {} {} {}",
                    entry.name,
                    scope_label(&entry.scope),
                    tag,
                    entry.path.display()
                );
                if !entry.description.is_empty() {
                    println!("    {}", entry.description);
                }
            }
            println!();
        }
        Err(e) => print_error(&format!("{:#}", e)),
    }
}

fn show_subskills(name: &str) {
    match skill::subskills(name) {
        Ok((label, members)) => {
            if members.len() <= 1 {
                print_info(&t!("skill.sub_none", label));
                return;
            }
            println!();
            println!("{}", t!("skill.sub_header", label));
            for (skill, description) in members {
                println!("  {:<24} {}", skill, description);
            }
            println!();
        }
        Err(e) => print_error(&format!("{:#}", e)),
    }
}

pub fn cmd_skill(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let command = match parse_args(args) {
        Ok(command) => command,
        Err(message) => {
            print_error(&message);
            return Ok(false);
        }
    };

    match command {
        SkillCmd::Help => print_help(),
        SkillCmd::List { all } => list_skills(all),
        SkillCmd::Avail => print_available(),
        SkillCmd::Info { harness } => info_skills(&harness),
        SkillCmd::Sub { name } => show_subskills(&name),
        SkillCmd::Download { spec, name, force } => {
            match skill::download(&spec, name.as_deref(), force) {
                Ok(downloaded) => {
                    for item in downloaded {
                        print_success(&t!("skill.downloaded", item.name, item.description));
                    }
                }
                Err(e) => print_error(&format!("{:#}", e)),
            }
        }
        SkillCmd::Add {
            name,
            harness,
            global,
            force,
        } => {
            let Some(harness) = resolve_harness(harness, "skill.usage_add") else {
                return Ok(false);
            };
            match skill::add(&[name], harness, global, force) {
                Ok(copied) => {
                    for (skill_name, path) in copied {
                        print_success(&t!("skill.added", skill_name, path.display()));
                    }
                }
                Err(e) => print_error(&format!("{:#}", e)),
            }
        }
        SkillCmd::Remove {
            name,
            harness,
            global,
            force,
        } => {
            let Some(harness) = resolve_harness(harness, "skill.usage_remove") else {
                return Ok(false);
            };
            match skill::remove(&[name], harness, global, force) {
                Ok(removed) => {
                    for (skill_name, path) in removed {
                        print_success(&t!("skill.removed", skill_name, path.display()));
                    }
                }
                Err(e) => print_error(&format!("{:#}", e)),
            }
        }
        SkillCmd::Delete { name, force } => match skill::delete(&[name], force) {
            Ok(deleted) => {
                for skill_name in deleted {
                    print_success(&t!("skill.deleted", skill_name));
                }
            }
            Err(e) => print_error(&format!("{:#}", e)),
        },
        SkillCmd::Edit { name } => {
            // The editor's embedded shell already runs one editor.
            if std::env::var("NTC_IN_EDITOR").is_ok() {
                print_error(&t!("misc.ne.already_in_editor"));
                return Ok(false);
            }
            match skill::edit_target(&name) {
                Ok((skill_name, path)) => {
                    print_info(&t!("skill.edit_opened", path.display()));
                    super::misc::launch_ne_editor(path, None);
                    match skill::after_edit(&skill_name) {
                        Ok(_) => print_success(&t!("skill.edit_refreshed", skill_name)),
                        Err(e) => print_error(&t!("skill.edit_invalid", format!("{:#}", e))),
                    }
                }
                Err(e) => print_error(&format!("{:#}", e)),
            }
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bare_and_list_verbs_list_the_library() {
        assert_eq!(parse_args("").unwrap(), SkillCmd::List { all: false });
        assert_eq!(parse_args("list").unwrap(), SkillCmd::List { all: false });
        assert_eq!(parse_args("ls").unwrap(), SkillCmd::List { all: false });
        assert_eq!(
            parse_args("ls --all").unwrap(),
            SkillCmd::List { all: true }
        );
        assert_eq!(parse_args("list -a").unwrap(), SkillCmd::List { all: true });
    }

    #[test]
    fn download_takes_a_spec_and_flags() {
        assert_eq!(
            parse_args("download superpowers").unwrap(),
            SkillCmd::Download {
                spec: "superpowers".into(),
                name: None,
                force: false
            }
        );
        assert_eq!(
            parse_args("download obra/superpowers --force").unwrap(),
            SkillCmd::Download {
                spec: "obra/superpowers".into(),
                name: None,
                force: true
            }
        );
        assert!(parse_args("download").is_err());
        assert!(parse_args("download a b").is_err());
    }

    #[test]
    fn install_is_an_alias_of_download() {
        for verb in ["install", "i", "dl"] {
            assert_eq!(
                parse_args(&format!("{verb} superpowers")).unwrap(),
                SkillCmd::Download {
                    spec: "superpowers".into(),
                    name: None,
                    force: false
                },
                "{verb}"
            );
        }
    }

    #[test]
    fn uninstall_is_an_alias_of_delete() {
        for verb in ["uninstall", "u", "del"] {
            assert_eq!(
                parse_args(&format!("{verb} superpowers --force")).unwrap(),
                SkillCmd::Delete {
                    name: "superpowers".into(),
                    force: true
                },
                "{verb}"
            );
        }
        assert!(parse_args("uninstall").is_err());
    }

    #[test]
    fn download_accepts_the_url_flag() {
        assert_eq!(
            parse_args("download --url https://github.com/obra/superpowers").unwrap(),
            SkillCmd::Download {
                spec: "https://github.com/obra/superpowers".into(),
                name: None,
                force: false
            }
        );
        assert_eq!(
            parse_args("download --url=https://github.com/obra/superpowers --force").unwrap(),
            SkillCmd::Download {
                spec: "https://github.com/obra/superpowers".into(),
                name: None,
                force: true
            }
        );
        assert!(parse_args("download --url").is_err());
        assert!(parse_args("download --url=").is_err());
        // A positional spec doubles as the URL: reject both at once.
        assert!(parse_args("download superpowers --url https://github.com/x/y").is_err());
        assert!(parse_args("download --url=a extra").is_err());
    }

    #[test]
    fn download_accepts_the_name_flag() {
        assert_eq!(
            parse_args("download --url https://github.com/x/y --name my-skill").unwrap(),
            SkillCmd::Download {
                spec: "https://github.com/x/y".into(),
                name: Some("my-skill".into()),
                force: false
            }
        );
        assert_eq!(
            parse_args("install superpowers --name=sp-copy").unwrap(),
            SkillCmd::Download {
                spec: "superpowers".into(),
                name: Some("sp-copy".into()),
                force: false
            }
        );
        assert!(parse_args("download superpowers --name").is_err());
        assert!(parse_args("download superpowers --name=").is_err());
        assert!(parse_args("download superpowers --name a --name b").is_err());
    }

    #[test]
    fn add_and_remove_parse_the_to_from_grammar() {
        assert_eq!(
            parse_args("add superpower to opencode").unwrap(),
            SkillCmd::Add {
                name: "superpower".into(),
                harness: Some("opencode".into()),
                global: false,
                force: false
            }
        );
        assert_eq!(
            parse_args("add brainstorming to claude --global").unwrap(),
            SkillCmd::Add {
                name: "brainstorming".into(),
                harness: Some("claude".into()),
                global: true,
                force: false
            }
        );
        assert_eq!(
            parse_args("remove superpowers from codex -f").unwrap(),
            SkillCmd::Remove {
                name: "superpowers".into(),
                harness: Some("codex".into()),
                global: false,
                force: true
            }
        );
        // Harness may be omitted (interactive menu / usage error at run time).
        assert_eq!(
            parse_args("add frontend-design").unwrap(),
            SkillCmd::Add {
                name: "frontend-design".into(),
                harness: None,
                global: false,
                force: false
            }
        );
        assert!(parse_args("add x to").is_err());
    }

    #[test]
    fn delete_and_edit_take_one_name() {
        assert_eq!(
            parse_args("delete superpowers --force").unwrap(),
            SkillCmd::Delete {
                name: "superpowers".into(),
                force: true
            }
        );
        assert_eq!(
            parse_args("edit brainstorming").unwrap(),
            SkillCmd::Edit {
                name: "brainstorming".into()
            }
        );
        assert!(parse_args("edit").is_err());
        assert!(parse_args("delete").is_err());
    }

    #[test]
    fn avail_info_and_sub_parse() {
        assert_eq!(parse_args("avail").unwrap(), SkillCmd::Avail);
        assert_eq!(parse_args("available").unwrap(), SkillCmd::Avail);
        assert!(parse_args("avail junk").is_err());

        assert_eq!(
            parse_args("info opencode").unwrap(),
            SkillCmd::Info {
                harness: "opencode".into()
            }
        );
        assert!(parse_args("info").is_err());

        assert_eq!(
            parse_args("sub superpowers ls").unwrap(),
            SkillCmd::Sub {
                name: "superpowers".into()
            }
        );
        assert_eq!(
            parse_args("sub superpowers").unwrap(),
            SkillCmd::Sub {
                name: "superpowers".into()
            }
        );
        assert!(parse_args("sub").is_err());
        assert!(parse_args("sub a ls b").is_err());
    }

    #[test]
    fn unknown_verbs_report_help() {
        let err = parse_args("frobnicate").unwrap_err();
        assert!(err.contains("frobnicate"), "{err}");
        assert!(parse_args("skill extra junk").is_err());
        assert_eq!(parse_args("help").unwrap(), SkillCmd::Help);
        assert_eq!(parse_args("--help").unwrap(), SkillCmd::Help);
    }
}
