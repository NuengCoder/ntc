//! `serve chess` — hotseat chess server with a hand-rolled rules engine.
//!
//! Single lobby in memory for the prototype (multi-room schema is
//! reserved in `chess.db` for the online phase). REST + polling like
//! the other serve services; no `--auth` by design.

use super::chess_engine::{
    self, apply_move, in_check, initial_position, legal_moves_for, parse_sq, sq_name, to_san,
    Color, Kind, Piece, Position, Status,
};
use super::serve::{
    json_error, json_ok, parse_query, print_share_block, response_bytes, spawn_cloudflared,
    tokenize, write_all_retry,
};
use crate::config::{Config, ServeService};
use crate::navigator::Navigator;
use crate::output::{print_error, print_info};
use crate::t;
use anyhow::{Context, Result};
use colored::*;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(rust_embed::Embed)]
#[folder = "ntc_chess_web/dist"]
struct ChessFrontend;

// ── Settings DB (chess.db) ────────────────────────────────────────────────

fn chess_db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ntc")
        .join("chess.db")
}

const SETTING_DEFAULTS: &[(&str, &str)] = &[
    ("sfx.master", "80"),
    ("sfx.move", "80"),
    ("sfx.capture", "80"),
    ("sfx.promote", "80"),
    ("sfx.castle", "80"),
    ("sfx.check", "80"),
    ("sfx.win", "80"),
    ("sfx.lose", "80"),
    ("chess.tc", "10+0"),
];

fn ensure_chess_db_at(path: &Path) -> Result<rusqlite::Connection> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = rusqlite::Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
         -- Reserved for the online phase (unused by hotseat v1).
         CREATE TABLE IF NOT EXISTS rooms (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, created_at TEXT DEFAULT (datetime('now')));
         CREATE TABLE IF NOT EXISTS room_players (room_id INTEGER NOT NULL, color TEXT NOT NULL, name TEXT NOT NULL DEFAULT '')",
    )?;
    for (k, v) in SETTING_DEFAULTS {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
            rusqlite::params![k, v],
        )?;
    }
    Ok(conn)
}

fn ensure_chess_db() -> Result<rusqlite::Connection> {
    ensure_chess_db_at(&chess_db_path())
}

fn db_setting(db: &rusqlite::Connection, key: &str) -> String {
    db.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get(0),
    )
    .unwrap_or_else(|_| {
        SETTING_DEFAULTS
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.to_string())
            .unwrap_or_default()
    })
}

