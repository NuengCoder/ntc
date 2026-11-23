//! Skill source resolution: seed aliases, `owner/repo[/skill]`, GitHub URLs
//! and user aliases from `<config>/ntc/skills/sources.toml`.

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::PathBuf;

/// `(alias, repo, skill filter)` for the documented repositories.
pub const SEED_ALIASES: &[(&str, &str, Option<&str>)] = &[
    ("superpowers", "obra/superpowers", None),
    ("superpower", "obra/superpowers", None),
    ("sp", "obra/superpowers", None),
    ("anthropic", "anthropics/skills", None),
    ("frontenddev", "anthropics/skills", Some("frontend-design")),
    (
        "frontend-design",
        "anthropics/skills",
        Some("frontend-design"),
    ),
    ("mcp-builder", "anthropics/skills", Some("mcp-builder")),
    (
        "webapp-testing",
        "anthropics/skills",
        Some("webapp-testing"),
    ),
    ("skill-creator", "anthropics/skills", Some("skill-creator")),
    (
        "sentry-fix-issues",
        "getsentry/sentry-agent-skills",
        Some("sentry-fix-issues"),
    ),
    (
        "static-analysis",
        "trailofbits/skills",
        Some("static-analysis"),
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias {
    pub repo: String,
    pub skill: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
    pub owner: String,
    pub repo: String,
    pub git_ref: Option<String>,
    /// Restrict a multi-skill repo to one skill.
    pub skill: Option<String>,
    /// Bundle name recorded in the registry (`superpowers`, `frontenddev`, ...).
    pub bundle: String,
}

/// Seed aliases only (used by tests and by the unknown-alias hint).
pub fn seed_aliases() -> BTreeMap<String, Alias> {
    SEED_ALIASES
        .iter()
        .map(|(name, repo, skill)| {
            (
                (*name).to_string(),
                Alias {
                    repo: (*repo).to_string(),
                    skill: skill.map(str::to_string),
                },
            )
        })
        .collect()
}

fn sources_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("ntc").join("skills").join("sources.toml"))
}

#[derive(serde::Deserialize)]
struct SourcesFile {
    #[serde(default)]
    aliases: BTreeMap<String, SourceEntry>,
}

#[derive(serde::Deserialize)]
struct SourceEntry {
    repo: String,
    #[serde(default)]
    skill: Option<String>,
}

pub fn aliases() -> BTreeMap<String, Alias> {
    let mut out = seed_aliases();
    let Some(path) = sources_path() else {
        return out;
    };
    let Ok(text) = fs::read_to_string(path) else {
        return out;
    };
    let Ok(parsed) = toml::from_str::<SourcesFile>(&text) else {
        return out;
    };
    for (name, entry) in parsed.aliases {
        let name = name.to_lowercase();
        if name.is_empty() || entry.repo.trim().is_empty() {
            continue;
        }
        out.insert(
            name,
            Alias {
                repo: entry.repo.trim().to_string(),
                skill: entry.skill.map(|s| s.trim().to_string()),
            },
        );
    }
    out
}

pub fn alias(name: &str) -> Option<Alias> {
    aliases().remove(&name.to_lowercase())
}

/// One entry of `skill avail`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Available {
    pub alias: String,
    pub repo: String,
    pub skill: Option<String>,
}

/// Built-in and user aliases, one line per distinct source. Shortcut aliases
/// that point at the same repo/skill (`sp`, `superpower`) collapse into the
/// first name in the seed table.
pub fn available() -> Vec<Available> {
    let merged = aliases();
    let mut out: Vec<Available> = Vec::new();
    let mut seen_pairs: HashSet<(String, Option<String>)> = HashSet::new();
    let mut seen_aliases: HashSet<String> = HashSet::new();
    for (name, repo, skill) in SEED_ALIASES {
        let alias = merged.get(*name).cloned().unwrap_or_else(|| Alias {
            repo: (*repo).to_string(),
            skill: skill.map(str::to_string),
        });
        seen_aliases.insert((*name).to_string());
        if seen_pairs.insert((alias.repo.clone(), alias.skill.clone())) {
            out.push(Available {
                alias: (*name).to_string(),
                repo: alias.repo,
                skill: alias.skill,
            });
        }
    }
    for (name, alias) in merged {
        if seen_aliases.contains(&name) {
            continue;
        }
        if seen_pairs.insert((alias.repo.clone(), alias.skill.clone())) {
            out.push(Available {
                alias: name,
                repo: alias.repo,
                skill: alias.skill,
            });
        }
    }
    out
}

