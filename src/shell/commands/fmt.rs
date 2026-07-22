use crate::navigator::Navigator;
use crate::output::{print_error, print_success};
use anyhow::{bail, Result};
use colored::*;
use std::path::{Path, PathBuf};

fn parse_fmt_args(args: &str, cwd: &Path) -> Result<FmtArgs> {
    let args = args.trim();
    if args.is_empty() {
        bail!("Usage: fmt json|yaml|toml <file> [to json|yaml|toml] [-o <file>]");
    }

    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let from_fmt = parts[0].to_lowercase();
    let rest = parts.get(1).unwrap_or(&"").trim();

    if !matches!(from_fmt.as_str(), "json" | "yaml" | "toml") {
        bail!("Unknown format: '{}'. Use json, yaml, or toml.", from_fmt);
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
            bail!("Unknown target format: '{}'. Use json, yaml, or toml.", to);
        }
        (file, Some(to))
    } else {
        (rest_clean, None)
    };

    if file_str.is_empty() {
        bail!("Missing file path");
    }

    let file_path = cwd.join(file_str);
    if !file_path.exists() {
        bail!("File not found: {}", file_str);
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
    serde_json::from_str(content).map_err(|e| anyhow::anyhow!("Invalid JSON: {}", e))
}

fn parse_as_yaml(content: &str) -> Result<serde_json::Value> {
    serde_yaml::from_str(content).map_err(|e| anyhow::anyhow!("Invalid YAML: {}", e))
}

fn parse_as_toml(content: &str) -> Result<serde_json::Value> {
    let value: toml::Value = toml::from_str(content)
        .map_err(|e| anyhow::anyhow!("Invalid TOML: {}", e))?;
    serde_json::to_value(value).map_err(|e| anyhow::anyhow!("TOML conversion error: {}", e))
}

fn serialize_as_json(value: &serde_json::Value) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(|e| anyhow::anyhow!("JSON serialization error: {}", e))
}

fn serialize_as_yaml(value: &serde_json::Value) -> Result<String> {
    serde_yaml::to_string(value).map_err(|e| anyhow::anyhow!("YAML serialization error: {}", e))
}

fn serialize_as_toml(value: &serde_json::Value) -> Result<String> {
    let toml_value = toml::Value::try_from(value.clone())
        .map_err(|e| anyhow::anyhow!("TOML serialization error: {}", e))?;
    toml::to_string_pretty(&toml_value)
        .map_err(|e| anyhow::anyhow!("TOML serialization error: {}", e))
}

fn parse_content(content: &str, fmt: &str) -> Result<serde_json::Value> {
    match fmt {
        "json" => parse_as_json(content),
        "yaml" => parse_as_yaml(content),
        "toml" => parse_as_toml(content),
        _ => bail!("Unknown format: {}", fmt),
    }
}

fn serialize_content(value: &serde_json::Value, fmt: &str) -> Result<String> {
    match fmt {
        "json" => serialize_as_json(value),
        "yaml" => serialize_as_yaml(value),
        "toml" => serialize_as_toml(value),
        _ => bail!("Unknown format: {}", fmt),
    }
}

fn print_usage() {
    println!("{}", "Usage:".cyan().bold());
    println!("  fmt json <file> [to yaml|toml] [-o <file>]    Convert JSON to YAML or TOML");
    println!("  fmt yaml <file> [to json|toml] [-o <file>]    Convert YAML to JSON or TOML");
    println!("  fmt toml <file> [to json|yaml] [-o <file>]    Convert TOML to JSON or YAML");
    println!();
    println!("{}", "If no target format is given, pretty-prints in the source format.".green());
    println!();
    println!("{}", "Examples:".green());
    println!("  fmt json package.json to yaml");
    println!("  fmt json config.json to toml -o config.toml");
    println!("  fmt yaml config.yaml to json");
    println!("  fmt toml Cargo.toml to json");
    println!("  fmt json data.json                      # pretty-print JSON");
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
            print_error(&format!("Failed to read file: {}", e));
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
            print_success(&format!("Written to: {}", path.display()));
        }
        None => {
            println!("{}", output);
        }
    }

    Ok(false)
}
