use anyhow::{bail, Result};
use colored::Colorize;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

// ============================================================================
// Result type for expression evaluation
// ============================================================================
#[derive(Clone)]
enum EvalResult {
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
// Tokens
// ============================================================================
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Num(f64),
    Str(String),
    Ident(String),
    Op(char),
    LParen,
    RParen,
    Comma,
    Equal,
    ReturnTok,
    Eof,
}

// ============================================================================
// AST nodes
// ============================================================================
#[derive(Clone)]
enum Expr {
    Num(f64),
    Str(String),
    Ident(String),
    Binary(Box<Expr>, char, Box<Expr>),
    Unary(Box<Expr>),
    Call(String, Vec<Expr>),
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
}

// ============================================================================
// Tokenizer — two entry points
// ============================================================================

/// Tokenize without position tracking (used by CLI).
fn tokenize(input: &str) -> Result<Vec<Token>> {
    let (tokens, _) = tokenize_with_pos(input)?;
    Ok(tokens)
}

/// Tokenize with byte-offset tracking (used by LSP).
pub(crate) fn tokenize_with_pos(input: &str) -> Result<(Vec<Token>, Vec<usize>)> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut positions = Vec::new();
    let mut i = 0;
    let mut byte_offset = 0;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '\u{feff}' { i += 1; byte_offset += 3; continue; }

        if ch.is_whitespace() {
            i += 1;
            byte_offset += 1;
            continue;
        }

        if ch == '#' {
            tokens.push(Token::Eof);
            positions.push(byte_offset);
            return Ok((tokens, positions));
        }
        if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            tokens.push(Token::Eof);
            positions.push(byte_offset);
            return Ok((tokens, positions));
        }

        // String literals
        if ch == '"' {
            let str_offset = byte_offset;
            i += 1;
            byte_offset += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1; byte_offset += 1; // skip escape char
                }
                i += 1;
                byte_offset += 1;
            }
            let raw: String = chars[start..i].iter().collect();
            // Process escapes
            let mut escaped = String::with_capacity(raw.len());
            let mut esc = raw.chars();
            while let Some(c) = esc.next() {
                if c == '\\' {
                    match esc.next() {
                        Some('n') => escaped.push('\n'),
                        Some('t') => escaped.push('\t'),
                        Some('r') => escaped.push('\r'),
                        Some('\\') => escaped.push('\\'),
                        Some('"') => escaped.push('"'),
                        Some(c) => { escaped.push('\\'); escaped.push(c); }
                        None => escaped.push('\\'),
                    }
                } else {
                    escaped.push(c);
                }
            }
            if i >= chars.len() {
                return Err(anyhow::anyhow!("Unterminated string literal"));
            }
            i += 1; // skip closing "
            byte_offset += 1;
            tokens.push(Token::Str(escaped));
            positions.push(str_offset);
            continue;
        }

        if "+-*/^".contains(ch) {
            tokens.push(Token::Op(ch));
            positions.push(byte_offset);
            i += 1;
            byte_offset += 1;
            continue;
        }

        if ch == '(' { tokens.push(Token::LParen); positions.push(byte_offset); i += 1; byte_offset += 1; continue; }
        if ch == ')' { tokens.push(Token::RParen); positions.push(byte_offset); i += 1; byte_offset += 1; continue; }
        if ch == ',' { tokens.push(Token::Comma); positions.push(byte_offset); i += 1; byte_offset += 1; continue; }
        if ch == '=' { tokens.push(Token::Equal); positions.push(byte_offset); i += 1; byte_offset += 1; continue; }

        if ch.is_ascii_digit() || (ch == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let num_offset = byte_offset;
            if ch == '0' && i + 1 < chars.len() {
                let next = chars[i + 1];
                if next == 'x' || next == 'X' {
                    i += 2; byte_offset += 2;
                    let start = i;
                    while i < chars.len() && chars[i].is_ascii_hexdigit() { i += 1; byte_offset += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 16)
                        .map_err(|_| anyhow::anyhow!("Invalid hex number: 0x{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                } else if next == 'b' || next == 'B' {
                    i += 2; byte_offset += 2;
                    let start = i;
                    while i < chars.len() && (chars[i] == '0' || chars[i] == '1') { i += 1; byte_offset += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 2)
                        .map_err(|_| anyhow::anyhow!("Invalid binary number: 0b{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                } else if next == 'o' || next == 'O' {
                    i += 2; byte_offset += 2;
                    let start = i;
                    while i < chars.len() && chars[i] >= '0' && chars[i] <= '7' { i += 1; byte_offset += 1; }
                    let s: String = chars[start..i].iter().collect();
                    let val = i64::from_str_radix(&s, 8)
                        .map_err(|_| anyhow::anyhow!("Invalid octal number: 0o{}", s))?;
                    tokens.push(Token::Num(val as f64));
                    positions.push(num_offset);
                    continue;
                }
            }
            let start = i;
            if ch == '.' { i += 1; byte_offset += 1; }
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                if chars[i] == '.' && i + 1 < chars.len() && chars[i + 1] == '.' { break; }
                i += 1;
                byte_offset += 1;
            }
            let s: String = chars[start..i].iter().collect();
            let val: f64 = s.parse().map_err(|_| anyhow::anyhow!("Invalid number: {}", s))?;
            tokens.push(Token::Num(val));
            positions.push(num_offset);
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let ident_offset = byte_offset;
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') { i += 1; byte_offset += 1; }
            let s: String = chars[start..i].iter().collect();
            positions.push(ident_offset);
            match s.as_str() {
                "return" => tokens.push(Token::ReturnTok),
                _ => tokens.push(Token::Ident(s)),
            }
            continue;
        }

        return Err(anyhow::anyhow!("Unexpected character '{}' at byte {}", ch, byte_offset));
    }

    tokens.push(Token::Eof);
    positions.push(byte_offset);
    Ok((tokens, positions))
}

// ============================================================================
// Syntax validation (used by LSP diagnostics)
// ============================================================================

/// Validate a math expression, returning `Ok(())` or `Err((message, byte_offset))`.
pub(crate) fn validate(input: &str) -> Result<(), (String, usize)> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return Ok(());
    }
    let trimmed = if trimmed.starts_with("//") { "" } else { trimmed };
    if trimmed.is_empty() {
        return Ok(());
    }
    let (tokens, positions) = tokenize_with_pos(trimmed)
        .map_err(|e| {
            let msg = e.to_string();
            let offset = if let Some(pos) = msg.rfind("at byte ") {
                msg[pos + 8..].parse::<usize>().unwrap_or(0)
            } else {
                0
            };
            (msg, offset)
        })?;
    let mut parser = Parser::with_positions(tokens, positions);
    parser.parse_statement().map_err(|e| {
        let offset = parser.current_offset();
        (e.to_string(), offset)
    })?;
    Ok(())
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

