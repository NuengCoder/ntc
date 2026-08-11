use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use anyhow::{anyhow, bail, Result};
use colored::*;
use std::path::Path;

fn resolve_target(raw: &str, cwd: &Path) -> Vec<u8> {
    let path = cwd.join(raw);
    if path.is_file() {
        std::fs::read(&path).unwrap_or_else(|_| raw.as_bytes().to_vec())
    } else {
        raw.as_bytes().to_vec()
    }
}

fn parse_out_flag(args: &str) -> (&str, Option<&str>) {
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

fn hex_preview(bytes: &[u8]) -> String {
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

fn handle_output(data: Vec<u8>, out_path: Option<&str>, cwd: &Path) -> Result<()> {
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

fn encode_b64(input: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input)
}

fn decode_b64(input: &str) -> Result<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, input.trim())
        .map_err(|e| anyhow!("{}", t!("encode.invalid_b64", e)))
}

fn encode_hex(input: &[u8]) -> String {
    input.iter().map(|b| format!("{:02x}", b)).collect()
}

fn decode_hex(input: &str) -> Result<Vec<u8>> {
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

fn encode_url(input: &[u8]) -> String {
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

fn encode_b2(input: &[u8]) -> String {
    input
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn decode_b2(input: &str) -> Result<Vec<u8>> {
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

fn decode_url(input: &str) -> Result<Vec<u8>> {
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

fn print_usage_encode() {
    println!("{}", t!("encode.usage_title").cyan().bold());
    println!("{}", t!("encode.usage_encode_b2"));
    println!("{}", t!("encode.usage_encode_b64"));
    println!("{}", t!("encode.usage_encode_hex"));
    println!("{}", t!("encode.usage_encode_url"));
    println!();
    println!("{}", t!("encode.usage_examples").green());
    println!("  encode b64 hello");
    println!("  encode b64 image.png -o encoded.txt");
    println!("  encode b2  \"AB\"");
    println!("  encode hex \"hello world\"");
    println!("  encode url \"foo bar\"");
}

fn print_usage_decode() {
    println!("{}", t!("encode.usage_title").cyan().bold());
    println!("{}", t!("decode.usage_decode_b2"));
    println!("{}", t!("decode.usage_decode_b64"));
    println!("{}", t!("decode.usage_decode_hex"));
    println!("{}", t!("decode.usage_decode_url"));
    println!();
    println!("{}", t!("encode.usage_examples").green());
    println!("  decode b64 SGVsbG8=");
    println!("  decode b64 encoded.txt -o image.png");
    println!("  decode b2  01000001 01000010");
    println!("  decode hex 68656c6c6f");
    println!("  decode url \"hello+world\"");
}

fn print_usage_cvcode() {
    println!("{}", t!("encode.usage_title").cyan().bold());
    println!("{}", t!("cvcode.usage"));
    println!();
    println!("{}", t!("cvcode.formats").cyan());
    println!();
    println!("{}", t!("encode.usage_examples").green());
    println!("  cvcode b64  aGVsbG8= to hex");
    println!("  cvcode hex  68656c6c6f to b64");
    println!("  cvcode url  hello+world to b2");
    println!("  cvcode b2   \"01101000 01100101\" to url -o output.txt");
}

pub fn cmd_encode(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();
    let parts: Vec<&str> = args.trim().splitn(2, ' ').collect();
    let sub = parts.first().copied().unwrap_or("");
    let rest = parts.get(1).unwrap_or(&"").trim();

    match sub {
        "" => {
            print_usage_encode();
            Ok(false)
        }
        "b2" => cmd_encode_b2(rest, &cwd),
        "b64" | "base64" => cmd_encode_b64(rest, &cwd),
        "hex" => cmd_encode_hex(rest, &cwd),
        "url" => cmd_encode_url(rest, &cwd),
        _ => {
            print_error(&t!("encode.unknown_format", sub));
            Ok(false)
        }
    }
}

fn cmd_encode_b2(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("encode.usage_cmd", "b2"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_b2(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_b64(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("encode.usage_cmd", "b64"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_b64(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_hex(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("encode.usage_cmd", "hex"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_hex(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_url(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("encode.usage_cmd", "url"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_url(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

pub fn cmd_decode(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();
    let parts: Vec<&str> = args.trim().splitn(2, ' ').collect();
    let sub = parts.first().copied().unwrap_or("");
    let rest = parts.get(1).unwrap_or(&"").trim();

    match sub {
        "" => {
            print_usage_decode();
            Ok(false)
        }
        "b2" => cmd_decode_b2(rest, &cwd),
        "b64" | "base64" => cmd_decode_b64(rest, &cwd),
        "hex" => cmd_decode_hex(rest, &cwd),
        "url" => cmd_decode_url(rest, &cwd),
        _ => {
            print_error(&t!("decode.unknown_format", sub));
            Ok(false)
        }
    }
}

fn cmd_decode_b2(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("decode.usage_cmd", "b2"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str =
        String::from_utf8(raw).map_err(|_| anyhow!("{}", t!("encode.input_not_text", "Binary")))?;
    let decoded = decode_b2(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

fn cmd_decode_b64(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("decode.usage_cmd", "b64"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str =
        String::from_utf8(raw).map_err(|_| anyhow!("{}", t!("encode.input_not_text", "Base64")))?;
    let decoded = decode_b64(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

fn cmd_decode_hex(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("decode.usage_cmd", "hex"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str =
        String::from_utf8(raw).map_err(|_| anyhow!("{}", t!("encode.input_not_text", "Hex")))?;
    let decoded = decode_hex(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

fn cmd_decode_url(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("decode.usage_cmd", "url"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str =
        String::from_utf8(raw).map_err(|_| anyhow!("{}", t!("encode.input_not_text", "URL")))?;
    let decoded = decode_url(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

// ── cvcode: convert between encoding formats ──────────────────────────

fn decode_by_format(input: &str, fmt: &str) -> Result<Vec<u8>> {
    match fmt {
        "b2" => decode_b2(input),
        "b64" | "base64" => decode_b64(input),
        "hex" => decode_hex(input),
        "url" => decode_url(input),
        _ => bail!("{}", t!("encode.unknown_format_generic", fmt)),
    }
}

fn encode_by_format(input: &[u8], fmt: &str) -> String {
    match fmt {
        "b2" => encode_b2(input),
        "b64" | "base64" => encode_b64(input),
        "hex" => encode_hex(input),
        "url" => encode_url(input),
        _ => t!("encode.unknown_format_fmt", fmt),
    }
}

pub fn cmd_cvcode(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let cwd = _nav.current_path().to_path_buf();
    let args = args.trim();

    if args.is_empty() {
        print_usage_cvcode();
        return Ok(false);
    }

    // Parse: <from_type> <data...> to <to_type> [-o <file>]
    let from_end = args.find(' ').unwrap_or(args.len());
    let from_type = &args[..from_end];
    let rest = args[from_end..].trim();

    let to_pos = rest.rfind(" to ");
    let to_pos = match to_pos {
        Some(p) => p,
        None => {
            print_error(&t!("cvcode.usage_detailed"));
            print_usage_cvcode();
            return Ok(false);
        }
    };

    let data_part = rest[..to_pos].trim();
    let to_part = rest[to_pos + 4..].trim();

    if data_part.is_empty() {
        print_error(&t!("cvcode.missing_data"));
        return Ok(false);
    }
    if to_part.is_empty() {
        print_error(&t!("cvcode.missing_to"));
        return Ok(false);
    }

    let (to_type_raw, out) = parse_out_flag(to_part);
    let to_type = to_type_raw.trim();

    let raw = resolve_target(data_part, &cwd);
    let input_str =
        String::from_utf8(raw).map_err(|_| anyhow!("{}", t!("encode.input_must_be_text")))?;

    let decoded = decode_by_format(&input_str, from_type)?;
    let encoded = encode_by_format(&decoded, to_type);

    handle_output(encoded.into_bytes(), out, &cwd)?;
    Ok(false)
}
