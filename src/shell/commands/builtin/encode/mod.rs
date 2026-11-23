//! `encode` — base64, hex, binary, decimal, and URL encoding.

pub(crate) mod completion;
pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{anyhow, Result};
use std::path::Path;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::codec::{
    encode_ascii, encode_b10, encode_b2, encode_b64, encode_hex, encode_url, encode_utf16,
    encode_utf8, handle_output, parse_out_flag, resolve_target,
};

fn print_usage_encode() {
    println!("{}", t!("encode.usage_title").cyan().bold());
    println!("{}", t!("encode.usage_encode_b2"));
    println!("{}", t!("encode.usage_encode_b10"));
    println!("{}", t!("encode.usage_encode_b64"));
    println!("{}", t!("encode.usage_encode_hex"));
    println!("{}", t!("encode.usage_encode_url"));
    println!("{}", t!("encode.usage_encode_utf8"));
    println!("{}", t!("encode.usage_encode_utf16"));
    println!("{}", t!("encode.usage_encode_ascii"));
    println!();
    println!("{}", t!("encode.usage_examples").green());
    println!("  encode b64 hello");
    println!("  encode b64 image.png -o encoded.txt");
    println!("  encode b2  \"AB\"");
    println!("  encode b10 \"AB\"");
    println!("  encode hex \"hello world\"");
    println!("  encode url \"foo bar\"");
    println!("  encode utf8 \"café\"");
    println!("  encode utf16 \"hi\"");
    println!("  encode ascii \"plain\"");
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
        "b10" => cmd_encode_b10(rest, &cwd),
        "b64" | "base64" => cmd_encode_b64(rest, &cwd),
        "hex" => cmd_encode_hex(rest, &cwd),
        "url" => cmd_encode_url(rest, &cwd),
        "utf8" => cmd_encode_utf8(rest, &cwd),
        "utf16" => cmd_encode_utf16(rest, &cwd),
        "ascii" => cmd_encode_ascii(rest, &cwd),
        _ => {
            print_error(&t!("encode.unknown_format", sub));
            Ok(false)
        }
    }
}

fn cmd_encode_b2(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "b2"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_b2(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_b10(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "b10"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_b10(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_b64(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
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
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "url"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_url(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_utf8(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "utf8"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_utf8(&data);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_utf16(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "utf16"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let text = std::str::from_utf8(&data)
        .map_err(|_| anyhow!("{}", t!("encode.input_must_be_text")))?;
    let encoded = encode_utf16(text);
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

fn cmd_encode_ascii(args: &str, cwd: &Path) -> Result<bool> {
    if args.is_empty() && !crate::shell::stdin::has_any_input() {
        print_error(&t!("encode.usage_cmd", "ascii"));
        return Ok(false);
    }
    let (target, out) = parse_out_flag(args);
    let data = resolve_target(target, cwd);
    let encoded = encode_ascii(&data)?;
    handle_output(encoded.into_bytes(), out, cwd)?;
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "encode",
    aliases: &[],
    help_topic: Some("encode"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_encode),
};
