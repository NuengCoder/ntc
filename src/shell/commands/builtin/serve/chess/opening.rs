//! Opening book for `serve chess` bots.
//!
//! A bot at Normal strength or above follows a named opening for its first
//! three moves instead of handing the initial position straight to the UCI
//! engine. The book is keyed by position, not by a scripted line, so it
//! replies naturally to whatever the opponent plays (and to transpositions);
//! when the position leaves the book the engine takes over.
//!
//! Candidate moves are picked at random, which keeps successive games from
//! always starting the same way. Moves that would repeat a position during
//! the first 20 moves are never chosen, so a bot does not stumble into an
//! early threefold-repetition draw.

use super::engine::{
    all_legal, apply_move, initial_position, move_from_uci, sq_name, to_fen, Color, Move, Position,
};
use std::collections::HashMap;
use std::sync::OnceLock;

/// How many book moves each side may play before the engine takes over.
pub const BOOK_MOVES: usize = 3;
/// Positions may be repeated only after this many plies (20 moves).
pub const REPETITION_SAFE_PLIES: usize = 40;

/// Book lines in UCI notation, both colours included. A line only needs the
/// first few moves; extra depth is used for transpositions and replies.
const LINES: &[(&str, &[&str])] = &[
    (
        "Italian Game",
        &[
            "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5", "d2d3", "d7d6",
        ],
    ),
    (
        "Two Knights Defense",
        &["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6"],
    ),
    (
        "Ruy Lopez",
        &["e2e4", "e7e5", "g1f3", "b8c6", "f1b5", "a7a6"],
    ),
    (
        "Ruy Lopez",
        &["e2e4", "e7e5", "g1f3", "b8c6", "f1b5", "g8f6"],
    ),
    (
        "Scotch Game",
        &["e2e4", "e7e5", "g1f3", "b8c6", "d2d4", "e5d4"],
    ),
    (
        "Vienna Game",
        &["e2e4", "e7e5", "b1c3", "g8f6", "g1f3", "b8c6"],
    ),
    (
        "King's Gambit",
        &["e2e4", "e7e5", "f2f4", "e5f4", "g1f3", "d7d5"],
    ),
    (
        "King's Gambit",
        &["e2e4", "e7e5", "f2f4", "e5f4", "g1f3", "g7g5"],
    ),
    (
        "Ponziani Opening",
        &["e2e4", "e7e5", "g1f3", "b8c6", "c2c3", "d7d5"],
    ),
    (
        "Four Knights Game",
        &["e2e4", "e7e5", "g1f3", "b8c6", "b1c3", "g8f6"],
    ),
    (
        "Petrov Defense",
        &["e2e4", "e7e5", "g1f3", "g8f6", "f3e5", "d7d6"],
    ),
    (
        "Philidor Defense",
        &["e2e4", "e7e5", "g1f3", "d7d6", "d2d4", "g8f6"],
    ),
    (
        "Bishop's Opening",
        &["e2e4", "e7e5", "f1c4", "g8f6", "d2d3", "f8c5"],
    ),
    (
        "Danish Gambit",
        &["e2e4", "e7e5", "d2d4", "e5d4", "c2c3", "d4c3"],
    ),
    (
        "Sicilian Defense",
        &["e2e4", "c7c5", "g1f3", "d7d6", "d2d4", "c5d4"],
    ),
    (
        "Sicilian Defense",
        &["e2e4", "c7c5", "g1f3", "b8c6", "d2d4", "c5d4"],
    ),
    (
        "French Defense",
        &["e2e4", "e7e6", "d2d4", "d7d5", "b1c3", "g8f6"],
    ),
    (
        "Caro-Kann Defense",
        &["e2e4", "c7c6", "d2d4", "d7d5", "b1c3", "g8f6"],
    ),
    (
        "Caro-Kann Defense",
        &["e2e4", "c7c6", "d2d4", "d7d5", "e4d5", "c6d5"],
    ),
    (
        "Scandinavian Defense",
        &["e2e4", "d7d5", "e4d5", "d8d5", "b1c3", "d5a5"],
    ),
    (
        "Scandinavian Defense",
        &["e2e4", "d7d5", "e4d5", "g8f6", "d2d4", "f6d5"],
    ),
    (
        "Alekhine Defense",
        &["e2e4", "g8f6", "e4e5", "f6d5", "d2d4", "d7d6"],
    ),
    (
        "Owen's Defense",
        &["e2e4", "b7b6", "d2d4", "c8b7", "b1c3", "e7e6"],
    ),
    (
        "Pirc Defense",
        &["e2e4", "d7d6", "d2d4", "g8f6", "b1c3", "g7g6"],
    ),
    (
        "Modern Defense",
        &["e2e4", "g7g6", "d2d4", "f8g7", "b1c3", "d7d6"],
    ),
    (
        "Queen's Gambit",
        &["d2d4", "d7d5", "c2c4", "e7e6", "b1c3", "g8f6"],
    ),
    (
        "Queen's Gambit Accepted",
        &["d2d4", "d7d5", "c2c4", "d5c4", "g1f3", "g8f6"],
    ),
    (
        "Slav Defense",
        &["d2d4", "d7d5", "c2c4", "c7c6", "g1f3", "g8f6"],
    ),
    (
        "London System",
        &["d2d4", "d7d5", "g1f3", "g8f6", "c1f4", "e7e6"],
    ),
    (
        "London System",
        &["d2d4", "g8f6", "g1f3", "g7g6", "c1f4", "f8g7"],
    ),
    (
        "English Opening",
        &["c2c4", "e7e5", "b1c3", "g8f6", "g2g3", "d7d5"],
    ),
    (
        "King's Indian Defense",
        &["d2d4", "g8f6", "c2c4", "g7g6", "b1c3", "f8g7"],
    ),
    (
        "Nimzo-Indian Defense",
        &["d2d4", "g8f6", "c2c4", "e7e6", "b1c3", "f8b4"],
    ),
    (
        "Queen's Indian Defense",
        &["d2d4", "g8f6", "c2c4", "e7e6", "g1f3", "b7b6"],
    ),
    (
        "Grunfeld Defense",
        &["d2d4", "g8f6", "c2c4", "g7g6", "b1c3", "d7d5"],
    ),
    (
        "Catalan Opening",
        &["d2d4", "g8f6", "c2c4", "e7e6", "g1f3", "d7d5"],
    ),
    (
        "Benoni Defense",
        &["d2d4", "g8f6", "c2c4", "c7c5", "d4d5", "e7e6"],
    ),
    (
        "Dutch Defense",
        &["d2d4", "f7f5", "c2c4", "e7e6", "b1c3", "g8f6"],
    ),
    (
        "Reti Opening",
        &["g1f3", "d7d5", "c2c4", "e7e6", "b2b3", "g8f6"],
    ),
    (
        "Bird Opening",
        &["f2f4", "d7d5", "g1f3", "g8f6", "e2e3", "e7e6"],
    ),
];

