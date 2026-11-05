//! Web path enumeration for `scan waep` (a.k.a. "which paths does this web
//! app expose?").
//!
//! The module is deliberately split so the interesting parts are testable
//! without a network: target parsing, wordlist parsing, URL joining and
//! status classification are pure, and [`enumerate`] takes a base URL plus a
//! word list and reports hits through a callback.
//!
//! Probing rules:
//! - redirects are **never** followed (`max_redirects(0)`): a `302` is a hit
//!   and its `Location` is reported, so `/download` and `/download/root` stay
//!   distinguishable and a scan cannot be bounced off-host by accident;
//! - non-2xx statuses are normal responses (`http_status_as_error(false)`), so
//!   `401`/`403`/`500` are reported instead of surfacing as errors;
//! - the response body is dropped after the headers, so a huge file costs one
//!   round trip, not a download.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Parallel probes when the user does not pass `-c`.
pub const DEFAULT_CONCURRENCY: usize = 16;
/// Per-probe timeout in milliseconds when `--timeout` is absent.
pub const DEFAULT_TIMEOUT_MS: u64 = 4000;
/// Status codes that count as a hit when `-s` is absent. `405` is included
/// because "this endpoint exists but not for GET" is still a discovered path.
pub const DEFAULT_STATUS: &[u16] = &[200, 204, 301, 302, 307, 308, 401, 403, 405];

/// One enumeration target: where to send the probes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub scheme: String,
    /// `host` or `host:port` exactly as the user typed it (already lowercased
    /// host, original port).
    pub authority: String,
    /// `""` or `/api` (leading slash, no trailing slash).
    pub base_path: String,
}

impl Target {
    /// Full URL for one word: `scheme://authority` + base + `/word`.
    pub fn url_for(&self, word: &str) -> String {
        let mut url = format!("{}://{}", self.scheme, self.authority);
        url.push_str(&join_path(&self.base_path, word));
        url
    }

    /// Base URL without the trailing slash (`http://host:8080/api`).
    pub fn base_url(&self) -> String {
        format!("{}://{}{}", self.scheme, self.authority, self.base_path)
    }

    /// Path shown for one word, relative to the scan base: scanning
    /// `http://host/api` reports `/files`, not `/api/files`, so the table
    /// reads as "what lives under this base".
    pub fn display_for(&self, word: &str) -> String {
        format!("/{}", word.trim_start_matches('/'))
    }
}

/// Join a base path and a word with exactly one `/` between them.
pub fn join_path(base: &str, word: &str) -> String {
    let base = base.trim_end_matches('/');
    let word = word.trim_start_matches('/');
    format!("{}/{}", base, word)
}

/// Parse `<host>`, `<host:port>`, `<url>` or `<url>/<base path>`.
pub fn parse_target(spec: &str) -> Result<Target, String> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err("target is empty".to_string());
    }
    if spec.chars().any(|c| c.is_whitespace()) {
        return Err(format!("target contains whitespace: {spec}"));
    }
    let (scheme, rest) = match spec.split_once("://") {
        Some((s, rest)) if !s.is_empty() => (s.to_lowercase(), rest),
        Some(_) => return Err(format!("invalid scheme: {spec}")),
        None => ("http".to_string(), spec),
    };
    if scheme != "http" && scheme != "https" {
        return Err(format!("unsupported scheme: {scheme}"));
    }
    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, ""),
    };
    if authority.is_empty() {
        return Err(format!("missing host: {spec}"));
    }
    // Rebuild the authority with a lowercased host but the original port.
    let authority = match authority.rsplit_once(':') {
        Some((host, port)) if !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()) => {
            // A port that cannot exist would make every probe fail with a
            // confusing transport error; reject it while parsing instead.
            match port.parse::<u32>() {
                Ok(n) if (1..=65535).contains(&n) => format!("{}:{}", host.to_lowercase(), port),
                _ => return Err(format!("invalid port: {port}")),
            }
        }
        _ => authority.to_lowercase(),
    };
    if let Some(host) = authority.split(':').next() {
        if host.is_empty()
            || !host
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '-' | '_' | '[' | ']'))
        {
            return Err(format!("invalid host: {authority}"));
        }
    }
    let base_path = path.trim_end_matches('/').to_string();
    Ok(Target {
        scheme,
        authority,
        base_path,
    })
}

