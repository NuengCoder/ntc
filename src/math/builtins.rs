use std::collections::HashMap;

use anyhow::{bail, Result};
use rand::Rng;

use super::ast::Parser;
use super::eval::EvalResult;
use super::token::tokenize;

// ============================================================================
// Built-in & user-defined function evaluation
// ============================================================================
pub(super) fn eval_function(name: &str, args: &[EvalResult], vars: &mut HashMap<String, f64>) -> Result<EvalResult> {
    let lower = name.to_lowercase();

    if let Ok(result) = eval_builtin(&lower, args) {
        return Ok(result);
    }

    let def = {
        let cfg = crate::config::Config::read_global();
        cfg.math_functions.get(&lower).cloned()
    };
    if let Some(def) = def {
        return eval_user_fn(&def, args, vars);
    }

    bail!("Unknown function: '{}'", name);
}

fn eval_builtin(name: &str, args: &[EvalResult]) -> Result<EvalResult> {
    // print handles both strings and numbers
    if name == "print" {
        let parts: Vec<String> = args.iter().map(|a| match a {
            EvalResult::Num(n) => {
                if n.fract() == 0.0 && n.is_finite() && n.abs() < 1e15 {
                    format!("{}", *n as i128)
                } else {
                    format!("{}", n)
                }
            }
            EvalResult::Str(s) => s.clone(),
        }).collect();
        println!("{}", parts.join(" "));
        return Ok(EvalResult::Num(0.0));
    }

    let nums: Vec<f64> = args.iter().map(|a| {
        match a {
            EvalResult::Num(n) => Ok(*n),
            _ => bail!("Numeric argument expected for '{}'", name),
        }
    }).collect::<Result<_>>()?;

    match name {
        // Trig
        "sin" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].sin())) }
        "cos" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].cos())) }
        "tan" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].tan())) }
        "cot" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(1.0 / nums[0].tan())) }
        "sec" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(1.0 / nums[0].cos())) }
        "csc" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(1.0 / nums[0].sin())) }
        "asin" | "arcsin" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].asin())) }
        "acos" | "arccos" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].acos())) }
        "atan" | "arctan" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].atan())) }
        "acot" | "arccot" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(std::f64::consts::FRAC_PI_2 - nums[0].atan())) }
        "asec" | "arcsec" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num((1.0 / nums[0]).acos())) }
        "acsc" | "arccsc" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num((1.0 / nums[0]).asin())) }

        // Math
        "sqrt" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].sqrt())) }
        "pow" => { check_args(name, 2, &nums)?; Ok(EvalResult::Num(nums[0].powf(nums[1]))) }
        "abs" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].abs())) }
        "floor" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].floor())) }
        "ceil" | "ceiling" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].ceil())) }
        "round" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].round())) }
        "ln" | "log" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].ln())) }
        "log2" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].log2())) }
        "log10" => { check_args(name, 1, &nums)?; Ok(EvalResult::Num(nums[0].log10())) }

        // Aggregate
        "sum" => { if nums.is_empty() { bail!("sum requires at least 1 argument"); }
            Ok(EvalResult::Num(nums.iter().sum())) }
        "min" => { if nums.is_empty() { bail!("min requires at least 1 argument"); }
            Ok(EvalResult::Num(nums.iter().cloned().fold(f64::INFINITY, f64::min))) }
        "max" => { if nums.is_empty() { bail!("max requires at least 1 argument"); }
            Ok(EvalResult::Num(nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max))) }
        "avg" | "average" | "mean" => {
            if nums.is_empty() { bail!("avg requires at least 1 argument"); }
            Ok(EvalResult::Num(nums.iter().sum::<f64>() / nums.len() as f64))
        }

        // Random
        "rand" | "random" => {
            check_args(name, 2, &nums)?;
            let lo = nums[0].ceil() as i64;
            let hi = nums[1].floor() as i64;
            if lo >= hi { bail!("rand requires min < max"); }
            let mut rng = rand::thread_rng();
            Ok(EvalResult::Num(rng.gen_range(lo..=hi) as f64))
        }

        // Conversions (number-based)
        "tobinary" => {
            check_args(name, 1, &nums)?;
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:b}", n)))
        }
        "tohex" => {
            check_args(name, 1, &nums)?;
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:X}", n)))
        }
        "to8" | "tooctal" => {
            check_args(name, 1, &nums)?;
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:o}", n)))
        }
        "todecimal" => {
            check_args(name, 1, &nums)?;
            Ok(EvalResult::Num(nums[0]))
        }
        "tohb" | "tohumanbytes" | "tohumanreadable" => {
            check_args(name, 1, &nums)?;
            Ok(EvalResult::Str(human_readable_bytes(nums[0])))
        }

        _ => bail!("Unknown function: '{}'", name),
    }
}

