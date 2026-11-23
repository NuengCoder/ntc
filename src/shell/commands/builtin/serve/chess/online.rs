//! Online chess rooms for `ntc serve chess --online`.
//!
//! The existing hotseat server stays untouched when `--online` is absent.
//! This module owns guest sessions, the in-memory room registry, and the
//! `/api/online/*` routes. The authoritative chess rules and clocks remain in
//! [`super::Game`].
//!
//! Size note: kept as one file (~2k lines) because guest sessions, the room
//! registry and the route handlers all mutate the same registry under one
//! lock; keeping them together makes the locking and room lifecycle visible in
//! one place.

use super::bot::BotManager;
use super::engine::{self, Color, Kind};
use super::{coach_snapshot, coach_suggestion, Coach, CoachSnapshot, Game};
use crate::shell::commands::support::net::http::{json_ok, parse_query, response_bytes};
use argon2::{Algorithm, Argon2, Params, Version};
use rusqlite::Connection;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::time::{Duration, Instant};

pub const GUEST_COOKIE: &str = "ntc_online";
pub const MAX_ROOMS: usize = 200;
pub const MAX_GUESTS: usize = 500;
pub const COACH_LIMIT: u8 = 3;
const GUEST_TTL_SECS: i64 = 30 * 24 * 60 * 60;
const WAITING_TTL: Duration = Duration::from_secs(15 * 60);
const FINISHED_TTL: Duration = Duration::from_secs(10 * 60);
const DRAW_OFFER_TTL: Duration = Duration::from_secs(30);
const DISCONNECT_GRACE: Duration = Duration::from_secs(60);
const CONNECTED_WINDOW: Duration = Duration::from_secs(15);
const ROOM_NAME_MAX: usize = 40;
const GUEST_NAME_MAX: usize = 20;
const PASSWORD_MIN: usize = 4;
const PASSWORD_MAX: usize = 64;

// Argon2id parameters for room passwords: interactive-safe, matching the
// project's existing vault settings.
const ARGON_M_COST: u32 = 19 * 1024;
const ARGON_T_COST: u32 = 2;
const ARGON_P_COST: u32 = 1;

#[derive(Debug)]
pub struct OnlineError {
    pub status: u16,
    pub code: &'static str,
    pub message: String,
}

impl OnlineError {
    fn new(status: u16, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    fn not_logged_in() -> Self {
        Self::new(401, "not_logged_in", "Choose a guest name first")
    }

    fn not_seated() -> Self {
        Self::new(403, "not_seated", "You are not seated in this room")
    }

    fn room_not_found() -> Self {
        Self::new(404, "room_not_found", "That room is no longer available")
    }

    fn as_response(&self) -> Vec<u8> {
        json_status(
            self.status,
            json!({"error": self.code, "message": self.message}),
        )
    }
}

fn json_status(status: u16, value: Value) -> Vec<u8> {
    response_bytes(
        status,
        "application/json",
        serde_json::to_vec(&value).unwrap_or_default(),
    )
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

fn read_lock<T>(m: &RwLock<T>) -> std::sync::RwLockReadGuard<'_, T> {
    m.read().unwrap_or_else(|e| e.into_inner())
}

fn write_lock<T>(m: &RwLock<T>) -> std::sync::RwLockWriteGuard<'_, T> {
    m.write().unwrap_or_else(|e| e.into_inner())
}

