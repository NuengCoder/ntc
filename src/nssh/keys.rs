//! Local key → xterm byte translation for the `nssh` interactive shell.
//!
//! The Windows console does not reliably emit xterm sequences itself
//! (arrows/modifiers differ between conhost and Windows Terminal, and
//! Ctrl+Z is swallowed as stdin EOF before any byte can be forwarded).
//! Instead of forwarding raw console bytes and hoping, the shell pump reads
//! structured [`crossterm::event::Event`]s and this module emits canonical
//! xterm bytes — identical on every local terminal, byte-identical to what
//! a Linux client sends, so remote apps (`ne`, vim, nvim, shells) all
//! behave.
//!
//! Also implements the OpenSSH-style `~.` escape: `~` as the first
//! character after a newline, followed by `.`, force-disconnects.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

/// BACKSPACE byte: DEL matches the Linux `stty erase` default (`^?`).
pub const BACKSPACE_BYTE: u8 = 0x7F;
/// Ctrl+Backspace byte: preserved legacy behavior.
pub const CTRL_BACKSPACE_BYTE: u8 = 0x08;

/// Outcome of feeding one local event to [`KeyTranslator`].
#[derive(Debug, PartialEq)]
pub enum TranslateOut {
    /// Forward these bytes to the remote PTY.
    Bytes(Vec<u8>),
    /// `~.` escape sequence complete: force-close the session.
    ForceQuit,
    /// Nothing to forward (releases, locks, unknown keys).
    Ignore,
}

/// Escape state for the `~.` sequence (`~` must lead a fresh line).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum EscapeState {
    #[default]
    Idle,
    AfterNewline,
    AfterTilde,
}

/// Stateful translator: pure key→bytes mapping plus the `~.` tracker.
#[derive(Debug, Default)]
pub struct KeyTranslator {
    state: EscapeState,
}

impl KeyTranslator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Translate one key event (caller filters to Press/Repeat).
    pub fn translate_key(&mut self, key: &KeyEvent) -> TranslateOut {
        match translate_key_bytes(key) {
            Some(bytes) => self.push_bytes(bytes),
            None => TranslateOut::Ignore,
        }
    }

    /// Feed already-encoded bytes (mouse reports, pastes) through the
    /// escape tracker.
    pub fn push_bytes(&mut self, bytes: Vec<u8>) -> TranslateOut {
        match self.state {
            EscapeState::AfterNewline if bytes == b"~" => {
                self.state = EscapeState::AfterTilde;
                TranslateOut::Ignore // swallow; `~.` or `~~` decides next
            }
            EscapeState::AfterTilde if bytes == b"." => {
                self.state = EscapeState::Idle;
                TranslateOut::ForceQuit
            }
            EscapeState::AfterTilde if bytes == b"~" => {
                // `~~` = literal tilde; line is no longer fresh.
                self.state = EscapeState::Idle;
                TranslateOut::Bytes(b"~".to_vec())
            }
            _ => {
                self.state = if ends_with_newline(&bytes) {
                    EscapeState::AfterNewline
                } else {
                    EscapeState::Idle
                };
                TranslateOut::Bytes(bytes)
            }
        }
    }
}

fn ends_with_newline(bytes: &[u8]) -> bool {
    matches!(bytes.last(), Some(b'\r') | Some(b'\n'))
}

/// xterm modifier parameter: 1 + shift(1) + alt(2) + ctrl(4).
fn csi_modifier(m: &KeyModifiers) -> u8 {
    1 + (m.contains(KeyModifiers::SHIFT) as u8)
        + (m.contains(KeyModifiers::ALT) as u8) * 2
        + (m.contains(KeyModifiers::CONTROL) as u8) * 4
}

