use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::ranfile::parser;
use crate::ranfile::runner::run_target;
use crate::ranfile::types::RanFormat;
use crate::shell::alias::expand_command_line_interactive;
use crate::t;

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

pub(crate) fn cmd_ran(args: &str, nav: &mut Navigator) -> Result<bool> {
    let args = args.trim();
    let cwd = nav.current_path().to_path_buf();

    let mut exec = |cmd: &str, path: &Path| -> Result<bool> {
        let trimmed = cmd.trim();
        // "run " prefix → direct system execution, no alias expansion, no built-in check
        if let Some(rest) = trimmed.strip_prefix("run ") {
            return Ok(run_target_cmd(rest, path));
        }
        // No "run " prefix → full ntc pipeline (interactive selects allowed)
        let expanded = match expand_command_line_interactive(cmd, nav)? {
            None => return Ok(false),
            Some(s) => s,
        };
        let lower = expanded.trim().to_lowercase();
        if lower.starts_with("ran ") || lower == "ran" {
            print_error(&t!("ran.calling_not_allowed"));
            return Ok(false);
        }
        match super::execute_command(&expanded, nav, &None) {
            Ok(_) => {}
            Err(e) => {
                print_error(&format!("{}", e));
                return Ok(false);
            }
        }
        Ok(true)
    };

    if args.is_empty() || args == "standalone" {
        if let Err(e) = run_target("standalone", &cwd, &mut exec) {
            print_error(&t!("ran.error_prefix", e));
        }
        return Ok(false);
    }

    let (sub, rest) = match args.split_once(' ') {
        Some((a, b)) => (a, b.trim()),
        None => (args, ""),
    };

    match sub {
        "init" => cmd_ran_init(rest),
        "deinit" => cmd_ran_deinit(rest),
        "help" => {
            print_ran_usage();
            Ok(false)
        }
        "list" | "ls" => cmd_ran_list(nav),
        "check" => cmd_ran_check(nav),
        "fmt" => cmd_ran_fmt(rest, nav),
        target => {
            let target = target.to_string();
            if let Err(e) = run_target(&target, &cwd, &mut exec) {
                print_error(&t!("ran.error_prefix", e));
            }
            Ok(false)
        }
    }
}

fn cmd_ran_init(_args: &str) -> Result<bool> {
    let cwd = std::env::current_dir()?;
    let fmt = parser::preferred_format();
    let path = cwd.join(fmt.file_name());

    if path.exists() {
        print_error(&t!("ran.already_exists", fmt.file_name(), cwd.display()));
        return Ok(false);
    }

    // A ranfile in another format may still be here. `ran` reads the preferred
    // format, so create that one and say which file it will use.
    if let Some((other, _)) = parser::resolve(&cwd) {
        crate::output::print_info(&t!("ran.fmt.old_kept", file_name(&other), fmt.file_name()));
    }

    std::fs::write(&path, parser::template(fmt)?)?;
    print_success(&t!("ran.created", path.display()));
    Ok(false)
}

fn cmd_ran_deinit(_args: &str) -> Result<bool> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            print_error(&t!("ran.get_cwd_failed", e));
            return Ok(false);
        }
    };

    // Remove every format: one project should not keep a leftover ranfile.
    let mut removed = false;
    for fmt in RanFormat::ALL {
        for name in fmt.file_names() {
            let path = cwd.join(name);
            if !path.exists() {
                continue;
            }
            removed = true;
            match std::fs::remove_file(&path) {
                Ok(_) => print_success(&t!("ran.removed", path.display())),
                Err(e) => print_error(&t!("ran.remove_failed", path.display(), e)),
            }
        }
    }

    if !removed {
        print_error(&t!("ran.not_found", parser::ranfile_names(), cwd.display()));
    }
    Ok(false)
}