fn db_set_setting(db: &rusqlite::Connection, key: &str, value: &str) -> Result<()> {
    db.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

fn is_setting_key(key: &str) -> bool {
    SETTING_DEFAULTS.iter().any(|(k, _)| *k == key)
}

// ── Sessions (one game per device) ──────────────────────────────────────────
//
// 2-player hotseat is per-screen ("offline"): every browser gets its own
// isolated game via a `ntc_chess` cookie, so a laptop and a phone never
// share a lobby. Same browser/tabs share cookies and therefore one game
// (correct). This store is also the future Online foundation (named
// joinable rooms instead of cookie sessions).

pub const SESSION_COOKIE: &str = "ntc_chess";
/// Idle sessions older than this are swept on the next creation.
const SESSION_TTL: std::time::Duration = std::time::Duration::from_secs(30 * 60);
/// Hard cap; beyond it the stalest session is evicted on creation.
const SESSION_CAP: usize = 64;

struct SessionEntry {
    game: Arc<Mutex<Game>>,
    last_seen: Instant,
}

pub struct ChessSessions {
    inner: Mutex<std::collections::HashMap<String, SessionEntry>>,
    default_tc: String,
}

fn new_session_id() -> String {
    let n: u128 = rand::random();
    format!("{:032x}", n)
}

/// Extract a well-formed session id from a `Cookie` header value.
/// Anything malformed (or another cookie's value) yields None so the
/// caller mints a fresh session.
fn cookie_session_id(header: &str) -> Option<String> {
    for part in header.split(';') {
        let (k, v) = part.split_once('=')?;
        if k.trim() == SESSION_COOKIE {
            let id = v.trim().trim_matches('"');
            if !id.is_empty() && id.len() <= 64 && id.bytes().all(|b| b.is_ascii_alphanumeric()) {
                return Some(id.to_string());
            }
            return None;
        }
    }
    None
}

/// Splice a `Set-Cookie` line into raw response bytes ahead of the blank
/// line. The status-line log parser only reads the first line, so it is
/// unaffected. Responses without `\r\n\r\n` pass through untouched.
fn inject_cookie(resp: Vec<u8>, id: &str) -> Vec<u8> {
    let needle = b"\r\n\r\n";
    let pos = resp.windows(4).position(|w| w == needle);
    match pos {
        Some(i) => {
            let line = format!(
                "Set-Cookie: {}={}; Path=/; SameSite=Lax\r\n",
                SESSION_COOKIE, id
            );
            let mut out = Vec::with_capacity(resp.len() + line.len());
            out.extend_from_slice(&resp[..i + 2]);
            out.extend_from_slice(line.as_bytes());
            out.extend_from_slice(&resp[i + 2..]);
            out
        }
        None => resp,
    }
}

impl ChessSessions {
    pub fn new(default_tc: String) -> Self {
        ChessSessions {
            inner: Mutex::new(std::collections::HashMap::new()),
            default_tc,
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, std::collections::HashMap<String, SessionEntry>> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Resolve the caller's game. Returns the game plus `Some(id)` when a
    /// fresh session was minted (caller must `Set-Cookie` it).
    pub fn session_for(&self, cookie_id: Option<String>) -> (Arc<Mutex<Game>>, Option<String>) {
        let mut map = self.lock();
        if let Some(ref id) = cookie_id {
            if let Some(entry) = map.get_mut(id) {
                if entry.last_seen.elapsed() < SESSION_TTL {
                    entry.last_seen = Instant::now();
                    return (entry.game.clone(), None);
                }
                map.remove(id);
            }
        }
        // Sweep idle sessions, then enforce the cap by evicting stalest.
        let now = Instant::now();
        map.retain(|_, e| now.duration_since(e.last_seen) < SESSION_TTL);
        if map.len() >= SESSION_CAP {
            if let Some(victim) = map
                .iter()
                .min_by_key(|(_, e)| e.last_seen)
                .map(|(k, _)| k.clone())
            {
                map.remove(&victim);
            }
        }
        let id = new_session_id();
        let game = Arc::new(Mutex::new(
            Game::new(&self.default_tc).unwrap_or_else(|_| Game::new("10+0").unwrap()),
        ));
        map.insert(
            id.clone(),
            SessionEntry {
                game: game.clone(),
                last_seen: now,
            },
        );
        (game, Some(id))
    }
}

// ── Game + clocks ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EndState {
    Checkmate { winner: Color },
    Stalemate,
    DrawFifty,
    DrawRepetition,
    DrawMaterial,
    DrawAgreement,
    Resigned { winner: Color },
    Timeout { winner: Color },
    Abandoned { winner: Color },
}

impl EndState {
    fn reason(&self) -> &'static str {
        match self {
            EndState::Checkmate { .. } => "checkmate",
            EndState::Stalemate => "stalemate",
            EndState::DrawFifty => "fifty-move",
            EndState::DrawRepetition => "repetition",
            EndState::DrawMaterial => "insufficient-material",
            EndState::DrawAgreement => "agreement",
            EndState::Resigned { .. } => "resign",
            EndState::Timeout { .. } => "timeout",
            EndState::Abandoned { .. } => "abandoned",
        }
    }

    fn winner(&self) -> Option<Color> {
        match self {
            EndState::Checkmate { winner }
            | EndState::Resigned { winner }
            | EndState::Timeout { winner }
            | EndState::Abandoned { winner } => Some(*winner),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MoveRecord {
    pub from: String,
    pub to: String,
    pub san: String,
}

/// Full takeback snapshot: everything a move can mutate, so `undo()`
/// restores board, history, trays, clocks and terminal state exactly.
#[derive(Clone, Debug)]
struct Snapshot {
    pos: Position,
    moves: Vec<MoveRecord>,
    captured_by_white: Vec<String>,
    captured_by_black: Vec<String>,
    white_ms: Option<i64>,
    black_ms: Option<i64>,
    over: Option<EndState>,
    last_result: Option<(Option<Color>, String)>,
}

pub struct Game {
    pos: Position,
    moves: Vec<MoveRecord>,
    captured_by_white: Vec<String>,
    captured_by_black: Vec<String>,
    white_ms: Option<i64>,
    black_ms: Option<i64>,
    incr_ms: i64,
    tc_label: String,
    last_tick: Option<Instant>,
    over: Option<EndState>,
    last_result: Option<(Option<Color>, String)>,
    history: Vec<Snapshot>,
}

fn color_name(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Black => "black",
    }
}

fn color_short(c: Color) -> &'static str {
    match c {
        Color::White => "w",
        Color::Black => "b",
    }
}

fn only_king(pos: &Position, color: Color) -> bool {
    pos.board.iter().all(|p| match p {
        None => true,
        Some(Piece { kind, .. }) if *kind == Kind::King => true,
        Some(Piece { color: c, .. }) => *c != color,
    })
}

/// Parse "10+0" / "5+3" / "unlimited" → (white_ms, black_ms, incr_ms, label).
fn parse_tc(spec: &str) -> Result<(Option<i64>, Option<i64>, i64, String)> {
    let s = spec.trim();
    if s.eq_ignore_ascii_case("unlimited") || s.eq_ignore_ascii_case("none") {
        return Ok((None, None, 0, "unlimited".to_string()));
    }
    let (mins, inc) = match s.split_once('+') {
        Some((m, i)) => (m.trim(), i.trim()),
        None => (s, "0"),
    };
    let m: u32 = mins.parse().map_err(|_| {
        anyhow::anyhow!("bad time control '{}' (use M+S like 10+0, or unlimited)", s)
    })?;
    let inc: u32 = inc.parse().map_err(|_| {
        anyhow::anyhow!("bad time control '{}' (use M+S like 10+0, or unlimited)", s)
    })?;
    if m == 0 || m > 180 {
        anyhow::bail!("bad time control '{}' (minutes 1-180)", s);
    }
    if inc > 300 {
        anyhow::bail!("bad time control '{}' (increment 0-300s)", s);
    }
    let ms = m as i64 * 60_000;
    Ok((
        Some(ms),
        Some(ms),
        inc as i64 * 1000,
        format!("{}+{}", m, inc),
    ))
}

impl Game {
    pub fn new(tc_spec: &str) -> Result<Game> {
        let (w, b, incr, label) = parse_tc(tc_spec)?;
        Ok(Game {
            pos: initial_position(),
            moves: Vec::new(),
            captured_by_white: Vec::new(),
            captured_by_black: Vec::new(),
            white_ms: w,
            black_ms: b,
            incr_ms: incr,
            tc_label: label,
            last_tick: None,
            over: None,
            last_result: None,
            history: Vec::new(),
        })
    }

    fn clock_mut(&mut self, color: Color) -> &mut Option<i64> {
        match color {
            Color::White => &mut self.white_ms,
            Color::Black => &mut self.black_ms,
        }
    }

    /// Tick the side-to-move clock. Flag-fall ends the game immediately.
    pub fn tick(&mut self) {
        if self.over.is_some() || self.moves.is_empty() {
            self.last_tick = Some(Instant::now());
            return;
        }
        let now = Instant::now();
        if let Some(last) = self.last_tick {
            let elapsed = last.elapsed().as_millis() as i64;
            let turn = self.pos.turn;
            if let Some(ms) = self.clock_mut(turn) {
                *ms -= elapsed;
                if *ms <= 0 {
                    *ms = 0;
                    // Timeout vs a bare king is a draw, not a win.
                    if only_king(&self.pos, turn.opp()) {
                        self.finish(EndState::DrawMaterial);
                    } else {
                        self.finish(EndState::Timeout { winner: turn.opp() });
                    }
                }
            }
        }
        self.last_tick = Some(now);
    }

    fn finish(&mut self, end: EndState) {
        self.last_result = Some((end.winner(), end.reason().to_string()));
        self.over = Some(end);
    }

    /// Used by tests and reserved for the online phase (spectator polling).
    #[allow(dead_code)]
    pub fn is_over(&self) -> bool {
        self.over.is_some()
    }

    /// Apply a move. Returns (san, sfx_event). Errors: game_over,
    /// bad_square, not_your_turn(implicit via empty), illegal,
    /// promotion_required.
    pub fn do_move(
        &mut self,
        from: u8,
        to: u8,
        promo: Option<Kind>,
    ) -> Result<(String, &'static str), String> {
        self.tick();
        if let Some(_end) = &self.over {
            return Err("game_over".to_string());
        }
        let piece = self.pos.board[from as usize].ok_or("bad_square".to_string())?;
        if piece.color != self.pos.turn {
            return Err("illegal".to_string());
        }
        let legal = legal_moves_for(&self.pos, from);
        if legal.is_empty() {
            return Err("illegal".to_string());
        }
        let needs_promo = legal.iter().any(|m| m.to == to && m.promo.is_some());
        if needs_promo && promo.is_none() {
            return Err("promotion_required".to_string());
        }
        let mv = legal
            .iter()
            .find(|m| m.to == to && m.promo == promo)
            .copied()
            .ok_or("illegal".to_string())?;

        // Takeback snapshot: everything below mutates, so a later
        // `undo()` restores board, history, trays, clocks and status.
        self.history.push(Snapshot {
            pos: self.pos.clone(),
            moves: self.moves.clone(),
            captured_by_white: self.captured_by_white.clone(),
            captured_by_black: self.captured_by_black.clone(),
            white_ms: self.white_ms,
            black_ms: self.black_ms,
            over: self.over.clone(),
            last_result: self.last_result.clone(),
        });

        let san = to_san(&self.pos, mv);
        let mover = self.pos.turn;
        let applied = apply_move(&mut self.pos, mv);
        if let Some(cap) = applied.captured {
            let tray = match mover {
                Color::White => &mut self.captured_by_white,
                Color::Black => &mut self.captured_by_black,
            };
            tray.push(cap.kind.code().to_string());
        }
        self.moves.push(MoveRecord {
            from: sq_name(from),
            to: sq_name(to),
            san: san.clone(),
        });
        // Increment for the mover.
        let incr = self.incr_ms;
        if incr > 0 {
            if let Some(ms) = self.clock_mut(mover) {
                *ms += incr;
            }
        }
        self.last_tick = Some(Instant::now());

        // Sound event + terminal status.
        let gives_check = in_check(&self.pos, self.pos.turn);
        let sfx: &'static str = match chess_engine::evaluate(&self.pos) {
            Status::Checkmate(w) => {
                self.finish(EndState::Checkmate { winner: w });
                "win"
            }
            Status::Stalemate => {
                self.finish(EndState::Stalemate);
                "win"
            }
            Status::DrawFifty => {
                self.finish(EndState::DrawFifty);
                "win"
            }
            Status::DrawRepetition => {
                self.finish(EndState::DrawRepetition);
                "win"
            }
            Status::DrawMaterial => {
                self.finish(EndState::DrawMaterial);
                "win"
            }
            Status::Ongoing => {
                if applied.captured.is_some() {
                    "capture"
                } else if applied.was_castle_kingside || applied.was_castle_queenside {
                    "castle"
                } else if mv.promo.is_some() {
                    "promote"
                } else if gives_check {
                    "check"
                } else {
                    "move"
                }
            }
        };
        Ok((san, sfx))
    }

    pub fn resign(&mut self, color: Color) -> Result<(), String> {
        self.tick();
        if self.over.is_some() {
            return Err("game_over".to_string());
        }
        self.finish(EndState::Resigned {
            winner: color.opp(),
        });
        Ok(())
    }

    pub fn draw_agree(&mut self) -> Result<(), String> {
        self.tick();
        if self.over.is_some() {
            return Err("game_over".to_string());
        }
        self.finish(EndState::DrawAgreement);
        Ok(())
    }

    /// Fresh game. A live game abandoned this way forfeits: side to move loses.
    pub fn reset(&mut self, tc_spec: &str) -> Result<bool> {
        let (w, b, incr, label) = parse_tc(tc_spec)?;
        let abandoned = self.over.is_none() && !self.moves.is_empty();
        if abandoned {
            self.finish(EndState::Abandoned {
                winner: self.pos.turn.opp(),
            });
        }
        let last_result = self.last_result.clone();
        *self = Game {
            pos: initial_position(),
            moves: Vec::new(),
            captured_by_white: Vec::new(),
            captured_by_black: Vec::new(),
            white_ms: w,
            black_ms: b,
            incr_ms: incr,
            tc_label: label,
            last_tick: None,
            over: None,
            last_result,
            history: Vec::new(),
        };
        Ok(abandoned)
    }

    /// Take back one ply, restoring board, history, trays, clocks and
    /// terminal state. Works after game over too (resurrects the game).
    /// Returns the undone move for the reverse-slide animation.
    /// No clock tick: restored clocks are authoritative, `last_tick`
    /// restarts now.
    pub fn undo(&mut self) -> Result<MoveRecord, String> {
        let undone = self
            .moves
            .last()
            .cloned()
            .ok_or("nothing_to_undo".to_string())?;
        let snap = self.history.pop().ok_or("nothing_to_undo".to_string())?;
        self.pos = snap.pos;
        self.moves = snap.moves;
        self.captured_by_white = snap.captured_by_white;
        self.captured_by_black = snap.captured_by_black;
        self.white_ms = snap.white_ms;
        self.black_ms = snap.black_ms;
        self.over = snap.over;
        self.last_result = snap.last_result;
        self.last_tick = Some(Instant::now());
        Ok(undone)
    }

    /// Historical board after ply K (0 = initial position). Read-only:
    /// no clock tick — previews are frozen by design. None when K is out
    /// of range. Invariant: `history.len() == moves.len()`, and
    /// `history[K]` is the snapshot taken before move K+1, i.e. the
    /// position after exactly K plies.
    pub fn position_json(&self, ply: usize) -> Option<Vec<u8>> {
        if ply > self.moves.len() {
            return None;
        }
        let (pos, white, black) = if ply == self.moves.len() {
            (&self.pos, &self.captured_by_white, &self.captured_by_black)
        } else {
            let s = self.history.get(ply)?;
            (&s.pos, &s.captured_by_white, &s.captured_by_black)
        };
        let last_move = if ply == 0 {
            None
        } else {
            self.moves.get(ply - 1)
        };
        serde_json::to_vec(&serde_json::json!({
            "ply": ply,
            "board": board_json(pos),
            "turn": color_short(pos.turn),
            "last_move": last_move.map(|m| serde_json::json!({"from": m.from, "to": m.to, "san": m.san})),
            "captured_by_white": white,
            "captured_by_black": black,
        }))
        .ok()
    }

    pub fn state_json(&mut self) -> Vec<u8> {
        self.tick();
        let board = board_json(&self.pos);
        let (status, winner) = match &self.over {
            None => {
                if in_check(&self.pos, self.pos.turn) {
                    ("check".to_string(), None)
                } else {
                    ("ongoing".to_string(), None)
                }
            }
            Some(end) => (end.reason().to_string(), end.winner().map(color_name)),
        };
        let last_move = self
            .moves
            .last()
            .map(|m| serde_json::json!({"from": m.from, "to": m.to, "san": m.san}));
        serde_json::to_vec(&serde_json::json!({
            "board": board,
            "turn": color_short(self.pos.turn),
            "status": status,
            "winner": winner,
            "in_check": in_check(&self.pos, self.pos.turn),
            "last_move": last_move,
            "moves": self.moves.iter().map(|m| &m.san).collect::<Vec<_>>(),
            "captured_by_white": self.captured_by_white,
            "captured_by_black": self.captured_by_black,
            "clocks": {"w": self.white_ms, "b": self.black_ms, "tc": self.tc_label},
            "halfmove": self.pos.halfmove,
            "repetition": chess_engine::repetition_count(&self.pos),
            "hot": chess_engine::hanging_squares(&self.pos).iter().map(|s| sq_name(*s)).collect::<Vec<_>>(),
            "pinned": chess_engine::pinned_squares(&self.pos).iter().map(|s| sq_name(*s)).collect::<Vec<_>>(),
            "silver": chess_engine::silver_squares(&self.pos).iter().map(|s| sq_name(*s)).collect::<Vec<_>>(),
            "last_result": self.last_result.as_ref().map(|(w, r)| serde_json::json!({
                "winner": w.map(color_name), "reason": r
            })),
        }))
        .unwrap_or_default()
    }
}

/// 64-entry wire board (`"wP"`…`"bK"`, nulls) shared by live state and
/// historical previews so both encode identically.
fn board_json(pos: &Position) -> Vec<serde_json::Value> {
    let mut board: Vec<serde_json::Value> = Vec::with_capacity(64);
    for sq in pos.board.iter() {
        match sq {
            None => board.push(serde_json::Value::Null),
            Some(Piece { kind, color }) => board.push(serde_json::Value::String(format!(
                "{}{}",
                color_short(*color),
                kind.code()
            ))),
        }
    }
    board
}

fn parse_color(s: &str) -> Option<Color> {
    match s.trim().to_lowercase().as_str() {
        "w" | "white" => Some(Color::White),
        "b" | "black" => Some(Color::Black),
        _ => None,
    }
}

fn parse_promo(s: &str) -> Option<Kind> {
    match s.trim().to_uppercase().as_str() {
        "Q" => Some(Kind::Queen),
        "R" => Some(Kind::Rook),
        "B" => Some(Kind::Bishop),
        "N" => Some(Kind::Knight),
        _ => None,
    }
}

// ── Entry ─────────────────────────────────────────────────────────────────

pub fn cmd_serve_chess(args: &str, _nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if super::serve::is_help_request(&tokens) {
        print_chess_help();
        return Ok(false);
    }

    let mut port = Config::global_get_serve_port(ServeService::Chess);
    let mut bind = Config::global_get_serve_bind(ServeService::Chess);
    let mut use_tunnel = false;
    let mut share_mode = false;
    let mut share_path: Option<String> = None;
    let mut no_qr = false;
    let mut want_qr = false;
    let mut web_dir = String::new();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].as_str() {
            "help" | "--help" | "-h" => {
                print_chess_help();
                return Ok(false);
            }
            "-p" | "--port" => {
                i += 1;
                port = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_port")))?
                    .parse()
                    .map_err(|_| anyhow::anyhow!("{}", t!("cmd.serve.invalid_port")))?;
            }
            "-b" | "--bind" => {
                i += 1;
                bind = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_bind")))?
                    .clone();
            }
            "-t" | "--tunnel" => use_tunnel = true,
            "--share" => {
                share_mode = true;
                if let Some(next) = tokens.get(i + 1) {
                    if !next.starts_with('-') {
                        share_path = Some(next.clone());
                        i += 1;
                    }
                }
            }
            "--qr" => {
                want_qr = true;
                share_mode = true;
            }
            "--no-qr" => no_qr = true,
            "-w" | "--web-dir" => {
                i += 1;
                web_dir = tokens
                    .get(i)
                    .ok_or_else(|| anyhow::anyhow!("{}", t!("cmd.serve.missing_dir")))?
                    .clone();
            }
            "--auth" => {
                anyhow::bail!("{}", t!("cmd.serve.chess_no_auth"));
            }
            "--spa" => {
                anyhow::bail!(
                    "{} (`serve chess` is always an SPA)",
                    t!("cmd.serve.unknown_option", tokens[i])
                );
            }
            "--" => break,
            other => {
                if let Ok(p) = other.parse::<u16>() {
                    port = p;
                } else if other.starts_with('-') {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                } else {
                    anyhow::bail!("{}", t!("cmd.serve.unknown_option", other));
                }
            }
        }
        i += 1;
    }

    let db = Arc::new(Mutex::new(ensure_chess_db()?));
    let default_tc = {
        let guard = db.lock().unwrap_or_else(|e| e.into_inner());
        db_setting(&guard, "chess.tc")
    };
    let sessions = Arc::new(ChessSessions::new(default_tc));

    let addr = format!("{}:{}", bind, port);
    let listener = TcpListener::bind(&addr).context(format!("Bind to {}", addr))?;
    print_info(&format!(
        "{} ({})",
        t!("cmd.serve.chess_serving"),
        format!("http://{}", addr).cyan()
    ));
    if share_mode {
        print_share_block(port, &bind, share_path.as_deref(), !no_qr || want_qr);
    }
    if use_tunnel {
        spawn_cloudflared(port);
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    crate::shell::commands::init_ctrlc_handler();
    crate::shell::commands::register_shutdown_flag(shutdown.clone());
    listener.set_nonblocking(true)?;

    let web = if web_dir.is_empty() {
        None
    } else {
        Some(PathBuf::from(web_dir))
    };
    while !shutdown.load(std::sync::atomic::Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                let sessions = sessions.clone();
                let db = db.clone();
                let web = web.clone();
                let shutdown = shutdown.clone();
                std::thread::spawn(move || {
                    handle_chess_connection(stream, sessions, db, web, shutdown)
                });
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            Err(e) => print_error(&format!("Accept: {}", e)),
        }
    }
    print_info(&t!("cmd.serve.stopped"));
    crate::shell::commands::unregister_shutdown_flag(&shutdown);
    crate::shell::commands::init_ctrlc_handler();
    Ok(false)
}

