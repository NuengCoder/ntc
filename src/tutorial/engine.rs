//! The tutorial shell: a guided sandbox session.
//!
//! `tutorial` creates the workspace, navigates into it and switches the shell
//! into tutorial mode. In that mode the learner types the commands; each
//! lesson step only states a goal and checks the result, so the answer is
//! never executed for them. `tutorial hint` reveals one nudge on request.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;

use super::lessons::{self, Check, LessonStep, LESSONS};
use super::render;

/// Commands a learner may run inside the workspace. Everything else needs
/// `tutorial` or a real shell, which keeps the lab (and their machine) safe.
const ALLOWED_IN_LAB: &[&str] = &[
    // shell + navigation
    "tutorial",
    "guide",
    "help",
    "clear",
    "cls",
    "exit",
    "quit",
    "back",
    "cd",
    "where",
    "pwd",
    "fgo",
    "view",
    "viewf",
    "viewd",
    "views",
    "txtf",
    "txtc",
    "ne",
    "nfe",
    "mkf",
    "mkd",
    "rmf",
    "rmd",
    "rnmf",
    "rnmd",
    "mv",
    "move",
    "cp",
    "ral",
    "run",
    "ran",
    "scr",
    "gs",
    "fs",
    "ds",
    "fsc",
    "locate",
    "qur",
    "echo",
    "say",
    "print",
    "sort",
    "rev",
    "head",
    "tail",
    "betw",
    "trim",
    "count",
    "tl",
    "size",
    "report",
    "backup",
    "tp",
    "tpb",
    "theme",
    // configuration + project files
    "config",
    "showcg",
    "opencg",
    "resetcg",
    "restorecg",
    "local",
    "init",
    "deinit",
    "ignore",
    "care",
    "igcare",
    "igcared",
    // network, machine and media tools
    "http",
    "dl",
    "up",
    "dns",
    "ping",
    "trace",
    "smtp",
    "serve",
    "ipcfg",
    "spec",
    "sys",
    "tmprt",
    "time",
    "git",
    "clip",
    "qsc",
    "todo",
    "arc",
    "scan",
    "ntpkg",
    "ntpx",
    "qr",
    "usrpath",
    "syspath",
    "autoup",
    "mode",
    "math",
    "py_activate",
    "py_deactivate",
    "showenv",
    "encode",
    "decode",
    "cvcode",
    "fmt",
    "image",
    "ndb",
    "nsheet",
    "nssh",
    "nms",
    "fake",
    "deadlock",
    "pathcp",
    "wi",
    "ui",
    "version",
];

/// Commands that still work when the learner wanders outside the workspace:
/// only things that bring them back or drop tutorial mode.
const ALLOWED_OUTSIDE: &[&str] = &[
    "tutorial", "guide", "help", "clear", "cls", "exit", "quit", "cd", "back", "where", "pwd",
];

struct State {
    root: PathBuf,
    return_dir: PathBuf,
    lesson: Option<usize>,
    step: usize,
    last_cmd: String,
    last_args: String,
    last_input: String,
    seq: u64,
    evaluated: u64,
}

fn session() -> &'static Mutex<Option<State>> {
    static SESSION: OnceLock<Mutex<Option<State>>> = OnceLock::new();
    SESSION.get_or_init(|| Mutex::new(None))
}

fn lock() -> MutexGuard<'static, Option<State>> {
    session().lock().unwrap_or_else(|e| e.into_inner())
}

pub fn is_active() -> bool {
    lock().is_some()
}

/// Prefix shown in the shell prompt while tutorial mode is on.
pub fn prompt_prefix() -> String {
    let guard = lock();
    let Some(state) = guard.as_ref() else {
        return String::new();
    };
    match state.lesson {
        Some(i) => {
            let lesson = &LESSONS[i];
            t!(
                "tutorial.prompt_fmt",
                lesson.key,
                (state.step + 1).min(lesson.steps.len()),
                lesson.steps.len()
            )
        }
        None => t!("tutorial.prompt_none"),
    }
}

fn inside(path: &Path, root: &Path) -> bool {
    let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    canon(path).starts_with(canon(root))
}

