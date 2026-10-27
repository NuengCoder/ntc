use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.clipboard.access_fail".into(),
        "Failed to access clipboard: {}".into(),
    );

    m.insert(
        "cmd.clipboard.already_empty".into(),
        "Clipboard history is already empty.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_error".into(),
        "Failed to clear system clipboard: {}.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_cancelled".into(),
        "Clear cancelled.".into(),
    );

    m.insert(
        "cmd.clipboard.clear_prompt".into(),
        "Clear clipboard history ({} entries)? [y/N] ".into(),
    );

    m.insert(
        "cmd.clipboard.cleared".into(),
        "Clipboard history cleared.".into(),
    );

    m.insert(
        "cmd.clipboard.copied_piped".into(),
        "Copied piped input to clipboard.".into(),
    );

    m.insert(
        "cmd.clipboard.copied_entry".into(),
        "Copied entry #{} ({}) to clipboard.".into(),
    );

    m.insert(
        "cmd.clipboard.empty".into(),
        "Clipboard history is empty.".into(),
    );

    m.insert(
        "cmd.clipboard.entry_not_found".into(),
        "Entry #{} not found (have {} entries).".into(),
    );

    m.insert(
        "cmd.clipboard.help_clear".into(),
        "  clip clear         Clear clipboard history".into(),
    );

    m.insert(
        "cmd.clipboard.help_list".into(),
        "  clip               Show clipboard history".into(),
    );

    m.insert(
        "cmd.clipboard.help_select".into(),
        "  clip select <N>    Copy entry N back to clipboard".into(),
    );

    m.insert(
        "cmd.clipboard.history_header".into(),
        "  Clipboard History (last {}):".into(),
    );

    m.insert(
        "cmd.clipboard.select_usage".into(),
        "Usage: clip select <N>".into(),
    );

    m.insert(
        "cmd.clipboard.set_fail".into(),
        "Failed to set clipboard: {}".into(),
    );

    m.insert(
        "cmd.clipboard.termux_unavailable".into(),
        "termux-clipboard-set not available: {}".into(),
    );

    m.insert(
        "cmd.clipboard.unknown_sub".into(),
        "Unknown clip subcommand: {}".into(),
    );
}
