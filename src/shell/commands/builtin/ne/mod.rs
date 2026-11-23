//! `ne` — open the ntc editor, or create files from templates.

pub(crate) mod completion;
pub(crate) mod editor;
pub(crate) mod help;
pub(crate) mod mql;
pub(crate) mod ntc;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::session::SessionState;
use crate::t;
use anyhow::Result;
use std::path::PathBuf;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::args::tokenize_quoted;

#[derive(Debug, PartialEq, Eq)]
enum NeCreateKind {
    Init,
    Super,
}

#[derive(Debug, PartialEq, Eq)]
enum NeError {
    MissingCreateTarget,
    UsageOpen,
    OpenNotCreated(String),
}

#[derive(Debug, PartialEq, Eq)]
enum NeAction {
    Welcome,
    Open {
        path: PathBuf,
        line: Option<usize>,
    },
    Create {
        kind: NeCreateKind,
        targets: Vec<PathBuf>,
        open: Option<usize>,
    },
    Error(NeError),
}

fn ne_flag_rest<'a>(args: &'a str, flag: &str) -> Option<&'a str> {
    let rest = args.strip_prefix(flag)?;
    if rest.is_empty() {
        Some("")
    } else if rest.starts_with(' ') || rest.starts_with('\t') {
        Some(rest.trim_start())
    } else {
        None
    }
}

fn parse_open_spec(raw: &str) -> (PathBuf, Option<usize>) {
    if let Some((file_part, line_part)) = raw.rsplit_once(':') {
        if let Ok(line) = line_part.parse::<usize>() {
            if line > 0 {
                return (PathBuf::from(file_part), Some(line));
            }
        }
    }
    (PathBuf::from(raw), None)
}

fn parse_ne_create(kind: NeCreateKind, rest: &str) -> NeAction {
    let tokens = tokenize_quoted(rest);
    if tokens.is_empty() {
        return NeAction::Error(NeError::MissingCreateTarget);
    }
    let mut targets = Vec::new();
    let mut open_name: Option<String> = None;
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "-I" {
            if i + 1 >= tokens.len() || i + 2 != tokens.len() {
                return NeAction::Error(NeError::UsageOpen);
            }
            open_name = Some(tokens[i + 1].clone());
            break;
        }
        for part in tokens[i].split(',') {
            let part = part.trim();
            if !part.is_empty() {
                targets.push(PathBuf::from(part));
            }
        }
        i += 1;
    }
    if targets.is_empty() {
        return NeAction::Error(NeError::MissingCreateTarget);
    }
    let open = match open_name {
        Some(name) => {
            let target = PathBuf::from(&name);
            match targets.iter().position(|p| p == &target) {
                Some(idx) => Some(idx),
                None => return NeAction::Error(NeError::OpenNotCreated(name)),
            }
        }
        None => None,
    };
    NeAction::Create {
        kind,
        targets,
        open,
    }
}

fn parse_ne_action(args: &str) -> NeAction {
    let args = args.trim();
    if args.is_empty() || args == "." {
        return NeAction::Welcome;
    }
    if let Some(rest) = ne_flag_rest(args, "--init") {
        return parse_ne_create(NeCreateKind::Init, rest);
    }
    if let Some(rest) = ne_flag_rest(args, "--super") {
        return parse_ne_create(NeCreateKind::Super, rest);
    }
    let (path, line) = parse_open_spec(args);
    NeAction::Open { path, line }
}

pub(crate) fn launch_ne_editor(raw_path: PathBuf, line: Option<usize>) {
    if let Some(line) = line {
        crate::shell::commands::builtin::ne::editor::GOTO_LINE
            .store(line, std::sync::atomic::Ordering::SeqCst);
    }
    let path = if raw_path.is_dir() {
        let welcome = raw_path.join("welcome.ntcEditor");
        let _ = crate::shell::commands::builtin::ne::editor::refresh_welcome_file(&welcome);
        welcome
    } else {
        raw_path
    };
    let restored = SessionState::read_global().editor_session.clone();
    match crate::shell::commands::builtin::ne::editor::edit_file_with_session(&path, restored) {
        Ok((_, captured)) => {
            SessionState::write_global().editor_session = captured;
            SessionState::save_global();
        }
        Err(e) => print_error(&t!("misc.ne.editor_error", e)),
    }
}