/// Blocked-command check. Returns the message to show when `cmd` may not run.
pub fn restriction(cmd: &str, nav: &crate::navigator::Navigator) -> Option<String> {
    let guard = lock();
    let state = guard.as_ref()?;
    if inside(nav.current_path(), &state.root) {
        if ALLOWED_IN_LAB.contains(&cmd) {
            None
        } else {
            Some(t!("tutorial.blocked_cmd", cmd))
        }
    } else if ALLOWED_OUTSIDE.contains(&cmd) {
        None
    } else {
        Some(t!("tutorial.blocked_outside", state.root.display()))
    }
}

/// Remember the raw line before alias expansion (needed for `ral` lessons).
pub fn note_input(line: &str) {
    let mut guard = lock();
    if let Some(state) = guard.as_mut() {
        state.seq = state.seq.saturating_add(1);
        state.last_input = line.trim().to_string();
        state.last_cmd.clear();
        state.last_args.clear();
    }
}

/// Remember the dispatched command (after expansion).
///
/// Only the first command of a line is kept: `ran hello` runs its target
/// through `execute_command` again, and the lesson needs to see `ran`, not
/// the `echo` the target expanded to.
pub fn note_command(cmd: &str, args: &str) {
    let mut guard = lock();
    if let Some(state) = guard.as_mut() {
        if state.last_cmd.is_empty() {
            state.last_cmd = cmd.to_lowercase();
            state.last_args = args.to_lowercase();
        }
    }
}

/// Entry-point for the `tutorial` command.
pub fn command(args: &str, nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let arg = args.trim();
    let mut parts = arg.splitn(2, ' ');
    let sub = parts.next().unwrap_or("").to_lowercase();
    let rest = parts.next().unwrap_or("").trim();

    match sub.as_str() {
        "help" | "--help" | "-h" => {
            render::usage();
            return Ok(false);
        }
        "exit" | "stop" => {
            leave(nav);
            return Ok(false);
        }
        _ => {}
    }

    let root = lessons::ensure_workspace()?;

    if !is_active() {
        let return_dir = nav.current_path().to_path_buf();
        *lock() = Some(State {
            root: root.clone(),
            return_dir,
            lesson: None,
            step: 0,
            last_cmd: String::new(),
            last_args: String::new(),
            last_input: String::new(),
            seq: 0,
            evaluated: 0,
        });
        nav.go_to(&root)?;
        render::mode_on(&root);
    } else {
        // Menu/lesson commands should work from anywhere: pull the learner
        // back into the workspace first (hint/skip/status stay put).
        if !inside(nav.current_path(), &root)
            && !matches!(sub.as_str(), "hint" | "skip" | "status" | "restart")
        {
            nav.go_to(&root)?;
        }
    }

    match sub.as_str() {
        "" | "list" | "ls" | "menu" | "lessons" => {
            render::menu();
        }
        "hint" => hint(),
        "skip" => skip(),
        "status" => status(),
        "restart" => restart(nav),
        "lesson" => {
            if rest.is_empty() {
                render::menu();
            } else {
                start_lesson(rest, &root, nav)?;
            }
        }
        _ => start_lesson(&sub, &root, nav)?,
    }

    Ok(false)
}

fn start_lesson(token: &str, root: &Path, nav: &mut crate::navigator::Navigator) -> Result<()> {
    let Some(index) = lessons::lesson_by_token(token) else {
        print_error(&t!(
            "tutorial.unknown_lesson",
            token,
            lessons::lesson_keys()
        ));
        render::menu();
        return Ok(());
    };

    lessons::reset_lesson(root, index)?;
    let lesson = &LESSONS[index];
    nav.go_to(&root.join(lesson.key))?;

    if let Some(state) = lock().as_mut() {
        state.lesson = Some(index);
        state.step = 0;
        // The `tutorial <lesson>` line itself must not satisfy step 1.
        state.evaluated = state.seq;
    }

    render::lesson(index);
    render::show_step(index, 0);
    Ok(())
}

fn hint() {
    let guard = lock();
    let Some(state) = guard.as_ref() else {
        print_info(&t!("tutorial.not_active"));
        return;
    };
    let Some(index) = state.lesson else {
        print_info(&t!("tutorial.not_in_lesson"));
        render::menu();
        return;
    };
    let lesson = &LESSONS[index];
    if state.step >= lesson.steps.len() {
        print_info(&t!("tutorial.already_done"));
        return;
    }
    let step = &lesson.steps[state.step];
    print_info(&t!("tutorial.hint_label"));
    println!("  {}", step.hint);
}