/// Stateless key→bytes mapping (escape tracking lives in [`KeyTranslator`]).
pub fn translate_key_bytes(key: &KeyEvent) -> Option<Vec<u8>> {
    let mods = &key.modifiers;
    let ctrl = mods.contains(KeyModifiers::CONTROL);
    let alt = mods.contains(KeyModifiers::ALT);
    match &key.code {
        KeyCode::Char(c) => {
            if ctrl && !alt {
                if *c == ' ' {
                    return Some(vec![0x00]); // Ctrl+Space = NUL
                }
                let lower = c.to_ascii_lowercase();
                if lower.is_ascii_lowercase() {
                    return Some(vec![lower as u8 - b'a' + 1]);
                }
                return None;
            }
            let mut out = Vec::new();
            if alt {
                out.push(0x1B);
                if ctrl {
                    let lower = c.to_ascii_lowercase();
                    if lower.is_ascii_lowercase() {
                        out.push(lower as u8 - b'a' + 1);
                        return Some(out);
                    }
                    return None;
                }
            }
            let mut buf = [0u8; 4];
            out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            Some(out)
        }
        KeyCode::Enter => {
            if alt {
                Some(vec![0x1B, b'\r'])
            } else {
                Some(vec![b'\r'])
            }
        }
        KeyCode::Tab => {
            if alt {
                Some(vec![0x1B, b'\t'])
            } else {
                Some(vec![b'\t'])
            }
        }
        KeyCode::BackTab => Some(b"\x1B[Z".to_vec()),
        KeyCode::Backspace => {
            if ctrl && !alt {
                Some(vec![CTRL_BACKSPACE_BYTE])
            } else if alt && !ctrl {
                Some(vec![0x1B, BACKSPACE_BYTE])
            } else if !alt {
                Some(vec![BACKSPACE_BYTE])
            } else {
                None
            }
        }
        KeyCode::Delete => Some(csi_tilde(3, mods)),
        KeyCode::Insert => Some(csi_tilde(2, mods)),
        KeyCode::Esc => Some(vec![0x1B]),
        KeyCode::Up => Some(csi_arrow(b'A', mods)),
        KeyCode::Down => Some(csi_arrow(b'B', mods)),
        KeyCode::Right => Some(csi_arrow(b'C', mods)),
        KeyCode::Left => Some(csi_arrow(b'D', mods)),
        KeyCode::Home => Some(csi_letter(b'H', mods)),
        KeyCode::End => Some(csi_letter(b'F', mods)),
        KeyCode::PageUp => Some(csi_tilde(5, mods)),
        KeyCode::PageDown => Some(csi_tilde(6, mods)),
        KeyCode::F(n) => translate_f(*n, mods),
        KeyCode::Null => Some(vec![0x00]),
        // Locks, media and bare-modifier keys carry no PTY meaning.
        KeyCode::CapsLock
        | KeyCode::ScrollLock
        | KeyCode::NumLock
        | KeyCode::PrintScreen
        | KeyCode::Pause
        | KeyCode::Menu
        | KeyCode::KeypadBegin
        | KeyCode::Media(_)
        | KeyCode::Modifier(_) => None,
    }
}

fn csi_arrow(final_byte: u8, mods: &KeyModifiers) -> Vec<u8> {
    let m = csi_modifier(mods);
    if m == 1 {
        vec![0x1B, b'[', final_byte]
    } else {
        vec![0x1B, b'[', b'1', b';', b'0' + m, final_byte]
    }
}

fn csi_letter(final_byte: u8, mods: &KeyModifiers) -> Vec<u8> {
    csi_arrow(final_byte, mods)
}

fn csi_tilde(n: u8, mods: &KeyModifiers) -> Vec<u8> {
    let m = csi_modifier(mods);
    // All current callers pass single-digit codes (2, 3, 5, 6).
    let mut out = vec![0x1B, b'[', b'0' + n];
    if m == 1 {
        out.push(b'~');
    } else {
        out.extend_from_slice(&[b';', b'0' + m, b'~']);
    }
    out
}

fn translate_f(n: u8, mods: &KeyModifiers) -> Option<Vec<u8>> {
    let m = csi_modifier(mods);
    match n {
        1..=4 => {
            let final_byte = b'P' + (n - 1);
            if m == 1 {
                Some(vec![0x1B, b'O', final_byte])
            } else {
                Some(vec![0x1B, b'[', b'1', b';', b'0' + m, final_byte])
            }
        }
        5..=12 => {
            let code = match n {
                5 => 15,
                6 => 17,
                7 => 18,
                8 => 19,
                9 => 20,
                10 => 21,
                11 => 23,
                _ => 24,
            };
            let mut out = vec![0x1B, b'[', b'0' + code / 10, b'0' + code % 10];
            if m == 1 {
                out.push(b'~');
            } else {
                out.extend_from_slice(&[b';', b'0' + m, b'~']);
            }
            Some(out)
        }
        _ => None,
    }
}