fn book() -> &'static HashMap<String, Vec<&'static str>> {
    static BOOK: OnceLock<HashMap<String, Vec<&'static str>>> = OnceLock::new();
    BOOK.get_or_init(|| {
        let mut book: HashMap<String, Vec<&'static str>> = HashMap::new();
        for (_, line) in LINES {
            let mut pos = initial_position();
            for uci in *line {
                let Some(mv) = move_from_uci(&pos, uci) else {
                    break;
                };
                let moves = book.entry(position_key(&pos)).or_default();
                if !moves.contains(uci) {
                    moves.push(uci);
                }
                apply_move(&mut pos, mv);
            }
        }
        book
    })
}

/// Book key: board, turn, castling rights and en-passant target. Halfmove
/// and fullmove counters are deliberately left out so the same position
/// reached by a transposition still matches.
fn position_key(pos: &Position) -> String {
    let fen = to_fen(pos, 1);
    let mut parts = fen.split(' ');
    format!(
        "{} {} {} {}",
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default()
    )
}

/// How many moves `mover` has already played after `plies` plies.
fn moves_played(mover: Color, plies: usize) -> usize {
    match mover {
        Color::White => plies.div_ceil(2),
        Color::Black => plies / 2,
    }
}

pub fn book_supports_level(level: &str) -> bool {
    matches!(level, "normal" | "hard" | "master")
}

