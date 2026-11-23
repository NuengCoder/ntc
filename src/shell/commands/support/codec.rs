//! Shared codec helpers for encode, decode, and cvcode.

use crate::output::{print_info, print_success, print_warning};
use crate::t;
use anyhow::{anyhow, bail, Result};
use std::path::Path;

pub(crate) fn resolve_target(raw: &str, cwd: &Path) -> Vec<u8> {
    // `-` (or no target) reads the piped/redirected input.
    if raw.is_empty() || raw == "-" {
        if let Some(bytes) = crate::shell::stdin::take_input() {
            return bytes;
        }
    }
    let path = cwd.join(raw);
    if path.is_file() {
        std::fs::read(&path).unwrap_or_else(|_| raw.as_bytes().to_vec())
    } else {
        raw.as_bytes().to_vec()
    }
}

pub(crate) fn parse_out_flag(args: &str) -> (&str, Option<&str>) {
    let args = args.trim();
    if let Some(pos) = args.rfind(" -o ") {
        let target = args[..pos].trim();
        let out = args[pos + 4..].trim();
        if !out.is_empty() && !out.starts_with('-') {
            return (target, Some(out));
        }
    }
    if let Some(pos) = args.rfind(" --output ") {
        let target = args[..pos].trim();
        let out = args[pos + 10..].trim();
        if !out.is_empty() && !out.starts_with('-') {
            return (target, Some(out));
        }
    }
    (args, None)
}

pub(crate) fn hex_preview(bytes: &[u8]) -> String {
    let preview = if bytes.len() > 32 {
        &bytes[..32]
    } else {
        bytes
    };
    let hex_part: String = preview.iter().map(|b| format!("{:02x} ", b)).collect();
    let ascii_part: String = preview
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            }
        })
        .collect();
    let total = bytes.len();
    if total > 32 {
        format!("{}...  |{}|  ({} total bytes)", hex_part, ascii_part, total)
    } else {
        format!("{} |{}|", hex_part, ascii_part)
    }
}

pub(crate) fn handle_output(data: Vec<u8>, out_path: Option<&str>, cwd: &Path) -> Result<()> {
    match out_path {
        Some(path) => {
            let full = cwd.join(path);
            std::fs::write(&full, &data)?;
            print_success(&t!("encode.written_to", full.display()));
        }
        None => {
            if let Ok(text) = String::from_utf8(data.clone()) {
                println!("{}", text);
            } else {
                print_warning(&t!("encode.binary_warning"));
                println!("{}", hex_preview(&data));
                print_info(&t!("encode.binary_hint"));
            }
        }
    }
    Ok(())
}

pub(crate) fn encode_b64(input: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input)
}

pub(crate) fn decode_b64(input: &str) -> Result<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, input.trim())
        .map_err(|e| anyhow!("{}", t!("encode.invalid_b64", e)))
}

pub(crate) fn encode_hex(input: &[u8]) -> String {
    input.iter().map(|b| format!("{:02x}", b)).collect()
}

pub(crate) fn decode_hex(input: &str) -> Result<Vec<u8>> {
    let s: String = input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    if s.len() % 2 != 0 {
        bail!("{}", t!("encode.hex_even"));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| anyhow!("{}", t!("encode.invalid_hex_pos", i)))
        })
        .collect()
}

pub(crate) fn encode_url(input: &[u8]) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    for &b in input {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(b as char);
            }
            b' ' => result.push_str("+"),
            _ => {
                result.push_str(&format!("%{:02X}", b));
            }
        }
    }
    result
}

