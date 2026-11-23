//! `report` — project reports and clipboard history display.

pub(crate) mod completion;
pub(crate) mod help;
#[allow(clippy::module_inception)] // engine submodule keeps its name for grep-ability
pub(crate) mod report;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::shell::commands::builtin::report::report::generate_report_to;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::report_project::{parse_project_report, project_output_path};

pub fn cmd_report(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let format = parts.first().copied().unwrap_or("");
    let subargs = parts.get(1).unwrap_or(&"").trim();

    if format.eq_ignore_ascii_case("project") {
        let req = match parse_project_report(subargs) {
            Ok(r) => r,
            Err(key) => {
                print_error(&t!(key));
                println!("{}", t!("cmd.report.project_usage"));
                return Ok(false);
            }
        };
        if req.path.is_file() {
            print_error(&t!(
                "cmd.report.dir_only",
                req.format.extension().to_uppercase()
            ));
            return Ok(false);
        }
        let out = match &req.name {
            Some(name) => project_output_path(name, req.format),
            None => {
                // Same default naming as `report <fmt>`.
                let dir_name = req
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "root".to_string());
                crate::output::build_output_path(&format!(
                    "{}.{}",
                    dir_name,
                    req.format.extension()
                ))
            }
        };
        generate_report_to(&req.path, req.format, &out.to_string_lossy())?;
        return Ok(false);
    }

    if format.eq_ignore_ascii_case("clip") {
        if subargs.is_empty() {
            print_error(&t!("cmd.clipboard.select_usage"));
            return Ok(false);
        }
        let n: usize = match subargs.split_whitespace().next().unwrap_or("").parse() {
            Ok(n) => n,
            Err(_) => {
                print_error(&t!("cmd.clipboard.select_usage"));
                return Ok(false);
            }
        };
        let entries = super::clip::load_history_public();
        if entries.is_empty() {
            print_info(&t!("cmd.clipboard.empty"));
            return Ok(false);
        }
        if n == 0 || n > entries.len() {
            print_error(&t!("cmd.clipboard.entry_not_found", n, entries.len()));
            return Ok(false);
        }
        let entry = &entries[n - 1];
        let date = chrono::DateTime::from_timestamp(entry.ts as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "?".to_string());
        let line_count = entry.content.lines().count().max(1);
        let char_count = entry.content.chars().count();
        println!("{}", t!("cmd.report.clip_header", n).cyan().bold());
        println!("  {}: {}", t!("cmd.report.clip_meta_date"), date);
        println!("  {}: {}", t!("cmd.report.clip_meta_lines"), line_count);
        println!("  {}: {}", t!("cmd.report.clip_meta_chars"), char_count);
        println!();
        print!("{}", entry.content);
        if !entry.content.is_empty() && !entry.content.ends_with('\n') {
            println!();
        }
        return Ok(false);
    }

    // Legacy `report txt|json|md|html|pdf|docx|xlsx <path>` shortcuts are removed.
    // Use `report project <fmt> [path] [--name ...]` instead.
    if matches!(
        format,
        "txt" | "json" | "md" | "html" | "pdf" | "docx" | "xlsx"
    ) {
        print_error(&t!("cmd.report.legacy_removed"));
        println!("  {}", t!("cmd.report.help_project"));
        return Ok(false);
    }

    println!("{}", t!("cmd.report.title").cyan().bold());
    println!("  {}", t!("cmd.report.help_project"));
    println!("  {}", t!("cmd.report.help_clip"));
    println!();
    println!("{}", t!("cmd.report.examples").green());
    println!("  {}", t!("cmd.report.example_project"));
    println!("  {}", t!("cmd.report.example_clip"));
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "report",
    aliases: &[],
    help_topic: Some("report"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_report),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_report_writes_the_named_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        std::fs::create_dir_all(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/notes.txt"), "hi").unwrap();
        let out = dir.path().join("out/report");
        let args = format!(
            "project html {} --name {}",
            dir.path().display(),
            out.display()
        );
        let mut nav = Navigator::new().unwrap();
        cmd_report(&args, &mut nav).unwrap();
        let file = dir.path().join("out/report.html");
        assert!(file.is_file(), "missing {}", file.display());
        let html = std::fs::read_to_string(&file).unwrap();
        // The tree is a collapsible widget, not an ASCII <pre> dump.
        assert!(html.contains("ntree-root"));
        assert!(html.contains("<details><summary>"));
        assert!(html.contains("main.rs"));
        assert!(html.contains("sub"));
    }
}