fn skip() {
    let mut guard = lock();
    let Some(state) = guard.as_mut() else {
        print_info(&t!("tutorial.not_active"));
        return;
    };
    let Some(index) = state.lesson else {
        print_info(&t!("tutorial.not_in_lesson"));
        return;
    };
    let lesson = &LESSONS[index];
    if state.step >= lesson.steps.len() {
        print_info(&t!("tutorial.already_done"));
        return;
    }
    print_warning(&t!("tutorial.skipped"));
    state.step += 1;
    if state.step < lesson.steps.len() {
        render::show_step(index, state.step);
    } else {
        render::lesson_done(lesson.key);
    }
}

fn status() {
    let guard = lock();
    let Some(state) = guard.as_ref() else {
        print_info(&t!("tutorial.not_active"));
        return;
    };
    let Some(index) = state.lesson else {
        print_info(&t!("tutorial.not_in_lesson"));
        return;
    };
    let lesson = &LESSONS[index];
    print_info(&t!(
        "tutorial.status_fmt",
        lesson.key,
        (state.step + 1).min(lesson.steps.len()),
        lesson.steps.len()
    ));
}

fn restart(nav: &mut crate::navigator::Navigator) {
    let (index, root) = {
        let guard = lock();
        let Some(state) = guard.as_ref() else {
            print_info(&t!("tutorial.not_active"));
            return;
        };
        let Some(index) = state.lesson else {
            print_info(&t!("tutorial.not_in_lesson"));
            return;
        };
        (index, state.root.clone())
    };
    let _ = lessons::reset_lesson(&root, index);
    let lesson = &LESSONS[index];
    let _ = nav.go_to(&root.join(lesson.key));
    if let Some(state) = lock().as_mut() {
        state.step = 0;
        state.evaluated = state.seq;
    }
    render::lesson(index);
    render::show_step(index, 0);
}

fn leave(nav: &mut crate::navigator::Navigator) {
    let taken = lock().take();
    match taken {
        Some(state) => {
            let _ = nav.go_to(&state.return_dir);
            print_success(&t!("tutorial.mode_off", state.return_dir.display()));
        }
        None => print_info(&t!("tutorial.not_active")),
    }
}

/// Called by the shell after every executed line.
pub fn after_command(nav: &crate::navigator::Navigator) {
    let mut guard = lock();
    let Some(state) = guard.as_mut() else {
        return;
    };
    if state.seq <= state.evaluated {
        return;
    }
    state.evaluated = state.seq;

    let Some(index) = state.lesson else {
        return;
    };
    let lesson = &LESSONS[index];
    if state.step >= lesson.steps.len() || !inside(nav.current_path(), &state.root) {
        return;
    }

    let step = &lesson.steps[state.step];
    let cwd = nav.current_path();
    if check_met(step, cwd, &state.root, state) {
        print_success(&t!("tutorial.step_done"));
        render::also_try(step.also_try);
        state.step += 1;
        if state.step < lesson.steps.len() {
            render::show_step(index, state.step);
        } else {
            render::lesson_done(lesson.key);
        }
    }
}