// ============================================================================
// Parser
// ============================================================================
struct Parser {
    tokens: Vec<Token>,
    positions: Vec<usize>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let len = tokens.len();
        Self { tokens, positions: vec![0; len], pos: 0 }
    }

    fn with_positions(tokens: Vec<Token>, positions: Vec<usize>) -> Self {
        Self { tokens, positions, pos: 0 }
    }

    fn peek(&self) -> &Token { &self.tokens[self.pos] }
    fn advance(&mut self) -> Token { let t = self.tokens[self.pos].clone(); self.pos += 1; t }
    fn current_offset(&self) -> usize { self.positions.get(self.pos).copied().unwrap_or(0) }

    fn expect(&mut self, tok: &Token) -> Result<()> {
        let actual = self.advance();
        if actual != *tok {
            bail!("Expected {:?}, got {:?}", tok, actual);
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<Expr> {
        match self.peek() {
            Token::ReturnTok => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Return(Box::new(expr)))
            }
            Token::Ident(_) => {
                let saved = self.pos;
                let name = if let Token::Ident(s) = self.advance() { s } else { unreachable!() };
                if matches!(self.peek(), Token::Equal) {
                    self.advance();
                    let val = self.parse_expr()?;
                    Ok(Expr::Assign(name, Box::new(val)))
                } else {
                    self.pos = saved;
                    self.parse_expr()
                }
            }
            _ => self.parse_expr(),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Token::Op('+') | Token::Op('-')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let right = self.parse_term()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Op('*') | Token::Op('/')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let right = self.parse_unary()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if matches!(self.peek(), Token::Op('-') | Token::Op('+')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let expr = self.parse_power()?;
            if op == '-' { return Ok(Expr::Unary(Box::new(expr))); }
            return Ok(expr);
        }
        self.parse_power()
    }

    fn parse_power(&mut self) -> Result<Expr> {
        let mut base = self.parse_call()?;
        if matches!(self.peek(), Token::Op('^')) {
            self.advance();
            let exp = self.parse_unary()?;
            base = Expr::Binary(Box::new(base), '^', Box::new(exp));
        }
        Ok(base)
    }

    fn parse_call(&mut self) -> Result<Expr> {
        let tok = self.peek().clone();
        match tok {
            Token::Ident(name) => {
                self.advance();
                match self.peek() {
                    Token::LParen => {
                        self.advance();
                        let mut args = Vec::new();
                        if !matches!(self.peek(), Token::RParen) {
                            args.push(self.parse_expr()?);
                            while matches!(self.peek(), Token::Comma) {
                                self.advance();
                                args.push(self.parse_expr()?);
                            }
                        }
                        self.expect(&Token::RParen)?;
                        Ok(Expr::Call(name, args))
                    }
                    Token::Equal => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Expr::Assign(name, Box::new(val)))
                    }
                    _ => Ok(Expr::Ident(name)),
                }
            }
            Token::Num(val) => { self.advance(); Ok(Expr::Num(val)) }
            Token::Str(s) => { self.advance(); Ok(Expr::Str(s)) }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            _ => bail!("Unexpected token: {:?}", self.peek()),
        }
    }
}