/// Split `owner/repo` into its two halves after basic validation.
fn split_repo(spec: &str) -> Result<(String, String)> {
    let mut parts = spec
        .trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .split('/')
        .filter(|p| !p.is_empty());
    let owner = parts
        .next()
        .context("skill source must include an owner")?
        .to_string();
    let repo = parts
        .next()
        .context("skill source must include a repository")?
        .to_string();
    if parts.next().is_some() {
        bail!("unexpected path in skill source '{}'", spec);
    }
    if !owner
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        bail!("invalid owner '{}' in skill source", owner);
    }
    if !repo
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        bail!("invalid repository '{}' in skill source", repo);
    }
    Ok((owner, repo))
}

fn spec_from_alias(base: &str, git_ref: Option<String>) -> Result<Spec> {
    let name = base.to_lowercase();
    let Some(found) = aliases().remove(&name) else {
        let seed = seed_aliases();
        let known: Vec<&str> = seed.keys().map(|s| s.as_str()).collect();
        bail!(
            "unknown skill '{}' (built-in aliases: {}; or use owner/repo)",
            base,
            known.join(", ")
        );
    };
    let (owner, repo) = split_repo(&found.repo).with_context(|| {
        format!(
            "alias '{}' points at an invalid repo '{}'",
            base, found.repo
        )
    })?;
    Ok(Spec {
        owner,
        repo,
        git_ref,
        skill: found.skill,
        bundle: name,
    })
}