/// Translate a local mouse event to an xterm SGR (1006) sequence.
/// Coordinates are 1-based on the wire; crossterm reports 0-based.
pub fn translate_mouse(m: &MouseEvent) -> Option<Vec<u8>> {
    let (mut cb, release) = match m.kind {
        MouseEventKind::Down(b) => (button_base(b), false),
        MouseEventKind::Up(_) => (3, true),
        MouseEventKind::Drag(b) => (button_base(b) + 32, false),
        MouseEventKind::Moved => (3 + 32, false),
        MouseEventKind::ScrollUp => (64, false),
        MouseEventKind::ScrollDown => (65, false),
        MouseEventKind::ScrollLeft => (66, false),
        MouseEventKind::ScrollRight => (67, false),
    };
    if m.modifiers.contains(KeyModifiers::SHIFT) {
        cb += 4;
    }
    if m.modifiers.contains(KeyModifiers::ALT) {
        cb += 8;
    }
    if m.modifiers.contains(KeyModifiers::CONTROL) {
        cb += 16;
    }
    let suffix = if release { b'm' } else { b'M' };
    Some(
        format!(
            "\x1B[<{0};{1};{2}{3}",
            cb,
            m.column + 1,
            m.row + 1,
            suffix as char
        )
        .into_bytes(),
    )
}

fn button_base(b: MouseButton) -> u8 {
    match b {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
    }
}