fn token_hash(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

fn new_session_token() -> String {
    let n: u128 = rand::random();
    format!("{:032x}", n)
}

fn now_secs() -> i64 {
    chrono::Utc::now().timestamp()
}

fn valid_token(token: &str) -> bool {
    token.len() == 32 && token.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Extract the online guest token from a `Cookie` header. Independent from
/// the hotseat `ntc_chess` cookie, so both can coexist.
pub fn cookie_token(header: &str) -> Option<String> {
    for part in header.split(';') {
        if let Some((k, v)) = part.split_once('=') {
            if k.trim() == GUEST_COOKIE {
                let token = v.trim().trim_matches('"');
                return if valid_token(token) {
                    Some(token.to_string())
                } else {
                    None
                };
            }
        }
    }
    None
}

/// Splice an arbitrary `Set-Cookie` line into a raw response.
pub fn inject_cookie(resp: Vec<u8>, line: &str) -> Vec<u8> {
    let needle = b"\r\n\r\n";
    match resp.windows(4).position(|w| w == needle) {
        Some(i) => {
            let header = format!("Set-Cookie: {}\r\n", line);
            let mut out = Vec::with_capacity(resp.len() + header.len());
            out.extend_from_slice(&resp[..i + 2]);
            out.extend_from_slice(header.as_bytes());
            out.extend_from_slice(&resp[i + 2..]);
            out
        }
        None => resp,
    }
}

fn guest_cookie(token: &str, secure: bool) -> String {
    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Lax{}",
        GUEST_COOKIE,
        token,
        if secure { "; Secure" } else { "" }
    )
}

fn clear_guest_cookie(secure: bool) -> String {
    format!(
        "{}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0{}",
        GUEST_COOKIE,
        if secure { "; Secure" } else { "" }
    )
}

// ── Names and passwords ───────────────────────────────────────────────────

fn clean_guest_name(raw: &str) -> Option<String> {
    let name = raw.trim();
    if name.is_empty() || name.chars().count() > GUEST_NAME_MAX {
        return None;
    }
    if name.chars().any(|c| c.is_control()) {
        return None;
    }
    Some(name.to_string())
}

fn clean_room_name(raw: &str) -> Option<String> {
    let name = raw.trim();
    if name.is_empty() || name.chars().count() > ROOM_NAME_MAX {
        return None;
    }
    if name.chars().any(|c| c.is_control()) {
        return None;
    }
    Some(name.to_string())
}

fn hash_password(password: &str) -> Result<String, OnlineError> {
    if !(PASSWORD_MIN..=PASSWORD_MAX).contains(&password.chars().count()) {
        return Err(OnlineError::new(
            400,
            "invalid_password",
            "Room password must be 4-64 characters",
        ));
    }
    let params = Params::new(ARGON_M_COST, ARGON_T_COST, ARGON_P_COST, Some(32))
        .map_err(|_| OnlineError::new(500, "hash_failed", "Could not secure the password"))?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let salt: [u8; 16] = rand::random();
    let mut out = [0u8; 32];
    argon
        .hash_password_into(password.as_bytes(), &salt, &mut out)
        .map_err(|_| OnlineError::new(500, "hash_failed", "Could not secure the password"))?;
    Ok(format!(
        "argon2id${}${}",
        hex::encode(salt),
        hex::encode(out)
    ))
}

fn verify_password(password: &str, encoded: &str) -> bool {
    let mut parts = encoded.split('$');
    if parts.next() != Some("argon2id") {
        return false;
    }
    let Some(salt_hex) = parts.next() else {
        return false;
    };
    let Some(hash_hex) = parts.next() else {
        return false;
    };
    let Ok(salt) = hex::decode(salt_hex) else {
        return false;
    };
    let Ok(want) = hex::decode(hash_hex) else {
        return false;
    };
    let Ok(params) = Params::new(ARGON_M_COST, ARGON_T_COST, ARGON_P_COST, Some(want.len())) else {
        return false;
    };
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut got = vec![0u8; want.len()];
    if argon
        .hash_password_into(password.as_bytes(), &salt, &mut got)
        .is_err()
    {
        return false;
    }
    if got.len() != want.len() {
        return false;
    }
    got.iter()
        .zip(want.iter())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

// ── Guest sessions ────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GuestSession {
    pub token_hash: String,
    pub name: String,
    pub expires_at: i64,
}

pub struct OnlineState {
    rooms: RwLock<HashMap<String, Arc<Mutex<OnlineRoom>>>>,
    room_names: Mutex<HashMap<String, String>>,
    guests: Mutex<HashMap<String, GuestSession>>,
    name_index: Mutex<HashMap<String, String>>,
    rates: Mutex<HashMap<String, (Instant, u32)>>,
    db: Arc<Mutex<Connection>>,
    default_tc: String,
}

impl OnlineState {
    pub fn new(db: Arc<Mutex<Connection>>, default_tc: String) -> anyhow::Result<Self> {
        {
            let conn = lock(&db);
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS online_guests (
                    token_hash         TEXT PRIMARY KEY,
                    display_name       TEXT NOT NULL,
                    display_name_lower TEXT NOT NULL UNIQUE,
                    created_at         INTEGER NOT NULL,
                    last_seen          INTEGER NOT NULL,
                    expires_at         INTEGER NOT NULL
                );",
            )?;
        }
        let state = Self {
            rooms: RwLock::new(HashMap::new()),
            room_names: Mutex::new(HashMap::new()),
            guests: Mutex::new(HashMap::new()),
            name_index: Mutex::new(HashMap::new()),
            rates: Mutex::new(HashMap::new()),
            db,
            default_tc,
        };
        state.load_guests()?;
        Ok(state)
    }

    fn load_guests(&self) -> anyhow::Result<()> {
        let conn = lock(&self.db);
        let mut stmt = conn.prepare(
            "SELECT token_hash, display_name, expires_at
             FROM online_guests WHERE expires_at > ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![now_secs()], |row| {
            Ok(GuestSession {
                token_hash: row.get(0)?,
                name: row.get(1)?,
                expires_at: row.get(2)?,
            })
        })?;
        let mut guests = lock(&self.guests);
        let mut names = lock(&self.name_index);
        for row in rows {
            let guest = row?;
            names.insert(guest.name.to_lowercase(), guest.token_hash.clone());
            guests.insert(guest.token_hash.clone(), guest);
        }
        Ok(())
    }

    fn allow(&self, bucket: &str, key: &str, max: u32) -> bool {
        let composite = format!("{}:{}", bucket, key);
        let now = Instant::now();
        let mut map = lock(&self.rates);
        let entry = map.entry(composite).or_insert((now, 0));
        if now.duration_since(entry.0) >= Duration::from_secs(60) {
            *entry = (now, 0);
        }
        if entry.1 >= max {
            return false;
        }
        entry.1 += 1;
        true
    }

    fn guest_for_token(&self, token: Option<&str>) -> Option<GuestSession> {
        let token = token?;
        let hash = token_hash(token);
        let mut expired_name = None;
        {
            let mut guests = lock(&self.guests);
            if let Some(guest) = guests.get(&hash) {
                if guest.expires_at > now_secs() {
                    return Some(guest.clone());
                }
                expired_name = Some(guest.name.to_lowercase());
                guests.remove(&hash);
            }
        }
        if let Some(name) = expired_name {
            lock(&self.name_index).remove(&name);
        }
        // The memory map is the active set. The DB is reloaded at startup.
        None
    }

    fn login(
        &self,
        token: Option<&str>,
        raw_name: &str,
    ) -> Result<(GuestSession, Option<String>), OnlineError> {
        let name = clean_guest_name(raw_name)
            .ok_or_else(|| OnlineError::new(400, "invalid_name", "Name must be 1-20 characters"))?;
        let lower = name.to_lowercase();
        let current = self.guest_for_token(token);

        if let Some(cur) = &current {
            if cur.name.eq_ignore_ascii_case(&name) {
                return Ok((cur.clone(), None));
            }
        }

        if let Some(cur) = current {
            let mut guests = lock(&self.guests);
            let mut names = lock(&self.name_index);
            if let Some(owner) = names.get(&lower) {
                if owner != &cur.token_hash {
                    return Err(OnlineError::new(
                        409,
                        "name_taken",
                        "That guest name is already in use",
                    ));
                }
            }
            let conn = lock(&self.db);
            conn.execute(
                "UPDATE online_guests
                 SET display_name = ?1, display_name_lower = ?2, last_seen = ?3
                 WHERE token_hash = ?4",
                rusqlite::params![name, lower, now_secs(), cur.token_hash],
            )
            .map_err(|_| {
                OnlineError::new(409, "name_taken", "That guest name is already in use")
            })?;
            drop(conn);
            names.remove(&cur.name.to_lowercase());
            names.insert(lower.clone(), cur.token_hash.clone());
            let updated = GuestSession {
                name: name.clone(),
                ..cur
            };
            guests.insert(updated.token_hash.clone(), updated.clone());
            return Ok((updated, None));
        }

        if lock(&self.guests).len() >= MAX_GUESTS {
            self.expire_guests();
            if lock(&self.guests).len() >= MAX_GUESTS {
                return Err(OnlineError::new(
                    429,
                    "guest_limit_reached",
                    "This server is full; try again later",
                ));
            }
        }

        let token = new_session_token();
        let hash = token_hash(&token);
        let expires = now_secs() + GUEST_TTL_SECS;
        {
            let mut guests = lock(&self.guests);
            let mut names = lock(&self.name_index);
            if names.contains_key(&lower) {
                return Err(OnlineError::new(
                    409,
                    "name_taken",
                    "That guest name is already in use",
                ));
            }
            let conn = lock(&self.db);
            conn.execute(
                "INSERT INTO online_guests
                 (token_hash, display_name, display_name_lower, created_at, last_seen, expires_at)
                 VALUES (?1, ?2, ?3, ?4, ?4, ?5)",
                rusqlite::params![hash, name, lower, now_secs(), expires],
            )
            .map_err(|_| {
                OnlineError::new(409, "name_taken", "That guest name is already in use")
            })?;
            names.insert(lower, hash.clone());
            let guest = GuestSession {
                token_hash: hash.clone(),
                name,
                expires_at: expires,
            };
            guests.insert(hash, guest.clone());
            Ok((guest, Some(token)))
        }
    }

    fn logout(&self, guest: &GuestSession) {
        let lower = guest.name.to_lowercase();
        lock(&self.guests).remove(&guest.token_hash);
        lock(&self.name_index).remove(&lower);
        let conn = lock(&self.db);
        let _ = conn.execute(
            "DELETE FROM online_guests WHERE token_hash = ?1",
            rusqlite::params![guest.token_hash],
        );
    }

    fn expire_guests(&self) {
        let now = now_secs();
        let expired: Vec<GuestSession> = {
            let guests = lock(&self.guests);
            guests
                .values()
                .filter(|g| g.expires_at <= now)
                .cloned()
                .collect()
        };
        if expired.is_empty() {
            return;
        }
        {
            let mut guests = lock(&self.guests);
            let mut names = lock(&self.name_index);
            for guest in &expired {
                guests.remove(&guest.token_hash);
                names.remove(&guest.name.to_lowercase());
            }
        }
        let conn = lock(&self.db);
        for guest in expired {
            let _ = conn.execute(
                "DELETE FROM online_guests WHERE token_hash = ?1",
                rusqlite::params![guest.token_hash],
            );
        }
    }

    // ── Room registry ─────────────────────────────────────────────────────

    fn active_room_for(&self, token_hash: &str) -> Option<(String, String)> {
        let rooms = read_lock(&self.rooms);
        let mut waiting = None;
        for (id, room) in rooms.iter() {
            let guard = lock(room);
            let Some(idx) = guard.seat_index(token_hash) else {
                continue;
            };
            if guard.left[idx] {
                continue;
            }
            if guard.status() == RoomStatus::Playing {
                return Some((id.clone(), guard.name.clone()));
            }
            if waiting.is_none() {
                waiting = Some((id.clone(), guard.name.clone()));
            }
        }
        waiting
    }

    fn room_map_get(&self, id: &str) -> Option<Arc<Mutex<OnlineRoom>>> {
        read_lock(&self.rooms).get(id).cloned()
    }

    fn remove_room(&self, id: &str) -> Option<Arc<Mutex<OnlineRoom>>> {
        let removed = {
            let mut rooms = write_lock(&self.rooms);
            rooms.remove(id)
        };
        if let Some(room) = &removed {
            let lower = lock(room).name_lower.clone();
            let mut names = lock(&self.room_names);
            if names.get(&lower).map(|v| v.as_str()) == Some(id) {
                names.remove(&lower);
            }
        }
        removed
    }

    fn leave_other_rooms(&self, token_hash: &str, keep: Option<&str>) -> Result<(), OnlineError> {
        if let Some((id, _)) = self.active_room_for(token_hash) {
            if Some(id.as_str()) != keep {
                let room = self
                    .room_map_get(&id)
                    .ok_or_else(OnlineError::room_not_found)?;
                let is_playing = {
                    let guard = lock(&room);
                    guard.status() == RoomStatus::Playing
                };
                if is_playing {
                    return Err(OnlineError::new(
                        409,
                        "already_in_game",
                        "Finish or leave your current game first",
                    ));
                }
                self.leave_room(token_hash, &id)?;
            }
        }
        Ok(())
    }

    pub fn create_room(
        &self,
        guest: &GuestSession,
        name: &str,
        password: Option<&str>,
        tc: &str,
        side: &str,
        coach_allowed: bool,
    ) -> Result<Value, OnlineError> {
        let name = clean_room_name(name).ok_or_else(|| {
            OnlineError::new(400, "invalid_name", "Room name must be 1-40 characters")
        })?;
        let name_lower = name.to_lowercase();
        self.leave_other_rooms(&guest.token_hash, None)?;

        let password_hash = match password.map(str::trim).filter(|p| !p.is_empty()) {
            Some(p) => Some(hash_password(p)?),
            None => None,
        };
        let (white, black) = match side.to_ascii_lowercase().as_str() {
            "white" => (Some(guest.clone()), None),
            "black" => (None, Some(guest.clone())),
            _ => {
                if rand::random::<bool>() {
                    (Some(guest.clone()), None)
                } else {
                    (None, Some(guest.clone()))
                }
            }
        };

        let id = loop {
            let candidate: String = {
                let n: u64 = rand::random();
                format!("{:08x}", n as u32)
            };
            if !read_lock(&self.rooms).contains_key(&candidate) {
                break candidate;
            }
        };

        // Consistent lock order: rooms first, then room_names.
        let mut rooms = write_lock(&self.rooms);
        if rooms.len() >= MAX_ROOMS {
            return Err(OnlineError::new(
                429,
                "room_limit_reached",
                "The server has reached its room limit",
            ));
        }
        let mut names = lock(&self.room_names);
        if names.contains_key(&name_lower) {
            return Err(OnlineError::new(
                409,
                "name_taken",
                "That room name is already in use",
            ));
        }
        names.insert(name_lower.clone(), id.clone());
        drop(names);

        let room = OnlineRoom::new(
            id.clone(),
            name,
            name_lower,
            password_hash,
            guest.name.clone(),
            tc.to_string(),
            coach_allowed,
            white,
            black,
        )?;
        let value = room.summary_json();
        rooms.insert(id.clone(), Arc::new(Mutex::new(room)));
        drop(rooms);
        Ok(json!({ "room": value }))
    }

    pub fn rooms_json(&self, query: &str) -> Value {
        let needle = query.trim().to_lowercase();
        let rooms = read_lock(&self.rooms);
        let mut rows: Vec<(Instant, Value)> = Vec::new();
        for room in rooms.values() {
            let mut guard = lock(room);
            guard.tick();
            if !needle.is_empty() && !guard.name_lower.contains(&needle) {
                continue;
            }
            rows.push((guard.created_at, guard.summary_json()));
        }
        rows.sort_by_key(|(created, _)| *created);
        let list: Vec<Value> = rows.into_iter().map(|(_, row)| row).collect();
        json!({ "rooms": list, "count": list.len() })
    }

    pub fn room_details(&self, id: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        guard.tick();
        Ok(guard.details_json())
    }

    pub fn join_room(
        &self,
        guest: &GuestSession,
        id: &str,
        password: Option<&str>,
    ) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        self.leave_other_rooms(&guest.token_hash, Some(id))?;

        let hash = {
            let mut guard = lock(&room);
            guard.tick();
            if guard.seat_index(&guest.token_hash).is_some() {
                return Ok(json!({ "room": guard.summary_json() }));
            }
            if guard.status() == RoomStatus::Finished {
                return Err(OnlineError::new(
                    409,
                    "room_finished",
                    "That game has already finished",
                ));
            }
            if guard.occupied() >= 2 {
                return Err(OnlineError::new(409, "room_full", "That room is full"));
            }
            guard.password_hash.clone()
        };
        if let Some(hash) = hash {
            let supplied = password.unwrap_or("");
            if !verify_password(supplied, &hash) {
                return Err(OnlineError::new(
                    403,
                    "bad_password",
                    "That password is incorrect",
                ));
            }
        }

        let mut guard = lock(&room);
        guard.tick();
        if guard.seat_index(&guest.token_hash).is_some() {
            return Ok(json!({ "room": guard.summary_json() }));
        }
        if guard.occupied() >= 2 {
            return Err(OnlineError::new(409, "room_full", "That room is full"));
        }
        if guard.status() == RoomStatus::Finished {
            return Err(OnlineError::new(
                409,
                "room_finished",
                "That game has already finished",
            ));
        }
        guard.seat(guest.clone())?;
        Ok(json!({ "room": guard.summary_json() }))
    }

    pub fn quick_join(&self, guest: &GuestSession) -> Result<Value, OnlineError> {
        if let Some((id, _)) = self.active_room_for(&guest.token_hash) {
            let room = self
                .room_map_get(&id)
                .ok_or_else(OnlineError::room_not_found)?;
            let guard = lock(&room);
            if guard.status() == RoomStatus::Playing
                || guard.seat_index(&guest.token_hash).is_some()
            {
                return Ok(json!({ "room": guard.summary_json() }));
            }
        }

        let candidates: Vec<String> = {
            let rooms = read_lock(&self.rooms);
            let mut rows: Vec<(Instant, String)> = Vec::new();
            for (id, room) in rooms.iter() {
                let guard = lock(room);
                if guard.status() == RoomStatus::Waiting
                    && guard.password_hash.is_none()
                    && guard.occupied() == 1
                    && guard.seat_index(&guest.token_hash).is_none()
                {
                    rows.push((guard.created_at, id.clone()));
                }
            }
            rows.sort_by_key(|(created, _)| *created);
            rows.into_iter().map(|(_, id)| id).collect()
        };

        for id in candidates {
            match self.join_room(guest, &id, None) {
                Ok(value) => return Ok(value),
                Err(e) if e.code == "room_full" || e.code == "room_not_found" => continue,
                Err(e) => return Err(e),
            }
        }

        let base = format!("{}'s room", guest.name);
        let mut name = base.clone();
        for n in 2..20 {
            if !lock(&self.room_names).contains_key(&name.to_lowercase()) {
                break;
            }
            name = format!("{} {}", base, n);
        }
        self.create_room(guest, &name, None, &self.default_tc, "random", false)
    }

    fn leave_room(&self, token_hash: &str, id: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let delete = {
            let mut guard = lock(&room);
            let Some(idx) = guard.seat_index(token_hash) else {
                return Err(OnlineError::not_seated());
            };
            if guard.left[idx] {
                return Ok(json!({ "ok": true }));
            }
            guard.tick();
            match guard.status() {
                RoomStatus::Waiting => true,
                RoomStatus::Playing => {
                    let color = if idx == 0 { Color::White } else { Color::Black };
                    let _ = guard.game.abandon_by(color);
                    guard.left[idx] = true;
                    guard.clear_offer();
                    guard.rematch_votes = [false, false];
                    false
                }
                RoomStatus::Finished => {
                    guard.left[idx] = true;
                    guard.rematch_votes = [false, false];
                    guard.left.iter().all(|left| *left)
                }
            }
        };
        if delete {
            self.remove_room(id);
        }
        Ok(json!({ "ok": true }))
    }

    fn state_for(&self, guest: &GuestSession, id: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.last_seen[idx] = Some(Instant::now());
        guard.tick();
        guard.state_json(&guest.token_hash)
    }

    fn moves_for(&self, guest: &GuestSession, id: &str, from: u8) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        guard.tick();
        if guard.left[idx] || guard.status() != RoomStatus::Playing {
            return Ok(json!({ "moves": [] }));
        }
        guard.last_seen[idx] = Some(Instant::now());
        let moves: Vec<Value> = guard
            .game
            .legal_targets(from)
            .into_iter()
            .map(|(to, promo, capture)| {
                json!({ "to": engine::sq_name(to), "promo": promo, "capture": capture })
            })
            .collect();
        Ok(json!({ "moves": moves }))
    }

    fn position_for(
        &self,
        guest: &GuestSession,
        id: &str,
        ply: usize,
    ) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.last_seen[idx] = Some(Instant::now());
        let bytes = guard
            .game
            .position_json(ply)
            .ok_or_else(|| OnlineError::new(400, "bad_ply", "That move does not exist"))?;
        serde_json::from_slice(&bytes)
            .map_err(|_| OnlineError::new(500, "serialize_failed", "Could not read position"))
    }

    fn move_piece(
        &self,
        guest: &GuestSession,
        id: &str,
        from: u8,
        to: u8,
        promo: Option<Kind>,
    ) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.last_seen[idx] = Some(Instant::now());
        guard.tick();
        if guard.status() != RoomStatus::Playing {
            return Err(OnlineError::new(
                409,
                "game_over",
                "The game has not started or has already finished",
            ));
        }
        let color = if idx == 0 { Color::White } else { Color::Black };
        if guard.game.turn_color() != color {
            return Err(OnlineError::new(
                403,
                "not_your_turn",
                "It is not your turn",
            ));
        }
        if guard.game.is_over() {
            return Err(OnlineError::new(
                409,
                "game_over",
                "The game has already finished",
            ));
        }
        match guard.game.do_move(from, to, promo) {
            Ok((san, sfx)) => {
                let mut state = guard.state_json(&guest.token_hash)?;
                state["san"] = Value::String(san);
                state["sfx"] = Value::String(sfx.to_string());
                guard.clear_offer();
                Ok(state)
            }
            Err(code) => Err(map_move_error(&code)),
        }
    }

    fn resign(&self, guest: &GuestSession, id: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.tick();
        if guard.status() != RoomStatus::Playing {
            return Err(OnlineError::new(
                409,
                "game_over",
                "The game has already finished",
            ));
        }
        let color = if idx == 0 { Color::White } else { Color::Black };
        guard.game.resign(color).map_err(|e| map_move_error(&e))?;
        guard.clear_offer();
        guard.rematch_votes = [false, false];
        guard.state_json(&guest.token_hash)
    }

    fn draw(&self, guest: &GuestSession, id: &str, action: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.tick();
        if guard.status() != RoomStatus::Playing {
            return Err(OnlineError::new(
                409,
                "game_over",
                "The game has already finished",
            ));
        }
        let color = if idx == 0 { Color::White } else { Color::Black };
        guard.expire_offer();
        match action {
            "offer" => {
                if let Some(offer) = guard.draw_offer {
                    if offer != color {
                        guard.game.draw_agree().map_err(|e| map_move_error(&e))?;
                        guard.clear_offer();
                    }
                } else {
                    guard.draw_offer = Some(color);
                    guard.draw_offer_at = Some(Instant::now());
                }
            }
            "accept" => {
                if guard.draw_offer.is_some_and(|offer| offer != color) {
                    guard.game.draw_agree().map_err(|e| map_move_error(&e))?;
                    guard.clear_offer();
                } else {
                    return Err(OnlineError::new(
                        409,
                        "no_draw_offer",
                        "There is no draw offer to accept",
                    ));
                }
            }
            "decline" => {
                if guard.draw_offer.is_some_and(|offer| offer != color) {
                    guard.clear_offer();
                } else {
                    return Err(OnlineError::new(
                        409,
                        "no_draw_offer",
                        "There is no draw offer to decline",
                    ));
                }
            }
            _ => {
                return Err(OnlineError::new(
                    400,
                    "invalid_action",
                    "Draw action must be offer, accept, or decline",
                ))
            }
        }
        guard.state_json(&guest.token_hash)
    }

    fn rematch(&self, guest: &GuestSession, id: &str) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        if guard.left[idx] {
            return Err(OnlineError::not_seated());
        }
        guard.tick();
        if guard.status() != RoomStatus::Finished {
            return Err(OnlineError::new(
                409,
                "game_over",
                "Rematch is available after the game ends",
            ));
        }
        if guard.left.iter().any(|left| *left) || guard.occupied() < 2 {
            return Err(OnlineError::new(
                409,
                "room_finished",
                "The other player has left the room",
            ));
        }
        guard.rematch_votes[idx] = true;
        if guard.rematch_votes.iter().all(|vote| *vote) {
            let tc = guard.game.tc_label();
            guard.game.reset(&tc).map_err(|_| {
                OnlineError::new(500, "rematch_failed", "Could not start the rematch")
            })?;
            guard.seats.swap(0, 1);
            guard.last_seen.swap(0, 1);
            guard.left = [false, false];
            guard.rematch_votes = [false, false];
            guard.coach_used = [0, 0];
            guard.coach_cache.clear();
            guard.draw_offer = None;
            guard.draw_offer_at = None;
            guard.finished_at = None;
        }
        guard.state_json(&guest.token_hash)
    }

    fn coach(
        &self,
        guest: &GuestSession,
        id: &str,
        bots: &BotManager,
        coach: &Arc<Mutex<Coach>>,
    ) -> Result<Value, OnlineError> {
        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let (idx, snapshot, cached) = {
            let mut guard = lock(&room);
            let idx = guard
                .seat_index(&guest.token_hash)
                .ok_or_else(OnlineError::not_seated)?;
            if guard.left[idx] {
                return Err(OnlineError::not_seated());
            }
            if !guard.coach_allowed {
                return Err(OnlineError::new(
                    403,
                    "coach_disabled",
                    "This room does not allow coach",
                ));
            }
            if guard.status() != RoomStatus::Playing {
                return Err(OnlineError::new(409, "game_over", "The game is not active"));
            }
            let color = if idx == 0 { Color::White } else { Color::Black };
            guard.tick();
            if guard.game.turn_color() != color {
                return Err(OnlineError::new(
                    403,
                    "not_your_turn",
                    "It is not your turn",
                ));
            }
            if guard.coach_used[idx] >= COACH_LIMIT {
                return Err(OnlineError::new(
                    429,
                    "coach_limit_reached",
                    "You have used all coach calls for this game",
                ));
            }
            let snapshot = coach_snapshot(&mut guard.game);
            let key = coach_key(&snapshot);
            let cached = guard.coach_cache.get(&key).cloned();
            guard.last_seen[idx] = Some(Instant::now());
            (idx, snapshot, cached)
        };

        if let Some(mut hit) = cached {
            let room = self
                .room_map_get(id)
                .ok_or_else(OnlineError::room_not_found)?;
            let guard = lock(&room);
            hit["coach_uses_left"] = json!(COACH_LIMIT.saturating_sub(guard.coach_used[idx]));
            return Ok(json!({ "coach": hit }));
        }

        let mut payload = coach_suggestion(&snapshot, coach, bots).map_err(|_| {
            OnlineError::new(503, "coach_unavailable", "Coach analysis is unavailable")
        })?;

        let room = self
            .room_map_get(id)
            .ok_or_else(OnlineError::room_not_found)?;
        let mut guard = lock(&room);
        let same_idx = guard
            .seat_index(&guest.token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        guard.tick();
        if same_idx != idx
            || guard.left[idx]
            || guard.status() != RoomStatus::Playing
            || guard.game.ply_count() != snapshot.ply
            || guard.game.turn_color() != snapshot.turn
        {
            return Err(OnlineError::new(
                409,
                "position_changed",
                "The position changed while coach was thinking",
            ));
        }
        if guard.coach_used[idx] >= COACH_LIMIT {
            return Err(OnlineError::new(
                429,
                "coach_limit_reached",
                "You have used all coach calls for this game",
            ));
        }
        guard.coach_used[idx] += 1;
        payload["coach_uses_left"] = json!(COACH_LIMIT.saturating_sub(guard.coach_used[idx]));
        guard
            .coach_cache
            .insert(coach_key(&snapshot), payload.clone());
        Ok(json!({ "coach": payload }))
    }

    /// Tick clocks, expire offers, forfeit abandoned seats, and clean rooms.
    /// Runs on a one-second thread while the online server is active.
    pub fn janitor(&self) {
        self.expire_guests();
        let snapshot: Vec<(String, Arc<Mutex<OnlineRoom>>)> = {
            let rooms = read_lock(&self.rooms);
            rooms
                .iter()
                .map(|(id, room)| (id.clone(), room.clone()))
                .collect()
        };
        let mut remove = Vec::new();
        for (id, room) in snapshot {
            let mut guard = lock(&room);
            guard.tick();
            guard.expire_offer();

            if guard.status() == RoomStatus::Playing {
                let mut forfeit = None;
                for idx in 0..2 {
                    if let Some(seen) = guard.last_seen[idx] {
                        if seen.elapsed() > DISCONNECT_GRACE && !guard.left[idx] {
                            forfeit = Some(idx);
                            break;
                        }
                    }
                }
                if let Some(idx) = forfeit {
                    let color = if idx == 0 { Color::White } else { Color::Black };
                    let _ = guard.game.abandon_by(color);
                    guard.left[idx] = true;
                    guard.clear_offer();
                    guard.rematch_votes = [false, false];
                    guard.tick();
                }
            }

            let now = Instant::now();
            let status = guard.status();
            let stale_waiting = status == RoomStatus::Waiting
                && guard
                    .last_seen
                    .iter()
                    .flatten()
                    .any(|seen| now.duration_since(*seen) > DISCONNECT_GRACE);
            let ttl = match status {
                RoomStatus::Waiting => now.duration_since(guard.created_at) > WAITING_TTL,
                RoomStatus::Finished => guard
                    .finished_at
                    .map(|at| now.duration_since(at) > FINISHED_TTL)
                    .unwrap_or(false),
                RoomStatus::Playing => false,
            };
            let all_left = guard.left.iter().all(|left| *left);
            if stale_waiting || ttl || all_left || guard.occupied() == 0 {
                remove.push(id);
            }
        }
        for id in remove {
            self.remove_room(&id);
        }
    }
}

