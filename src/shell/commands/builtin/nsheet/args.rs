use crate::t;

pub(super) struct NsheetArgs {
    pub(super) file: String,
    pub(super) delimiter: Option<char>,
    pub(super) header: Option<bool>,
    pub(super) limit: Option<usize>,
    pub(super) all: bool,
    pub(super) sort_col: Option<String>,
    pub(super) sort_desc: bool,
    pub(super) filter: Option<String>,
    pub(super) export: Option<String>,
    pub(super) output: Option<String>,
}

pub(super) fn parse_delimiter_value(val: &str) -> Result<Option<char>, String> {
    let low = val.to_lowercase();
    match low.as_str() {
        "auto" => Ok(None),
        "tab" | "\\t" => Ok(Some('\t')),
        "comma" => Ok(Some(',')),
        "semicolon" => Ok(Some(';')),
        "pipe" => Ok(Some('|')),
        _ => {
            let mut chars = val.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => Ok(Some(c)),
                _ => Err(t!("cmd.nsheet.invalid_delimiter", val)),
            }
        }
    }
}

pub(super) fn parse_nsheet_args(tokens: &[String]) -> Result<NsheetArgs, String> {
    let mut args = NsheetArgs {
        file: String::new(),
        delimiter: None,
        header: None,
        limit: None,
        all: false,
        sort_col: None,
        sort_desc: false,
        filter: None,
        export: None,
        output: None,
    };
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "-d" | "--delimiter" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.delimiter_usage"))?;
                args.delimiter = parse_delimiter_value(val)?;
                i += 2;
            }
            "--header" => {
                args.header = Some(true);
                i += 1;
            }
            "--no-header" => {
                args.header = Some(false);
                i += 1;
            }
            "-l" | "--limit" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.limit_usage"))?;
                match val.parse::<usize>() {
                    Ok(n) => args.limit = Some(n),
                    Err(_) => return Err(t!("cmd.nsheet.invalid_limit", val)),
                }
                i += 2;
            }
            "--all" => {
                args.all = true;
                i += 1;
            }
            "--sort" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.sort_usage"))?;
                args.sort_col = Some(val.clone());
                i += 2;
            }
            "--desc" => {
                args.sort_desc = true;
                i += 1;
            }
            "--filter" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.filter_usage"))?;
                args.filter = Some(val.clone());
                i += 2;
            }
            "--export" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.export_usage"))?;
                let low = val.to_lowercase();
                if low != "json" && low != "md" {
                    return Err(t!("cmd.nsheet.invalid_export", val));
                }
                args.export = Some(low);
                i += 2;
            }
            "-o" | "--output" => {
                let val = tokens
                    .get(i + 1)
                    .ok_or_else(|| t!("cmd.nsheet.output_usage"))?;
                args.output = Some(val.clone());
                i += 2;
            }
            _ => {
                let tok = &tokens[i];
                if tok.starts_with('-') {
                    return Err(t!("cmd.nsheet.unknown_option", tok));
                }
                if !args.file.is_empty() {
                    return Err(t!("cmd.nsheet.unknown_option", tok));
                }
                args.file = tok.clone();
                i += 1;
            }
        }
    }
    if args.file.is_empty() {
        return Err(t!("cmd.nsheet.missing_file"));
    }
    Ok(args)
}