fn check_args(name: &str, expected: usize, args: &[f64]) -> Result<()> {
    if args.len() != expected {
        bail!("'{}' expects {} argument(s), got {}", name, expected, args.len());
    }
    Ok(())
}

fn human_readable_bytes(bytes: f64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    if bytes < 1.0 { return format!("{:.2} B", bytes); }
    let exp = (bytes.ln() / 1024_f64.ln()).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    let val = bytes / 1024_f64.powi(exp as i32);
    format!("{:.2} {}", val, UNITS[exp])
}

fn eval_user_fn(def: &str, args: &[EvalResult], vars: &mut HashMap<String, f64>) -> Result<EvalResult> {
    let nums: Vec<f64> = args.iter().map(|a| {
        match a {
            EvalResult::Num(n) => Ok(*n),
            _ => bail!("User function requires numeric arguments"),
        }
    }).collect::<Result<_>>()?;

    let arrow = def.find("=>").ok_or_else(|| anyhow::anyhow!("Invalid function definition: '{}'", def))?;
    let params_str = def[..arrow].trim();
    let body_str = def[arrow + 2..].trim();

    let param_names: Vec<&str> = params_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    if nums.len() != param_names.len() {
        bail!("Expected {} arguments, got {}", param_names.len(), nums.len());
    }

    let mut local_vars = vars.clone();
    for (name, val) in param_names.iter().zip(nums.iter()) {
        local_vars.insert(name.to_string(), *val);
    }

    let tokens = tokenize(body_str)?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;
    super::eval::eval(&expr, &mut local_vars)
}

/// Return the list of built-in function names for the given prefix.
pub(crate) fn builtin_function_names() -> Vec<&'static str> {
    vec![
        "sin", "cos", "tan", "cot", "sec", "csc",
        "asin", "acos", "atan", "acot", "asec", "acsc",
        "arcsin", "arccos", "arctan", "arccot", "arcsec", "arccsc",
        "sqrt", "pow", "abs", "floor", "ceil", "ceiling", "round",
        "ln", "log", "log2", "log10",
        "sum", "min", "max", "avg", "average", "mean",
        "rand", "random",
        "print",
        "tobinary", "tohex", "to8", "tooctal", "todecimal",
        "tohb", "tohumanbytes", "tohumanreadable",
    ]
}

/// Return the list of constant names.
pub(crate) fn constant_names() -> Vec<&'static str> {
    vec!["PI", "pi", "E", "e", "EXP", "exp", "PHI", "phi", "TAU", "tau"]
}

pub(super) const RESERVED: &[&str] = &[
    "sin", "cos", "tan", "cot", "sec", "csc",
    "asin", "acos", "atan", "acot", "asec", "acsc",
    "arcsin", "arccos", "arctan", "arccot", "arcsec", "arccsc",
    "sqrt", "pow", "abs", "floor", "ceil", "ceiling", "round",
    "ln", "log", "log2", "log10",
    "sum", "min", "max", "avg", "average", "mean",
    "rand", "random",
    "tobinary", "tohex", "to8", "tooctal", "todecimal",
    "tohb", "tohumanbytes", "tohumanreadable",
    "pi", "e", "exp", "phi", "tau",
    "timer", "return",
];
