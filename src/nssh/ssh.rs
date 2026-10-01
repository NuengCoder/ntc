//! Native SSH transport for `nssh`, built on pure-Rust `russh`
//! (no C/OpenSSL toolchain needed — same rationale as the `rustls` choice
//! for SMTP). Covers connect + host-key verification, key/password auth,
//! one-shot exec, interactive PTY shell and SFTP file transfer.

use anyhow::{anyhow, Context, Result};
use russh::client::{self, Handler};
use russh::keys::known_hosts::{
    check_known_hosts, known_host_keys, learn_known_hosts,
};
use russh::keys::{load_secret_key, PrivateKeyWithHashAlg, PublicKey, PublicKeyOrCertificate};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

/// How we prove who we are: an explicit key file and/or a password.
#[derive(Debug, Default)]
pub struct SshAuth {
    pub key_path: Option<PathBuf>,
    pub password: Option<String>,
}

/// An established, authenticated session plus its identity for messages.
pub struct Connected {
    pub handle: client::Handle<NsshHandler>,
    pub host: String,
    pub user: String,
}

/// Host-key policy for one connection.
#[derive(Debug, Clone)]
pub struct NsshHandler {
    host: String,
    port: u16,
    trust_new: bool,
}

impl Handler for NsshHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        presented: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let server_key = presented.public_key();
        // Changed key = hard fail, always (possible MITM; never auto-learn).
        if let Ok(known) = known_host_keys(&self.host, self.port) {
            if !known.is_empty() && !known.iter().any(|(_, k)| k == &server_key) {
                return Err(anyhow!(
                    "HOST KEY CHANGED for {}:{} — possible man-in-the-middle. \
                     Remove the old entry from ~/.ssh/known_hosts if you replaced the server key.",
                    self.host,
                    self.port
                ));
            }
        }
        if check_known_hosts(&self.host, self.port, &server_key).unwrap_or(false) {
            return Ok(true);
        }
        let fp = fingerprint_of(&server_key);
        if self.trust_new {
            learn_known_hosts(&self.host, self.port, &server_key)
                .map_err(|e| anyhow!("cannot record host key: {}", e))?;
            return Ok(true);
        }
        Err(anyhow!(
            "unknown host key {} for {}:{} ({}). Rerun with --trust to accept and record it.",
            fp,
            self.host,
            self.port,
            key_alg_name(&server_key)
        ))
    }
}

fn fingerprint_of(key: &PublicKey) -> String {
    use russh::keys::HashAlg;
    key.fingerprint(HashAlg::Sha256).to_string()
}

fn key_alg_name(key: &PublicKey) -> String {
    format!("{:?}", key.algorithm())
}

/// Connect + authenticate. Tries explicit key, default keys, then password.
pub async fn connect(
    host: &str,
    port: u16,
    user: &str,
    auth: &SshAuth,
    trust_new: bool,
    timeout: Duration,
) -> Result<Connected> {
    let config = Arc::new(client::Config::default());
    let handler = NsshHandler {
        host: host.to_string(),
        port,
        trust_new,
    };
    let addr = (host.to_string(), port);
    let mut handle = tokio::time::timeout(timeout, client::connect(config, addr, handler))
        .await
        .map_err(|_| anyhow!("connection to {}:{} timed out", host, port))?
        .map_err(|e| anyhow!("cannot connect to {}:{}: {}", host, port, e))?;

    // 1. Explicit key file.
    if let Some(ref key_path) = auth.key_path {
        let key = load_key(key_path)?;
        if handle
            .authenticate_publickey(user, PrivateKeyWithHashAlg::new(Arc::new(key), None))
            .await
            .map_err(|e| anyhow!("key auth failed: {}", e))?
            .success()
        {
            return Ok(Connected {
                handle,
                host: host.to_string(),
                user: user.to_string(),
            });
        }
    } else {
        // 2. Default key candidates.
        for candidate in crate::nssh::store::default_key_candidates() {
            if let Ok(key) = load_secret_key(&candidate, None) {
                if handle
                    .authenticate_publickey(user, PrivateKeyWithHashAlg::new(Arc::new(key), None))
                    .await
                    .map(|r| r.success())
                    .unwrap_or(false)
                {
                    return Ok(Connected {
                        handle,
                        host: host.to_string(),
                        user: user.to_string(),
                    });
                }
            }
        }
    }

    // 3. Password.
    if let Some(ref password) = auth.password {
        if handle
            .authenticate_password(user, password)
            .await
            .map_err(|e| anyhow!("password auth failed: {}", e))?
            .success()
        {
            return Ok(Connected {
                handle,
                host: host.to_string(),
                user: user.to_string(),
            });
        }
        return Err(anyhow!("authentication failed for {}@{}", user, host));
    }

    Err(anyhow!(
        "authentication failed for {}@{} (tried keys, no password given — use --ask-pass or set pass_env)",
        user,
        host
    ))
}

