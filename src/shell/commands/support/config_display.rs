//! Shared formatting helpers for config/showcg output.

use crate::config::{Config, ServeService};
use crate::utils::theme::style::ThemeColorize;

pub(crate) fn on_off(state: bool) -> &'static str {
    if state {
        "ON"
    } else {
        "OFF"
    }
}

/// One `showcg` row: label on the left, `bind:port` on the right.
pub(crate) fn print_serve_row(label: &str, svc: ServeService) {
    println!(
        "│ {:<20} {:<42}",
        label,
        format!(
            "{}:{}",
            Config::global_get_serve_bind(svc),
            Config::global_get_serve_port(svc)
        )
        .cyan()
    );
}
