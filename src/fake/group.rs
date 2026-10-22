// src/fake/group.rs
//! `fake group` — CRUD for named fake-data groups.

use colored::*;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::{Config, FakeColumn};
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::Result;

use super::types;

/// `fake group <sub> …`
pub fn cmd_group(rest: &str) -> Result<()> {
    let rest = rest.trim();
    let (sub, args) = match rest.split_once(char::is_whitespace) {
        Some((a, b)) => (a.to_lowercase(), b.trim()),
        None => (rest.to_lowercase(), ""),
    };

    match sub.as_str() {
        "" | "help" | "-h" | "--help" => print_usage(),
        "add" | "a" => add_group(args, false)?,
        "edit" | "ed" => add_group(args, true)?,
        "rm" | "remove" | "delete" => rm_group(args)?,
        "rnm" | "rename" | "mv" => rename_group(args)?,
        "ls" | "list" => list_groups(args)?,
        "info" | "show" => info_group(args)?,
        "clear" | "cls" => clear_groups(args)?,
        "export" | "exp" => export_groups(args)?,
        "import" | "imp" => import_groups(args)?,
        other => {
            print_error(&t!("cmd.fake.group.unknown_sub", other));
            println!();
            print_usage();
        }
    }
    Ok(())
}

/// Parse `[alias as type, alias as type, …]` (brackets optional).
///
/// A trailing `!` marks the column unique (`contract as phone_number !`).
/// `type` may be a built-in type or a `ref:<group>.<column>` reference.
fn parse_columns(spec: &str) -> Result<Vec<FakeColumn>, String> {
    let spec = spec.trim();
    let spec = spec
        .strip_prefix('[')
        .unwrap_or(spec)
        .strip_suffix(']')
        .unwrap_or(spec)
        .trim();
    if spec.is_empty() {
        return Err(t!("cmd.fake.group.no_columns"));
    }
    let mut cols = Vec::new();
    for raw in spec.split(',') {
        let mut raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        // A trailing `!` (with or without a preceding space) marks unique.
        let unique = raw.ends_with('!');
        if unique {
            raw = raw[..raw.len() - 1].trim();
        }
        if raw.is_empty() {
            return Err(t!("cmd.fake.group.bad_column", raw));
        }
        let (alias, type_name) = match raw.split_once(" as ") {
            Some((a, ty)) => (a.trim().to_string(), ty.trim().to_string()),
            None => {
                let only = raw.trim().to_string();
                // `ref:group.column` defaults its alias to the target column.
                if let Some((_, column)) = parse_ref(&only) {
                    (column, only)
                } else {
                    (only.clone(), only)
                }
            }
        };
        if alias.is_empty() || type_name.is_empty() {
            return Err(t!("cmd.fake.group.bad_column", raw));
        }
        let canonical = resolve_type_name(&type_name)?;
        let mut col = FakeColumn::new(alias, canonical);
        col.unique = unique;
        cols.push(col);
    }
    if cols.is_empty() {
        return Err(t!("cmd.fake.group.no_columns"));
    }
    Ok(cols)
}

/// Parse `ref:<group>.<column>`; the group is lowercased (config keys are),
/// the column name keeps its case.
pub fn parse_ref(type_name: &str) -> Option<(String, String)> {
    let rest = type_name.trim().strip_prefix("ref:")?;
    let (group, column) = rest.split_once('.')?;
    Some((group.trim().to_lowercase(), column.trim().to_string()))
}

/// Validate a column type spec and return its canonical stored form: either a
/// built-in type name or a normalized `ref:<group>.<column>`.
fn resolve_type_name(type_name: &str) -> Result<String, String> {
    if let Some((group, column)) = parse_ref(type_name) {
        if group.is_empty() || column.is_empty() {
            return Err(t!("cmd.fake.group.bad_ref", type_name.to_string()));
        }
        // Validate eagerly when the target group already exists (a target may
        // legitimately be created afterwards, so a missing group is allowed).
        if let Some(cols) = Config::get_fake_group(&group) {
            if !cols.iter().any(|c| c.alias.eq_ignore_ascii_case(&column)) {
                return Err(t!("cmd.fake.group.ref_col_missing", column, group));
            }
        }
        return Ok(format!("ref:{}.{}", group, column));
    }
    let def = types::find(type_name)
        .ok_or_else(|| t!("cmd.fake.group.unknown_type", type_name.to_string()))?;
    Ok(def.name.to_string())
}

