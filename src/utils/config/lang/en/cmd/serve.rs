use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.serve.db_title".into(), "Database:".into());

    m.insert(
        "cmd.serve.example_default".into(),
        "  serve files            HTTP on port 8080".into(),
    );

    m.insert(
        "cmd.serve.example_port".into(),
        "  serve files 3000       HTTP on port 3000".into(),
    );

    m.insert(
        "cmd.serve.example_spa".into(),
        "  serve files --spa      SPA fallback for React Router".into(),
    );

    m.insert(
        "cmd.serve.example_tunnel".into(),
        "  serve files --tunnel   HTTP + cloudflared tunnel".into(),
    );

    m.insert("cmd.serve.examples_title".into(), "Examples:".into());

    m.insert(
        "cmd.serve.files_stored_title".into(),
        "Files stored in:".into(),
    );

    m.insert(
        "cmd.serve.help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: 0.0.0.0)".into(),
    );

    m.insert(
        "cmd.serve.help_help".into(),
        "  --help, -h            Show this help".into(),
    );

    m.insert(
        "cmd.serve.help_port".into(),
        "  --port, -p <port>     Port to listen on (default: 8080)".into(),
    );

    m.insert(
        "cmd.serve.help_spa".into(),
        "  --spa                 SPA mode (fallback to index.html)".into(),
    );

    m.insert("cmd.serve.help_title".into(), "HTTP File Server".into());

    m.insert(
        "cmd.serve.help_tunnel".into(),
        "  --tunnel, -t          Expose via cloudflared tunnel".into(),
    );

    m.insert(
        "cmd.serve.help_share".into(),
        "  --share [path]        Check internet, show LAN URL + QR for phones".into(),
    );

    m.insert(
        "cmd.serve.help_qr".into(),
        "  --qr                  Force QR display (implies --share)".into(),
    );

    m.insert(
        "cmd.serve.help_no_qr".into(),
        "  --no-qr               Suppress QR display".into(),
    );

    m.insert(
        "cmd.serve.example_share".into(),
        "  serve files 8080 --share  Check internet, print LAN URL + QR".into(),
    );

    m.insert(
        "cmd.serve.share_checking".into(),
        "Checking internet connection...".into(),
    );

    m.insert(
        "cmd.serve.share_online".into(),
        "Internet OK — share mode".into(),
    );

    m.insert(
        "cmd.serve.share_offline".into(),
        "No internet connection — showing localhost only.".into(),
    );

    m.insert("cmd.serve.share_lan_title".into(), "LAN:".into());

    m.insert("cmd.serve.share_path_title".into(), "Share:".into());

    m.insert(
        "cmd.serve.share_no_lan".into(),
        "No LAN address found — showing localhost only.".into(),
    );

    m.insert(
        "cmd.serve.share_bound_loopback".into(),
        "Bound to loopback: LAN devices cannot reach this server (use default 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.share_not_private".into(),
        "Detected address {} is not a private LAN range.".into(),
    );

    m.insert(
        "cmd.serve.share_not_private_hint".into(),
        "Expected 10.x.x.x, 172.16-31.x.x or 192.168.x.x for phone sharing.".into(),
    );

    m.insert(
        "cmd.serve.share_qr_fail".into(),
        "QR render failed: {}".into(),
    );

    m.insert(
        "cmd.serve.help_usage".into(),
        "serve files [port] [options]".into(),
    );

    m.insert(
        "cmd.serve.help_web_dir".into(),
        "  --web-dir, -w <dir>   Serve custom frontend directory".into(),
    );

    m.insert(
        "cmd.serve.help_auth".into(),
        "  --auth <user:pass>    Require login for /api/* (e.g. --auth admin:secret)".into(),
    );

    m.insert(
        "cmd.serve.help_lock".into(),
        "  --lock                Read-only: allow download/list, block upload/edit/create/delete"
            .into(),
    );

    m.insert(
        "cmd.serve.lock_on".into(),
        "Read-only mode (--lock): uploads, edits, creates and deletes are blocked".into(),
    );
    m.insert(
        "cmd.serve.url_web".into(),
        "web UI:        {}  (browse + drag-and-drop upload)".into(),
    );

    m.insert(
        "cmd.serve.url_upload".into(),
        "upload:        {}  (POST multipart, field 'file')".into(),
    );

    m.insert(
        "cmd.serve.url_download".into(),
        "download:      {}  (append the file path)".into(),
    );

    m.insert(
        "cmd.serve.url_info".into(),
        "info:          {}  (what this is + curl/wget/ntc recipes)".into(),
    );

    m.insert(
        "cmd.serve.example_auth".into(),
        "  serve files 8080 --auth admin:secret   HTTP on 8080, API needs login".into(),
    );

    m.insert(
        "cmd.serve.invalid_auth".into(),
        "Invalid --auth (use --auth <user:pass>)".into(),
    );

    m.insert(
        "cmd.serve.missing_auth".into(),
        "Missing value for --auth".into(),
    );

    m.insert(
        "cmd.serve.auth_on".into(),
        "API auth enabled for user '{}'".into(),
    );

    m.insert("cmd.serve.invalid_port".into(), "Invalid port".into());

    m.insert("cmd.serve.missing_bind".into(), "Missing bind".into());

    m.insert("cmd.serve.missing_dir".into(), "Missing dir".into());

    m.insert("cmd.serve.missing_port".into(), "Missing port".into());

    m.insert("cmd.serve.options_title".into(), "Options:".into());

    m.insert("cmd.serve.stopped".into(), "Server stopped.".into());

    m.insert("cmd.serve.unknown_option".into(), "Unknown: {}".into());

    m.insert("cmd.serve.usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.serve.cloudflared_starting".into(),
        "Cloudflared tunnel starting (PID {})".into(),
    );

    m.insert(
        "cmd.serve.cloudflared_missing".into(),
        "cloudflared not found: {}. Install: {}".into(),
    );

    m.insert(
        "cmd.serve.chess_soon".into(),
        "`serve chess` is coming soon (online multiplayer)".into(),
    );

    m.insert(
        "cmd.serve.services_title".into(),
        "Serve — pick a service".into(),
    );

    m.insert(
        "cmd.serve.services_usage".into(),
        "serve <service> [options]  (bare `serve` runs nothing)".into(),
    );

    m.insert("cmd.serve.services_header".into(), "Services:".into());

    m.insert(
        "cmd.serve.services_row_files".into(),
        "  files         File-drive web UI (upload/download in browser)".into(),
    );

    m.insert(
        "cmd.serve.services_row_http".into(),
        "  http-server   Static server for the current directory".into(),
    );

    m.insert(
        "cmd.serve.services_row_chess".into(),
        "  chess         2-player hotseat chess in the browser".into(),
    );

    m.insert(
        "cmd.serve.services_row_git".into(),
        "  git           Git GUI: status, diff, stage, commit, branches, tags, stash, push/pull"
            .into(),
    );

    m.insert(
        "cmd.serve.services_row_arc".into(),
        "  arc           Archive GUI: create, browse/extract archives".into(),
    );

    m.insert(
        "cmd.serve.services_row_theme".into(),
        "  theme         Theme GUI: edit, preview, save and apply themes".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        Show/set per-service defaults (port, bind)".into(),
    );

    m.insert(
        "cmd.serve.services_example_files".into(),
        "  serve files --port 9000".into(),
    );

    m.insert(
        "cmd.serve.services_example_http".into(),
        "  serve http-server ./dist".into(),
    );

    m.insert(
        "cmd.serve.services_example_config".into(),
        "  serve config files --port-default 9000".into(),
    );

    m.insert(
        "cmd.serve.http_title".into(),
        "HTTP Static Server (CWD)".into(),
    );

    m.insert(
        "cmd.serve.http_usage".into(),
        "serve http-server [dir] [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_http_default".into(),
        "  serve http-server            Serve CWD on port 8080".into(),
    );

    m.insert(
        "cmd.serve.example_http_dir".into(),
        "  serve http-server ./dist --port 8001   Serve ./dist on 8001".into(),
    );

    m.insert(
        "cmd.serve.example_http_auth".into(),
        "  serve http-server --auth admin:secret  Require login".into(),
    );

    m.insert(
        "cmd.serve.help_auth_http".into(),
        "  --auth <user:pass>    Require login for all routes (e.g. --auth admin:secret)".into(),
    );

    m.insert("cmd.serve.http_serving".into(), "Serving".into());

    m.insert(
        "cmd.serve.http_no_dir".into(),
        "Directory '{}' not found".into(),
    );

    m.insert(
        "cmd.serve.current_defaults".into(),
        "Configured defaults (serve config): port {}, bind {}".into(),
    );

    m.insert(
        "cmd.serve.chess_no_auth".into(),
        "`serve chess` takes no --auth (hotseat has no login)".into(),
    );

    m.insert("cmd.serve.chess_serving".into(), "Chess server on".into());

    m.insert(
        "cmd.serve.chess_title".into(),
        "Chess (2-player hotseat)".into(),
    );

    m.insert(
        "cmd.serve.chess_usage".into(),
        "serve chess [--port 8000] [options]".into(),
    );

    m.insert(
        "cmd.serve.example_chess_default".into(),
        "  serve chess              Chess on port 8080".into(),
    );

    m.insert(
        "cmd.serve.example_chess_port".into(),
        "  serve chess --port 9000  Chess on port 9000".into(),
    );

    m.insert(
        "cmd.serve.help_online".into(),
        "  --online              Enable online lobby + rooms (guests, quick match, coach rooms)"
            .into(),
    );

    m.insert(
        "cmd.serve.example_chess_online".into(),
        "  serve chess --online -b 0.0.0.0 --share  Online chess over LAN with QR".into(),
    );

    m.insert(
        "cmd.serve.git_serving".into(),
        "Git server on {} ({})".into(),
    );

    m.insert("cmd.serve.git_title".into(), "Git (browser UI)".into());

    m.insert(
        "cmd.serve.git_usage".into(),
        "serve git [dir] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.git_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.git_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.git_example".into(),
        "  serve git --port 9000  Serve the current repository on 9000".into(),
    );

    m.insert(
        "cmd.serve.git_no_repo".into(),
        "Not a git repository: {}".into(),
    );

    m.insert(
        "cmd.serve.git_no_git".into(),
        "git is not installed or not on PATH".into(),
    );

    m.insert(
        "cmd.serve.git_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.arc_serving".into(),
        "Archive server on {} ({})".into(),
    );

    m.insert("cmd.serve.arc_title".into(), "Archive (browser UI)".into());

    m.insert(
        "cmd.serve.arc_usage".into(),
        "serve arc [dir] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.arc_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.arc_example".into(),
        "  serve arc --port 9000  Serve the current directory on 9000".into(),
    );

    m.insert(
        "cmd.serve.arc_no_dir".into(),
        "Directory '{}' not found".into(),
    );

    m.insert(
        "cmd.serve.arc_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert("cmd.serve.theme_title".into(), "Theme (browser UI)".into());

    m.insert(
        "cmd.serve.theme_usage".into(),
        "serve theme [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.theme_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.theme_example".into(),
        "  serve theme --port 9000  Edit themes on port 9000".into(),
    );

    m.insert(
        "cmd.serve.theme_serving".into(),
        "Theme server on {} ({})".into(),
    );

    m.insert(
        "cmd.serve.theme_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.services_row_ndb".into(),
        "  ndb           SQLite browser: pick a database in the folder, browse it, optional --write".into(),
    );

    m.insert("cmd.serve.ndb_title".into(), "Database browser".into());

    m.insert(
        "cmd.serve.ndb_usage".into(),
        "serve ndb [--port 8000] [--bind addr] [--auth user:pass] [--write]".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.ndb_help_write".into(),
        "  --write               Allow INSERT/UPDATE/DELETE/REPLACE; a <name>.bak.<stamp> copy is made before the first write".into(),
    );

    m.insert(
        "cmd.serve.ndb_serving".into(),
        "Serving the databases in {} on {}".into(),
    );

    m.insert(
        "cmd.serve.ndb_write_on".into(),
        "Writes enabled: one INSERT, UPDATE, DELETE or REPLACE at a time; a <name>.bak.<stamp> backup lands next to the database before its first write.".into(),
    );

    m.insert(
        "cmd.serve.ndb_readonly".into(),
        "Read-only: SELECT, PRAGMA, EXPLAIN, WITH, VALUES and TABLE only; start with --write to allow row-level writes.".into(),
    );

    m.insert(
        "cmd.serve.ndb_local_warning".into(),
        "Loopback only: LAN devices cannot reach this server (use --bind 0.0.0.0 plus --auth to share).".into(),
    );

    m.insert(
        "cmd.serve.ndb_no_db".into(),
        "No readable SQLite database found in {} - copy a .db, .sqlite or .sqlite3 file there"
            .into(),
    );

    m.insert(
        "cmd.serve.ndb_flags_only".into(),
        "'{}' is not an option: `serve ndb` takes flags only - pick the database in the page"
            .into(),
    );

    m.insert(
        "cmd.serve.ndb_requires_auth".into(),
        "Refusing to expose the database on {} without --auth <user:pass>".into(),
    );

    m.insert(
        "cmd.serve.ndb_example".into(),
        "  serve ndb --port 9000  List this folder's databases on port 9000".into(),
    );

    m.insert(
        "cmd.serve.services_row_fake".into(),
        "  fake          Fake-data GUI: pick types or a saved group, preview rows".into(),
    );

    m.insert(
        "cmd.serve.services_row_config".into(),
        "  config        Configuration GUI: every setting and serve default in the browser".into(),
    );

    m.insert(
        "cmd.serve.services_example_fake".into(),
        "  serve fake --port 9000".into(),
    );

    m.insert(
        "cmd.serve.fake_title".into(),
        "Fake data (browser UI)".into(),
    );

    m.insert(
        "cmd.serve.fake_usage".into(),
        "serve fake [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.fake_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.fake_example".into(),
        "  serve fake --port 9000  Open the fake-data UI on port 9000".into(),
    );

    m.insert(
        "cmd.serve.fake_serving".into(),
        "Fake-data server on {}".into(),
    );

    m.insert(
        "cmd.serve.fake_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.config_title".into(),
        "NTC configuration (browser UI)".into(),
    );

    m.insert(
        "cmd.serve.config_usage".into(),
        "serve config [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.config_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.config_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.config_example".into(),
        "  serve config --port 9000  Open the configuration UI on port 9000".into(),
    );

    m.insert(
        "cmd.serve.config_serving".into(),
        "Configuration server on {}".into(),
    );

    m.insert(
        "cmd.serve.config_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.services_row_backup".into(),
        "  backup        Backup GUI: create, diff, restore, undo, clear this folder".into(),
    );

    m.insert(
        "cmd.serve.services_example_backup".into(),
        "  serve backup --port 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_title".into(),
        "Backup (browser UI)".into(),
    );

    m.insert(
        "cmd.serve.backup_usage".into(),
        "serve backup [path] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.backup_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.backup_example".into(),
        "  serve backup --port 9000  Manage this folder's backups on port 9000".into(),
    );

    m.insert(
        "cmd.serve.backup_serving".into(),
        "Backup server on {} ({})".into(),
    );

    m.insert(
        "cmd.serve.backup_local_warning".into(),
        "Bound to loopback: LAN devices cannot reach this server (use --bind 0.0.0.0).".into(),
    );

    m.insert(
        "cmd.serve.backup_no_dir".into(),
        "Directory '{}' not found".into(),
    );

    m.insert(
        "cmd.serve.services_row_mdview".into(),
        "  mdview        Markdown editor with a live preview (writes .md files)".into(),
    );

    m.insert(
        "cmd.serve.mdview_title".into(),
        "Markdown editor (browser UI)".into(),
    );

    m.insert(
        "cmd.serve.mdview_usage".into(),
        "serve mdview [dir] [--port 8000] [--bind addr]".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_port".into(),
        "  --port, -p <port>     Port to listen on (default: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_help_bind".into(),
        "  --bind, -b <addr>     Bind address (default: {})".into(),
    );

    m.insert(
        "cmd.serve.mdview_example".into(),
        "  serve mdview docs --port 9000  Edit the markdown under docs/ on port 9000".into(),
    );

    m.insert(
        "cmd.serve.mdview_serving".into(),
        "Markdown editor for {} on {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_no_dir".into(),
        "Not a folder: {}".into(),
    );

    m.insert(
        "cmd.serve.mdview_local_warning".into(),
        "Bound to loopback: only this machine can edit these files (use --bind 0.0.0.0 to share)".into(),
    );
}
