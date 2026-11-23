//! `decode` — base64, hex, binary, decimal, and URL decoding.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{anyhow, Result};
use std::path::Path;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::codec::{
    decode_ascii, decode_b10, decode_b2, decode_b64, decode_hex, decode_url, decode_utf16,
    decode_utf8, handle_output, parse_out_flag, resolve_target,
};

fn print_usage_decode() {
    println!("{}", t!("encode.usage_title").cyan().bold());
    println!("{}", t!("decode.usage_decode_b2"));
    println!("{}", t!("decode.usage_decode_b10"));
    println!("{}", t!("decode.usage_decode_b64"));
    println!("{}", t!("decode.usage_decode_hex"));
    println!("{}", t!("decode.usage_decode_url"));
    println!("{}", t!("decode.usage_decode_utf8"));
    println!("{}", t!("decode.usage_decode_utf16"));
    println!("{}", t!("decode.usage_decode_ascii"));
    println!();
    println!("{}", t!("encode.usage_examples").green());
    println!("  decode b64 SGVsbG8=");
    println!("  decode b64 encoded.txt -o image.png");
    println!("  decode b2  01000001 01000010");
    println!("  decode b10 65 66");
    println!("  decode hex 68656c6c6f");
    println!("  decode url \"hello+world\"");
    println!("  decode utf8 \"caf\\xc3\\xa9\"");
    println!("  decode utf16 \"\\u0068\\u0069\"");
    println!("  decode ascii \"plain\"");
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
        "b10" => cmd_decode_b10(rest, &cwd),
        "b64" | "base64" => cmd_decode_b64(rest, &cwd),
        "hex" => cmd_decode_hex(rest, &cwd),
        "url" => cmd_decode_url(rest, &cwd),
        "utf8" => cmd_decode_utf8(rest, &cwd),
        "utf16" => cmd_decode_utf16(rest, &cwd),
        "ascii" => cmd_decode_ascii(rest, &cwd),
        _ => {
            print_error(&t!("decode.unknown_format", sub));
            Ok(false)
        }
    }
}

fn cmd_decode_b2(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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

fn cmd_decode_b10(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("decode.usage_cmd", "b10"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str = String::from_utf8(raw)
        .map_err(|_| anyhow!("{}", t!("encode.input_not_text", "Decimal")))?;
    let decoded = decode_b10(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

fn cmd_decode_b64(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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

fn cmd_decode_utf8(args: &str, cwd: &Path) -> Result<bool> {
    decode_text_format(args, cwd, "utf8", decode_utf8)
}

fn cmd_decode_utf16(args: &str, cwd: &Path) -> Result<bool> {
    decode_text_format(args, cwd, "utf16", decode_utf16)
}

fn cmd_decode_ascii(args: &str, cwd: &Path) -> Result<bool> {
    decode_text_format(args, cwd, "ascii", decode_ascii)
}

/// Shared body for the escaped-text formats: read the argument or piped text
/// and hand the decoded bytes to `handle_output`.
fn decode_text_format(
    args: &str,
    cwd: &Path,
    name: &str,
    decode: fn(&str) -> Result<Vec<u8>>,
) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("decode.usage_cmd", name));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let raw = resolve_target(target, cwd);
    let input_str = String::from_utf8(raw)
        .map_err(|_| anyhow!("{}", t!("encode.input_not_text", name)))?;
    let decoded = decode(&input_str)?;
    handle_output(decoded, out, cwd)?;
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "decode",
    aliases: &[],
    help_topic: Some("decode"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_decode),
};
