//! Parse and validate the YAML frontmatter at the top of a `SKILL.md`.
//!
//! Only `name` and `description` are required. The name doubles as the folder
//! name in the library and in every harness, so it is validated strictly: a
//! traversal or uppercase name is a hard error, never a silent rename.

use anyhow::{bail, Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontmatter {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
struct Raw {
    name: Option<String>,
    description: Option<String>,
}

/// `^[a-z0-9][a-z0-9-]*$` without pulling in a regex dependency.
pub fn valid_skill_name(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    name.len() <= 64 && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub fn parse_skill_md(text: &str) -> Result<Frontmatter> {
    // Editors on Windows happily add a BOM and CRLF endings.
    let text = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
    let mut lines = text.lines();
    if lines.next().map(str::trim_end) != Some("---") {
        bail!("SKILL.md must start with a '---' frontmatter block");
    }
    let mut yaml = String::new();
    let mut closed = false;
    for line in lines {
        if line.trim_end() == "---" {
            closed = true;
            break;
        }
        yaml.push_str(line);
        yaml.push('\n');
    }
    if !closed {
        bail!("SKILL.md frontmatter is not closed with '---'");
    }

    let raw: Raw = serde_yaml::from_str(&yaml).context("parse SKILL.md frontmatter")?;
    let name = raw.name.unwrap_or_default().trim().to_string();
    let description = raw.description.unwrap_or_default().trim().to_string();
    if name.is_empty() {
        bail!("SKILL.md frontmatter is missing 'name'");
    }
    if description.is_empty() {
        bail!("SKILL.md frontmatter is missing 'description'");
    }
    if !valid_skill_name(&name) {
        bail!(
            "SKILL.md name '{}' is not a safe folder name (lowercase letters, digits and '-')",
            name
        );
    }
    Ok(Frontmatter { name, description })
}

/// Replace the frontmatter `name:` value, keeping the rest of the file (and
/// its line endings) untouched. Used by `skill download --name`.
pub fn rename_skill_md(text: &str, new_name: &str) -> Result<String> {
    if !valid_skill_name(new_name) {
        bail!(
            "'{}' is not a safe skill name (lowercase letters, digits and '-')",
            new_name
        );
    }
    let bom = if text.starts_with('\u{feff}') {
        "\u{feff}"
    } else {
        ""
    };
    let body = text.trim_start_matches('\u{feff}');
    let mut out = String::new();
    let mut in_front = false;
    let mut closed = false;
    let mut renamed = false;
    for (index, raw) in body.split_inclusive('\n').enumerate() {
        let line = raw.trim_end_matches(['\r', '\n']);
        let ending = &raw[line.len()..];
        if index == 0 && line.trim_end() == "---" {
            in_front = true;
            out.push_str(raw);
            continue;
        }
        if in_front && !closed && line.trim_end() == "---" {
            closed = true;
            out.push_str(raw);
            continue;
        }
        if in_front && !closed && !renamed && line.trim_start().starts_with("name:") {
            out.push_str(&format!("name: {}{}", new_name, ending));
            renamed = true;
            continue;
        }
        out.push_str(raw);
    }
    if !closed {
        bail!("SKILL.md frontmatter is not closed with '---'");
    }
    if !renamed {
        bail!("SKILL.md frontmatter is missing 'name'");
    }
    let out = format!("{}{}", bom, out);
    // Never write a file the parser would reject afterwards.
    let parsed = parse_skill_md(&out)?;
    if parsed.name != new_name {
        bail!("could not rename SKILL.md to '{}'", new_name);
    }
    Ok(out)
}

/// Parse a skill that lives in `dir_name` and require the two to agree.
pub fn validate_in_dir(text: &str, dir_name: &str) -> Result<Frontmatter> {
    let fm = parse_skill_md(text)?;
    if fm.name != dir_name {
        bail!(
            "SKILL.md name '{}' does not match its folder '{}'",
            fm.name,
            dir_name
        );
    }
    Ok(fm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "---\nname: brainstorming\ndescription: \"Use this before any creative work\"\n---\n\n# Brainstorming\n";

    #[test]
    fn parses_valid_frontmatter() {
        let fm = parse_skill_md(VALID).unwrap();
        assert_eq!(fm.name, "brainstorming");
        assert_eq!(fm.description, "Use this before any creative work");
    }

    #[test]
    fn accepts_bom_and_crlf() {
        let text = format!("\u{feff}{}", VALID.replace('\n', "\r\n"));
        let fm = parse_skill_md(&text).unwrap();
        assert_eq!(fm.name, "brainstorming");
    }

    #[test]
    fn requires_description() {
        let text = "---\nname: brainstorming\n---\n";
        let err = parse_skill_md(text).unwrap_err().to_string();
        assert!(err.contains("description"), "{err}");
    }

    #[test]
    fn rejects_unsafe_names() {
        for name in ["../x", "Bad Name", "UPPER", "-leading", "with/slash"] {
            assert!(!valid_skill_name(name), "{name} must be rejected");
        }
        assert!(valid_skill_name("brainstorming"));
        assert!(valid_skill_name("frontend-design"));
        assert!(valid_skill_name("skill2"));
    }

    #[test]
    fn folder_mismatch_is_reported() {
        let err = validate_in_dir(VALID, "other").unwrap_err().to_string();
        assert!(
            err.contains("brainstorming") && err.contains("other"),
            "{err}"
        );
    }

    #[test]
    fn unclosed_frontmatter_is_an_error() {
        assert!(parse_skill_md("---\nname: x\n").is_err());
        assert!(parse_skill_md("# no frontmatter").is_err());
    }

    #[test]
    fn rename_keeps_the_body_and_rewrites_the_name() {
        let renamed = rename_skill_md(VALID, "my-skill").unwrap();
        assert!(renamed.contains("name: my-skill\n"), "{renamed}");
        assert!(renamed.contains("# Brainstorming"), "{renamed}");
        assert_eq!(parse_skill_md(&renamed).unwrap().name, "my-skill");

        let crlf = format!("\u{feff}{}", VALID.replace('\n', "\r\n"));
        let renamed = rename_skill_md(&crlf, "other").unwrap();
        assert!(renamed.contains("\r\n"), "{renamed}");
        assert_eq!(parse_skill_md(&renamed).unwrap().name, "other");
    }

    #[test]
    fn rename_rejects_unsafe_names_and_missing_frontmatter() {
        assert!(rename_skill_md(VALID, "Bad Name").is_err());
        assert!(rename_skill_md("# no frontmatter", "ok").is_err());
    }
}
