// src/fake/types.rs
//! The `fake` type registry: every supported type maps to a generator.

use chrono::NaiveDate;
use colored::*;

use crate::output::print_info;
use crate::t;

use super::data::*;
use super::locale::{Locale, NamePair};
use super::rng::Gen;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Person,
    Contact,
    Address,
    Geo,
    DateTime,
    Numeric,
    Ident,
    Network,
    Web,
    Finance,
    Company,
    Text,
    Security,
    File,
    Misc,
}

impl Category {
    pub fn label(self) -> &'static str {
        match self {
            Category::Person => "person",
            Category::Contact => "contact",
            Category::Address => "address",
            Category::Geo => "geo",
            Category::DateTime => "date/time",
            Category::Numeric => "numeric",
            Category::Ident => "identifier",
            Category::Network => "network",
            Category::Web => "web",
            Category::Finance => "finance",
            Category::Company => "company",
            Category::Text => "text",
            Category::Security => "security",
            Category::File => "file",
            Category::Misc => "misc",
        }
    }
}

pub struct TypeDef {
    pub name: &'static str,
    pub category: Category,
    pub about: &'static str,
    pub gen: fn(&mut Gen) -> String,
}

/// Every type accepted by `fake gen <types>` / `fake group add`.
pub static TYPES: &[TypeDef] = &[
    // ---- person ----
    td("name", Category::Person, "full name", g_name),
    td("first_name", Category::Person, "given name", g_first_name),
    td("last_name", Category::Person, "family name", g_last_name),
    td(
        "username",
        Category::Person,
        "handle / login name",
        g_username,
    ),
    td("nickname", Category::Person, "playful handle", g_nickname),
    td("title", Category::Person, "honorific (Mr/Ms/Dr/…)", g_title),
    td("gender", Category::Person, "gender identity", g_gender),
    td("pronoun", Category::Person, "personal pronouns", g_pronoun),
    // ---- contact ----
    td("email", Category::Contact, "email address", g_email),
    td(
        "phone_number",
        Category::Contact,
        "phone number",
        g_phone_number,
    ),
    td(
        "phone_country_code",
        Category::Contact,
        "phone country code (+1, +44, …)",
        g_phone_country_code,
    ),
    // ---- address ----
    td(
        "address",
        Category::Address,
        "full postal address",
        g_address,
    ),
    td("street", Category::Address, "street line", g_street),
    td("city", Category::Address, "city name", g_city),
    td("state", Category::Address, "state / province", g_state),
    td("country", Category::Address, "country name", g_country),
    td(
        "postal_code",
        Category::Address,
        "postal / ZIP code",
        g_postal_code,
    ),
    // ---- geo ----
    td("latitude", Category::Geo, "decimal latitude", g_latitude),
    td("longitude", Category::Geo, "decimal longitude", g_longitude),
    td("timezone", Category::Geo, "IANA timezone", g_timezone),
    // ---- date/time ----
    td("age", Category::DateTime, "age in years", g_age),
    td(
        "born_date",
        Category::DateTime,
        "birth date (YYYY-MM-DD)",
        g_born_date,
    ),
    td("joined", Category::DateTime, "join / signup date", g_joined),
    td("date", Category::DateTime, "calendar date", g_date),
    td("datetime", Category::DateTime, "date + time", g_datetime),
    td("time", Category::DateTime, "clock time", g_time),
    td("year", Category::DateTime, "4-digit year", g_year),
    td("month", Category::DateTime, "month number (1-12)", g_month),
    td("day", Category::DateTime, "day of month (1-31)", g_day),
    td(
        "unix_timestamp",
        Category::DateTime,
        "Unix epoch seconds",
        g_unix_timestamp,
    ),
    // ---- numeric ----
    td("float", Category::Numeric, "signed decimal", g_float),
    td("bool", Category::Numeric, "true / false", g_bool),
    // ---- identifier ----
    td("uuid", Category::Ident, "UUID v4", g_uuid),
    td("ulid", Category::Ident, "ULID", g_ulid),
    td("nanoid", Category::Ident, "NanoID (21 chars)", g_nanoid),
    td("id", Category::Ident, "sequential row id", g_id),
    // ---- web / network ----
    td("url", Category::Web, "https URL", g_url),
    td("domain", Category::Web, "domain name", g_domain),
    td("slug", Category::Web, "url slug", g_slug),
    td("ipv4", Category::Network, "IPv4 address", g_ipv4),
    td("ipv6", Category::Network, "IPv6 address", g_ipv6),
    td(
        "mac_address",
        Category::Network,
        "MAC address",
        g_mac_address,
    ),
    td("port", Category::Network, "TCP port", g_port),
    td(
        "user_agent",
        Category::Network,
        "HTTP User-Agent",
        g_user_agent,
    ),
    td(
        "http_method",
        Category::Network,
        "HTTP method",
        g_http_method,
    ),
    td(
        "status_code",
        Category::Network,
        "HTTP status code",
        g_status_code,
    ),
    // ---- finance ----
    td(
        "credit_card",
        Category::Finance,
        "card number (Luhn-valid)",
        g_credit_card,
    ),
    td(
        "identity_card",
        Category::Finance,
        "national ID number",
        g_identity_card,
    ),
    td(
        "id_card",
        Category::Finance,
        "national ID (locale-aware)",
        g_id_card,
    ),
    td("cvv", Category::Finance, "card security code", g_cvv),
    td("iban", Category::Finance, "IBAN (checksum-valid)", g_iban),
    td(
        "currency",
        Category::Finance,
        "ISO currency code",
        g_currency,
    ),
    td("amount", Category::Finance, "decimal amount", g_amount),
    td("price", Category::Finance, "decimal price", g_price),
    td(
        "btc_address",
        Category::Finance,
        "Bitcoin address",
        g_btc_address,
    ),
    td(
        "eth_address",
        Category::Finance,
        "Ethereum address",
        g_eth_address,
    ),
    // ---- company ----
    td("company", Category::Company, "company name", g_company),
    td("job_title", Category::Company, "job title", g_job_title),
    td("department", Category::Company, "department", g_department),
    td("industry", Category::Company, "industry", g_industry),
    td("salary", Category::Company, "annual salary", g_salary),
    // ---- text ----
    td("word", Category::Text, "single word", g_word),
    td("sentence", Category::Text, "sentence", g_sentence),
    td("paragraph", Category::Text, "paragraph", g_paragraph),
    td("lorem", Category::Text, "lorem ipsum snippet", g_lorem),
    td("quote", Category::Text, "famous quote", g_quote),
    td("hashtag", Category::Text, "hashtag", g_hashtag),
    // ---- security ----
    td(
        "password",
        Category::Security,
        "random password",
        g_password,
    ),
    td(
        "password_ez",
        Category::Security,
        "easy password (lowercase + digits)",
        g_password_ez,
    ),
    td(
        "password_mid",
        Category::Security,
        "medium password (mixed case + digits)",
        g_password_mid,
    ),
    td(
        "password_hard",
        Category::Security,
        "hard password (mixed case + symbols)",
        g_password_hard,
    ),
    td(
        "passphrase",
        Category::Security,
        "diceware-style passphrase",
        g_passphrase,
    ),
    td("otp4", Category::Security, "4-digit OTP", g_otp4),
    td("otp6", Category::Security, "6-digit OTP", g_otp6),
    td("otp8", Category::Security, "8-digit OTP", g_otp8),
    td("otp12", Category::Security, "12-digit OTP", g_otp12),
    // ---- file ----
    td("filename", Category::File, "file name", g_filename),
    td("file_ext", Category::File, "file extension", g_file_ext),
    td("mime_type", Category::File, "MIME type", g_mime_type),
    td("path", Category::File, "filesystem path", g_path),
    // ---- misc ----
    td(
        "color_hex",
        Category::Misc,
        "hex color (#RRGGBB)",
        g_color_hex,
    ),
    td("color_name", Category::Misc, "named color", g_color_name),
    td("emoji", Category::Misc, "emoji", g_emoji),
    td("semver", Category::Misc, "semantic version", g_semver),
    td(
        "hash_md5",
        Category::Misc,
        "MD5-shaped hex digest",
        g_hash_md5,
    ),
    td(
        "hash_sha1",
        Category::Misc,
        "SHA-1-shaped hex digest",
        g_hash_sha1,
    ),
    td(
        "hash_sha224",
        Category::Misc,
        "SHA-224-shaped hex digest",
        g_hash_sha224,
    ),
    td(
        "hash_sha256",
        Category::Misc,
        "SHA-256-shaped hex digest",
        g_hash_sha256,
    ),
    td(
        "hash_sha384",
        Category::Misc,
        "SHA-384-shaped hex digest",
        g_hash_sha384,
    ),
    td(
        "hash_sha512",
        Category::Misc,
        "SHA-512-shaped hex digest",
        g_hash_sha512,
    ),
    td(
        "hash_sha3_256",
        Category::Misc,
        "SHA3-256-shaped hex digest",
        g_hash_sha3_256,
    ),
    td(
        "hash_sha3_512",
        Category::Misc,
        "SHA3-512-shaped hex digest",
        g_hash_sha3_512,
    ),
    td(
        "hash_crc32",
        Category::Misc,
        "CRC-32 hex checksum",
        g_hash_crc32,
    ),
    td(
        "image_url",
        Category::Misc,
        "placeholder image URL",
        g_image_url,
    ),
    td(
        "avatar_url",
        Category::Misc,
        "placeholder avatar URL",
        g_avatar_url,
    ),
    td("book_title", Category::Misc, "book title", g_book_title),
    td("movie_title", Category::Misc, "movie title", g_movie_title),
    td(
        "music_title",
        Category::Misc,
        "music track title",
        g_music_title,
    ),
    td("language", Category::Misc, "language name", g_language),
    td(
        "country_code",
        Category::Misc,
        "ISO country code",
        g_country_code,
    ),
    td("locale", Category::Misc, "locale (en_US)", g_locale),
    td("isbn", Category::Misc, "ISBN-13", g_isbn),
    td("ean", Category::Misc, "EAN-13 barcode", g_ean),
];

