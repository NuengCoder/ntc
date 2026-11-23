//! `tmprt weather`, plus the `tmprt key` / `tmprt city` config helpers.

use crate::config::Config;
use crate::output::{print_error, print_info, print_success, print_warning};
use crate::t;
use crate::utils::theme::style::ThemeColorize;
use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;

const WEATHER_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Deserialize)]
struct WeatherResp {
    #[serde(default)]
    coord: Coord,
    #[serde(default)]
    weather: Vec<Weather>,
    #[serde(default)]
    main: Main,
    #[serde(default)]
    wind: Wind,
    #[serde(default)]
    clouds: Clouds,
    #[serde(default)]
    sys: Sys,
    #[serde(default)]
    name: String,
    #[serde(default)]
    visibility: Option<i64>,
}

#[derive(Default, Deserialize)]
struct Coord {
    #[serde(default)]
    lon: f64,
    #[serde(default)]
    lat: f64,
}

#[derive(Default, Deserialize)]
struct Weather {
    #[serde(default)]
    main: String,
    #[serde(default)]
    description: String,
}

#[derive(Default, Deserialize)]
struct Main {
    #[serde(default)]
    temp: f64,
    #[serde(default)]
    feels_like: f64,
    #[serde(default)]
    temp_min: f64,
    #[serde(default)]
    temp_max: f64,
    #[serde(default)]
    pressure: f64,
    #[serde(default)]
    humidity: f64,
}

#[derive(Default, Deserialize)]
struct Wind {
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    deg: i64,
}

#[derive(Default, Deserialize)]
struct Clouds {
    #[serde(default)]
    all: i64,
}

#[derive(Default, Deserialize)]
struct Sys {
    #[serde(default)]
    country: String,
    #[serde(default)]
    sunrise: i64,
    #[serde(default)]
    sunset: i64,
}

fn resolve_api_key() -> Option<String> {
    if let Ok(k) = std::env::var("OPENWEATHER_API_KEY") {
        let k = k.trim().to_string();
        if !k.is_empty() {
            return Some(k);
        }
    }
    Config::global_get_openweather_api_key().filter(|k| !k.trim().is_empty())
}

fn resolve_default_city() -> Option<String> {
    Config::global_get_openweather_default_city().filter(|c| !c.trim().is_empty())
}

pub(super) fn cmd_tmprt_weather(args: &str) -> Result<bool> {
    let (city_arg, units) = parse_weather_args(args);

    let key = match resolve_api_key() {
        Some(k) => k,
        None => {
            print_error(&t!("cmd.tmprt.weather.no_key"));
            print_info(&t!("cmd.tmprt.weather.no_key_hint"));
            return Ok(false);
        }
    };

    let city = match city_arg {
        Some(c) => c,
        None => match resolve_default_city() {
            Some(c) => c,
            None => {
                print_error(&t!("cmd.tmprt.weather.no_city"));
                print_info(&t!("cmd.tmprt.weather.no_city_hint"));
                return Ok(false);
            }
        },
    };

    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(15)))
        .http_status_as_error(false)
        .build()
        .new_agent();

    let response = match agent
        .get(WEATHER_URL)
        .query("q", &city)
        .query("appid", &key)
        .query("units", units.as_str())
        .call()
    {
        Ok(r) => r,
        Err(e) => {
            print_error(&t!("cmd.tmprt.weather.request_failed", e));
            return Ok(false);
        }
    };

    let status = response.status();
    let body = response.into_body().read_to_string().unwrap_or_default();

    if status != 200 {
        match status.as_u16() {
            401 => print_error(&t!("cmd.tmprt.weather.bad_key")),
            404 => print_error(&t!("cmd.tmprt.weather.city_not_found", city)),
            _ => {
                let msg = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| {
                        v.get("message")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "<no detail>".to_string());
                print_error(&t!("cmd.tmprt.weather.http_error", status.as_u16(), msg));
            }
        }
        return Ok(false);
    }

    let w: WeatherResp = match serde_json::from_str(&body) {
        Ok(w) => w,
        Err(e) => {
            print_error(&t!("cmd.tmprt.weather.parse_failed", e));
            return Ok(false);
        }
    };

    print_weather(&w, &units);
    Ok(false)
}

fn parse_weather_args(args: &str) -> (Option<String>, String) {
    let mut city = None;
    let mut units = "metric".to_string();
    for token in args.split_whitespace() {
        if let Some(v) = token.strip_prefix("--units=") {
            units = if v.eq_ignore_ascii_case("imperial") {
                "imperial".to_string()
            } else {
                "metric".to_string()
            };
        } else if token == "--imperial" || token == "-i" {
            units = "imperial".to_string();
        } else if token == "--metric" || token == "-m" {
            units = "metric".to_string();
        } else if city.is_none() {
            city = Some(token.to_string());
        }
    }
    (city, units)
}