pub(crate) fn encode_b2(input: &[u8]) -> String {
    input
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn decode_b2(input: &str) -> Result<Vec<u8>> {
    let s = input.trim();
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split_whitespace()
        .map(|token| {
            let t = token.trim().trim_start_matches("0b");
            u8::from_str_radix(t, 2).map_err(|_| anyhow!("{}", t!("encode.invalid_b2", token)))
        })
        .collect()
}

pub(crate) fn encode_b10(input: &[u8]) -> String {
    input
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn decode_b10(input: &str) -> Result<Vec<u8>> {
    let s = input.trim();
    if s.is_empty() {
        return Ok(Vec::new());
    }
    s.split_whitespace()
        .map(|token| {
            token
                .parse::<u8>()
                .map_err(|_| anyhow!("{}", t!("encode.invalid_b10", token)))
        })
        .collect()
}

pub(crate) fn decode_url(input: &str) -> Result<Vec<u8>> {
    let s = input.trim();
    let mut result = Vec::with_capacity(s.len());
    let mut chars = s.bytes().peekable();
    while let Some(c) = chars.next() {
        match c {
            b'+' => result.push(b' '),
            b'%' => {
                let hi = chars
                    .next()
                    .ok_or_else(|| anyhow!("{}", t!("encode.truncated_pct")))?;
                let lo = chars
                    .next()
                    .ok_or_else(|| anyhow!("{}", t!("encode.truncated_pct")))?;
                let hex = [hi, lo];
                let hex_str = std::str::from_utf8(&hex)
                    .map_err(|_| anyhow!("{}", t!("encode.invalid_pct")))?;
                let byte = u8::from_str_radix(hex_str, 16)
                    .map_err(|_| anyhow!("{}", t!("encode.invalid_hex_pct", hex_str)))?;
                result.push(byte);
            }
            _ => result.push(c),
        }
    }
    Ok(result)
}

/// Write one byte as printable text: ASCII stays literal, the usual control
/// characters get a short escape and everything else becomes `\xNN`.
fn push_escaped_byte(out: &mut String, b: u8) {
    match b {
        b'\\' => out.push_str("\\\\"),
        b'\n' => out.push_str("\\n"),
        b'\t' => out.push_str("\\t"),
        b'\r' => out.push_str("\\r"),
        0x20..=0x7e => out.push(b as char),
        _ => out.push_str(&format!("\\x{:02x}", b)),
    }
}

fn invalid_escape(seq: &str) -> anyhow::Error {
    anyhow!("{}", t!("encode.invalid_escape", seq))
}

/// Parse `\\`, `\n`, `\t`, `\r`, `\0` and `\xNN` escapes back into bytes.
fn parse_escaped_bytes(input: &str) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        match chars.next() {
            Some('\\') => out.push(b'\\'),
            Some('n') => out.push(b'\n'),
            Some('t') => out.push(b'\t'),
            Some('r') => out.push(b'\r'),
            Some('0') => out.push(0),
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                let byte = u8::from_str_radix(&hex, 16).map_err(|_| invalid_escape("\\x"))?;
                out.push(byte);
            }
            Some(other) => return Err(invalid_escape(&format!("\\{}", other))),
            None => return Err(invalid_escape("\\")),
        }
    }
    Ok(out)
}

/// UTF-8 as escaped bytes: `é` (c3 a9) becomes `\xc3\xa9`.
pub(crate) fn encode_utf8(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for &b in input {
        push_escaped_byte(&mut out, b);
    }
    out
}

pub(crate) fn decode_utf8(input: &str) -> Result<Vec<u8>> {
    let bytes = parse_escaped_bytes(input)?;
    std::str::from_utf8(&bytes).map_err(|_| anyhow!("{}", t!("encode.invalid_utf8")))?;
    Ok(bytes)
}

/// UTF-16 code units as `\uNNNN`, surrogates included (`\ud83d\ude00`).
pub(crate) fn encode_utf16(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 6);
    for unit in input.encode_utf16() {
        out.push_str(&format!("\\u{:04X}", unit));
    }
    out
}

pub(crate) fn decode_utf16(input: &str) -> Result<Vec<u8>> {
    let mut units: Vec<u16> = Vec::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u16; 2];
            units.extend_from_slice(c.encode_utf16(&mut buf));
            continue;
        }
        match chars.next() {
            Some('\\') => units.push(b'\\' as u16),
            Some('n') => units.push(b'\n' as u16),
            Some('t') => units.push(b'\t' as u16),
            Some('r') => units.push(b'\r' as u16),
            Some('0') => units.push(0),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                let unit = u16::from_str_radix(&hex, 16).map_err(|_| invalid_escape("\\u"))?;
                units.push(unit);
            }
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                let byte = u8::from_str_radix(&hex, 16).map_err(|_| invalid_escape("\\x"))?;
                units.push(byte as u16);
            }
            Some(other) => return Err(invalid_escape(&format!("\\{}", other))),
            None => return Err(invalid_escape("\\")),
        }
    }
    let text = String::from_utf16(&units).map_err(|_| anyhow!("{}", t!("encode.invalid_utf16")))?;
    Ok(text.into_bytes())
}

/// ASCII as printable text; any character above U+007F is rejected.
pub(crate) fn encode_ascii(input: &[u8]) -> Result<String> {
    let text = std::str::from_utf8(input).map_err(|_| anyhow!("{}", t!("encode.input_must_be_text")))?;
    let mut out = String::with_capacity(input.len());
    for ch in text.chars() {
        if !ch.is_ascii() {
            return Err(anyhow!("{}", t!("encode.not_ascii", ch)));
        }
        push_escaped_byte(&mut out, ch as u8);
    }
    Ok(out)
}