const fn td(
    name: &'static str,
    category: Category,
    about: &'static str,
    gen: fn(&mut Gen) -> String,
) -> TypeDef {
    TypeDef {
        name,
        category,
        about,
        gen,
    }
}

/// Case-insensitive exact lookup. `-` is treated as `_`.
pub fn find(name: &str) -> Option<&'static TypeDef> {
    let key = name.trim().to_lowercase().replace('-', "_");
    TYPES.iter().find(|t| t.name == key)
}

/// Return every type whose name matches `pattern` (substring or fuzzy).
/// An empty pattern returns all types in registry order.
pub fn search(pattern: &str) -> Vec<&'static TypeDef> {
    let q = pattern.trim().to_lowercase();
    if q.is_empty() {
        return TYPES.iter().collect();
    }
    let mut scored: Vec<(&'static TypeDef, f64)> = TYPES
        .iter()
        .filter_map(|t| {
            let name = t.name.to_lowercase();
            if name.contains(&q) {
                return Some((t, 1.0));
            }
            let score = crate::fuzzy::jaro_winkler(&name, &q);
            if score >= 0.62 {
                Some((t, score))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| b.1.total_cmp(&a.1));
    scored.into_iter().map(|(t, _)| t).collect()
}

/// `fake types [-s|--search] [pattern]`
pub fn cmd_types(rest: &str) -> anyhow::Result<()> {
    let mut pattern = String::new();
    for tok in rest.split_whitespace() {
        match tok {
            "-s" | "--search" => {}
            other => {
                if !pattern.is_empty() {
                    pattern.push(' ');
                }
                pattern.push_str(other);
            }
        }
    }

    if pattern.is_empty() {
        println!("{}", t!("cmd.fake.types.title", TYPES.len()).cyan().bold());
        for cat in [
            Category::Person,
            Category::Contact,
            Category::Address,
            Category::Geo,
            Category::DateTime,
            Category::Numeric,
            Category::Ident,
            Category::Network,
            Category::Web,
            Category::Finance,
            Category::Company,
            Category::Text,
            Category::Security,
            Category::File,
            Category::Misc,
        ] {
            let items: Vec<&TypeDef> = TYPES.iter().filter(|t| t.category == cat).collect();
            if items.is_empty() {
                continue;
            }
            println!();
            println!("  {}", cat.label().yellow().bold());
            for def in items {
                println!("    {:<20} {}", def.name.green(), def.about.dimmed());
            }
        }
    } else {
        let hits = search(&pattern);
        if hits.is_empty() {
            print_info(&t!("cmd.fake.types.none", pattern));
        } else {
            println!(
                "{}",
                t!("cmd.fake.types.matches", pattern, hits.len())
                    .cyan()
                    .bold()
            );
            for def in hits {
                println!(
                    "    {:<20} {:<12} {}",
                    def.name.green(),
                    def.category.label().yellow(),
                    def.about.dimmed()
                );
            }
        }
    }
    Ok(())
}

/// `fake locales` — list the supported `--locale` codes with a sample row.
pub fn cmd_locales() -> anyhow::Result<()> {
    println!(
        "{}",
        t!("cmd.fake.locales.title", Locale::all().len())
            .cyan()
            .bold()
    );
    let mut g = Gen::thread();
    for &loc in Locale::all() {
        g.set_locale(loc);
        g.begin_row(0);
        let name = g_name(&mut g);
        g.begin_row(0);
        let city = g_city(&mut g);
        g.begin_row(0);
        let phone = g_phone_number(&mut g);
        println!(
            "  {:<6} {}  {}  {}",
            loc.code().green(),
            name,
            city.dimmed(),
            phone.dimmed()
        );
    }
    println!();
    print_info(&t!("cmd.fake.locales.note"));
    Ok(())
}

// ============================================================================
// generators
// ============================================================================
/// Pick a `(native, roman)` name pair, remembering the roman half on the
/// generator so row-linked types (`email`, `username`) can reuse it.
fn pick_name_pair(g: &mut Gen, last: bool) -> NamePair {
    let pool = if last {
        g.locale_data().last_names
    } else {
        g.locale_data().first_names
    };
    let pair = pool[g.range_usize(0, pool.len() - 1)];
    if last {
        g.set_roman_last(pair.roman);
    } else {
        g.set_roman_first(pair.roman);
    }
    pair
}

fn g_first_name(g: &mut Gen) -> String {
    pick_name_pair(g, false).native.to_string()
}

fn g_last_name(g: &mut Gen) -> String {
    pick_name_pair(g, true).native.to_string()
}

fn g_name(g: &mut Gen) -> String {
    let last_first = g.locale_data().last_first;
    let first = pick_name_pair(g, false);
    let last = pick_name_pair(g, true);
    if last_first {
        format!("{}{}", last.native, first.native)
    } else {
        format!("{} {}", first.native, last.native)
    }
}

/// Roman (ASCII) first/last name for the current row. Missing halves are
/// generated on demand so `email` / `username` also work on their own.
fn row_name_parts(g: &mut Gen) -> (String, String) {
    if g.roman_first().is_none() {
        let _ = g_first_name(g);
    }
    if g.roman_last().is_none() {
        let _ = g_last_name(g);
    }
    (
        g.roman_first().unwrap_or_default().to_string(),
        g.roman_last().unwrap_or_default().to_string(),
    )
}

fn g_username(g: &mut Gen) -> String {
    let (f, l) = row_name_parts(g);
    let first = sanitize_handle(&f);
    let last = sanitize_handle(&l);
    match g.range_usize(0, 2) {
        0 => format!("{}{}{}", first, last, g.range_usize(1, 99)),
        1 => format!("{}_{}{}", first, last, g.range_usize(1, 999)),
        _ => format!("{}{}", first, g.digits(4)),
    }
}

fn g_nickname(g: &mut Gen) -> String {
    let adj = g.pick_str(NICKNAME_ADJ);
    let noun = g.pick_str(NICKNAME_NOUN);
    let adj = capitalize(adj);
    let noun = capitalize(noun);
    format!("{}{}", adj, noun)
}

fn g_title(g: &mut Gen) -> String {
    g.pick_str(TITLES).to_string()
}

fn g_gender(g: &mut Gen) -> String {
    g.pick_str(GENDERS).to_string()
}

fn g_pronoun(g: &mut Gen) -> String {
    g.pick_str(PRONOUNS).to_string()
}

fn g_email(g: &mut Gen) -> String {
    let (f, l) = row_name_parts(g);
    let first = sanitize_handle(&f);
    let last = sanitize_handle(&l);
    let domain = g_domain(g);
    format!("{}.{}{}@{}", first, last, g.range_usize(1, 99), domain)
}

fn g_phone_number(g: &mut Gen) -> String {
    render_pattern(g, g.locale_data().phone_pattern)
}

fn g_phone_country_code(g: &mut Gen) -> String {
    g.locale_data().phone_cc.to_string()
}

fn g_street(g: &mut Gen) -> String {
    format!(
        "{} {} {}",
        g.range_usize(1, 9999),
        g.pick_str(g.locale_data().street_names),
        g.pick_str(g.locale_data().street_suffixes)
    )
}

fn g_city(g: &mut Gen) -> String {
    g.pick_str(g.locale_data().cities).to_string()
}

fn g_state(g: &mut Gen) -> String {
    g.pick_str(g.locale_data().states).to_string()
}

fn g_country(g: &mut Gen) -> String {
    g.pick(COUNTRIES).0.to_string()
}

fn g_country_code(g: &mut Gen) -> String {
    g.pick(COUNTRIES).1.to_string()
}

fn g_postal_code(g: &mut Gen) -> String {
    render_pattern(g, g.locale_data().postal_pattern)
}

fn g_address(g: &mut Gen) -> String {
    format!(
        "{}, {}, {} {}",
        g_street(g),
        g_city(g),
        g_state(g),
        g_postal_code(g)
    )
}

fn g_latitude(g: &mut Gen) -> String {
    format!("{:.6}", g.range_f64(-90.0, 90.0))
}

fn g_longitude(g: &mut Gen) -> String {
    format!("{:.6}", g.range_f64(-180.0, 180.0))
}

fn g_timezone(g: &mut Gen) -> String {
    g.pick_str(TIMEZONES).to_string()
}

fn rand_date(g: &mut Gen, y0: i32, y1: i32) -> NaiveDate {
    let y = g.range_usize(y0 as usize, y1 as usize) as i32;
    let m = g.range_usize(1, 12) as u32;
    let d = g.range_usize(1, 28) as u32;
    NaiveDate::from_ymd_opt(y, m, d).unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
}

fn g_date(g: &mut Gen) -> String {
    rand_date(g, 1970, 2026).format("%Y-%m-%d").to_string()
}

fn g_born_date(g: &mut Gen) -> String {
    rand_date(g, 1950, 2006).format("%Y-%m-%d").to_string()
}

fn g_joined(g: &mut Gen) -> String {
    rand_date(g, 2010, 2026).format("%Y-%m-%d").to_string()
}

fn g_datetime(g: &mut Gen) -> String {
    format!(
        "{} {:02}:{:02}:{:02}",
        rand_date(g, 2015, 2026).format("%Y-%m-%d"),
        g.range_usize(0, 23),
        g.range_usize(0, 59),
        g.range_usize(0, 59)
    )
}

fn g_time(g: &mut Gen) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        g.range_usize(0, 23),
        g.range_usize(0, 59),
        g.range_usize(0, 59)
    )
}

