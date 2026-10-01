//! `nssh` — native SSH profiles, remote exec/shell, SFTP copy and port scan.
//!
//! ```text
//! ntc [project]> nssh add prod --host 203.0.113.10 --user root
//! ntc [project]> nssh ssh prod --cmd "uptime"
//! ntc [project]> nssh cp ./app.tar.gz prod:/tmp/
//! ntc [project]> nssh scan example.com --top 20
//! ```
//!
//! Transport is pure-Rust `russh` (see [`ssh`]); profiles live in
//! [`store`]; port scanning in [`scan`] needs no SSH at all.

pub mod keys;
pub mod scan;
pub mod ssh;
pub mod store;

pub use keys::{translate_key_bytes, translate_mouse, translate_paste, KeyTranslator, TranslateOut};

pub use scan::{parse_ports, resolve_host, scan_ports, service_name, ScanHit, TOP_PORTS};
pub use ssh::{connect, exec, shell, sftp, Connected, NsshHandler, ShellEnd, SshAuth};
pub use store::{
    delete_profile, expand_home, get_profile, list_profiles, save_profile, store_path,
    valid_profile_name, Profile,
};
