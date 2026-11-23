//! ntc skill manager: download `SKILL.md` folders from GitHub, keep them in
//! an ntc-owned library and copy them into AI harnesses.
//!
//! Storage lives under `<config>/ntc/skills/`:
//!
//! ```text
//! registry.toml
//! sources.toml     (optional user aliases)
//! library/<skill>/
//! ```
//!
//! Everything here is local: downloaded files are copied as text/assets and
//! never executed.

pub mod catalog;
pub mod frontmatter;
pub mod github;
pub mod harness;
pub mod registry;

use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use registry::{AddedRecord, BundleRecord, Registry, SkillRecord};

pub struct Paths {
    pub root: PathBuf,
    pub library: PathBuf,
    pub registry: PathBuf,
}

impl Paths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            library: root.join("library"),
            registry: root.join("registry.toml"),
            root,
        }
    }
}

pub fn skills_root() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("skills")
}

fn default_paths() -> Paths {
    Paths::new(skills_root())
}

pub fn load_registry() -> Result<Registry> {
    load_registry_at(&default_paths())
}

fn load_registry_at(paths: &Paths) -> Result<Registry> {
    Registry::load(&paths.registry)
}

fn save_registry_at(paths: &Paths, registry: &Registry) -> Result<()> {
    registry.save(&paths.registry)
}

