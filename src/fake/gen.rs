// src/fake/gen.rs
//! `fake gen` — generate rows from types or a saved group.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use crate::config::Config;
use crate::output::{print_error, print_success};
use crate::t;
use anyhow::Result;

use super::format::{self, Fmt, Table};
use super::group;
use super::locale::Locale;
use super::rng::Gen;
use super::types;

/// How many times a row is regenerated before a unique constraint is deemed
/// unsatisfiable (e.g. `bool !` with `-n 5`).
const MAX_UNIQUE_ATTEMPTS: usize = 5_000;

struct Args {
    types: Vec<String>,
    group: Option<String>,
    count: usize,
    format: Option<Fmt>,
    seed: Option<u64>,
    output: Option<PathBuf>,
    locale: Option<Locale>,
    /// `None` = no uniqueness; `Some([])` = whole-row unique; `Some(cols)` =
    /// those named columns unique.
    unique: Option<Vec<String>>,
}

/// Where a column's values come from.
enum Source {
    /// A built-in generator.
    Func(fn(&mut Gen) -> String),
    /// A pool harvested from another group (`ref:group.column`); one value is
    /// picked per row.
    Pool(Vec<String>),
}

struct Column {
    name: String,
    /// Canonical type backing this column (`name`, `email`, `uuid`, or
    /// `ref:group.col`). Drives the producer/consumer generation order.
    type_name: String,
    unique: bool,
    source: Source,
}

/// `fake gen [<types>] [-n N] [-f fmt] [-s seed] [-o file] [-g group]`
/// `         [-l locale] [-u [cols]]`
pub fn run(rest: &str) -> Result<()> {
    let args = match parse(rest) {
        Ok(a) => a,
        Err(e) => {
            print_error(&e);
            return Ok(());
        }
    };

    // Resolve the output format: explicit `-f` wins, else infer from `-o`.
    let mut fmt = args.format;
    if fmt.is_none() {
        if let Some(out) = &args.output {
            if let Some(ext) = out.extension().and_then(|e| e.to_str()) {
                fmt = Fmt::from_ext(ext);
                if fmt.is_none() {
                    print_error(&t!("cmd.fake.gen.unknown_ext", ext));
                    return Ok(());
                }
            }
        }
    }
    let fmt = fmt.unwrap_or(Fmt::Txt);

    if fmt == Fmt::Xlsx && args.output.is_none() {
        print_error(&t!("cmd.fake.gen.xlsx_needs_output"));
        return Ok(());
    }

    let mut gen = match args.seed {
        Some(s) => Gen::seeded(s),
        None => Gen::thread(),
    };
    if let Some(locale) = args.locale {
        gen.set_locale(locale);
    }

    let table = match build_table(&args, &mut gen) {
        Ok(t) => t,
        Err(e) => {
            print_error(&e);
            return Ok(());
        }
    };

    if let Some(path) = &args.output {
        if fmt == Fmt::Xlsx {
            format::write_xlsx(&table, path)?;
        } else if let Some(text) = format::to_text(fmt, &table) {
            std::fs::write(path, text)?;
        }
        print_success(&t!(
            "cmd.fake.gen.wrote",
            table.rows.len(),
            path.display().to_string(),
            fmt.ext()
        ));
    } else if let Some(text) = format::to_text(fmt, &table) {
        print!("{}", text);
    }

    Ok(())
}

fn build_table(args: &Args, gen: &mut Gen) -> std::result::Result<Table, String> {
    if let Some(group) = &args.group {
        let mut resolver = Resolver::new(gen);
        let resolved = resolver.resolve(group, args.count)?;
        Ok(Table {
            name: group.to_lowercase(),
            columns: resolved.aliases.clone(),
            rows: resolved.rows.clone(),
        })
    } else {
        let mut columns: Vec<Column> = Vec::with_capacity(args.types.len());
        for name in &args.types {
            let def =
                types::find(name).ok_or_else(|| t!("cmd.fake.gen.unknown_type", name.clone()))?;
            columns.push(Column {
                name: def.name.to_string(),
                type_name: def.name.to_string(),
                unique: false,
                source: Source::Func(def.gen),
            });
        }

        let unique_all = match &args.unique {
            None => false,
            Some(cols) if cols.is_empty() => true,
            Some(cols) => {
                for want in cols {
                    let key = want.to_lowercase().replace('-', "_");
                    match columns.iter_mut().find(|c| c.name.to_lowercase() == key) {
                        Some(c) => c.unique = true,
                        None => {
                            return Err(t!("cmd.fake.gen.unique_unknown_col", want.clone()));
                        }
                    }
                }
                false
            }
        };

        let rows = generate_rows(&columns, args.count, unique_all, gen)?;
        let names = columns.iter().map(|c| c.name.clone()).collect();
        Ok(Table {
            name: "fake".to_string(),
            columns: names,
            rows,
        })
    }
}

