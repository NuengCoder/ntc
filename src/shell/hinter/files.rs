//! Filesystem/path/format completion: cwd listings, format files, extension
//! slots, the fs/view/scan path engine and star-glob expansion.

use crate::config::Config;

/// All plain file names in CWD, optionally filtered.
pub(crate) fn cwd_files(matching: impl Fn(&str) -> bool) -> Vec<String> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
            if !is_file {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                if matching(name) {
                    out.push(name.to_string());
                }
            }
        }
    }
    out.sort();
    out
}

/// All subdirectory names in CWD, sorted.
pub(crate) fn cwd_dirs() -> Vec<String> {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if !is_dir {
                continue;
            }
            if let Some(name) = entry.file_name().to_str() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

/// Stems of CWD files matching `exts` (empty = all files), sorted/deduped.
/// Shared stem rule: see `crate::shell::alias::file_stem_matching`.
pub(crate) fn cwd_file_stems(exts: &[String]) -> Vec<String> {
    let names = cwd_files(|n| {
        exts.is_empty() || {
            let lower = n.to_lowercase();
            exts.iter()
                .any(|e| lower == *e || lower.ends_with(&format!(".{}", e)))
        }
    });
    let mut out: Vec<String> = names
        .iter()
        .filter_map(|n| crate::shell::alias::file_stem_matching(n, exts))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// Format file suffixes each command reads, keyed by command name.
pub(crate) const FORMAT_FILE_EXTS: &[(&str, &[&str])] = &[
    ("ral", &[".ntc.ral"]),
    ("math", &[".ntc.math"]),
    ("igcare", &[".ntc.igcare"]),
    ("theme", &[".ntc_theme"]),
    ("fake", &[".ntc_fake"]),
    ("scr", &[".ntc"]),
    ("script", &[".ntc"]),
    (
        "ran",
        &[
            "NTCRANFILE.toml",
            "NTCRANFILE.json",
            "NTCRANFILE.yaml",
            "NTCRANFILE.yml",
        ],
    ),
];

/// Format files in the working directory whose name starts with `frag`.
pub(crate) fn format_files(exts: &[&str], frag: &str) -> Vec<String> {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return vec![],
    };
    let lower = frag.to_lowercase();
    let mut out: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            }; // Extensions are matched case-insensitively (`NTCRANFILE.toml`
               // is spelled uppercase in the table above).
            let lower_name = name.to_lowercase();
            if !exts
                .iter()
                .any(|ext| lower_name.ends_with(&ext.to_lowercase()))
            {
                continue;
            }
            if name.to_lowercase().starts_with(&lower) && name.len() > frag.len() {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

/// Format files for the slots that take one: `math <TAB>`, `scr <TAB>`,
/// `ral import <TAB>`, `igcare import <TAB>`, `theme import <TAB>`,
/// `fake group import <TAB>` and `ran check <TAB>`.
pub(crate) fn format_file_candidates(parent_parts: &[&str], frag: &str) -> Vec<String> {
    if parent_parts.is_empty() {
        return vec![];
    }
    let cmd = parent_parts[0].to_lowercase();
    let takes_file = match (cmd.as_str(), parent_parts.len()) {
        ("math" | "scr" | "script", 1) => true,
        ("ral" | "igcare" | "theme", 2) => parent_parts[1].eq_ignore_ascii_case("import"),
        ("fake", 3) => {
            parent_parts[1].eq_ignore_ascii_case("group")
                && parent_parts[2].eq_ignore_ascii_case("import")
        }
        ("ran", 2) => parent_parts[1].eq_ignore_ascii_case("check"),
        _ => false,
    };
    if !takes_file {
        return vec![];
    }
    let exts = FORMAT_FILE_EXTS
        .iter()
        .find(|(name, _)| *name == cmd)
        .map(|(_, exts)| *exts)
        .unwrap_or(&[]);
    format_files(exts, frag)
}

/// `<cmd> check <TAB>` takes the format file and nothing else. Returns `None`
/// when the cursor is not in a check slot.
pub(crate) fn check_only_candidates(parent_parts: &[&str], frag: &str) -> Option<Vec<String>> {
    let last = parent_parts.last()?.to_lowercase();
    if last != "check" {
        return None;
    }
    let cmd = parent_parts[0].to_lowercase();
    let exts = FORMAT_FILE_EXTS
        .iter()
        .find(|(name, _)| *name == cmd)
        .map(|(_, exts)| *exts)?;
    Some(format_files(exts, frag))
}

/// Archive extensions completed for `arc list` / `arc extract`.
pub(crate) const ARCHIVE_EXTS: &[&str] = &[".zip", ".tar.gz", ".tgz", ".tar"];

/// Sheet extensions completed for `nsheet open|view <file>` (bare `nsheet <file>` too).
pub(crate) const SHEET_EXTS: &[&str] = &[".csv", ".tsv", ".json", ".jsonl", ".ndjson", ".txt"];

/// Extensions completed for `fmt <file>`'s source-file slot. `pdf` and
/// `xlsx` are output-only, so they are not offered as inputs.
pub(crate) const FMT_SOURCE_EXTS: &[&str] = &[
    ".json",
    ".yml",
    ".yaml",
    ".toml",
    ".md",
    ".markdown",
    ".html",
    ".htm",
    ".xml",
    ".docx",
    ".txt",
    ".csv",
    ".tsv",
];

/// Extension palette offered by the `-o`/`--output` filename slot.
pub(crate) const FMT_EXTS: &[&str] = &[
    ".json",
    ".yml",
    ".yaml",
    ".toml",
    ".md",
    ".markdown",
    ".html",
    ".htm",
    ".xml",
    ".docx",
    ".txt",
    ".pdf",
    ".csv",
    ".tsv",
    ".xlsx",
];

/// Image extensions completed for `image|img <file>` and
/// `image|img display|show <file>`. Directories always pass so
/// `dir/<Tab>` traversal keeps working (handled by the `ext_filter`
/// logic in `fs_candidates_for`); non-image files are suppressed.
pub(crate) const IMAGE_EXTS: &[&str] = &[
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".tiff", ".tif", ".webp",
];

/// Audio + video extensions completed for `nms play <file>`.
///
/// `.ts` is deliberately omitted (it clashes with TypeScript); `.m2ts` stays.
pub(crate) const MEDIA_EXTS: &[&str] = &[
    ".mp3", ".wav", ".wave", ".flac", ".m4a", ".aac", ".ogg", ".oga", ".opus", ".wma", ".aiff",
    ".aif", ".aifc", ".mka", ".weba", ".mp4a", ".mp4", ".m4v", ".mkv", ".webm", ".mov", ".avi",
    ".mpg", ".mpeg", ".wmv", ".flv", ".3gp", ".m2ts",
];

/// True when `view`'s optional target-path slot is already filled by a bare
/// token in `args`. Flag tokens, the values after value-taking flags
/// (`-d`/`--depth`, `-p`/`--path`, `--head`/`--tail`/`--sort`) and `-ext`
/// value runs don't count — mirroring `extract_target_spec` in
/// `helpers.rs` plus the ext pre-parse in `report.rs`. `-s`/`-fs`/`-ds`
/// consume the next token only when it parses as a size value.
pub(crate) fn view_path_taken(args: &[&str]) -> bool {
    const VALUE_FLAGS: &[&str] = &[
        "-d", "--depth", "-p", "--path", "--head", "--tail", "--betw", "--sort",
    ];
    const OPTIONAL_SIZE_FLAGS: &[&str] = &["-s", "--size", "-fs", "--filesize", "-ds", "--dirsize"];
    const MULTI_FLAGS: &[&str] = &["-ext", "--ext", "-e"];
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut i = 0;
    while i < args.len() {
        let lower = args[i].to_lowercase();
        if VALUE_FLAGS.contains(&lower.as_str()) {
            i += 2; // skip the flag and its value
            continue;
        }
        if OPTIONAL_SIZE_FLAGS.contains(&lower.as_str())
            && i + 1 < args.len()
            && crate::explorer::parse_size_value(args[i + 1]).is_some()
        {
            i += 2; // `-s 10MB`: value consumed, not a path
            continue;
        }
        if MULTI_FLAGS.contains(&lower.as_str()) {
            i += 1;
            while i < args.len() && !crate::filetype::is_ext_value_stop(args[i], &cwd) {
                i += 1;
            }
            continue;
        }
        let tok = args[i];
        if tok.starts_with('-') || tok.is_empty() {
            i += 1;
            continue;
        }
        return true; // bare path token (including `@tp/sub`)
    }
    false
}

/// True when `qur` is currently in a file-glob slot (files should complete).
/// `parent_parts` excludes the fragment being typed. Globs come before
/// content flags: once `-s`/`-rp`/`-dp` appears, later slots are flags/values,
/// not more globs. A trailing value-taking flag means its value is expected.
pub(crate) fn qur_glob_slot(parent_parts: &[&str]) -> bool {
    const VALUE_FLAGS: &[&str] = &[
        "-s",
        "--select",
        "-rp",
        "--replace",
        "-d",
        "--depth",
        "-l",
        "--limit",
    ];
    const CONTENT_FLAGS: &[&str] = &["-s", "--select", "-rp", "--replace", "-dp", "--display"];
    if parent_parts.is_empty() {
        return false;
    }
    // Trailing value flag → its value is being typed, not a glob.
    if let Some(last) = parent_parts.last().map(|s| s.to_lowercase()) {
        if VALUE_FLAGS.contains(&last.as_str()) || last.starts_with("--limit=") {
            return false;
        }
    }
    // Any content flag already present (with or without value yet) →
    // glob phase is over; next slots are flags/values.
    for tok in &parent_parts[1..] {
        let lower = tok.to_lowercase();
        if CONTENT_FLAGS.contains(&lower.as_str()) {
            return false;
        }
        // Skip the value right after a value flag.
        let _ = lower;
    }
    // Skip values after value flags when scanning for stray flags:
    // `qur *.rs -d 3` still has no content flag → glob slot stays open
    // for extra globs (`qur *.rs *.md -d 3`), which parse_qur_args allows.
    let mut i = 1;
    while i < parent_parts.len() {
        let lower = parent_parts[i].to_lowercase();
        if VALUE_FLAGS.contains(&lower.as_str()) {
            i += 2;
            continue;
        }
        if lower.starts_with("--limit=") || lower == "-dp" || lower == "--display" {
            return false;
        }
        i += 1;
    }
    true
}

/// True when the next word after `parts` (fragment excluded) is an extension
/// value: the last parent token is an ext flag (`-ext`/`--ext`), an ext
/// subcommand (`scan`/`tl` + `ext`), or a bare ext already following one —
/// with no intervening flag or path.
pub(crate) fn in_ext_slot(parts: &[&str]) -> bool {
    if parts.len() < 2 {
        return false;
    }
    let root = parts[0].to_lowercase();
    let mut i = parts.len() - 1;
    loop {
        let l = parts[i].to_lowercase();
        if l == "-ext" || l == "--ext" || l == "-e" {
            return matches!(root.as_str(), "view" | "viewf");
        }
        if l == "ext" && i == 1 && (root == "scan" || root == "tl") {
            return true;
        }
        if l.starts_with('-') || l.starts_with('@') {
            return false;
        }
        if l.contains('/') || l.contains('\\') {
            return false;
        }
        if i <= 1 {
            return false;
        }
        i -= 1;
    }
}

/// Extension-name candidates for `scan ext`, `tl ext` and `view -ext`.
/// `frag` is the word being typed; comma-aware (`rs,c` completes after the
/// comma and returns full `rs,…` replacements). Empty frag → no candidates
/// (flags/subcommands already cover the bare slot).
pub(crate) fn ext_names_for_command(parts: &[&str], frag: &str) -> Vec<String> {
    if frag.is_empty() || !in_ext_slot(parts) {
        return vec![];
    }
    let (prefix, stub) = match frag.rfind(',') {
        Some(idx) => (&frag[..idx + 1], &frag[idx + 1..]),
        None => ("", frag),
    };
    let lower = stub.to_lowercase();
    crate::filetype::all_known_extensions()
        .into_iter()
        .filter(|e| e.starts_with(&lower) && e.len() > stub.len())
        .map(|e| format!("{}{}", prefix, e))
        .collect()
}

/// Value hints for `ignore|care dir|file <name>` and `ignore|care ext <ext>`.
///
/// Names are listed bare — never with a trailing `/`, since the config stores
/// directory and file names verbatim and a `src/` entry would never match —
/// and a comma inside one token completes its next element
/// (`ignore dir src,do<Tab>` → `src,docs`). `ext`
/// offers the extensions of the files in the working directory (the useful
/// palette when choosing what to ignore), lowercased like the stored value.
///
/// `ignore` only offers items that are not ignored yet; `care` only offers
/// items that already are (the effective global + local ignore sets), so the
/// two commands never hint a no-op. `None` when the line is not an
/// ignore/care value slot.
pub(crate) fn igcare_value_candidates_for(parts: &[&str], frag: &str) -> Option<Vec<String>> {
    if parts.len() < 2 || !matches!(parts[0].to_lowercase().as_str(), "ignore" | "care") {
        return None;
    }
    let ignoring = parts[0].eq_ignore_ascii_case("ignore");
    let kind = parts[1].to_lowercase();
    if !matches!(kind.as_str(), "dir" | "file" | "ext") {
        return None;
    }
    // Comma-separated lists: keep everything up to the last comma as prefix.
    let (prefix, stub) = match frag.rfind(',') {
        Some(idx) => (&frag[..idx + 1], frag[idx + 1..].trim_start()),
        None => ("", frag),
    };
    let lower = stub.to_lowercase();
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let names: Vec<String> = match kind.as_str() {
        "ext" => {
            let ignored = Config::get_ignored_extensions_combined();
            dir_extensions(&cwd, &lower)
                .into_iter()
                .filter(|e| ignored.contains(e) != ignoring)
                .collect()
        }
        "dir" => {
            let ignored = Config::get_ignored_dirs_combined();
            cwd_dirs()
                .into_iter()
                .filter(|d| ignored.contains(&d.to_lowercase()) != ignoring)
                .collect()
        }
        _ => {
            let ignored = Config::get_ignored_files_combined();
            cwd_files(|_| true)
                .into_iter()
                .filter(|f| ignored.contains(f) != ignoring)
                .collect()
        }
    };
    Some(
        names
            .into_iter()
            .filter(|n| {
                let l = n.to_lowercase();
                l.starts_with(&lower) && l.len() > stub.len()
            })
            .map(|n| format!("{}{}", prefix, n))
            .collect(),
    )
}

/// Output-filename candidates for `fmt … -o <name>` / `fmt … --output <name>`
/// (the word slot immediately after the flag). Returns full last-word
/// replacements that keep the typed basename and only grow the extension:
/// `o.j` → `o.json …`; a dotless stub `j` → `j.json …`; a bare `-o <Tab>`
/// suggests a default name, `out.json …`.
pub(crate) fn fmt_output_candidates_for(parts: &[&str], frag: &str) -> Vec<String> {
    let in_output_slot = parts.len() >= 2
        && parts[0].to_lowercase() == "fmt"
        && matches!(
            parts[parts.len() - 1].to_lowercase().as_str(),
            "-o" | "--output"
        );
    if !in_output_slot {
        return vec![];
    }
    // Everything up to the last dot stays as the basename; the stub after
    // it filters the extension palette.
    let (stem, partial) = match frag.rfind('.') {
        Some(idx) => (frag[..idx].to_string(), frag[idx + 1..].to_lowercase()),
        None => (
            if frag.is_empty() { "out" } else { frag }.to_string(),
            String::new(),
        ),
    };
    FMT_EXTS
        .iter()
        .map(|e| e.trim_start_matches('.'))
        .filter(|e| e.starts_with(&partial))
        .map(|e| format!("{}.{}", stem, e))
        .collect()
}

/// Filesystem candidates for commands that take paths.
/// Returns full last-word replacements (e.g. `src/main`, `docs/`).
pub(crate) fn fs_candidates_for(parent_parts: &[&str], last_frag: &str) -> Vec<String> {
    if parent_parts.is_empty() {
        return vec![];
    }
    let root = parent_parts[0].to_lowercase();

    // `serve ndb` takes flags only: the database is picked in the page, so a
    // positional path would just be rejected.
    if root == "serve" && parent_parts.len() >= 2 && parent_parts[1].eq_ignore_ascii_case("ndb") {
        return vec![];
    }

    // (dirs_only, files_only, extension filter, readable_only)
    // - dirs_only: directories only, files never pass (`cd`, `ds`).
    // - files_only: files only, directories never pass (`fs`, `mkf`).
    // - ext_filter: files must match one of these suffixes; directories
    //   always pass so `dir/<Tab>` traversal keeps working.
    // - readable_only: files must not be known binaries/archives
    //   (`txtc`/`txtf`); directories pass for traversal.
    let (dirs_only, files_only, ext_filter, readable_only): (bool, bool, Option<&[&str]>, bool) =
        match root.as_str() {
            "cd" => {
                // `cd to <name>` is a teleport jump — no filesystem
                if parent_parts.len() >= 2 && parent_parts[1].to_lowercase() == "to" {
                    return vec![];
                }
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `tl file <path>` / `tl dir <path>` take paths; `tl ext` takes an extension
            "tl" => {
                if in_ext_slot(parent_parts) {
                    // Extension slot — ext names complete, not files.
                    return vec![];
                }
                if parent_parts.len() != 2
                    || !matches!(parent_parts[1].to_lowercase().as_str(), "file" | "dir")
                {
                    return vec![];
                }
                (false, false, None, false)
            }
            // `mkd`/`rmd` take comma-separated dir lists (`mkd a, b`) —
            // every slot completes dirs.
            "mkd" | "rmd" => (true, false, None, false),
            "rnmd" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `ds <pattern>` searches directories — dirs only.
            "ds" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `view` takes flags plus one optional target path in any order
            // (`view -ds <path>`, `view <path> --dirsize`, …).
            "view" => {
                if in_ext_slot(parent_parts) {
                    // Extension slot — ext names complete, not files.
                    return vec![];
                }
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                let file = parent_parts[1..]
                    .iter()
                    .any(|t| matches!(t.to_lowercase().as_str(), "-f" | "--file"));
                let dir = parent_parts[1..]
                    .iter()
                    .any(|t| matches!(t.to_lowercase().as_str(), "-dir" | "--directory"));
                match (file, dir) {
                    (true, false) => (false, true, None, false),
                    (false, true) => (true, false, None, false),
                    _ => (false, false, None, false),
                }
            }
            // `viewf` takes a file or directory target (fixed depth 1).
            "viewf" => {
                if in_ext_slot(parent_parts) || view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                (false, false, None, false)
            }
            // `viewd` takes a directory target.
            "viewd" => {
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `views` takes one directory target after its flags.
            "views" => {
                if view_path_taken(&parent_parts[1..]) {
                    return vec![];
                }
                let awaiting = parent_parts
                    .last()
                    .map(|t| {
                        matches!(
                            t.to_lowercase().as_str(),
                            "--sort" | "-s" | "--size" | "--trim"
                        )
                    })
                    .unwrap_or(false)
                    && parent_parts.len() > 1;
                if awaiting {
                    return vec![];
                }
                (true, false, None, false)
            }
            // `txtc`/`txtf <file>` display readable files: suppress known
            // binaries/archives (`.exe`, `.zip`, …); dirs pass for traversal.
            "txtc" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `txtf [--head N] [--tail N] [--betw A:B] [file]`: flags in
            // any order; no file completion while a flag awaits its value
            // or once the file slot is filled.
            "txtf" => {
                if let Some(last) = parent_parts.last().map(|s| s.to_lowercase()) {
                    if last == "--head" || last == "--tail" || last == "--betw" {
                        return vec![];
                    }
                }
                let mut i = 1;
                let mut bare = 0;
                while i < parent_parts.len() {
                    let lower = parent_parts[i].to_lowercase();
                    if lower == "--head" || lower == "--tail" || lower == "--betw" {
                        i += 2; // skip the flag and its value
                        continue;
                    }
                    if !parent_parts[i].starts_with('-') {
                        bare += 1;
                    }
                    i += 1;
                }
                if bare > 0 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `head`/`tail`/`betw` take an optional count/range then an
            // optional file; offer files until that file slot is filled.
            "head" | "tail" | "betw" => {
                let mut bare = 0;
                for tok in parent_parts.iter().skip(1) {
                    if !tok.starts_with('-') {
                        bare += 1;
                    }
                }
                if bare >= 2 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // `sort`/`rev` take an optional single file; offer files until it
            // is given (flags are skipped).
            "sort" | "rev" => {
                let mut bare = 0;
                for tok in parent_parts.iter().skip(1) {
                    if !tok.starts_with('-') {
                        bare += 1;
                    }
                }
                if bare >= 1 {
                    return vec![];
                }
                (false, false, None, true)
            }
            // Pattern-search commands take a pattern, not a path, but
            // filename completion helps compose it — files only.
            "fs" | "fsc" | "fgo" | "locate" | "gs" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `qur <glob> [-s <pat>] [-rp <new>] [-dp] [-d <n>] [-l <n>]`:
            // files only in the glob slot — never for `-s`/`-rp` values
            // (content patterns) nor `-d`/`-l` values (numbers), and once a
            // content flag is present the next slot is flags, not more globs.
            "qur" => {
                if !qur_glob_slot(parent_parts) {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `mkf`/`rmf` take comma-separated file lists (`rmf a, b`) —
            // every slot completes files.
            "mkf" | "rmf" => (false, true, None, false),
            // `rnmf <old> <new>`: only the existing-file slot completes
            // (the new name must not exist).
            "rnmf" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, true, None, false)
            }
            // `arc create <out> <inputs…>` / `arc list|extract <archive> [dest]`
            "arc" => {
                if parent_parts.len() < 2 {
                    return vec![];
                }
                match parent_parts[1].to_lowercase().as_str() {
                    // Output slot is a free-form archive name — no hint.
                    // Deeper positions are input files/dirs.
                    "create" | "c" | "mk" => {
                        if parent_parts.len() == 2 {
                            return vec![];
                        }
                        (false, false, None, false)
                    }
                    "list" | "ls" | "l" => {
                        if parent_parts.len() == 2 {
                            (false, false, Some(ARCHIVE_EXTS), false)
                        } else {
                            return vec![];
                        }
                    }
                    "extract" | "x" | "unzip" => {
                        if parent_parts.len() == 2 {
                            (false, false, Some(ARCHIVE_EXTS), false)
                        } else if parent_parts.len() == 3 {
                            // destination directory
                            (true, false, None, false)
                        } else {
                            return vec![];
                        }
                    }
                    _ => return vec![],
                }
            }
            // `nssh cp <local|name:remote>`: complete local paths only when the
            // fragment is not a remote `name:…` spec.
            "nssh" => {
                if parent_parts.len() == 2
                    && matches!(parent_parts[1].to_lowercase().as_str(), "cp")
                    && !last_frag.contains(':')
                {
                    (false, false, None, false)
                } else {
                    return vec![];
                }
            }
            // `nsheet open|view <file>` and bare `nsheet <file>` take sheet files.
            // Later positions are flags/values — no file completion there.
            "nsheet" => {
                if parent_parts.len() == 1 {
                    (false, false, Some(SHEET_EXTS), false)
                } else if parent_parts.len() == 2
                    && matches!(parent_parts[1].to_lowercase().as_str(), "open" | "view")
                {
                    (false, false, Some(SHEET_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // `nms play <file>` completes audio/video media files.
            "nms" => {
                if parent_parts.len() == 2 && parent_parts[1].to_lowercase() == "play" {
                    (false, false, Some(MEDIA_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // `image <TAB>` and `image display|show <TAB>` complete
            // image files + dirs; deeper positions (flags/values)
            // complete nothing.
            "image" | "img"
                if parent_parts.len() == 1
                    || (parent_parts.len() == 2
                        && matches!(
                            parent_parts[1].to_lowercase().as_str(),
                            "display" | "show"
                        )) =>
            {
                (false, false, Some(IMAGE_EXTS), false)
            }
            "image" | "img" => {
                return vec![];
            }
            // `fmt <TAB>` completes the source-file slot with the
            // supported formats; deeper positions (keyword/flag slots)
            // are handled by `subcommands_for` / the `-o` slot helper.
            "fmt" => {
                if parent_parts.len() == 1 {
                    (false, false, Some(FMT_SOURCE_EXTS), false)
                } else {
                    return vec![];
                }
            }
            // General path-takers accept files and dirs. For `cp`/`mv` only
            // the src slot completes (dst is a free-form name/dir) and
            // `-f`/`--force` flags never consume it.
            "mv" | "move" | "cp" => {
                let operands = parent_parts[1..]
                    .iter()
                    .filter(|t| !t.starts_with('-'))
                    .count();
                if operands > 0 {
                    return vec![];
                }
                (false, false, None, false)
            }
            "size" | "report" | "explorer" | "nfe" | "ne" | "ntceditor" => {
                if parent_parts.len() > 1 {
                    return vec![];
                }
                (false, false, None, false)
            }
            _ => return vec![],
        };

    // Split `dir/prefix` so `cd src/ma` lists inside `src/`
    let (dir_part, file_part) = match last_frag.rfind('/') {
        Some(idx) => (&last_frag[..idx], &last_frag[idx + 1..]),
        None => ("", last_frag),
    };
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    let base = if dir_part.is_empty() {
        cwd
    } else {
        let p = cwd.join(dir_part);
        if !p.is_dir() {
            return vec![];
        }
        p
    };

    // `rmf *.<Tab>`: complete `*.<ext>` delete patterns from the file
    // extensions present in the listed directory (`*.log`, `sub/*.tmp`).
    // `cp`/`mv` pattern sources (`cp *.py <Tab>`) behave the same, and a
    // bare `*` also offers `*` itself so Tab cycles `*` → `*.py` → ….
    if matches!(root.as_str(), "rmf" | "mv" | "move" | "cp")
        && (file_part == "*" || file_part.starts_with("*."))
    {
        return star_pattern_candidates(&base, dir_part, file_part);
    }

    list_path_candidates(
        &base,
        dir_part,
        file_part,
        dirs_only,
        files_only,
        ext_filter,
        readable_only,
    )
}

/// Shared directory listing for command slots and `>>`/`<` redirect
/// targets. `base` must be an existing directory; names are filtered by
/// `file_part`, the per-command options, and (when `readable_only`) hidden
/// when they are binaries, images, audio/video or archives.
pub(crate) fn list_path_candidates(
    base: &std::path::Path,
    dir_part: &str,
    file_part: &str,
    dirs_only: bool,
    files_only: bool,
    ext_filter: Option<&[&str]>,
    readable_only: bool,
) -> Vec<String> {
    let lower = file_part.to_lowercase();
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if name.starts_with('.') && !lower.starts_with('.') {
                continue;
            }
            if !name.to_lowercase().starts_with(&lower) {
                continue;
            }
            if name.len() <= file_part.len() && !file_part.is_empty() {
                continue;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if dirs_only && !is_dir {
                continue;
            }
            if files_only && is_dir {
                continue;
            }
            if let Some(exts) = ext_filter {
                if !is_dir && !exts.iter().any(|e| name.to_lowercase().ends_with(e)) {
                    continue;
                }
            }
            if readable_only && !is_dir && crate::filetype::is_finder_hidden(&entry.path()) {
                continue;
            }
            let display = if dir_part.is_empty() {
                name
            } else {
                format!("{}/{}", dir_part, name)
            };
            if is_dir {
                out.push(format!("{}/", display));
            } else {
                out.push(display);
            }
        }
    }
    out
}

/// `*`-pattern candidates for `rmf`/`cp`/`mv` from the extensions of the
/// files inside `base`. `frag` is the typed last word (`*`, `*.`, `*.py`).
/// Returns full last-word replacements. A bare `*` also offers `*` itself so
/// Tab cycles `*` → `*.py` → …; exact-length matches are excluded so the
/// ghost only ever shows an untyped remainder.
pub(crate) fn star_pattern_candidates(
    base: &std::path::Path,
    dir_part: &str,
    frag: &str,
) -> Vec<String> {
    let render = |ext: &str| {
        if dir_part.is_empty() {
            format!("*.{}", ext)
        } else {
            format!("{}/*.{}", dir_part, ext)
        }
    };
    let mut out: Vec<String> = Vec::new();
    // Bare `*`: offer `*` plus every `*.<ext>` present.
    if frag == "*" {
        out.push(if dir_part.is_empty() {
            "*".to_string()
        } else {
            format!("{}/*", dir_part)
        });
        for ext in dir_extensions(base, "") {
            out.push(render(&ext));
        }
        return out;
    }
    // `*.` / `*.stub`: only matching extensions.
    let stub = frag.strip_prefix("*.").unwrap_or("").to_lowercase();
    for ext in dir_extensions(base, &stub) {
        out.push(render(&ext));
    }
    out
}

/// Sorted, deduped extensions of the non-hidden files directly inside
/// `base`, filtered by `stub` (empty = all). Extensions are lowercased and
/// exact-stub-length matches are dropped so a fully-typed pattern ghosts
/// nothing.
pub(crate) fn dir_extensions(base: &std::path::Path, stub: &str) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(true) {
                continue;
            }
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if name.starts_with('.') {
                continue;
            }
            let Some(dot) = name.rfind('.') else {
                continue;
            };
            let ext = name[dot + 1..].to_lowercase();
            if ext.is_empty() || !ext.starts_with(stub) {
                continue;
            }
            // Exact-length matches ghost nothing — skip them.
            if ext.len() <= stub.len() {
                continue;
            }
            if !exts.contains(&ext) {
                exts.push(ext);
            }
        }
    }
    exts.sort();
    exts
}