/// Resolve one user-supplied source into an owner/repo pair (plus optional
/// skill filter). Supports aliases, `owner/repo`, `owner/repo/skill` and a
/// `github.com` URL, each with an optional `#ref`. Listing pages on
/// `agenticskills.io` resolve through the same alias table.
pub fn parse_spec(input: &str) -> Result<Spec> {
    let raw = input.trim();
    if raw.is_empty() {
        bail!("missing skill source");
    }
    let (base, git_ref) = match raw.split_once('#') {
        Some((base, frag)) if !frag.trim().is_empty() => {
            (base.trim(), Some(frag.trim().to_string()))
        }
        _ => (raw, None),
    };

    if base.starts_with("http://") {
        bail!("only https URLs are supported for skill sources");
    }

    // `https://agenticskills.io/skills/<alias>` is a listing page: the slug
    // names a known skill whose source lives on GitHub.
    if let Some(rest) = base.strip_prefix("https://agenticskills.io/skills/") {
        let slug = rest.trim_matches('/');
        if slug.is_empty() || slug.contains('/') {
            bail!("unsupported skill listing '{}'", base);
        }
        return spec_from_alias(slug, git_ref);
    }

    if let Some(rest) = base.strip_prefix("https://") {
        let (host, path) = rest
            .split_once('/')
            .context("GitHub URL must include owner/repo")?;
        if !matches!(host, "github.com" | "www.github.com") {
            bail!(
                "unsupported skill host '{}' (only github.com is supported)",
                host
            );
        }
        let mut parts = path
            .trim_end_matches('/')
            .split('/')
            .filter(|p| !p.is_empty());
        let owner = parts.next().context("GitHub URL must include an owner")?;
        let repo = parts
            .next()
            .context("GitHub URL must include a repository")?;
        let skill = parts.next().map(str::to_string);
        if parts.next().is_some() {
            bail!("unsupported GitHub URL '{}'", base);
        }
        let (owner, repo) = split_repo(&format!("{owner}/{repo}"))?;
        let bundle = skill.clone().unwrap_or_else(|| repo.clone());
        return Ok(Spec {
            owner,
            repo,
            git_ref,
            skill,
            bundle,
        });
    }

    // Alias first: `frontend-design` resolves through the seed table even
    // though it is not an `owner/repo` pair.
    if !base.contains('/') {
        return spec_from_alias(base, git_ref);
    }

    let mut parts = base
        .trim_end_matches('/')
        .split('/')
        .filter(|p| !p.is_empty());
    let owner = parts.next().unwrap_or_default();
    let repo = parts
        .next()
        .context("skill source must include a repository")?;
    let skill = parts.next().map(str::to_string);
    if parts.next().is_some() {
        bail!("skill source '{}' has too many path segments", base);
    }
    let (owner, repo) = split_repo(&format!("{owner}/{repo}"))?;
    let bundle = skill.clone().unwrap_or_else(|| repo.clone());
    Ok(Spec {
        owner,
        repo,
        git_ref,
        skill,
        bundle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_resolve_to_the_documented_repos() {
        let a = parse_spec("superpowers").unwrap();
        assert_eq!((a.owner.as_str(), a.repo.as_str()), ("obra", "superpowers"));
        assert_eq!(a.bundle, "superpowers");
        assert!(a.skill.is_none());

        let b = parse_spec("frontenddev").unwrap();
        assert_eq!(
            (b.owner.as_str(), b.repo.as_str()),
            ("anthropics", "skills")
        );
        assert_eq!(b.skill.as_deref(), Some("frontend-design"));
        assert_eq!(b.bundle, "frontenddev");

        // `superpower` (singular, as the todo writes it) is the same bundle.
        assert_eq!(parse_spec("superpower").unwrap().bundle, "superpower");
        assert_eq!(parse_spec("SP").unwrap().repo, "superpowers");
    }

    #[test]
    fn parses_owner_repo_and_single_skill() {
        let a = parse_spec("obra/superpowers").unwrap();
        assert_eq!(a.bundle, "superpowers");
        assert!(a.skill.is_none());

        let b = parse_spec("anthropics/skills/frontend-design").unwrap();
        assert_eq!(b.skill.as_deref(), Some("frontend-design"));
        assert_eq!(b.bundle, "frontend-design");
    }

    #[test]
    fn parses_github_urls_and_refs() {
        let a = parse_spec("https://github.com/obra/superpowers").unwrap();
        assert_eq!((a.owner.as_str(), a.repo.as_str()), ("obra", "superpowers"));

        let b = parse_spec("https://github.com/obra/superpowers#v1.2").unwrap();
        assert_eq!(b.git_ref.as_deref(), Some("v1.2"));

        let c = parse_spec("https://github.com/obra/superpowers.git/").unwrap();
        assert_eq!(c.repo, "superpowers");

        let d = parse_spec("obra/superpowers#dev").unwrap();
        assert_eq!(d.git_ref.as_deref(), Some("dev"));
    }

    #[test]
    fn rejects_non_github_hosts_and_plain_http() {
        assert!(parse_spec("https://example.com/owner/repo").is_err());
        assert!(parse_spec("http://github.com/owner/repo").is_err());
    }

    #[test]
    fn rejects_unknown_aliases_and_junk() {
        let err = parse_spec("nope").unwrap_err().to_string();
        assert!(err.contains("superpowers"), "{err}");
        assert!(parse_spec("owner").is_err());
        assert!(parse_spec("owner/repo/extra/more").is_err());
        assert!(parse_spec("   ").is_err());
    }

    #[test]
    fn seed_catalog_has_no_duplicates() {
        let mut names: Vec<&str> = SEED_ALIASES.iter().map(|(n, _, _)| *n).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len());
        for (_, repo, _) in SEED_ALIASES {
            assert!(repo.contains('/'), "{repo}");
        }
    }
}
