//! `sys` — interactive process/system monitor (alias `top`).
//!
//! State and key handling live in [`monitor`], terminal painting in
//! [`render`]; this module owns the event loop and the registry entry.

pub(crate) mod help;

mod completion;
mod logging;
mod monitor;
mod render;
#[cfg(test)]
mod tests;

use anyhow::{bail, Context, Result};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::stdout;
use std::time::{Duration, Instant};

use self::logging::{DEFAULT_DELAY, DEFAULT_INTERVAL_SECS};

use self::monitor::{kill_process, SysAction, SysMon, REFRESH_MS};
use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            DisableMouseCapture,
            Show,
            LeaveAlternateScreen
        );
    }
}

pub fn cmd_sys(args: &str, nav: &mut crate::navigator::Navigator) -> Result<bool> {
    let log_interval = parse_log_flags(args)?;
    let mut sm = SysMon::new();
    if let Some(interval) = log_interval {
        let cwd = nav.current_path().to_path_buf();
        sm.start_logging(&cwd, interval).with_context(|| {
            format!(
                "failed to open {}/{} in {}",
                logging::LOG_DIR,
                logging::LOG_FILE,
                cwd.display()
            )
        })?;
        sm.flash(format!(
            "logging to {}/{} every {}s",
            logging::LOG_DIR,
            logging::LOG_FILE,
            interval.as_secs()
        ));
    }
    let mut stdout = stdout();
    let _guard = TerminalGuard;

    execute!(stdout, EnterAlternateScreen, Hide, EnableMouseCapture)?;
    execute!(stdout, Clear(ClearType::All))?;
    terminal::enable_raw_mode()?;

    sm.render(&mut stdout)?;
    let mut last_refresh = Instant::now();

    loop {
        let poll_ms = REFRESH_MS.min(200);
        if !event::poll(Duration::from_millis(poll_ms))? {
            // Logging can be due between UI refreshes; refresh early so each
            // interval gets a current snapshot instead of the previous one.
            if sm.log_due() || last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
                sm.refresh();
                sm.render(&mut stdout)?;
                last_refresh = Instant::now();
            }
            continue;
        }

        let ev = event::read()?;
        let action = match ev {
            Event::Key(ke) => {
                if ke.kind != KeyEventKind::Press {
                    SysAction::None
                } else {
                    sm.handle_key(ke)
                }
            }
            Event::Resize(w, h) => {
                sm.term_w = w as usize;
                sm.term_h = h as usize;
                SysAction::Render
            }
            Event::Mouse(m) => match m.kind {
                MouseEventKind::ScrollUp => sm.move_selection(false),
                MouseEventKind::ScrollDown => sm.move_selection(true),
                _ => SysAction::None,
            },
            _ => SysAction::None,
        };

        match action {
            SysAction::None => {}
            SysAction::Render => sm.render(&mut stdout)?,
            SysAction::Refresh => {
                sm.refresh();
                sm.render(&mut stdout)?;
                last_refresh = Instant::now();
            }
            SysAction::Kill(pid) => {
                kill_process(pid);
                sm.forget_pid(pid);
                sm.flash(format!("killed PID {pid}"));
                sm.render(&mut stdout)?;
            }
            SysAction::Quit => break,
        }

        if last_refresh.elapsed().as_millis() as u64 >= REFRESH_MS {
            sm.refresh();
            sm.render(&mut stdout)?;
            last_refresh = Instant::now();
        }
    }

    Ok(false)
}

/// Parse `sys [-l|--log] [-d|--delay [seconds]]`.
///
/// `--log` is a switch and takes no value; the write interval comes from
/// `--delay`, whose own value defaults to -1 (the built-in interval) when it is
/// omitted. Short flags may be combined, so `-ld 5` is `-l -d 5`. Returns
/// `None` when logging was not requested.
pub(super) fn parse_log_flags(args: &str) -> Result<Option<Duration>> {
    let mut words = args.split_whitespace().peekable();
    let mut log = false;
    let mut delay: Option<i64> = None;

    while let Some(arg) = words.next() {
        match arg {
            "-l" | "--log" => log = true,
            "-d" | "--delay" => delay = Some(take_delay(&mut words)?),
            _ if arg.starts_with("--log=") || arg.starts_with("-l=") => bail!(
                "'{}' takes no value: pass the interval with -d/--delay <seconds>",
                arg.split('=').next().unwrap_or(arg)
            ),
            _ if arg.starts_with("--delay=") => delay = Some(parse_seconds_arg(&arg[8..])?),
            _ if arg.starts_with("-d=") => delay = Some(parse_seconds_arg(&arg[3..])?),
            _ if arg.len() > 2 && arg.starts_with('-') && !arg.starts_with("--") => {
                let flags = &arg[1..];
                for (i, flag) in flags.char_indices() {
                    match flag {
                        'l' => log = true,
                        'd' => {
                            let rest = &flags[i + 1..];
                            if let Ok(secs) = rest.parse::<i64>() {
                                delay = Some(secs);
                                break;
                            }
                            // No number inside the cluster: take the next word
                            // when it is one (`-ld 5`, `-dl 5`), else default.
                            delay = Some(take_delay(&mut words)?);
                            if !rest.is_empty() {
                                continue; // `-dl`: keep scanning the flags
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if !log {
        return Ok(None);
    }
    let secs = delay.unwrap_or(DEFAULT_DELAY);
    let secs = if secs < 0 {
        DEFAULT_INTERVAL_SECS
    } else {
        secs as u64
    };
    if secs == 0 {
        bail!("log delay must be at least 1 second");
    }
    Ok(Some(Duration::from_secs(secs)))
}

fn parse_seconds(token: &str) -> Option<i64> {
    token.parse::<i64>().ok()
}

/// Value for `-d`/`--delay`: the next word when it is a number, the default
/// when it looks like another flag, and an error when it is neither.
fn take_delay(words: &mut std::iter::Peekable<std::str::SplitWhitespace<'_>>) -> Result<i64> {
    let Some(next) = words.peek().copied() else {
        return Ok(DEFAULT_DELAY);
    };
    if let Some(secs) = parse_seconds(next) {
        words.next();
        return Ok(secs);
    }
    if next.starts_with('-') {
        return Ok(DEFAULT_DELAY);
    }
    bail!("invalid delay '{next}': expected a whole number of seconds")
}

fn parse_seconds_arg(raw: &str) -> Result<i64> {
    raw.parse::<i64>()
        .with_context(|| format!("invalid delay '{raw}': expected a whole number of seconds"))
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "sys",
    aliases: &["top"],
    help_topic: Some("sys"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_sys),
};
