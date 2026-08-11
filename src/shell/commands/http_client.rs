use crate::navigator::Navigator;
use crate::t;
use anyhow::{Context, Result};
use colored::*;
use std::time::{Duration, Instant};

#[derive(Default)]
struct RequestConfig {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    silent: bool,
    output: Option<String>,
    verbose: bool,
    json_body: Option<String>,
    auth: Option<String>,
    bearer: Option<String>,
    query: Vec<(String, String)>,
    timeout: u64,
    no_pretty: bool,
}

pub fn cmd_http(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let first = parts.first().copied().unwrap_or("");

    if first.is_empty() || first == "help" || first == "--help" || first == "-h" {
        print_help();
        return Ok(false);
    }

    let method;
    let rest;

    match first.to_uppercase().as_str() {
        "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" => {
            method = first.to_uppercase();
            rest = parts.get(1).unwrap_or(&"").trim();
        }
        _ => {
            method = "GET".to_string();
            rest = args.trim();
        }
    }

    let cfg = parse_request(&method, rest)?;
    execute_request(&cfg)?;
    Ok(false)
}

fn parse_request(method: &str, args: &str) -> Result<RequestConfig> {
    let tokens = tokenize(args);
    let mut cfg = RequestConfig {
        method: method.to_string(),
        timeout: 30,
        ..Default::default()
    };
    let mut i = 0;
    let mut url_found = false;

    while i < tokens.len() {
        let t = &tokens[i];
        match t.as_str() {
            "-H" | "--header" => {
                i += 1;
                let h = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_header")))?;
                if let Some((k, v)) = h.split_once(':') {
                    cfg.headers
                        .push((k.trim().to_string(), v.trim().to_string()));
                } else {
                    anyhow::bail!("{}", t!("cmd.http.invalid_header", h));
                }
            }
            "-d" | "--data" | "--body" => {
                i += 1;
                cfg.body = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_body_data")))?
                        .clone(),
                );
            }
            "-s" | "--silent" => cfg.silent = true,
            "-o" | "--output" => {
                i += 1;
                cfg.output = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_filename")))?
                        .clone(),
                );
            }
            "-v" | "--verbose" => cfg.verbose = true,
            "--json" => {
                i += 1;
                cfg.json_body = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_json")))?
                        .clone(),
                );
            }
            "-a" | "--auth" => {
                i += 1;
                cfg.auth = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_auth")))?
                        .clone(),
                );
            }
            "--bearer" => {
                i += 1;
                cfg.bearer = Some(
                    tokens
                        .get(i)
                        .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_bearer")))?
                        .clone(),
                );
            }
            "-q" | "--query" => {
                i += 1;
                let q = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_query")))?
                    .clone();
                if let Some((k, v)) = q.split_once('=') {
                    cfg.query.push((k.to_string(), v.to_string()));
                } else {
                    anyhow::bail!("{}", t!("cmd.http.invalid_query", q));
                }
            }
            "-t" | "--timeout" => {
                i += 1;
                let secs = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.http.missing_timeout")))?
                    .clone();
                cfg.timeout = secs
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.http.invalid_timeout", secs)))?;
            }
            "--no-pretty" => cfg.no_pretty = true,
            "--" => break,
            _ => {
                if !url_found {
                    cfg.url = tokens[i].clone();
                    url_found = true;
                }
            }
        }
        i += 1;
    }

    if cfg.url.is_empty() {
        anyhow::bail!("{}", t!("cmd.http.url_required"));
    }

    Ok(cfg)
}

fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for ch in input.chars() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            ' ' | '\t' if !in_single && !in_double => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn build_url(base: &str, query: &[(String, String)]) -> String {
    if query.is_empty() {
        return base.to_string();
    }
    let params: Vec<String> = query
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect();
    let sep = if base.contains('?') { "&" } else { "?" };
    format!("{}{}{}", base, sep, params.join("&"))
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

fn execute_request(cfg: &RequestConfig) -> Result<()> {
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(cfg.timeout)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let url = build_url(&cfg.url, &cfg.query);

    let mut req_builder = ureq::http::Request::builder()
        .method(cfg.method.as_str())
        .uri(&url);

    for (k, v) in &cfg.headers {
        req_builder = req_builder.header(k.as_str(), v.as_str());
    }

    if let Some(ref token) = cfg.bearer {
        req_builder = req_builder.header("Authorization", &format!("Bearer {}", token));
    }

    if let Some(ref up) = cfg.auth {
        let encoded =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, up.as_str());
        req_builder = req_builder.header("Authorization", &format!("Basic {}", encoded));
    }

    let body_to_send = if let Some(ref j) = cfg.json_body {
        if !cfg
            .headers
            .iter()
            .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        {
            req_builder = req_builder.header("Content-Type", "application/json");
        }
        j.clone()
    } else if let Some(ref b) = cfg.body {
        b.clone()
    } else {
        String::new()
    };

    if cfg.verbose {
        eprintln!(
            "{} {} {}",
            t!("cmd.http.verbose_prefix").blue().bold(),
            cfg.method.blue().bold(),
            url.blue()
        );
        for (k, v) in &cfg.headers {
            eprintln!(
                "{} {}: {}",
                t!("cmd.http.verbose_prefix").blue(),
                k.cyan(),
                v
            );
        }
        if cfg.bearer.is_some() {
            eprintln!(
                "{} {} {}",
                t!("cmd.http.verbose_prefix").blue(),
                t!("cmd.http.authorization_label"),
                t!("cmd.http.bearer_placeholder").dimmed()
            );
        }
        if cfg.auth.is_some() {
            eprintln!(
                "{} {} {}",
                t!("cmd.http.verbose_prefix").blue(),
                t!("cmd.http.authorization_label"),
                t!("cmd.http.basic_placeholder").dimmed()
            );
        }
        if !body_to_send.is_empty() {
            eprintln!("{}", t!("cmd.http.verbose_prefix").blue());
            if body_to_send.len() <= 500 {
                eprintln!(
                    "{} {}",
                    t!("cmd.http.verbose_prefix").blue(),
                    body_to_send.dimmed()
                );
            } else {
                eprintln!(
                    "{} {}...",
                    t!("cmd.http.verbose_prefix").blue(),
                    body_to_send[..500].dimmed()
                );
            }
        }
    }

    let start = Instant::now();
    let has_body = !body_to_send.is_empty();
    let response = if has_body {
        agent.run(
            req_builder
                .body(body_to_send)
                .map_err(|e| anyhow::anyhow!("{}", t!("cmd.http.invalid_body", e)))?,
        )
    } else {
        agent.run(
            req_builder
                .body(())
                .map_err(|e| anyhow::anyhow!("{}", t!("cmd.http.invalid_request", e)))?,
        )
    }
    .with_context(|| t!("cmd.http.request_failed", cfg.method.clone(), url.clone()).to_string())?;

    let elapsed = start.elapsed();

    let status = response.status();
    let unknown = t!("cmd.http.unknown_status");
    let status_text = status.canonical_reason().unwrap_or(&unknown);
    let headers: Vec<(String, String)> = response
        .headers()
        .iter()
        .map(|(n, v)| {
            (
                n.as_str().to_string(),
                v.to_str().unwrap_or("<binary>").to_string(),
            )
        })
        .collect();
    let body = response.into_body().read_to_string().unwrap_or_default();
    let byte_count = body.len();

    if !cfg.silent {
        let status_line = t!(
            "cmd.http.status_line",
            &cfg.method,
            &cfg.url,
            status.as_u16()
        );
        if status.is_success() {
            print!("{}", status_line.green().bold());
        } else if status.is_server_error() {
            print!("{}", status_line.red().bold());
        } else if status.is_client_error() {
            print!("{}", status_line.yellow().bold());
        } else {
            print!("{}", status_line.cyan().bold());
        }
        println!(
            "{}  {}",
            format_elapsed(elapsed).dimmed(),
            t!("cmd.http.byte_count", byte_count).dimmed()
        );

        println!(
            "{}",
            format!("{} {}", status.as_u16(), status_text).dimmed()
        );
        for (name, value) in &headers {
            println!("{}: {}", name.cyan(), value);
        }
        println!();
    }

    if let Some(path) = &cfg.output {
        std::fs::write(path, &body).with_context(|| t!("cmd.http.write_fail", path).to_string())?;
        if !cfg.silent {
            crate::output::print_success(&t!("cmd.http.saved_bytes", byte_count, path));
        }
    } else if cfg.silent {
        println!("{}", body);
    } else if !body.is_empty() {
        if cfg.no_pretty {
            println!("{}", body);
        } else {
            let should_pretty = body.trim().starts_with('{') || body.trim().starts_with('[');
            if should_pretty {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Ok(pretty) = serde_json::to_string_pretty(&v) {
                        highlight_json(&pretty);
                    } else {
                        println!("{}", body);
                    }
                } else {
                    println!("{}", body);
                }
            } else {
                println!("{}", body);
            }
        }
    }

    Ok(())
}

fn format_elapsed(d: Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{}ms", ms)
    } else {
        let secs = ms as f64 / 1000.0;
        format!("{:.1}s", secs)
    }
}

fn highlight_json(text: &str) {
    for line in text.lines() {
        let trimmed = line.trim();
        let prefix = &line[..line.len() - trimmed.len()];
        if trimmed.starts_with('"') {
            let mut after_key = String::new();
            let mut colon = false;
            for (i, ch) in trimmed.char_indices() {
                if ch == ':' {
                    let key = &trimmed[..i];
                    let rest = &trimmed[i..];
                    after_key = format!("{}{}", key.cyan().bold(), rest);
                    colon = true;
                    break;
                }
            }
            if colon {
                println!("{}{}", prefix, after_key);
            } else {
                println!("{}{}", prefix, trimmed.cyan());
            }
        } else if trimmed.starts_with('{')
            || trimmed.starts_with('}')
            || trimmed.starts_with('[')
            || trimmed.starts_with(']')
        {
            println!("{}{}", prefix, trimmed.dimmed());
        } else if trimmed.starts_with("null") {
            println!("{}{}", prefix, trimmed.dimmed().italic());
        } else if trimmed.starts_with("true") || trimmed.starts_with("false") {
            println!("{}{}", prefix, trimmed.yellow().bold());
        } else {
            println!("{}{}", prefix, trimmed.green());
        }
    }
}