fn g_year(g: &mut Gen) -> String {
    g.range_usize(1970, 2026).to_string()
}

fn g_month(g: &mut Gen) -> String {
    g.range_usize(1, 12).to_string()
}

fn g_day(g: &mut Gen) -> String {
    g.range_usize(1, 28).to_string()
}

fn g_unix_timestamp(g: &mut Gen) -> String {
    (1_600_000_000i64 + g.range_i64(-200_000_000, 200_000_000)).to_string()
}

fn g_age(g: &mut Gen) -> String {
    g.range_usize(18, 80).to_string()
}

fn g_float(g: &mut Gen) -> String {
    format!("{:.3}", g.range_f64(-1000.0, 1000.0))
}

fn g_bool(g: &mut Gen) -> String {
    if g.chance(0.5) { "true" } else { "false" }.to_string()
}

fn g_uuid(g: &mut Gen) -> String {
    let mut b = g.bytes(16);
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
    )
}

fn g_ulid(g: &mut Gen) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    // 48-bit timestamp (ms) keeps the ULID lexicographically time-sortable.
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let mut s = String::with_capacity(26);
    for i in (0..10).rev() {
        if i < 10 {
            let shift = (i as u32) * 5;
            let idx = if shift >= 48 {
                g.range_usize(0, 31)
            } else {
                ((ms >> shift) & 0x1f) as usize
            };
            s.push(ALPHABET[idx] as char);
        }
    }
    for _ in 0..16 {
        s.push(ALPHABET[g.range_usize(0, 31)] as char);
    }
    s
}