/// Wrap pasted text in bracketed-paste markers.
pub fn translate_paste(text: &str) -> Vec<u8> {
    format!("\x1B[200~{}\x1B[201~", text).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    fn key_mod(code: KeyCode, mods: KeyModifiers) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: mods,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    fn tx(code: KeyCode) -> TranslateOut {
        KeyTranslator::new().translate_key(&key(code))
    }

    #[test]
    fn basics() {
        assert_eq!(tx(KeyCode::Enter), TranslateOut::Bytes(vec![b'\r']));
        assert_eq!(tx(KeyCode::Tab), TranslateOut::Bytes(vec![b'\t']));
        assert_eq!(
            tx(KeyCode::BackTab),
            TranslateOut::Bytes(b"\x1B[Z".to_vec())
        );
        // Agreed default: DEL matches Linux `stty erase ^?`.
        assert_eq!(tx(KeyCode::Backspace), TranslateOut::Bytes(vec![0x7F]));
        assert_eq!(tx(KeyCode::Esc), TranslateOut::Bytes(vec![0x1B]));
        assert_eq!(tx(KeyCode::Char('a')), TranslateOut::Bytes(b"a".to_vec()));
        // Ctrl+Z must reach remote undo as 0x1A, never console-EOF us.
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Char('z'), KeyModifiers::CONTROL)),
            TranslateOut::Bytes(vec![0x1A])
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Char('C'), KeyModifiers::CONTROL)),
            TranslateOut::Bytes(vec![0x03])
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Char('x'), KeyModifiers::ALT)),
            TranslateOut::Bytes(vec![0x1B, b'x'])
        );
    }

    #[test]
    fn navigation_keys() {
        assert_eq!(tx(KeyCode::Up), TranslateOut::Bytes(b"\x1B[A".to_vec()));
        assert_eq!(tx(KeyCode::Down), TranslateOut::Bytes(b"\x1B[B".to_vec()));
        assert_eq!(tx(KeyCode::Right), TranslateOut::Bytes(b"\x1B[C".to_vec()));
        assert_eq!(tx(KeyCode::Left), TranslateOut::Bytes(b"\x1B[D".to_vec()));
        assert_eq!(tx(KeyCode::Home), TranslateOut::Bytes(b"\x1B[H".to_vec()));
        assert_eq!(tx(KeyCode::End), TranslateOut::Bytes(b"\x1B[F".to_vec()));
        assert_eq!(
            tx(KeyCode::PageUp),
            TranslateOut::Bytes(b"\x1B[5~".to_vec())
        );
        assert_eq!(
            tx(KeyCode::PageDown),
            TranslateOut::Bytes(b"\x1B[6~".to_vec())
        );
        assert_eq!(
            tx(KeyCode::Insert),
            TranslateOut::Bytes(b"\x1B[2~".to_vec())
        );
        assert_eq!(
            tx(KeyCode::Delete),
            TranslateOut::Bytes(b"\x1B[3~".to_vec())
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Left, KeyModifiers::ALT)),
            TranslateOut::Bytes(b"\x1B[1;3D".to_vec())
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Up, KeyModifiers::CONTROL)),
            TranslateOut::Bytes(b"\x1B[1;5A".to_vec())
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(
                KeyCode::PageUp,
                KeyModifiers::SHIFT | KeyModifiers::CONTROL
            )),
            TranslateOut::Bytes(b"\x1B[5;6~".to_vec())
        );
    }

    #[test]
    fn function_keys() {
        assert_eq!(tx(KeyCode::F(1)), TranslateOut::Bytes(b"\x1BOP".to_vec()));
        assert_eq!(tx(KeyCode::F(4)), TranslateOut::Bytes(b"\x1BOS".to_vec()));
        assert_eq!(tx(KeyCode::F(5)), TranslateOut::Bytes(b"\x1B[15~".to_vec()));
        assert_eq!(
            tx(KeyCode::F(12)),
            TranslateOut::Bytes(b"\x1B[24~".to_vec())
        );
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::F(5), KeyModifiers::CONTROL)),
            TranslateOut::Bytes(b"\x1B[15;5~".to_vec())
        );
        assert_eq!(tx(KeyCode::F(99)), TranslateOut::Ignore);
    }

    #[test]
    fn ignored_keys() {
        assert_eq!(tx(KeyCode::CapsLock), TranslateOut::Ignore);
        assert_eq!(
            KeyTranslator::new().translate_key(&key_mod(KeyCode::Char('a'), KeyModifiers::empty())),
            TranslateOut::Bytes(b"a".to_vec())
        );
    }

    #[test]
    fn tilde_dot_escapes() {
        let mut tr = KeyTranslator::new();
        // `~` mid-line is literal.
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('~'))),
            TranslateOut::Bytes(b"~".to_vec())
        );
        // Fresh line: `~` arms, `.` quits.
        assert_eq!(
            tr.translate_key(&key(KeyCode::Enter)),
            TranslateOut::Bytes(vec![b'\r'])
        );
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('~'))),
            TranslateOut::Ignore
        );
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('.'))),
            TranslateOut::ForceQuit
        );
        // `~~` is a literal tilde, no quit follows.
        let mut tr = KeyTranslator::new();
        assert_eq!(
            tr.translate_key(&key(KeyCode::Enter)),
            TranslateOut::Bytes(vec![b'\r'])
        );
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('~'))),
            TranslateOut::Ignore
        );
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('~'))),
            TranslateOut::Bytes(b"~".to_vec())
        );
        assert_eq!(
            tr.translate_key(&key(KeyCode::Char('.'))),
            TranslateOut::Bytes(b".".to_vec())
        );
    }

    #[test]
    fn mouse_sgr() {
        let m = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::empty(),
        };
        assert_eq!(translate_mouse(&m).unwrap(), b"\x1B[<0;1;1M".to_vec());
        let up = MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 9,
            row: 4,
            modifiers: KeyModifiers::empty(),
        };
        assert_eq!(translate_mouse(&up).unwrap(), b"\x1B[<3;10;5m".to_vec());
        let wheel = MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::empty(),
        };
        assert_eq!(translate_mouse(&wheel).unwrap(), b"\x1B[<64;1;1M".to_vec());
    }

    #[test]
    fn paste_wrapped() {
        assert_eq!(translate_paste("hi"), b"\x1B[200~hi\x1B[201~".to_vec());
    }
}
