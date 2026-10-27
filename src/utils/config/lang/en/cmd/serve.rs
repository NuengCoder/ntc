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
}
