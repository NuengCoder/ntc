//! `say` — print text, optionally repeated.

pub(crate) mod help;

pub(crate) mod completion;

use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_warning};
use crate::t;
use anyhow::Result;

use super::super::core::spec::{CommandSource, CommandSpec, FlagPolicy, Handler};
use super::super::support::args::tokenize_quoted;

pub(crate) const MAX_SAY_LOOP: i32 = 10000;

pub(crate) fn cmd_say(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize_quoted(args);

    let mut text = String::new();
    let mut loop_count: i32 = 1;
    let mut use_tabs = false; // -t → space-separated single line; else newline-separated

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if token == "--loop" {
            i += 1;
            if i < tokens.len() {
                if let Ok(n) = tokens[i].parse::<i32>() {
                    if n <= 0 {
                        print_warning(&t!("misc.say.loop_warning"));
                        loop_count = 1;
                    } else if n > MAX_SAY_LOOP {
                        print_warning(&t!("misc.say.loop_exceed", MAX_SAY_LOOP));
                        loop_count = MAX_SAY_LOOP;
                    } else {
                        loop_count = n;
                    }
                } else {
                    print_warning(&t!("misc.say.loop_invalid", tokens[i]));
                }
            } else {
                print_warning(&t!("misc.say.loop_requires"));
            }
        } else if token == "-n" {
            use_tabs = false;
        } else if token == "-t" {
            use_tabs = true;
        } else if token == "--" {
            break;
        } else {
            if !text.is_empty() {
                text.push(' ');
            }
            text.push_str(token);
        }
        i += 1;
    }

    if text.is_empty() {
        // No text given → read piped/redirected input and echo it.
        if let Some(input) = crate::shell::stdin::read_to_string() {
            print!("{}", input);
            if !input.is_empty() && !input.ends_with('\n') {
                println!();
            }
            return Ok(false);
        }
        print_error(&t!("misc.say.usage"));
        print_info(&t!("misc.say.example1"));
        print_info(&t!("misc.say.example2"));
        return Ok(false);
    }

    if use_tabs {
        let line = (0..loop_count)
            .map(|_| text.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        println!("{}", line);
    } else {
        for _ in 0..loop_count {
            println!("{}", text);
        }
    }

    Ok(false)
}

pub(crate) static SPEC: CommandSpec = CommandSpec {
    name: "say",
    aliases: &[],
    help_topic: Some("say"),
    flags: FlagPolicy::HelpIsHelp,
    contextual: true,
    non_contextual_aliases: &[],
    complete: &completion::COMPLETION,
    subcommand_highlight: false,
    source: CommandSource::Builtin,
    run: Handler::Legacy(cmd_say),
};