fn map_move_error(code: &str) -> OnlineError {
    match code {
        "game_over" => OnlineError::new(409, "game_over", "The game has already finished"),
        "promotion_required" => {
            OnlineError::new(400, "promotion_required", "Choose a promotion piece")
        }
        "bad_square" | "illegal" => OnlineError::new(400, "illegal_move", "That move is not legal"),
        _ => OnlineError::new(400, "illegal_move", "That move is not legal"),
    }
}

fn coach_key(snapshot: &CoachSnapshot) -> String {
    engine::to_fen(&snapshot.pos, 1)
        .split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomStatus {
    Waiting,
    Playing,
    Finished,
}

impl RoomStatus {
    fn as_str(self) -> &'static str {
        match self {
            RoomStatus::Waiting => "waiting",
            RoomStatus::Playing => "playing",
            RoomStatus::Finished => "finished",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Seat {
    pub token_hash: String,
    pub name: String,
}

pub struct OnlineRoom {
    pub id: String,
    pub name: String,
    pub name_lower: String,
    pub password_hash: Option<String>,
    pub host_name: String,
    pub created_at: Instant,
    pub finished_at: Option<Instant>,
    pub tc: String,
    pub coach_allowed: bool,
    pub game: Game,
    pub seats: [Option<Seat>; 2],
    pub last_seen: [Option<Instant>; 2],
    pub left: [bool; 2],
    pub draw_offer: Option<Color>,
    pub draw_offer_at: Option<Instant>,
    pub rematch_votes: [bool; 2],
    pub coach_used: [u8; 2],
    pub coach_cache: HashMap<String, Value>,
}

impl OnlineRoom {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: String,
        name: String,
        name_lower: String,
        password_hash: Option<String>,
        host_name: String,
        tc: String,
        coach_allowed: bool,
        white: Option<GuestSession>,
        black: Option<GuestSession>,
    ) -> Result<Self, OnlineError> {
        let game = Game::new(&tc).map_err(|_| {
            OnlineError::new(
                400,
                "invalid_time_control",
                "Use a time control like 10+0 or unlimited",
            )
        })?;
        let tc = game.tc_label();
        let seat = |guest: Option<GuestSession>| {
            guest.map(|g| Seat {
                token_hash: g.token_hash,
                name: g.name,
            })
        };
        let now = Instant::now();
        let seats = [seat(white), seat(black)];
        let mut last_seen = [None, None];
        for idx in 0..2 {
            if seats[idx].is_some() {
                last_seen[idx] = Some(now);
            }
        }
        Ok(Self {
            id,
            name,
            name_lower,
            password_hash,
            host_name,
            created_at: now,
            finished_at: None,
            tc,
            coach_allowed,
            game,
            seats,
            last_seen,
            left: [false, false],
            draw_offer: None,
            draw_offer_at: None,
            rematch_votes: [false, false],
            coach_used: [0, 0],
            coach_cache: HashMap::new(),
        })
    }

    fn seat_index(&self, token_hash: &str) -> Option<usize> {
        self.seats
            .iter()
            .position(|seat| seat.as_ref().is_some_and(|s| s.token_hash == token_hash))
    }

    fn occupied(&self) -> usize {
        self.seats.iter().filter(|seat| seat.is_some()).count()
    }

    fn status(&self) -> RoomStatus {
        if self.seats.iter().any(|seat| seat.is_none()) {
            RoomStatus::Waiting
        } else if self.game.is_over() {
            RoomStatus::Finished
        } else {
            RoomStatus::Playing
        }
    }

    fn tick(&mut self) {
        if self.occupied() == 2 {
            self.game.tick();
        }
        if self.game.is_over() && self.finished_at.is_none() {
            self.finished_at = Some(Instant::now());
        }
    }

    fn seat(&mut self, guest: GuestSession) -> Result<(), OnlineError> {
        let idx = if self.seats[0].is_none() { 0 } else { 1 };
        if self.seats[idx].is_some() {
            return Err(OnlineError::new(409, "room_full", "That room is full"));
        }
        self.seats[idx] = Some(Seat {
            token_hash: guest.token_hash,
            name: guest.name,
        });
        self.last_seen[idx] = Some(Instant::now());
        self.left[idx] = false;
        self.draw_offer = None;
        self.draw_offer_at = None;
        self.rematch_votes = [false, false];
        Ok(())
    }

    fn expire_offer(&mut self) {
        if let Some(at) = self.draw_offer_at {
            if at.elapsed() > DRAW_OFFER_TTL {
                self.clear_offer();
            }
        }
    }

    fn clear_offer(&mut self) {
        self.draw_offer = None;
        self.draw_offer_at = None;
    }

    fn available_side(&self) -> Option<&'static str> {
        match (&self.seats[0], &self.seats[1]) {
            (Some(_), None) => Some("black"),
            (None, Some(_)) => Some("white"),
            (None, None) => Some("white"),
            (Some(_), Some(_)) => None,
        }
    }

    fn summary_json(&self) -> Value {
        json!({
            "id": self.id,
            "name": self.name,
            "locked": self.password_hash.is_some(),
            "players": self.occupied(),
            "status": self.status().as_str(),
            "host": self.host_name,
            "tc": self.tc,
            "coach_allowed": self.coach_allowed,
            "available_side": self.available_side(),
        })
    }

    fn details_json(&self) -> Value {
        let mut value = self.summary_json();
        value["full"] = json!(self.occupied() >= 2);
        value
    }

    fn connected(&self, idx: usize) -> bool {
        self.seats[idx].is_some()
            && !self.left[idx]
            && self.last_seen[idx]
                .map(|seen| seen.elapsed() <= CONNECTED_WINDOW)
                .unwrap_or(false)
    }

    fn state_json(&mut self, token_hash: &str) -> Result<Value, OnlineError> {
        self.tick();
        let idx = self
            .seat_index(token_hash)
            .ok_or_else(OnlineError::not_seated)?;
        let color = if idx == 0 { Color::White } else { Color::Black };
        let mut state: Value =
            serde_json::from_slice(&self.game.state_json()).unwrap_or_else(|_| json!({}));
        state["mode"] = json!("online");
        state["online"] = json!({
            "room_id": self.id,
            "room_name": self.name,
            "color": if color == Color::White { "w" } else { "b" },
            "white": self.seats[0].as_ref().map(|s| s.name.clone()),
            "black": self.seats[1].as_ref().map(|s| s.name.clone()),
            "white_connected": self.connected(0),
            "black_connected": self.connected(1),
            "status": self.status().as_str(),
            "draw_offer": self.draw_offer.map(|c| if c == Color::White { "w" } else { "b" }),
            "rematch_offered": self.rematch_votes[idx],
            "coach_allowed": self.coach_allowed,
            "coach_uses_left": COACH_LIMIT.saturating_sub(self.coach_used[idx]),
        });
        Ok(state)
    }
}

// ── HTTP API ──────────────────────────────────────────────────────────────

pub struct OnlineApiRequest<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub query: &'a str,
    pub body: &'a [u8],
    pub cookie: Option<&'a str>,
    pub host: &'a str,
    pub origin: Option<&'a str>,
    pub secure: bool,
    pub peer: &'a str,
    pub state: &'a OnlineState,
    pub bots: &'a Arc<BotManager>,
    pub coach: &'a Arc<Mutex<Coach>>,
}