/// Curated default word list: the paths a fresh ntc `serve`, a typical
/// framework or a forgotten admin panel tends to expose.
pub fn default_words() -> &'static [&'static str] {
    &[
        "api",
        "api/v1",
        "api/v2",
        "api/v3",
        "api/health",
        "api/files",
        "api/upload",
        "api/upload-chunk",
        "api/create-folder",
        "api/download-token",
        "api/download",
        "api/login",
        "api/user",
        "api/users",
        "api/status",
        "api/docs",
        "admin",
        "admin/login",
        "administrator",
        "login",
        "logout",
        "signin",
        "signup",
        "register",
        "account",
        "dashboard",
        "settings",
        "config",
        "upload",
        "upload-chunk",
        "create-folder",
        "download-token",
        "uploads",
        "download",
        "downloads",
        "files",
        "file",
        "media",
        "images",
        "img",
        "assets",
        "static",
        "public",
        "css",
        "js",
        "fonts",
        "docs",
        "help",
        "info",
        "about",
        "contact",
        "terms",
        "privacy",
        "health",
        "healthz",
        "ready",
        "status",
        "version",
        "metrics",
        "debug",
        "test",
        "tests",
        "dev",
        "staging",
        "backup",
        "backups",
        "db",
        "database",
        "sql",
        "logs",
        "log",
        "tmp",
        "temp",
        "cache",
        "console",
        "shell",
        "terminal",
        "actuator",
        "graphql",
        "swagger",
        "swagger-ui",
        "openapi.json",
        "api-docs",
        "robots.txt",
        "sitemap.xml",
        "favicon.ico",
        "crossdomain.xml",
        "security.txt",
        ".git/HEAD",
        ".git/config",
        ".env",
        ".env.local",
        ".htaccess",
        ".htpasswd",
        ".ds_store",
        "phpmyadmin",
        "pma",
        "adminer",
        "server-status",
        "server-info",
        "web.config",
        "package.json",
        "composer.json",
        "wp-admin",
        "wp-login.php",
        "cgi-bin",
        "private",
        "secret",
        "secrets",
        "keys",
        "token",
        "auth",
        "oauth",
        "callback",
        "webhook",
        "hooks",
        "events",
        "queue",
        "jobs",
        "cron",
        "monitor",
        "monitoring",
        "trace",
        "ping",
        "echo",
    ]
}

/// Re-base a word list on `base_path`: entries that repeat the base (for
/// example `api/files` while scanning `http://host/api`) are shortened to
/// their remainder, so they probe `/api/files` and not `/api/api/files`.
/// Other entries are kept as-relative-to-base, which is how a wordlist is
/// normally written.
pub fn adapt_words_to_base(words: &[String], base_path: &str) -> Vec<String> {
    let base = base_path.trim_matches('/').to_lowercase();
    if base.is_empty() {
        return words.to_vec();
    }
    let prefix = format!("{}/", base);
    let mut out: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for word in words {
        let lower = word.to_lowercase();
        let rel = if lower == base {
            // The base itself is what we are scanning; probing it again adds
            // nothing.
            continue;
        } else if lower.starts_with(&prefix) {
            word.get(base.len() + 1..).unwrap_or(word).to_string()
        } else {
            word.clone()
        };
        if rel.is_empty() {
            continue;
        }
        if seen.insert(rel.to_lowercase()) {
            out.push(rel);
        }
    }
    out
}

/// Normalise one wordlist file: `#` comments, blank lines, CRLF, inner spaces
/// and duplicates are dropped; entries are stored without a leading `/`.
pub fn parse_words(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // A wordlist line is one path; keep the first token so `path  # note`
        // still works.
        let word = line
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_start_matches('/');
        if word.is_empty() {
            continue;
        }
        if seen.insert(word.to_lowercase()) {
            out.push(word.to_string());
        }
    }
    out
}

/// Expand `-x php,html` suffixes: `admin` becomes `admin.php`, `admin.html`.
/// Words that already carry one of the suffixes are left alone.
pub fn expand_suffixes(words: &[String], exts: &[String]) -> Vec<String> {
    if exts.is_empty() {
        return words.to_vec();
    }
    let mut out: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let push = |out: &mut Vec<String>, seen: &mut std::collections::HashSet<String>, w: String| {
        if seen.insert(w.to_lowercase()) {
            out.push(w);
        }
    };
    for word in words {
        push(&mut out, &mut seen, word.clone());
        let has_suffix = exts.iter().any(|ext| {
            let ext = ext.trim_start_matches('.');
            word.to_lowercase()
                .ends_with(&format!(".{}", ext.to_lowercase()))
        });
        if has_suffix {
            continue;
        }
        for ext in exts {
            let ext = ext.trim_start_matches('.');
            if ext.is_empty() {
                continue;
            }
            push(&mut out, &mut seen, format!("{}.{}", word, ext));
        }
    }
    out
}