// ── Connection ────────────────────────────────────────────────────────────

fn read_body_capped(
    reader: &mut BufReader<TcpStream>,
    len: u64,
) -> std::io::Result<Option<Vec<u8>>> {
    const CAP: u64 = 64 * 1024;
    if len > CAP {
        return Ok(None);
    }
    // Framed by Content-Length; the socket-wide idle timeout applies
    // (64 KB bodies never need the old 30 s allowance).
    let mut body = vec![0u8; len as usize];
    reader.read_exact(&mut body)?;
    Ok(Some(body))
}

fn log_access(peer: &str, method: &str, full_path: &str, status: u16) {
    println!("{} - {} {} {}", peer, method, full_path, status);
}

/// Rewrite our `Connection: close` header for keep-alive responses.
/// Every chess response is built by `response_bytes`/`respond`/the HEAD
/// helper, which all emit exactly that literal — anything else passes
/// through untouched.
fn set_connection(resp: Vec<u8>, keep: bool) -> Vec<u8> {
    if !keep {
        return resp;
    }
    const CLOSE: &[u8] = b"Connection: close";
    const KEEP: &[u8] = b"Connection: keep-alive";
    let mut resp = resp;
    if let Some(i) = resp.windows(CLOSE.len()).position(|w| w == CLOSE) {
        resp.splice(i..i + CLOSE.len(), KEEP.iter().copied());
    }
    resp
}