/// `fake group add|edit <name> [<alias> as <type>, …]`
fn add_group(args: &str, edit: bool) -> Result<()> {
    let (name, spec) = match args.split_once(char::is_whitespace) {
        Some((n, s)) => (n.trim(), s.trim()),
        None => (args.trim(), ""),
    };
    if name.is_empty() {
        print_error(&t!("cmd.fake.group.add_usage"));
        return Ok(());
    }
    if !valid_group_name(name) {
        print_error(&t!("cmd.fake.group.bad_name", name));
        return Ok(());
    }
    let cols = match parse_columns(spec) {
        Ok(c) => c,
        Err(e) => {
            print_error(&e);
            return Ok(());
        }
    };

    let exists = Config::get_fake_group(name).is_some();
    if edit && !exists {
        print_error(&t!("cmd.fake.group.not_found", name));
        return Ok(());
    }
    if !edit && exists {
        print_error(&t!("cmd.fake.group.exists", name));
        return Ok(());
    }

    let local = if edit {
        Config::update_fake_group(name, &cols).unwrap_or(false)
    } else {
        Config::add_fake_group(name, &cols).unwrap_or(false)
    };
    let scope = if local {
        t!("cmd.fake.group.scope_local")
    } else {
        t!("cmd.fake.group.scope_global")
    };
    let key = if edit {
        "cmd.fake.group.updated"
    } else {
        "cmd.fake.group.added"
    };
    print_success(&t!(key, name, cols.len(), scope));
    Ok(())
}

fn rm_group(args: &str) -> Result<()> {
    let name = args.trim();
    if name.is_empty() {
        print_error(&t!("cmd.fake.group.rm_usage"));
        return Ok(());
    }
    match Config::remove_fake_group(name) {
        Ok(true) => print_success(&t!("cmd.fake.group.removed", name)),
        Ok(false) => print_error(&t!("cmd.fake.group.not_found", name)),
        Err(e) => print_error(&format!("{}", e)),
    }
    Ok(())
}

fn rename_group(args: &str) -> Result<()> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    // Accept `rnm <old> <new>` and `rnm <old> to <new>`.
    let (old, new) = match parts.as_slice() {
        [old, new] => (*old, *new),
        [old, "to", new] | [old, "->", new] => (*old, *new),
        _ => {
            print_error(&t!("cmd.fake.group.rnm_usage"));
            return Ok(());
        }
    };
    if Config::get_fake_group(old).is_none() {
        print_error(&t!("cmd.fake.group.not_found", old));
        return Ok(());
    }
    if Config::get_fake_group(new).is_some() {
        print_error(&t!("cmd.fake.group.exists", new));
        return Ok(());
    }
    if !valid_group_name(new) {
        print_error(&t!("cmd.fake.group.bad_name", new));
        return Ok(());
    }
    if Config::rename_fake_group(old, new) {
        print_success(&t!("cmd.fake.group.renamed", old, new));
    } else {
        print_error(&t!("cmd.fake.group.not_found", old));
    }
    Ok(())
}

fn list_groups(pattern: &str) -> Result<()> {
    let pattern = pattern.trim();
    let groups = Config::get_fake_groups();
    if groups.is_empty() {
        print_info(&t!("cmd.fake.group.ls_empty"));
        return Ok(());
    }
    let mut names: Vec<String> = groups
        .keys()
        .filter(|n| pattern.is_empty() || glob_match(pattern, n))
        .cloned()
        .collect();
    names.sort();
    if names.is_empty() {
        print_info(&t!("cmd.fake.group.ls_none", pattern));
        return Ok(());
    }
    for name in names {
        let cols = &groups[&name];
        let expanded: Vec<String> = cols
            .iter()
            .map(|c| {
                format!(
                    "{}:{}{}",
                    c.alias,
                    c.type_name,
                    if c.unique { "!" } else { "" }
                )
            })
            .collect();
        println!(
            "  {:<16} {}",
            name.cyan().bold(),
            format!("[{}]", expanded.join(", ")).dimmed()
        );
    }
    Ok(())
}

