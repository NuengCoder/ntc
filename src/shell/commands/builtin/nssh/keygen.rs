use crate::output::print_warning;
use crate::t;
use anyhow::Result;

use super::tokenize;

pub(super) fn cmd_keygen(args: &str) -> Result<bool> {
    let status = std::process::Command::new("ssh-keygen")
        .args(tokenize(args))
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.nssh.keygen_missing", e)))?
        .wait()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    if !status.success() {
        print_warning(&t!(
            "cmd.nssh.keygen_exit",
            status.code().unwrap_or(-1).to_string()
        ));
    }
    Ok(false)
}
