pub fn generate_welcome_content() -> String {
    r#"
╔═══════════════════════════════════════════════════════════════╗
║                    Welcome to ntcEditor                       ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝

A fast, terminal-based code editor built in Rust.

Getting Started:
  * Ctrl + O  ->  Start Editing (INSERT mode)
  * Ctrl + W  ->  Start Viewing (VISUAL mode)
  * Ctrl + H  ->  Open Help
  * Ctrl + G  ->  Jump to line (Jump mode)
  * Ctrl + E  ->  Command mode (alt)
  * :         ->  Command mode

Navigation:
  * Arrow keys  ->  Move cursor (primary only when multi-cursor)
  * Shift+Arrow ->  Word-level jump / selection extend
  * Home / End  ->  Start / End of line
  * PgUp/PgDn   ->  Page scroll
  * Ctrl+PgDn   ->  Next tab
  * Ctrl+PgUp   ->  Previous tab
  * Ctrl+G      ->  Jump to line number
  * Mouse wheel ->  Scroll up/down

Editing:
  * Ctrl + S  ->  Save (auto-save is on by default)
  * Ctrl + F  ->  Find / Search in file
  * Ctrl + D  ->  Add cursor at next occurrence
  * Ctrl+Alt+D -> Split primary cursor
  * Ctrl + Z  ->  Undo
  * Ctrl + R  ->  Redo
  * Ctrl + Y  ->  Yank (copy) current line
  * Ctrl + P  ->  File finder
  * Ctrl + T  ->  Toggle terminal panel

File Operations:
  * Ctrl + B  ->  Toggle sidebar
  * Ctrl + N  ->  Open file in new tab
  * Ctrl + Q  ->  Close tab / Quit
  * Ctrl+Shift+Q -> Close all tabs

Commands (press : then type):
  * :w           ->  Save
  * :q           ->  Quit
  * :wq          ->  Save and quit
  * :ne <file>   ->  Open/create file
  * :cd <path>   ->  Change directory
  * :run <cmd>   ->  Execute command in persistent panel

For full help, press Ctrl + H

──────────────────────────────────────────────────────────────────
Insert: Ctrl+O    |    Visual: Ctrl+W    |    Jump: Ctrl+G    |    Help: Ctrl+H
"#
    .to_string()
}