// ============================================================================
// Evaluator
// ============================================================================
fn eval(expr: &Expr, vars: &mut HashMap<String, f64>) -> Result<EvalResult> {
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

fn get_constant(name: &str) -> Option<f64> {
    match name {
        "PI" | "pi" => Some(std::f64::consts::PI),
        "E" | "e" => Some(std::f64::consts::E),
        "EXP" | "exp" => Some(std::f64::consts::E),
        "PHI" | "phi" => Some(1.618033988749894848204586834365638117720309179805762862135448),
        "TAU" | "tau" => Some(std::f64::consts::TAU),
        _ => None,
    }
}

// ============================================================================
// Built-in & user-defined function evaluation
// ============================================================================
fn eval_function(name: &str, args: &[EvalResult], vars: &mut HashMap<String, f64>) -> Result<EvalResult> {
    let lower = name.to_lowercase();

    if let Ok(result) = eval_builtin(&lower, args) {
        return Ok(result);
    }

    let def = {
        let config = crate::config::Config::global();
        let cfg = config.read().unwrap();
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
        "sin" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].sin())) }
        "cos" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].cos())) }
        "tan" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].tan())) }
        "cot" => { check_args(name, 1, &nums); Ok(EvalResult::Num(1.0 / nums[0].tan())) }
        "sec" => { check_args(name, 1, &nums); Ok(EvalResult::Num(1.0 / nums[0].cos())) }
        "csc" => { check_args(name, 1, &nums); Ok(EvalResult::Num(1.0 / nums[0].sin())) }
        "asin" | "arcsin" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].asin())) }
        "acos" | "arccos" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].acos())) }
        "atan" | "arctan" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].atan())) }
        "acot" | "arccot" => { check_args(name, 1, &nums); Ok(EvalResult::Num(std::f64::consts::FRAC_PI_2 - nums[0].atan())) }
        "asec" | "arcsec" => { check_args(name, 1, &nums); Ok(EvalResult::Num((1.0 / nums[0]).acos())) }
        "acsc" | "arccsc" => { check_args(name, 1, &nums); Ok(EvalResult::Num((1.0 / nums[0]).asin())) }

        // Math
        "sqrt" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].sqrt())) }
        "pow" => { check_args(name, 2, &nums); Ok(EvalResult::Num(nums[0].powf(nums[1]))) }
        "abs" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].abs())) }
        "floor" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].floor())) }
        "ceil" | "ceiling" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].ceil())) }
        "round" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].round())) }
        "ln" | "log" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].ln())) }
        "log2" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].log2())) }
        "log10" => { check_args(name, 1, &nums); Ok(EvalResult::Num(nums[0].log10())) }

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
            check_args(name, 2, &nums);
            let lo = nums[0].ceil() as i64;
            let hi = nums[1].floor() as i64;
            if lo >= hi { bail!("rand requires min < max"); }
            let mut rng = rand::thread_rng();
            Ok(EvalResult::Num(rng.gen_range(lo..=hi) as f64))
        }

        // Conversions (number-based)
        "tobinary" => {
            check_args(name, 1, &nums);
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:b}", n)))
        }
        "tohex" => {
            check_args(name, 1, &nums);
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:X}", n)))
        }
        "to8" | "tooctal" => {
            check_args(name, 1, &nums);
            let n = nums[0] as i64;
            Ok(EvalResult::Str(format!("{:o}", n)))
        }
        "todecimal" => {
            check_args(name, 1, &nums);
            Ok(EvalResult::Num(nums[0]))
        }
        "tohb" | "tohumanbytes" | "tohumanreadable" => {
            check_args(name, 1, &nums);
            Ok(EvalResult::Str(human_readable_bytes(nums[0])))
        }

        _ => bail!("Unknown function: '{}'", name),
    }
}

