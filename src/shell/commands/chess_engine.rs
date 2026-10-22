//! Hand-rolled chess rules engine for `serve chess`.
//!
//! No external chess crate: board, move generation, check/checkmate/
//! stalemate detection, castling, en passant, promotion, threefold
//! repetition, fifty-move rule, insufficient material and SAN formatting
//! are all implemented here. Everything is pure (explicit state in,
//! values out) so the whole ruleset is unit-testable without I/O.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opp(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    /// Pawn travel direction in rank units (+1 white, -1 black).
    fn pawn_dir(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    fn start_rank(self) -> u8 {
        match self {
            Color::White => 1,
            Color::Black => 6,
        }
    }

    fn promo_rank(self) -> u8 {
        match self {
            Color::White => 7,
            Color::Black => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Kind {
    /// SAN letter (empty for pawns).
    pub fn letter(self) -> &'static str {
        match self {
            Kind::Pawn => "",
            Kind::Knight => "N",
            Kind::Bishop => "B",
            Kind::Rook => "R",
            Kind::Queen => "Q",
            Kind::King => "K",
        }
    }

    /// Board-wire code (pawn is "P", unlike SAN).
    pub fn code(self) -> &'static str {
        match self {
            Kind::Pawn => "P",
            Kind::Knight => "N",
            Kind::Bishop => "B",
            Kind::Rook => "R",
            Kind::Queen => "Q",
            Kind::King => "K",
        }
    }

    pub fn promo_kinds() -> [Kind; 4] {
        [Kind::Queen, Kind::Rook, Kind::Bishop, Kind::Knight]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

/// A move. Castling is encoded as king moving two squares; the rook
/// hop is derived in [`apply_move`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promo: Option<Kind>,
}

// Castling rights bitmask.
pub const WK: u8 = 1;
pub const WQ: u8 = 2;
pub const BK: u8 = 4;
pub const BQ: u8 = 8;

#[derive(Clone, Debug)]
pub struct Position {
    pub board: [Option<Piece>; 64],
    pub turn: Color,
    pub castle: u8,
    pub ep: Option<u8>,
    pub halfmove: u32,
    pub hashes: Vec<u64>,
}

// ── Squares ───────────────────────────────────────────────────────────────

pub fn file_of(sq: u8) -> u8 {
    sq % 8
}

pub fn rank_of(sq: u8) -> u8 {
    sq / 8
}

pub fn sq_at(file: u8, rank: u8) -> u8 {
    rank * 8 + file
}

fn try_sq(file: i8, rank: i8) -> Option<u8> {
    if (0..8).contains(&file) && (0..8).contains(&rank) {
        Some(sq_at(file as u8, rank as u8))
    } else {
        None
    }
}

/// "e4" ↔ 28. Returns None on malformed input.
pub fn parse_sq(s: &str) -> Option<u8> {
    let b = s.trim().as_bytes();
    if b.len() != 2 {
        return None;
    }
    let f = b[0].to_ascii_lowercase().wrapping_sub(b'a');
    let r = b[1].wrapping_sub(b'1');
    if f < 8 && r < 8 {
        Some(sq_at(f, r))
    } else {
        None
    }
}

pub fn sq_name(sq: u8) -> String {
    format!(
        "{}{}",
        (b'a' + file_of(sq)) as char,
        (b'1' + rank_of(sq)) as char
    )
}

pub fn initial_position() -> Position {
    let mut board: [Option<Piece>; 64] = [None; 64];
    let back = [
        Kind::Rook,
        Kind::Knight,
        Kind::Bishop,
        Kind::Queen,
        Kind::King,
        Kind::Bishop,
        Kind::Knight,
        Kind::Rook,
    ];
    for f in 0..8 {
        board[sq_at(f, 0) as usize] = Some(Piece {
            kind: back[f as usize],
            color: Color::White,
        });
        board[sq_at(f, 1) as usize] = Some(Piece {
            kind: Kind::Pawn,
            color: Color::White,
        });
        board[sq_at(f, 6) as usize] = Some(Piece {
            kind: Kind::Pawn,
            color: Color::Black,
        });
        board[sq_at(f, 7) as usize] = Some(Piece {
            kind: back[f as usize],
            color: Color::Black,
        });
    }
    let mut pos = Position {
        board,
        turn: Color::White,
        castle: WK | WQ | BK | BQ,
        ep: None,
        halfmove: 0,
        hashes: Vec::new(),
    };
    let h = hash_position(&pos);
    pos.hashes.push(h);
    pos
}

// ── Attack detection ──────────────────────────────────────────────────────

const KNIGHT_STEPS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

const KING_STEPS: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const DIAG: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const STRAIGHT: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

/// True when `sq` is attacked by any piece of color `by`.
pub fn is_attacked(pos: &Position, sq: u8, by: Color) -> bool {
    let f = file_of(sq) as i8;
    let r = rank_of(sq) as i8;

    // Pawns: a `by` pawn attacks from one rank behind the target
    // (relative to its own travel direction).
    let pr = r - by.pawn_dir();
    for df in [-1i8, 1] {
        if let Some(a) = try_sq(f + df, pr) {
            if pos.board[a as usize]
                == Some(Piece {
                    kind: Kind::Pawn,
                    color: by,
                })
            {
                return true;
            }
        }
    }
    // Knights.
    for (df, dr) in KNIGHT_STEPS {
        if let Some(a) = try_sq(f + df, r + dr) {
            if pos.board[a as usize]
                == Some(Piece {
                    kind: Kind::Knight,
                    color: by,
                })
            {
                return true;
            }
        }
    }
    // King.
    for (df, dr) in KING_STEPS {
        if let Some(a) = try_sq(f + df, r + dr) {
            if pos.board[a as usize]
                == Some(Piece {
                    kind: Kind::King,
                    color: by,
                })
            {
                return true;
            }
        }
    }
    // Sliders.
    for (df, dr) in DIAG {
        let mut nf = f + df;
        let mut nr = r + dr;
        while let Some(a) = try_sq(nf, nr) {
            match pos.board[a as usize] {
                None => {
                    nf += df;
                    nr += dr;
                }
                Some(p) => {
                    if p.color == by && (p.kind == Kind::Bishop || p.kind == Kind::Queen) {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    for (df, dr) in STRAIGHT {
        let mut nf = f + df;
        let mut nr = r + dr;
        while let Some(a) = try_sq(nf, nr) {
            match pos.board[a as usize] {
                None => {
                    nf += df;
                    nr += dr;
                }
                Some(p) => {
                    if p.color == by && (p.kind == Kind::Rook || p.kind == Kind::Queen) {
                        return true;
                    }
                    break;
                }
            }
        }
    }
    false
}

fn king_square(pos: &Position, color: Color) -> Option<u8> {
    pos.board
        .iter()
        .position(|p| {
            *p == Some(Piece {
                kind: Kind::King,
                color,
            })
        })
        .map(|i| i as u8)
}

pub fn in_check(pos: &Position, color: Color) -> bool {
    match king_square(pos, color) {
        Some(k) => is_attacked(pos, k, color.opp()),
        None => false,
    }
}

// ── Move generation ───────────────────────────────────────────────────────

fn push_pawn_moves(pos: &Position, from: u8, color: Color, out: &mut Vec<Move>) {
    let f = file_of(from) as i8;
    let r = rank_of(from) as i8;
    let dir = color.pawn_dir();
    // Quiet push.
    if let Some(one) = try_sq(f, r + dir) {
        if pos.board[one as usize].is_none() {
            if rank_of(one) == color.promo_rank() {
                for k in Kind::promo_kinds() {
                    out.push(Move {
                        from,
                        to: one,
                        promo: Some(k),
                    });
                }
            } else {
                out.push(Move {
                    from,
                    to: one,
                    promo: None,
                });
                // Double push from the starting rank.
                if rank_of(from) == color.start_rank() {
                    if let Some(two) = try_sq(f, r + 2 * dir) {
                        if pos.board[two as usize].is_none() {
                            out.push(Move {
                                from,
                                to: two,
                                promo: None,
                            });
                        }
                    }
                }
            }
        }
    }
    // Captures (including en passant).
    for df in [-1i8, 1] {
        if let Some(t) = try_sq(f + df, r + dir) {
            let is_capture = matches!(pos.board[t as usize], Some(p) if p.color != color);
            let is_ep = pos.ep == Some(t) && pos.board[t as usize].is_none();
            if is_capture || is_ep {
                if rank_of(t) == color.promo_rank() {
                    for k in Kind::promo_kinds() {
                        out.push(Move {
                            from,
                            to: t,
                            promo: Some(k),
                        });
                    }
                } else {
                    out.push(Move {
                        from,
                        to: t,
                        promo: None,
                    });
                }
            }
        }
    }
}

fn push_slider(pos: &Position, from: u8, color: Color, dirs: &[(i8, i8)], out: &mut Vec<Move>) {
    let f = file_of(from) as i8;
    let r = rank_of(from) as i8;
    for (df, dr) in dirs {
        let mut nf = f + df;
        let mut nr = r + dr;
        while let Some(t) = try_sq(nf, nr) {
            match pos.board[t as usize] {
                None => out.push(Move {
                    from,
                    to: t,
                    promo: None,
                }),
                Some(p) => {
                    if p.color != color {
                        out.push(Move {
                            from,
                            to: t,
                            promo: None,
                        });
                    }
                    break;
                }
            }
            nf += df;
            nr += dr;
        }
    }
}

fn push_castles(pos: &Position, color: Color, out: &mut Vec<Move>) {
    let (home, rights_k, rights_q) = match color {
        Color::White => (sq_at(4, 0), WK, WQ),
        Color::Black => (sq_at(4, 7), BK, BQ),
    };
    // King must be home.
    if pos.board[home as usize]
        != Some(Piece {
            kind: Kind::King,
            color,
        })
    {
        return;
    }
    let r = rank_of(home);
    let opp = color.opp();
    // Kingside: f,g empty; e,f,g unattacked.
    if pos.castle & rights_k != 0
        && pos.board[sq_at(5, r) as usize].is_none()
        && pos.board[sq_at(6, r) as usize].is_none()
        && !is_attacked(pos, home, opp)
        && !is_attacked(pos, sq_at(5, r), opp)
        && !is_attacked(pos, sq_at(6, r), opp)
    {
        out.push(Move {
            from: home,
            to: sq_at(6, r),
            promo: None,
        });
    }
    // Queenside: d,c,b empty; e,d,c unattacked.
    if pos.castle & rights_q != 0
        && pos.board[sq_at(3, r) as usize].is_none()
        && pos.board[sq_at(2, r) as usize].is_none()
        && pos.board[sq_at(1, r) as usize].is_none()
        && !is_attacked(pos, home, opp)
        && !is_attacked(pos, sq_at(3, r), opp)
        && !is_attacked(pos, sq_at(2, r), opp)
    {
        out.push(Move {
            from: home,
            to: sq_at(2, r),
            promo: None,
        });
    }
}

/// Pseudo-legal moves (self-check NOT filtered).
pub fn pseudo_moves(pos: &Position, from: u8) -> Vec<Move> {
    let mut out = Vec::new();
    let piece = match pos.board[from as usize] {
        Some(p) => p,
        None => return out,
    };
    let color = piece.color;
    let f = file_of(from) as i8;
    let r = rank_of(from) as i8;
    match piece.kind {
        Kind::Pawn => push_pawn_moves(pos, from, color, &mut out),
        Kind::Knight => {
            for (df, dr) in KNIGHT_STEPS {
                if let Some(t) = try_sq(f + df, r + dr) {
                    match pos.board[t as usize] {
                        None => out.push(Move {
                            from,
                            to: t,
                            promo: None,
                        }),
                        Some(p) if p.color != color => out.push(Move {
                            from,
                            to: t,
                            promo: None,
                        }),
                        _ => {}
                    }
                }
            }
        }
        Kind::Bishop => push_slider(pos, from, color, &DIAG, &mut out),
        Kind::Rook => push_slider(pos, from, color, &STRAIGHT, &mut out),
        Kind::Queen => {
            push_slider(pos, from, color, &DIAG, &mut out);
            push_slider(pos, from, color, &STRAIGHT, &mut out);
        }
        Kind::King => {
            for (df, dr) in KING_STEPS {
                if let Some(t) = try_sq(f + df, r + dr) {
                    match pos.board[t as usize] {
                        None => out.push(Move {
                            from,
                            to: t,
                            promo: None,
                        }),
                        Some(p) if p.color != color => out.push(Move {
                            from,
                            to: t,
                            promo: None,
                        }),
                        _ => {}
                    }
                }
            }
            push_castles(pos, color, &mut out);
        }
    }
    out
}

fn clear_rights_for_square(color: Color, sq: u8, rights: u8) -> u8 {
    // a1/h1/a8/h8 rook home squares clear their side's right.
    match (color, sq) {
        (Color::White, s) if s == sq_at(0, 0) => rights & !WQ,
        (Color::White, s) if s == sq_at(7, 0) => rights & !WK,
        (Color::Black, s) if s == sq_at(0, 7) => rights & !BQ,
        (Color::Black, s) if s == sq_at(7, 7) => rights & !BK,
        _ => rights,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Applied {
    pub captured: Option<Piece>,
    pub was_castle_kingside: bool,
    pub was_castle_queenside: bool,
    pub was_ep: bool,
}

/// Apply a (previously generated) move. Returns what was captured/how.
pub fn apply_move(pos: &mut Position, mv: Move) -> Applied {
    let mut info = Applied::default();
    let piece = pos.board[mv.from as usize].expect("apply on empty square");
    let color = piece.color;

    // En passant capture removes the pawn behind the target.
    if piece.kind == Kind::Pawn && pos.ep == Some(mv.to) && pos.board[mv.to as usize].is_none() {
        info.was_ep = true;
        let cap_sq = sq_at(file_of(mv.to), rank_of(mv.from));
        info.captured = pos.board[cap_sq as usize].take();
    } else {
        info.captured = pos.board[mv.to as usize].take();
    }

    // Rook captured on its home square loses castling rights.
    if let Some(cap) = info.captured {
        if cap.kind == Kind::Rook {
            pos.castle = clear_rights_for_square(cap.color, mv.to, pos.castle);
        }
    }

    let moved = Piece {
        kind: mv.promo.unwrap_or(piece.kind),
        color,
    };
    pos.board[mv.from as usize] = None;
    pos.board[mv.to as usize] = Some(moved);

    // Castling rook hop.
    if piece.kind == Kind::King && (file_of(mv.from) as i8 - file_of(mv.to) as i8).abs() == 2 {
        let r = rank_of(mv.from);
        if file_of(mv.to) == 6 {
            info.was_castle_kingside = true;
            pos.board[sq_at(5, r) as usize] = pos.board[sq_at(7, r) as usize].take();
        } else {
            info.was_castle_queenside = true;
            pos.board[sq_at(3, r) as usize] = pos.board[sq_at(0, r) as usize].take();
        }
    }

    // Castling rights: king move clears both; rook leaving home clears one.
    if piece.kind == Kind::King {
        pos.castle &= match color {
            Color::White => !(WK | WQ),
            Color::Black => !(BK | BQ),
        };
    } else if piece.kind == Kind::Rook {
        pos.castle = clear_rights_for_square(color, mv.from, pos.castle);
    }

    // En passant target: only a double pawn push sets it.
    let double_push =
        piece.kind == Kind::Pawn && (rank_of(mv.from) as i8 - rank_of(mv.to) as i8).abs() == 2;
    pos.ep = if double_push {
        try_sq(
            file_of(mv.from) as i8,
            (rank_of(mv.from) as i8 + rank_of(mv.to) as i8) / 2,
        )
    } else {
        None
    };

    if piece.kind == Kind::Pawn || info.captured.is_some() {
        pos.halfmove = 0;
    } else {
        pos.halfmove += 1;
    }
    pos.turn = color.opp();
    pos.hashes.push(hash_position(pos));
    info
}

/// Fully legal moves for the piece on `from` (side to move only).
pub fn legal_moves_for(pos: &Position, from: u8) -> Vec<Move> {
    match pos.board[from as usize] {
        Some(p) if p.color == pos.turn => {}
        _ => return Vec::new(),
    }
    let color = pos.turn;
    pseudo_moves(pos, from)
        .into_iter()
        .filter(|mv| {
            let mut probe = pos.clone();
            // Cloned probe shares history; hash push is harmless here.
            probe.hashes.pop();
            apply_move(&mut probe, *mv);
            probe.hashes.pop();
            !in_check(&probe, color)
        })
        .collect()
}

/// All legal moves for the side to move.
pub fn all_legal(pos: &Position) -> Vec<Move> {
    let mut out = Vec::new();
    for sq in 0..64u8 {
        if matches!(pos.board[sq as usize], Some(p) if p.color == pos.turn) {
            out.extend(legal_moves_for(pos, sq));
        }
    }
    out
}

// ── Position hashing (threefold repetition) ───────────────────────────────

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Hash of board + turn + castling + ep (FIDE repetition key).
pub fn hash_position(pos: &Position) -> u64 {
    let mut buf = Vec::with_capacity(70);
    for sq in pos.board.iter() {
        buf.push(match sq {
            None => 0,
            Some(Piece { kind, color }) => {
                let k = match kind {
                    Kind::Pawn => 1,
                    Kind::Knight => 2,
                    Kind::Bishop => 3,
                    Kind::Rook => 4,
                    Kind::Queen => 5,
                    Kind::King => 6,
                };
                let c = match color {
                    Color::White => 0,
                    Color::Black => 8,
                };
                k + c
            }
        });
    }
    buf.push(match pos.turn {
        Color::White => 0,
        Color::Black => 1,
    });
    buf.push(pos.castle);
    buf.push(pos.ep.unwrap_or(64));
    fnv(&buf)
}

pub fn repetition_count(pos: &Position) -> usize {
    let cur = hash_position(pos);
    pos.hashes.iter().filter(|h| **h == cur).count()
}

// ── Insufficient material ─────────────────────────────────────────────────

/// True when no legal sequence can possibly checkmate (auto-draw):
/// bare kings, K+minor vs K, or K+B vs K+B with same-colored bishops.
pub fn insufficient_material(pos: &Position) -> bool {
    let mut minors: Vec<(Kind, Color, u8)> = Vec::new();
    for (i, sq) in pos.board.iter().enumerate() {
        match sq {
            None => {}
            Some(Piece { kind, .. }) if *kind == Kind::King => {}
            Some(Piece {
                kind: Kind::Pawn | Kind::Rook | Kind::Queen,
                ..
            }) => return false,
            Some(Piece { kind, color }) => minors.push((*kind, *color, i as u8)),
        }
    }
    match minors.len() {
        0 => true,
        1 => true, // single bishop or knight cannot mate
        2 => {
            // K+B vs K+B on the same square color cannot mate.
            let (k1, c1, s1) = minors[0];
            let (k2, c2, s2) = minors[1];
            k1 == Kind::Bishop
                && k2 == Kind::Bishop
                && c1 != c2
                && (file_of(s1) + rank_of(s1)) % 2 == (file_of(s2) + rank_of(s2)) % 2
        }
        _ => false,
    }
}

// ── Hanging pieces (orange highlight) ─────────────────────────────────────

// True when the piece on `from` attacks `sq`, regardless of who occupies
// `sq` (a piece defends own-occupied squares; pseudo-move generation only
// yields enemy captures, so it cannot see defenders — hence this direct
// geometry test). Blockers before `sq` stop sliders.
fn attacks_square(pos: &Position, from: u8, sq: u8) -> bool {
    let piece = match pos.board[from as usize] {
        Some(p) => p,
        None => return false,
    };
    if from == sq {
        return false;
    }
    let ff = file_of(from) as i8;
    let rf = rank_of(from) as i8;
    let tf = file_of(sq) as i8;
    let tr = rank_of(sq) as i8;
    let df = tf - ff;
    let dr = tr - rf;
    match piece.kind {
        Kind::Pawn => dr == piece.color.pawn_dir() && df.abs() == 1,
        Kind::Knight => KNIGHT_STEPS
            .iter()
            .any(|(sdf, sdr)| *sdf == df && *sdr == dr),
        Kind::King => df.abs() <= 1 && dr.abs() <= 1,
        Kind::Bishop | Kind::Rook | Kind::Queen => {
            let straight = df == 0 || dr == 0;
            let diag = df.abs() == dr.abs();
            let ok = match piece.kind {
                Kind::Bishop => diag,
                Kind::Rook => straight,
                _ => straight || diag,
            };
            if !ok {
                return false;
            }
            let step_f = df.signum();
            let step_r = dr.signum();
            let mut nf = ff + step_f;
            let mut nr = rf + step_r;
            while nf != tf || nr != tr {
                match try_sq(nf, nr) {
                    Some(s) if pos.board[s as usize].is_none() => {
                        nf += step_f;
                        nr += step_r;
                    }
                    _ => return false,
                }
            }
            true
        }
    }
}

// Count pieces of color `by` attacking `sq` (defense-aware: own-occupied
// targets count as defended). Pin-aware: a pinned attacker's credit is
// switched off everywhere except its pinner's square (see `pinned_map`).
//
// Battery / x-ray support (sealed chain): on each of the 8 rays from `sq`,
// the first own piece (F1) anchors only with genuine direct credit; every
// further own piece counts iff it is an aligned unpinned slider (R/Q on
// straights, B/Q on diags) or has direct credit itself (e.g. a jumping
// knight) — and every link keeps the chain alive. The first own piece
// WITHOUT credit seals the ray: it never moves in the exchange, so
// nothing behind it can ever recapture (e.g. a pawn on the diagonal
// walls its queen off for good). An enemy piece stops the ray outright.
// Applies symmetrically to attackers and defenders.
fn battery_aligned(kind: Kind, straight: bool) -> bool {
    matches!(
        (straight, kind),
        (true, Kind::Rook) | (true, Kind::Queen) | (false, Kind::Bishop) | (false, Kind::Queen)
    )
}

/// Direct credit for one piece, pin-aware: pinned pieces count only on
/// their pinner's square and only with true geometry (a pinned knight,
/// e.g., can never reach its pinner). Single source of truth shared by
/// the direct loop and the battery chain walk below.
fn direct_credit(
    pos: &Position,
    from: u8,
    sq: u8,
    piece: Piece,
    pins: &std::collections::HashMap<u8, u8>,
) -> Option<Kind> {
    match pins.get(&from) {
        Some(pinner) if *pinner == sq && attacks_square(pos, from, sq) => Some(piece.kind),
        Some(_) => None,
        None if attacks_square(pos, from, sq) => Some(piece.kind),
        None => None,
    }
}

fn attackers_of(
    pos: &Position,
    sq: u8,
    by: Color,
    pins: &std::collections::HashMap<u8, u8>,
) -> Vec<Kind> {
    let mut out = Vec::new();
    for (i, p) in pos.board.iter().enumerate() {
        match p {
            Some(piece) if piece.color == by => {
                if let Some(kind) = direct_credit(pos, i as u8, sq, *piece, pins) {
                    out.push(kind);
                }
            }
            _ => {}
        }
    }
    // Sealed battery chain per ray: every own link must participate.
    let tf = file_of(sq) as i8;
    let tr = rank_of(sq) as i8;
    for (df, dr) in KING_STEPS {
        let straight = df == 0 || dr == 0;
        let mut nf = tf + df;
        let mut nr = tr + dr;
        let mut first = true;
        while let Some(s) = try_sq(nf, nr) {
            match pos.board[s as usize] {
                None => {
                    nf += df;
                    nr += dr;
                }
                // Enemy screens the whole rest of the ray.
                Some(piece) if piece.color != by => break,
                Some(piece) => {
                    if first {
                        // F1: direct credit (already pushed above) opens
                        // the chain; anything else seals it shut — no
                        // phantom x-ray without an anchor (e.g. Rh8
                        // through own Ph7 must not light an
                        // otherwise-unattacked Qh5). A king seals it
                        // too: it never vacates its ray in an exchange,
                        // so nothing stacked behind it can ever join —
                        // while keeping its own direct credit.
                        first = false;
                        if piece.kind == Kind::King
                            || direct_credit(pos, s, sq, piece, pins).is_none()
                        {
                            break;
                        }
                    } else if battery_aligned(piece.kind, straight) && !pins.contains_key(&s) {
                        out.push(piece.kind);
                    } else if direct_credit(pos, s, sq, piece, pins).is_none() {
                        // Own blocker that takes no part in the exchange
                        // (e.g. a pawn sitting on the diagonal): it never
                        // vacates, so everything behind it is walled off.
                        break;
                    }
                    // Counted links (battery or direct, e.g. a jumping
                    // knight) vacate in the exchange — chain stays alive.
                    nf += df;
                    nr += dr;
                }
            }
        }
    }
    out
}

// ── Pins (gray highlight) + relative pins (silver highlight) ───────────────

// Absolute pins to the king: victim square → pinner square. Ray-cast from
// each king; the first own non-king piece on a ray is pinned when the next
// piece beyond is an enemy slider aligned with the ray.
pub fn pinned_map(pos: &Position) -> std::collections::HashMap<u8, u8> {
    let mut pins = std::collections::HashMap::new();
    for (i, p) in pos.board.iter().enumerate() {
        let king_color = match p {
            Some(piece) if piece.kind == Kind::King => piece.color,
            _ => continue,
        };
        let kf = file_of(i as u8) as i8;
        let kr = rank_of(i as u8) as i8;
        for (df, dr) in KING_STEPS {
            let straight = df == 0 || dr == 0;
            let mut nf = kf + df;
            let mut nr = kr + dr;
            // First piece on the ray: pin candidate (must be own, non-king).
            let candidate = loop {
                match try_sq(nf, nr) {
                    None => break None,
                    Some(s) => match pos.board[s as usize] {
                        None => {
                            nf += df;
                            nr += dr;
                        }
                        Some(piece) if piece.color == king_color && piece.kind != Kind::King => {
                            break Some(s)
                        }
                        // Own king (impossible — kings can't be adjacent... the
                        // enemy king may sit on the ray, which also blocks),
                        // enemy piece first, or anything else: no pin.
                        _ => break None,
                    },
                }
            };
            let victim = match candidate {
                Some(s) => s,
                None => continue,
            };
            // Second piece beyond must be an enemy slider aligned with the ray.
            nf += df;
            nr += dr;
            loop {
                match try_sq(nf, nr) {
                    None => break,
                    Some(s) => match pos.board[s as usize] {
                        None => {
                            nf += df;
                            nr += dr;
                        }
                        Some(piece)
                            if piece.color != king_color
                                && matches!(
                                    (straight, piece.kind),
                                    (true, Kind::Rook)
                                        | (true, Kind::Queen)
                                        | (false, Kind::Bishop)
                                        | (false, Kind::Queen)
                                ) =>
                        {
                            pins.insert(victim, s);
                            break;
                        }
                        _ => break,
                    },
                }
            }
        }
    }
    pins
}

/// Victim squares of absolute pins (gray highlight). Kings are never victims.
pub fn pinned_squares(pos: &Position) -> Vec<u8> {
    let mut out: Vec<u8> = pinned_map(pos).keys().copied().collect();
    out.sort_unstable();
    out
}

/// Material value for relative-pin comparison (king excluded — absolute).
fn silver_value(kind: Kind) -> u8 {
    match kind {
        Kind::Pawn => 1,
        Kind::Knight | Kind::Bishop => 3,
        Kind::Rook => 5,
        Kind::Queen => 9,
        Kind::King => 255,
    }
}

// Relative pins to a pricier friendly piece (silver highlight):
// victim square → pinner square. Ray-cast from each own non-king piece
// treated as "shielded"; the first own non-king piece on a ray is a
// silver victim when it is strictly cheaper than the shielded piece and
// the next piece beyond is an enemy slider aligned with the ray that is
// itself strictly cheaper than the shielded piece (a same-tier or
// pricier pinner never wants the trade, so no pin).
// E.g. wN f3 between wQ d1 and bB g4 (e2 empty): f3 → g4.
// Absolute (king) pins are NOT included here — gray wins ties.
pub fn silver_map(pos: &Position) -> std::collections::HashMap<u8, u8> {
    let mut pins = std::collections::HashMap::new();
    let abs = pinned_map(pos);
    for (i, p) in pos.board.iter().enumerate() {
        let (shield_kind, shield_color) = match p {
            Some(piece) if piece.kind != Kind::King => (piece.kind, piece.color),
            _ => continue,
        };
        let kf = file_of(i as u8) as i8;
        let kr = rank_of(i as u8) as i8;
        for (df, dr) in KING_STEPS {
            let straight = df == 0 || dr == 0;
            let mut nf = kf + df;
            let mut nr = kr + dr;
            // First piece on the ray: candidate (own, non-king, cheaper).
            let victim = loop {
                match try_sq(nf, nr) {
                    None => break None,
                    Some(s) => match pos.board[s as usize] {
                        None => {
                            nf += df;
                            nr += dr;
                        }
                        Some(piece)
                            if piece.color == shield_color
                                && piece.kind != Kind::King
                                && silver_value(piece.kind) < silver_value(shield_kind) =>
                        {
                            break Some(s)
                        }
                        _ => break None,
                    },
                }
            };
            let victim = match victim {
                Some(s) => s,
                None => continue,
            };
            if abs.contains_key(&victim) || pins.contains_key(&victim) {
                continue;
            }
            // Second piece beyond must be an enemy slider aligned with the ray
            // — and strictly cheaper than the shielded piece, otherwise the
            // "pin" is toothless (same-tier B→B, R→R, Q→Q or a pricier
            // pinner never wants the trade).
            nf += df;
            nr += dr;
            loop {
                match try_sq(nf, nr) {
                    None => break,
                    Some(s) => match pos.board[s as usize] {
                        None => {
                            nf += df;
                            nr += dr;
                        }
                        Some(piece)
                            if piece.color != shield_color
                                && silver_value(piece.kind) < silver_value(shield_kind)
                                && matches!(
                                    (straight, piece.kind),
                                    (true, Kind::Rook)
                                        | (true, Kind::Queen)
                                        | (false, Kind::Bishop)
                                        | (false, Kind::Queen)
                                ) =>
                        {
                            pins.insert(victim, s);
                            break;
                        }
                        _ => break,
                    },
                }
            }
        }
    }
    pins
}

/// Victim squares of relative pins (silver highlight). Absolute-pin
/// victims are excluded (gray wins).
pub fn silver_squares(pos: &Position) -> Vec<u8> {
    let mut out: Vec<u8> = silver_map(pos).keys().copied().collect();
    out.sort_unstable();
    out
}

/// Combined pin map for attack credit: absolute pins win ties over
/// silver ones. A pinned piece (either kind) attacks only its pinner's
/// square — B/P/R behind their pinner still eye it back when the
/// geometry reaches, N-like pieces never do.
fn combined_pins(pos: &Position) -> std::collections::HashMap<u8, u8> {
    let mut pins = silver_map(pos);
    for (k, v) in pinned_map(pos) {
        pins.insert(k, v);
    }
    pins
}

/// True when an `attacker`-kind piece threatening a `victim`-kind piece
/// always forces a hot highlight (cheaper piece eyes pricier piece),
/// regardless of defender counts:
///
/// - pawn → knight | bishop | rook | queen
/// - knight | bishop → rook | queen
/// - rook → queen
///
/// Pawn↔pawn and knight↔bishop threats never trigger (count rule only).
fn value_triggered(victim: Kind, attacker: Kind) -> bool {
    match victim {
        Kind::Pawn | Kind::King => false,
        Kind::Knight | Kind::Bishop => attacker == Kind::Pawn,
        Kind::Rook => matches!(attacker, Kind::Pawn | Kind::Knight | Kind::Bishop),
        Kind::Queen => matches!(
            attacker,
            Kind::Pawn | Kind::Knight | Kind::Bishop | Kind::Rook
        ),
    }
}

/// Hot squares (orange highlight):
///
/// 1. Occupied non-king squares where a cheaper piece threatens a
///    pricier one (see [`value_triggered`]) OR enemy attackers outnumber
///    own defenders (`A > D`).
/// 2. King danger ring: neighbor squares of both kings (even empty ones)
///    where enemy attackers outnumber own defenders, counting the king
///    itself as one defender. Ring squares glow only on genuine
///    overloads (2v0, 3v1, …), never on a single eye.
///
/// Pseudo-legal counting, pin-aware: a pinned piece (absolute or silver)
/// attacks only its pinner's square (capturing the pinner aside, its moves
/// are illegal — see [`pinned_map`] / [`silver_map`]). Battery-aware: stacked
/// own sliders behind the first own blocker on any ray all count (unlimited
/// chain, both colors).
/// Known gap: a pinned capture of the pinner that
/// would expose the king to a *second* attacker still counts.
/// King squares are excluded (check already renders red).
pub fn hanging_squares(pos: &Position) -> Vec<u8> {
    let pins = combined_pins(pos);
    let mut out = Vec::new();
    for (i, p) in pos.board.iter().enumerate() {
        let piece = match p {
            Some(piece) => *piece,
            None => continue,
        };
        if piece.kind == Kind::King {
            continue;
        }
        let sq = i as u8;
        let attackers = attackers_of(pos, sq, piece.color.opp(), &pins);
        if attackers.is_empty() {
            continue;
        }
        if attackers.iter().any(|a| value_triggered(piece.kind, *a)) {
            if !out.contains(&sq) {
                out.push(sq);
            }
            continue;
        }
        let defenders = attackers_of(pos, sq, piece.color, &pins).len();
        if attackers.len() > defenders {
            out.push(sq);
        }
    }
    for (i, p) in pos.board.iter().enumerate() {
        let king_color = match p {
            Some(piece) if piece.kind == Kind::King => piece.color,
            _ => continue,
        };
        let kf = file_of(i as u8) as i8;
        let kr = rank_of(i as u8) as i8;
        for (df, dr) in KING_STEPS {
            let nsq = match try_sq(kf + df, kr + dr) {
                Some(s) => s,
                None => continue,
            };
            if out.contains(&nsq) {
                continue;
            }
            let a = attackers_of(pos, nsq, king_color.opp(), &pins).len();
            if a == 0 {
                continue;
            }
            // The king always defends its own ring: +1 on top of real
            // defenders. Ring squares glow only on genuine overloads
            // (2v0, 3v1, …), never on a single eye.
            let d = attackers_of(pos, nsq, king_color, &pins)
                .into_iter()
                .filter(|k| *k != Kind::King)
                .count()
                + 1;
            if a > d {
                out.push(nsq);
            }
        }
    }
    out
}

// ── Game status ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Ongoing,
    Checkmate(Color), // winner
    Stalemate,
    DrawFifty,
    DrawRepetition,
    DrawMaterial,
}

pub fn evaluate(pos: &Position) -> Status {
    if all_legal(pos).is_empty() {
        if in_check(pos, pos.turn) {
            return Status::Checkmate(pos.turn.opp());
        } else {
            return Status::Stalemate;
        }
    }
    if insufficient_material(pos) {
        return Status::DrawMaterial;
    }
    if pos.halfmove >= 100 {
        return Status::DrawFifty;
    }
    if repetition_count(pos) >= 3 {
        return Status::DrawRepetition;
    }
    Status::Ongoing
}

// ── SAN ───────────────────────────────────────────────────────────────────

/// Standard Algebraic Notation for `mv` in `pos` (mv must be legal).
/// Includes disambiguation, `x`, `=Q`, `+` and `#`.
pub fn to_san(pos: &Position, mv: Move) -> String {
    let piece = pos.board[mv.from as usize].expect("san on empty square");
    let is_castle_k =
        piece.kind == Kind::King && (file_of(mv.from) as i8 - file_of(mv.to) as i8).abs() == 2;

    let mut san = String::new();
    if is_castle_k {
        if file_of(mv.to) == 6 {
            san.push_str("O-O");
        } else {
            san.push_str("O-O-O");
        }
    } else {
        let is_capture = matches!(pos.board[mv.to as usize], Some(p) if p.color != piece.color)
            || (piece.kind == Kind::Pawn
                && pos.ep == Some(mv.to)
                && pos.board[mv.to as usize].is_none());
        if piece.kind == Kind::Pawn {
            if is_capture {
                san.push((b'a' + file_of(mv.from)) as char);
                san.push('x');
            }
        } else {
            san.push_str(piece.kind.letter());
            san.push_str(&disambiguation(pos, mv));
            if is_capture {
                san.push('x');
            }
        }
        san.push_str(&sq_name(mv.to));
        if let Some(k) = mv.promo {
            san.push('=');
            san.push_str(k.letter());
        }
    }

    // Check / mate suffix from the resulting position.
    let mut after = pos.clone();
    after.hashes.pop();
    apply_move(&mut after, mv);
    after.hashes.pop();
    match evaluate(&after) {
        Status::Checkmate(w) if w == piece.color => san.push('#'),
        _ if in_check(&after, after.turn) => san.push('+'),
        _ => {}
    }
    san
}

/// File/rank/both disambiguation for non-pawn moves.
fn disambiguation(pos: &Position, mv: Move) -> String {
    let piece = pos.board[mv.from as usize].expect("disamb on empty");
    if piece.kind == Kind::Pawn || piece.kind == Kind::King {
        return String::new();
    }
    let mut others: Vec<u8> = Vec::new();
    for sq in 0..64u8 {
        if sq == mv.from {
            continue;
        }
        if pos.board[sq as usize] != Some(piece) {
            continue;
        }
        // Same piece kind+color that can also legally reach `to`.
        if legal_moves_for(pos, sq).iter().any(|m| m.to == mv.to) {
            others.push(sq);
        }
    }
    if others.is_empty() {
        return String::new();
    }
    let f = file_of(mv.from);
    let r = rank_of(mv.from);
    let same_file = others.iter().any(|s| file_of(*s) == f);
    let same_rank = others.iter().any(|s| rank_of(*s) == r);
    if !same_file {
        format!("{}", (b'a' + f) as char)
    } else if !same_rank {
        format!("{}", (b'1' + r) as char)
    } else {
        sq_name(mv.from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mv(from: &str, to: &str) -> Move {
        Move {
            from: parse_sq(from).unwrap(),
            to: parse_sq(to).unwrap(),
            promo: None,
        }
    }

    fn play(pos: &mut Position, from: &str, to: &str, promo: Option<Kind>) {
        let m = Move {
            from: parse_sq(from).unwrap(),
            to: parse_sq(to).unwrap(),
            promo,
        };
        let legal = legal_moves_for(pos, m.from);
        assert!(
            legal.contains(&m),
            "illegal fixture move {}{} in {:?}",
            from,
            to,
            legal
                .iter()
                .map(|x| format!("{}{}", sq_name(x.from), sq_name(x.to)))
                .collect::<Vec<_>>()
        );
        apply_move(pos, m);
    }

    #[test]
    fn initial_move_count_is_20() {
        let pos = initial_position();
        assert_eq!(all_legal(&pos).len(), 20);
    }

    #[test]
    fn pawn_double_first_then_single_only() {
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        // Probe White's options from this board (turn is Black's now).
        pos.turn = Color::White;
        // e4 pawn can only go e5 now (double push spent).
        let targets: Vec<u8> = legal_moves_for(&pos, parse_sq("e4").unwrap())
            .iter()
            .map(|m| m.to)
            .collect();
        assert_eq!(targets, vec![parse_sq("e5").unwrap()]);
    }

    #[test]
    fn pawn_double_blocked() {
        let mut pos = initial_position();
        // 1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 — e-pawn blocked by e5... use d-file:
        // 1. d4 d5 blocks d4 pawn.
        play(&mut pos, "d2", "d4", None);
        play(&mut pos, "d7", "d5", None);
        let targets: Vec<u8> = legal_moves_for(&pos, parse_sq("d4").unwrap())
            .iter()
            .map(|m| m.to)
            .collect();
        assert!(targets.is_empty());
    }

    #[test]
    fn en_passant_window() {
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "a7", "a6", None);
        play(&mut pos, "e4", "e5", None);
        play(&mut pos, "d7", "d5", None);
        // exd6 e.p. available…
        let targets: Vec<u8> = legal_moves_for(&pos, parse_sq("e5").unwrap())
            .iter()
            .map(|m| m.to)
            .collect();
        assert!(targets.contains(&parse_sq("d6").unwrap()));
        // …but expires after another move.
        play(&mut pos, "g1", "f3", None);
        play(&mut pos, "g8", "f6", None);
        let targets: Vec<u8> = legal_moves_for(&pos, parse_sq("e5").unwrap())
            .iter()
            .map(|m| m.to)
            .collect();
        assert!(!targets.contains(&parse_sq("d6").unwrap()));
        // SAN form of an executed e.p. capture.
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "a7", "a6", None);
        play(&mut pos, "e4", "e5", None);
        play(&mut pos, "d7", "d5", None);
        assert_eq!(to_san(&pos, mv("e5", "d6")), "exd6".to_string());
    }

    #[test]
    fn kingside_castle_and_rights() {
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "e7", "e5", None);
        play(&mut pos, "g1", "f3", None);
        play(&mut pos, "b8", "c6", None);
        play(&mut pos, "f1", "c4", None);
        play(&mut pos, "f8", "c5", None);
        let king_moves = legal_moves_for(&pos, parse_sq("e1").unwrap());
        assert!(king_moves.contains(&mv("e1", "g1")));
        assert_eq!(to_san(&pos, mv("e1", "g1")), "O-O".to_string());
        play(&mut pos, "e1", "g1", None);
        assert_eq!(pos.castle & (WK | WQ), 0);
        // Rook hopped.
        assert_eq!(
            pos.board[parse_sq("f1").unwrap() as usize].map(|p| p.kind),
            Some(Kind::Rook)
        );
    }

    #[test]
    fn castle_through_check_denied() {
        // White: Ke1, Rh1, pawns cleared; black bishop on c4 attacks f1.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: WK,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        pos.board[parse_sq("e1").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::White,
        });
        pos.board[parse_sq("h1").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::White,
        });
        pos.board[parse_sq("c4").unwrap() as usize] = Some(Piece {
            kind: Kind::Bishop,
            color: Color::Black,
        });
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.hashes.push(hash_position(&pos));
        let king_moves = legal_moves_for(&pos, parse_sq("e1").unwrap());
        assert!(!king_moves.contains(&mv("e1", "g1")));
        // …but normal king steps elsewhere still exist.
        assert!(king_moves.contains(&mv("e1", "e2")) || king_moves.contains(&mv("e1", "d1")));
    }

    #[test]
    fn scholars_mate_is_checkmate_with_hash_suffix() {
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "e7", "e5", None);
        play(&mut pos, "d1", "h5", None);
        play(&mut pos, "b8", "c6", None);
        play(&mut pos, "f1", "c4", None);
        play(&mut pos, "g8", "f6", None);
        assert_eq!(to_san(&pos, mv("h5", "f7")), "Qxf7#".to_string());
        play(&mut pos, "h5", "f7", None);
        assert_eq!(evaluate(&pos), Status::Checkmate(Color::White));
    }

    #[test]
    fn stalemate_fixture() {
        // Kf7 vs Kh8 pawn g6: black to move, no legal moves, not in check.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::Black,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("f7", Kind::King, Color::White),
            ("g6", Kind::Pawn, Color::White),
            ("h8", Kind::King, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        assert_eq!(evaluate(&pos), Status::Stalemate);
    }

    #[test]
    fn promotion_choices_and_san() {
        // White pawn a7, white king h1, black king h8: a8=Q/R/B/N all legal.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("a7", Kind::Pawn, Color::White),
            ("h1", Kind::King, Color::White),
            ("h8", Kind::King, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        let promos: Vec<Option<Kind>> = legal_moves_for(&pos, parse_sq("a7").unwrap())
            .iter()
            .map(|m| m.promo)
            .collect();
        assert_eq!(promos.len(), 4);
        let m = Move {
            from: parse_sq("a7").unwrap(),
            to: parse_sq("a8").unwrap(),
            promo: Some(Kind::Queen),
        };
        assert_eq!(to_san(&pos, m), "a8=Q+".to_string());
    }

    #[test]
    fn san_disambiguation_file_rank_both() {
        // File case: knights b1 + f3, both reach d2 → Nbd2 (files differ).
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("e1", Kind::King, Color::White),
            ("b1", Kind::Knight, Color::White),
            ("f3", Kind::Knight, Color::White),
            ("e8", Kind::King, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        assert_eq!(to_san(&pos, mv("b1", "d2")), "Nbd2".to_string());
        assert_eq!(to_san(&pos, mv("f3", "d2")), "Nfd2".to_string());

        // Rank case: rooks a1 + a8-path… use rooks a1/a5? same file → rank digit.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("e4", Kind::King, Color::White),
            ("a1", Kind::Rook, Color::White),
            ("a5", Kind::Rook, Color::White),
            ("h8", Kind::King, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        assert_eq!(to_san(&pos, mv("a1", "a3")), "R1a3".to_string());
        assert_eq!(to_san(&pos, mv("a5", "a3")), "R5a3".to_string());
    }

    #[test]
    fn san_capture_check_suffix() {
        // Bg5 takes f6 pawn delivering check (king e8, no escape after…).
        // Just assert capture + suffix shape on a crafted line.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("e1", Kind::King, Color::White),
            ("g5", Kind::Bishop, Color::White),
            ("e8", Kind::King, Color::Black),
            ("f6", Kind::Pawn, Color::Black),
            ("d7", Kind::Pawn, Color::Black),
            ("f7", Kind::Pawn, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        let san = to_san(&pos, mv("g5", "f6"));
        assert!(san.starts_with("Bxf6"), "got {}", san);
    }

    #[test]
    fn threefold_repetition_auto_draw() {
        let mut pos = initial_position();
        // Knights shuttle: Nf3 Nf6 Ng1 Ng8 ×2 → initial repeats 3×.
        for _ in 0..2 {
            play(&mut pos, "g1", "f3", None);
            play(&mut pos, "g8", "f6", None);
            play(&mut pos, "f3", "g1", None);
            play(&mut pos, "f6", "g8", None);
        }
        assert_eq!(evaluate(&pos), Status::DrawRepetition);
    }

    #[test]
    fn fifty_move_rule() {
        let mut pos = initial_position();
        pos.halfmove = 100;
        pos.hashes = vec![0]; // avoid accidental repetition counts
        assert_eq!(evaluate(&pos), Status::DrawFifty);
    }

    #[test]
    fn insufficient_material_cases() {
        // Bare kings.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        pos.board[parse_sq("e1").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::White,
        });
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.hashes.push(hash_position(&pos));
        assert!(insufficient_material(&pos));

        // K+N vs K → draw.
        pos.board[parse_sq("c3").unwrap() as usize] = Some(Piece {
            kind: Kind::Knight,
            color: Color::White,
        });
        assert!(insufficient_material(&pos));

        // K+R vs K → not draw.
        pos.board[parse_sq("c3").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::White,
        });
        assert!(!insufficient_material(&pos));

        // K+B vs K+B, same square color → draw.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("e1", Kind::King, Color::White),
            ("c1", Kind::Bishop, Color::White), // c1: dark
            ("e8", Kind::King, Color::Black),
            ("f8", Kind::Bishop, Color::Black), // f8: dark
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        assert!(insufficient_material(&pos));
    }

    #[test]
    fn pinned_piece_has_no_legal_moves() {
        // White Ke1, white Bf1 pinned by black Ra1-line rook? Craft: black
        // rook e8, white king e1, white knight e2 — knight cannot move.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        for (sq, kind, color) in [
            ("e1", Kind::King, Color::White),
            ("e2", Kind::Knight, Color::White),
            ("e8", Kind::Rook, Color::Black),
            ("h8", Kind::King, Color::Black),
        ] {
            pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
        }
        pos.hashes.push(hash_position(&pos));
        assert!(legal_moves_for(&pos, parse_sq("e2").unwrap()).is_empty());
    }

    fn empty_pos(turn: Color) -> Position {
        let mut pos = Position {
            board: [None; 64],
            turn,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        pos.board[parse_sq("e1").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::White,
        });
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.hashes.push(hash_position(&pos));
        pos
    }

    fn put(pos: &mut Position, sq: &str, kind: Kind, color: Color) {
        pos.board[parse_sq(sq).unwrap() as usize] = Some(Piece { kind, color });
    }

    #[test]
    fn hanging_empty_initial_position() {
        let pos = initial_position();
        assert!(hanging_squares(&pos).is_empty());
    }

    #[test]
    fn hanging_defended_piece_not_hot() {
        // White knight d4 attacked by black knight c6, defended by white
        // pawn c3 (c3 attacks b4/d4) → 1v1, not hot.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Knight, Color::White);
        put(&mut pos, "c6", Kind::Knight, Color::Black);
        put(&mut pos, "c3", Kind::Pawn, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "defended 1v1 must not be hot: {:?}",
            hot
        );
    }

    #[test]
    fn hanging_outnumbered_piece_is_hot() {
        // White queen d4 attacked by black rooks d8 + a4, defended once by
        // white rook d1 → 2v1, hot. Kings far away.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Queen, Color::White);
        put(&mut pos, "d8", Kind::Rook, Color::Black);
        put(&mut pos, "a4", Kind::Rook, Color::Black);
        put(&mut pos, "d1", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "2v1 queen must be hot: {:?}",
            hot
        );
    }

    #[test]
    fn hanging_skips_kings() {
        // Black king surrounded is never listed even when attacked.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e7", Kind::Queen, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("e8").unwrap()),
            "king must never be hot: {:?}",
            hot
        );
    }

    #[test]
    fn value_pawn_threatens_knight_hot_despite_defense() {
        // 1v1 would be quiet, but pawn→knight always forces hot.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Knight, Color::White);
        put(&mut pos, "c5", Kind::Pawn, Color::Black); // attacks b4+d4
        put(&mut pos, "c3", Kind::Pawn, Color::White); // defends d4
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "pawn-eyed knight must be hot: {:?}",
            hot
        );
    }

    #[test]
    fn value_bishop_threatens_queen_hot_despite_defense() {
        // Black bishop a1 eyes d4 down the open diagonal; white pawn e3
        // defends (1v1) — value rule still fires.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Queen, Color::White);
        put(&mut pos, "a1", Kind::Bishop, Color::Black);
        put(&mut pos, "e3", Kind::Pawn, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "bishop-eyed queen must be hot: {:?}",
            hot
        );
    }

    #[test]
    fn value_rook_threatens_queen_hot_despite_defense() {
        // Black rook d8 eyes d4 down the file; white rook d1 defends
        // (1v1) — value rule still fires.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Queen, Color::White);
        put(&mut pos, "d8", Kind::Rook, Color::Black);
        put(&mut pos, "d1", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "rook-eyed queen must be hot: {:?}",
            hot
        );
    }

    #[test]
    fn value_same_tier_never_triggers() {
        // Rook→rook and pawn→pawn at 1v1 stay quiet (count rule only).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Rook, Color::White);
        put(&mut pos, "d8", Kind::Rook, Color::Black);
        put(&mut pos, "d1", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "rook-eyed rook at 1v1 must not be hot: {:?}",
            hot
        );
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Pawn, Color::White);
        put(&mut pos, "c5", Kind::Pawn, Color::Black);
        put(&mut pos, "c3", Kind::Pawn, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "pawn-eyed pawn at 1v1 must not be hot: {:?}",
            hot
        );
    }

    #[test]
    fn ring_marks_undefended_escape_squares() {
        // White king e1; black queen d3 + rook e7. e2 faces two attackers
        // vs just the king → hot. d1/d2/f1 face one attacker each with the
        // king counting as defender → quiet. f2 faces none and c1 is out
        // of reach → quiet.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d3", Kind::Queen, Color::Black);
        put(&mut pos, "e7", Kind::Rook, Color::Black);
        let hot = hanging_squares(&pos);
        for sq in ["e2"] {
            assert!(
                hot.contains(&parse_sq(sq).unwrap()),
                "{} must be hot: {:?}",
                sq,
                hot
            );
        }
        for sq in ["d1", "d2", "f1", "f2", "c1"] {
            assert!(
                !hot.contains(&parse_sq(sq).unwrap()),
                "{} must not be hot: {:?}",
                sq,
                hot
            );
        }
    }

    #[test]
    fn ring_covers_both_kings_and_board_edges() {
        // Edge king h1; black queen g3 + bishop c6. g2 faces two attackers
        // vs just the king → hot. h2 faces one with the king counting as
        // defender → quiet. g1 faces one but white knight e2 defends →
        // quiet. No off-board panic.
        let mut pos = Position {
            board: [None; 64],
            turn: Color::White,
            castle: 0,
            ep: None,
            halfmove: 0,
            hashes: vec![],
        };
        pos.board[parse_sq("h1").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::White,
        });
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.hashes.push(hash_position(&pos));
        put(&mut pos, "g3", Kind::Queen, Color::Black);
        put(&mut pos, "c6", Kind::Bishop, Color::Black);
        put(&mut pos, "e2", Kind::Knight, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("g2").unwrap()),
            "g2 must be hot: {:?}",
            hot
        );
        assert!(
            !hot.contains(&parse_sq("h2").unwrap()),
            "h2 with the king counting as defender must not be hot: {:?}",
            hot
        );
        assert!(
            !hot.contains(&parse_sq("g1").unwrap()),
            "g1 defended 1v1 must not be hot: {:?}",
            hot
        );
    }

    #[test]
    fn pin_detected_on_open_file() {
        // Ke1, white Ne2, black Re8 with a clear e-file: e2 pinned by e8.
        // A second own piece in between (white pawn e3) breaks the pin.
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        put(&mut pos, "e2", Kind::Knight, Color::White);
        let pins = pinned_map(&pos);
        assert_eq!(
            pins.get(&parse_sq("e2").unwrap()),
            Some(&parse_sq("e8").unwrap())
        );
        assert_eq!(pinned_squares(&pos), vec![parse_sq("e2").unwrap()]);
        put(&mut pos, "e3", Kind::Pawn, Color::White);
        assert!(pinned_map(&pos).is_empty());
        assert!(pinned_squares(&pos).is_empty());
    }

    #[test]
    fn pinned_knight_contributes_nothing_off_ray() {
        // Same pin, plus a black bishop on d4 (a natural Ne2 target).
        // The pinned knight's attack is switched off: d4 stays quiet
        // (without pin-awareness it would be hot, 1v0).
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        put(&mut pos, "e2", Kind::Knight, Color::White);
        put(&mut pos, "d4", Kind::Bishop, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "pinned knight must not light d4: {:?}",
            hot
        );
        // The pin victim itself is still evaluated normally (1v1 quiet).
        assert!(!hot.contains(&parse_sq("e2").unwrap()));
    }

    #[test]
    fn pinned_rook_attack_back_counts_on_pinner_only() {
        // Ke1, white Re2 pinned by black Re8 (e-file clear). The rook's
        // attack back counts on e8 (hot, 1v0) but its ray attacks
        // elsewhere would not (e4 tested via the diagonal fixture below).
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        put(&mut pos, "e2", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("e8").unwrap()),
            "attack-back on the pinner must count: {:?}",
            hot
        );
    }

    #[test]
    fn pinned_bishop_diagonal_attack_back_and_drop() {
        // White Ke1, white Bd2 pinned by black Ba5 (a5-b4-c3-d2-e1 clear).
        // Black Ne3 sits on the victim's ray but off the pin: quiet
        // (without pin-awareness it would be hot, 1v0). The pinner a5 is
        // hot via the attack back.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d2", Kind::Bishop, Color::White);
        put(&mut pos, "a5", Kind::Bishop, Color::Black);
        put(&mut pos, "e3", Kind::Knight, Color::Black);
        let pins = pinned_map(&pos);
        assert_eq!(
            pins.get(&parse_sq("d2").unwrap()),
            Some(&parse_sq("a5").unwrap())
        );
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("a5").unwrap()),
            "attack-back on the pinner must count: {:?}",
            hot
        );
        assert!(
            !hot.contains(&parse_sq("e3").unwrap()),
            "pinned bishop must not light e3: {:?}",
            hot
        );
        // The victim's own square still evaluates normally (1v1 quiet).
        assert!(!hot.contains(&parse_sq("d2").unwrap()));
    }

    #[test]
    fn queen_pins_f7_pawn() {
        // 1.e4 e5 2.Qh5 genuinely pins the f7 pawn on the h5-e8
        // diagonal (moving it exposes check) — and nothing else.
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "e7", "e5", None);
        play(&mut pos, "d1", "h5", None);
        let pins = pinned_map(&pos);
        assert_eq!(
            pins.get(&parse_sq("f7").unwrap()),
            Some(&parse_sq("h5").unwrap())
        );
        assert_eq!(pinned_squares(&pos), vec![parse_sq("f7").unwrap()]);
    }

    #[test]
    fn pinned_pawn_cannot_reach_pinner_gives_no_credit() {
        // 1.e4 e5 2.Qh5 pins f7 — but the pawn's geometry never reaches
        // h5, so the queen gets no phantom attacker (regression: credit
        // used to apply without a geometry check).
        let mut pos = initial_position();
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "e7", "e5", None);
        play(&mut pos, "d1", "h5", None);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("h5").unwrap()),
            "queen must not feel a phantom pawn attack: {:?}",
            hot
        );
        // e5 is genuinely hanging (queen eyes it, undefended) — sanity
        // that counting still works around the pin.
        assert!(
            hot.contains(&parse_sq("e5").unwrap()),
            "e5 must stay hot: {:?}",
            hot
        );
    }

    #[test]
    fn no_pins_in_opening_line() {
        // 1.e4 e5 2.Nf3 Nc6 creates no pin geometry (contrast 2.Qh5,
        // which genuinely pins the f7 pawn on the h5-e8 diagonal):
        // pin sets stay empty so the feature changes nothing here.
        let mut pos = initial_position();
        assert!(pinned_squares(&pos).is_empty());
        play(&mut pos, "e2", "e4", None);
        play(&mut pos, "e7", "e5", None);
        play(&mut pos, "g1", "f3", None);
        play(&mut pos, "b8", "c6", None);
        assert!(pinned_squares(&pos).is_empty());
    }

    #[test]
    fn battery_queen_behind_pawn_defends_bishop() {
        // bB d4, bP e5, bQ f6 on one diagonal: pawn defends directly,
        // queen batteries through it → D=2. Two white attackers (knights
        // c6 + e6, same-tier so the value rule stays out) → 2v2 quiet.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Bishop, Color::Black);
        put(&mut pos, "e5", Kind::Pawn, Color::Black);
        put(&mut pos, "f6", Kind::Queen, Color::Black);
        put(&mut pos, "c6", Kind::Knight, Color::White);
        put(&mut pos, "e6", Kind::Knight, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "battery 2v2 must not be hot: {:?}",
            hot
        );
        // Without the queen it is 2v1 → hot (sanity that the queen
        // is the piece making the difference).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Bishop, Color::Black);
        put(&mut pos, "e5", Kind::Pawn, Color::Black);
        put(&mut pos, "c6", Kind::Knight, Color::White);
        put(&mut pos, "e6", Kind::Knight, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "2v1 without battery must be hot: {:?}",
            hot
        );
    }

    #[test]
    fn battery_rook_file_unlimited_chain() {
        // White pawn d4 backed by rooks d2 + d3? No — d3 must stay empty
        // so d2 anchors directly and d1 batteries through it: D=2 vs two
        // black rooks (d8 + a4... a4 eyes d4 along the rank) → quiet.
        // Rook↔pawn never value-triggers, so this is pure count logic.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Pawn, Color::White);
        put(&mut pos, "d2", Kind::Rook, Color::White);
        put(&mut pos, "d1", Kind::Rook, Color::White);
        put(&mut pos, "d8", Kind::Rook, Color::Black);
        put(&mut pos, "a4", Kind::Rook, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "file battery 2v2 must not be hot: {:?}",
            hot
        );
    }

    #[test]
    fn battery_attacker_symmetry_bishop_behind_pawn() {
        // Mirror: white pawn c3 + bishop b2 both eye black d4 (bishop
        // x-rays through its own pawn) → A=2 vs one defender (black pawn
        // e5) → hot. Pawn victim keeps the value rule out (pawn↔pawn and
        // bishop→pawn never trigger), so this is pure count logic.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Pawn, Color::Black);
        put(&mut pos, "c3", Kind::Pawn, Color::White);
        put(&mut pos, "b2", Kind::Bishop, Color::White);
        put(&mut pos, "e5", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("d4").unwrap()),
            "battery attack 2v1 must be hot: {:?}",
            hot
        );
        // Pawn alone is 1v1 → quiet.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Pawn, Color::Black);
        put(&mut pos, "c3", Kind::Pawn, Color::White);
        put(&mut pos, "e5", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "single pawn 1v1 must not be hot: {:?}",
            hot
        );
    }

    #[test]
    fn battery_enemy_breaks_chain_and_needs_anchor() {
        // Enemy piece between anchor and battery stops the chain: white
        // pawn c3 anchors on d4, but a BLACK pawn on b2 sits between the
        // white bishop a1 and the target → bishop must not count (1v1
        // quiet; pawn victim keeps the value rule out).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d4", Kind::Pawn, Color::Black);
        put(&mut pos, "c3", Kind::Pawn, Color::White);
        put(&mut pos, "a1", Kind::Bishop, Color::White);
        put(&mut pos, "b2", Kind::Pawn, Color::Black);
        put(&mut pos, "e5", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("d4").unwrap()),
            "blocked battery must stay 1v1 quiet: {:?}",
            hot
        );
        // No direct anchor → no phantom battery: black Rh8 behind own
        // Ph7 must not invent an attack on an otherwise unattacked Qh5.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "h5", Kind::Queen, Color::White);
        put(&mut pos, "h7", Kind::Pawn, Color::Black);
        pos.board[parse_sq("h8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("h5").unwrap()),
            "unanchored x-ray must not light h5: {:?}",
            hot
        );
    }

    #[test]
    fn silver_knight_pinned_to_queen() {
        // wQ d1, wN f3, bB g4 with e2 empty: f3 shields the pricier
        // queen → silver (not gray), and the knight's attacks are cut
        // (same credit rule as king pins).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d1", Kind::Queen, Color::White);
        put(&mut pos, "f3", Kind::Knight, Color::White);
        put(&mut pos, "g4", Kind::Bishop, Color::Black);
        let silver = silver_squares(&pos);
        assert_eq!(silver, vec![parse_sq("f3").unwrap()]);
        assert!(pinned_squares(&pos).is_empty());
        let pins = silver_map(&pos);
        assert_eq!(
            pins.get(&parse_sq("f3").unwrap()),
            Some(&parse_sq("g4").unwrap())
        );
        // Blocker on e2 breaks the ray → no silver.
        put(&mut pos, "e2", Kind::Pawn, Color::White);
        assert!(silver_squares(&pos).is_empty());
    }

    #[test]
    fn silver_needs_cheaper_victim() {
        // wQ e2 shielding wR e1 from bR e8: victim queen (9) is pricier
        // than the rook (5) → no silver.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e1", Kind::Rook, Color::White);
        put(&mut pos, "e2", Kind::Queen, Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        assert!(silver_squares(&pos).is_empty());
        // Flipped: wR e2 (5) shielding wQ e1 (9) → silver on e2.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e1", Kind::Queen, Color::White);
        put(&mut pos, "e2", Kind::Rook, Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        assert_eq!(silver_squares(&pos), vec![parse_sq("e2").unwrap()]);
    }

    #[test]
    fn silver_attack_credit_matches_king_pins() {
        // Silver knight lights nothing off-ray: bB g4 pins wN f3 to wQ
        // d1; black pawn e4 sits on a natural Nf3 square but must stay
        // quiet (without silver-awareness it would be hot, 1v0).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d1", Kind::Queen, Color::White);
        put(&mut pos, "f3", Kind::Knight, Color::White);
        put(&mut pos, "g4", Kind::Bishop, Color::Black);
        put(&mut pos, "e4", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("e4").unwrap()),
            "silver knight must not light e4: {:?}",
            hot
        );
        // Silver bishop still eyes its pinner back: wB f3 shielding wQ
        // d1 from bB g4 → g4 hot via the attack back (1v0).
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d1", Kind::Queen, Color::White);
        put(&mut pos, "f3", Kind::Bishop, Color::White);
        put(&mut pos, "g4", Kind::Bishop, Color::Black);
        assert_eq!(silver_squares(&pos), vec![parse_sq("f3").unwrap()]);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("g4").unwrap()),
            "silver bishop attack-back must light g4: {:?}",
            hot
        );
    }

    #[test]
    fn silver_yields_to_gray_on_double_pin() {
        // d2 is absolute-pinned (Ba5 on the a5-e1 diagonal) and also
        // shields Qd1 from Rd8 on the d-file → gray wins, silver empty.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d2", Kind::Bishop, Color::White);
        put(&mut pos, "a5", Kind::Bishop, Color::Black);
        put(&mut pos, "d1", Kind::Queen, Color::White);
        put(&mut pos, "d8", Kind::Rook, Color::Black);
        assert_eq!(pinned_squares(&pos), vec![parse_sq("d2").unwrap()]);
        assert!(silver_squares(&pos).is_empty());
    }

    #[test]
    fn silver_needs_cheaper_pinner() {
        // Same-tier pinners never want the trade: bishop→bishop,
        // rook→rook and queen→queen stay dark even with a cheaper
        // victim in between.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "c1", Kind::Bishop, Color::White);
        put(&mut pos, "b2", Kind::Pawn, Color::White);
        put(&mut pos, "a3", Kind::Bishop, Color::Black);
        assert!(silver_squares(&pos).is_empty());
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e1", Kind::Queen, Color::White);
        put(&mut pos, "e2", Kind::Pawn, Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Queen,
            color: Color::Black,
        });
        assert!(silver_squares(&pos).is_empty());
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e1", Kind::Rook, Color::White);
        put(&mut pos, "e2", Kind::Pawn, Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        assert!(silver_squares(&pos).is_empty());
        // Pricier pinner eyeing cheaper shield: bQ g4 pins wN f3 to wR
        // d1 — queen (9) wants nothing with a rook (5) → dark.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d1", Kind::Rook, Color::White);
        put(&mut pos, "f3", Kind::Knight, Color::White);
        put(&mut pos, "g4", Kind::Queen, Color::Black);
        assert!(silver_squares(&pos).is_empty());
        // Cheap pinners still light: bishop eyes queen, rook eyes queen.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "d1", Kind::Queen, Color::White);
        put(&mut pos, "f3", Kind::Knight, Color::White);
        put(&mut pos, "g4", Kind::Bishop, Color::Black);
        assert_eq!(silver_squares(&pos), vec![parse_sq("f3").unwrap()]);
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "e1", Kind::Queen, Color::White);
        put(&mut pos, "e2", Kind::Pawn, Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = Some(Piece {
            kind: Kind::Rook,
            color: Color::Black,
        });
        assert_eq!(silver_squares(&pos), vec![parse_sq("e2").unwrap()]);
    }

    #[test]
    fn battery_sealed_by_nonparticipating_pawn() {
        // Case 1: wP a3 defended by wP b2 (D=1). Black bishop c5 eyes
        // a3 directly, but bQ e7 sits behind bP d6 on the same diagonal
        // — the pawn never touches a3, never vacates, so the queen is
        // walled off: A=1, quiet. (Same-tier victim/attacker keeps the
        // value rule out: pawn↔pawn.)
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "a3", Kind::Pawn, Color::White);
        put(&mut pos, "b2", Kind::Pawn, Color::White);
        put(&mut pos, "c5", Kind::Bishop, Color::Black);
        put(&mut pos, "d6", Kind::Pawn, Color::Black);
        put(&mut pos, "e7", Kind::Queen, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("a3").unwrap()),
            "sealed queen must leave 1v1 quiet: {:?}",
            hot
        );
        // Control: pawn off the diagonal → queen joins through the
        // bishop anchor: A=2 vs D=1 → hot.
        let mut pos = empty_pos(Color::White);
        put(&mut pos, "a3", Kind::Pawn, Color::White);
        put(&mut pos, "b2", Kind::Pawn, Color::White);
        put(&mut pos, "c5", Kind::Bishop, Color::Black);
        put(&mut pos, "e7", Kind::Queen, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("a3").unwrap()),
            "open diagonal battery must be 2v1 hot: {:?}",
            hot
        );
    }

    #[test]
    fn battery_rook_walled_by_own_pawn() {
        // Case 2: black pawn f7 defended by Kg8 + Rf8 + Bg6 (D=3). White
        // attacks with Bc4 + Ne5 + Qf3, while wR f1 sits behind wP f2 on
        // the file — the pawn never touches f7, so the rook is sealed:
        // A=3, 3v3 quiet. Pawn victim keeps the value rule out.
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = None;
        pos.board[parse_sq("g8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.board[parse_sq("e1").unwrap() as usize] = None;
        put(&mut pos, "f7", Kind::Pawn, Color::Black);
        put(&mut pos, "f8", Kind::Rook, Color::Black);
        put(&mut pos, "g6", Kind::Bishop, Color::Black);
        put(&mut pos, "c4", Kind::Bishop, Color::White);
        put(&mut pos, "e5", Kind::Knight, Color::White);
        put(&mut pos, "f3", Kind::Queen, Color::White);
        put(&mut pos, "f2", Kind::Pawn, Color::White);
        put(&mut pos, "f1", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("f7").unwrap()),
            "walled rook must leave 3v3 quiet: {:?}",
            hot
        );
        // Control: f2 pawn gone → rook joins through the queen anchor:
        // A=4 vs D=3 → hot.
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e8").unwrap() as usize] = None;
        pos.board[parse_sq("g8").unwrap() as usize] = Some(Piece {
            kind: Kind::King,
            color: Color::Black,
        });
        pos.board[parse_sq("e1").unwrap() as usize] = None;
        put(&mut pos, "f7", Kind::Pawn, Color::Black);
        put(&mut pos, "f8", Kind::Rook, Color::Black);
        put(&mut pos, "g6", Kind::Bishop, Color::Black);
        put(&mut pos, "c4", Kind::Bishop, Color::White);
        put(&mut pos, "e5", Kind::Knight, Color::White);
        put(&mut pos, "f3", Kind::Queen, Color::White);
        put(&mut pos, "f1", Kind::Rook, Color::White);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("f7").unwrap()),
            "open file battery must be 4v3 hot: {:?}",
            hot
        );
    }

    #[test]
    fn battery_king_seals_ray() {
        // wK e2, wQ e1, bN e3, bP d4: the queen sits behind its own king
        // on the e-file — the king never vacates, so the queen can never
        // join. A=1 (king) vs D=1 (pawn): quiet. Knight victim + king
        // attacker never value-triggers.
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e1").unwrap() as usize] = None;
        put(&mut pos, "e2", Kind::King, Color::White);
        put(&mut pos, "e1", Kind::Queen, Color::White);
        put(&mut pos, "e3", Kind::Knight, Color::Black);
        put(&mut pos, "d4", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            !hot.contains(&parse_sq("e3").unwrap()),
            "queen behind king must leave 1v1 quiet: {:?}",
            hot
        );
        // Control: king off the file (d2) → queen joins through open
        // air... e1 still eyes e3 with e2 empty: A=2 vs D=1 → hot.
        let mut pos = empty_pos(Color::White);
        pos.board[parse_sq("e1").unwrap() as usize] = None;
        put(&mut pos, "d2", Kind::King, Color::White);
        put(&mut pos, "e1", Kind::Queen, Color::White);
        put(&mut pos, "e3", Kind::Knight, Color::Black);
        put(&mut pos, "d4", Kind::Pawn, Color::Black);
        let hot = hanging_squares(&pos);
        assert!(
            hot.contains(&parse_sq("e3").unwrap()),
            "open file must be 2v1 hot: {:?}",
            hot
        );
    }
}
