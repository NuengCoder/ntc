//! Language/localization system for ntc.
//! Provides a global language pack loaded from built-in English defaults
//! with optional external `.ntc_lang` overrides from `~/.config/ntc/lang/`.

pub mod en;
pub mod lo;
pub mod th;
pub mod zh;

use std::collections::HashMap;
use std::sync::RwLock;

/// Macro to get a localized string by dot-notation key.
///
/// # Examples
/// ```
/// let title = ntc::t!("help.nav.title");
/// let label = ntc::t!("config.lang.set", "thai");
/// assert!(!title.is_empty());
/// assert!(!label.is_empty());
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

    let lang_dir = dirs::config_dir().map(|d| d.join("ntc").join("lang"));
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
        guard.get(key).cloned().unwrap_or_else(|| key.to_string())
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

    const TIME_KEYS: &[&str] = &[
        "cmd.time.count_usage",
        "cmd.time.run_usage",
        "cmd.time.executing",
        "cmd.time.elapsed",
        "cmd.time.exit_status",
        "cmd.time.terminated",
        "cmd.time.first_command",
        "cmd.time.second_command",
        "cmd.time.first_result",
        "cmd.time.second_result",
        "cmd.time.outputs_match",
        "cmd.time.outputs_differ",
        "cmd.time.large_diff",
        "cmd.time.invalid_duration",
        "cmd.time.cancelled",
        "cmd.time.finished",
        "cmd.time.remaining",
        "cmd.time.compare_usage",
        "cmd.time.bench_usage",
        "cmd.time.invalid_runs",
        "cmd.time.bench_header",
        "cmd.time.bench_run",
        "cmd.time.bench_summary",
        "cmd.time.bench_min",
        "cmd.time.bench_max",
        "cmd.time.bench_mean",
        "cmd.time.bench_median",
        "cmd.time.bench_note",
        "cmd.time.status_ok",
        "cmd.time.status_failed",
        "cmd.time.ignoring_whitespace",
        "help.time.title",
        "help.time.run",
        "help.time.alias",
        "help.time.compare",
        "help.time.compare_with",
        "help.time.bench",
        "help.time.count",
        "help.time.help",
        "help.time.note",
    ];

    #[test]
    fn time_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", TIME_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", TIME_KEYS);
        assert_keys_present(super::th::build_map(), "thai", TIME_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", TIME_KEYS);
    }

    const QUR_KEYS: &[&str] = &[
        "misc.qur.usage",
        "misc.qur.example1",
        "misc.qur.example2",
        "misc.qur.example3",
        "misc.qur.example4",
        "misc.qur.searching",
        "misc.qur.no_files",
        "misc.qur.no_matches",
        "misc.qur.files_header",
        "misc.qur.summary_header",
        "misc.qur.summary_hint",
        "misc.qur.replace_preview",
        "misc.qur.confirm",
        "misc.qur.cancelled",
        "misc.qur.replaced",
        "misc.qur.skipped",
        "misc.qur.many_results",
        "misc.qur.replace_needs_select",
        "misc.qur.display_needs_select",
        "misc.qur.empty_select",
        "misc.qur.empty_replace",
        "misc.qur.stdin_needs_select",
        "help.context.qur.needs_glob",
        "help.context.qur.select_value",
        "help.context.qur.replace_value",
        "help.context.qur.depth_value",
        "help.context.qur.limit_value",
        "help.context.qur.next_select",
        "help.context.qur.next_display_replace",
        "help.context.qur.next_depth",
        "help.search.qur",
    ];

    #[test]
    fn qur_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", QUR_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", QUR_KEYS);
        assert_keys_present(super::th::build_map(), "thai", QUR_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", QUR_KEYS);
    }

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

    /// Assert these keys are really translated — present *and* different from
    /// the English master pack — so an empty stub cannot hide behind the
    /// fallback.
    fn assert_translated(
        map: HashMap<String, String>,
        en: &HashMap<String, String>,
        lang: &str,
        keys: &[&str],
    ) {
        let untranslated: Vec<&str> = keys
            .iter()
            .copied()
            .filter(|k| map.get(*k).map(|v| en.get(*k) == Some(v)).unwrap_or(true))
            .collect();
        assert!(
            untranslated.is_empty(),
            "{lang} pack still shows English for: {untranslated:?}"
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

    /// Every `views` / `count` key must exist in all four bundled packs.
    const VIEWS_COUNT_KEYS: &[&str] = &[
        "help.views.title",
        "help.views.usage",
        "help.views.size",
        "help.views.sort",
        "help.views.newline",
        "help.views.note",
        "help.views.example1",
        "help.views.example2",
        "cmd.views.usage",
        "cmd.views.invalid_size",
        "cmd.views.invalid_sort",
        "cmd.views.invalid_sort_value",
        "cmd.views.empty",
        "help.pipe.count",
        "misc.count.usage",
    ];

    #[test]
    fn views_and_count_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", VIEWS_COUNT_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", VIEWS_COUNT_KEYS);
        assert_keys_present(super::th::build_map(), "thai", VIEWS_COUNT_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", VIEWS_COUNT_KEYS);
    }

    /// `views` help/error text must be genuinely translated in every
    /// non-English pack (the modules used to be empty stubs).
    const VIEWS_KEYS: &[&str] = &[
        "help.views.title",
        "help.views.usage",
        "help.views.size",
        "help.views.sort",
        "help.views.newline",
        "help.views.note",
        "help.views.example1",
        "help.views.example2",
        "cmd.views.usage",
        "cmd.views.invalid_size",
        "cmd.views.invalid_sort",
        "cmd.views.invalid_sort_value",
        "cmd.views.empty",
    ];

    #[test]
    fn views_keys_are_translated_outside_english() {
        let en = super::en::build_map();
        assert_translated(super::zh::build_map(), &en, "chinese", VIEWS_KEYS);
        assert_translated(super::th::build_map(), &en, "thai", VIEWS_KEYS);
        assert_translated(super::lo::build_map(), &en, "lao", VIEWS_KEYS);
    }

    /// `trim` help/usage keys.
    const TRIM_KEYS: &[&str] = &[
        "help.pipe.trim",
        "help.pipe.example6",
        "misc.trim.usage",
        "misc.trim.empty_pattern",
    ];

    #[test]
    fn trim_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", TRIM_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", TRIM_KEYS);
        assert_keys_present(super::th::build_map(), "thai", TRIM_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", TRIM_KEYS);
    }

    #[test]
    fn trim_keys_are_translated_outside_english() {
        let en = super::en::build_map();
        assert_translated(super::zh::build_map(), &en, "chinese", TRIM_KEYS);
        assert_translated(super::th::build_map(), &en, "thai", TRIM_KEYS);
        assert_translated(super::lo::build_map(), &en, "lao", TRIM_KEYS);
    }

    /// Every `intro_view` config key must exist in all four bundled packs.
    const INTRO_VIEW_KEYS: &[&str] = &[
        "help.config.intro_view",
        "msg.setIV.status",
        "msg.setIV.on",
        "msg.setIV.off",
        "msg.setIV.error",
        "msg.showcg.label_intro_view",
    ];

    #[test]
    fn intro_view_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", INTRO_VIEW_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", INTRO_VIEW_KEYS);
        assert_keys_present(super::th::build_map(), "thai", INTRO_VIEW_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", INTRO_VIEW_KEYS);
    }

    #[test]
    fn dir_view_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", DIR_VIEW_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", DIR_VIEW_KEYS);
        assert_keys_present(super::th::build_map(), "thai", DIR_VIEW_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", DIR_VIEW_KEYS);
    }

    /// Every `file_view` / `size` config key must exist in all four packs.
    const FILE_VIEW_SIZE_KEYS: &[&str] = &[
        "help.config.file_view",
        "help.config.size",
        "msg.setFV.status",
        "msg.setFV.on",
        "msg.setFV.off",
        "msg.setFV.error",
        "msg.setSZ.status",
        "msg.setSZ.target_status",
        "msg.setSZ.on",
        "msg.setSZ.off",
        "msg.setSZ.error",
        "msg.setSZ.slow_warn",
        "msg.setSZ.usage",
        "msg.showcg.label_file_view",
        "msg.showcg.label_auto_size",
        "msg.showcg.auto_size_value",
    ];

    #[test]
    fn file_view_size_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", FILE_VIEW_SIZE_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", FILE_VIEW_SIZE_KEYS);
        assert_keys_present(super::th::build_map(), "thai", FILE_VIEW_SIZE_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", FILE_VIEW_SIZE_KEYS);
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

    /// Every `deadlock` key must exist in all four bundled language packs.
    const DEADLOCK_KEYS: &[&str] = &[
        "deadlock.no_args",
        "deadlock.created",
        "deadlock.default_master_hint",
        "deadlock.master_prompt",
        "deadlock.wrong_master",
        "deadlock.too_many_attempts",
        "deadlock.force_change_notice",
        "deadlock.new_master_prompt",
        "deadlock.confirm_master_prompt",
        "deadlock.empty_master",
        "deadlock.default_master_rejected",
        "deadlock.master_mismatch",
        "deadlock.master_changed",
        "deadlock.old_master_prompt",
        "deadlock.wrong_master_once",
        "deadlock.unlocked",
        "deadlock.add_usage",
        "deadlock.invalid_name",
        "deadlock.value_prompt",
        "deadlock.empty_password",
        "deadlock.added",
        "deadlock.updated",
        "deadlock.ls_title",
        "deadlock.ls_empty",
        "deadlock.rmpw_usage",
        "deadlock.removed",
        "deadlock.not_found",
        "deadlock.unknown_cmd",
        "deadlock.hint_help",
        "deadlock.bye",
        "deadlock.editor_fail",
        "deadlock.keychain_registered",
        "deadlock.keychain_unavailable",
        "deadlock.forget_done",
        "deadlock.forget_failed",
        "deadlock.destroy_confirm",
        "deadlock.destroy_cancelled",
        "deadlock.destroyed",
        "deadlock.destroy_failed",
        "deadlock.os_protected",
        "deadlock.os_unprotected",
        "help.deadlock.title",
        "help.deadlock.launch",
        "help.deadlock.ls",
        "help.deadlock.add",
        "help.deadlock.rmpw",
        "help.deadlock.master",
        "help.deadlock.forget",
        "help.deadlock.destroy",
        "help.deadlock.exit",
        "help.deadlock.note",
        "help.deadlock.example1",
        "help.deadlock.example2",
        "deadlock.vault_missing",
        "deadlock.corrupt_notice",
        "deadlock.corrupt_restore_prompt",
        "deadlock.corrupt_restore_failed",
        "deadlock.corrupt_restored",
        "deadlock.corrupt_recreate_prompt",
        "deadlock.corrupt_cancelled",
        "deadlock.corrupt_aborted",
        "deadlock.corrupt_kept",
        "deadlock.corrupt_recreate_failed",
        "deadlock.corrupt_recreated",
    ];

    #[test]
    fn deadlock_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", DEADLOCK_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", DEADLOCK_KEYS);
        assert_keys_present(super::th::build_map(), "thai", DEADLOCK_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", DEADLOCK_KEYS);
    }

    /// Every `smtp --deadlock` key must exist in all four bundled language packs.
    const SMTP_DEADLOCK_KEYS: &[&str] =
        &["cmd.smtp.help_info_deadlock", "cmd.smtp.password_hidden"];

    #[test]
    fn smtp_deadlock_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", SMTP_DEADLOCK_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", SMTP_DEADLOCK_KEYS);
        assert_keys_present(super::th::build_map(), "thai", SMTP_DEADLOCK_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", SMTP_DEADLOCK_KEYS);
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

    /// Every `serve files --lock` key must exist in all four bundled language packs.
    const LOCK_KEYS: &[&str] = &["cmd.serve.help_lock", "cmd.serve.lock_on"];

    #[test]
    fn lock_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", LOCK_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", LOCK_KEYS);
        assert_keys_present(super::th::build_map(), "thai", LOCK_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", LOCK_KEYS);
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
        "cmd.nssh.pass_env_not_in_vault",
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
        "cmd.nssh.pass_env_registered",
        "cmd.nssh.pass_env_pending",
        "cmd.nssh.pass_env_from_vault",
        "cmd.nssh.master_unlock_failed",
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
        "cmd.nssh.password_prompt",
        "cmd.nssh.password_incorrect",
        "cmd.nssh.auth_failed",
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

    /// Every `nms` (NTC Music player + media scan) key must exist in all
    /// four bundled packs.
    const NMS_KEYS: &[&str] = &[
        "cmd.nms.usage_title",
        "cmd.nms.help_play",
        "cmd.nms.help_scan",
        "cmd.nms.help_help",
        "cmd.nms.options_title",
        "cmd.nms.help_depth",
        "cmd.nms.help_size",
        "cmd.nms.help_long",
        "cmd.nms.help_care",
        "cmd.nms.keys_title",
        "cmd.nms.help_keys",
        "cmd.nms.examples_title",
        "cmd.nms.example_play",
        "cmd.nms.example_video",
        "cmd.nms.example_scan",
        "cmd.nms.example_long",
        "cmd.nms.play_missing_file",
        "cmd.nms.file_not_found",
        "cmd.nms.not_media",
        "cmd.nms.no_sound_device",
        "cmd.nms.play_failed",
        "cmd.nms.unknown_sub",
        "cmd.nms.unknown_option",
        "cmd.nms.depth_usage",
        "cmd.nms.invalid_depth",
        "cmd.nms.long_usage",
        "cmd.nms.invalid_long",
        "cmd.nms.long_none",
        "cmd.nms.no_files",
        "cmd.nms.list_title",
        "cmd.nms.total_files",
        "cmd.nms.total_files_size",
        "help.nms.title",
        "help.nms.play",
        "help.nms.scan",
        "help.nms.long",
        "help.nms.keys",
        "help.nms.example1",
        "help.nms.example2",
        "help.nms.example3",
        "help.nms.example4",
    ];

    #[test]
    fn nms_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", NMS_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", NMS_KEYS);
        assert_keys_present(super::th::build_map(), "thai", NMS_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", NMS_KEYS);
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

    /// Every local/global scope key (the `[global]` / `[local]` listings, the
    /// `-g/--global` hints and the local "cared" lists) must exist in all four
    /// bundled language packs.
    const LOCAL_SCOPE_KEYS: &[&str] = &[
        "alias.ral.section_global",
        "alias.ral.section_local",
        "alias.ral.none",
        "alias.ral.no_local",
        "alias.ral.shadowed",
        "alias.ral.shadowed_global",
        "alias.ral.global_active_again",
        "alias.ral.global_only_hint",
        "alias.ral.local_priority_note",
        "alias.ral.scope_local",
        "alias.ral.scope_global",
        "alias.ral.cls_target",
        "cmd.igcared.section_global",
        "cmd.igcared.section_local",
        "cmd.igcared.cared_dirs",
        "cmd.igcared.cared_exts",
        "cmd.igcared.cared_files",
        "cmd.igcared.none",
        "cmd.igcared.no_local",
        "cmd.igcared.scope_help",
        "cmd.ignore.scope_local",
        "cmd.ignore.scope_global",
        "cmd.ignore.scope_help",
        "cmd.ignore.scope_target",
        "cmd.care.scope_help",
        "cmd.config.local.init_all_removed",
        "cmd.config.local.precedence_hint",
        "igcare_export.cared_dirs",
        "igcare_export.cared_extensions",
        "igcare_export.cared_files",
    ];

    #[test]
    fn local_scope_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", LOCAL_SCOPE_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", LOCAL_SCOPE_KEYS);
        assert_keys_present(super::th::build_map(), "thai", LOCAL_SCOPE_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", LOCAL_SCOPE_KEYS);
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

    /// Every `fake` command / help key must exist in all four bundled packs.
    const FAKE_KEYS: &[&str] = &[
        "cmd.fake.usage_title",
        "cmd.fake.usage_gen",
        "cmd.fake.usage_types",
        "cmd.fake.usage_locales",
        "cmd.fake.usage_group",
        "cmd.fake.usage_help",
        "cmd.fake.options_title",
        "cmd.fake.opt_count",
        "cmd.fake.opt_format",
        "cmd.fake.opt_seed",
        "cmd.fake.opt_output",
        "cmd.fake.opt_group",
        "cmd.fake.opt_locale",
        "cmd.fake.opt_unique",
        "cmd.fake.examples_header",
        "cmd.fake.example1",
        "cmd.fake.example2",
        "cmd.fake.example3",
        "cmd.fake.example4",
        "cmd.fake.unknown_sub",
        "cmd.fake.gen.wrote",
        "cmd.fake.gen.unknown_ext",
        "cmd.fake.gen.xlsx_needs_output",
        "cmd.fake.gen.count_usage",
        "cmd.fake.gen.bad_count",
        "cmd.fake.gen.format_usage",
        "cmd.fake.gen.bad_format",
        "cmd.fake.gen.seed_usage",
        "cmd.fake.gen.bad_seed",
        "cmd.fake.gen.output_usage",
        "cmd.fake.gen.group_usage",
        "cmd.fake.gen.unknown_flag",
        "cmd.fake.gen.group_and_types",
        "cmd.fake.gen.need_input",
        "cmd.fake.gen.unknown_type",
        "cmd.fake.gen.unique_with_group",
        "cmd.fake.gen.unique_unknown_col",
        "cmd.fake.gen.unique_exhausted",
        "cmd.fake.gen.locale_usage",
        "cmd.fake.gen.bad_locale",
        "cmd.fake.gen.ref_cycle",
        "cmd.fake.gen.ref_unknown_col",
        "cmd.fake.types.title",
        "cmd.fake.types.none",
        "cmd.fake.types.matches",
        "cmd.fake.locales.title",
        "cmd.fake.locales.note",
        "cmd.fake.group.usage_title",
        "cmd.fake.group.help_add",
        "cmd.fake.group.help_edit",
        "cmd.fake.group.help_rm",
        "cmd.fake.group.help_rnm",
        "cmd.fake.group.help_ls",
        "cmd.fake.group.help_info",
        "cmd.fake.group.help_clear",
        "cmd.fake.group.help_export",
        "cmd.fake.group.help_import",
        "cmd.fake.group.example1",
        "cmd.fake.group.example2",
        "cmd.fake.group.unknown_sub",
        "cmd.fake.group.no_columns",
        "cmd.fake.group.bad_column",
        "cmd.fake.group.unknown_type",
        "cmd.fake.group.add_usage",
        "cmd.fake.group.bad_name",
        "cmd.fake.group.not_found",
        "cmd.fake.group.exists",
        "cmd.fake.group.scope_local",
        "cmd.fake.group.scope_global",
        "cmd.fake.group.added",
        "cmd.fake.group.updated",
        "cmd.fake.group.rm_usage",
        "cmd.fake.group.removed",
        "cmd.fake.group.rnm_usage",
        "cmd.fake.group.renamed",
        "cmd.fake.group.ls_empty",
        "cmd.fake.group.ls_none",
        "cmd.fake.group.info_usage",
        "cmd.fake.group.info_title",
        "cmd.fake.group.info_count",
        "cmd.fake.group.clear_empty",
        "cmd.fake.group.clear_prompt",
        "cmd.fake.group.clear_cancelled",
        "cmd.fake.group.cleared",
        "cmd.fake.group.bad_ref",
        "cmd.fake.group.ref_col_missing",
        "cmd.fake.group.export_usage",
        "cmd.fake.group.import_usage",
        "cmd.fake.group.exported",
        "cmd.fake.group.import_ok",
        "cmd.fake.group.import_exists",
        "cmd.fake.group.import_bad_file",
        "help.fake.title",
        "help.fake.gen",
        "help.fake.gen_options",
        "help.fake.gen_group",
        "help.fake.types",
        "help.fake.types_search",
        "help.fake.locales",
        "help.fake.group_add",
        "help.fake.group_edit",
        "help.fake.group_rm",
        "help.fake.group_rnm",
        "help.fake.group_ls",
        "help.fake.group_info",
        "help.fake.group_clear",
        "help.fake.group_export",
        "help.fake.group_import",
        "help.fake.group_ref",
        "help.fake.help",
        "help.fake.example1",
        "help.fake.example2",
        "help.fake.example3",
        "help.fake.example4",
    ];

    #[test]
    fn fake_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", FAKE_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", FAKE_KEYS);
        assert_keys_present(super::th::build_map(), "thai", FAKE_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", FAKE_KEYS);
    }

    /// Every `init`/`deinit` OpenCode scaffold key must exist in all four packs.
    const OPENCODE_INIT_KEYS: &[&str] = &[
        "cmd.config.init.usage",
        "cmd.config.init.opencode_ready",
        "cmd.config.deinit.usage",
        "cmd.config.deinit.opencode_not_found",
        "help.other.init",
    ];

    #[test]
    fn opencode_init_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", OPENCODE_INIT_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", OPENCODE_INIT_KEYS);
        assert_keys_present(super::th::build_map(), "thai", OPENCODE_INIT_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", OPENCODE_INIT_KEYS);
    }

    /// `where <binary>` PATH lookup output and its help rows.
    const WHERE_BINARY_KEYS: &[&str] = &[
        "cmd.nav.where.not_found",
        "cmd.nav.where.path_title",
        "help.nav.where",
        "help.nav.example5",
    ];

    #[test]
    fn where_binary_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", WHERE_BINARY_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", WHERE_BINARY_KEYS);
        assert_keys_present(super::th::build_map(), "thai", WHERE_BINARY_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", WHERE_BINARY_KEYS);
    }

    /// `rmd <wildcard>` bulk-delete listing/prompt output.
    const RMD_WILDCARD_KEYS: &[&str] = &[
        "misc.rmd.no_match",
        "misc.rmd.pattern_matches",
        "misc.rmd.confirm_delete",
        "misc.rmd.cancelled",
        "misc.rmd.deadlock_vault_admin_only",
        "help.fileops.rmd",
    ];

    #[test]
    fn rmd_wildcard_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", RMD_WILDCARD_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", RMD_WILDCARD_KEYS);
        assert_keys_present(super::th::build_map(), "thai", RMD_WILDCARD_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", RMD_WILDCARD_KEYS);
    }

    /// Clipboard keys used in clipboard.rs.
    const CLIPBOARD_KEYS: &[&str] = &[
        "cmd.clipboard.copied_piped",
        "cmd.clipboard.clear_error",
        "help.pipe.filters",
    ];

    #[test]
    fn clipboard_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", CLIPBOARD_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", CLIPBOARD_KEYS);
        assert_keys_present(super::th::build_map(), "thai", CLIPBOARD_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", CLIPBOARD_KEYS);
    }

    /// Report clip keys used in report.rs.
    const REPORT_CLIP_KEYS: &[&str] = &[
        "help.report.clip",
        "help.report.clip_example",
        "cmd.report.help_clip",
        "cmd.report.legacy_removed",
        "cmd.report.example_project",
        "cmd.report.example_clip",
        "cmd.report.clip_header",
        "cmd.report.clip_meta_date",
        "cmd.report.clip_meta_lines",
        "cmd.report.clip_meta_chars",
    ];

    #[test]
    fn report_clip_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", REPORT_CLIP_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", REPORT_CLIP_KEYS);
        assert_keys_present(super::th::build_map(), "thai", REPORT_CLIP_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", REPORT_CLIP_KEYS);
    }

    /// Todo format keys used in config_commands.rs.
    const TODO_FORMAT_KEYS: &[&str] = &[
        "msg.setTD.status",
        "msg.setTD.set",
        "msg.setTD.error",
        "help.config.todo",
        "cmd.todo.help_format",
        "cmd.todo.unknown_format",
        "cmd.todo.parse_failed",
    ];

    #[test]
    fn todo_format_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", TODO_FORMAT_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", TODO_FORMAT_KEYS);
        assert_keys_present(super::th::build_map(), "thai", TODO_FORMAT_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", TODO_FORMAT_KEYS);
    }

    /// NFE keys used in editor/nfe.rs and editor/types.rs.
    const NFE_KEYS: &[&str] = &[
        "editor.nfe_confirm",
        "editor.nfe_rm_root",
        "editor.hint_nfe",
    ];

    #[test]
    fn nfe_keys_exist_in_all_languages() {
        assert_keys_present(super::en::build_map(), "english", NFE_KEYS);
        assert_keys_present(super::zh::build_map(), "chinese", NFE_KEYS);
        assert_keys_present(super::th::build_map(), "thai", NFE_KEYS);
        assert_keys_present(super::lo::build_map(), "lao", NFE_KEYS);
    }

    /// The `next.txt` key set must be genuinely translated (not just present
    /// through the English fallback) in the Lao pack.
    const NEXT_TXT_KEYS: &[&str] = &[
        "help.deadlock.title",
        "help.deadlock.launch",
        "help.deadlock.ls",
        "help.deadlock.add",
        "help.deadlock.rmpw",
        "help.deadlock.master",
        "help.deadlock.forget",
        "help.deadlock.destroy",
        "help.deadlock.exit",
        "help.deadlock.note",
        "help.time.title",
        "help.time.run",
        "help.time.alias",
        "help.time.compare",
        "help.time.compare_with",
        "help.time.bench",
        "help.time.count",
        "help.time.help",
        "help.time.note",
        "cmd.report.clip_header",
        "cmd.report.clip_meta_date",
        "cmd.report.clip_meta_lines",
        "cmd.report.clip_meta_chars",
        "help.report.project",
        "help.report.project_example",
        "help.report.clip",
        // help.deadlock.example1/2 and help.report.clip_example are literal
        // command examples and are identical in every language on purpose.
    ];

    #[test]
    fn next_txt_keys_are_translated_in_lao() {
        let en = super::en::build_map();
        let lo = super::lo::build_map();
        let untranslated: Vec<&str> = NEXT_TXT_KEYS
            .iter()
            .copied()
            .filter(|k| lo.get(*k).map(|v| en.get(*k) == Some(v)).unwrap_or(true))
            .collect();
        assert!(
            untranslated.is_empty(),
            "lao pack still shows English for: {untranslated:?}"
        );
    }
}