// Keep-alive bounds: one thread per connection, so idle sockets must die
// and no socket lives forever.
const KEEPALIVE_MAX_REQUESTS: u32 = 100;
const KEEPALIVE_IDLE_SECS: u64 = 15;

fn handle_chess_connection(
    stream: TcpStream,
    sessions: Arc<ChessSessions>,
    db: Arc<Mutex<rusqlite::Connection>>,
    web: Option<PathBuf>,
    shutdown: Arc<AtomicBool>,
) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(KEEPALIVE_IDLE_SECS)));
    let peer = stream
        .peer_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "-".to_string());
    // Owned reader, kept across loop iterations so read-ahead is never
    // discarded; writes go through `reader.get_mut()`.
    let mut reader = BufReader::new(stream);
    let mut served: u32 = 0;

    loop {
        // Ctrl+C stops the server after the in-flight response, instead
        // of parking this thread for another idle window.
        if shutdown.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }
        let mut request_line = String::new();
        match reader.read_line(&mut request_line) {
            Err(_) => return, // idle timeout / reset
            Ok(0) => return,  // clean EOF
            Ok(_) => {}
        }
        let request_line = request_line.trim().to_string();
        if request_line.is_empty() {
            return;
        }
        let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
        if parts.len() < 2 {
            return;
        }
        let method = parts[0].to_string();
        let full_path = parts[1].to_string();
        let version = parts.get(2).copied().unwrap_or("");
        let (path, query) = full_path
            .split_once('?')
            .unwrap_or((full_path.as_str(), ""));
        let path = path.to_string();
        let query = query.to_string();

        let mut content_length: u64 = 0;
        let mut cookie: Option<String> = None;
        let mut connection = String::new();
        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() {
                return;
            }
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                if k.trim().eq_ignore_ascii_case("content-length") {
                    content_length = v.trim().parse().unwrap_or(0);
                }
                if k.trim().eq_ignore_ascii_case("cookie") {
                    cookie = Some(v.trim().to_string());
                }
                if k.trim().eq_ignore_ascii_case("connection") {
                    connection = v.trim().to_string();
                }
            }
        }

        // Only POST routes carry a (small JSON) body.
        let mut body = Vec::new();
        if method == "POST" {
            match read_body_capped(&mut reader, content_length) {
                Ok(Some(b)) => body = b,
                Ok(None) => {
                    super::serve::respond(
                        reader.get_mut(),
                        413,
                        "text/plain",
                        b"413 Payload Too Large",
                    );
                    log_access(&peer, &method, &full_path, 413);
                    return;
                }
                Err(_) => {
                    super::serve::respond(reader.get_mut(), 400, "text/plain", b"400 Bad Request");
                    log_access(&peer, &method, &full_path, 400);
                    return;
                }
            }
        }

        // HTTP/1.1 defaults to keep-alive; explicit `close` (or HTTP/1.0
        // without `keep-alive`) closes. Error paths above always close:
        // their framing can't be trusted for a second request.
        let conn_lower = connection.to_lowercase();
        let mut keep = if conn_lower.contains("close") {
            false
        } else if conn_lower.contains("keep-alive") {
            true
        } else {
            version.eq_ignore_ascii_case("HTTP/1.1")
        };
        served += 1;
        if served >= KEEPALIVE_MAX_REQUESTS {
            keep = false;
        }

        let is_api = path.starts_with("/api/chess/");
        let (status, bytes) = if is_api {
            // Per-device session: each browser owns an isolated hotseat game.
            let (game, new_cookie) =
                sessions.session_for(cookie.as_deref().and_then(cookie_session_id));
            let mut bytes = route_chess_api(&method, &path, &query, &body, &game, &db);
            if let Some(id) = new_cookie {
                bytes = inject_cookie(bytes, &id);
            }
            // Status is embedded as the first 3 bytes? No — route returns full
            // response bytes; extract status from the status line for logging.
            let status = bytes
                .split(|b| *b == b'\n')
                .next()
                .and_then(|l| {
                    String::from_utf8_lossy(l)
                        .split_whitespace()
                        .nth(1)?
                        .parse()
                        .ok()
                })
                .unwrap_or(200);
            (status, set_connection(bytes, keep))
        } else if method != "GET" && method != "HEAD" {
            (
                405,
                set_connection(
                    super::serve::response_bytes(
                        405,
                        "text/plain",
                        b"405 Method Not Allowed".to_vec(),
                    ),
                    keep,
                ),
            )
        } else {
            let is_head = method == "HEAD";
            let bytes = serve_chess_static(&path, &web, is_head);
            let status = bytes
                .split(|b| *b == b'\n')
                .next()
                .and_then(|l| {
                    String::from_utf8_lossy(l)
                        .split_whitespace()
                        .nth(1)?
                        .parse()
                        .ok()
                })
                .unwrap_or(200);
            (status, set_connection(bytes, keep))
        };
        let _ = write_all_retry(reader.get_mut(), &bytes);
        log_access(&peer, &method, &full_path, status);
        if !keep {
            return;
        }
    }
}

