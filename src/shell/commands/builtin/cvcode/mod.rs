//! `cvcode` — convert data between encoding formats.

pub(crate) mod help;

use crate::navigator::Navigator;
use crate::output::print_error;
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::{anyhow, Result};

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::codec::{
    decode_by_format, encode_by_format, handle_output, is_plaintext_type, parse_out_flag,
    resolve_target,
};

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
    println!("  cvcode b2   00101100 to b10");
    println!("  cvcode b10  35 to b2");
    println!("  cvcode b64  aGVsbG8= to plaintext");
    println!("  cvcode plaintext hello to utf16");
    println!("  cvcode utf16 \"\\u0068\\u0069\" to utf8");
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
    // `plaintext` (or `pt`) targets skip encoding entirely, so binary output
    // still gets the hex preview that `handle_output` prints for it.
    let bytes = if is_plaintext_type(to_type) {
        decoded
    } else {
        encode_by_format(&decoded, to_type)?.into_bytes()
    };

    handle_output(bytes, out, &cwd)?;
    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "cvcode",
    aliases: &[],
    help_topic: Some("cvcode"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &crate::shell::commands::core::completion::EMPTY,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_cvcode),
};
