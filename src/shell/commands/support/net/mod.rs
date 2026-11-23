//! Shared download/upload/DNS tokenization, size formatting and HTTP
//! plumbing used by the `serve` services.

pub(crate) mod diag;
pub(crate) mod dns;
pub(crate) mod http;

// ── Shared helpers ────────────────────────────────────────────────────────

pub(crate) fn tokenize(input: &str) -> Vec<String> {
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

pub(crate) fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len().saturating_sub(1) {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{}{}", bytes, UNITS[unit])
    } else {
        format!("{:.1}{}", size, UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_splits_a_bare_host() {
        // `tokienize` only splits; each command parses its own flags.
        let t = tokenize("example.com");
        assert_eq!(t, vec!["example.com"]);
    }

    #[test]
    fn tokenize_keeps_host_and_flags_in_order() {
        let t = tokenize("example.com -c 5 -p 443 -t 1");
        assert_eq!(t, vec!["example.com", "-c", "5", "-p", "443", "-t", "1"]);
    }

    #[test]
    fn test_tokenize_simple() {
        let t = tokenize("https://example.com/file.zip -o out.zip");
        assert_eq!(t, vec!["https://example.com/file.zip", "-o", "out.zip"]);
    }

    #[test]
    fn test_tokenize_quoted() {
        let t = tokenize("-H \"Content-Type: application/json\" -d 'body data'");
        assert_eq!(
            t,
            vec!["-H", "Content-Type: application/json", "-d", "body data"]
        );
    }

    #[test]
    fn test_tokenize_empty() {
        assert!(tokenize("").is_empty());
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0B");
        assert_eq!(format_size(1), "1B");
        assert_eq!(format_size(1023), "1023B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.0KB");
        assert_eq!(format_size(1536), "1.5KB");
        assert_eq!(format_size(1024 * 1023), "1023.0KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.0MB");
        assert_eq!(format_size(1024 * 1024 * 10), "10.0MB");
        assert_eq!(format_size(1024 * 1024 + 512 * 1024), "1.5MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0GB");
        assert_eq!(format_size(5 * 1024 * 1024 * 1024), "5.0GB");
    }
}