/// What one probe found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Probe {
    pub status: u16,
    /// Milliseconds, rounded.
    pub ms: u64,
    pub content_type: String,
    pub location: String,
}

impl Probe {
    /// Presentation category for colouring and the `Kind` column.
    pub fn kind(&self) -> Kind {
        match self.status {
            200..=299 => Kind::Ok,
            300..=399 => Kind::Redirect,
            401 | 403 => Kind::Forbidden,
            405 => Kind::Method,
            500..=599 => Kind::Error,
            _ => Kind::Missing,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ok,
    Redirect,
    Forbidden,
    /// The path exists but refuses this method (`405`).
    Method,
    Error,
    Missing,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Ok => "ok",
            Kind::Redirect => "->",
            Kind::Forbidden => "auth",
            Kind::Method => "mth",
            Kind::Error => "err",
            Kind::Missing => "miss",
        }
    }
}

/// Probe options shared by the whole run.
#[derive(Debug, Clone)]
pub struct Options {
    /// `GET` or `HEAD`.
    pub method: String,
    pub timeout_ms: u64,
    pub concurrency: usize,
    /// Extra `Key: Value` headers.
    pub headers: Vec<(String, String)>,
    pub user_agent: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            timeout_ms: DEFAULT_TIMEOUT_MS,
            concurrency: DEFAULT_CONCURRENCY,
            headers: Vec::new(),
            user_agent: "ntc-scan".to_string(),
        }
    }
}

/// Build the agent used by the whole run (one connection pool).
pub fn agent(opts: &Options) -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_millis(opts.timeout_ms)))
        .http_status_as_error(false)
        .max_redirects(0)
        .build()
        .new_agent()
}

/// One HTTP probe. Never panics: a transport error becomes `None`.
pub fn probe(agent: &ureq::Agent, url: &str, opts: &Options) -> Option<Probe> {
    let started = Instant::now();
    let mut req = if opts.method.eq_ignore_ascii_case("HEAD") {
        agent.head(url)
    } else {
        agent.get(url)
    };
    req = req.header("User-Agent", &opts.user_agent);
    for (k, v) in &opts.headers {
        req = req.header(k, v);
    }
    let resp = req.call().ok()?;
    let ms = started.elapsed().as_millis() as u64;
    let header = |name: &str| {
        resp.headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
    };
    // The body is intentionally not read: status + headers are the answer and
    // dropping the response closes the connection.
    Some(Probe {
        status: resp.status().as_u16(),
        ms,
        content_type: header("content-type"),
        location: header("location"),
    })
}

/// One hit: the URL that answered, the path shown to the user and the probe.
#[derive(Debug, Clone)]
pub struct Hit {
    pub path: String,
    pub url: String,
    pub probe: Probe,
}

/// Aggregate result of a run.
#[derive(Debug, Clone, Default)]
pub struct Summary {
    pub probed: usize,
    pub hits: usize,
    /// Probes that produced no HTTP response (DNS, connect, TLS, timeout).
    pub errors: usize,
    pub elapsed_ms: u64,
}