fn print_help() {
    println!("{}", t!("cmd.http.help_title").bold());
    println!();
    println!("{}", t!("cmd.http.help_usage_title").bold());
    println!("  {}", t!("cmd.http.help_usage_line"));
    println!("  {}", t!("cmd.http.help_usage_default"));
    println!();
    println!("{}", t!("cmd.http.help_methods_title").bold());
    println!("  {}", t!("cmd.http.help_methods"));
    println!();
    println!("{}", t!("cmd.http.help_options_title").bold());
    println!("  {}", t!("cmd.http.help_header"));
    println!("  {}", t!("cmd.http.help_body"));
    println!("  {}", t!("cmd.http.help_json"));
    println!("  {}", t!("cmd.http.help_query"));
    println!("  {}", t!("cmd.http.help_auth"));
    println!("  {}", t!("cmd.http.help_bearer"));
    println!("  {}", t!("cmd.http.help_timeout"));
    println!("  {}", t!("cmd.http.help_silent"));
    println!("  {}", t!("cmd.http.help_output"));
    println!("  {}", t!("cmd.http.help_verbose"));
    println!("  {}", t!("cmd.http.help_no_pretty"));
    println!("  {}", t!("cmd.http.help_help"));
    println!();
    println!("{}", t!("cmd.http.help_examples_title").bold());
    println!("  {}", t!("cmd.http.help_example1"));
    println!("  {}", t!("cmd.http.help_example2"));
    println!("  {}", t!("cmd.http.help_example3"));
    println!("  {}", t!("cmd.http.help_example4"));
    println!("  {}", t!("cmd.http.help_example5"));
    println!("  {}", t!("cmd.http.help_example6"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let t = tokenize("GET https://example.com");
        assert_eq!(t, vec!["GET", "https://example.com"]);
    }

    #[test]
    fn test_tokenize_quoted() {
        let t = tokenize("GET -H \"Content-Type: application/json\" -d 'hello world'");
        assert_eq!(
            t,
            vec![
                "GET",
                "-H",
                "Content-Type: application/json",
                "-d",
                "hello world"
            ]
        );
    }

    #[test]
    fn test_tokenize_mixed_quotes() {
        let t = tokenize("-H \"key: value\" -d '{\"name\":\"test\"}'");
        assert_eq!(t, vec!["-H", "key: value", "-d", "{\"name\":\"test\"}"]);
    }

    #[test]
    fn test_tokenize_empty() {
        let t = tokenize("");
        assert!(t.is_empty());
    }

    #[test]
    fn test_build_url_no_query() {
        assert_eq!(build_url("https://example.com", &[]), "https://example.com");
    }

    #[test]
    fn test_build_url_with_query() {
        let q = vec![("q".into(), "hello".into()), ("page".into(), "1".into())];
        let url = build_url("https://example.com/search", &q);
        assert_eq!(url, "https://example.com/search?q=hello&page=1");
    }

    #[test]
    fn test_build_url_existing_query() {
        let q = vec![("page".into(), "2".into())];
        let url = build_url("https://example.com/search?q=test", &q);
        assert_eq!(url, "https://example.com/search?q=test&page=2");
    }

    #[test]
    fn test_url_encode_safe() {
        assert_eq!(url_encode("hello"), "hello");
        assert_eq!(url_encode("a b"), "a%20b");
    }

    #[test]
    fn test_url_encode_special() {
        assert_eq!(url_encode("hello world!"), "hello%20world%21");
        assert_eq!(url_encode("a/b?c"), "a%2Fb%3Fc");
    }

    #[test]
    fn test_format_elapsed_ms() {
        assert_eq!(format_elapsed(Duration::from_millis(0)), "0ms");
        assert_eq!(format_elapsed(Duration::from_millis(500)), "500ms");
        assert_eq!(format_elapsed(Duration::from_millis(999)), "999ms");
    }

    #[test]
    fn test_format_elapsed_seconds() {
        assert_eq!(format_elapsed(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_elapsed(Duration::from_secs(2)), "2.0s");
        assert_eq!(format_elapsed(Duration::from_millis(1500)), "1.5s");
    }

    #[test]
    fn test_highlight_json_no_panic() {
        // Just ensure it doesn't panic on various inputs
        highlight_json("{}");
        highlight_json("{\"key\": \"value\"}");
        highlight_json("[1, 2, 3]");
        highlight_json("null");
        highlight_json("true");
        highlight_json("");
        highlight_json("plain text");
    }
}