fn info_group(args: &str) -> Result<()> {
    let name = args.trim();
    if name.is_empty() {
        print_error(&t!("cmd.fake.group.info_usage"));
        return Ok(());
    }
    let cols = match Config::get_fake_group(name) {
        Some(c) => c,
        None => {
            print_error(&t!("cmd.fake.group.not_found", name));
            return Ok(());
        }
    };
    println!("{}", t!("cmd.fake.group.info_title", name).cyan().bold());
    for (i, c) in cols.iter().enumerate() {
        let about = types::find(&c.type_name).map(|t| t.about).unwrap_or("");
        let alias = if c.unique {
            format!("{}!", c.alias)
        } else {
            c.alias.clone()
        };
        println!(
            "  {:>2}. {:<20} {:<16} {}",
            i + 1,
            alias.green(),
            c.type_name.yellow(),
            about.dimmed()
        );
    }
    println!("  {}", t!("cmd.fake.group.info_count", cols.len()).dimmed());
    Ok(())
}

fn clear_groups(args: &str) -> Result<()> {
    let force =
        args.trim().eq_ignore_ascii_case("--force") || args.trim().eq_ignore_ascii_case("-f");
    let count = Config::get_fake_groups().len();
    if count == 0 {
        print_info(&t!("cmd.fake.group.clear_empty"));
        return Ok(());
    }
    if !force {
        print_warning(&t!("cmd.fake.group.clear_prompt", count));
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            print_info(&t!("cmd.fake.group.clear_cancelled"));
            return Ok(());
        }
    }
    match Config::clear_fake_groups() {
        Ok(n) => print_success(&t!("cmd.fake.group.cleared", n)),
        Err(e) => print_error(&format!("{}", e)),
    }
    Ok(())
}

/// A shareable `.ntc_fake` bundle (`.ntc_fake` by default, JSON if the file
/// ends `.json`).
#[derive(Debug, Serialize, Deserialize)]
struct FakeGroupFile {
    #[serde(default = "default_fake_file_version")]
    version: u32,
    #[serde(default)]
    groups: HashMap<String, Vec<FakeColumn>>,
}

fn default_fake_file_version() -> u32 {
    1
}

fn file_is_json(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
}

/// `fake group export [<name>|--all] [-o file]`
fn export_groups(args: &str) -> Result<()> {
    let mut all = false;
    let mut name: Option<String> = None;
    let mut output: Option<PathBuf> = None;

    let toks: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    while i < toks.len() {
        match toks[i] {
            "--all" | "-a" => all = true,
            "-o" | "--output" => {
                i += 1;
                match toks.get(i) {
                    Some(v) => output = Some(PathBuf::from(v)),
                    None => {
                        print_error(&t!("cmd.fake.group.export_usage"));
                        return Ok(());
                    }
                }
            }
            other if other.starts_with('-') => {
                print_error(&t!("cmd.fake.group.export_usage"));
                return Ok(());
            }
            other => name = Some(other.to_string()),
        }
        i += 1;
    }

    let merged = Config::get_fake_groups();
    let groups: HashMap<String, Vec<FakeColumn>> = if all {
        merged
    } else if let Some(n) = &name {
        let key = n.to_lowercase();
        match merged.get(&key) {
            Some(cols) => HashMap::from([(key, cols.clone())]),
            None => {
                print_error(&t!("cmd.fake.group.not_found", n.clone()));
                return Ok(());
            }
        }
    } else {
        print_error(&t!("cmd.fake.group.export_usage"));
        return Ok(());
    };

    if groups.is_empty() {
        print_info(&t!("cmd.fake.group.ls_empty"));
        return Ok(());
    }

    let file = FakeGroupFile { version: 1, groups };
    let json = output.as_deref().is_some_and(file_is_json);
    let text = if json {
        serde_json::to_string_pretty(&file).map_err(|e| anyhow::anyhow!("{e}"))?
    } else {
        toml::to_string_pretty(&file).map_err(|e| anyhow::anyhow!("{e}"))?
    };

    match &output {
        Some(path) => {
            std::fs::write(path, text)?;
            print_success(&t!(
                "cmd.fake.group.exported",
                file.groups.len(),
                path.display().to_string()
            ));
        }
        None => print!("{text}"),
    }
    Ok(())
}

