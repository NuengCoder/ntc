use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "idle.tp".into(),
        "try: tp add <dir_name>  then: @<dir_name>   (save and jump back to this path)".into(),
    );

    m.insert(
        "idle.explorer".into(),
        "try: nfe   (browse this folder with the file explorer; f finds files)".into(),
    );

    m.insert(
        "idle.ne".into(),
        "try: ne --init <file>  (create a file from a template)".into(),
    );

    m.insert(
        "idle.view".into(),
        "try: view -d 2   (tree, two levels deep)".into(),
    );

    m.insert(
        "idle.search".into(),
        "try: gs <text>   (find that text inside this folder's files)".into(),
    );

    m.insert(
        "idle.todo".into(),
        "try: todo add \"task name\"   (this folder already has a todo list)".into(),
    );

    m.insert(
        "idle.git".into(),
        "try: git status   (this is a git repository)".into(),
    );

    m.insert(
        "idle.serve".into(),
        "try: serve files --lock   (share this folder read-only over HTTP)".into(),
    );

    m.insert(
        "idle.tutorial".into(),
        "try: tutorial   (hands-on lessons in a throwaway sandbox)".into(),
    );

    m.insert(
        "idle.ral".into(),
        "try: ral add <name> \"<command>\"   (type <name> to run it again)".into(),
    );

    m.insert(
        "idle.ran".into(),
        "try: ran list   (this folder has an NTCRANFILE.toml with tasks)".into(),
    );

    m.insert(
        "idle.pipe".into(),
        "try: gs <text> | sort -u | head 10   (pipe one command into another)".into(),
    );

    m.insert(
        "idle.math".into(),
        "try: math 3+4*5   (calculator with functions, no external tool)".into(),
    );

    m.insert(
        "idle.qr".into(),
        "try: qr <text|url>   (print a QR code you can scan)".into(),
    );

    m.insert(
        "idle.theme".into(),
        "try: theme list   (then: theme <name> - recolor the shell)".into(),
    );

    m.insert(
        "idle.backup".into(),
        "try: backup create   (snapshot this folder before risky edits)".into(),
    );

    m.insert(
        "idle.empty".into(),
        "try: mkf notes.md   (this folder is empty - start with a file)".into(),
    );

    m.insert(
        "idle.help".into(),
        "try: help   (list every command, or: help <topic>)".into(),
    );

    m.insert(
        "idle.tl".into(),
        "try: tl dir .   (count the lines of every supported file here)".into(),
    );

    m.insert(
        "idle.size".into(),
        "try: size   (total size of this folder)".into(),
    );

    m.insert(
        "idle.report".into(),
        "try: report project md   (save a markdown project report here)".into(),
    );

    m.insert(
        "idle.scr".into(),
        "try: scr init demo   (start a reusable .ntc script)".into(),
    );

    m.insert(
        "idle.time".into(),
        "try: time bench --runs 3 <cmd>   (average a command's runtime)".into(),
    );

    m.insert(
        "idle.encode".into(),
        "try: encode b64 \"hello\"   (base64, hex, url and more)".into(),
    );

    m.insert(
        "idle.fmt".into(),
        "try: fmt Cargo.toml to json   (convert config formats)".into(),
    );

    m.insert(
        "idle.arc".into(),
        "try: arc list <file.zip>   (inspect an archive without extracting)".into(),
    );

    m.insert(
        "idle.scan".into(),
        "try: scan ext rs   (list every .rs file here)".into(),
    );

    m.insert(
        "idle.sys".into(),
        "try: sys   (live CPU, memory and process monitor)".into(),
    );

    m.insert(
        "idle.showcg".into(),
        "try: showcg   (see every current setting at a glance)".into(),
    );

    m.insert(
        "idle.ntpkg".into(),
        "try: ntpkg list   (runtimes installed through ntc)".into(),
    );

    m.insert(
        "idle.ndb".into(),
        "try: ndb   (browse SQLite databases in this folder)".into(),
    );

    m.insert(
        "idle.nsheet".into(),
        "try: nsheet open <file.csv>   (browse a spreadsheet)".into(),
    );

    m.insert(
        "idle.fgo".into(),
        "try: fgo task.txt   (find a file by name, then jump to it)".into(),
    );

    m.insert(
        "idle.back".into(),
        "try: back   (up one folder; gos, gosc and godrive are one-word pickers too)".into(),
    );

    m.insert(
        "idle.viewf".into(),
        "try: viewf .   (files only) \u{b7} viewd . keeps folders only".into(),
    );

    m.insert(
        "idle.views".into(),
        "try: views .   (folder listing with --size and --sort)".into(),
    );

    m.insert(
        "idle.txtf".into(),
        "try: txtf task.txt --head 20   (print a file) \u{b7} txtc copies it".into(),
    );

    m.insert(
        "idle.text".into(),
        "try: head 5 task.txt   (also tail, betw 2:4, sort, rev)".into(),
    );

    m.insert(
        "idle.count".into(),
        "try: view task.txt | count ntc   (count matches in piped text) \u{b7} trim removes".into(),
    );

    m.insert(
        "idle.files".into(),
        "try: mkf notes.md   (then mkd, cp, mv, rnmf, rmf, rmd)".into(),
    );

    m.insert(
        "idle.config".into(),
        "try: config output   (one setting; showcg shows them all)".into(),
    );

    m.insert(
        "idle.opencg".into(),
        "try: opencg   (edit config.toml) \u{b7} resetcg, restorecg bring it back".into(),
    );

    m.insert(
        "idle.local".into(),
        "try: config local init   (settings that apply to this folder only)".into(),
    );

    m.insert(
        "idle.project".into(),
        "try: init   (create NTCRANFILE.toml here) \u{b7} deinit removes it".into(),
    );

    m.insert(
        "idle.tpb".into(),
        "try: tpb history   (the paths you jumped to earlier)".into(),
    );

    m.insert(
        "idle.qur".into(),
        "try: qur *.txt -s \"old\" -dp   (select first, replace with -rp)".into(),
    );

    m.insert(
        "idle.fsc".into(),
        "try: fsc task.txt   (find a file, then open it) \u{b7} fs files, ds folders".into(),
    );

    m.insert(
        "idle.ignore".into(),
        "try: ignore dir target   (skip a folder in scans) \u{b7} care takes it back".into(),
    );

    m.insert(
        "idle.igcare".into(),
        "try: igcared   (every ignore and care rule in effect)".into(),
    );

    m.insert(
        "idle.http".into(),
        "try: http --help   (HTTP client: methods, headers, bodies)".into(),
    );

    m.insert(
        "idle.dl".into(),
        "try: dl --help   (download a file with resume)".into(),
    );

    m.insert(
        "idle.dns".into(),
        "try: dns example.com -t MX   (look up DNS records)".into(),
    );

    m.insert(
        "idle.up".into(),
        "try: up --help   (upload a file over HTTP)".into(),
    );

    m.insert(
        "idle.ping".into(),
        "try: ping 127.0.0.1   (check that a host answers)".into(),
    );

    m.insert(
        "idle.trace".into(),
        "try: trace 127.0.0.1   (hop-by-hop route to a host)".into(),
    );

    m.insert(
        "idle.smtp".into(),
        "try: smtp --help   (send mail; passwords live in deadlock)".into(),
    );

    m.insert(
        "idle.ipcfg".into(),
        "try: ipcfg   (adapters and addresses of this machine)".into(),
    );

    m.insert(
        "idle.wi".into(),
        "try: wi .   (watch this folder change live; q quits)".into(),
    );

    m.insert(
        "idle.spec".into(),
        "try: spec   (CPU, RAM, disk and OS summary)".into(),
    );

    m.insert(
        "idle.tmprt".into(),
        "try: tmprt city   (set the default city) \u{b7} tmprt com for sensors".into(),
    );

    m.insert(
        "idle.clip".into(),
        "try: clip   (clipboard slots) \u{b7} clip 2 recalls slot 2".into(),
    );

    m.insert(
        "idle.qsc".into(),
        "try: qsc status   (is the global hotkey daemon running)".into(),
    );

    m.insert(
        "idle.pathman".into(),
        "try: usrpath show   (your PATH) \u{b7} syspath show for the system PATH".into(),
    );

    m.insert(
        "idle.autoup".into(),
        "try: autoup list   (programs that start with the machine)".into(),
    );

    m.insert(
        "idle.mode".into(),
        "try: mode   (current admin or normal mode)".into(),
    );

    m.insert(
        "idle.python".into(),
        "try: showenv   (session env vars) \u{b7} py_activate turns a venv on".into(),
    );

    m.insert(
        "idle.decode".into(),
        "try: decode b64 \"aGVsbG8=\"   (undo encode) \u{b7} cvcode converts formats".into(),
    );

    m.insert(
        "idle.image".into(),
        "try: image --help   (show images inline in the terminal)".into(),
    );

    m.insert(
        "idle.nssh".into(),
        "try: nssh ls   (saved SSH profiles)".into(),
    );

    m.insert(
        "idle.nms".into(),
        "try: nms scan .   (find playable media here)".into(),
    );

    m.insert(
        "idle.fake".into(),
        "try: fake gen name,email -n 3   (test data; fake types lists all)".into(),
    );

    m.insert(
        "idle.deadlock".into(),
        "try: deadlock   (encrypted password vault)".into(),
    );

    m.insert(
        "idle.ntpx".into(),
        "try: ntpx   (run a runtime installed by ntpkg)".into(),
    );

    m.insert(
        "idle.pathcp".into(),
        "try: pathcp   (copy the current path to the clipboard)".into(),
    );

    m.insert(
        "idle.say".into(),
        "try: say hello --loop 3   (print text; print is the same)".into(),
    );

    m.insert(
        "idle.ui".into(),
        "try: ui   (show the UI mode) \u{b7} version prints the build".into(),
    );
}
