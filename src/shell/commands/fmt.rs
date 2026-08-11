use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use crate::t;
use anyhow::{bail, Result};
use colored::*;
use std::path::{Path, PathBuf};

fn parse_fmt_args(args: &str, cwd: &Path) -> Result<FmtArgs> {
    let args = args.trim();
    if args.is_empty() {
        bail!("{}", t!("cmd.fmt.usage"));
    }

    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let from_fmt = parts[0].to_lowercase();
    let rest = parts.get(1).unwrap_or(&"").trim();

    if !matches!(from_fmt.as_str(), "json" | "yaml" | "toml") {
        bail!("{}", t!("cmd.fmt.unknown_format", from_fmt));
    }

    // Extract -o flag first (from the end)
    let (rest_clean, out_path) = {
        let r = rest;
        if let Some(pos) = r.rfind(" -o ") {
            let target = r[..pos].trim();
            let out = r[pos + 4..].trim();
            if !out.is_empty() && !out.starts_with('-') {
                (target, Some(cwd.join(out)))
            } else {
                (r, None)
            }
        } else if let Some(pos) = r.rfind(" --output ") {
            let target = r[..pos].trim();
            let out = r[pos + 10..].trim();
            if !out.is_empty() && !out.starts_with('-') {
                (target, Some(cwd.join(out)))
            } else {
                (r, None)
            }
        } else {
            (r, None)
        }
    };

    // Check for "to <fmt>"
    let (file_str, to_fmt) = if let Some(pos) = rest_clean.find(" to ") {
        let file = rest_clean[..pos].trim();
        let to = rest_clean[pos + 4..].trim().to_lowercase();
        if !matches!(to.as_str(), "json" | "yaml" | "toml") {
            bail!("{}", t!("cmd.fmt.unknown_target", to));
        }
        (file, Some(to))
    } else {
        (rest_clean, None)
    };

    if file_str.is_empty() {
        bail!("{}", t!("cmd.fmt.missing_file"));
    }

    let file_path = cwd.join(file_str);
    if !file_path.exists() {
        bail!("{}", t!("cmd.fmt.file_not_found", file_str));
    }

    Ok(FmtArgs {
        from: from_fmt,
        file: file_path,
        to: to_fmt,
        output: out_path,
    })
}

struct FmtArgs {
    from: String,
    file: PathBuf,
    to: Option<String>,
    output: Option<PathBuf>,
}

fn parse_as_json(content: &str) -> Result<serde_json::Value> {
    serde_json::from_str(content).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_json", e)))
}

fn parse_as_yaml(content: &str) -> Result<serde_json::Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_yaml", e)))
}

fn parse_as_toml(content: &str) -> Result<serde_json::Value> {
    let value: toml::Value = toml::from_str(content)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.invalid_toml", e)))?;
    serde_json::to_value(value).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_conv_err", e)))
}

fn serialize_as_json(value: &serde_json::Value) -> Result<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.json_ser_err", e)))
}

fn serialize_as_yaml(value: &serde_json::Value) -> Result<String> {
    serde_yaml::to_string(value).map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.yaml_ser_err", e)))
}

fn serialize_as_toml(value: &serde_json::Value) -> Result<String> {
    let toml_value = toml::Value::try_from(value.clone())
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_ser_err", e)))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| anyhow::anyhow!("{}", t!("cmd.fmt.toml_ser_err", e)))
}

fn parse_content(content: &str, fmt: &str) -> Result<serde_json::Value> {
    match fmt {
        "json" => parse_as_json(content),
        "yaml" => parse_as_yaml(content),
        "toml" => parse_as_toml(content),
        _ => bail!("{}", t!("cmd.fmt.unknown_format", fmt)),
    }
}

fn serialize_content(value: &serde_json::Value, fmt: &str) -> Result<String> {
    match fmt {
        "json" => serialize_as_json(value),
        "yaml" => serialize_as_yaml(value),
        "toml" => serialize_as_toml(value),
        _ => bail!("{}", t!("cmd.fmt.unknown_format", fmt)),
    }
}

fn print_usage() {
    println!("{}", t!("fmt.help_usage").cyan().bold());
    println!("  {}", t!("fmt.help_json_line"));
    println!("  {}", t!("fmt.help_yaml_line"));
    println!("  {}", t!("fmt.help_toml_line"));
    println!();
    println!("{}", t!("fmt.help_no_target").green());
    println!();
    println!("{}", t!("fmt.help_examples").green());
    println!("  {}", t!("fmt.help_example1"));
    println!("  {}", t!("fmt.help_example2"));
    println!("  {}", t!("fmt.help_example3"));
    println!("  {}", t!("fmt.help_example4"));
    println!("  {}", t!("fmt.help_example5"));
}

pub fn cmd_fmt(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();

    let fmt_args = match parse_fmt_args(args, &cwd) {
        Ok(a) => a,
        Err(e) => {
            print_error(&e.to_string());
            println!();
            print_usage();
            return Ok(false);
        }
    };

    let content_raw = match std::fs::read_to_string(&fmt_args.file) {
        Ok(c) => c,
        Err(e) => {
            print_error(&t!("cmd.fmt.read_fail", e));
            return Ok(false);
        }
    };
    let content = content_raw.trim_start_matches('\u{feff}').to_string();

    let value = match parse_content(&content, &fmt_args.from) {
        Ok(v) => v,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    let target_fmt = fmt_args.to.as_deref().unwrap_or(&fmt_args.from);
    let output = match serialize_content(&value, target_fmt) {
        Ok(s) => s,
        Err(e) => {
            print_error(&e.to_string());
            return Ok(false);
        }
    };

    match &fmt_args.output {
        Some(path) => {
            std::fs::write(path, &output)?;
            print_success(&t!("cmd.fmt.written_to", path.display()));
        }
        None => {
            println!("{}", output);
        }
    }

    Ok(false)
}