fn g_nanoid(g: &mut Gen) -> String {
    const ALPHABET: &[u8] = b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    g.from_alphabet(21, ALPHABET)
}

fn g_domain(g: &mut Gen) -> String {
    format!(
        "{}{}.{}",
        g.pick_str(DOMAIN_WORDS),
        if g.chance(0.4) {
            g.range_usize(1, 99).to_string()
        } else {
            String::new()
        },
        g.pick_str(TLDS)
    )
}

fn g_url(g: &mut Gen) -> String {
    let domain = g_domain(g);
    if g.chance(0.5) {
        format!("https://{}/{}", domain, g_slug(g))
    } else {
        format!("https://www.{}", domain)
    }
}

fn g_slug(g: &mut Gen) -> String {
    let n = g.range_usize(2, 4);
    let mut words: Vec<&str> = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(g.pick_str(LOREM_WORDS));
    }
    words.join("-")
}

fn g_ipv4(g: &mut Gen) -> String {
    format!(
        "{}.{}.{}.{}",
        g.range_usize(1, 254),
        g.range_usize(0, 255),
        g.range_usize(0, 255),
        g.range_usize(1, 254)
    )
}

fn g_ipv6(g: &mut Gen) -> String {
    let groups: Vec<String> = (0..8)
        .map(|_| format!("{:04x}", g.range_usize(0, 0xffff)))
        .collect();
    groups.join(":")
}

fn g_mac_address(g: &mut Gen) -> String {
    let parts: Vec<String> = (0..6)
        .map(|_| format!("{:02x}", g.range_usize(0, 255)))
        .collect();
    parts.join(":")
}

fn g_port(g: &mut Gen) -> String {
    match g.range_usize(0, 2) {
        0 => g.range_usize(1, 1023).to_string(),
        1 => g.range_usize(1024, 49151).to_string(),
        _ => g.range_usize(49152, 65535).to_string(),
    }
}

fn g_user_agent(g: &mut Gen) -> String {
    g.pick_str(USER_AGENTS).to_string()
}

fn g_http_method(g: &mut Gen) -> String {
    g.pick_str(HTTP_METHODS).to_string()
}

fn g_status_code(g: &mut Gen) -> String {
    g.pick(STATUS_CODES).to_string()
}