#[derive(Debug, Clone)]
pub struct Downloaded {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Found {
    pub dir: PathBuf,
    pub name: String,
    pub description: String,
}

pub struct Entry {
    pub name: String,
    pub description: String,
    pub bundle: String,
    pub added: Vec<AddedRecord>,
}

/// Every skill found anywhere in the extracted tree (repo layouts differ, so
/// discovery never assumes a top-level `skills/` folder).
///
/// Real repositories do not always name the folder after the frontmatter
/// (anthropics/skills ships `template/` with `name: template-skill`), so the
/// frontmatter name wins and unparseable folders are reported, not fatal.
fn discover_skills(root: &Path) -> (Vec<Found>, Vec<(PathBuf, String)>) {
    let mut out = Vec::new();
    let mut skipped = Vec::new();
    let walk = walkdir::WalkDir::new(root)
        .max_depth(6)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !e.file_name().to_string_lossy().starts_with('.'));
    for entry in walk {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() || entry.file_name() != "SKILL.md" {
            continue;
        }
        let Some(dir) = entry.path().parent() else {
            continue;
        };
        let text = match fs::read_to_string(entry.path()) {
            Ok(text) => text,
            Err(e) => {
                skipped.push((entry.path().to_path_buf(), format!("{:#}", e)));
                continue;
            }
        };
        match frontmatter::parse_skill_md(&text) {
            Ok(fm) => out.push(Found {
                dir: dir.to_path_buf(),
                name: fm.name,
                description: fm.description,
            }),
            Err(e) => skipped.push((entry.path().to_path_buf(), format!("{:#}", e))),
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.dedup_by(|a, b| a.name == b.name);
    (out, skipped)
}

pub fn download(input: &str, name: Option<&str>, force: bool) -> Result<Vec<Downloaded>> {
    download_at(&default_paths(), input, name, force)
}

fn download_at(
    paths: &Paths,
    input: &str,
    name: Option<&str>,
    force: bool,
) -> Result<Vec<Downloaded>> {
    let spec = catalog::parse_spec(input)?;
    let fetched = github::fetch_skills(&spec.owner, &spec.repo, spec.git_ref.as_deref())?;

    fs::create_dir_all(&paths.root).with_context(|| format!("create {}", paths.root.display()))?;
    let tmp = paths.root.join(format!(
        ".tmp-{}-{}",
        std::process::id(),
        rand::random::<u64>()
    ));
    fs::create_dir_all(&tmp).with_context(|| format!("create {}", tmp.display()))?;

    let result = (|| -> Result<Vec<Downloaded>> {
        let repo_dir = tmp.join("repo");
        github::extract_tar_gz(&fetched.bytes, &repo_dir)?;
        let (found, skipped) = discover_skills(&repo_dir);
        for (path, reason) in &skipped {
            crate::output::print_warning(&format!(
                "skipped {}: {}",
                path.strip_prefix(&repo_dir).unwrap_or(path).display(),
                reason
            ));
        }
        if found.is_empty() {
            bail!(
                "no SKILL.md found in {}/{} at ref '{}'",
                spec.owner,
                spec.repo,
                fetched.used_ref
            );
        }

        let chosen: Vec<Found> = match &spec.skill {
            Some(want) => {
                // Match either the frontmatter name or the folder name so a
                // repo that renames its template still resolves.
                let matched: Vec<Found> = found
                    .into_iter()
                    .filter(|f| {
                        &f.name == want
                            || f.dir.file_name().and_then(|n| n.to_str()) == Some(want.as_str())
                    })
                    .collect();
                if matched.is_empty() {
                    bail!("no skill named '{}' in {}/{}", want, spec.owner, spec.repo);
                }
                matched
            }
            None => found,
        };

        if let Some(new_name) = name {
            if chosen.len() != 1 {
                bail!(
                    "--name needs exactly one skill but the source has {}; ask for one skill first",
                    chosen.len()
                );
            }
            if !frontmatter::valid_skill_name(new_name) {
                bail!(
                    "'{}' is not a safe skill name (lowercase letters, digits and '-')",
                    new_name
                );
            }
        }

        let mut registry = load_registry_at(paths)?;
        if !force {
            if let Some(clash) = chosen.iter().find(|f| {
                let final_name = name.unwrap_or(f.name.as_str());
                registry.skills.contains_key(final_name)
            }) {
                let final_name = name.unwrap_or(clash.name.as_str());
                bail!(
                    "skill '{}' is already installed (use --force to replace it)",
                    final_name
                );
            }
        }

        fs::create_dir_all(&paths.library)
            .with_context(|| format!("create {}", paths.library.display()))?;
        let now = chrono::Utc::now().to_rfc3339();
        let source = format!("{}/{}", spec.owner, spec.repo);
        let bundle_name = name
            .map(str::to_string)
            .unwrap_or_else(|| spec.bundle.clone());
        let mut downloaded = Vec::new();

        for (index, skill) in chosen.iter().enumerate() {
            let final_name = name.unwrap_or(skill.name.as_str()).to_string();
            let dst = paths.library.join(&final_name);
            let stage = paths.library.join(format!(
                ".stage-{}-{}-{}",
                std::process::id(),
                index,
                rand::random::<u64>()
            ));
            harness::copy_dir(&skill.dir, &stage)?;
            if dst.exists() {
                fs::remove_dir_all(&dst).with_context(|| format!("replace {}", dst.display()))?;
            }
            if fs::rename(&stage, &dst).is_err() {
                harness::copy_dir(&stage, &dst)?;
                fs::remove_dir_all(&stage)?;
            }

            // `--name` renames the skill itself, not just the folder: every
            // harness reads the name from the frontmatter.
            if let Some(new_name) = name {
                let md = dst.join("SKILL.md");
                let text =
                    fs::read_to_string(&md).with_context(|| format!("read {}", md.display()))?;
                fs::write(&md, frontmatter::rename_skill_md(&text, new_name)?)
                    .with_context(|| format!("write {}", md.display()))?;
            }

            let sha256 = harness::sha256_file(&dst.join("SKILL.md"))?;
            let rel = skill
                .dir
                .strip_prefix(&repo_dir)
                .unwrap_or(&skill.dir)
                .to_string_lossy()
                .replace('\\', "/");
            let previous_added = registry
                .skills
                .get(&final_name)
                .map(|rec| rec.added.clone())
                .unwrap_or_default();
            registry.skills.insert(
                final_name.clone(),
                SkillRecord {
                    bundle: bundle_name.clone(),
                    source: source.clone(),
                    path: rel,
                    description: skill.description.clone(),
                    sha256,
                    installed_at: now.clone(),
                    added: previous_added,
                },
            );
            downloaded.push(Downloaded {
                name: final_name,
                description: skill.description.clone(),
            });
        }

        let members: Vec<String> = downloaded.iter().map(|d| d.name.clone()).collect();
        let bundle = registry
            .bundles
            .entry(bundle_name.clone())
            .or_insert_with(|| BundleRecord {
                source: source.clone(),
                git_ref: fetched.used_ref.clone(),
                commit: None,
                installed_at: now.clone(),
                skills: Vec::new(),
            });
        bundle.source = source;
        bundle.git_ref = fetched.used_ref.clone();
        bundle.commit = fetched.commit.clone();
        bundle.installed_at = now;
        for member in members {
            if !bundle.skills.contains(&member) {
                bundle.skills.push(member);
            }
        }
        bundle.skills.sort();

        save_registry_at(paths, &registry)?;
        Ok(downloaded)
    })();

    let _ = fs::remove_dir_all(&tmp);
    result
}

/// Expand a skill name or bundle name into concrete skill names.
fn resolve_skills(registry: &Registry, name: &str) -> Result<Vec<String>> {
    let name = name.trim();
    if name.is_empty() {
        bail!("missing skill name");
    }
    if registry.skills.contains_key(name) {
        return Ok(vec![name.to_string()]);
    }
    if let Some(bundle) = registry.bundles.get(name) {
        if !bundle.skills.is_empty() {
            return Ok(bundle.skills.clone());
        }
    }
    if let Some(alias) = catalog::alias(name) {
        let repo = alias.repo.to_lowercase();
        let repo_name = repo.rsplit('/').next().unwrap_or_default().to_string();
        for key in [repo_name, name.to_lowercase()] {
            if let Some(bundle) = registry.bundles.get(&key) {
                if !bundle.skills.is_empty() {
                    return Ok(bundle.skills.clone());
                }
            }
        }
        if let Some(bundle) = registry
            .bundles
            .values()
            .find(|b| b.source.to_lowercase() == repo)
        {
            if !bundle.skills.is_empty() {
                return Ok(bundle.skills.clone());
            }
        }
    }
    bail!(
        "'{}' is not installed; run 'skill download {}' first",
        name,
        name
    )
}

fn unique_skills(registry: &Registry, names: &[String]) -> Result<Vec<String>> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for name in names {
        for skill in resolve_skills(registry, name)? {
            if seen.insert(skill.clone()) {
                out.push(skill);
            }
        }
    }
    Ok(out)
}

pub fn add(
    names: &[String],
    harness: &str,
    global: bool,
    force: bool,
) -> Result<Vec<(String, PathBuf)>> {
    let root = harness::target_root(harness, global)?;
    add_to(&default_paths(), names, harness, global, &root, force)
}

fn add_to(
    paths: &Paths,
    names: &[String],
    harness: &str,
    global: bool,
    target_root: &Path,
    force: bool,
) -> Result<Vec<(String, PathBuf)>> {
    let harness = harness::canonical_harness(harness)?;
    let scope = if global { "global" } else { "project" };
    let mut registry = load_registry_at(paths)?;
    let skills = unique_skills(&registry, names)?;
    let mut copied = Vec::new();

    for skill_name in skills {
        let src = paths.library.join(&skill_name);
        if !src.is_dir() {
            bail!(
                "library copy of '{}' is missing; download it again",
                skill_name
            );
        }
        let dst = target_root.join(&skill_name);
        let record = registry
            .skills
            .get_mut(&skill_name)
            .with_context(|| format!("skill '{}' is not in the registry", skill_name))?;
        let managed = record
            .added
            .iter()
            .any(|add| add.harness == harness && add.scope == scope);
        if dst.exists() {
            if !managed && !force {
                bail!(
                    "{} already exists and was not installed by ntc (use --force to replace it)",
                    dst.display()
                );
            }
            harness::remove_dir(&dst)?;
        }
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        harness::copy_dir(&src, &dst)?;
        let path = dst.to_string_lossy().to_string();
        if !record
            .added
            .iter()
            .any(|add| add.harness == harness && add.scope == scope && add.path == path)
        {
            record.added.push(AddedRecord {
                harness: harness.to_string(),
                scope: scope.to_string(),
                path: path.clone(),
                added_at: chrono::Utc::now().to_rfc3339(),
            });
        }
        copied.push((skill_name, dst));
    }

    save_registry_at(paths, &registry)?;
    Ok(copied)
}

pub fn remove(
    names: &[String],
    harness: &str,
    global: bool,
    force: bool,
) -> Result<Vec<(String, PathBuf)>> {
    let root = harness::target_root(harness, global)?;
    remove_from(&default_paths(), names, harness, global, &root, force)
}

fn remove_from(
    paths: &Paths,
    names: &[String],
    harness: &str,
    global: bool,
    _target_root: &Path,
    force: bool,
) -> Result<Vec<(String, PathBuf)>> {
    let harness = harness::canonical_harness(harness)?;
    let scope = if global { "global" } else { "project" };
    let mut registry = load_registry_at(paths)?;
    let skills = unique_skills(&registry, names)?;
    let mut removed = Vec::new();

    for skill_name in skills {
        let record = registry
            .skills
            .get_mut(&skill_name)
            .with_context(|| format!("skill '{}' is not in the registry", skill_name))?;
        let matching: Vec<usize> = record
            .added
            .iter()
            .enumerate()
            .filter(|(_, add)| add.harness == harness && add.scope == scope)
            .map(|(i, _)| i)
            .collect();
        if matching.is_empty() {
            bail!(
                "'{}' is not added to {} ({} scope)",
                skill_name,
                harness,
                scope
            );
        }
        let recorded_sha = record.sha256.clone();
        let mut keep = Vec::new();
        for (index, add) in record.added.iter().enumerate() {
            if !matching.contains(&index) {
                keep.push(add.clone());
                continue;
            }
            let path = PathBuf::from(&add.path);
            if path.exists() {
                let current = harness::sha256_file(&path.join("SKILL.md")).unwrap_or_default();
                if !force && current != recorded_sha {
                    bail!(
                        "{} was edited after it was added (use --force to remove it)",
                        path.display()
                    );
                }
                harness::remove_dir(&path)?;
            }
            removed.push((skill_name.clone(), path));
        }
        record.added = keep;
    }

    save_registry_at(paths, &registry)?;
    Ok(removed)
}

pub fn delete(names: &[String], force: bool) -> Result<Vec<String>> {
    delete_at(&default_paths(), names, force)
}

fn delete_at(paths: &Paths, names: &[String], force: bool) -> Result<Vec<String>> {
    let mut registry = load_registry_at(paths)?;
    let skills = unique_skills(&registry, names)?;
    let mut deleted = Vec::new();

    for skill_name in skills {
        if let Some(record) = registry.skills.get(&skill_name) {
            if !record.added.is_empty() && !force {
                let places: Vec<String> = record
                    .added
                    .iter()
                    .map(|add| format!("{} ({})", add.harness, add.scope))
                    .collect();
                bail!(
                    "'{}' is still added to {}; remove it first or use --force",
                    skill_name,
                    places.join(", ")
                );
            }
            if force {
                for add in &record.added {
                    let path = PathBuf::from(&add.path);
                    if path.exists() {
                        harness::remove_dir(&path)?;
                    }
                }
            }
        }
        let dir = paths.library.join(&skill_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).with_context(|| format!("remove {}", dir.display()))?;
        }
        registry.skills.remove(&skill_name);
        deleted.push(skill_name);
    }

