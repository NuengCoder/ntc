// Submodules
pub(crate) mod token;
mod ast;
mod eval;
mod builtins;
mod check;
mod function;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use colored::Colorize;

use self::ast::{Expr, Parser};
use self::eval::{eval, EvalResult};
use self::token::tokenize;
pub(crate) use self::builtins::{builtin_function_names, constant_names};
pub(crate) use self::check::validate;

// ============================================================================
// Main entry point
// ============================================================================
pub fn run(input: &str) -> Result<()> {
    let input = input.trim();

    if input.is_empty() {
        print_help();
        return Ok(());
    }

    if let Some(stripped) = input.strip_prefix("timer") {
        return handle_timer(stripped.trim());
    }

    if input.starts_with("fun ") || input == "fun" {
        return function::handle_fun(input[3..].trim());
    }

    let path = Path::new(input);
    if path.exists() {
        if path.is_file() {
            return handle_file(path);
        } else {
            bail!("'{}' is a directory, not a file", input);
        }
    }

    if input.ends_with(".ntc.math") {
        bail!("File not found: '{}'", input);
    }

    let mut vars = HashMap::new();
    eval_line(input, &mut vars, true)?;
    Ok(())
}

// ============================================================================
// Help
// ============================================================================
fn print_help() {
    println!("{}", "Math Module - Usage:".cyan().bold());
    println!("  math <expression>       Evaluate a math expression");
    println!("  math <func>(<args>)     Call a built-in or user-defined function");
    println!("  math timer [seconds]    Lap timer or countdown with alarm");
    println!("  math fun add <name>(<params>) = <body>   Define a function");
    println!("  math fun rm <name>      Remove a function");
    println!("  math fun edit <name> = <body>  Update a function");
    println!("  math fun info <name>    Show function details");
    println!("  math fun ls             List all user-defined functions");
    println!("  math <file>.ntc.math    Compile and run a math file");
    println!();
    println!("{}", "Examples:".green());
    println!("  math 3+4*5                           → 23");
    println!("  math sin(PI/2)                       → 1");
    println!("  math sqrt(144)                        → 12");
    println!("  math rand(1,100)                      → 42");
    println!("  math toBinary(255)                    → 11111111");
    println!("  math toHex(255)                       → FF");
    println!("  math toHB(1048576)                    → 1.00 MB");
    println!("  math 0xFF + 0b1010                   → 265");
    println!("  math timer 10                        → countdown 10s");
    println!("  math fun add square(x) = x^2");
    println!("  math square(5)                        → 25");
    println!();
    println!("{}", "Built-in functions:".yellow());
    println!("  print: print values (like Python)");
    println!("  trig: sin, cos, tan, cot, sec, csc");
    println!("  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc");
    println!("  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10");
    println!("  agg:  sum, min, max, avg");
    println!("  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB");
    println!("  rand: rand(min,max)");
    println!("  constants: PI, E, PHI, TAU");
}

// ============================================================================
// Timer
// ============================================================================
static TIMER_START: Mutex<Option<Instant>> = Mutex::new(None);