fn g_credit_card(g: &mut Gen) -> String {
    // Visa-like: leading 4 + 15 digits, last digit is the Luhn check.
    let mut digits: Vec<u8> = Vec::with_capacity(16);
    digits.push(4);
    for _ in 0..14 {
        digits.push(g.range_usize(0, 9) as u8);
    }
    let check = luhn_check_digit(&digits);
    digits.push(check);
    digits.iter().map(|d| (b'0' + d) as char).collect()
}

fn g_identity_card(g: &mut Gen) -> String {
    // 12 digits with an optional trailing letter, generic national ID shape.
    let base = g.digits(12);
    if g.chance(0.4) {
        format!("{}{}", base, (b'A' + g.range_usize(0, 25) as u8) as char)
    } else {
        base
    }
}

/// Sequential primary-key-ish id (1-based row number, stable for a seed).
fn g_id(g: &mut Gen) -> String {
    (g.row_index() + 1).to_string()
}

/// Locale-aware national ID number with a valid check digit / shape.
fn g_id_card(g: &mut Gen) -> String {
    match g.locale() {
        Locale::ThTh => thai_id_card(g),
        Locale::ZhCn => zh_id_card(g),
        Locale::EnGb => {
            const PREFIX: &[u8] = b"ABCEGHJKLMNPRSTWXYZ";
            let a = PREFIX[g.range_usize(0, PREFIX.len() - 1)] as char;
            let b = PREFIX[g.range_usize(0, PREFIX.len() - 1)] as char;
            let suffix = ['A', 'B', 'C', 'D'][g.range_usize(0, 3)];
            format!("{}{}{}{}", a, b, g.digits(6), suffix)
        }
        Locale::LoLa => g.digits(9),
        Locale::EnUs => format!("{}-{}-{}", g.digits(3), g.digits(2), g.digits(4)),
    }
}

/// Thai national ID: 12 digits + mod-11 check digit (weights 13..2).
fn thai_id_card(g: &mut Gen) -> String {
    let mut d: Vec<u8> = Vec::with_capacity(13);
    for _ in 0..12 {
        d.push(g.range_usize(0, 9) as u8);
    }
    let sum: u32 = d
        .iter()
        .enumerate()
        .map(|(i, &x)| x as u32 * (13 - i as u32))
        .sum();
    d.push(((11 - (sum % 11)) % 10) as u8);
    d.iter().map(|x| (b'0' + x) as char).collect()
}

/// Chinese resident ID: 6-digit region + birth date + sequence + ISO 7064 check.
fn zh_id_card(g: &mut Gen) -> String {
    const WEIGHTS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    const CHECK: &[u8] = b"10X98765432";
    let year = g.range_usize(1960, 2005);
    let month = g.range_usize(1, 12);
    let day = g.range_usize(1, 28);
    let base = format!(
        "{}{:04}{:02}{:02}{}",
        g.digits(6),
        year,
        month,
        day,
        g.digits(3)
    );
    let sum: u32 = base
        .bytes()
        .enumerate()
        .map(|(i, b)| (b - b'0') as u32 * WEIGHTS[i])
        .sum();
    format!("{}{}", base, CHECK[(sum % 11) as usize] as char)
}

fn g_cvv(g: &mut Gen) -> String {
    g.digits(3)
}

fn g_iban(g: &mut Gen) -> String {
    let cc = g.pick_str(&["DE", "GB", "FR", "ES", "IT", "NL", "BE"]);
    let bban = g.digits(18);
    let check = iban_check(cc, &bban);
    format!("{}{:02}{}", cc, check, bban)
}

fn g_currency(g: &mut Gen) -> String {
    g.pick_str(CURRENCIES).to_string()
}

fn g_amount(g: &mut Gen) -> String {
    format!("{:.2}", g.range_f64(1.0, 9999.0))
}

fn g_price(g: &mut Gen) -> String {
    format!("{:.2}", g.range_f64(0.5, 999.0))
}

fn g_btc_address(g: &mut Gen) -> String {
    const BECH32: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    format!("bc1q{}", g.from_alphabet(38, BECH32))
}

fn g_eth_address(g: &mut Gen) -> String {
    format!("0x{}", g.hex(40))
}

fn g_company(g: &mut Gen) -> String {
    format!(
        "{} {}",
        g.pick_str(COMPANY_PREFIX),
        g.pick_str(COMPANY_SUFFIX)
    )
}

fn g_job_title(g: &mut Gen) -> String {
    g.pick_str(JOB_TITLES).to_string()
}

fn g_department(g: &mut Gen) -> String {
    g.pick_str(DEPARTMENTS).to_string()
}

fn g_industry(g: &mut Gen) -> String {
    g.pick_str(INDUSTRIES).to_string()
}

fn g_salary(g: &mut Gen) -> String {
    (g.range_usize(30, 200) * 1000).to_string()
}

fn g_word(g: &mut Gen) -> String {
    g.pick_str(LOREM_WORDS).to_string()
}

fn g_sentence(g: &mut Gen) -> String {
    let n = g.range_usize(6, 14);
    sentence(g, n)
}

fn g_paragraph(g: &mut Gen) -> String {
    let n = g.range_usize(3, 6);
    let mut parts = Vec::with_capacity(n);
    for _ in 0..n {
        let w = g.range_usize(8, 16);
        parts.push(sentence(g, w));
    }
    parts.join(" ")
}

fn g_lorem(g: &mut Gen) -> String {
    format!(
        "{} {} {} {} {}",
        g.pick_str(LOREM_WORDS),
        g.pick_str(LOREM_WORDS),
        g.pick_str(LOREM_WORDS),
        g.pick_str(LOREM_WORDS),
        g.pick_str(LOREM_WORDS)
    )
}

fn g_quote(g: &mut Gen) -> String {
    g.pick_str(QUOTES).to_string()
}

fn g_hashtag(g: &mut Gen) -> String {
    format!("#{}", g.pick_str(LOREM_WORDS))
}

