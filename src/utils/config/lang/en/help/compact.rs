use std::collections::HashMap;

/// One-liners for the compact `help` overview (`help --all` keeps the long
/// reference). Keep these short: a command, what it is, no argument syntax.
pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "Command overview".into());
    m.insert(
        "help.compact.footer".into(),
        "Type 'help <command>' for details, 'help --all' for the full reference.".into(),
    );

    m.insert(
        "help.compact.nav".into(),
        "Change directory, go back, jump".into(),
    );
    m.insert(
        "help.compact.config".into(),
        "Show or change ntc settings".into(),
    );
    m.insert(
        "help.compact.editor".into(),
        "Open the built-in editor".into(),
    );
    m.insert(
        "help.compact.theme".into(),
        "Switch, add, export themes".into(),
    );
    m.insert(
        "help.compact.teleport".into(),
        "Saved paths (tp jump <name>)".into(),
    );
    m.insert(
        "help.compact.ran".into(),
        "Run tasks from NTCRANFILE.toml".into(),
    );
    m.insert("help.compact.scr".into(), "Run saved script files".into());
    m.insert(
        "help.compact.alias".into(),
        "Run aliases (ral add <name> ...)".into(),
    );
    m.insert(
        "help.compact.view".into(),
        "Directory and file trees".into(),
    );
    m.insert("help.compact.size".into(), "Directory sizes".into());
    m.insert(
        "help.compact.report".into(),
        "Project reports (report project)".into(),
    );
    m.insert(
        "help.compact.backup".into(),
        "Back up and restore a project".into(),
    );
    m.insert(
        "help.compact.search".into(),
        "Find text, files and folders".into(),
    );
    m.insert(
        "help.compact.igcare".into(),
        "Ignore/care rules for scans".into(),
    );
    m.insert(
        "help.compact.network".into(),
        "dl, up, http, serve, ping, dns, smtp".into(),
    );
    m.insert(
        "help.compact.tmprt".into(),
        "Temperature and weather".into(),
    );
    m.insert(
        "help.compact.time".into(),
        "Time and benchmark commands".into(),
    );
    m.insert(
        "help.compact.git".into(),
        "Git shortcuts (status, branch, log)".into(),
    );
    m.insert(
        "help.compact.clipboard".into(),
        "Clipboard history (clip list)".into(),
    );
    m.insert("help.compact.qsc".into(), "Quick command listener".into());
    m.insert("help.compact.todo".into(), "Project todo list".into());
    m.insert(
        "help.compact.arc".into(),
        "Create and extract archives".into(),
    );
    m.insert(
        "help.compact.scan".into(),
        "Scan extensions, ports, IPs".into(),
    );
    m.insert("help.compact.tl".into(), "Count lines in files".into());
    m.insert(
        "help.compact.ntpkg".into(),
        "Install and manage tools (ntpx)".into(),
    );
    m.insert("help.compact.qr".into(), "Make and read QR codes".into());
    m.insert(
        "help.compact.pathman".into(),
        "Manage PATH entries (usrpath, syspath)".into(),
    );
    m.insert(
        "help.compact.autoup".into(),
        "Manage startup programs".into(),
    );
    m.insert("help.compact.mode".into(), "Admin / normal mode".into());
    m.insert("help.compact.explorer".into(), "File explorer (nfe)".into());
    m.insert(
        "help.compact.math".into(),
        "Evaluate math expressions".into(),
    );
    m.insert(
        "help.compact.python".into(),
        "Activate a Python venv".into(),
    );
    m.insert(
        "help.compact.encdec".into(),
        "encode, decode, cvcode".into(),
    );
    m.insert("help.compact.fmt".into(), "Format files".into());
    m.insert(
        "help.compact.image".into(),
        "Show images inline in the terminal".into(),
    );
    m.insert("help.compact.ndb".into(), "Browse SQLite databases".into());
    m.insert("help.compact.nsheet".into(), "View spreadsheets".into());
    m.insert(
        "help.compact.nssh".into(),
        "SSH profiles and file copy".into(),
    );
    m.insert("help.compact.nms".into(), "Terminal music player".into());
    m.insert("help.compact.fake".into(), "Generate fake data".into());
    m.insert(
        "help.compact.deadlock".into(),
        "Encrypted password vault".into(),
    );
    m.insert(
        "help.compact.syscmd".into(),
        "Run any system command".into(),
    );
    m.insert(
        "help.compact.pipe".into(),
        "Pipes, redirects and filters".into(),
    );
    m.insert(
        "help.compact.fileops".into(),
        "mkf, mkd, rmf, rmd, mv, cp".into(),
    );
    m.insert(
        "help.compact.other".into(),
        "clear, version, say, ui, exit, help".into(),
    );
}
