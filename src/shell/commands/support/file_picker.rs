//! Interactive file picker shared by `txtc` / `txtf` and the `view` family.

use crate::config::Config;
use crate::filetype::FormatConfig;
use crate::navigator::Navigator;
use crate::output::{print_info, print_success};
use crate::utils::interactive::{InteractiveMenu, MenuItem};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) fn list_supported_files(nav: &Navigator) -> Result<Vec<(String, PathBuf)>> {
    list_supported_files_in(nav.current_path())
}

/// Supported files directly inside `dir` (not recursive), sorted by name.
pub(crate) fn list_supported_files_in(dir: &Path) -> Result<Vec<(String, PathBuf)>> {
    let fmt_cfg = FormatConfig::from_both();
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if crate::filetype::is_supported_format_with_config(&path, &fmt_cfg) {
                        files.push((name.to_string(), path));
                    }
                }
            }
        }
    }

    files.sort_by_cached_key(|a| a.0.to_lowercase());
    Ok(files)
}

pub(crate) fn show_file_selection_menu(
    _nav: &Navigator,
    files: Vec<(String, PathBuf)>,
    copy_mode: bool,
) -> Result<()> {
    show_file_selection_menu_range(_nav, files, copy_mode, None, None)
}

pub(crate) fn show_file_selection_menu_range(
    _nav: &Navigator,
    files: Vec<(String, PathBuf)>,
    copy_mode: bool,
    range: Option<crate::output::FileLineRange>,
    lines: Option<bool>,
) -> Result<()> {
    let title = if copy_mode {
        "Select a file to COPY to clipboard"
    } else {
        "Select a file to DISPLAY"
    };

    let items: Vec<MenuItem> = files
        .iter()
        .map(|(name, path)| MenuItem::with_detail(name, get_file_size(path)))
        .collect();

    let menu = InteractiveMenu::new(title)
        .items(items)
        .prompt(format!("Enter number (1-{}) or 0: ", files.len()));

    if let Some(idx) = menu.run()? {
        let (name, path) = &files[idx];
        if copy_mode {
            print_info(&format!("Copying '{}' to clipboard...", name));
            // `txtc --line/--no-line` must win over the config default here
            // too, so the picker behaves exactly like `txtf`'s picker.
            let show_lines = lines.unwrap_or_else(Config::global_get_show_line_numbers);
            let content = crate::output::cat_file_with_line_numbers(path, show_lines)?;
            crate::output::copy_to_clipboard(&content, "TXT")?;
            print_success(&format!("File '{}' copied to clipboard!", name));
        } else {
            let show_lines = lines.unwrap_or_else(Config::global_get_show_line_numbers);
            crate::output::cat_file_with_syntax_range(path, show_lines, range)?;
        }
    }

    Ok(())
}

pub(crate) fn get_file_size(path: &Path) -> String {
    if let Ok(metadata) = std::fs::metadata(path) {
        let size = metadata.len();
        if size < 1024 {
            format!("({} B)", size)
        } else if size < 1024 * 1024 {
            format!("({:.1} KB)", size as f64 / 1024.0)
        } else {
            format!("({:.1} MB)", size as f64 / (1024.0 * 1024.0))
        }
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_supported_files_in_filters_one_level_and_sorts() {
        let _cwd = crate::utils::test_support::TempCwd::with_fixture(
            &[
                ("docs/b.txt", "b"),
                ("docs/a.rs", "a"),
                ("docs/image.png", "\0fake"),
                ("docs/nested/deep.txt", "deep"),
            ],
            &["docs", "docs/nested"],
        );
        let files = list_supported_files_in(Path::new("docs")).unwrap();
        let names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, vec!["a.rs", "b.txt"]);
    }
}