fn g_password(g: &mut Gen) -> String {
    const ALPHABET: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*-_";
    let n = g.range_usize(12, 16);
    g.from_alphabet(n, ALPHABET)
}

/// Easy password: 8 lowercase/digit chars, guaranteed to contain a digit.
fn g_password_ez(g: &mut Gen) -> String {
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const DIGIT: &[u8] = b"0123456789";
    let mut out = Vec::with_capacity(8);
    out.push(pick_byte(g, DIGIT));
    for _ in 1..8 {
        let alphabet = if g.chance(0.3) { DIGIT } else { LOWER };
        out.push(pick_byte(g, alphabet));
    }
    shuffle_bytes(g, &mut out);
    String::from_utf8(out).unwrap()
}

/// Medium password: 12 chars, mixed case + digits (≥1 of each class).
fn g_password_mid(g: &mut Gen) -> String {
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGIT: &[u8] = b"0123456789";
    let mut out = Vec::with_capacity(12);
    out.push(pick_byte(g, LOWER));
    out.push(pick_byte(g, UPPER));
    out.push(pick_byte(g, DIGIT));
    for _ in 3..12 {
        let alphabet = match g.range_usize(0, 2) {
            0 => LOWER,
            1 => UPPER,
            _ => DIGIT,
        };
        out.push(pick_byte(g, alphabet));
    }
    shuffle_bytes(g, &mut out);
    String::from_utf8(out).unwrap()
}

/// Hard password: 20 chars, mixed case + digits + symbols (≥1 of each class).
fn g_password_hard(g: &mut Gen) -> String {
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGIT: &[u8] = b"0123456789";
    const SYMBOL: &[u8] = b"!@#$%^&*-_=+?";
    let mut out = Vec::with_capacity(20);
    out.push(pick_byte(g, LOWER));
    out.push(pick_byte(g, UPPER));
    out.push(pick_byte(g, DIGIT));
    out.push(pick_byte(g, SYMBOL));
    for _ in 4..20 {
        let alphabet = match g.range_usize(0, 3) {
            0 => LOWER,
            1 => UPPER,
            2 => DIGIT,
            _ => SYMBOL,
        };
        out.push(pick_byte(g, alphabet));
    }
    shuffle_bytes(g, &mut out);
    String::from_utf8(out).unwrap()
}

fn g_passphrase(g: &mut Gen) -> String {
    let n = g.range_usize(4, 5);
    let mut words: Vec<String> = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(g.pick_str(WORDS_POOL).to_string());
    }
    words.join("-")
}

fn g_otp4(g: &mut Gen) -> String {
    g.digits(4)
}
fn g_otp6(g: &mut Gen) -> String {
    g.digits(6)
}
fn g_otp8(g: &mut Gen) -> String {
    g.digits(8)
}
fn g_otp12(g: &mut Gen) -> String {
    g.digits(12)
}

fn g_filename(g: &mut Gen) -> String {
    format!("{}.{}", g_slug(g), g.pick_str(FILE_EXTS))
}

fn g_file_ext(g: &mut Gen) -> String {
    g.pick_str(FILE_EXTS).to_string()
}

fn g_mime_type(g: &mut Gen) -> String {
    g.pick_str(MIME_TYPES).to_string()
}

fn g_path(g: &mut Gen) -> String {
    let n = g.range_usize(2, 4);
    let mut segments = vec!["/".to_string()];
    for _ in 0..n {
        segments.push(g.pick_str(PATH_SEGMENTS).to_string());
    }
    format!(
        "{}/{}",
        segments.join("/").replace("//", "/"),
        g_filename(g)
    )
}

fn g_color_hex(g: &mut Gen) -> String {
    format!("#{:06X}", g.range_usize(0, 0xFF_FF_FF))
}

fn g_color_name(g: &mut Gen) -> String {
    g.pick(COLOR_NAMES).0.to_string()
}

fn g_emoji(g: &mut Gen) -> String {
    g.pick_str(EMOJIS).to_string()
}

fn g_semver(g: &mut Gen) -> String {
    let base = format!(
        "{}.{}.{}",
        g.range_usize(0, 9),
        g.range_usize(0, 20),
        g.range_usize(0, 40)
    );
    if g.chance(0.25) {
        let pre = g.pick_str(&["alpha", "beta", "rc"]);
        format!("{}-{}.{}", base, pre, g.range_usize(1, 9))
    } else {
        base
    }
}

fn g_hash_md5(g: &mut Gen) -> String {
    g.hex(32)
}

fn g_hash_sha1(g: &mut Gen) -> String {
    g.hex(40)
}

fn g_hash_sha224(g: &mut Gen) -> String {
    g.hex(56)
}

fn g_hash_sha256(g: &mut Gen) -> String {
    g.hex(64)
}

fn g_hash_sha384(g: &mut Gen) -> String {
    g.hex(96)
}

fn g_hash_sha512(g: &mut Gen) -> String {
    g.hex(128)
}

fn g_hash_sha3_256(g: &mut Gen) -> String {
    g.hex(64)
}

fn g_hash_sha3_512(g: &mut Gen) -> String {
    g.hex(128)
}

fn g_hash_crc32(g: &mut Gen) -> String {
    g.hex(8)
}

fn g_image_url(g: &mut Gen) -> String {
    format!(
        "https://picsum.photos/seed/{}/{}",
        g.hex(8),
        g.range_usize(200, 1200)
    )
}

fn g_avatar_url(g: &mut Gen) -> String {
    format!("https://i.pravatar.cc/300?u={}", g.hex(10))
}

fn g_book_title(g: &mut Gen) -> String {
    g.pick_str(BOOK_TITLES).to_string()
}

fn g_movie_title(g: &mut Gen) -> String {
    g.pick_str(MOVIE_TITLES).to_string()
}

fn g_music_title(g: &mut Gen) -> String {
    g.pick_str(MUSIC_TITLES).to_string()
}

fn g_language(g: &mut Gen) -> String {
    g.pick_str(LANGUAGES).to_string()
}