/// True when playing `mv` would recreate an earlier position while the game
/// is still inside the 20-move anti-repetition window.
pub fn leads_to_repetition(pos: &Position, mv: Move, plies: usize) -> bool {
    if plies >= REPETITION_SAFE_PLIES {
        return false;
    }
    let mut probe = pos.clone();
    apply_move(&mut probe, mv);
    let after = match probe.hashes.last() {
        Some(h) => *h,
        None => return false,
    };
    pos.hashes.contains(&after)
}

/// Random legal move that does not repeat an earlier position, falling back
/// to any legal move when the side has no repetition-free choice.
pub fn random_safe_legal(pos: &Position, plies: usize) -> Option<Move> {
    let legal = all_legal(pos);
    if legal.is_empty() {
        return None;
    }
    let safe: Vec<Move> = legal
        .iter()
        .copied()
        .filter(|mv| !leads_to_repetition(pos, *mv, plies))
        .collect();
    let pool = if safe.is_empty() { legal } else { safe };
    Some(pool[rand::random::<u64>() as usize % pool.len()])
}

/// Pick a book move for `mover`, or `None` when the level is too low, the
/// three-move book allowance is used up, or the position is out of book.
pub fn book_move(pos: &Position, level: &str, plies: usize, mover: Color) -> Option<Move> {
    if !book_supports_level(level) || moves_played(mover, plies) >= BOOK_MOVES {
        return None;
    }
    let candidates = book().get(&position_key(pos))?;
    let safe: Vec<Move> = candidates
        .iter()
        .filter_map(|uci| move_from_uci(pos, uci))
        .filter(|mv| !leads_to_repetition(pos, *mv, plies))
        .collect();
    if safe.is_empty() {
        return None;
    }
    Some(safe[rand::random::<u64>() as usize % safe.len()])
}

/// True when `mv` appears in the opening book for `pos` (the exact position
/// the move was played from), regardless of bot level or move allowance.
pub fn is_book_move(pos: &Position, mv: Move) -> bool {
    let key = position_key(pos);
    let Some(candidates) = book().get(&key) else {
        return false;
    };
    let uci = move_to_uci(mv);
    candidates.iter().any(|c| *c == uci)
}