fn check_args(_name: &str, expected: usize, args: &[f64]) {
    if args.len() != expected {
        // non-fatal, will result in error propagation
    }
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
    eval(&expr, &mut local_vars)
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

// ============================================================================
// Main entry point
// ============================================================================
pub fn run(input: &str) -> Result<()> {
    let input = input.trim();

    if input.is_empty() {
        print_help();
        return Ok(());
    }

    if input.starts_with("timer") {
        return handle_timer(input[5..].trim());
    }

    if input.starts_with("fun ") || input == "fun" {
        return handle_fun(&input[3..].trim());
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
            // Assignment in file mode: store but don't print
            Ok(None)
        }
        Expr::Return(_) => {
            let result = eval(&stmt, vars)?;
            Ok(Some(result))
        }
        Expr::Call(name, _) if name == "print" => {
            let _result = eval(&stmt, vars)?;
            // print handles its own output; don't print the return value
            Ok(None)
        }
        _ => {
            let result = eval(&stmt, vars)?;
            Ok(Some(result))
        }
    }
}

// ============================================================================
// Function management (math fun ...)
// ============================================================================
fn handle_fun(args: &str) -> Result<()> {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    let subcmd = parts[0].to_lowercase();
    let subargs = parts.get(1).unwrap_or(&"").trim();

    match subcmd.as_str() {
        "add" | "define" => fun_add(subargs),
        "rm" | "remove" | "del" | "delete" => fun_rm(subargs),
        "edit" | "update" => fun_edit(subargs),
        "info" | "show" => fun_info(subargs),
        "ls" | "list" => fun_ls(),
        _ => {
            println!("{}", "math fun commands:".cyan().bold());
            println!("  math fun add <name>(<params>) = <body>    Define a function");
            println!("  math fun rm <name>                       Remove a function");
            println!("  math fun edit <name> = <body>            Update a function");
            println!("  math fun info <name>                     Show function details");
            println!("  math fun ls                              List all functions");
            println!();
            println!("{}", "Examples:".green());
            println!("  math fun add square(x) = x^2");
            println!("  math fun add add3(a,b,c) = a + b + c");
            println!("  math fun rm square");
            Ok(())
        }
    }
}