    let live: HashSet<String> = registry.skills.values().map(|r| r.bundle.clone()).collect();
    registry.bundles.retain(|key, _| live.contains(key));

    save_registry_at(paths, &registry)?;
    Ok(deleted)
}

pub fn list() -> Result<Vec<Entry>> {
    list_at(&default_paths())
}

fn list_at(paths: &Paths) -> Result<Vec<Entry>> {
    let registry = load_registry_at(paths)?;
    let mut out: Vec<Entry> = registry
        .skills
        .iter()
        .map(|(name, record)| Entry {
            name: name.clone(),
            description: record.description.clone(),
            bundle: record.bundle.clone(),
            added: record.added.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// One installed bundle as shown by `skill ls` (no `--all`).
#[derive(Debug, Clone)]
pub struct BundleSummary {
    pub name: String,
    pub skills: Vec<String>,
    pub added: Vec<AddedRecord>,
}

pub fn list_bundles() -> Result<Vec<BundleSummary>> {
    list_bundles_at(&default_paths())
}

fn list_bundles_at(paths: &Paths) -> Result<Vec<BundleSummary>> {
    let registry = load_registry_at(paths)?;
    let mut groups: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (name, record) in &registry.skills {
        groups
            .entry(record.bundle.clone())
            .or_default()
            .push(name.clone());
    }
    let mut out = Vec::new();
    for (name, mut skills) in groups {
        skills.sort();
        let mut added: Vec<AddedRecord> = Vec::new();
        for skill in &skills {
            let Some(record) = registry.skills.get(skill) else {
                continue;
            };
            for entry in &record.added {
                if !added.iter().any(|known| {
                    known.harness == entry.harness
                        && known.scope == entry.scope
                        && known.path == entry.path
                }) {
                    added.push(entry.clone());
                }
            }
        }
        out.push(BundleSummary {
            name,
            skills,
            added,
        });
    }
    Ok(out)
}

/// One skill folder found in a harness directory.
#[derive(Debug, Clone)]
pub struct HarnessEntry {
    pub name: String,
    pub path: PathBuf,
    pub scope: String,
    pub managed: bool,
    pub description: String,
}

pub fn info(harness: &str) -> Result<Vec<HarnessEntry>> {
    let harness = harness::canonical_harness(harness)?;
    let registry = load_registry_at(&default_paths())?;
    let mut out = Vec::new();
    for (global, scope) in [(false, "project"), (true, "global")] {
        let root = harness::target_root(harness, global)?;
        out.extend(scan_harness(&registry, &root, scope));
    }
    out.sort_by(|a, b| a.name.cmp(&b.name).then(a.scope.cmp(&b.scope)));
    out.dedup_by(|a, b| a.path == b.path);
    Ok(out)
}

/// Read a harness skills folder: every subfolder with a `SKILL.md` counts,
/// whether ntc put it there or the user did.
fn scan_harness(registry: &Registry, root: &Path, scope: &str) -> Vec<HarnessEntry> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let md = path.join("SKILL.md");
        if !md.is_file() {
            continue;
        }
        let front = fs::read_to_string(&md)
            .ok()
            .and_then(|text| frontmatter::parse_skill_md(&text).ok());
        let name = front
            .as_ref()
            .map(|fm| fm.name.clone())
            .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());
        let description = front.map(|fm| fm.description).unwrap_or_default();
        let managed = registry.skills.values().any(|record| {
            record
                .added
                .iter()
                .any(|add| same_path(Path::new(&add.path), &path))
        });
        out.push(HarnessEntry {
            name,
            path,
            scope: scope.to_string(),
            managed,
            description,
        });
    }
    out
}