/// `ran check` — interpret NTCRANFILE.toml the way `ran` will and report
/// every problem with the line that has to change. Shares the reporter used by
/// `ral check`, `math check`, … so every format reads the same way.
fn cmd_ran_check(nav: &mut Navigator) -> Result<bool> {
    let cwd = nav.current_path().to_path_buf();
    let Some((path, fmt)) = parser::resolve(&cwd) else {
        print_error(&t!("ran.not_found", parser::ranfile_names(), cwd.display()));
        return Ok(false);
    };
    println!("{}", t!("check.title", "ran").cyan().bold());
    let validator: fn(&str) -> Vec<super::check_common::Diagnostic> = match fmt {
        RanFormat::Toml => super::check_formats::ran,
        RanFormat::Json => super::check_formats::ran_json,
        RanFormat::Yaml => super::check_formats::ran_yaml,
    };
    super::check_common::run_one(&path, &file_name(&path), validator);
    Ok(false)
}

/// `ran fmt <toml|json|yaml> [-o <file>] [-y]` — convert the ranfile.
///
/// The chosen format also becomes the default for `ran init`, so a user who
/// declines the conversion (or has no ranfile yet) can create the new format
/// with a plain `ran init`.
fn cmd_ran_fmt(args: &str, nav: &mut Navigator) -> Result<bool> {
    let mut target: Option<String> = None;
    let mut output: Option<String> = None;
    let mut assume_yes = false;
    let mut tokens = args.split_whitespace();

    while let Some(token) = tokens.next() {
        match token {
            "-y" | "--yes" => assume_yes = true,
            "-o" | "--out" | "--output" => match tokens.next() {
                Some(value) => output = Some(value.to_string()),
                None => {
                    print_error(&t!("ran.fmt.usage"));
                    return Ok(false);
                }
            },
            other if target.is_none() && !other.starts_with('-') => {
                target = Some(other.to_string())
            }
            _ => {
                print_error(&t!("ran.fmt.usage"));
                return Ok(false);
            }
        }
    }

    let Some(raw) = target else {
        let current = parser::preferred_format();
        crate::output::print_info(&t!("ran.fmt.current", current.ext(), current.file_name()));
        print_error(&t!("ran.fmt.usage"));
        return Ok(false);
    };

    let Some(fmt) = RanFormat::parse(&raw) else {
        print_error(&t!("ran.fmt.unknown", raw));
        return Ok(false);
    };

    let cwd = nav.current_path().to_path_buf();
    // A plain `ran fmt <fmt>` records the project format, so a declined
    // conversion still lets `ran init` create that file. `-o` is a one-off
    // copy to an explicit path and must not change what `ran` reads here.
    let switch_format = output.is_none();

    if switch_format {
        crate::utils::config::Config::global_set_ran_format(fmt.ext());
    }

    let Some((source, source_fmt)) = parser::resolve(&cwd) else {
        crate::output::print_info(&t!("ran.fmt.no_file", fmt.ext(), fmt.file_name()));
        return Ok(false);
    };
    let source_name = file_name(&source);

    // Same format is a no-op unless an explicit output path was asked for,
    // where the point is a copy at that path rather than a conversion.
    if source_fmt == fmt && switch_format {
        crate::output::print_info(&t!("ran.fmt.already", source_name, fmt.ext()));
        return Ok(false);
    }

    let dest = match output {
        Some(raw) => {
            let expanded = crate::utils::paths::expand_path_str(&raw);
            let path = Path::new(&expanded);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                cwd.join(path)
            }
        }
        None => cwd.join(fmt.file_name()),
    };

    if dest.exists() {
        print_error(&t!("ran.fmt.exists", dest.display()));
        return Ok(false);
    }

    let prompt = t!("ran.fmt.confirm", source_name, fmt.ext(), file_name(&dest));
    if !assume_yes && !confirm(&prompt)? {
        crate::output::print_info(&t!("ran.fmt.skipped", fmt.file_name()));
        return Ok(false);
    }

    let ranfile = parser::parse_file(&source)?;
    let body = parser::render(&ranfile, fmt)?;
    std::fs::write(&dest, body)?;
    print_success(&t!("ran.fmt.written", dest.display()));
    if switch_format {
        crate::output::print_info(&t!("ran.fmt.old_kept", source_name, file_name(&dest)));
    }
    Ok(false)
}