fn load_key(path: &Path) -> Result<russh::keys::PrivateKey> {
    load_secret_key(path, None)
        .with_context(|| format!("cannot load key '{}'", path.display()))
}

/// Run one remote command; streams stdout/stderr; returns the exit status.
pub async fn exec(conn: &Connected, command: &str) -> Result<u32> {
    let mut channel = conn
        .handle
        .channel_open_session()
        .await
        .map_err(|e| anyhow!("cannot open channel: {}", e))?;
    channel
        .exec(true, command.as_bytes())
        .await
        .map_err(|e| anyhow!("exec failed: {}", e))?;
    let stdout = std::io::stdout();
    let mut code = 0u32;
    loop {
        match channel.wait().await {
            Some(russh::ChannelMsg::Data { data }) => {
                use std::io::Write as _;
                let _ = stdout.lock().write_all(&data);
                let _ = stdout.lock().flush();
            }
            Some(russh::ChannelMsg::ExtendedData { data, .. }) => {
                use std::io::Write as _;
                let _ = std::io::stderr().write_all(&data);
            }
            Some(russh::ChannelMsg::ExitStatus { exit_status }) => code = exit_status,
            Some(russh::ChannelMsg::Eof) | None => break,
            _ => {}
        }
    }
    let _ = channel.eof().await;
    Ok(code)
}

/// How an interactive shell ended.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShellEnd {
    /// Remote side closed (or our input stream ended).
    Closed,
    /// User typed the `~.` escape.
    Forced,
}

/// TERM advertised to the server: local value when sane, else xterm.
fn session_term() -> String {
    match std::env::var("TERM") {
        Ok(v) => {
            let t = v.trim();
            if !t.is_empty()
                && t.len() <= 32
                && t.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
            {
                return t.to_string();
            }
            "xterm-256color".to_string()
        }
        Err(_) => "xterm-256color".to_string(),
    }
}

/// Standard OpenSSH-client PTY modes (canonical input, echo, signals,
/// `erase = DEL` to match [`crate::nssh::keys::BACKSPACE_BYTE`]).
fn pty_modes() -> Vec<(russh::Pty, u32)> {
    use russh::Pty::*;
    vec![
        (VINTR, 3),
        (VQUIT, 28),
        (VERASE, 127),
        (VKILL, 21),
        (VEOF, 4),
        (VSTART, 17),
        (VSTOP, 19),
        (VSUSP, 26),
        (VREPRINT, 18),
        (VWERASE, 23),
        (VLNEXT, 22),
        (VFLUSH, 255),
        (VSWTCH, 255),
        (VSTATUS, 255),
        (VDISCARD, 255),
        (IGNPAR, 0),
        (PARMRK, 0),
        (INPCK, 0),
        (ISTRIP, 0),
        (INLCR, 0),
        (IGNCR, 0),
        (ICRNL, 1),
        (IXON, 1),
        (IXANY, 0),
        (IXOFF, 0),
        (IMAXBEL, 0),
        (ISIG, 1),
        (ICANON, 1),
        (ECHO, 1),
        (ECHOE, 1),
        (ECHOK, 1),
        (ECHONL, 0),
        (NOFLSH, 0),
        (TOSTOP, 0),
        (IEXTEN, 1),
        (ECHOCTL, 1),
        (ECHOKE, 1),
        (PENDIN, 0),
    ]
}

/// Input-side message from the console reader thread.
enum InputMsg {
    Data(Vec<u8>),
    ForceQuit,
}