pub(crate) fn decode_ascii(input: &str) -> Result<Vec<u8>> {
    let bytes = parse_escaped_bytes(input)?;
    if let Some(b) = bytes.iter().find(|b| !b.is_ascii()) {
        return Err(anyhow!("{}", t!("encode.not_ascii_byte", format!("\\x{:02x}", b))));
    }
    Ok(bytes)
}

/// `plaintext` / `pt` is the unencoded format: raw text in, raw text out.
pub(crate) fn is_plaintext_type(fmt: &str) -> bool {
    matches!(fmt, "plaintext" | "pt")
}

pub(crate) fn decode_by_format(input: &str, fmt: &str) -> Result<Vec<u8>> {
    match fmt {
        "b2" => decode_b2(input),
        "b10" => decode_b10(input),
        "b64" | "base64" => decode_b64(input),
        "hex" => decode_hex(input),
        "url" => decode_url(input),
        "utf8" => decode_utf8(input),
        "utf16" => decode_utf16(input),
        "ascii" => decode_ascii(input),
        "plaintext" | "pt" => Ok(input.as_bytes().to_vec()),
        _ => bail!("{}", t!("encode.unknown_format_generic", fmt)),
    }
}

pub(crate) fn encode_by_format(input: &[u8], fmt: &str) -> Result<String> {
    Ok(match fmt {
        "b2" => encode_b2(input),
        "b10" => encode_b10(input),
        "b64" | "base64" => encode_b64(input),
        "hex" => encode_hex(input),
        "url" => encode_url(input),
        "utf8" => encode_utf8(input),
        "utf16" => {
            let text = std::str::from_utf8(input)
                .map_err(|_| anyhow!("{}", t!("encode.input_must_be_text")))?;
            encode_utf16(text)
        }
        "ascii" => encode_ascii(input)?,
        _ => t!("encode.unknown_format_fmt", fmt),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_escapes_non_ascii_bytes_only() {
        assert_eq!(encode_utf8("café".as_bytes()), "caf\\xc3\\xa9");
        assert_eq!(decode_utf8("caf\\xc3\\xa9").unwrap(), "café".as_bytes());
        assert_eq!(encode_utf8(b"plain"), "plain");
    }

    #[test]
    fn utf8_escapes_controls_and_backslashes() {
        assert_eq!(encode_utf8(b"a\tb\\c\n"), "a\\tb\\\\c\\n");
        assert_eq!(decode_utf8("a\\tb\\\\c\\n").unwrap(), b"a\tb\\c\n");
    }

    #[test]
    fn utf16_uses_four_digit_escapes_and_surrogate_pairs() {
        assert_eq!(encode_utf16("Hi"), "\\u0048\\u0069");
        assert_eq!(decode_utf16("\\u0048\\u0069").unwrap(), b"Hi");
        assert_eq!(encode_utf16("😀"), "\\uD83D\\uDE00");
        assert_eq!(decode_utf16("\\uD83D\\uDE00").unwrap(), "😀".as_bytes());
    }

    #[test]
    fn utf16_rejects_an_unpaired_surrogate() {
        assert!(decode_utf16("\\uD83D").is_err());
    }

    #[test]
    fn ascii_rejects_characters_above_127() {
        assert_eq!(encode_ascii(b"plain").unwrap(), "plain");
        assert!(encode_ascii("café".as_bytes()).is_err());
        assert!(decode_ascii("\\xc3").is_err());
    }

    #[test]
    fn unknown_escapes_are_rejected() {
        assert!(decode_utf8("\\q").is_err());
        assert!(decode_ascii("\\x").is_err());
    }

    #[test]
    fn format_dispatch_handles_the_new_types() {
        assert_eq!(encode_by_format(b"Hi", "utf16").unwrap(), "\\u0048\\u0069");
        assert_eq!(decode_by_format("\\u0048\\u0069", "utf16").unwrap(), b"Hi");
        assert_eq!(decode_by_format("\\xc3\\xa9", "utf8").unwrap(), "é".as_bytes());
        assert_eq!(decode_by_format("plain", "plaintext").unwrap(), b"plain");
        assert_eq!(decode_by_format("plain", "pt").unwrap(), b"plain");
        assert!(is_plaintext_type("pt") && is_plaintext_type("plaintext"));
        assert!(decode_by_format("zz", "nope").is_err());
    }
}