fn lock_game(game: &Arc<Mutex<Game>>) -> std::sync::MutexGuard<'_, Game> {
    game.lock().unwrap_or_else(|e| e.into_inner())
}

fn lock_db(
    db: &Arc<Mutex<rusqlite::Connection>>,
) -> std::sync::MutexGuard<'_, rusqlite::Connection> {
    db.lock().unwrap_or_else(|e| e.into_inner())
}

fn route_chess_api(
    method: &str,
    path: &str,
    query: &str,
    body: &[u8],
    game: &Arc<Mutex<Game>>,
    db: &Arc<Mutex<rusqlite::Connection>>,
) -> Vec<u8> {
    match (method, path) {
        ("GET", "/api/chess/state") => {
            let bytes = lock_game(game).state_json();
            json_ok(bytes)
        }
        ("GET", "/api/chess/moves") => {
            let q = parse_query(query);
            let sq = q.get("sq").and_then(|s| parse_sq(s));
            match sq {
                None => json_error(400),
                Some(s) => {
                    let mut g = lock_game(game);
                    g.tick();
                    let targets: Vec<serde_json::Value> = legal_moves_for(&g.pos, s)
                        .iter()
                        .map(|m| {
                            let capture = matches!(g.pos.board[m.to as usize], Some(p) if p.color != g.pos.turn)
                                || (g.pos.board[s as usize]
                                    .map(|p| p.kind == Kind::Pawn)
                                    .unwrap_or(false)
                                    && g.pos.ep == Some(m.to)
                                    && g.pos.board[m.to as usize].is_none());
                            serde_json::json!({
                                "to": sq_name(m.to),
                                "promo": m.promo.is_some(),
                                "capture": capture,
                            })
                        })
                        .collect();
                    json_ok(serde_json::to_vec(&serde_json::json!({"moves": targets})).unwrap())
                }
            }
        }
        ("GET", "/api/chess/settings") => {
            let conn = lock_db(db);
            let mut map = serde_json::Map::new();
            for (k, _) in SETTING_DEFAULTS {
                map.insert(
                    k.to_string(),
                    serde_json::Value::String(db_setting(&conn, k)),
                );
            }
            json_ok(serde_json::to_vec(&map).unwrap())
        }
        ("POST", "/api/chess/move") => {
            let v: serde_json::Value = match serde_json::from_slice(body) {
                Ok(v) => v,
                Err(_) => return json_error(400),
            };
            let from = v.get("from").and_then(|x| x.as_str()).and_then(parse_sq);
            let to = v.get("to").and_then(|x| x.as_str()).and_then(parse_sq);
            let promo = v
                .get("promo")
                .and_then(|x| x.as_str())
                .and_then(parse_promo);
            match (from, to) {
                (Some(f), Some(t)) => {
                    let mut g = lock_game(game);
                    match g.do_move(f, t, promo) {
                        Ok((san, sfx)) => {
                            let mut state: serde_json::Value =
                                serde_json::from_slice(&g.state_json()).unwrap_or_default();
                            state["san"] = serde_json::Value::String(san);
                            state["sfx"] = serde_json::Value::String(sfx.to_string());
                            json_ok(serde_json::to_vec(&state).unwrap())
                        }
                        Err(code) => json_ok(
                            serde_json::to_vec(&serde_json::json!({"error": code})).unwrap(),
                        ),
                    }
                }
                _ => json_error(400),
            }
        }
        ("POST", "/api/chess/new") => {
            let v: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();
            let tc = v
                .get("tc")
                .and_then(|x| x.as_str())
                .unwrap_or("10+0")
                .to_string();
            let mut g = lock_game(game);
            match g.reset(&tc) {
                Ok(abandoned) => {
                    let mut state: serde_json::Value =
                        serde_json::from_slice(&g.state_json()).unwrap_or_default();
                    state["abandoned"] = serde_json::Value::Bool(abandoned);
                    json_ok(serde_json::to_vec(&state).unwrap())
                }
                Err(e) => json_ok(
                    serde_json::to_vec(&serde_json::json!({"error": e.to_string()})).unwrap(),
                ),
            }
        }
        ("POST", "/api/chess/resign") => {
            let v: serde_json::Value = serde_json::from_slice(body).unwrap_or_default();
            let color = v
                .get("color")
                .and_then(|x| x.as_str())
                .and_then(parse_color);
            let mut g = lock_game(game);
            match color {
                Some(c) => match g.resign(c) {
                    Ok(()) => json_ok(g.state_json()),
                    Err(code) => {
                        json_ok(serde_json::to_vec(&serde_json::json!({"error": code})).unwrap())
                    }
                },
                None => json_error(400),
            }
        }
        ("POST", "/api/chess/draw") => {
            let mut g = lock_game(game);
            match g.draw_agree() {
                Ok(()) => json_ok(g.state_json()),
                Err(code) => {
                    json_ok(serde_json::to_vec(&serde_json::json!({"error": code})).unwrap())
                }
            }
        }
        ("POST", "/api/chess/undo") => {
            let mut g = lock_game(game);
            match g.undo() {
                Ok(undone) => {
                    let mut state: serde_json::Value =
                        serde_json::from_slice(&g.state_json()).unwrap_or_default();
                    state["undone"] = serde_json::json!({
                        "from": undone.from,
                        "to": undone.to,
                        "san": undone.san,
                    });
                    json_ok(serde_json::to_vec(&state).unwrap())
                }
                Err(code) => {
                    json_ok(serde_json::to_vec(&serde_json::json!({"error": code})).unwrap())
                }
            }
        }
        ("GET", "/api/chess/position") => {
            let q = parse_query(query);
            let g = lock_game(game);
            match q
                .get("ply")
                .and_then(|s| s.parse::<usize>().ok())
                .and_then(|k| g.position_json(k))
            {
                Some(bytes) => json_ok(bytes),
                None => json_error(400),
            }
        }
        ("POST", "/api/chess/rematch") => {
            let tc = {
                let g = lock_game(game);
                g.tc_label.clone()
            };
            let mut g = lock_game(game);
            match g.reset(&tc) {
                Ok(abandoned) => {
                    let mut state: serde_json::Value =
                        serde_json::from_slice(&g.state_json()).unwrap_or_default();
                    state["abandoned"] = serde_json::Value::Bool(abandoned);
                    json_ok(serde_json::to_vec(&state).unwrap())
                }
                Err(e) => json_ok(
                    serde_json::to_vec(&serde_json::json!({"error": e.to_string()})).unwrap(),
                ),
            }
        }
        ("POST", "/api/chess/settings") => {
            let v: serde_json::Value = match serde_json::from_slice(body) {
                Ok(v) => v,
                Err(_) => return json_error(400),
            };
            let key = v.get("key").and_then(|x| x.as_str()).unwrap_or("");
            let value = v.get("value").and_then(|x| x.as_str()).unwrap_or("");
            if !is_setting_key(key) {
                return json_error(400);
            }
            if key.starts_with("sfx.") {
                match value.parse::<u32>() {
                    Ok(n) if n <= 100 => {}
                    _ => return json_error(400),
                }
            }
            let conn = lock_db(db);
            match db_set_setting(&conn, key, value) {
                Ok(()) => json_ok(b"{\"ok\":true}".to_vec()),
                Err(_) => json_error(500),
            }
        }
        _ => json_error(404),
    }
}