fn fun_add(args: &str) -> Result<()> {
    let (name, params, body) = parse_function_def(args)?;

    if name.starts_with("fun") || RESERVED.contains(&name.as_str()) {
        bail!("'{}' is a reserved name", name);
    }

    let def = format!("{} => {}", params.join(","), body);
    crate::config::Config::global_add_math_fn(&name, &def);
    crate::config::Config::reload_global();
    println!("  Defined: {}({}) = {}", name.green(), params.join(", ").cyan(), body.yellow());
    Ok(())
}

fn fun_rm(args: &str) -> Result<()> {
    let name = args.split_whitespace().next().ok_or_else(|| anyhow::anyhow!("Usage: math fun rm <name>"))?;
    crate::config::Config::global_remove_math_fn(name);
    crate::config::Config::reload_global();
    println!("  Removed: {}", name.green());
    Ok(())
}

fn fun_edit(args: &str) -> Result<()> {
    let (name, params, body) = parse_function_def(args)?;
    let def = format!("{} => {}", params.join(","), body);

    let config = crate::config::Config::global();
    let cfg = config.read().unwrap();
    if !cfg.math_functions.contains_key(&name.to_lowercase()) {
        bail!("Function '{}' not found", name);
    }
    drop(cfg);

    crate::config::Config::global_update_math_fn(&name, &def);
    crate::config::Config::reload_global();
    println!("  Updated: {}({}) = {}", name.green(), params.join(", ").cyan(), body.yellow());
    Ok(())
}

fn fun_info(args: &str) -> Result<()> {
    let name = args.split_whitespace().next().ok_or_else(|| anyhow::anyhow!("Usage: math fun info <name>"))?;
    let lower = name.to_lowercase();

    let config = crate::config::Config::global();
    let cfg = config.read().unwrap();
    match cfg.math_functions.get(&lower) {
        Some(def) => {
            let arrow = def.find("=>").unwrap();
            let params = def[..arrow].trim();
            let body = def[arrow + 2..].trim();
            println!("  Name:   {}", name.green().bold());
            println!("  Params: {}", params.cyan());
            println!("  Body:   {}", body.yellow());
        }
        None => {
            bail!("Function '{}' not found", name);
        }
    }
    Ok(())
}

fn fun_ls() -> Result<()> {
    let config = crate::config::Config::global();
    let cfg = config.read().unwrap();
    let fns = &cfg.math_functions;

    if fns.is_empty() {
        println!("  No user-defined functions. Use 'math fun add <name>(<params>) = <body>'");
        return Ok(());
    }

    println!("{}", "User-defined math functions:".cyan().bold());
    let mut sorted: Vec<(&String, &String)> = fns.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    for (name, def) in &sorted {
        let arrow = def.find("=>").unwrap_or(0);
        let params = def[..arrow].trim();
        let body = def[arrow + 2..].trim();
        println!("  {}({}) = {}", name.green(), params.cyan(), body.yellow());
    }
    Ok(())
}

const RESERVED: &[&str] = &[
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

fn parse_function_def(input: &str) -> Result<(String, Vec<String>, String)> {
    let input = input.trim();

    let eq_pos = input.find('=').ok_or_else(|| anyhow::anyhow!("Missing '=' in function definition"))?;
    let lhs = input[..eq_pos].trim();
    let body = input[eq_pos + 1..].trim();

    if body.is_empty() {
        bail!("Function body cannot be empty");
    }

    let paren_pos = lhs.find('(');
    let (name, params_str) = match paren_pos {
        Some(p) => {
            if !lhs.ends_with(')') {
                bail!("Missing closing parenthesis in function definition");
            }
            let n = lhs[..p].trim();
            let p_str = lhs[p + 1..lhs.len() - 1].trim();
            (n, p_str)
        }
        None => {
            let n = lhs;
            (n, "")
        }
    };

    if name.is_empty() {
        bail!("Function name cannot be empty");
    }

    let params: Vec<String> = if params_str.is_empty() {
        Vec::new()
    } else {
        params_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };

    Ok((name.to_string(), params, body.to_string()))
}
