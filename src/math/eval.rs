use std::collections::HashMap;
use std::fmt;

use anyhow::{bail, Result};

use super::ast::Expr;
use super::builtins::eval_function;

// ============================================================================
// Result type for expression evaluation
// ============================================================================
#[derive(Clone)]
pub(super) enum EvalResult {
    Num(f64),
    Str(String),
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalResult::Num(n) => {
                if n.is_infinite() {
                    write!(f, "Infinity")
                } else if n.is_nan() {
                    write!(f, "NaN")
                } else if n.fract() == 0.0 && n.is_finite() && n.abs() < 1e15 {
                    write!(f, "{}", *n as i128)
                } else {
                    write!(f, "{}", n)
                }
            }
            EvalResult::Str(s) => write!(f, "{}", s),
        }
    }
}

// ============================================================================
// Evaluator
// ============================================================================
pub(super) fn eval(expr: &Expr, vars: &mut HashMap<String, f64>) -> Result<EvalResult> {
    match expr {
        Expr::Num(n) => Ok(EvalResult::Num(*n)),
        Expr::Str(s) => Ok(EvalResult::Str(s.clone())),
        Expr::Ident(name) => {
            if let Some(&val) = vars.get(name.as_str()) {
                return Ok(EvalResult::Num(val));
            }
            let val = get_constant(name);
            if let Some(v) = val {
                return Ok(EvalResult::Num(v));
            }
            bail!("Undefined identifier: '{}'", name);
        }
        Expr::Binary(left, op, right) => {
            let l = eval(left, vars)?;
            let r = eval(right, vars)?;
            let lv = match l { EvalResult::Num(n) => n, _ => bail!("Binary op requires numbers") };
            let rv = match r { EvalResult::Num(n) => n, _ => bail!("Binary op requires numbers") };
            let result = match op {
                '+' => lv + rv,
                '-' => lv - rv,
                '*' => lv * rv,
                '/' => {
                    if rv == 0.0 { bail!("Division by zero"); }
                    lv / rv
                }
                '^' => lv.powf(rv),
                _ => bail!("Unknown operator: {}", op),
            };
            Ok(EvalResult::Num(result))
        }
        Expr::Unary(expr) => {
            let v = eval(expr, vars)?;
            match v {
                EvalResult::Num(n) => Ok(EvalResult::Num(-n)),
                _ => bail!("Unary minus requires number"),
            }
        }
        Expr::Call(name, args) => {
            let evaluated_args: Vec<EvalResult> = args.iter().map(|a| eval(a, vars)).collect::<Result<_>>()?;
            eval_function(name, &evaluated_args, vars)
        }
        Expr::Assign(name, expr) => {
            let val = eval(expr, vars)?;
            match val {
                EvalResult::Num(n) => {
                    vars.insert(name.clone(), n);
                    Ok(EvalResult::Num(n))
                }
                EvalResult::Str(_) => {
                    bail!("Cannot assign string to variable '{}'", name);
                }
            }
        }
        Expr::Return(expr) => {
            let val = eval(expr, vars)?;
            Ok(val)
        }
    }
}

pub(super) fn get_constant(name: &str) -> Option<f64> {
    match name {
        "PI" | "pi" => Some(std::f64::consts::PI),
        "E" | "e" => Some(std::f64::consts::E),
        "PHI" | "phi" => Some(1.618_033_988_749_895),
        "TAU" | "tau" => Some(std::f64::consts::TAU),
        _ => None,
    }
}