/// `fake group import <file> [--force] [--prefix p]`
fn import_groups(args: &str) -> Result<()> {
    let mut force = false;
    let mut prefix = String::new();
    let mut path: Option<PathBuf> = None;

    let toks: Vec<&str> = args.split_whitespace().collect();
    let mut i = 0;
    while i < toks.len() {
        match toks[i] {
            "--force" | "-f" => force = true,
            "--prefix" | "-p" => {
                i += 1;
                match toks.get(i) {
                    Some(v) => prefix = v.to_string(),
                    None => {
                        print_error(&t!("cmd.fake.group.import_usage"));
                        return Ok(());
                    }
                }
            }
            other if other.starts_with('-') => {
                print_error(&t!("cmd.fake.group.import_usage"));
                return Ok(());
            }
            other => path = Some(PathBuf::from(other)),
        }
        i += 1;
    }

    let Some(path) = path else {
        print_error(&t!("cmd.fake.group.import_usage"));
        return Ok(());
    };
    let raw = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            print_error(&t!("cmd.fake.group.import_bad_file", e.to_string()));
            return Ok(());
        }
    };

    let parsed: std::result::Result<FakeGroupFile, String> = if file_is_json(&path) {
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    } else {
        toml::from_str(&raw).map_err(|e| e.to_string())
    };
    let file = match parsed {
        Ok(f) => f,
        Err(e) => {
            print_error(&t!("cmd.fake.group.import_bad_file", e));
            return Ok(());
        }
    };
    if file.groups.is_empty() {
        print_info(&t!("cmd.fake.group.ls_empty"));
        return Ok(());
    }

    let merged = Config::get_fake_groups();
    let conflicts: Vec<String> = file
        .groups
        .keys()
        .map(|n| format!("{prefix}{n}"))
        .filter(|n| merged.contains_key(&n.to_lowercase()))
        .collect();
    if !conflicts.is_empty() && !force {
        print_error(&t!("cmd.fake.group.import_exists", conflicts.join(", ")));
        return Ok(());
    }

    let mut count = 0usize;
    for (name, cols) in &file.groups {
        let key = format!("{prefix}{name}");
        if Config::add_fake_group(&key, cols).is_ok() {
            count += 1;
        }
    }
    print_success(&t!("cmd.fake.group.import_ok", count));
    Ok(())
}

pub fn print_usage() {
    println!("{}", t!("cmd.fake.group.usage_title").cyan().bold());
    println!("  {}", t!("cmd.fake.group.help_add"));
    println!("  {}", t!("cmd.fake.group.help_edit"));
    println!("  {}", t!("cmd.fake.group.help_rm"));
    println!("  {}", t!("cmd.fake.group.help_rnm"));
    println!("  {}", t!("cmd.fake.group.help_ls"));
    println!("  {}", t!("cmd.fake.group.help_info"));
    println!("  {}", t!("cmd.fake.group.help_clear"));
    println!("  {}", t!("cmd.fake.group.help_export"));
    println!("  {}", t!("cmd.fake.group.help_import"));
    println!();
    println!("{}", t!("cmd.fake.examples_header").green());
    println!("  {}", t!("cmd.fake.group.example1"));
    println!("  {}", t!("cmd.fake.group.example2"));
}