// ── Static ────────────────────────────────────────────────────────────────

fn serve_chess_static(path: &str, web_dir: &Option<PathBuf>, is_head: bool) -> Vec<u8> {
    fn head_only(body_len: usize, ct: &str) -> Vec<u8> {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
            ct, body_len
        )
        .into_bytes()
    }

    let clean = if path == "/" {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    let mut hit: Option<(Vec<u8>, String)> = None;
    if let Some(dir) = web_dir {
        let fp = dir.join(clean);
        if fp.is_file() {
            if let Ok(d) = std::fs::read(&fp) {
                let ct = mime_guess::from_path(&fp)
                    .first_or_octet_stream()
                    .to_string();
                hit = Some((d, ct));
            }
        }
    }
    if hit.is_none() {
        if let Some(f) = ChessFrontend::get(clean) {
            let ct = mime_guess::from_path(clean)
                .first_or_octet_stream()
                .to_string();
            hit = Some((f.data.to_vec(), ct));
        }
    }
    // SPA fallback: unknown paths serve the app shell.
    if hit.is_none() {
        if let Some(f) = ChessFrontend::get("index.html") {
            hit = Some((f.data.to_vec(), "text/html; charset=utf-8".to_string()));
        }
    }
    match hit {
        Some((data, ct)) => {
            if is_head {
                head_only(data.len(), &ct)
            } else {
                response_bytes(200, &ct, data)
            }
        }
        None => super::serve::response_bytes(404, "text/plain", b"404 Not Found".to_vec()),
    }
}

// ── Help ───────────────────────────────────────────────────────────────────