pub struct OnlineApiResponse {
    pub bytes: Vec<u8>,
    pub cookie: Option<String>,
}

impl OnlineApiResponse {
    fn plain(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            cookie: None,
        }
    }
}

fn origin_matches(origin: &str, host: &str) -> bool {
    let origin = origin.trim();
    if origin.is_empty() || origin.eq_ignore_ascii_case("null") {
        return false;
    }
    let without_scheme = origin
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(origin);
    let origin_host = without_scheme.split('/').next().unwrap_or(without_scheme);
    origin_host.eq_ignore_ascii_case(host.trim())
}

fn parse_body(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

fn parse_square(value: Option<&Value>) -> Option<u8> {
    match value {
        Some(Value::String(s)) => engine::parse_sq(s),
        Some(Value::Number(n)) => n.as_u64().filter(|v| *v < 64).map(|v| v as u8),
        _ => None,
    }
}

fn parse_promo(value: Option<&Value>) -> Option<Kind> {
    let s = value?.as_str()?.trim().to_uppercase();
    match s.as_str() {
        "Q" => Some(Kind::Queen),
        "R" => Some(Kind::Rook),
        "B" => Some(Kind::Bishop),
        "N" => Some(Kind::Knight),
        _ => None,
    }
}

fn room_id_from_path(path: &str) -> Option<(&str, Option<&str>)> {
    let rest = path.strip_prefix("/api/online/rooms/")?;
    if rest.is_empty() {
        return None;
    }
    match rest.split_once('/') {
        Some((id, action)) => Some((id, Some(action))),
        None => Some((rest, None)),
    }
}

pub fn route_api(req: OnlineApiRequest<'_>) -> OnlineApiResponse {
    if req.method == "POST" {
        if let Some(origin) = req.origin {
            if !origin_matches(origin, req.host) {
                return OnlineApiResponse::plain(json_status(
                    403,
                    json!({"error":"bad_origin","message":"Cross-origin request rejected"}),
                ));
            }
        }
    }

    let token = req.cookie.and_then(cookie_token);
    let guest = token
        .as_deref()
        .and_then(|t| req.state.guest_for_token(Some(t)));

    let result: Result<(Value, Option<String>), OnlineError> = (|| match (req.method, req.path) {
        ("GET", "/api/online/info") => Ok((
            json!({
                "online": true,
                "max_rooms": MAX_ROOMS,
                "coach_default_limit": COACH_LIMIT,
            }),
            None,
        )),
        ("GET", "/api/online/me") => {
            let room = guest
                .as_ref()
                .and_then(|g| req.state.active_room_for(&g.token_hash))
                .map(|(id, name)| json!({"id": id, "name": name}));
            Ok((
                json!({
                    "guest": guest.as_ref().map(|g| json!({"name": g.name})),
                    "room": room,
                }),
                None,
            ))
        }
        ("POST", "/api/online/login") => {
            if !req.state.allow("login", req.peer, 10) {
                return Err(OnlineError::new(
                    429,
                    "rate_limited",
                    "Too many name changes",
                ));
            }
            let body = parse_body(req.body)
                .ok_or_else(|| OnlineError::new(400, "bad_request", "Invalid JSON body"))?;
            let name = body
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| OnlineError::new(400, "invalid_name", "Name is required"))?;
            let (guest, new_token) = req.state.login(token.as_deref(), name)?;
            let cookie = new_token.map(|t| guest_cookie(&t, req.secure));
            Ok((json!({"guest": {"name": guest.name}}), cookie))
        }
        ("POST", "/api/online/logout") => {
            let Some(guest) = guest else {
                return Ok((json!({"ok": true}), Some(clear_guest_cookie(req.secure))));
            };
            if let Some((id, _)) = req.state.active_room_for(&guest.token_hash) {
                let _ = req.state.leave_room(&guest.token_hash, &id);
            }
            req.state.logout(&guest);
            Ok((json!({"ok": true}), Some(clear_guest_cookie(req.secure))))
        }
        ("GET", "/api/online/rooms") => {
            let q = parse_query(req.query);
            let filter = q.get("q").map(|s| s.as_str()).unwrap_or("");
            Ok((req.state.rooms_json(filter), None))
        }
        ("POST", "/api/online/rooms") => {
            let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
            if !req.state.allow("create", &guest.token_hash, 5) {
                return Err(OnlineError::new(
                    429,
                    "rate_limited",
                    "Too many room creations",
                ));
            }
            let body = parse_body(req.body)
                .ok_or_else(|| OnlineError::new(400, "bad_request", "Invalid JSON body"))?;
            let name = body
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or_else(|| OnlineError::new(400, "invalid_name", "Room name is required"))?;
            let password = body.get("password").and_then(|v| v.as_str());
            let tc = body.get("tc").and_then(|v| v.as_str()).unwrap_or("10+0");
            let side = body
                .get("side")
                .and_then(|v| v.as_str())
                .unwrap_or("random");
            let coach_allowed = body
                .get("coach_allowed")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            Ok((
                req.state
                    .create_room(&guest, name, password, tc, side, coach_allowed)?,
                None,
            ))
        }
        ("POST", "/api/online/quick-join") => {
            let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
            if !req.state.allow("join", &guest.token_hash, 20) {
                return Err(OnlineError::new(
                    429,
                    "rate_limited",
                    "Too many join attempts",
                ));
            }
            Ok((req.state.quick_join(&guest)?, None))
        }
        _ => {
            if let Some((id, action)) = room_id_from_path(req.path) {
                if id.is_empty() {
                    return Err(OnlineError::room_not_found());
                }
                match (req.method, action) {
                    ("GET", None) => return Ok((req.state.room_details(id)?, None)),
                    ("POST", Some("join")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        if !req.state.allow("join", &guest.token_hash, 20) {
                            return Err(OnlineError::new(
                                429,
                                "rate_limited",
                                "Too many join attempts",
                            ));
                        }
                        let body = parse_body(req.body).unwrap_or_else(|| json!({}));
                        let password = body.get("password").and_then(|v| v.as_str());
                        return Ok((req.state.join_room(&guest, id, password)?, None));
                    }
                    ("POST", Some("leave")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        return Ok((req.state.leave_room(&guest.token_hash, id)?, None));
                    }
                    ("GET", Some("state")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        return Ok((req.state.state_for(&guest, id)?, None));
                    }
                    ("GET", Some("moves")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        let q = parse_query(req.query);
                        let sq = q
                            .get("sq")
                            .and_then(|s| engine::parse_sq(s))
                            .ok_or_else(|| OnlineError::new(400, "bad_square", "Invalid square"))?;
                        return Ok((req.state.moves_for(&guest, id, sq)?, None));
                    }
                    ("GET", Some("position")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        let q = parse_query(req.query);
                        let ply = q
                            .get("ply")
                            .and_then(|s| s.parse::<usize>().ok())
                            .ok_or_else(|| OnlineError::new(400, "bad_ply", "Invalid ply"))?;
                        return Ok((req.state.position_for(&guest, id, ply)?, None));
                    }
                    ("POST", Some("move")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        let body = parse_body(req.body).ok_or_else(|| {
                            OnlineError::new(400, "bad_request", "Invalid JSON body")
                        })?;
                        let from = parse_square(body.get("from")).ok_or_else(|| {
                            OnlineError::new(400, "bad_square", "Invalid from square")
                        })?;
                        let to = parse_square(body.get("to")).ok_or_else(|| {
                            OnlineError::new(400, "bad_square", "Invalid to square")
                        })?;
                        let promo = parse_promo(body.get("promo"));
                        return Ok((req.state.move_piece(&guest, id, from, to, promo)?, None));
                    }
                    ("POST", Some("resign")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        return Ok((req.state.resign(&guest, id)?, None));
                    }
                    ("POST", Some("draw")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        let body = parse_body(req.body).unwrap_or_else(|| json!({}));
                        let action = body
                            .get("action")
                            .and_then(|v| v.as_str())
                            .unwrap_or("offer");
                        return Ok((req.state.draw(&guest, id, action)?, None));
                    }
                    ("POST", Some("rematch")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        return Ok((req.state.rematch(&guest, id)?, None));
                    }
                    ("POST", Some("coach")) => {
                        let guest = guest.ok_or_else(OnlineError::not_logged_in)?;
                        if !req.state.allow("coach", &guest.token_hash, 10) {
                            return Err(OnlineError::new(
                                429,
                                "rate_limited",
                                "Too many coach requests",
                            ));
                        }
                        return Ok((req.state.coach(&guest, id, req.bots, req.coach)?, None));
                    }
                    _ => {}
                }
            }
            Err(OnlineError::new(
                404,
                "not_found",
                "Unknown online endpoint",
            ))
        }
    })();

    match result {
        Ok((value, cookie)) => OnlineApiResponse {
            bytes: json_ok(serde_json::to_vec(&value).unwrap_or_default()),
            cookie,
        },
        Err(e) => OnlineApiResponse::plain(e.as_response()),
    }
}

