use crate::output::{print_error, print_info, print_warning};
use crate::shell::commands::builtin::nssh::nssh::ssh as transport;
use crate::t;
use anyhow::Result;
use rpassword;
use std::time::Duration;

use super::auth::{build_auth, is_auth_failure, rt};
use super::profiles::resolve_or_pick;

/// Maximum total SSH authentication attempts (env password + prompted retries).
const MAX_AUTH_ATTEMPTS: u32 = 3;

pub(super) fn cmd_ssh(tokens: &[String]) -> Result<bool> {
    // nssh ssh [name] [--cmd "..."] [--trust] [--ask-pass] [--timeout <s>]
    let mut name: Option<String> = None;
    let mut cmd: Option<String> = None;
    let mut trust = false;
    let mut _ask_pass = false;
    let mut timeout = Duration::from_secs(30);
    let mut i = 1;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "--cmd" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_cmd")))?;
                cmd = Some(v.clone());
                i += 2;
            }
            "--trust" => {
                trust = true;
                i += 1;
            }
            "--ask-pass" => {
                _ask_pass = true;
                i += 1;
            }
            "--timeout" => {
                let v = tokens
                    .get(i + 1)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.nssh.missing_timeout")))?;
                let s: u64 = v
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.nssh.invalid_timeout", v)))?;
                timeout = Duration::from_secs(s.max(1));
                i += 2;
            }
            _ => {
                if tokens[i].starts_with('-') {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                if name.is_some() {
                    print_error(&t!("cmd.nssh.unknown_option", tokens[i]));
                    return Ok(false);
                }
                name = Some(tokens[i].clone());
                i += 1;
            }
        }
    }
    let Some((pname, profile)) = resolve_or_pick(name.as_deref())? else {
        return Ok(false);
    };
    let _ = pname;
    let auth = build_auth(&profile)?;
    let key_path = auth.key_path;
    let initial_password = auth.password;

    print_info(&t!(
        "cmd.nssh.ssh_connecting",
        profile.user,
        profile.host,
        profile.port.to_string()
    ));
    let runtime = rt()?;
    let mut password = initial_password.clone();
    let connect = |password: Option<&str>| {
        runtime.block_on(transport::connect(
            &profile.host,
            profile.port,
            &profile.user,
            &transport::SshAuth {
                key_path: key_path.clone(),
                password: password.map(|v| v.to_string()),
            },
            trust,
            timeout,
        ))
    };

    for attempt in 1..=MAX_AUTH_ATTEMPTS {
        match connect(password.as_deref()) {
            Ok(conn) => {
                if let Some(c) = cmd {
                    let code = runtime.block_on(transport::exec(&conn, &c))?;
                    if code != 0 {
                        print_warning(&t!("cmd.nssh.cmd_exit", code.to_string()));
                    }
                } else {
                    print_info(&t!("cmd.nssh.shell_hint"));
                    match runtime.block_on(transport::shell(&conn))? {
                        transport::ShellEnd::Forced => print_warning(&t!("cmd.nssh.shell_forced")),
                        transport::ShellEnd::Closed => {}
                    }
                    print_info(&t!("cmd.nssh.shell_back"));
                }
                let _ = runtime.block_on(conn.handle.disconnect(
                    russh::Disconnect::ByApplication,
                    "nssh done",
                    "",
                ));
                return Ok(false);
            }
            Err(e) => {
                if !is_auth_failure(&e) || attempt == MAX_AUTH_ATTEMPTS {
                    print_error(&format!("{}", e));
                    print_info(&t!("cmd.nssh.shell_back"));
                    return Ok(false);
                }
                let prompt = if attempt == 1 && initial_password.is_none() {
                    t!("cmd.nssh.password_prompt", profile.user, profile.host)
                } else {
                    t!("cmd.nssh.password_incorrect", profile.user, profile.host)
                };
                match rpassword::prompt_password(prompt) {
                    Ok(pwd) if !pwd.is_empty() => password = Some(pwd),
                    _ => {
                        print_info(&t!("cmd.nssh.shell_back"));
                        return Ok(false);
                    }
                }
            }
        }
    }

    unreachable!()
}
