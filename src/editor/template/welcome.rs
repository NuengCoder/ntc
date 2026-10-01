/// Version stamp embedded as the last line of generated files.
/// Bump when the content changes so stale copies can be refreshed.
pub const WELCOME_VERSION: u32 = 3;
const WELCOME_STAMP: &str = "<!-- ntc-welcome v3 -->";

pub fn generate_welcome_content() -> String {
    format!(
        r#"
╔═══════════════════════════════════════════════════════════════╗
║                    Welcome to ntcEditor                       ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝

A fast, terminal-based code editor built in Rust.
Press ? in Normal mode for the full help screen.

Modes (Normal is home — Esc always walks back to it):
  Normal   single-letter verbs, nothing is typed
  Insert   actually edit text (i from Normal, Esc/Ctrl+Q back)
  Visual   select with hjkl/arrows (v from Normal, Esc back)

Normal mode:
  i / v / q      Insert / Visual / Query panel
  n / e / b      Save / file explorer / file finder
  f / t / u      Search / terminal panel / encoding picker
  g / Ctrl+J     g-mode menu / jump to line
  : then cmd     Command (:w :q :q! :wq :e! :ne :cd :run)
  ? / 0          Help screen / quit
  h j k l        move (also a s w d, arrows)
  Shift+dir      extend selection
  Home/End       line start/end      PgUp/PgDn  page scroll

Insert mode:
  Esc / Ctrl+Q   back to Normal (Esc first dismisses popups)
  Ctrl+S         save              Ctrl+Z / Ctrl+R  undo / redo
  Ctrl+A / C / X select all / copy / cut     Ctrl+P  paste
  Ctrl+D         multi-cursor at next occurrence (Ctrl+Alt+D split)
  Ctrl+K / L / M kill line / select line / split line
  Ctrl+Y         yank line         Tab / Shift+Tab  indent / dedent
  Ctrl+F / N / T search / file finder / terminal
  Ctrl+J         jump to line      Ctrl+E  command

MQL completion (.mq4/.mq5/.mqh):
  Just type — Up/Down cycle, Enter accepts, Esc dismisses.
  Accepting name( opens signature help; the active
  parameter is highlighted, PgUp/PgDn page the list.
  member completion after obj. — private members excluded.

Commands (press : then type):
  :w             Save
  :q             Quit (prompts on unsaved edits)
  :q!            Quit without saving
  :wq            Save and quit
  :e!            Reload from disk, discarding edits
  :ne <file>     Open/create file
  :cd <path>     Change directory
  :run <cmd>     Execute command in persistent panel

Query mode (q in Normal, Ctrl+Alt+Q in Insert, :x in Command):
  Leading / is optional — `s "hi"` works like `/s "hi"`.
  Esc exits (typing q is safe); Enter runs; ; chains commands.
  /s "text"      select next occurrence (wraps, repeat to walk)
  (bare /s)      select word under cursor
  /l N           go to line N (cursor to line start)
  /sa "text"     select all occurrences in buffer
  /sal           select all lines
  /sl N          select line N (/sl A B range, /sl A , B , … discrete)
  /rp <f> <r>    replace in selection (quote multi-word: /rp "a b" "c")
  /add <t> to #end|#start|#hm   append/prepend on selected lines
  /set <p> to #end|#start|#hm   align first occurrence on selected lines
  /c | /cs       clear selection
  /upper /lower  case of selection (selection required)
  /qal add|rm|info|edit|rnm|list    query aliases
  <name> [args]  run alias directly (no / needed)
  ${{1}} ${{N:-def}} ${{N[]}}  alias params (positional / default / rest)

External edits (vscode, vim, MetaEditor, …) reload
automatically while the buffer is clean; with unsaved
edits you get a warning instead — :e! loads the disk copy.

{stamp}
"#,
        stamp = WELCOME_STAMP
    )
}

/// True when `content` is an outdated generated welcome file:
/// a previous stamped version, or the legacy unstamped text
/// (recognized by its old keybinding lines). User-edited files
/// never match and are left alone.
pub fn is_stale_welcome(content: &str) -> bool {
    if content.contains("ntc-welcome v") {
        return !content.contains(WELCOME_STAMP);
    }
    content.contains("Welcome to ntcEditor") && content.contains("Ctrl + O")
}

/// Overwrite `path` with fresh welcome content when it is missing
/// or stale. Returns true when the file was written.
pub fn refresh_welcome_file(path: &std::path::Path) -> bool {
    if !path.exists() {
        return std::fs::write(path, generate_welcome_content()).is_ok();
    }
    match std::fs::read_to_string(path) {
        Ok(content) if is_stale_welcome(&content) => {
            std::fs::write(path, generate_welcome_content()).is_ok()
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_content_is_current() {
        let content = generate_welcome_content();
        assert!(content.contains(WELCOME_STAMP));
        assert!(!is_stale_welcome(&content));
        // Must document the modes/keys that actually exist.
        for must in [
            "Esc",
            "Ctrl+Q",
            ":e!",
            "Up/Down",
            "signature help",
            ".mq4",
            "/sa",
            "/sl",
            "/rp",
            "/c | /cs",
            "qal",
        ] {
            assert!(content.contains(must), "missing {}", must);
        }
        // Must not document the removed legacy keymap.
        assert!(!content.contains("Ctrl + O"));
        assert!(!content.contains("Jump mode"));
    }

    #[test]
    fn stale_detection() {
        assert!(is_stale_welcome("<!-- ntc-welcome v1 -->\nold"));
        assert!(is_stale_welcome(
            "Welcome to ntcEditor\nCtrl + O  ->  Start Editing"
        ));
        assert!(!is_stale_welcome("my own notes\nWelcome to ntcEditor"));
        assert!(!is_stale_welcome("anything without markers"));
    }
}