fn print_weather(w: &WeatherResp, units: &str) {
    let unit_sym = if units == "imperial" { "'oF" } else { "'oC" };
    let wind_unit = if units == "imperial" { "mph" } else { "m/s" };
    let temp = |c: f64| format!("{:.1}{unit_sym}", c);
    let desc = w
        .weather
        .first()
        .map(|wd| wd.description.as_str())
        .unwrap_or("");

    let location = format!("{}, {}", w.name, w.sys.country);
    println!();
    println!("{}", "=".repeat(46).cyan());
    println!(
        "  {}  {}",
        t!("cmd.tmprt.weather.title").cyan().bold(),
        location.bold()
    );
    println!("{}", "=".repeat(46).cyan());

    if !desc.is_empty() {
        println!(
            "  {:<12} {} {}",
            t!("cmd.tmprt.weather.condition").cyan(),
            w.weather
                .first()
                .map(|x| x.main.as_str())
                .unwrap_or("")
                .bold(),
            format!("({})", desc).dimmed()
        );
    }
    println!(
        "  {:<12} {}  ({} {})",
        t!("cmd.tmprt.weather.temp").cyan(),
        temp(w.main.temp).bold(),
        t!("cmd.tmprt.weather.feels"),
        temp(w.main.feels_like).dimmed()
    );
    println!(
        "  {:<12} {} / {}",
        t!("cmd.tmprt.weather.range").cyan(),
        temp(w.main.temp_min),
        temp(w.main.temp_max)
    );
    println!(
        "  {:<12} {}%",
        t!("cmd.tmprt.weather.humidity").cyan(),
        w.main.humidity.round()
    );
    println!(
        "  {:<12} {} hPa",
        t!("cmd.tmprt.weather.pressure").cyan(),
        w.main.pressure.round()
    );
    println!(
        "  {:<12} {} {wind_unit} ({}, {})'o",
        t!("cmd.tmprt.weather.wind").cyan(),
        w.wind.speed,
        compass_dir(w.wind.deg),
        w.wind.deg
    );
    if let Some(vis) = w.visibility {
        let vis_km = if units == "imperial" {
            (vis as f64 / 1609.344).round() as i64
        } else {
            vis / 1000
        };
        println!(
            "  {:<12} {} {}",
            t!("cmd.tmprt.weather.visibility").cyan(),
            vis_km,
            if units == "imperial" { "mi" } else { "km" }
        );
    }
    println!(
        "  {:<12} {}%",
        t!("cmd.tmprt.weather.clouds").cyan(),
        w.clouds.all
    );
    let sunrise = fmt_unix_time(w.sys.sunrise);
    let sunset = fmt_unix_time(w.sys.sunset);
    if let (Some(rise), Some(set)) = (sunrise, sunset) {
        println!(
            "  {:<12} {}   {} {}",
            t!("cmd.tmprt.weather.sunrise").cyan(),
            rise,
            t!("cmd.tmprt.weather.sunset").cyan(),
            set
        );
    }
    println!(
        "  {:<12} {:.2}'o, {:.2}'o",
        t!("cmd.tmprt.weather.coords").cyan(),
        w.coord.lat,
        w.coord.lon
    );
    println!("{}", "=".repeat(46).cyan());
    println!();
}

fn compass_dir(deg: i64) -> &'static str {
    const DIRS: [&str; 16] = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let idx = ((deg as f64 / 22.5).round() as usize) % 16;
    DIRS[idx]
}

fn fmt_unix_time(ts: i64) -> Option<String> {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.with_timezone(&chrono::Local).format("%H:%M").to_string())
}

// ─────────────────────────────────────────────────────────────
// tmprt key / city — config helpers
// ─────────────────────────────────────────────────────────────

pub(super) fn cmd_tmprt_key(args: &str) -> Result<bool> {
    let key = args.trim();
    if key.is_empty() {
        match resolve_api_key() {
            Some(k) => {
                let masked = mask_key(&k);
                if std::env::var("OPENWEATHER_API_KEY")
                    .ok()
                    .is_some_and(|v| !v.trim().is_empty())
                {
                    print_info(&t!("cmd.tmprt.key.env_source", masked));
                } else {
                    print_info(&t!("cmd.tmprt.key.status", masked));
                }
            }
            None => {
                print_warning(&t!("cmd.tmprt.key.not_set"));
                print_info(&t!("cmd.tmprt.weather.no_key_hint"));
            }
        }
        return Ok(false);
    }

    Config::global_set_openweather_api_key(Some(key.to_string()));
    print_success(&t!("cmd.tmprt.key.saved", mask_key(key)));
    Ok(false)
}

pub(super) fn cmd_tmprt_city(args: &str) -> Result<bool> {
    let city = args.trim();
    if city.is_empty() {
        match resolve_default_city() {
            Some(c) => print_info(&t!("cmd.tmprt.city.status", c)),
            None => {
                print_warning(&t!("cmd.tmprt.city.not_set"));
                print_info(&t!("cmd.tmprt.weather.no_city_hint"));
            }
        }
        return Ok(false);
    }

    Config::global_set_openweather_default_city(Some(city.to_string()));
    print_success(&t!("cmd.tmprt.city.saved", city));
    Ok(false)
}

fn mask_key(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    if chars.len() <= 8 {
        let head: String = chars.iter().take(4).collect();
        return format!("{}****", head);
    }
    let start: String = chars.iter().take(4).collect();
    let end: String = chars[chars.len() - 4..].iter().collect();
    format!("{}****{}", start, end)
}