fn g_locale(g: &mut Gen) -> String {
    g.pick_str(LOCALES).to_string()
}

fn g_isbn(g: &mut Gen) -> String {
    // ISBN-13: 978 prefix + 9 digits + mod-10 check.
    let mut d: Vec<u8> = vec![9, 7, 8];
    for _ in 0..9 {
        d.push(g.range_usize(0, 9) as u8);
    }
    let check = ean13_check(&d);
    d.push(check);
    d.iter().map(|x| (b'0' + x) as char).collect()
}

fn g_ean(g: &mut Gen) -> String {
    let mut d: Vec<u8> = Vec::with_capacity(13);
    for _ in 0..12 {
        d.push(g.range_usize(0, 9) as u8);
    }
    let check = ean13_check(&d);
    d.push(check);
    d.iter().map(|x| (b'0' + x) as char).collect()
}

// ============================================================================
// helpers
// ============================================================================

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Lowercase ASCII handle safe for emails / usernames. Non-ASCII characters are
/// dropped (roman name pools are already ASCII, this is a safety net).
fn sanitize_handle(s: &str) -> String {
    let out: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if out.is_empty() {
        "user".to_string()
    } else {
        out
    }
}

/// One random byte from `alphabet` (which must be non-empty).
fn pick_byte(g: &mut Gen, alphabet: &[u8]) -> u8 {
    alphabet[g.range_usize(0, alphabet.len() - 1)]
}

/// Render a locale pattern: `#` → digit, `A` → uppercase, `a` → lowercase.
fn render_pattern(g: &mut Gen, pattern: &str) -> String {
    let mut out = String::with_capacity(pattern.len());
    for ch in pattern.chars() {
        match ch {
            '#' => out.push((b'0' + g.range_usize(0, 9) as u8) as char),
            'A' => out.push((b'A' + g.range_usize(0, 25) as u8) as char),
            'a' => out.push((b'a' + g.range_usize(0, 25) as u8) as char),
            c => out.push(c),
        }
    }
    out
}

/// In-place Fisher–Yates shuffle.
fn shuffle_bytes(g: &mut Gen, bytes: &mut [u8]) {
    for i in (1..bytes.len()).rev() {
        let j = g.range_usize(0, i);
        bytes.swap(i, j);
    }
}

fn sentence(g: &mut Gen, words: usize) -> String {
    let mut out = String::new();
    for i in 0..words {
        let w = g.pick_str(LOREM_WORDS);
        if i == 0 {
            out.push_str(&capitalize(w));
        } else {
            out.push(' ');
            out.push_str(w);
        }
    }
    out.push('.');
    out
}