fn check_met(step: &LessonStep, cwd: &Path, root: &Path, state: &State) -> bool {
    match step.check {
        Check::InDir(rel) => {
            let target = root.join(rel);
            let canon = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
            canon(cwd) == canon(&target)
        }
        Check::Typed { cmd, contains } => {
            state.last_cmd == cmd && state.last_args.contains(&contains.to_lowercase())
        }
        Check::TypedCmd(cmd) => state.last_cmd == cmd,
        Check::InputStarts(prefix) => state.last_input.to_lowercase().starts_with(prefix),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(cmd: &str, args: &str, input: &str) -> State {
        State {
            root: PathBuf::from("/tutorial"),
            return_dir: PathBuf::from("/"),
            lesson: Some(0),
            step: 0,
            last_cmd: cmd.into(),
            last_args: args.into(),
            last_input: input.into(),
            seq: 1,
            evaluated: 0,
        }
    }

    #[test]
    fn typed_checks_look_at_the_command_and_its_arguments() {
        let step = &LESSONS[0].steps[2];
        assert!(check_met(
            step,
            Path::new("/tutorial/cd"),
            Path::new("/tutorial"),
            &state("where", "", "where")
        ));
        assert!(!check_met(
            step,
            Path::new("/tutorial/cd"),
            Path::new("/tutorial"),
            &state("view", "notes.txt", "view notes.txt")
        ));
    }

    #[test]
    fn view_lesson_needs_the_flag_and_the_file() {
        let step = &LESSONS[1].steps[1];
        assert!(check_met(
            step,
            Path::new("/tutorial/view"),
            Path::new("/tutorial"),
            &state("view", "-d 2 ..", "view -d 2 ..")
        ));
        assert!(!check_met(
            step,
            Path::new("/tutorial/view"),
            Path::new("/tutorial"),
            &state("view", "notes.txt", "view notes.txt")
        ));
    }

    #[test]
    fn aliases_are_detected_from_the_raw_line() {
        let step = &LESSONS[3].steps[1];
        assert!(check_met(
            step,
            Path::new("/tutorial/ral"),
            Path::new("/tutorial"),
            &state("echo", "hello", "tuthello")
        ));
        assert!(!check_met(
            step,
            Path::new("/tutorial/ral"),
            Path::new("/tutorial"),
            &state("echo", "hello", "ral list")
        ));
    }

    #[test]
    fn lab_commands_are_allowlisted() {
        assert!(ALLOWED_IN_LAB.contains(&"view"));
        assert!(ALLOWED_IN_LAB.contains(&"ral"));
        // Every ntc command has a lesson, so the lab has to allow it. The raw
        // shell `rm` and the folder-hopping pickers stay blocked, and only the
        // way-back commands work outside the workspace.
        assert!(!ALLOWED_IN_LAB.contains(&"rm"));
        assert!(!ALLOWED_IN_LAB.contains(&"gos"));
        assert!(!ALLOWED_IN_LAB.contains(&"godrive"));
        assert!(ALLOWED_OUTSIDE.contains(&"cd"));
        assert!(!ALLOWED_OUTSIDE.contains(&"view"));
    }

    /// Every command the tutorial prints must be runnable in the lab — it is
    /// the answer the learner is told to type, so a blocked one would strand
    /// them. An alias the same lesson creates is allowed: typing it expands to
    /// a lab command before the restriction check runs.
    #[test]
    fn every_step_command_stays_inside_the_lab() {
        for lesson in LESSONS {
            let aliases: Vec<&str> = lesson
                .steps
                .iter()
                .filter_map(|s| {
                    let mut parts = s.command.split_whitespace();
                    match (parts.next(), parts.next()) {
                        (Some("ral"), Some("add")) => parts.next(),
                        _ => None,
                    }
                })
                .collect();
            for (i, step) in lesson.steps.iter().enumerate() {
                let cmd = step.command.split_whitespace().next().unwrap_or_default();
                assert!(
                    ALLOWED_IN_LAB.contains(&cmd) || aliases.contains(&cmd),
                    "{} step {}: `{}` is blocked in the lab",
                    lesson.key,
                    i + 1,
                    step.command
                );
            }
        }
    }

    /// The "Also try:" line must be runnable inside the lab, and typing it
    /// must never be the way to pass the next step.
    #[test]
    fn also_try_suggestions_stay_inside_the_lab() {
        for lesson in LESSONS {
            for (i, step) in lesson.steps.iter().enumerate() {
                let cmd = step.also_try.split(' ').next().unwrap_or_default();
                assert!(
                    ALLOWED_IN_LAB.contains(&cmd),
                    "{} step {}: `{}` is blocked",
                    lesson.key,
                    i + 1,
                    step.also_try
                );
                if let Some(next) = lesson.steps.get(i + 1) {
                    assert_ne!(
                        step.also_try,
                        next.command,
                        "{} step {}: suggestion is the next answer",
                        lesson.key,
                        i + 1
                    );
                    if let Check::Typed { cmd, contains } = &next.check {
                        assert!(
                            !(step.also_try.starts_with(cmd) && step.also_try.contains(contains)),
                            "{} step {}: `{}` would pass step {}",
                            lesson.key,
                            i + 1,
                            step.also_try,
                            i + 2
                        );
                    }
                }
            }
        }
    }
}
