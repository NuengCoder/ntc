use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.compact.title".into(), "Command overview".into());

    m.insert(
        "help.compact.footer".into(),
        "Type 'help <command>' for details, 'help --all' for the full reference.".into(),
    );

    m.insert(
        "help.compact.arc".into(),
        "Create and extract archives".into(),
    );

    m.insert(
        "help.compact.autoup".into(),
        "Manage startup programs [Windows Only]".into(),
    );

    m.insert(
        "help.compact.back".into(),
        "Go back to parent directory".into(),
    );

    m.insert(
        "help.compact.backup".into(),
        "Back up and restore a project".into(),
    );

    m.insert("help.compact.betw".into(), "Print lines A to B".into());

    m.insert("help.compact.care".into(), "Care (un-ignore) rules".into());

    m.insert("help.compact.cd".into(), "Change directory".into());

    m.insert("help.compact.clear".into(), "Clear the screen".into());

    m.insert("help.compact.clip".into(), "Clipboard history".into());

    m.insert(
        "help.compact.config".into(),
        "Show or change ntc settings".into(),
    );

    m.insert(
        "help.compact.count".into(),
        "Count occurrences in text".into(),
    );

    m.insert("help.compact.cp".into(), "Copy a file or directory".into());

    m.insert(
        "help.compact.cvcode".into(),
        "Convert between formats".into(),
    );

    m.insert(
        "help.compact.deadlock".into(),
        "Encrypted password vault".into(),
    );

    m.insert(
        "help.compact.decode".into(),
        "Decode hex, base64, URL, binary".into(),
    );

    m.insert("help.compact.deinit".into(), "Remove project files".into());

    m.insert("help.compact.dl".into(), "Download a file".into());

    m.insert("help.compact.dns".into(), "DNS lookup".into());

    m.insert(
        "help.compact.doctor".into(),
        "Check config, PATH, ranfile and ntpkg".into(),
    );

    m.insert(
        "help.compact.ds".into(),
        "Search directories by name".into(),
    );

    m.insert(
        "help.compact.encode".into(),
        "Encode hex, base64, URL, binary".into(),
    );

    m.insert("help.compact.exit".into(), "Exit ntc".into());

    m.insert(
        "help.compact.explorer".into(),
        "Full-screen file explorer".into(),
    );

    m.insert("help.compact.fake".into(), "Generate fake data".into());

    m.insert(
        "help.compact.fgo".into(),
        "Find files and jump to one".into(),
    );

    m.insert(
        "help.compact.fmt".into(),
        "Convert between file formats".into(),
    );

    m.insert("help.compact.fs".into(), "Search files by name".into());

    m.insert(
        "help.compact.fsc".into(),
        "Search files and open one".into(),
    );

    m.insert(
        "help.compact.git".into(),
        "Git shortcuts (status, branch, log)".into(),
    );

    m.insert(
        "help.compact.godrive".into(),
        "Switch drives or /mnt mounts".into(),
    );

    m.insert(
        "help.compact.gos".into(),
        "Pick a subdirectory to enter".into(),
    );

    m.insert(
        "help.compact.gosc".into(),
        "Browse subdirectories continuously".into(),
    );

    m.insert("help.compact.gs".into(), "Search file contents".into());

    m.insert(
        "help.compact.head".into(),
        "Print the first lines of a file".into(),
    );

    m.insert("help.compact.help".into(), "Show this help".into());

    m.insert("help.compact.http".into(), "Make an HTTP request".into());

    m.insert(
        "help.compact.igcare".into(),
        "Import and export ignore/care rules".into(),
    );

    m.insert(
        "help.compact.igcared".into(),
        "Show all ignore/care settings".into(),
    );

    m.insert(
        "help.compact.ignore".into(),
        "Ignore dirs, files and extensions".into(),
    );

    m.insert(
        "help.compact.image".into(),
        "Show images inline in the terminal".into(),
    );

    m.insert(
        "help.compact.init".into(),
        "Initialize project files".into(),
    );

    m.insert("help.compact.ipcfg".into(), "Show IP configuration".into());

    m.insert(
        "help.compact.local".into(),
        "Manage the per-directory config".into(),
    );

    m.insert(
        "help.compact.locate".into(),
        "Search files and directories".into(),
    );

    m.insert(
        "help.compact.math".into(),
        "Evaluate math expressions".into(),
    );

    m.insert("help.compact.mkd".into(), "Create directories".into());

    m.insert("help.compact.mkf".into(), "Create files".into());

    m.insert("help.compact.mode".into(), "Admin / normal mode".into());

    m.insert(
        "help.compact.mv".into(),
        "Move or rename files and directories".into(),
    );

    m.insert("help.compact.ndb".into(), "Browse SQLite databases".into());

    m.insert("help.compact.ne".into(), "Open the built-in editor".into());

    m.insert("help.compact.nms".into(), "Terminal music player".into());

    m.insert("help.compact.nsheet".into(), "View spreadsheets".into());

    m.insert(
        "help.compact.nssh".into(),
        "SSH profiles and file copy".into(),
    );

    m.insert(
        "help.compact.ntpkg".into(),
        "Install and manage tools".into(),
    );

    m.insert(
        "help.compact.ntpx".into(),
        "Run an installed package binary".into(),
    );

    m.insert(
        "help.compact.opencg".into(),
        "Open config.toml in the editor".into(),
    );

    m.insert(
        "help.compact.pathcp".into(),
        "Copy the current path to clipboard".into(),
    );

    m.insert("help.compact.ping".into(), "Ping a host".into());

    m.insert(
        "help.compact.pipe".into(),
        "Pipes, redirects and filters [NON-COMMAND]".into(),
    );

    m.insert(
        "help.compact.py_activate".into(),
        "Activate a Python venv".into(),
    );

    m.insert(
        "help.compact.py_deactivate".into(),
        "Deactivate the Python venv".into(),
    );

    m.insert("help.compact.qr".into(), "Make and read QR codes".into());

    m.insert("help.compact.qsc".into(), "Quick command listener".into());

    m.insert(
        "help.compact.qur".into(),
        "Query files and replace text".into(),
    );

    m.insert("help.compact.ral".into(), "Run aliases".into());

    m.insert(
        "help.compact.ran".into(),
        "Run tasks from NTCRANFILE.toml".into(),
    );

    m.insert(
        "help.compact.report".into(),
        "Project reports (report project)".into(),
    );

    m.insert(
        "help.compact.resetcg".into(),
        "Reset config to defaults".into(),
    );

    m.insert(
        "help.compact.restorecg".into(),
        "Restore config from backup".into(),
    );

    m.insert("help.compact.rev".into(), "Reverse each line".into());

    m.insert("help.compact.rmd".into(), "Delete directories".into());

    m.insert("help.compact.rmf".into(), "Delete files".into());

    m.insert("help.compact.rnmd".into(), "Rename a directory".into());

    m.insert("help.compact.rnmf".into(), "Rename a file".into());

    m.insert("help.compact.run".into(), "Execute an alias".into());

    m.insert("help.compact.say".into(), "Print text".into());

    m.insert(
        "help.compact.nprintf".into(),
        "Bash-style formatted output".into(),
    );

    m.insert(
        "help.compact.scan".into(),
        "Scan extensions, ports, IPs".into(),
    );

    m.insert("help.compact.scr".into(), "Run saved script files".into());

    m.insert(
        "help.compact.serve".into(),
        "Serve a directory or app".into(),
    );

    m.insert(
        "help.compact.showcg".into(),
        "Show the current configuration".into(),
    );

    m.insert(
        "help.compact.showenv".into(),
        "Show environment and PATH".into(),
    );

    m.insert("help.compact.size".into(), "Directory sizes".into());

    m.insert("help.compact.skill".into(), "Install AI skills".into());

    m.insert(
        "help.compact.skip".into(),
        "Skip the first lines or chosen lines".into(),
    );

    m.insert(
        "help.compact.smtp".into(),
        "Send email (SMTP client)".into(),
    );

    m.insert("help.compact.sort".into(), "Sort lines".into());

    m.insert(
        "help.compact.spec".into(),
        "Show computer specifications".into(),
    );

    m.insert("help.compact.sys".into(), "Live system monitor".into());

    m.insert(
        "help.compact.syspath".into(),
        "Manage the system PATH".into(),
    );

    m.insert(
        "help.compact.tail".into(),
        "Print the last lines of a file".into(),
    );

    m.insert(
        "help.compact.theme".into(),
        "Switch, add, export themes".into(),
    );

    m.insert(
        "help.compact.time".into(),
        "Command timing and countdowns".into(),
    );

    m.insert("help.compact.tl".into(), "Count lines in files".into());

    m.insert(
        "help.compact.tmprt".into(),
        "Temperature and weather".into(),
    );

    m.insert("help.compact.todo".into(), "Project todo list".into());

    m.insert(
        "help.compact.tp".into(),
        "Saved paths (tp jump <name>)".into(),
    );

    m.insert(
        "help.compact.tpb".into(),
        "Teleport back to the previous location".into(),
    );

    m.insert(
        "help.compact.trace".into(),
        "Trace the route to a host".into(),
    );

    m.insert(
        "help.compact.trim".into(),
        "Strip a pattern from text".into(),
    );

    m.insert(
        "help.compact.tutorial".into(),
        "Guided hands-on lessons".into(),
    );

    m.insert(
        "help.compact.txtc".into(),
        "Copy file content to clipboard".into(),
    );

    m.insert("help.compact.txtf".into(), "Display file content".into());

    m.insert("help.compact.ui".into(), "Show current UI mode".into());

    m.insert("help.compact.up".into(), "Upload a file".into());

    m.insert(
        "help.compact.usrpath".into(),
        "Manage the user PATH".into(),
    );

    m.insert("help.compact.version".into(), "Show version".into());

    m.insert(
        "help.compact.view".into(),
        "Directory and file trees".into(),
    );

    m.insert("help.compact.viewd".into(), "Directories-only tree".into());

    m.insert("help.compact.viewf".into(), "Files-only tree".into());

    m.insert(
        "help.compact.views".into(),
        "Short flat directory listing".into(),
    );

    m.insert(
        "help.compact.where".into(),
        "Show the ntc path or find binaries".into(),
    );

    m.insert("help.compact.wi".into(), "Live file-change feed".into());

    m.insert(
        "help.compact.line".into(),
        "Number each line".into(),
    );
}