// Keep the table helper close to the state constructor for tests and future
// schema migrations.
#[cfg(test)]
pub fn ensure_online_schema(path: &std::path::Path) -> anyhow::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS online_guests (
            token_hash         TEXT PRIMARY KEY,
            display_name       TEXT NOT NULL,
            display_name_lower TEXT NOT NULL UNIQUE,
            created_at         INTEGER NOT NULL,
            last_seen          INTEGER NOT NULL,
            expires_at         INTEGER NOT NULL
        );",
    )?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> Arc<Mutex<Connection>> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("online.db");
        let conn = ensure_online_schema(&path).unwrap();
        // Keep the directory alive for the duration of the test process; the
        // connection keeps the SQLite file open on Windows too.
        std::mem::forget(dir);
        Arc::new(Mutex::new(conn))
    }

    fn state() -> OnlineState {
        OnlineState::new(db(), "10+0".to_string()).unwrap()
    }

    fn guest(state: &OnlineState, name: &str) -> GuestSession {
        state.login(None, name).unwrap().0
    }

    #[test]
    fn guest_names_are_trimmed_unique_and_case_insensitive() {
        let st = state();
        let a = guest(&st, "  Alice  ");
        assert_eq!(a.name, "Alice");
        assert!(st.login(None, " alice ").is_err());
        assert!(st.login(None, "").is_err());
        assert!(st.login(None, &"x".repeat(21)).is_err());
    }

    #[test]
    fn password_hash_roundtrips() {
        let hash = hash_password("secret").unwrap();
        assert!(verify_password("secret", &hash));
        assert!(!verify_password("wrong", &hash));
        assert!(!verify_password("secret", "garbage"));
    }

    #[test]
    fn room_names_and_create_seat_assignment() {
        let st = state();
        let alice = guest(&st, "Alice");
        let created = st
            .create_room(&alice, "Friday", None, "5+3", "black", true)
            .unwrap();
        let room = &created["room"];
        assert_eq!(room["name"], "Friday");
        assert_eq!(room["players"], 1);
        assert_eq!(room["available_side"], "white");
        assert_eq!(room["coach_allowed"], true);
        let bob = guest(&st, "Bob");
        assert!(st
            .create_room(&bob, "Friday", None, "5+3", "random", false)
            .is_err());
    }

    #[test]
    fn join_full_and_locked_rooms() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Locked", Some("pass"), "5+3", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        assert_eq!(
            st.join_room(&bob, &id, Some("bad")).unwrap_err().code,
            "bad_password"
        );
        let joined = st.join_room(&bob, &id, Some("pass")).unwrap();
        assert_eq!(joined["room"]["players"], 2);
        let carol = guest(&st, "Carol");
        assert_eq!(
            st.join_room(&carol, &id, Some("pass")).unwrap_err().code,
            "room_full"
        );
    }

    #[test]
    fn quick_match_joins_oldest_public_room_then_creates_one() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Public", None, "5+3", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap();
        let joined = st.quick_join(&bob).unwrap();
        assert_eq!(joined["room"]["id"], id);

        let carol = guest(&st, "Carol");
        let auto = st.quick_join(&carol).unwrap();
        assert_eq!(auto["room"]["players"], 1);
        assert!(auto["room"]["name"].as_str().unwrap().contains("Carol"));
    }

    #[test]
    fn turn_authorization_and_draw_contract() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Game", None, "5+0", "white", true)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let e2 = engine::parse_sq("e2").unwrap();
        let e4 = engine::parse_sq("e4").unwrap();
        assert_eq!(
            st.move_piece(&bob, &id, e2, e4, None).unwrap_err().code,
            "not_your_turn"
        );
        let state = st.move_piece(&alice, &id, e2, e4, None).unwrap();
        assert_eq!(state["moves"].as_array().unwrap().len(), 1);
        let offered = st.draw(&bob, &id, "offer").unwrap();
        assert_eq!(offered["online"]["draw_offer"], "b");
        let accepted = st.draw(&alice, &id, "accept").unwrap();
        assert_eq!(accepted["status"], "agreement");
    }

    #[test]
    fn rematch_requires_both_votes_and_swaps_colors() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Swap", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let e2 = engine::parse_sq("e2").unwrap();
        let e4 = engine::parse_sq("e4").unwrap();
        st.move_piece(&alice, &id, e2, e4, None).unwrap();
        st.resign(&bob, &id).unwrap();
        let one = st.rematch(&alice, &id).unwrap();
        assert_eq!(one["online"]["rematch_offered"], true);
        assert!(st.state_for(&alice, &id).unwrap()["status"]
            .as_str()
            .is_some());
        // Colors swap: Alice was White, so Bob is White in the rematch.
        let two = st.rematch(&bob, &id).unwrap();
        assert_eq!(two["online"]["color"], "w");
        assert_eq!(two["moves"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn coach_disabled_and_limit_are_server_side() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "No coach", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let bots = Arc::new(BotManager::new());
        let coach = Arc::new(Mutex::new(Coach::new()));
        assert_eq!(
            st.coach(&alice, &id, &bots, &coach).unwrap_err().code,
            "coach_disabled"
        );
    }

    #[test]
    fn leave_waiting_room_deletes_it_and_playing_room_forfeits() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Leave", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.leave_room(&alice.token_hash, &id).unwrap();
        assert!(st.room_details(&id).is_err());

        let created = st
            .create_room(&alice, "Resign", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let white = engine::parse_sq("e2").unwrap();
        let e4 = engine::parse_sq("e4").unwrap();
        st.move_piece(&alice, &id, white, e4, None).unwrap();
        st.leave_room(&bob.token_hash, &id).unwrap();
        let state = st.state_for(&alice, &id).unwrap();
        assert_eq!(state["status"], "abandoned");
        assert_eq!(state["winner"], "white");
    }

    #[test]
    fn coach_cache_avoids_spending_and_limit_is_enforced() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Cached coach", None, "5+0", "white", true)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let room = st.room_map_get(&id).unwrap();
        {
            let mut guard = lock(&room);
            let snapshot = coach_snapshot(&mut guard.game);
            guard.coach_cache.insert(
                coach_key(&snapshot),
                json!({"enabled": true, "available": true, "ply": 0, "suggestion": null}),
            );
        }
        let bots = Arc::new(BotManager::new());
        let coach = Arc::new(Mutex::new(Coach::new()));
        let cached = st.coach(&alice, &id, &bots, &coach).unwrap();
        assert_eq!(cached["coach"]["coach_uses_left"], 3);
        assert_eq!(lock(&room).coach_used[0], 0);

        {
            let mut guard = lock(&room);
            guard.coach_used[0] = COACH_LIMIT;
        }
        assert_eq!(
            st.coach(&alice, &id, &bots, &coach).unwrap_err().code,
            "coach_limit_reached"
        );
    }

    #[test]
    fn janitor_expires_offers_and_removes_abandoned_waiting_rooms() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let waiting = st
            .create_room(&alice, "Abandoned", None, "5+0", "white", false)
            .unwrap();
        let waiting_id = waiting["room"]["id"].as_str().unwrap().to_string();
        let waiting_room = st.room_map_get(&waiting_id).unwrap();
        {
            let mut guard = lock(&waiting_room);
            guard.last_seen[0] = Some(Instant::now() - DISCONNECT_GRACE - Duration::from_secs(1));
        }
        st.janitor();
        assert!(st.room_details(&waiting_id).is_err());

        let created = st
            .create_room(&alice, "Draw", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let e2 = engine::parse_sq("e2").unwrap();
        let e4 = engine::parse_sq("e4").unwrap();
        st.move_piece(&alice, &id, e2, e4, None).unwrap();
        st.draw(&bob, &id, "offer").unwrap();
        let room = st.room_map_get(&id).unwrap();
        {
            let mut guard = lock(&room);
            guard.draw_offer_at = Some(Instant::now() - DRAW_OFFER_TTL - Duration::from_secs(1));
        }
        st.janitor();
        assert!(lock(&room).draw_offer.is_none());
    }

    #[test]
    fn janitor_forfeits_a_disconnected_player() {
        let st = state();
        let alice = guest(&st, "Alice");
        let bob = guest(&st, "Bob");
        let created = st
            .create_room(&alice, "Gone", None, "5+0", "white", false)
            .unwrap();
        let id = created["room"]["id"].as_str().unwrap().to_string();
        st.join_room(&bob, &id, None).unwrap();
        let room = st.room_map_get(&id).unwrap();
        {
            let mut guard = lock(&room);
            guard.last_seen[0] = Some(Instant::now() - DISCONNECT_GRACE - Duration::from_secs(1));
        }
        st.janitor();
        let state = st.state_for(&bob, &id).unwrap();
        assert_eq!(state["status"], "abandoned");
        assert_eq!(state["winner"], "black");
    }

    #[test]
    fn cookie_parser_and_injection_are_independent() {
        assert_eq!(
            cookie_token("ntc_chess=abc; ntc_online=0123456789abcdef0123456789abcdef"),
            Some("0123456789abcdef0123456789abcdef".to_string())
        );
        assert_eq!(cookie_token("ntc_chess=abc"), None);
        let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".to_vec();
        let out = inject_cookie(raw, "ntc_online=x; Path=/");
        assert!(String::from_utf8(out)
            .unwrap()
            .contains("Set-Cookie: ntc_online=x; Path=/"));
    }
}