/// A fully generated group: ordered aliases + row-major values.
struct Resolved {
    aliases: Vec<String>,
    rows: Vec<Vec<String>>,
}

/// Resolves `ref:<group>.<column>` references recursively, caching each group's
/// generated rows so a referenced group is only produced once.
struct Resolver<'a> {
    gen: &'a mut Gen,
    cache: HashMap<String, Rc<Resolved>>,
    stack: Vec<String>,
}

impl<'a> Resolver<'a> {
    fn new(gen: &'a mut Gen) -> Self {
        Resolver {
            gen,
            cache: HashMap::new(),
            stack: Vec::new(),
        }
    }

    fn resolve(&mut self, name: &str, count: usize) -> std::result::Result<Rc<Resolved>, String> {
        let key = name.to_lowercase();
        if let Some(hit) = self.cache.get(&key) {
            return Ok(hit.clone());
        }
        if self.stack.iter().any(|s| s == &key) {
            return Err(t!("cmd.fake.gen.ref_cycle", name.to_string()));
        }

        let cols = Config::get_fake_group(name)
            .ok_or_else(|| t!("cmd.fake.group.not_found", name.to_string()))?;
        self.stack.push(key.clone());

        let mut columns: Vec<Column> = Vec::with_capacity(cols.len());
        for c in &cols {
            let source = if let Some((target_group, target_col)) = group::parse_ref(&c.type_name) {
                let target = self.resolve(&target_group, count)?;
                let idx = target
                    .aliases
                    .iter()
                    .position(|a| a.eq_ignore_ascii_case(&target_col))
                    .ok_or_else(|| {
                        t!(
                            "cmd.fake.gen.ref_unknown_col",
                            target_col.clone(),
                            target_group.clone()
                        )
                    })?;
                let pool: Vec<String> = target.rows.iter().map(|r| r[idx].clone()).collect();
                Source::Pool(pool)
            } else {
                let def = types::find(&c.type_name)
                    .ok_or_else(|| t!("cmd.fake.group.unknown_type", c.type_name.clone()))?;
                Source::Func(def.gen)
            };
            columns.push(Column {
                name: c.alias.clone(),
                type_name: c.type_name.clone(),
                unique: c.unique,
                source,
            });
        }

        let rows = generate_rows(&columns, count, false, self.gen)?;
        let aliases: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();

        self.stack.pop();
        let resolved = Rc::new(Resolved { aliases, rows });
        self.cache.insert(key, resolved.clone());
        Ok(resolved)
    }
}

/// Generation priority: name producers run first, then the types that link to
/// them, then everything else.
fn column_role(type_name: &str) -> u8 {
    match type_name {
        "name" | "first_name" | "last_name" => 0,
        "email" | "username" => 1,
        _ => 2,
    }
}

/// Generate `count` rows, retrying a row until every `unique` column (and,
/// when `unique_all`, the whole row) is fresh.
fn generate_rows(
    columns: &[Column],
    count: usize,
    unique_all: bool,
    gen: &mut Gen,
) -> std::result::Result<Vec<Vec<String>>, String> {
    let mut seen: Vec<HashSet<String>> = columns.iter().map(|_| HashSet::new()).collect();
    let mut rows_seen: HashSet<Vec<String>> = HashSet::new();
    let mut rows: Vec<Vec<String>> = Vec::with_capacity(count);

    // Names are produced before `email`/`username` so linked columns agree
    // regardless of the order they were typed; values are still stored back at
    // their original column positions.
    let mut order: Vec<usize> = (0..columns.len()).collect();
    order.sort_by_key(|&i| (column_role(&columns[i].type_name), i));

    for row_index in 0..count {
        let mut attempts = 0usize;
        loop {
            gen.begin_row(row_index);
            let mut row: Vec<String> = vec![String::new(); columns.len()];
            for &ci in &order {
                row[ci] = match &columns[ci].source {
                    Source::Func(f) => f(gen),
                    Source::Pool(pool) => {
                        if pool.is_empty() {
                            String::new()
                        } else {
                            pool[gen.range_usize(0, pool.len() - 1)].clone()
                        }
                    }
                };
            }

            let mut ok = true;
            for (i, c) in columns.iter().enumerate() {
                if c.unique && seen[i].contains(&row[i]) {
                    ok = false;
                    break;
                }
            }
            if ok && unique_all && rows_seen.contains(&row) {
                ok = false;
            }

            if ok {
                for (i, c) in columns.iter().enumerate() {
                    if c.unique {
                        seen[i].insert(row[i].clone());
                    }
                }
                if unique_all {
                    rows_seen.insert(row.clone());
                }
                rows.push(row);
                break;
            }

            attempts += 1;
            if attempts >= MAX_UNIQUE_ATTEMPTS {
                let who = columns
                    .iter()
                    .find(|c| c.unique)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| "row".to_string());
                return Err(t!("cmd.fake.gen.unique_exhausted", who));
            }
        }
    }
    Ok(rows)
}