/// Run the enumeration: `words` are probed under `target`, and every response
/// whose status is accepted is reported through `on_hit` as it arrives.
pub fn enumerate(
    target: &Target,
    words: &[String],
    opts: &Options,
    accept: &(dyn Fn(u16) -> bool + Sync),
    on_hit: &mut dyn FnMut(&Hit),
) -> Summary {
    let started = Instant::now();
    let agent = agent(opts);
    let next = AtomicUsize::new(0);
    let failures = AtomicUsize::new(0);
    let (tx, rx) = mpsc::channel::<(usize, Hit)>();
    let workers = opts.concurrency.max(1).min(words.len().max(1));

    let mut reported = 0usize;
    std::thread::scope(|scope| {
        for _ in 0..workers {
            let tx = tx.clone();
            let agent = agent.clone();
            let next = &next;
            let failures = &failures;
            scope.spawn(move || loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= words.len() {
                    break;
                }
                let word = &words[i];
                let url = target.url_for(word);
                match probe(&agent, &url, opts) {
                    Some(probe) => {
                        if accept(probe.status) {
                            let hit = Hit {
                                path: target.display_for(word),
                                url,
                                probe,
                            };
                            if tx.send((i, hit)).is_err() {
                                break;
                            }
                        }
                    }
                    // No HTTP answer at all: count it so the caller can tell
                    // "nothing matched" from "the host was unreachable".
                    None => {
                        failures.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
        drop(tx);
        // Out-of-order arrival: collect everything, then report in wordlist
        // order so repeated scans print the same table.
        let mut collected: Vec<(usize, Hit)> = rx.into_iter().collect();
        collected.sort_by_key(|(i, _)| *i);
        reported = collected.len();
        for (_, hit) in &collected {
            on_hit(hit);
        }
    });

    Summary {
        probed: words.len(),
        hits: reported,
        errors: failures.load(Ordering::Relaxed),
        elapsed_ms: started.elapsed().as_millis() as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    #[test]
    fn target_parsing_covers_hosts_ports_and_base_paths() {
        let t = parse_target("example.com").unwrap();
        assert_eq!(
            t,
            Target {
                scheme: "http".into(),
                authority: "example.com".into(),
                base_path: String::new()
            }
        );
        assert_eq!(t.base_url(), "http://example.com");
        assert_eq!(t.url_for("api"), "http://example.com/api");

        let t = parse_target("example.com:3000").unwrap();
        assert_eq!(t.authority, "example.com:3000");
        // A word that already ends in `/` keeps it: directory probes rely
        // on the trailing slash reaching the server.
        assert_eq!(t.url_for("api/"), "http://example.com:3000/api/");

        let t = parse_target("https://Example.COM").unwrap();
        assert_eq!(t.scheme, "https");
        assert_eq!(t.authority, "example.com");
        assert_eq!(t.url_for("v1"), "https://example.com/v1");

        let t = parse_target("example.com/api").unwrap();
        assert_eq!(t.base_path, "/api");
        assert_eq!(t.url_for("v1"), "http://example.com/api/v1");
        // Results are shown relative to the base the user scanned.
        assert_eq!(t.display_for("v1"), "/v1");
        assert_eq!(t.url_for("/v1"), "http://example.com/api/v1");

        let t = parse_target("192.168.1.5:8080/admin/").unwrap();
        assert_eq!(t.authority, "192.168.1.5:8080");
        assert_eq!(t.base_path, "/admin");

        assert!(parse_target("").is_err());
        assert!(parse_target("ftp://host").is_err());
        assert!(parse_target("http://").is_err());
        // Bad ports and whitespace are rejected up front, not per probe.
        assert!(parse_target("example.com:99999").is_err());
        assert!(parse_target("example.com:0").is_err());
        assert!(parse_target("exa mple.com").is_err());
    }

    #[test]
    fn base_path_words_are_shortened_and_displayed_relative() {
        let words: Vec<String> = ["api", "api/files", "files", "api/upload", "admin"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            adapt_words_to_base(&words, "/api"),
            vec![
                "files".to_string(),
                "upload".to_string(),
                "admin".to_string()
            ]
        );
        // No base path: unchanged.
        assert_eq!(adapt_words_to_base(&words, ""), words);
        // A base that is not a prefix of the words leaves them alone.
        assert_eq!(
            adapt_words_to_base(&words, "/v2"),
            vec![
                "api".to_string(),
                "api/files".to_string(),
                "files".to_string(),
                "api/upload".to_string(),
                "admin".to_string()
            ]
        );
    }

    #[test]
    fn wordlist_parsing_handles_comments_crlf_and_dupes() {
        let text = "# comment\n/api\n\n  /admin  \r\napi\n/deduped # trailing note\n";
        assert_eq!(
            parse_words(text),
            vec![
                "api".to_string(),
                "admin".to_string(),
                "deduped".to_string()
            ]
        );
        assert!(parse_words("# only comments\n\n").is_empty());
    }

    #[test]
    fn suffix_expansion_adds_extensions_once() {
        let words = vec!["admin".to_string(), "index.php".to_string()];
        let exts = vec!["php".to_string(), ".html".to_string()];
        // `index.php` already carries a suffix, so it is not expanded again;
        // `admin` gets one variant per extension.
        assert_eq!(
            expand_suffixes(&words, &exts),
            vec![
                "admin".to_string(),
                "admin.php".to_string(),
                "admin.html".to_string(),
                "index.php".to_string(),
            ]
        );
        assert_eq!(expand_suffixes(&words, &[]), words);
        // A word already carrying the extension is not expanded again.
        assert_eq!(
            expand_suffixes(&["a.php".to_string()], &["php".to_string()]),
            vec!["a.php".to_string()]
        );
    }

    #[test]
    fn kind_classification() {
        let p = |status| Probe {
            status,
            ms: 1,
            content_type: String::new(),
            location: String::new(),
        };
        assert_eq!(p(200).kind(), Kind::Ok);
        assert_eq!(p(204).kind(), Kind::Ok);
        assert_eq!(p(301).kind(), Kind::Redirect);
        assert_eq!(p(302).kind(), Kind::Redirect);
        assert_eq!(p(401).kind(), Kind::Forbidden);
        assert_eq!(p(403).kind(), Kind::Forbidden);
        assert_eq!(p(405).kind(), Kind::Method);
        assert_eq!(p(404).kind(), Kind::Missing);
        assert_eq!(p(500).kind(), Kind::Error);
    }

    /// Minimal local server: `/api` 200, `/admin` 403, `/old` 302, else 404.
    fn fixture_server() -> (String, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            for stream in listener.incoming().take(16) {
                let Ok(mut stream) = stream else { continue };
                let mut buf = [0u8; 1024];
                let Ok(n) = stream.read(&mut buf) else {
                    continue;
                };
                let req = String::from_utf8_lossy(&buf[..n]).to_string();
                let path = req
                    .lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1))
                    .unwrap_or("/")
                    .to_string();
                let (status, extra) = match path.as_str() {
                    "/api" | "/api/v1" => ("200 OK", "Content-Type: application/json\r\n"),
                    "/admin" => ("403 Forbidden", "Content-Type: text/html\r\n"),
                    "/old" => ("302 Found", "Location: /new\r\n"),
                    _ => ("404 Not Found", "Content-Type: text/plain\r\n"),
                };
                let body = "";
                let resp = format!(
                    "HTTP/1.1 {status}\r\n{extra}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        (format!("127.0.0.1:{}", addr.port()), handle)
    }

    #[test]
    fn enumerate_reports_only_accepted_statuses_in_listing_order() {
        let (authority, server) = fixture_server();
        let target = parse_target(&authority).unwrap();
        let words: Vec<String> = ["api", "admin", "old", "missing"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let opts = Options {
            method: "GET".to_string(),
            timeout_ms: 2000,
            concurrency: 2,
            ..Options::default()
        };
        let mut seen: Vec<(String, u16)> = Vec::new();
        let summary = enumerate(
            &target,
            &words,
            &opts,
            &|status| DEFAULT_STATUS.contains(&status),
            &mut |hit| seen.push((hit.path.clone(), hit.probe.status)),
        );
        assert_eq!(summary.probed, 4);
        assert_eq!(
            seen,
            vec![
                ("/api".to_string(), 200),
                ("/admin".to_string(), 403),
                ("/old".to_string(), 302),
            ],
            "hits must be printed in wordlist order"
        );
        // Redirects are reported, never followed.
        assert_eq!(seen[2].0, "/old");
        drop(server);
    }

    #[test]
    fn enumerate_honours_a_base_path() {
        let (authority, server) = fixture_server();
        let target = parse_target(&format!("{}/api", authority)).unwrap();
        let words = vec!["v1".to_string(), "nope".to_string()];
        let opts = Options::default();
        let mut seen: Vec<String> = Vec::new();
        let summary = enumerate(
            &target,
            &words,
            &opts,
            &|status| status == 200,
            &mut |hit| seen.push(hit.path.clone()),
        );
        assert_eq!(summary.probed, 2);
        assert_eq!(seen, vec!["/v1".to_string()]);
        drop(server);
    }

    #[test]
    fn default_wordlist_is_clean_and_unique() {
        let words = parse_words(&default_words().join("\n"));
        assert!(words.len() >= 100, "{} words", words.len());
        // Every default entry must survive normalisation unchanged.
        assert!(words.iter().any(|w| w == "api/upload"));
        assert!(words.iter().any(|w| w == ".env"));
    }
}
