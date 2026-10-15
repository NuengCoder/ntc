//! Language/localization system for ntc.
//! Provides a global language pack loaded from built-in English defaults
//! with optional external `.ntc_lang` overrides from `~/.config/ntc/lang/`.

pub mod en;
pub mod th;
pub mod lo;
pub mod zh;

use std::collections::HashMap;
use std::sync::RwLock;

/// Macro to get a localized string by dot-notation key.
///
/// # Examples
/// ```
/// t!("help.nav.title")           // -> String
/// t!("config.lang.set", "thai")  // -> String with format arg
/// ```
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::utils::config::lang::get($key)
    };
    ($key:expr $(, $arg:expr)* $(,)?) => {
        $crate::utils::config::lang::format_get($key, &[$($arg.to_string()),*])
    };
}

/// Flatten nested TOML tables into dot-notation keys, merging into `map`.
fn flatten_toml(value: &toml::Value, prefix: &str, map: &mut HashMap<String, String>) {
    match value {
        toml::Value::Table(table) => {
            for (k, v) in table {
                let new_prefix = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten_toml(v, &new_prefix, map);
            }
        }
        toml::Value::String(s) => {
            map.insert(prefix.to_string(), s.clone());
        }
        _ => {}
    }
}

/// Load a language pack: start with built-in English defaults,
/// then merge external `.ntc_lang` if available.
pub fn load(code: &str) -> HashMap<String, String> {
    let mut map = match code {
        "chinese" => zh::build_map(),
        "thai" => th::build_map(),
        "lao" => lo::build_map(),
        _ => en::build_map(),
    };

    let lang_dir = dirs::config_dir()
        .map(|d| d.join("ntc").join("lang"));
    if let Some(dir) = lang_dir {
        let lang_file = dir.join(format!("{}.ntc_lang", code));
        if lang_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&lang_file) {
                if let Ok(toml_value) = content.parse::<toml::Value>() {
                    flatten_toml(&toml_value, "", &mut map);
                }
            }
        }
    }

    map
}

/// Global language pack singleton.
static LANG_PACK: std::sync::LazyLock<RwLock<HashMap<String, String>>> =
    std::sync::LazyLock::new(|| RwLock::new(en::build_map()));

/// Directly set the global language pack (no GLOBAL_CONFIG access — safe during boot).
pub fn set_map(map: HashMap<String, String>) {
    if let Ok(mut guard) = LANG_PACK.write() {
        *guard = map;
    }
}

/// Re-initialize the global language pack from the current config setting.
pub fn reload() {
    let code = crate::utils::config::Config::global_get_language();
    let map = load(&code);
    set_map(map);
}

/// Get a localized string by key. Falls back to the key itself if not found.
pub fn get(key: &str) -> String {
    if let Ok(guard) = LANG_PACK.read() {
        guard
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    } else {
        key.to_string()
    }
}

/// Get a localized string with runtime `{}` substitution.
/// Replaces `{}` in the template with the given args in order.
pub fn format_get(key: &str, args: &[String]) -> String {
    let template = get(key);
    if args.is_empty() {
        return template;
    }
    let mut result = String::with_capacity(template.len());
    let mut rest = template.as_str();
    for arg in args {
        if let Some(pos) = rest.find("{}") {
            result.push_str(&rest[..pos]);
            result.push_str(arg);
            rest = &rest[pos + 2..];
        } else {
            result.push_str(rest);
            rest = "";
            break;
        }
    }
    result.push_str(rest);
    result
}

