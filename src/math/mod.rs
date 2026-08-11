// Submodules
mod ast;
mod builtins;
mod check;
mod eval;
mod function;
pub(crate) mod token;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};
use colored::Colorize;

use crate::t;

use self::ast::{Expr, Parser};
pub(crate) use self::builtins::{builtin_function_names, constant_names};
pub(crate) use self::check::validate;
use self::eval::{eval, EvalResult};
use self::token::tokenize;

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
            bail!(t!("math.is_directory", input));
        }
    }

    if input.ends_with(".ntc.math") {
        bail!(t!("math.file_not_found", input));
    }

    let mut vars = HashMap::new();
    eval_line(input, &mut vars, true)?;
    Ok(())
}

// ============================================================================
// Help
// ============================================================================
fn print_help() {
    println!("{}", t!("math.help_title").cyan().bold());
    println!("{}", t!("math.help_expr"));
    println!("{}", t!("math.help_func_call"));
    println!("{}", t!("math.help_timer"));
    println!("{}", t!("math.help_fun_add"));
    println!("{}", t!("math.help_fun_rm"));
    println!("{}", t!("math.help_fun_edit"));
    println!("{}", t!("math.help_fun_info"));
    println!("{}", t!("math.help_fun_ls"));
    println!("{}", t!("math.help_file"));
    println!();
    println!("{}", t!("math.examples_header").green());
    println!("{}", t!("math.example_add"));
    println!("{}", t!("math.example_sin"));
    println!("{}", t!("math.example_sqrt"));
    println!("{}", t!("math.example_rand"));
    println!("{}", t!("math.example_tobinary"));
    println!("{}", t!("math.example_tohex"));
    println!("{}", t!("math.example_tohb"));
    println!("{}", t!("math.example_hexbin"));
    println!("{}", t!("math.example_timer"));
    println!("{}", t!("math.example_fun_def"));
    println!("{}", t!("math.example_fun_call"));
    println!();
    println!("{}", t!("math.builtin_header").yellow());
    println!("{}", t!("math.builtin_print"));
    println!("{}", t!("math.builtin_trig"));
    println!("{}", t!("math.builtin_inv_trig"));
    println!("{}", t!("math.builtin_math"));
    println!("{}", t!("math.builtin_agg"));
    println!("{}", t!("math.builtin_conv"));
    println!("{}", t!("math.builtin_rand"));
    println!("{}", t!("math.builtin_constants"));
}

// ============================================================================
// Timer
// ============================================================================
static TIMER_START: Mutex<Option<Instant>> = Mutex::new(None);

fn handle_timer(args: &str) -> Result<()> {
    if args.is_empty() {
        let mut timer = TIMER_START.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(start) = *timer {
            let elapsed = start.elapsed();
            println!(
                "{}",
                t!("math.lap_time", elapsed.as_secs(), elapsed.subsec_millis())
            );
            *timer = None;
        } else {
            *timer = Some(Instant::now());
            println!("{}", t!("math.timer_started"));
        }
        return Ok(());
    }

    let seconds: u64 = args
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| anyhow::anyhow!(t!("math.timer_usage")))?;

    println!("{}", t!("math.countdown", seconds));
    for remaining in (0..=seconds).rev() {
        print!("\r{}", t!("math.time_remaining", remaining));
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

    println!("{}", t!("math.times_up").red().bold());
    Ok(())
}

// ============================================================================
// File compilation (.ntc.math)
// ============================================================================
fn handle_file(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!(t!("math.failed_read", path.display(), e)))?;
    let content = content.trim_start_matches('\u{feff}');

    println!(
        "{}",
        t!("math.compiled_header", path.display()).cyan().bold()
    );

    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut has_return = false;

    for (line_num, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }

        match eval_line_in_file(line, &mut vars) {
            Ok(Some(result)) => {
                println!("{}", result);
                has_return = true;
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!(
                    "  {} at line {}: {}",
                    t!("math.error").red().bold(),
                    line_num + 1,
                    e
                );
            }
        }
    }

    if !has_return {
        // If no return statement, print the value of the last non-assignment line
    }

    println!("{}", t!("math.exec_end", path.display()).cyan().bold());
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
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return Ok(());
    }

    let tokens = tokenize(trimmed)?;
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_statement()?;

    let result = eval(&stmt, vars)?;

    if print_result {
        println!("{}", result);
    }

    Ok(())
}