/// Windows records the same folder as both `C:\...` and `\\?\C:\...`
/// depending on where the path came from, so compare resolved paths.
fn same_path(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Members of the bundle an installed skill belongs to, for `skill sub ls`.
pub fn subskills(name: &str) -> Result<(String, Vec<(String, String)>)> {
    subskills_at(&default_paths(), name)
}

fn subskills_at(paths: &Paths, name: &str) -> Result<(String, Vec<(String, String)>)> {
    let registry = load_registry_at(paths)?;
    let resolved = resolve_skills(&registry, name)?;
    let members: Vec<String> = if resolved.len() > 1 {
        resolved.clone()
    } else {
        registry
            .skills
            .get(&resolved[0])
            .and_then(|record| registry.bundles.get(&record.bundle))
            .map(|bundle| bundle.skills.clone())
            .filter(|skills| skills.len() > 1)
            .unwrap_or_else(|| resolved.clone())
    };
    let mut out = Vec::new();
    for skill in members {
        let description = registry
            .skills
            .get(&skill)
            .map(|record| record.description.clone())
            .unwrap_or_default();
        out.push((skill, description));
    }
    Ok((name.trim().to_string(), out))
}

/// Resolve exactly one skill and return its `SKILL.md`, for `skill edit`.
pub fn edit_target(name: &str) -> Result<(String, PathBuf)> {
    let paths = default_paths();
    let registry = load_registry_at(&paths)?;
    let skills = resolve_skills(&registry, name)?;
    if skills.len() > 1 {
        bail!(
            "'{}' is a bundle ({} skills); edit one of them by name",
            name,
            skills.len()
        );
    }
    let skill = skills
        .into_iter()
        .next()
        .unwrap_or_else(|| name.to_string());
    let path = paths.library.join(&skill).join("SKILL.md");
    if !path.is_file() {
        bail!("SKILL.md for '{}' is missing; download it again", skill);
    }
    Ok((skill, path))
}

/// Refresh the registry entry after the editor saved the file. Invalid
/// frontmatter is reported, but the user's saved file is left untouched.
pub fn after_edit(name: &str) -> Result<String> {
    let paths = default_paths();
    let md = paths.library.join(name).join("SKILL.md");
    let text = fs::read_to_string(&md).with_context(|| format!("read {}", md.display()))?;
    let fm = frontmatter::validate_in_dir(&text, name)?;
    let mut registry = load_registry_at(&paths)?;
    if let Some(record) = registry.skills.get_mut(name) {
        record.description = fm.description.clone();
        record.sha256 = harness::sha256_file(&md)?;
        save_registry_at(&paths, &registry)?;
    }
    Ok(fm.description)
}

/// Installed skill and bundle names, newest registry first is meaningless:
/// sorted for shell completion.
pub fn installed_names() -> Vec<String> {
    let Ok(registry) = load_registry() else {
        return Vec::new();
    };
    let mut names: Vec<String> = registry
        .skills
        .keys()
        .chain(registry.bundles.keys())
        .cloned()
        .collect();
    names.sort();
    names.dedup();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_library(paths: &Paths, name: &str, body: &str) {
        let dir = paths.library.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: {body}\n---\n"),
        )
        .unwrap();
        let mut registry = load_registry_at(paths).unwrap();
        registry.skills.insert(
            name.to_string(),
            SkillRecord {
                bundle: "demo".into(),
                source: "owner/repo".into(),
                path: format!("skills/{name}"),
                description: body.into(),
                sha256: harness::sha256_file(&dir.join("SKILL.md")).unwrap(),
                installed_at: "now".into(),
                added: Vec::new(),
            },
        );
        registry.bundles.insert(
            "demo".into(),
            BundleRecord {
                source: "owner/repo".into(),
                git_ref: "main".into(),
                commit: None,
                installed_at: "now".into(),
                skills: vec![name.to_string()],
            },
        );
        save_registry_at(paths, &registry).unwrap();
    }

    fn setup() -> (tempfile::TempDir, Paths, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let paths = Paths::new(dir.path().join("skills"));
        fs::create_dir_all(&paths.library).unwrap();
        let project = dir.path().join("project");
        (dir, paths, project)
    }

    #[test]
    fn add_copies_and_records_then_list_shows_it() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");

        let copied = add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();
        let target = project.join(".opencode/skills/demo");
        assert_eq!(copied.len(), 1);
        assert_eq!(copied[0].1, target);
        assert!(target.join("SKILL.md").is_file());

        let registry = load_registry_at(&paths).unwrap();
        let added = &registry.skills["demo"].added;
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].harness, "opencode");
        assert_eq!(added[0].scope, "project");

        // Re-adding refreshes the copy without duplicating the record.
        add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();
        let registry = load_registry_at(&paths).unwrap();
        assert_eq!(registry.skills["demo"].added.len(), 1);

        let listed = list_at(&paths).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "demo");
        assert_eq!(listed[0].added.len(), 1);
    }

    #[test]
    fn bundle_names_expand_to_their_skills() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        fs::create_dir_all(paths.library.join("second")).unwrap();
        fs::write(
            paths.library.join("second/SKILL.md"),
            "---\nname: second\ndescription: Second\n---\n",
        )
        .unwrap();
        let mut registry = load_registry_at(&paths).unwrap();
        registry.skills.insert(
            "second".into(),
            SkillRecord {
                bundle: "demo".into(),
                source: "owner/repo".into(),
                path: "skills/second".into(),
                description: "Second".into(),
                sha256: harness::sha256_file(&paths.library.join("second/SKILL.md")).unwrap(),
                installed_at: "now".into(),
                added: Vec::new(),
            },
        );
        registry.bundles.insert(
            "collection".into(),
            BundleRecord {
                source: "owner/repo".into(),
                git_ref: "main".into(),
                commit: None,
                installed_at: "now".into(),
                skills: vec!["demo".into(), "second".into()],
            },
        );
        registry.skills.get_mut("second").unwrap().bundle = "collection".into();
        save_registry_at(&paths, &registry).unwrap();

        let copied = add_to(
            &paths,
            &["collection".to_string()],
            "claude",
            false,
            &project.join(".claude/skills"),
            false,
        )
        .unwrap();
        assert_eq!(copied.len(), 2);
        assert!(project.join(".claude/skills/second/SKILL.md").is_file());
    }

    #[test]
    fn add_refuses_unmanaged_existing_folder_without_force() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        let target = project.join(".opencode/skills/demo");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("mine.txt"), "hand made").unwrap();

        let err = add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("already exists"), "{err}");
        assert!(target.join("mine.txt").is_file());

        add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            true,
        )
        .unwrap();
        assert!(!target.join("mine.txt").exists());
        assert!(target.join("SKILL.md").is_file());
    }

    #[test]
    fn remove_deletes_copy_and_keeps_library() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        add_to(
            &paths,
            &["demo".to_string()],
            "codex",
            false,
            &project.join(".codex/skills"),
            false,
        )
        .unwrap();
        let target = project.join(".codex/skills/demo");
        assert!(target.is_dir());

        let removed = remove_from(
            &paths,
            &["demo".to_string()],
            "codex",
            false,
            &project.join(".codex/skills"),
            false,
        )
        .unwrap();
        assert_eq!(removed.len(), 1);
        assert!(!target.exists());
        assert!(paths.library.join("demo/SKILL.md").is_file());
    }

    #[test]
    fn remove_refuses_hand_edited_copy_without_force() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        add_to(
            &paths,
            &["demo".to_string()],
            "agents",
            false,
            &project.join(".agents/skills"),
            false,
        )
        .unwrap();
        let target = project.join(".agents/skills/demo");
        fs::write(
            target.join("SKILL.md"),
            "---\nname: demo\ndescription: edited\n---\n",
        )
        .unwrap();

        let err = remove_from(
            &paths,
            &["demo".to_string()],
            "agents",
            false,
            &project.join(".agents/skills"),
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("edited"), "{err}");
        assert!(target.exists());

        remove_from(
            &paths,
            &["demo".to_string()],
            "agents",
            false,
            &project.join(".opencode/skills"),
            true,
        )
        .unwrap();
        assert!(!target.exists());
    }

    #[test]
    fn remove_missing_copy_drops_the_stale_record() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();
        fs::remove_dir_all(project.join(".opencode/skills/demo")).unwrap();

        remove_from(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();
        let registry = load_registry_at(&paths).unwrap();
        assert!(registry.skills["demo"].added.is_empty());
    }

    #[test]
    fn delete_refuses_while_added_unless_forced() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        add_to(
            &paths,
            &["demo".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();

        let err = delete_at(&paths, &["demo".to_string()], false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("still added"), "{err}");
        assert!(paths.library.join("demo").is_dir());

        delete_at(&paths, &["demo".to_string()], true).unwrap();
        assert!(!paths.library.join("demo").exists());
        assert!(!project.join(".opencode/skills/demo").exists());
        let registry = load_registry_at(&paths).unwrap();
        assert!(!registry.skills.contains_key("demo"));
        assert!(!registry.bundles.contains_key("demo"));
    }

    #[test]
    fn unknown_name_points_at_download() {
        let (_dir, paths, _project) = setup();
        let err = resolve_skills(&load_registry_at(&paths).unwrap(), "ghost")
            .unwrap_err()
            .to_string();
        assert!(err.contains("skill download ghost"), "{err}");
    }

    #[test]
    fn discovery_uses_frontmatter_names_and_skips_broken_skills() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("repo");
        for (folder, text) in [
            (
                "skills/frontend-design",
                "---\nname: frontend-design\ndescription: UI\n---\n",
            ),
            (
                "skills/template",
                "---\nname: template-skill\ndescription: Template\n---\n",
            ),
            ("skills/broken", "no frontmatter here"),
        ] {
            fs::create_dir_all(root.join(folder)).unwrap();
            fs::write(root.join(folder).join("SKILL.md"), text).unwrap();
        }
        let (found, skipped) = discover_skills(&root);
        let names: Vec<&str> = found.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["frontend-design", "template-skill"]);
        assert_eq!(skipped.len(), 1);
        assert!(skipped[0].0.ends_with("broken/SKILL.md"));
    }

    #[test]
    fn list_bundles_groups_members_and_dedupes_copies() {
        let (_dir, paths, project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        let second = paths.library.join("second");
        fs::create_dir_all(&second).unwrap();
        fs::write(
            second.join("SKILL.md"),
            "---\nname: second\ndescription: Second\n---\n",
        )
        .unwrap();
        let mut registry = load_registry_at(&paths).unwrap();
        registry.skills.insert(
            "second".into(),
            SkillRecord {
                bundle: "collection".into(),
                source: "owner/repo".into(),
                path: "skills/second".into(),
                description: "Second".into(),
                sha256: harness::sha256_file(&second.join("SKILL.md")).unwrap(),
                installed_at: "now".into(),
                added: Vec::new(),
            },
        );
        registry.skills.get_mut("demo").unwrap().bundle = "collection".into();
        registry.bundles.insert(
            "collection".into(),
            BundleRecord {
                source: "owner/repo".into(),
                git_ref: "main".into(),
                commit: None,
                installed_at: "now".into(),
                skills: vec!["demo".into(), "second".into()],
            },
        );
        save_registry_at(&paths, &registry).unwrap();
        add_to(
            &paths,
            &["collection".to_string()],
            "opencode",
            false,
            &project.join(".opencode/skills"),
            false,
        )
        .unwrap();

        let bundles = list_bundles_at(&paths).unwrap();
        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].name, "collection");
        assert_eq!(bundles[0].skills, ["demo", "second"]);
        // Two copies, still one bundle line.
        assert_eq!(bundles[0].added.len(), 2);
    }

    #[test]
    fn subskills_returns_bundle_members_or_the_skill_itself() {
        let (_dir, paths, _project) = setup();
        seed_library(&paths, "demo", "Demo skill");
        let second = paths.library.join("second");
        fs::create_dir_all(&second).unwrap();
        fs::write(
            second.join("SKILL.md"),
            "---\nname: second\ndescription: Second\n---\n",
        )
        .unwrap();
        let mut registry = load_registry_at(&paths).unwrap();
        registry.skills.insert(
            "second".into(),
            SkillRecord {
                bundle: "demo".into(),
                source: "owner/repo".into(),
                path: "skills/second".into(),
                description: "Second".into(),
                sha256: harness::sha256_file(&second.join("SKILL.md")).unwrap(),
                installed_at: "now".into(),
                added: Vec::new(),
            },
        );
        registry.bundles.get_mut("demo").unwrap().skills = vec!["demo".into(), "second".into()];
        save_registry_at(&paths, &registry).unwrap();

        let (label, members) = subskills_at(&paths, "demo").unwrap();
        assert_eq!(label, "demo");
        let names: Vec<&str> = members.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, ["demo", "second"]);

        // A single skill with no siblings is not a bundle.
        seed_library(&paths, "solo", "Solo skill");
        let (_, members) = subskills_at(&paths, "solo").unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].0, "solo");
    }

    #[test]
    fn harness_scan_reads_every_skill_folder() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("skills");
        for (folder, text) in [
            ("managed", "---\nname: managed\ndescription: Managed\n---\n"),
            (
                "handmade",
                "---\nname: handmade\ndescription: Handmade\n---\n",
            ),
        ] {
            fs::create_dir_all(root.join(folder)).unwrap();
            fs::write(root.join(folder).join("SKILL.md"), text).unwrap();
        }
        fs::create_dir_all(root.join("not-a-skill")).unwrap();

        let mut registry = Registry::default();
        registry.skills.insert(
            "managed".into(),
            SkillRecord {
                bundle: "demo".into(),
                source: "owner/repo".into(),
                path: "skills/managed".into(),
                description: "Managed".into(),
                sha256: "abc".into(),
                installed_at: "now".into(),
                added: vec![AddedRecord {
                    harness: "opencode".into(),
                    scope: "project".into(),
                    // A different spelling of the same folder still counts.
                    path: root.join("managed/.").to_string_lossy().to_string(),
                    added_at: "now".into(),
                }],
            },
        );

        let entries = scan_harness(&registry, &root, "project");
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["handmade", "managed"]);
        let managed = entries.iter().find(|e| e.name == "managed").unwrap();
        assert!(managed.managed);
        assert_eq!(managed.description, "Managed");
        assert!(
            !entries
                .iter()
                .find(|e| e.name == "handmade")
                .unwrap()
                .managed
        );
    }

    #[test]
    fn same_path_resolves_dot_segments() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("a/b")).unwrap();
        assert!(same_path(
            &dir.path().join("a/b"),
            &dir.path().join("a/./b")
        ));
        assert!(!same_path(&dir.path().join("a/b"), &dir.path().join("a")));
    }

    #[test]
    fn default_paths_live_under_the_ntc_config_dir() {
        let root = skills_root();
        assert!(root.ends_with("ntc/skills") || root.ends_with("ntc\\skills"));
    }
}
