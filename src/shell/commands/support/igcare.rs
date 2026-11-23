//! Shared ignore/care write helpers.

use crate::config::{Config, ConfigScope};
use crate::output::{print_info, print_success};
use crate::t;
use anyhow::Result;

pub(crate) fn apply_igcare(
    args: &str,
    usage: &str,
    action: fn(&str, ConfigScope) -> std::result::Result<(), Box<dyn std::error::Error>>,
    action_desc: &str,
    scope: ConfigScope,
) -> Result<bool> {
    if args.is_empty() {
        println!("{}", usage);
    } else {
        let mut count = 0;
        for item in args.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if action(item, scope).is_ok() {
                count += 1;
            }
        }
        Config::reload_global();
        if count > 0 {
            print_success(&t!("cmd.ignore.applied", action_desc, count));
            print_info(&t!("cmd.ignore.scope_target", scope_label(scope)));
            print_info(&t!("cmd.ignore.check_hint"));
        }
    }
    Ok(false)
}

/// Human label of the config store a write targets.
fn scope_label(scope: ConfigScope) -> String {
    if scope.is_local() {
        t!("cmd.ignore.scope_local").to_string()
    } else {
        t!("cmd.ignore.scope_global").to_string()
    }
}