fn parse(rest: &str) -> Result<Args, String> {
    let mut types: Vec<String> = Vec::new();
    let mut group: Option<String> = None;
    let mut count: usize = 1;
    let mut format: Option<Fmt> = None;
    let mut seed: Option<u64> = None;
    let mut output: Option<PathBuf> = None;
    let mut locale: Option<Locale> = None;
    let mut unique: Option<Vec<String>> = None;

    let tokens = tokenize(rest);
    let mut i = 0;
    while i < tokens.len() {
        let tok = &tokens[i];
        let (flag, inline) = match tok.split_once('=') {
            Some((f, v)) if f.starts_with('-') => (f, Some(v.to_string())),
            _ => (tok.as_str(), None),
        };
        let take = |i: &mut usize| -> Option<String> {
            if let Some(v) = inline.clone() {
                return Some(v);
            }
            *i += 1;
            tokens.get(*i).cloned()
        };
        match flag {
            "-n" | "--count" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.count_usage"))?;
                count = v
                    .parse()
                    .ok()
                    .filter(|n| *n > 0)
                    .ok_or_else(|| t!("cmd.fake.gen.bad_count", v))?;
            }
            "-f" | "--format" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.format_usage"))?;
                format = Some(Fmt::parse(&v).ok_or_else(|| t!("cmd.fake.gen.bad_format", v))?);
            }
            "-s" | "--seed" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.seed_usage"))?;
                seed = Some(v.parse().map_err(|_| t!("cmd.fake.gen.bad_seed", v))?);
            }
            "-o" | "--output" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.output_usage"))?;
                output = Some(PathBuf::from(v));
            }
            "-g" | "--group" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.group_usage"))?;
                group = Some(v);
            }
            "-l" | "--locale" => {
                let v = take(&mut i).ok_or_else(|| t!("cmd.fake.gen.locale_usage"))?;
                locale = Some(Locale::parse(&v).ok_or_else(|| t!("cmd.fake.gen.bad_locale", v))?);
            }
            "-u" | "--unique" => {
                unique = Some(match inline.clone() {
                    Some(v) => split_list(&v),
                    None => match tokens.get(i + 1) {
                        Some(next) if !next.starts_with('-') => {
                            i += 1;
                            split_list(next)
                        }
                        _ => Vec::new(),
                    },
                });
            }
            other if other.starts_with('-') => {
                return Err(t!("cmd.fake.gen.unknown_flag", other));
            }
            other => {
                for part in other.split(',') {
                    let part = part.trim();
                    if !part.is_empty() {
                        types.push(part.to_string());
                    }
                }
            }
        }
        i += 1;
    }

    if group.is_some() && !types.is_empty() {
        return Err(t!("cmd.fake.gen.group_and_types"));
    }
    if group.is_some() && unique.is_some() {
        return Err(t!("cmd.fake.gen.unique_with_group"));
    }
    if group.is_none() && types.is_empty() {
        return Err(t!("cmd.fake.gen.need_input"));
    }

    Ok(Args {
        types,
        group,
        count,
        format,
        seed,
        output,
        locale,
        unique,
    })
}

fn split_list(s: &str) -> Vec<String> {
    s.split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| p.to_string())
        .collect()
}

