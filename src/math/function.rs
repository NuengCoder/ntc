use anyhow::{bail, Result};
use colored::Colorize;

use super::builtins::RESERVED;

// ============================================================================
// Function management (math fun ...)
// ============================================================================
pub(super) fn handle_fun(args: &str) -> Result<()> {
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

    let cfg = crate::config::Config::read_global();
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

    let cfg = crate::config::Config::read_global();
    match cfg.math_functions.get(&lower) {
        Some(def) => {
            let (params, body) = if let Some(arrow) = def.find("=>") {
                (def[..arrow].trim().to_string(), def[arrow + 2..].trim().to_string())
            } else {
                ("?".to_string(), def.clone())
            };
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
    let cfg = crate::config::Config::read_global();
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
        let body = if arrow + 2 <= def.len() { def[arrow + 2..].trim() } else { "" };
        println!("  {}({}) = {}", name.green(), params.cyan(), body.yellow());
    }
    Ok(())
}

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