/// Group names: letters/digits/`_`/`-`, no spaces.
fn valid_group_name(name: &str) -> bool {
    !name.trim().is_empty()
        && !name.contains(char::is_whitespace)
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Case-insensitive glob match supporting `*` and `?`, with a substring
/// fallback when the pattern carries no wildcard.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let text = text.to_lowercase();
    if !pattern.contains('*') && !pattern.contains('?') {
        return text.contains(&pattern);
    }
    glob_chars(
        &pattern.chars().collect::<Vec<_>>(),
        &text.chars().collect::<Vec<_>>(),
    )
}

fn glob_chars(pat: &[char], text: &[char]) -> bool {
    // Classic iterative glob with backtracking for `*`.
    let (mut p, mut t) = (0usize, 0usize);
    let (mut star, mut mark) = (None, 0usize);
    while t < text.len() {
        if p < pat.len() && (pat[p] == '?' || pat[p] == text[t]) {
            p += 1;
            t += 1;
        } else if p < pat.len() && pat[p] == '*' {
            star = Some(p);
            mark = t;
            p += 1;
        } else if let Some(sp) = star {
            p = sp + 1;
            mark += 1;
            t = mark;
        } else {
            return false;
        }
    }
    while p < pat.len() && pat[p] == '*' {
        p += 1;
    }
    p == pat.len()
}

/// Names of every stored group (used for tab-completion).
pub fn group_names() -> Vec<String> {
    let mut names: Vec<String> = Config::get_fake_groups().keys().cloned().collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_alias_as_type() {
        let cols = parse_columns("[username as name, year as age]").unwrap();
        assert_eq!(cols[0], FakeColumn::new("username", "name"));
        assert_eq!(cols[1], FakeColumn::new("year", "age"));
    }

    #[test]
    fn bare_type_alias_defaults_to_type() {
        let cols = parse_columns("email, city").unwrap();
        assert_eq!(cols[0], FakeColumn::new("email", "email"));
        assert_eq!(cols[1], FakeColumn::new("city", "city"));
    }

    #[test]
    fn unknown_type_is_rejected() {
        assert!(parse_columns("[x as not_a_type]").is_err());
    }

    #[test]
    fn glob_matches_star_and_question() {
        assert!(glob_match("u*", "users"));
        assert!(glob_match("user?", "users"));
        assert!(!glob_match("x*", "users"));
        // no wildcard → substring
        assert!(glob_match("ser", "users"));
        assert!(!glob_match("zzz", "users"));
    }

    #[test]
    fn parses_unique_marker() {
        let cols = parse_columns("[contract as phone_number !]").unwrap();
        assert!(cols[0].unique);
        assert_eq!(cols[0].alias, "contract");
        assert_eq!(cols[0].type_name, "phone_number");

        // no space before `!`
        let cols = parse_columns("email!").unwrap();
        assert!(cols[0].unique);
        assert_eq!(cols[0].alias, "email");

        let cols = parse_columns("name").unwrap();
        assert!(!cols[0].unique);
    }

    #[test]
    fn parses_ref_columns() {
        let cols = parse_columns("[user_id as ref:zz_nonexistent_grp.id]").unwrap();
        assert_eq!(cols[0].alias, "user_id");
        assert_eq!(cols[0].type_name, "ref:zz_nonexistent_grp.id");

        // bare ref defaults its alias to the target column
        let cols = parse_columns("ref:zz_nonexistent_grp.id").unwrap();
        assert_eq!(cols[0].alias, "id");
        assert_eq!(cols[0].type_name, "ref:zz_nonexistent_grp.id");

        // group is normalized to lowercase
        let cols = parse_columns("[u as ref:Zz_Nonexistent_Grp.Id]").unwrap();
        assert_eq!(cols[0].type_name, "ref:zz_nonexistent_grp.Id");
    }

    #[test]
    fn rejects_malformed_ref() {
        assert!(parse_columns("[x as ref:users]").is_err());
        assert!(parse_columns("[x as ref:]").is_err());
    }
}