/// Split on whitespace, honoring single/double quotes.
fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    for ch in s.chars() {
        match quote {
            Some(q) => {
                if ch == q {
                    quote = None;
                } else {
                    cur.push(ch);
                }
            }
            None => match ch {
                '\'' | '"' => quote = Some(ch),
                c if c.is_whitespace() => {
                    if !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            },
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_types_and_flags() {
        let a = parse("name,email -n 3 -f json -s 5 -o out.json").unwrap();
        assert_eq!(a.types, vec!["name", "email"]);
        assert_eq!(a.count, 3);
        assert_eq!(a.format, Some(Fmt::Json));
        assert_eq!(a.seed, Some(5));
        assert_eq!(a.output.unwrap().to_string_lossy(), "out.json");
    }

    #[test]
    fn equals_style_flags() {
        let a = parse("uuid --count=2 --seed=9").unwrap();
        assert_eq!(a.count, 2);
        assert_eq!(a.seed, Some(9));
    }

    #[test]
    fn group_and_types_conflict() {
        assert!(parse("name -g users").is_err());
    }

    #[test]
    fn unique_flag_is_rejected_with_group() {
        assert!(parse("-g users --unique").is_err());
        assert!(parse("-g users -u email").is_err());
    }

    #[test]
    fn parses_unique_columns_and_whole_row() {
        let a = parse("email,name -u email").unwrap();
        assert_eq!(a.unique, Some(vec!["email".to_string()]));

        let a = parse("email,name --unique=email,name").unwrap();
        assert_eq!(
            a.unique,
            Some(vec!["email".to_string(), "name".to_string()])
        );

        // bare `--unique` before a flag → whole row
        let a = parse("email --unique -n 2").unwrap();
        assert_eq!(a.unique, Some(vec![]));

        // bare `-u` at the end → whole row
        let a = parse("email -u").unwrap();
        assert_eq!(a.unique, Some(vec![]));
    }

    #[test]
    fn parses_locale() {
        let a = parse("city -l th_TH").unwrap();
        assert_eq!(a.locale, Some(Locale::ThTh));
        assert!(parse("city --locale=xx_YY").is_err());
    }

    #[test]
    fn requires_some_input() {
        assert!(parse("-n 5").is_err());
    }

    #[test]
    fn quoted_group_name() {
        let a = parse("-g \"my group\"").unwrap();
        assert_eq!(a.group.as_deref(), Some("my group"));
    }

    #[test]
    fn unique_loop_aborts_on_exhaustion() {
        // `bool !` can only hold two values, so three rows must fail.
        let columns = vec![Column {
            name: "flag".into(),
            type_name: "bool".into(),
            unique: true,
            source: Source::Func(types::find("bool").unwrap().gen),
        }];
        let mut g = Gen::seeded(1);
        assert!(generate_rows(&columns, 3, false, &mut g).is_err());
        // ...but two rows are fine.
        let mut g = Gen::seeded(1);
        assert!(generate_rows(&columns, 2, false, &mut g).is_ok());
    }

    #[test]
    fn unique_columns_produce_distinct_values() {
        let columns = vec![Column {
            name: "id".into(),
            type_name: "uuid".into(),
            unique: true,
            source: Source::Func(types::find("uuid").unwrap().gen),
        }];
        let mut g = Gen::seeded(7);
        let rows = generate_rows(&columns, 20, false, &mut g).unwrap();
        let mut set = HashSet::new();
        for r in &rows {
            assert!(set.insert(r[0].clone()), "duplicate {}", r[0]);
        }
    }

    #[test]
    fn whole_row_uniqueness_is_deterministic() {
        let columns = vec![Column {
            name: "a".into(),
            type_name: "float".into(),
            unique: false,
            source: Source::Func(types::find("float").unwrap().gen),
        }];
        let mut x = Gen::seeded(9);
        let mut y = Gen::seeded(9);
        let a = generate_rows(&columns, 5, true, &mut x).unwrap();
        let b = generate_rows(&columns, 5, true, &mut y).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn email_links_to_name_in_same_row() {
        // `email` is listed before `name` on purpose: the engine must still
        // generate the name first so the two columns agree.
        let columns = vec![
            Column {
                name: "email".into(),
                type_name: "email".into(),
                unique: false,
                source: Source::Func(types::find("email").unwrap().gen),
            },
            Column {
                name: "name".into(),
                type_name: "name".into(),
                unique: false,
                source: Source::Func(types::find("name").unwrap().gen),
            },
        ];
        let mut g = Gen::seeded(21);
        let rows = generate_rows(&columns, 10, false, &mut g).unwrap();
        for row in rows {
            let parts: Vec<String> = row[1]
                .to_lowercase()
                .split(' ')
                .map(str::to_string)
                .collect();
            let prefix = format!("{}.{}", parts[0], parts[1]);
            assert!(
                row[0].starts_with(&prefix),
                "name={} email={} prefix={prefix}",
                row[1],
                row[0]
            );
        }
    }

    #[test]
    fn id_column_is_sequential() {
        let columns = vec![Column {
            name: "id".into(),
            type_name: "id".into(),
            unique: false,
            source: Source::Func(types::find("id").unwrap().gen),
        }];
        let mut g = Gen::seeded(1);
        let rows = generate_rows(&columns, 4, false, &mut g).unwrap();
        let got: Vec<&str> = rows.iter().map(|r| r[0].as_str()).collect();
        assert_eq!(got, vec!["1", "2", "3", "4"]);
    }
}