fn print_chess_help() {
    println!("{}", t!("cmd.serve.chess_title").bold());
    println!();
    println!("{}", t!("cmd.serve.usage_title").cyan().bold());
    println!("  {}", t!("cmd.serve.chess_usage"));
    println!();
    println!("{}", t!("cmd.serve.options_title").cyan().bold());
    println!("  {}", t!("cmd.serve.help_port"));
    println!("  {}", t!("cmd.serve.help_bind"));
    println!("  {}", t!("cmd.serve.help_tunnel"));
    println!("  {}", t!("cmd.serve.help_share"));
    println!("  {}", t!("cmd.serve.help_qr"));
    println!("  {}", t!("cmd.serve.help_no_qr"));
    println!("  {}", t!("cmd.serve.help_web_dir"));
    println!("  {}", t!("cmd.serve.help_help"));
    println!();
    println!("{}", t!("cmd.serve.examples_title").cyan().bold());
    println!("  {}", t!("cmd.serve.example_chess_default"));
    println!("  {}", t!("cmd.serve.example_chess_port"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Game {
        Game::new("10+0").unwrap()
    }

    #[test]
    fn full_scholars_mate_flow() {
        let mut g = game();
        for (f, t) in [
            ("e2", "e4"),
            ("e7", "e5"),
            ("d1", "h5"),
            ("b8", "c6"),
            ("f1", "c4"),
            ("g8", "f6"),
        ] {
            let (san, _) = g
                .do_move(parse_sq(f).unwrap(), parse_sq(t).unwrap(), None)
                .unwrap();
            let _ = san;
        }
        let (san, sfx) = g
            .do_move(parse_sq("h5").unwrap(), parse_sq("f7").unwrap(), None)
            .unwrap();
        assert_eq!(san, "Qxf7#");
        assert_eq!(sfx, "win");
        assert!(g.is_over());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["status"], "checkmate");
        assert_eq!(state["winner"], "white");
        assert_eq!(state["moves"].as_array().unwrap().len(), 7);
    }

    #[test]
    fn illegal_and_promotion_required() {
        let mut g = game();
        assert!(g
            .do_move(parse_sq("e2").unwrap(), parse_sq("e5").unwrap(), None)
            .is_err());
        // Wrong turn (black pawn on white's move).
        assert!(g
            .do_move(parse_sq("e7").unwrap(), parse_sq("e5").unwrap(), None)
            .is_err());
        // Empty square.
        assert!(g
            .do_move(parse_sq("e3").unwrap(), parse_sq("e4").unwrap(), None)
            .is_err());
    }

    #[test]
    fn resign_draw_rematch() {
        let mut g = game();
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        assert!(g.resign(Color::Black).is_ok());
        assert!(g.is_over());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["status"], "resign");
        assert_eq!(state["winner"], "white");
        // Moves rejected after game over.
        assert!(g
            .do_move(parse_sq("e7").unwrap(), parse_sq("e5").unwrap(), None)
            .is_err());

        let mut g = game();
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        assert!(g.draw_agree().is_ok());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["status"], "agreement");
        assert!(state["winner"].is_null());

        // Rematch resets with the same clock.
        assert!(g.reset("3+2").is_ok());
        assert!(!g.is_over());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["moves"].as_array().unwrap().len(), 0);
        assert_eq!(state["clocks"]["tc"], "3+2");
    }

    #[test]
    fn undo_restores_everything() {
        let mut g = game();
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        g.do_move(parse_sq("e7").unwrap(), parse_sq("e5").unwrap(), None)
            .unwrap();
        // e4 pawn takes... play Bc4 then undo twice.
        g.do_move(parse_sq("f1").unwrap(), parse_sq("c4").unwrap(), None)
            .unwrap();
        let undone = g.undo().unwrap();
        assert_eq!((undone.from.as_str(), undone.to.as_str()), ("f1", "c4"));
        assert_eq!(undone.san, "Bc4");
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["moves"].as_array().unwrap().len(), 2);
        assert_eq!(state["turn"], "w");
        // Board: bishop back home on f1.
        let board = state["board"].as_array().unwrap();
        assert_eq!(board[parse_sq("f1").unwrap() as usize], "wB");
        assert_eq!(
            board[parse_sq("c4").unwrap() as usize],
            serde_json::Value::Null
        );
        // Second undo pops e5.
        let undone = g.undo().unwrap();
        assert_eq!(undone.san, "e5");
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["moves"].as_array().unwrap().len(), 1);
        assert_eq!(state["turn"], "b");
    }

    #[test]
    fn undo_empty_errors_and_reset_clears_history() {
        let mut g = game();
        assert!(g.undo().is_err());
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        assert!(g.reset("10+0").is_ok());
        assert!(g.undo().is_err());
    }

    #[test]
    fn undo_after_mate_resurrects() {
        let mut g = game();
        for (f, t) in [
            ("e2", "e4"),
            ("e7", "e5"),
            ("d1", "h5"),
            ("b8", "c6"),
            ("f1", "c4"),
            ("g8", "f6"),
        ] {
            g.do_move(parse_sq(f).unwrap(), parse_sq(t).unwrap(), None)
                .unwrap();
        }
        g.do_move(parse_sq("h5").unwrap(), parse_sq("f7").unwrap(), None)
            .unwrap();
        assert!(g.is_over());
        let undone = g.undo().unwrap();
        assert_eq!(undone.san, "Qxf7#");
        assert!(!g.is_over());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["status"], "ongoing");
        assert!(state["winner"].is_null());
        assert_eq!(state["moves"].as_array().unwrap().len(), 6);
    }

    #[test]
    fn position_json_walks_history() {
        let mut g = game();
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        g.do_move(parse_sq("e7").unwrap(), parse_sq("e5").unwrap(), None)
            .unwrap();
        // Ply 0 = initial: e-pawns home, white to move, no last move.
        let p0: serde_json::Value = serde_json::from_slice(&g.position_json(0).unwrap()).unwrap();
        assert_eq!(p0["turn"], "w");
        assert!(p0["last_move"].is_null());
        let b0 = p0["board"].as_array().unwrap();
        assert_eq!(b0[parse_sq("e2").unwrap() as usize], "wP");
        // Ply 1 = after e4: e2 empty, e4 pawn, black to move.
        let p1: serde_json::Value = serde_json::from_slice(&g.position_json(1).unwrap()).unwrap();
        assert_eq!(p1["turn"], "b");
        assert_eq!(p1["last_move"]["san"], "e4");
        assert_eq!(p1["last_move"]["from"], "e2");
        let b1 = p1["board"].as_array().unwrap();
        assert_eq!(
            b1[parse_sq("e2").unwrap() as usize],
            serde_json::Value::Null
        );
        assert_eq!(b1[parse_sq("e4").unwrap() as usize], "wP");
        // Ply 2 = live position (equals current board).
        let p2: serde_json::Value = serde_json::from_slice(&g.position_json(2).unwrap()).unwrap();
        assert_eq!(p2["last_move"]["san"], "e5");
        let live: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(p2["board"], live["board"]);
        // Out of range, and history shrinks on undo.
        assert!(g.position_json(3).is_none());
        g.undo().unwrap();
        assert!(g.position_json(2).is_none());
        let p1b: serde_json::Value = serde_json::from_slice(&g.position_json(1).unwrap()).unwrap();
        assert_eq!(p1b["last_move"]["san"], "e4");
        // Reset clears history: only ply 0 (== fresh initial) remains.
        assert!(g.reset("10+0").is_ok());
        assert!(g.position_json(1).is_none());
        let p0b: serde_json::Value = serde_json::from_slice(&g.position_json(0).unwrap()).unwrap();
        assert_eq!(p0b["board"].as_array().unwrap(), b0);
    }

    #[test]
    fn new_game_abandons_live_game() {
        let mut g = game();
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        // Black to move abandons → white wins by abandonment.
        let abandoned = g.reset("10+0").unwrap();
        assert!(abandoned);
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["last_result"]["winner"], "white");
        assert_eq!(state["last_result"]["reason"], "abandoned");
    }

    #[test]
    fn bad_time_controls_rejected() {
        assert!(Game::new("0+0").is_err());
        assert!(Game::new("999+0").is_err());
        assert!(Game::new("abc").is_err());
        assert!(Game::new("unlimited").is_ok());
        assert!(Game::new("1+0").is_ok());
    }

    #[test]
    fn flag_fall_ends_game() {
        let mut g = Game::new("1+0").unwrap();
        // Shrink white's clock to ~0 and let it elapse on black's... force
        // white to move with (near-)zero time.
        g.white_ms = Some(1);
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        g.black_ms = Some(1);
        std::thread::sleep(std::time::Duration::from_millis(15));
        g.tick();
        assert!(g.is_over());
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["status"], "timeout");
        assert_eq!(state["winner"], "white");
    }

    #[test]
    fn state_json_carries_hot_squares() {
        let mut g = game();
        // Initial position: nothing hanging, nothing pinned.
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert_eq!(state["hot"].as_array().unwrap().len(), 0);
        assert_eq!(state["pinned"].as_array().unwrap().len(), 0);
        // After 1. e4, the e4 pawn is defended... still nothing hanging
        // in a sound opening (smoke: field present and well-formed).
        g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
            .unwrap();
        let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
        assert!(state["hot"].is_array());
    }

    #[test]
    fn settings_db_defaults_and_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("chess.db");
        let conn = ensure_chess_db_at(&path).unwrap();
        assert_eq!(db_setting(&conn, "sfx.move"), "80");
        assert_eq!(db_setting(&conn, "sfx.check"), "80");
        assert_eq!(db_setting(&conn, "chess.tc"), "10+0");
        db_set_setting(&conn, "sfx.move", "42").unwrap();
        assert_eq!(db_setting(&conn, "sfx.move"), "42");
        assert!(!is_setting_key("nope"));
        assert!(is_setting_key("sfx.win"));
        assert!(is_setting_key("sfx.check"));
    }

    #[test]
    fn parse_helpers() {
        assert_eq!(parse_color("w"), Some(Color::White));
        assert_eq!(parse_color("Black"), Some(Color::Black));
        assert_eq!(parse_color("x"), None);
        assert_eq!(parse_promo("q"), Some(Kind::Queen));
        assert_eq!(parse_promo("n"), Some(Kind::Knight));
        assert_eq!(parse_promo("x"), None);
    }

    #[test]
    fn cookie_session_id_parses_strictly() {
        assert_eq!(
            cookie_session_id("ntc_chess=abc123"),
            Some("abc123".to_string())
        );
        // Mixed jar: picks ours, ignores others; quotes tolerated.
        assert_eq!(
            cookie_session_id("a=1; ntc_chess=\"AbC9\"; b=2"),
            Some("AbC9".to_string())
        );
        assert_eq!(cookie_session_id("a=1; b=2"), None);
        assert_eq!(cookie_session_id(""), None);
        // Malformed values mint fresh sessions instead of matching.
        assert_eq!(cookie_session_id("ntc_chess="), None);
        assert_eq!(cookie_session_id("ntc_chess=../../etc"), None);
        assert_eq!(cookie_session_id("ntc_chess=has space"), None);
    }

    #[test]
    fn inject_cookie_preserves_response() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 7\r\nConnection: close\r\n\r\n{\"a\":1}".to_vec();
        let out = inject_cookie(raw, "abc123");
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("Set-Cookie: ntc_chess=abc123; Path=/; SameSite=Lax\r\n"));
        assert!(text.ends_with("{\"a\":1}"));
        // Status-line log parser still sees 200 (first line untouched).
        let first = text.split('\n').next().unwrap_or("");
        assert!(first.contains("200"));
        // No blank line → untouched.
        let bare = b"garbage".to_vec();
        assert_eq!(inject_cookie(bare.clone(), "x"), bare);
    }

    #[test]
    fn sessions_isolate_devices() {
        let store = ChessSessions::new("10+0".to_string());
        // No cookie → fresh session + id to set.
        let (_a, id_a) = store.session_for(None);
        let id_a = id_a.expect("first visit must mint");
        assert_eq!(id_a.len(), 32);
        // Same cookie → same game, no new id.
        let (a2, none) = store.session_for(Some(id_a.clone()));
        assert!(none.is_none());
        // A move in A is invisible to a fresh device B.
        {
            let mut g = a2.lock().unwrap_or_else(|e| e.into_inner());
            g.do_move(parse_sq("e2").unwrap(), parse_sq("e4").unwrap(), None)
                .unwrap();
        }
        let (b, id_b) = store.session_for(None);
        assert!(id_b.is_some());
        assert_ne!(id_a, id_b.unwrap());
        {
            let mut g = b.lock().unwrap_or_else(|e| e.into_inner());
            let state: serde_json::Value = serde_json::from_slice(&g.state_json()).unwrap();
            // BUG repro: before sessions, B would have seen A's e4.
            assert_eq!(state["moves"].as_array().unwrap().len(), 0);
        }
        // Unknown-but-wellformed id → fresh session, not an error.
        let (_, id_c) = store.session_for(Some("deadbeefdeadbeefdeadbeefdeadbeef".to_string()));
        assert!(id_c.is_some());
        // Malformed id → fresh session.
        let (_, id_d) = store.session_for(Some("../../x".to_string()));
        assert!(id_d.is_some());
    }

    #[test]
    fn sessions_expire_and_cap() {
        let store = ChessSessions::new("10+0".to_string());
        let (_, id) = store.session_for(None);
        let id = id.unwrap();
        // Age it past TTL directly (same module: private fields visible).
        {
            let mut map = store.lock();
            let entry = map.get_mut(&id).unwrap();
            entry.last_seen = Instant::now() - SESSION_TTL - std::time::Duration::from_secs(1);
        }
        // Expired id mints fresh (old game dropped).
        let (_, fresh) = store.session_for(Some(id.clone()));
        assert!(fresh.is_some());
        assert_ne!(fresh.unwrap(), id);
    }

    /// Read one framed response: status line, headers, exact body.
    fn read_one(sock: &mut std::net::TcpStream) -> (String, Vec<(String, String)>, Vec<u8>) {
        use std::io::{BufRead, BufReader, Read};
        let mut status = String::new();
        let mut headers = Vec::new();
        let mut body = Vec::new();
        {
            let mut r = BufReader::new(&mut *sock);
            r.read_line(&mut status).unwrap();
            loop {
                let mut line = String::new();
                r.read_line(&mut line).unwrap();
                let line = line.trim().to_string();
                if line.is_empty() {
                    break;
                }
                if let Some((k, v)) = line.split_once(':') {
                    headers.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
            let len: usize = headers
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
                .and_then(|(_, v)| v.parse().ok())
                .unwrap_or(0);
            body.resize(len, 0);
            r.read_exact(&mut body).unwrap();
        }
        (status, headers, body)
    }

    fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
        headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    #[test]
    fn keepalive_reuses_one_socket_then_closes() {
        use std::io::Write;
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let sessions = Arc::new(ChessSessions::new("10+0".to_string()));
        let dir = tempfile::tempdir().unwrap();
        let db = Arc::new(Mutex::new(
            ensure_chess_db_at(&dir.path().join("c.db")).unwrap(),
        ));
        let shutdown = Arc::new(AtomicBool::new(false));
        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_chess_connection(stream, sessions, db, None, shutdown);
        });

        let mut sock = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        sock.set_read_timeout(Some(std::time::Duration::from_secs(10)))
            .unwrap();
        let send = |sock: &mut std::net::TcpStream, raw: &str| {
            sock.write_all(raw.as_bytes()).unwrap();
        };

        // 1. Fresh state over keep-alive: 200 + keep-alive echo + session cookie.
        send(
            &mut sock,
            "GET /api/chess/state HTTP/1.1\r\nHost: x\r\nConnection: keep-alive\r\n\r\n",
        );
        let (st, hd, body) = read_one(&mut sock);
        assert!(st.contains("200"), "got: {}", st);
        assert_eq!(header(&hd, "connection"), Some("keep-alive"));
        let cookie = header(&hd, "set-cookie").expect("new session must set cookie");
        assert!(cookie.starts_with("ntc_chess="));
        let id = cookie
            .split(';')
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .to_string();
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["moves"].as_array().unwrap().len(), 0);

        // 2. Same socket, move with the cookie: session continuity.
        let mv = "{\"from\":\"e2\",\"to\":\"e4\"}";
        send(
            &mut sock,
            &format!(
                "POST /api/chess/move HTTP/1.1\r\nHost: x\r\nContent-Length: {}\r\nConnection: keep-alive\r\nCookie: ntc_chess={}\r\n\r\n{}",
                mv.len(),
                id,
                mv
            ),
        );
        let (st, _, body) = read_one(&mut sock);
        assert!(st.contains("200"), "got: {}", st);
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["san"], "e4");

        // 3. Same socket, state shows the move (no Set-Cookie repeat).
        send(
            &mut sock,
            &format!(
                "GET /api/chess/state HTTP/1.1\r\nHost: x\r\nConnection: keep-alive\r\nCookie: ntc_chess={}\r\n\r\n",
                id
            ),
        );
        let (st, hd, body) = read_one(&mut sock);
        assert!(st.contains("200"), "got: {}", st);
        assert!(hd
            .iter()
            .all(|(k, _)| !k.eq_ignore_ascii_case("set-cookie")));
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["moves"].as_array().unwrap().len(), 1);

        // 4. Explicit close: 200 + close echo, then EOF.
        send(
            &mut sock,
            &format!(
                "GET /api/chess/state HTTP/1.1\r\nHost: x\r\nConnection: close\r\nCookie: ntc_chess={}\r\n\r\n",
                id
            ),
        );
        let (st, hd, _) = read_one(&mut sock);
        assert!(st.contains("200"), "got: {}", st);
        assert_eq!(header(&hd, "connection"), Some("close"));
        let mut one = [0u8; 1];
        use std::io::Read as _;
        match sock.read(&mut one) {
            Ok(0) => {}
            Err(_) => {}
            Ok(_) => panic!("server must close after Connection: close"),
        }
        handle.join().unwrap();
    }
}