/// Evaluate a math expression and return the result as a string (for testing).
#[cfg(test)]
fn eval_to_string(input: &str) -> Result<String> {
    let trimmed = input.trim();
    let tokens = tokenize(trimmed)?;
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse_statement()?;
    let mut vars = HashMap::new();
    let result = eval(&stmt, &mut vars)?;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Arithmetic ----

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(eval_to_string("1+2").unwrap(), "3");
        assert_eq!(eval_to_string("3*4+5").unwrap(), "17");
        assert_eq!(eval_to_string("10/2").unwrap(), "5");
        assert_eq!(eval_to_string("2^8").unwrap(), "256");
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval_to_string("(1+2)*3").unwrap(), "9");
        assert_eq!(eval_to_string("(2+3)*(4+5)").unwrap(), "45");
    }

    // ---- Built-in functions ----

    #[test]
    fn test_builtin_functions() {
        assert_eq!(eval_to_string("sin(0)").unwrap(), "0");
        assert_eq!(eval_to_string("cos(0)").unwrap(), "1");
        assert_eq!(eval_to_string("sqrt(4)").unwrap(), "2");
        assert_eq!(eval_to_string("abs(-5)").unwrap(), "5");
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
        assert_eq!(eval_to_string("3.14 * 2").unwrap(), "6.28");
        let v: f64 = eval_to_string("0.5 + 0.25").unwrap().parse().unwrap();
        assert!((v - 0.75).abs() < 1e-10);
    }

    #[test]
    fn test_negation() {
        assert_eq!(eval_to_string("-5 + 3").unwrap(), "-2");
    }

    #[test]
    fn test_math_constants() {
        assert_eq!(eval_to_string("PI").unwrap(), "3.141592653589793");
        assert_eq!(eval_to_string("E").unwrap(), "2.718281828459045");
    }

    // ---- Tokenizer ----

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

    // ---- More comprehensive value assertions ----

    #[test]
    fn test_rand_returns_number() {
        let result = eval_to_string("rand(1,100)").unwrap();
        let v: f64 = result.parse().unwrap();
        assert!((1.0..=100.0).contains(&v));
    }

    #[test]
    fn test_tobinary() {
        let result = eval_to_string("toBinary(255)").unwrap();
        assert_eq!(result, "11111111");
    }

    #[test]
    fn test_tohex() {
        let result = eval_to_string("toHex(255)").unwrap();
        assert_eq!(result, "FF");
    }

    #[test]
    fn test_tohb() {
        let result = eval_to_string("toHB(1048576)").unwrap();
        assert_eq!(result, "1.00 MB");
    }

    #[test]
    fn test_modulo_and_power() {
        assert_eq!(eval_to_string("2^10").unwrap(), "1024");
        assert_eq!(eval_to_string("pow(3,3)").unwrap(), "27");
    }

    #[test]
    fn test_trig_functions() {
        let sin_pi: f64 = eval_to_string("sin(PI)").unwrap().parse().unwrap();
        assert!(
            (sin_pi - 0.0).abs() < 1e-10,
            "sin(PI) should be ~0, got {}",
            sin_pi
        );
        let cos_pi: f64 = eval_to_string("cos(PI)").unwrap().parse().unwrap();
        assert!(
            (cos_pi + 1.0).abs() < 1e-10,
            "cos(PI) should be ~-1, got {}",
            cos_pi
        );
    }
}