/// UCI string for a move (promotion included).
pub fn move_to_uci(mv: Move) -> String {
    let mut out = format!("{}{}", sq_name(mv.from), sq_name(mv.to));
    if let Some(kind) = mv.promo {
        out.push_str(&kind.code().to_ascii_lowercase());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::commands::builtin::serve::chess::engine::{legal_moves_for, parse_sq};

    fn play(pos: &mut Position, uci: &str) {
        let mv = move_from_uci(pos, uci).unwrap_or_else(|| panic!("illegal book move {uci}"));
        apply_move(pos, mv);
    }

    #[test]
    fn every_book_line_is_legal() {
        for (name, line) in LINES {
            let mut pos = initial_position();
            for uci in *line {
                assert!(
                    move_from_uci(&pos, uci).is_some(),
                    "{name}: illegal move {uci}"
                );
                play(&mut pos, uci);
            }
        }
    }

    #[test]
    fn starting_position_has_several_book_replies() {
        let moves = book().get(&position_key(&initial_position())).unwrap();
        assert!(moves.len() >= 4, "book start needs variety, got {moves:?}");
        assert!(moves.contains(&"e2e4"));
        assert!(moves.contains(&"d2d4"));
        assert!(moves.contains(&"c2c4"));
    }

    #[test]
    fn book_is_only_for_normal_and_above() {
        let pos = initial_position();
        assert!(book_move(&pos, "easy", 0, Color::White).is_none());
        for level in ["normal", "hard", "master"] {
            assert!(
                book_move(&pos, level, 0, Color::White).is_some(),
                "level {level} should use the book"
            );
        }
    }

    #[test]
    fn book_stops_after_three_own_moves() {
        // 1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 — at 6 plies each side has played
        // three moves and must hand over to the engine.
        let mut pos = initial_position();
        for uci in ["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5"] {
            play(&mut pos, uci);
        }
        assert!(book_move(&pos, "normal", 6, Color::White).is_none());
        assert!(book_move(&pos, "normal", 6, Color::Black).is_none());
    }

    #[test]
    fn is_book_move_matches_the_book() {
        let mut pos = initial_position();
        let e4 = move_from_uci(&pos, "e2e4").unwrap();
        assert!(is_book_move(&pos, e4));
        // A legal but unbooked first move.
        let a3 = move_from_uci(&pos, "a2a3").unwrap();
        assert!(!is_book_move(&pos, a3));
        // Still book for Black's reply, and out of book once the line ends.
        play(&mut pos, "e2e4");
        let e5 = move_from_uci(&pos, "e7e5").unwrap();
        assert!(is_book_move(&pos, e5));
    }

    #[test]
    fn book_replies_to_both_colours() {
        // After 1. e4 the book knows Black's major replies.
        let mut pos = initial_position();
        play(&mut pos, "e2e4");
        let replies = book().get(&position_key(&pos)).unwrap();
        for expected in ["c7c5", "e7e5", "e7e6", "c7c6", "d7d5", "g8f6"] {
            assert!(replies.contains(&expected), "missing reply {expected}");
        }
    }

    #[test]
    fn repetition_is_detected_within_twenty_moves() {
        // 1. Nf3 Nf6 2. Ng1 — playing ...Ng8 recreates the initial position.
        let mut pos = initial_position();
        for uci in ["g1f3", "g8f6", "f3g1"] {
            play(&mut pos, uci);
        }
        let repeat = move_from_uci(&pos, "f6g8").unwrap();
        assert!(leads_to_repetition(&pos, repeat, 3));
        // After the window it is allowed again.
        assert!(!leads_to_repetition(&pos, repeat, REPETITION_SAFE_PLIES));
    }

    #[test]
    fn safe_move_choice_avoids_the_repeat() {
        let mut pos = initial_position();
        for uci in ["g1f3", "g8f6", "f3g1"] {
            play(&mut pos, uci);
        }
        // From the same position a random safe move must never be the
        // repeating knight retreat.
        let retreat = move_from_uci(&pos, "f6g8").unwrap();
        for _ in 0..50 {
            let chosen = random_safe_legal(&pos, 3).unwrap();
            assert_ne!(chosen, retreat);
        }
    }

    #[test]
    fn book_move_is_legal_and_repetition_free() {
        let mut pos = initial_position();
        for uci in ["e2e4", "e7e5", "g1f3", "b8c6"] {
            play(&mut pos, uci);
        }
        for _ in 0..20 {
            let mv = book_move(&pos, "hard", 4, Color::White).unwrap();
            assert!(legal_moves_for(&pos, mv.from).contains(&mv));
            assert!(!leads_to_repetition(&pos, mv, 4));
        }
    }

    #[test]
    fn move_uci_roundtrip() {
        let mv = Move {
            from: parse_sq("e7").unwrap(),
            to: parse_sq("e8").unwrap(),
            promo: Some(crate::shell::commands::builtin::serve::chess::engine::Kind::Queen),
        };
        assert_eq!(move_to_uci(mv), "e7e8q");
    }
}