fn handle_timer(args: &str) -> Result<()> {
    if args.is_empty() {
        let mut timer = TIMER_START.lock().unwrap();
        if let Some(start) = *timer {
            let elapsed = start.elapsed();
            println!("Lap time: {}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());
            *timer = None;
        } else {
            *timer = Some(Instant::now());
            println!("Timer started...");
        }
        return Ok(());
    }

    let seconds: u64 = args.split_whitespace().next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow::anyhow!("Usage: math timer <seconds>"))?;

    println!("Countdown: {} seconds", seconds);
    for remaining in (0..=seconds).rev() {
        print!("\rTime remaining: {:>3} seconds  ", remaining);
        use std::io::Write;
        std::io::stdout().flush()?;
        if remaining > 0 {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    println!();

    for _ in 0..5 {
        print!("\x07");
        use std::io::Write;
        std::io::stdout().flush()?;
        std::thread::sleep(Duration::from_millis(300));
    }

    println!("{}", "Time's up!".red().bold());
    Ok(())
}

// ============================================================================
// File compilation (.ntc.math)
// ============================================================================
fn handle_file(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read '{}': {}", path.display(), e))?;
    let content = content.trim_start_matches('\u{feff}');

    println!("{}", format!("╔═ Compiled: {} ═╗", path.display()).cyan().bold());

    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut has_return = false;

    for (line_num, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") { continue; }

        match eval_line_in_file(line, &mut vars) {
            Ok(Some(result)) => {
                println!("{}", result);
                has_return = true;
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("  {} at line {}: {}", "Error".red().bold(), line_num + 1, e);
            }
        }
    }

    if !has_return {
        // If no return statement, print the value of the last non-assignment line
    }

    println!("{}", format!("╚═ End: {} ═╝", path.display()).cyan().bold());
    Ok(())
}

fn eval_line_in_file(line: &str, vars: &mut HashMap<String, f64>) -> Result<Option<EvalResult>> {
    let tokens = tokenize(line)?;
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_statement()?;

    match &stmt {
        Expr::Assign(_, _) => {
            let _result = eval(&stmt, vars)?;
            Ok(None)
        }
        Expr::Return(_) => {
            let result = eval(&stmt, vars)?;
            Ok(Some(result))
        }
        Expr::Call(name, _) if name == "print" => {
            let _result = eval(&stmt, vars)?;
            Ok(None)
        }
        _ => {
            let result = eval(&stmt, vars)?;
            Ok(Some(result))
        }
    }
}

// ============================================================================
// Line evaluation (for both interactive and file modes)
// ============================================================================
fn eval_line(line: &str, vars: &mut HashMap<String, f64>, print_result: bool) -> Result<()> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') { return Ok(()); }

    let tokens = tokenize(trimmed)?;
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_statement()?;

    let result = eval(&stmt, vars)?;

    if print_result {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        assert!(run("1+2").is_ok());
        assert!(run("3*4+5").is_ok());
        assert!(run("10/2").is_ok());
        assert!(run("2^8").is_ok());
    }

    #[test]
    fn test_parentheses() {
        assert!(run("(1+2)*3").is_ok());
        assert!(run("(2+3)*(4+5)").is_ok());
    }

    #[test]
    fn test_builtin_functions() {
        assert!(run("sin(0)").is_ok());
        assert!(run("cos(0)").is_ok());
        assert!(run("sqrt(4)").is_ok());
        assert!(run("abs(-5)").is_ok());
    }

    #[test]
    fn test_variable_assignment() {
        assert!(run("x = 42").is_ok());
    }

    #[test]
    fn test_string_output() {
        assert!(run(r#"print("hello")"#).is_ok());
    }

    #[test]
    fn test_invalid_expression() {
        assert!(run("1+/2").is_err());
        assert!(run("(1+2").is_err());
        assert!(run("sin").is_err());
    }

    #[test]
    fn test_empty_input() {
        assert!(run("").is_ok());
        assert!(run("  ").is_ok());
    }

    #[test]
    fn test_comment_line() {
        assert!(run("# this is a comment").is_ok());
    }

    #[test]
    fn test_float_arithmetic() {
        assert!(run("3.14 * 2").is_ok());
        assert!(run("0.5 + 0.25").is_ok());
    }

    #[test]
    fn test_negation() {
        assert!(run("-5 + 3").is_ok());
    }

    #[test]
    fn test_math_constants() {
        assert!(run("PI").is_ok());
        assert!(run("E").is_ok());
    }

    #[test]
    fn test_tokenize_simple() -> Result<()> {
        let (tokens, _) = crate::math::token::tokenize_with_pos("1+2")?;
        assert!(!tokens.is_empty());
        assert_eq!(tokens.len(), 4);
        Ok(())
    }

    #[test]
    fn test_tokenize_with_parens() -> Result<()> {
        let (tokens, _) = crate::math::token::tokenize_with_pos("(a+b)")?;
        assert_eq!(tokens.len(), 6);
        Ok(())
    }

    #[test]
    fn test_tokenize_string_literal() -> Result<()> {
        let (tokens, _) = crate::math::token::tokenize_with_pos(r#""hello world""#)?;
        assert_eq!(tokens.len(), 2);
        Ok(())
    }
}