/// Bare file name for messages.
fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string())
}

/// Ask a yes/no question on the terminal. Without a terminal a conversion is
/// refused unless `-y` was given, so a piped `ran fmt` can never hang.
fn confirm(prompt: &str) -> Result<bool> {
    use std::io::{IsTerminal, Write};
    if !std::io::stdin().is_terminal() {
        print_error(&t!("ran.fmt.no_terminal"));
        return Ok(false);
    }
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(matches!(line.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn cmd_ran_list(nav: &mut Navigator) -> Result<bool> {
    let cwd = nav.current_path();
    match parser::parse(cwd) {
        Ok(ranfile) => parser::list_targets(&ranfile),
        Err(e) => print_error(&format!("{}", e)),
    }
    Ok(false)
}

fn run_target_cmd(cmd: &str, cwd: &Path) -> bool {
    use crate::output::print_info;
    print_info(&t!("ran.executing", cmd));
    println!();
    match super::run_system_command(cmd, cwd) {
        Ok(status) => {
            println!();
            if status.success() {
                crate::output::print_success(&t!("ran.completed"));
                true
            } else {
                match status.code() {
                    Some(code) => crate::output::print_error(&t!("ran.exit_code", code)),
                    None => crate::output::print_warning(&t!("ran.terminated")),
                }
                false
            }
        }
        Err(e) => {
            println!();
            crate::output::print_error(&t!("ran.exec_failed", e));
            false
        }
    }
}

fn print_ran_usage() {
    println!("{}", t!("ran.help_title").cyan().bold());
    println!();
    println!("{}", t!("ran.help_usage"));
    println!("{}", t!("ran.help_standalone"));
    println!("{}", t!("ran.help_target"));
    let preferred = parser::preferred_format();
    println!("{}", t!("ran.help_init_fmt", preferred.file_name()));
    println!("{}", t!("ran.help_deinit_fmt", parser::ranfile_names()));
    println!("{}", t!("ran.help_list"));
    println!("{}", t!("ran.help_check_fmt", parser::ranfile_names()));
    println!("{}", t!("ran.help_fmt"));
    println!("{}", t!("ran.help_help"));
    println!();
    println!(
        "{}",
        t!("ran.fmt.current", preferred.ext(), preferred.file_name()).dimmed()
    );
    println!();
    println!("NTCRANFILE.toml example:");
    println!("  [vars]");
    println!("  CC = \"gcc\"");
    println!("  FLAGS = \"-Wall -O2\"");
    println!();
    println!("  [targets.standalone]");
    println!("  deps = []");
    println!("  cmd = \"echo 'Hello from ran!'\"");
    println!();
    println!("  [targets.build]");
    println!("  deps = [\"lint\"]");
    println!("  cmd = \"$(CC) $(FLAGS) main.c -o app\"");
    println!();
    println!("  [targets.clean]");
    println!("  deps = []");
    println!("  cmd = \"rm -rf *.o app\"");
    println!();
    println!("{}", t!("ran.section_vars").yellow());
    println!("{}", t!("ran.desc_vars"));
    println!("{}", t!("ran.desc_vars2"));
    println!();
    println!("{}", t!("ran.section_deps").yellow());
    println!("{}", t!("ran.desc_deps"));
    println!();
    println!("{}", t!("ran.section_aliases").yellow());
    println!("{}", t!("ran.desc_aliases"));
    println!("{}", t!("ran.desc_aliases2"));
    println!();
    println!("{}", t!("ran.section_reserved").red());
    println!("{}", t!("ran.desc_reserved"));
    println!();
    println!("{}", t!("ran.section_note").yellow());
    println!("{}", t!("ran.desc_note"));
}