fn luhn_check_digit(digits: &[u8]) -> u8 {
    let mut sum = 0u32;
    let mut double = true;
    for &d in digits.iter().rev() {
        let mut v = d as u32;
        if double {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
        double = !double;
    }
    ((10 - (sum % 10)) % 10) as u8
}

fn ean13_check(first12: &[u8]) -> u8 {
    let mut sum = 0u32;
    for (i, &d) in first12.iter().enumerate() {
        sum += d as u32 * if i % 2 == 0 { 1 } else { 3 };
    }
    ((10 - (sum % 10)) % 10) as u8
}

fn iban_check(country: &str, bban: &str) -> u8 {
    let rearranged = format!("{}{}00", bban, country);
    let mut numeric = String::new();
    for c in rearranged.chars() {
        if c.is_ascii_digit() {
            numeric.push(c);
        } else if c.is_ascii_uppercase() {
            numeric.push_str(&(c as u8 - b'A' + 10).to_string());
        }
    }
    let mut rem: u32 = 0;
    for ch in numeric.chars() {
        if let Some(d) = ch.to_digit(10) {
            rem = (rem * 10 + d) % 97;
        }
    }
    (98 - rem) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_type_generates_non_empty() {
        let mut g = Gen::seeded(42);
        for t in TYPES {
            let v = (t.gen)(&mut g);
            assert!(!v.is_empty(), "type '{}' produced an empty value", t.name);
        }
    }

    #[test]
    fn find_is_case_insensitive() {
        assert!(find("First_Name").is_some());
        assert!(find("first-name").is_some());
        assert!(find("nope").is_none());
    }

    #[test]
    fn seeded_is_deterministic() {
        let mut a = Gen::seeded(7);
        let mut b = Gen::seeded(7);
        for t in TYPES {
            assert_eq!((t.gen)(&mut a), (t.gen)(&mut b), "type {}", t.name);
        }
    }

    #[test]
    fn search_finds_substring_and_fuzzy() {
        assert!(search("first").iter().any(|t| t.name == "first_name"));
        assert!(search("emial").iter().any(|t| t.name == "email"));
    }

    #[test]
    fn hash_lengths_match_algorithm() {
        let cases: &[(&str, usize)] = &[
            ("hash_md5", 32),
            ("hash_sha1", 40),
            ("hash_sha224", 56),
            ("hash_sha256", 64),
            ("hash_sha384", 96),
            ("hash_sha512", 128),
            ("hash_sha3_256", 64),
            ("hash_sha3_512", 128),
            ("hash_crc32", 8),
        ];
        let mut g = Gen::seeded(3);
        for (name, len) in cases {
            let def = find(name).unwrap_or_else(|| panic!("missing type {name}"));
            let v = (def.gen)(&mut g);
            assert_eq!(v.len(), *len, "{name} length");
            assert!(
                v.chars().all(|c| c.is_ascii_hexdigit()),
                "{name} should be hex, got {v}"
            );
        }
    }

    #[test]
    fn locale_switches_pools_and_patterns() {
        use crate::fake::locale::Locale;
        let mut g = Gen::seeded(1);
        g.set_locale(Locale::ZhCn);
        assert_eq!((find("postal_code").unwrap().gen)(&mut g).len(), 6);
        g.set_locale(Locale::ThTh);
        let phone = (find("phone_number").unwrap().gen)(&mut g);
        assert!(phone.starts_with("+66"), "got {phone}");
        g.set_locale(Locale::LoLa);
        let phone = (find("phone_number").unwrap().gen)(&mut g);
        assert!(phone.starts_with("+856"), "got {phone}");
    }

    #[test]
    fn password_tiers_have_expected_classes() {
        let mut g = Gen::seeded(11);
        for _ in 0..25 {
            let ez = g_password_ez(&mut g);
            assert_eq!(ez.len(), 8);
            assert!(ez.chars().any(|c| c.is_ascii_digit()), "ez needs a digit");
            assert!(
                ez.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()),
                "ez must be lowercase/digits only, got {ez}"
            );

            let mid = g_password_mid(&mut g);
            assert_eq!(mid.len(), 12);
            assert!(mid.chars().any(|c| c.is_ascii_lowercase()));
            assert!(mid.chars().any(|c| c.is_ascii_uppercase()));
            assert!(mid.chars().any(|c| c.is_ascii_digit()));

            let hard = g_password_hard(&mut g);
            assert_eq!(hard.len(), 20);
            assert!(hard.chars().any(|c| c.is_ascii_lowercase()));
            assert!(hard.chars().any(|c| c.is_ascii_uppercase()));
            assert!(hard.chars().any(|c| c.is_ascii_digit()));
            assert!(hard.chars().any(|c| !c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn email_reuses_row_name() {
        let mut g = Gen::seeded(5);
        let name = g_name(&mut g);
        let email = g_email(&mut g);
        let parts: Vec<String> = name.to_lowercase().split(' ').map(str::to_string).collect();
        assert_eq!(parts.len(), 2, "unexpected name shape: {name}");
        let prefix = format!("{}.{}", parts[0], parts[1]);
        assert!(
            email.starts_with(&prefix),
            "name={name} email={email} prefix={prefix}"
        );
    }

    #[test]
    fn email_is_ascii_in_every_locale() {
        for &loc in Locale::all() {
            let mut g = Gen::seeded(3);
            g.set_locale(loc);
            for i in 0..25 {
                g.begin_row(i);
                let email = g_email(&mut g);
                assert!(email.is_ascii(), "{:?} produced {email}", loc.code());
                assert!(email.contains('@'), "{:?} produced {email}", loc.code());
            }
        }
    }

    #[test]
    fn username_is_ascii_in_every_locale() {
        for &loc in Locale::all() {
            let mut g = Gen::seeded(4);
            g.set_locale(loc);
            for i in 0..25 {
                g.begin_row(i);
                let u = g_username(&mut g);
                assert!(u.is_ascii(), "{:?} produced {u}", loc.code());
                assert!(!u.is_empty(), "{:?} produced an empty username", loc.code());
            }
        }
    }

    #[test]
    fn name_respects_locale_order() {
        let mut g = Gen::seeded(8);
        g.set_locale(Locale::ThTh);
        let th = g_name(&mut g);
        assert!(th.contains(' '), "Thai name should be 'first last': {th}");
        g.set_locale(Locale::ZhCn);
        let zh = g_name(&mut g);
        assert!(
            !zh.contains(' '),
            "Chinese name should be 'lastfirst': {zh}"
        );
        assert_eq!(
            zh.chars().count(),
            2,
            "Chinese name should be 2 chars: {zh}"
        );
    }

    #[test]
    fn id_is_sequential() {
        let mut g = Gen::seeded(1);
        for i in 0..5 {
            g.begin_row(i);
            assert_eq!(g_id(&mut g), (i + 1).to_string());
        }
    }

    #[test]
    fn id_card_matches_locale_shape() {
        let mut g = Gen::seeded(2);
        g.set_locale(Locale::ThTh);
        for _ in 0..50 {
            let id = g_id_card(&mut g);
            assert_eq!(id.len(), 13, "th id {id}");
            let d: Vec<u32> = id.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let sum: u32 = d[..12]
                .iter()
                .enumerate()
                .map(|(i, &x)| x * (13 - i as u32))
                .sum();
            assert_eq!(d[12], (11 - sum % 11) % 10, "th check digit for {id}");
        }
        g.set_locale(Locale::ZhCn);
        for _ in 0..50 {
            let id = g_id_card(&mut g);
            assert_eq!(id.len(), 18, "zh id {id}");
            let (base, check) = id.split_at(17);
            assert!(base.chars().all(|c| c.is_ascii_digit()), "zh base {base}");
            assert!("10X98765432".contains(check), "zh check {check}");
        }
        g.set_locale(Locale::EnUs);
        let id = g_id_card(&mut g);
        assert_eq!(id.len(), 11, "us id {id}");
        assert_eq!(&id[3..4], "-");
        assert_eq!(&id[6..7], "-");
        g.set_locale(Locale::EnGb);
        assert_eq!(g_id_card(&mut g).len(), 9);
        g.set_locale(Locale::LoLa);
        assert_eq!(g_id_card(&mut g).len(), 9);
    }

    #[test]
    fn new_identifier_types_resolve() {
        assert!(find("id").is_some());
        assert!(find("id_card").is_some());
        assert!(find("id-card").is_some());
        assert!(find("identity_card").is_some());
    }

    #[test]
    fn credit_card_is_luhn_valid() {
        let mut g = Gen::seeded(1);
        for _ in 0..50 {
            let card = g_credit_card(&mut g);
            let digits: Vec<u8> = card.bytes().map(|b| b - b'0').collect();
            // A full Luhn sum over all digits (including check) is ≡ 0 mod 10.
            let mut sum = 0u32;
            let mut double = false;
            for &d in digits.iter().rev() {
                let mut v = d as u32;
                if double {
                    v *= 2;
                    if v > 9 {
                        v -= 9;
                    }
                }
                sum += v;
                double = !double;
            }
            assert_eq!(sum % 10, 0, "card {card} is not Luhn valid");
        }
    }
}
