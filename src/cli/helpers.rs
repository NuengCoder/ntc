use crate::t;
use anyhow::Result;
use colored::*;

pub(super) fn print_help() {
    println!("{}", "ntc — Navigate, Toolkit, Center".green().bold());
    println!();
    println!("{}", t!("cli.new.usage_header"));
    println!("{}", t!("cli.new.usage_shell"));
    println!("{}", t!("cli.new.usage_cmd"));
    println!("{}", t!("cli.new.usage_new"));
    println!("{}", t!("cli.new.usage_upgrade"));
    println!("{}", t!("cli.new.usage_version"));
    println!("{}", t!("cli.new.usage_help"));
    println!();
    println!("{}", t!("cli.new.examples_header"));
    println!("{}", t!("cli.new.ex_help"));
    println!("{}", t!("cli.new.ex_version"));
    println!("{}", t!("cli.new.ex_alias"));
    println!("{}", t!("cli.new.ex_new"));
    println!("{}", t!("cli.new.ex_new_cmd"));
}

/// Spawn a detached new terminal running this ntc binary with `child_args`.
///
/// When `stay` is true the child gets `NTC_STAY=1` so it stays interactive
/// after running the command (macOS embeds it via `env` since
/// `do script` starts a fresh login shell that drops inherited env).
pub(super) fn spawn_new_terminal(child_args: &[String], stay: bool) -> Result<()> {
    let exe = std::env::current_exe()?;

    #[cfg(windows)]
    {
        let mut cmd = std::process::Command::new("cmd");
        cmd.args(["/c", "start", ""]);
        cmd.arg(&exe);
        for a in child_args {
            cmd.arg(a);
        }
        if stay {
            cmd.env(crate::cli::run::STAY_ENV, "1");
        }
        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        cmd.spawn()?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        let mut tried: Vec<String> = Vec::new();
        // (terminal binary, prefix args before the ntc exe)
        let mut candidates: Vec<(String, Vec<String>)> = Vec::new();
        if let Ok(term) = std::env::var("TERMINAL") {
            if !term.trim().is_empty() {
                candidates.push((term, vec!["-e".to_string()]));
            }
        }
        for (bin, pre) in [
            ("x-terminal-emulator", "-e"),
            ("gnome-terminal", "--"),
            ("konsole", "-e"),
            ("xfce4-terminal", "-e"),
            ("xterm", "-e"),
        ] {
            candidates.push((bin.to_string(), vec![pre.to_string()]));
        }
        for (bin, pre) in &candidates {
            let mut cmd = std::process::Command::new(bin);
            for p in pre {
                cmd.arg(p);
            }
            cmd.arg(&exe);
            for a in child_args {
                cmd.arg(a);
            }
            if stay {
                cmd.env(crate::cli::run::STAY_ENV, "1");
            }
            cmd.stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null());
            match cmd.spawn() {
                Ok(_) => return Ok(()),
                Err(_) => tried.push(bin.clone()),
            }
        }
        anyhow::bail!("{}", t!("cli.new.no_terminal", tried.join(", ")))
    }

    #[cfg(target_os = "macos")]
    {
        fn sh_quote(s: &str) -> String {
            format!("'{}'", s.replace('\'', "'\\''"))
        }
        fn applescript_escape(s: &str) -> String {
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
        let exe_str = exe.to_string_lossy().to_string();
        let mut parts: Vec<String> = Vec::new();
        if stay {
            parts.push(format!(
                "env {}=1 {}",
                crate::cli::run::STAY_ENV,
                sh_quote(&exe_str)
            ));
        } else {
            parts.push(sh_quote(&exe_str));
        }
        for a in child_args {
            parts.push(sh_quote(a));
        }
        let shell_cmd = parts.join(" ");
        let do_script = format!(
            "tell application \"Terminal\" to do script \"{}\"",
            applescript_escape(&shell_cmd)
        );
        std::process::Command::new("osascript")
            .args(["-e", do_script.as_str(), "-e"])
            .arg("tell application \"Terminal\" to activate")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        Ok(())
    }

    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        let _ = (&exe, child_args, stay);
        anyhow::bail!("{}", t!("cli.new.unsupported"))
    }
}