/// Interactive PTY shell.
///
/// Local input is read as structured crossterm events and translated to
/// canonical xterm bytes (see [`crate::nssh::keys`]) — never raw console
/// bytes, so arrows/modifiers/Ctrl+Z behave identically on every Windows
/// terminal. `~.` (tilde first on a fresh line, then dot) force-closes.
pub async fn shell(conn: &Connected) -> Result<ShellEnd> {
    use crate::nssh::keys::{translate_mouse, translate_paste, KeyTranslator, TranslateOut};
    use crossterm::event::{Event, KeyEventKind};

    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let mut channel = conn
        .handle
        .channel_open_session()
        .await
        .map_err(|e| anyhow!("cannot open channel: {}", e))?;
    channel
        .request_pty(
            true,
            &session_term(),
            cols as u32,
            rows as u32,
            0,
            0,
            &pty_modes(),
        )
        .await
        .map_err(|e| anyhow!("pty request failed: {}", e))?;
    channel
        .request_shell(true)
        .await
        .map_err(|e| anyhow!("shell request failed: {}", e))?;

    struct RawGuard;
    impl Drop for RawGuard {
        fn drop(&mut self) {
            let _ = crossterm::terminal::disable_raw_mode();
        }
    }
    crossterm::terminal::enable_raw_mode()?;
    let _guard = RawGuard;

    // NOTE: writer first — `make_writer` is 'static (no borrow held),
    // while `make_reader` borrows the channel mutably.
    let mut writer = channel.make_writer();
    let mut reader = channel.make_reader();
    let mut stdout = tokio::io::stdout();

    // Detached console thread: structured events → xterm bytes. Exits on
    // read error or once the receiver is gone (session end). A thread
    // blocked in `event::read` with no further keys simply ends with the
    // process; exactly one such thread exists per session.
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<InputMsg>();
    std::thread::spawn(move || {
        let mut tr = KeyTranslator::new();
        loop {
            match crossterm::event::read() {
                Ok(Event::Key(k)) => {
                    if k.kind == KeyEventKind::Release {
                        continue;
                    }
                    match tr.translate_key(&k) {
                        TranslateOut::Bytes(b) => {
                            if tx.send(InputMsg::Data(b)).is_err() {
                                break;
                            }
                        }
                        TranslateOut::ForceQuit => {
                            let _ = tx.send(InputMsg::ForceQuit);
                            break;
                        }
                        TranslateOut::Ignore => {}
                    }
                }
                Ok(Event::Mouse(m)) => {
                    if let Some(b) = translate_mouse(&m) {
                        if tx.send(InputMsg::Data(b)).is_err() {
                            break;
                        }
                    }
                }
                Ok(Event::Paste(s)) => {
                    if tx.send(InputMsg::Data(translate_paste(&s))).is_err() {
                        break;
                    }
                }
                Ok(_) => {}
                Err(_) => break,
            }
        }
    });

    // The copy future borrows the halves, which borrow the channel: scope
    // it so everything is released before the polite close below.
    let end = {
        let copy_out = tokio::io::copy(&mut reader, &mut stdout);
        tokio::pin!(copy_out);
        loop {
            tokio::select! {
                r = &mut copy_out => {
                    r.map_err(|e| anyhow!("shell I/O failed: {}", e))?;
                    break ShellEnd::Closed;
                }
                m = rx.recv() => match m {
                    Some(InputMsg::Data(b)) => {
                        use tokio::io::AsyncWriteExt as _;
                        writer
                            .write_all(&b)
                            .await
                            .map_err(|e| anyhow!("shell I/O failed: {}", e))?;
                    }
                    Some(InputMsg::ForceQuit) => break ShellEnd::Forced,
                    None => break ShellEnd::Closed,
                },
            }
        }
    };
    drop(reader);
    drop(writer);
    if end == ShellEnd::Forced {
        let _ = channel.close().await;
    } else {
        let _ = channel.eof().await;
    }
    Ok(end)
}

/// Open an SFTP subsystem session over a fresh channel.
pub async fn sftp(conn: &Connected) -> Result<russh_sftp::client::SftpSession> {
    let channel = conn
        .handle
        .channel_open_session()
        .await
        .map_err(|e| anyhow!("cannot open channel: {}", e))?;
    channel
        .request_subsystem(true, "sftp")
        .await
        .map_err(|e| anyhow!("sftp subsystem refused: {}", e))?;
    russh_sftp::client::SftpSession::new(channel.into_stream())
        .await
        .map_err(|e| anyhow!("sftp init failed: {}", e))
}