pub(crate) fn cmd_ne(args: &str, nav: &mut Navigator) -> Result<bool> {
    // Refuse nested editors inside the editor's embedded ntc shell
    // (spawned with `NTC_IN_EDITOR=1`).
    if std::env::var("NTC_IN_EDITOR").is_ok() {
        print_error(&t!("misc.ne.already_in_editor"));
        return Ok(false);
    }
    match parse_ne_action(args) {
        NeAction::Welcome => {
            let cwd = nav.current_path();
            let welcome = cwd.join("welcome.ntcEditor");
            let _ = crate::shell::commands::builtin::ne::editor::refresh_welcome_file(&welcome);
            print_info(&t!("misc.ne.opened_welcome", nav.current_path().display()));
            launch_ne_editor(welcome, None);
        }
        NeAction::Open { path, line } => {
            if !path.exists() {
                print_error(&t!("misc.ne.file_missing", path.display()));
                return Ok(false);
            }
            launch_ne_editor(path, line);
        }
        NeAction::Create {
            kind,
            targets,
            open,
        } => {
            for target in &targets {
                if target.exists() {
                    print_warning(&t!("misc.mkf.exists", target.display()));
                    continue;
                }
                let created = match kind {
                    NeCreateKind::Init => {
                        crate::shell::commands::builtin::ne::editor::init_file(target)
                    }
                    NeCreateKind::Super => {
                        crate::shell::commands::builtin::ne::editor::init_super_file(target)
                    }
                };
                match created {
                    Ok(true) => {
                        let msg = match kind {
                            NeCreateKind::Init => {
                                t!("misc.ne.created_template", target.display())
                            }
                            NeCreateKind::Super => {
                                t!("misc.ne.created_super", target.display())
                            }
                        };
                        eprintln!("{}", msg);
                    }
                    Ok(false) => {
                        print_error(&t!("misc.ne.create_failed", target.display()));
                    }
                    Err(e) => {
                        print_error(&t!(
                            "misc.ne.create_failed",
                            format!("{}: {}", target.display(), e)
                        ));
                    }
                }
            }
            if let Some(idx) = open {
                if let Some(target) = targets.get(idx) {
                    if target.exists() {
                        launch_ne_editor(target.clone(), None);
                    }
                }
            }
        }
        NeAction::Error(err) => match err {
            NeError::MissingCreateTarget => print_error(&t!("misc.ne.missing_file")),
            NeError::UsageOpen => print_error(&t!("misc.ne.usage_open")),
            NeError::OpenNotCreated(name) => print_error(&t!("misc.ne.open_not_created", name)),
        },
    }
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "ne",
    aliases: &["ntceditor"],
    help_topic: Some("ne"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_ne),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ne_parser_create_modes() {
        assert_eq!(
            parse_ne_action("--init a.py b.html -I a.py"),
            NeAction::Create {
                kind: NeCreateKind::Init,
                targets: vec![PathBuf::from("a.py"), PathBuf::from("b.html")],
                open: Some(0),
            }
        );
        assert_eq!(
            parse_ne_action("--super a.py,b.html -I b.html"),
            NeAction::Create {
                kind: NeCreateKind::Super,
                targets: vec![PathBuf::from("a.py"), PathBuf::from("b.html")],
                open: Some(1),
            }
        );
        assert_eq!(
            parse_ne_action("--init a b"),
            NeAction::Create {
                kind: NeCreateKind::Init,
                targets: vec![PathBuf::from("a"), PathBuf::from("b")],
                open: None,
            }
        );
    }

    #[test]
    fn ne_parser_open_flag_errors() {
        assert_eq!(
            parse_ne_action("--init a b -I"),
            NeAction::Error(NeError::UsageOpen)
        );
        assert_eq!(
            parse_ne_action("--init a b -I a b"),
            NeAction::Error(NeError::UsageOpen)
        );
        assert_eq!(
            parse_ne_action("--init a b -I c"),
            NeAction::Error(NeError::OpenNotCreated("c".to_string()))
        );
        assert_eq!(
            parse_ne_action("--init"),
            NeAction::Error(NeError::MissingCreateTarget)
        );
    }

    #[test]
    fn ne_parser_creates_and_plain_open() {
        assert_eq!(
            parse_ne_action("--super run.sh,run.ps1 -I run.sh"),
            NeAction::Create {
                kind: NeCreateKind::Super,
                targets: vec![PathBuf::from("run.sh"), PathBuf::from("run.ps1")],
                open: Some(0),
            }
        );
        assert_eq!(
            parse_ne_action("--init demo.py -I missing.py"),
            NeAction::Error(NeError::OpenNotCreated("missing.py".to_string()))
        );
        assert_eq!(
            parse_ne_action("src/main.rs:42"),
            NeAction::Open {
                path: PathBuf::from("src/main.rs"),
                line: Some(42),
            }
        );
        assert_eq!(parse_ne_action("."), NeAction::Welcome);
    }

    #[test]
    fn ne_refuses_nested_editor_when_embedded() {
        // Guard must trigger before any editor launch is attempted.
        std::env::set_var("NTC_IN_EDITOR", "1");
        let mut nav = Navigator::new().expect("nav");
        let result = cmd_ne("some_file.txt", &mut nav);
        std::env::remove_var("NTC_IN_EDITOR");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