/// List all available language codes: built-in + external `.ntc_lang` files.
pub fn available_languages() -> Vec<String> {
    let mut langs: Vec<String> = vec![
        "english".to_string(),
        "thai".to_string(),
        "lao".to_string(),
        "chinese".to_string(),
    ];

    if let Some(dir) = dirs::config_dir().map(|d| d.join("ntc").join("lang")) {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |e| e == "ntc_lang") {
                        if let Some(stem) = path.file_stem() {
                            let code = stem.to_string_lossy().to_string();
                            if !langs.contains(&code) {
                                langs.push(code);
                            }
                        }
                    }
                }
            }
        }
    }

    langs
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// Keys referenced by the Termux / npm-guide / cloudflared features.
    /// Every key must exist in ALL four bundled language packs so that
    /// non-English users never see a raw key string.
    const REQUIRED_KEYS: &[&str] = &[
        // Termux pkg recommendation
        "ntpkg.termux_use_pkg",
        "ntpkg.termux_cmd",
        "ntpkg.termux_no_pkg",
        // npm guide
        "ntpkg.npm_section_node_first",
        "ntpkg.npm_node_found_at",
        "ntpkg.npm_node_not_found",
        "ntpkg.npm_install_via_pkg",
        "ntpkg.npm_install_via_ntpkg",
        "ntpkg.npm_ntpkg_cmd",
        "ntpkg.npm_cmd",
        "ntpkg.npm_or_download_nodejs",
        "ntpkg.npm_or_fnm_nvm_volta",
        "ntpkg.npm_found_at",
        "ntpkg.npm_not_found",
        "ntpkg.npm_then_run",
        "ntpkg.npm_then_rerun",
        "ntpkg.npm_bin_not_in_path",
        "ntpkg.path_add_label",
        "ntpkg.npm_install_section",
        "ntpkg.npm_run_cmd_label",
        // cloudflared tunnel
        "cmd.serve.cloudflared_starting",
        "cmd.serve.cloudflared_missing",
        // contextual --help <cr> hint
        "help.context.cr",
    ];

    fn assert_all_present(map: HashMap<String, String>, lang: &str) {
        assert_keys_present(map, lang, REQUIRED_KEYS);
    }

    fn assert_keys_present(map: HashMap<String, String>, lang: &str, keys: &[&str]) {
        let missing: Vec<&str> = keys
            .iter()
            .copied()
            .filter(|k| !map.contains_key(*k))
            .collect();
        assert!(
            missing.is_empty(),
            "{lang} pack is missing keys: {:?}",
            missing
        );
    }

    #[test]
    fn termux_and_cloudflared_keys_exist_in_all_languages() {
        assert_all_present(super::en::build_map(), "english");
        assert_all_present(super::zh::build_map(), "chinese");
        assert_all_present(super::th::build_map(), "thai");
        assert_all_present(super::lo::build_map(), "lao");
    }

    /// Every `scr` script-runner key must exist in all four bundled language packs.
    const SCR_KEYS: &[&str] = &[
        "help.scr.continue",
        "help.scr.example1",
        "help.scr.example2",
        "help.scr.example3",
        "help.scr.help",
        "help.scr.init",
        "help.scr.ls",
        "help.scr.run",
        "help.scr.title",
        "scr.calling_not_allowed",
        "scr.cmd_failed",
        "scr.completed",
        "scr.continue_warn",
        "scr.desc_args",
        "scr.desc_force",
        "scr.desc_interp",
        "scr.desc_set",
        "scr.done_errors",
        "scr.error_at",
        "scr.executing",
        "scr.failed_abort",
        "scr.help_continue",
        "scr.help_help",
        "scr.help_init",
        "scr.help_ls",
        "scr.help_run",
        "scr.help_title",
        "scr.help_usage",
        "scr.init_created",
        "scr.init_exists",
        "scr.ls_empty",
        "scr.ls_title",
        "scr.no_commands",
        "scr.not_found",
        "scr.start",
        "scr.syscmd",
        "scr.syscmd_exit",
        "scr.syscmd_terminated",
    ];

    #[test]
    fn scr_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SCR_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SCR_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SCR_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SCR_KEYS);
    }

    /// Every `git` integration key must exist in all four bundled language packs.
    const GIT_KEYS: &[&str] = &[
        "git.ahead",
        "git.behind",
        "git.branch_failed",
        "git.branch_header",
        "git.checking_status",
        "git.clean",
        "git.completed",
        "git.detached_header",
        "git.executing",
        "git.exit_code",
        "git.failed",
        "git.help_branch",
        "git.help_diff",
        "git.help_example1",
        "git.help_example2",
        "git.help_log",
        "git.help_passthrough",
        "git.help_status",
        "git.help_title",
        "git.examples_header",
        "git.log_empty",
        "git.log_failed",
        "git.no_git_binary",
        "git.not_repo",
        "git.section_conflicts",
        "git.section_staged",
        "git.section_unstaged",
        "git.section_untracked",
        "git.status_failed",
        "git.terminated",
        "git.upstream",
        "editor.git_disabled",
        "editor.git_no_blame",
        "editor.git_no_diff",
        "editor.git_quit_hint",
        "editor.mode_git_diff",
        "help.config.git",
        "help.git.branch",
        "help.git.diff",
        "help.git.example1",
        "help.git.example2",
        "help.git.example3",
        "help.git.log",
        "help.git.passthrough",
        "help.git.status",
        "help.git.title",
        "msg.setGit.error",
        "msg.setGit.off",
        "msg.setGit.on",
        "msg.setGit.status",
    ];

    #[test]
    fn git_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", GIT_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", GIT_KEYS);
        assert_keys_present(super::th::build_map(), "thai", GIT_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", GIT_KEYS);
    }

    /// Every `dir_view` config key must exist in all four bundled language packs.
    const DIR_VIEW_KEYS: &[&str] = &[
        "help.config.dir_view",
        "msg.setDV.status",
        "msg.setDV.on",
        "msg.setDV.off",
        "msg.setDV.error",
        "msg.showcg.label_dir_view",
    ];

    #[test]
    fn dir_view_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", DIR_VIEW_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", DIR_VIEW_KEYS);
        assert_keys_present(super::th::build_map(), "thai", DIR_VIEW_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", DIR_VIEW_KEYS);
    }

    /// Every `ndb` (database shell) key must exist in all four bundled packs.
    const NDB_KEYS: &[&str] = &[
        "ndb.banner",
        "ndb.bye",
        "ndb.closed_db",
        "ndb.col_columns",
        "ndb.col_constraints",
        "ndb.col_from",
        "ndb.col_kind",
        "ndb.col_property",
        "ndb.col_name",
        "ndb.col_rows",
        "ndb.col_sql",
        "ndb.col_table",
        "ndb.col_to",
        "ndb.col_type",
        "ndb.col_unique",
        "ndb.col_value",
        "ndb.dbal_add",
        "ndb.dbal_add_usage",
        "ndb.dbal_added",
        "ndb.dbal_blocked",
        "ndb.dbal_cls",
        "ndb.dbal_cls_usage",
        "ndb.dbal_cleared",
        "ndb.dbal_edit",
        "ndb.dbal_edit_usage",
        "ndb.dbal_exec",
        "ndb.dbal_info",
        "ndb.dbal_info_usage",
        "ndb.dbal_invalid_name",
        "ndb.dbal_list",
        "ndb.dbal_name_rules",
        "ndb.dbal_no_aliases",
        "ndb.dbal_note",
        "ndb.dbal_not_found",
        "ndb.dbal_params",
        "ndb.dbal_single_only",
        "ndb.dbal_removed",
        "ndb.dbal_renamed",
        "ndb.dbal_rm",
        "ndb.dbal_rm_usage",
        "ndb.dbal_rnm",
        "ndb.dbal_rnm_usage",
        "ndb.dbal_run",
        "ndb.dbal_run_usage",
        "ndb.dbal_scope_global",
        "ndb.dbal_scope_local",
        "ndb.dbal_title",
        "ndb.dbal_unknown_sub",
        "ndb.dbal_updated",
        "ndb.columns_count",
        "ndb.cwd_line",
        "ndb.db_files_header",
        "ndb.editor_fail",
        "ndb.help_db_clear",
        "ndb.help_db_exit",
        "ndb.help_db_help",
        "ndb.help_db_indexes",
        "ndb.help_db_info",
        "ndb.help_db_interface",
        "ndb.help_db_more",
        "ndb.help_db_schema",
        "ndb.help_db_sql",
        "ndb.help_db_tablef",
        "ndb.help_db_tables",
        "ndb.help_db_title",
        "ndb.help_db_write",
        "ndb.help_db_edit",
        "ndb.help_db_backup",
        "ndb.help_db_viewf",
        "ndb.help_dbal",
        "ndb.help_limit",
        "ndb.help_note",
        "ndb.help_readonly",
        "ndb.help_root_clear",
        "ndb.help_root_exit",
        "ndb.help_root_help",
        "ndb.help_root_interface",
        "ndb.help_root_sql",
        "ndb.help_root_title",
        "ndb.help_root_view",
        "ndb.fkeys_title",
        "ndb.hint_help",
        "ndb.hint_next",
        "ndb.indexes_title",
        "ndb.info_file",
        "ndb.info_indexes",
        "ndb.info_pages",
        "ndb.info_page",
        "ndb.info_size",
        "ndb.info_sqlite",
        "ndb.info_tables",
        "ndb.info_title",
        "ndb.info_triggers",
        "ndb.info_views",
        "ndb.interface_usage",
        "ndb.kind_table",
        "ndb.kind_view",
        "ndb.more_empty",
        "ndb.multiline_hint",
        "ndb.no",
        "ndb.no_args",
        "ndb.no_columns",
        "ndb.no_db_files",
        "ndb.no_db_files_hint",
        "ndb.no_indexes",
        "ndb.no_query_yet",
        "ndb.no_tables",
        "ndb.not_a_db",
        "ndb.null",
        "ndb.open_failed",
        "ndb.opened",
        "ndb.pick_db",
        "ndb.pick_prompt",
        "ndb.pick_table",
        "ndb.query_failed",
        "ndb.query_time",
        "ndb.rows_count",
        "ndb.rows_empty",
        "ndb.rows_shown",
        "ndb.rows_truncated",
        "ndb.schema_header",
        "ndb.schema_usage",
        "ndb.showing_table",
        "ndb.sql_blocked",
        "ndb.sql_error",
        "ndb.sql_needs_db",
        "ndb.sql_usage",
        "ndb.start_hint",
        "ndb.table_not_found",
        "ndb.tables_count",
        "ndb.triggers_title",
        "ndb.unknown_cmd",
        "ndb.edit_on",
        "ndb.edit_warn",
        "ndb.edit_failed",
        "ndb.edit_off",
        "ndb.edit_status",
        "ndb.edit_state_on",
        "ndb.edit_state_off",
        "ndb.edit_usage",
        "ndb.write_needs_edit",
        "ndb.write_usage",
        "ndb.write_blocked",
        "ndb.write_confirm",
        "ndb.write_confirm_hint",
        "ndb.write_done",
        "ndb.write_failed",
        "ndb.backup_done",
        "ndb.backup_failed",
        "ndb.yes",
        "help.ndb.db_exit",
        "help.ndb.db_schema",
        "help.ndb.db_sql",
        "help.ndb.db_write",
        "help.ndb.db_edit",
        "help.ndb.db_backup",
        "help.ndb.db_tablef",
        "help.ndb.db_tables",
        "help.ndb.db_viewf",
        "help.ndb.dbal",
        "help.ndb.example1",
        "help.ndb.example2",
        "help.ndb.indexes",
        "help.ndb.info",
        "help.ndb.launch",
        "help.ndb.more",
        "help.ndb.readonly",
        "help.ndb.root_exit",
        "help.ndb.root_help",
        "help.ndb.root_interface",
        "help.ndb.root_sql",
        "help.ndb.root_view",
        "help.ndb.title",
    ];

    #[test]
    fn ndb_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", NDB_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", NDB_KEYS);
        assert_keys_present(super::th::build_map(), "thai", NDB_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", NDB_KEYS);
    }

    /// Every `serve --share` key must exist in all four bundled language packs.
    const SHARE_KEYS: &[&str] = &[
        "cmd.serve.help_share",
        "cmd.serve.help_qr",
        "cmd.serve.help_no_qr",
        "cmd.serve.example_share",
        "cmd.serve.share_checking",
        "cmd.serve.share_online",
        "cmd.serve.share_offline",
        "cmd.serve.share_lan_title",
        "cmd.serve.share_path_title",
        "cmd.serve.share_no_lan",
        "cmd.serve.share_bound_loopback",
        "cmd.serve.share_not_private",
        "cmd.serve.share_not_private_hint",
        "cmd.serve.share_qr_fail",
    ];

    #[test]
    fn share_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SHARE_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SHARE_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SHARE_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SHARE_KEYS);
    }

    /// Every `serve chess` key must exist in all four bundled language packs.
    const SERVE_CHESS_KEYS: &[&str] = &[
        "cmd.serve.chess_no_auth",
        "cmd.serve.chess_serving",
        "cmd.serve.chess_title",
        "cmd.serve.chess_usage",
        "cmd.serve.example_chess_default",
        "cmd.serve.example_chess_port",
    ];

    #[test]
    fn serve_chess_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SERVE_CHESS_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SERVE_CHESS_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SERVE_CHESS_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SERVE_CHESS_KEYS);
    }

    /// Every `serve config` / `config serve` key must exist in all four
    /// bundled language packs.
    const SERVE_CONFIG_KEYS: &[&str] = &[
        "help.config.serve",
        "cmd.config.serve.title",
        "cmd.config.serve.status",
        "cmd.config.serve.usage",
        "cmd.config.serve.example_show",
        "cmd.config.serve.example_show_one",
        "cmd.config.serve.example_set",
        "cmd.config.serve.saved",
        "cmd.config.serve.invalid_service",
        "cmd.config.serve.invalid_port",
        "cmd.config.serve.empty_bind",
        "cmd.config.serve.unknown_option",
        "cmd.serve.current_defaults",
        "msg.showcg.label_serve_files",
        "msg.showcg.label_serve_http",
        "msg.showcg.label_serve_chess",
        "cmd.serve.services_title",
        "cmd.serve.services_usage",
        "cmd.serve.services_header",
        "cmd.serve.services_row_files",
        "cmd.serve.services_row_http",
        "cmd.serve.services_row_chess",
        "cmd.serve.services_row_config",
        "cmd.serve.services_example_files",
        "cmd.serve.services_example_http",
        "cmd.serve.services_example_config",
    ];

    #[test]
    fn serve_config_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SERVE_CONFIG_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SERVE_CONFIG_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SERVE_CONFIG_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SERVE_CONFIG_KEYS);
    }

    /// Every `nsheet` (sheet viewer) key must exist in all four bundled packs.
    const NSHEET_KEYS: &[&str] = &[
        "cmd.nsheet.usage_title",
        "cmd.nsheet.help_open",
        "cmd.nsheet.help_view",
        "cmd.nsheet.options_title",
        "cmd.nsheet.help_delimiter",
        "cmd.nsheet.help_header",
        "cmd.nsheet.help_limit",
        "cmd.nsheet.help_sort",
        "cmd.nsheet.help_filter",
        "cmd.nsheet.help_export",
        "cmd.nsheet.help_output",
        "cmd.nsheet.help_help",
        "cmd.nsheet.examples_title",
        "cmd.nsheet.example_open",
        "cmd.nsheet.example_view",
        "cmd.nsheet.example_filter",
        "cmd.nsheet.example_export",
        "cmd.nsheet.missing_file",
        "cmd.nsheet.file_not_found",
        "cmd.nsheet.load_fail",
        "cmd.nsheet.empty_sheet",
        "cmd.nsheet.no_rows",
        "cmd.nsheet.unknown_option",
        "cmd.nsheet.unknown_column",
        "cmd.nsheet.invalid_limit",
        "cmd.nsheet.invalid_delimiter",
        "cmd.nsheet.invalid_export",
        "cmd.nsheet.delimiter_usage",
        "cmd.nsheet.limit_usage",
        "cmd.nsheet.sort_usage",
        "cmd.nsheet.filter_usage",
        "cmd.nsheet.export_usage",
        "cmd.nsheet.output_usage",
        "cmd.nsheet.filter_fail",
        "cmd.nsheet.export_fail",
        "cmd.nsheet.exported",
        "cmd.nsheet.rows_shown",
        "cmd.nsheet.rows_truncated",
        "cmd.nsheet.tui_hint",
        "cmd.nsheet.tui_search_prompt",
        "cmd.nsheet.tui_sort_prompt",
        "cmd.nsheet.tui_filter_prompt",
        "cmd.nsheet.tui_export_prompt",
        "cmd.nsheet.tui_cell_title",
        "cmd.nsheet.tui_popup_hint",
        "cmd.nsheet.tui_no_matches",
        "cmd.nsheet.tui_large",
        "help.nsheet.title",
        "help.nsheet.open",
        "help.nsheet.view",
        "help.nsheet.sort",
        "help.nsheet.filter",
        "help.nsheet.export",
        "help.nsheet.tui",
        "help.nsheet.example1",
        "help.nsheet.example2",
        "help.nsheet.example3",
    ];

    #[test]
    fn nsheet_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", NSHEET_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", NSHEET_KEYS);
        assert_keys_present(super::th::build_map(), "thai", NSHEET_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", NSHEET_KEYS);
    }

    /// Every `nssh` (SSH profiles/shell/scan) key must exist in all four packs.
    const NSSH_KEYS: &[&str] = &[
        "cmd.nssh.usage_title",
        "cmd.nssh.help_add",
        "cmd.nssh.help_ssh",
        "cmd.nssh.help_cp",
        "cmd.nssh.help_scan",
        "cmd.nssh.help_manage",
        "cmd.nssh.help_keygen",
        "cmd.nssh.help_help",
        "cmd.nssh.options_title",
        "cmd.nssh.help_host",
        "cmd.nssh.help_user",
        "cmd.nssh.help_port",
        "cmd.nssh.help_key",
        "cmd.nssh.help_pass_env",
        "cmd.nssh.help_cmd",
        "cmd.nssh.help_trust",
        "cmd.nssh.help_ask_pass",
        "cmd.nssh.help_timeout",
        "cmd.nssh.help_top",
        "cmd.nssh.help_scan_ports",
        "cmd.nssh.help_yes",
        "cmd.nssh.examples_title",
        "cmd.nssh.example_add",
        "cmd.nssh.example_ssh",
        "cmd.nssh.example_cp",
        "cmd.nssh.example_scan",
        "cmd.nssh.missing_host",
        "cmd.nssh.missing_user",
        "cmd.nssh.missing_port",
        "cmd.nssh.missing_key",
        "cmd.nssh.missing_pass_env",
        "cmd.nssh.missing_cmd",
        "cmd.nssh.missing_timeout",
        "cmd.nssh.missing_ports",
        "cmd.nssh.missing_top",
        "cmd.nssh.invalid_port",
        "cmd.nssh.invalid_timeout",
        "cmd.nssh.invalid_top",
        "cmd.nssh.invalid_name",
        "cmd.nssh.unknown_option",
        "cmd.nssh.add_usage",
        "cmd.nssh.profile_exists",
        "cmd.nssh.profile_saved",
        "cmd.nssh.profile_not_found",
        "cmd.nssh.key_not_found",
        "cmd.nssh.pass_env_missing",
        "cmd.nssh.ls_empty",
        "cmd.nssh.ls_title",
        "cmd.nssh.info_title",
        "cmd.nssh.info_usage",
        "cmd.nssh.info_host",
        "cmd.nssh.info_user",
        "cmd.nssh.info_port",
        "cmd.nssh.info_key",
        "cmd.nssh.info_pass",
        "cmd.nssh.rm_usage",
        "cmd.nssh.rm_confirm",
        "cmd.nssh.rm_deleted",
        "cmd.nssh.rm_cancelled",
        "cmd.nssh.edit_usage",
        "cmd.nssh.edit_saved",
        "cmd.nssh.pick_profile",
        "cmd.nssh.ask_pass",
        "cmd.nssh.pass_env_unset",
        "cmd.nssh.ssh_connecting",
        "cmd.nssh.cmd_exit",
        "cmd.nssh.cp_usage",
        "cmd.nssh.cp_no_remote",
        "cmd.nssh.cp_two_sides",
        "cmd.nssh.cp_local_missing",
        "cmd.nssh.cp_read_fail",
        "cmd.nssh.cp_too_large",
        "cmd.nssh.cp_uploading",
        "cmd.nssh.cp_downloading",
        "cmd.nssh.cp_done",
        "cmd.nssh.scan_usage",
        "cmd.nssh.scan_start",
        "cmd.nssh.scan_title",
        "cmd.nssh.scan_none",
        "cmd.nssh.scan_done",
        "cmd.nssh.keygen_missing",
        "cmd.nssh.keygen_exit",
        "cmd.nssh.shell_hint",
        "cmd.nssh.shell_back",
        "cmd.nssh.shell_forced",
        "help.nssh.title",
        "help.nssh.add",
        "help.nssh.ssh",
        "help.nssh.cp",
        "help.nssh.scan",
        "help.nssh.manage",
        "help.nssh.keygen",
        "help.nssh.example1",
        "help.nssh.example2",
        "help.nssh.example3",
    ];

    #[test]
    fn nssh_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", NSSH_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", NSSH_KEYS);
        assert_keys_present(super::th::build_map(), "thai", NSSH_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", NSSH_KEYS);
    }

    /// Every `ral hintn` / inline-values / named-list key must exist in all
    /// four bundled language packs.
    const RAL_HINTN_KEYS: &[&str] = &[
        "alias.ral.hintn_cmd",
        "alias.ral.usage_hintn",
        "alias.ral.usage_hintn_add",
        "alias.ral.usage_hintn_rm",
        "alias.ral.usage_hintn_rnm",
        "alias.ral.usage_hintn_info",
        "alias.ral.hintn_added",
        "alias.ral.hintn_removed",
        "alias.ral.hintn_not_found",
        "alias.ral.hintn_renamed",
        "alias.ral.hintn_empty",
        "alias.ral.hintn_referenced",
        "alias.ral.hintn_info_title",
        "alias.ral.hintn_info_name",
        "alias.ral.hintn_info_values",
        "alias.ral.hintn_info_used_by",
        "alias.ral.hintn_list_title_global",
        "alias.ral.hintn_list_title_local",
        "alias.ral.hintn_value_count",
        "alias.ral.unknown_hint_list",
        "alias.ral.hintn_create_hint",
        "alias.ral.example_hint_values",
        "alias.ral.example_hintn",
        "help.alias.hintn",
        "help.alias.example7",
    ];

    #[test]
    fn ral_hintn_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", RAL_HINTN_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", RAL_HINTN_KEYS);
        assert_keys_present(super::th::build_map(), "thai", RAL_HINTN_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", RAL_HINTN_KEYS);
    }

    /// Every `scan files` / `scan dir` key must exist in all four bundled
    /// language packs.
    const SCAN_FILES_KEYS: &[&str] = &[
        "cmd.scan.files_usage",
        "cmd.scan.dir_usage",
        "cmd.scan.help_files",
        "cmd.scan.help_dir",
        "cmd.scan.no_dirs",
        "cmd.scan.total_dirs",
        "cmd.scan.total_dirs_size",
        "help.scan.files",
        "help.scan.dir",
        "help.scan.example6",
        "help.scan.example7",
    ];

    #[test]
    fn scan_files_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SCAN_FILES_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SCAN_FILES_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SCAN_FILES_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SCAN_FILES_KEYS);
    }

    /// Every `ntc -n/--new` CLI key must exist in all four bundled packs.
    const CLI_NEW_KEYS: &[&str] = &[
        "cli.new.usage_header",
        "cli.new.usage_shell",
        "cli.new.usage_cmd",
        "cli.new.usage_new",
        "cli.new.usage_upgrade",
        "cli.new.usage_version",
        "cli.new.usage_help",
        "cli.new.examples_header",
        "cli.new.ex_help",
        "cli.new.ex_version",
        "cli.new.ex_alias",
        "cli.new.ex_new",
        "cli.new.ex_new_cmd",
        "cli.new.opened",
        "cli.new.no_terminal",
        "cli.new.unsupported",
    ];

    #[test]
    fn cli_new_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", CLI_NEW_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", CLI_NEW_KEYS);
        assert_keys_present(super::th::build_map(), "thai", CLI_NEW_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", CLI_NEW_KEYS);
    }
}
