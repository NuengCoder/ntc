//! Lesson catalogue and the tutorial workspace it lives in.
//!
//! The tutorial is a disposable sandbox under `<config>/ntc/tutorial/`: one
//! folder per command area, each holding its own fixtures plus a generated
//! `task.txt`. Lessons never run the command for the learner — they only
//! state a goal and check that the learner actually did it.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// What counts as "the learner did it".
pub(crate) enum Check {
    /// Current directory is this folder (relative to the tutorial root).
    InDir(&'static str),
    /// The last command was `cmd` and its arguments contained `contains`.
    Typed {
        cmd: &'static str,
        contains: &'static str,
    },
    /// The last command had this name, whatever its arguments.
    TypedCmd(&'static str),
    /// The raw line typed in the shell started with this text (aliases).
    InputStarts(&'static str),
}

pub(crate) struct LessonStep {
    pub goal: &'static str,
    /// The exact line the learner should type. Shown up front, never run by
    /// ntc itself: the learner types it and presses Enter.
    pub command: &'static str,
    /// A one-line nudge for `tutorial hint`, not a copy of the command.
    pub hint: &'static str,
    /// One more thing to try with the same idea, shown once the goal is met.
    /// Must be a command the lab allows and must not satisfy the next step.
    pub also_try: &'static str,
    pub check: Check,
}

pub(crate) struct Lesson {
    pub key: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub steps: &'static [LessonStep],
}

pub(crate) const LESSONS: &[Lesson] = &[
    Lesson {
        key: "cd",
        title: "Navigation",
        summary: "move around with cd / back / where",
        steps: &[
            LessonStep {
                goal: "Go into the `inner` folder of this lesson.",
                command: "cd inner/",
                hint: "`cd` takes a folder name, and `inner` is right here.",
                also_try: "view sample.txt",
                check: Check::InDir("cd/inner"),
            },
            LessonStep {
                goal: "Now go back up one level, to the lesson folder.",
                command: "cd ..",
                hint: "`..` always means the parent folder.",
                also_try: "back",
                check: Check::InDir("cd"),
            },
            LessonStep {
                goal: "Ask ntc where you are (its own one-word command, not the OS `pwd`).",
                command: "where",
                hint: "`where` prints the full path of the current folder.",
                also_try: "help nav",
                check: Check::TypedCmd("where"),
            },
        ],
    },
    Lesson {
        key: "view",
        title: "Reading files",
        summary: "view files and folders, line numbers, depth",
        steps: &[
            LessonStep {
                goal: "Show the contents of `notes.txt` here without opening the editor.",
                command: "view notes.txt",
                hint: "`view` prints a file without opening the editor.",
                also_try: "head 20 notes.txt",
                check: Check::Typed {
                    cmd: "view",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Show this whole lesson folder as a tree two levels deep.",
                command: "view -d 2 ..",
                hint: "`-d` sets the tree depth; `..` is the parent folder.",
                also_try: "view --sort size ..",
                check: Check::Typed {
                    cmd: "view",
                    contains: "-d",
                },
            },
        ],
    },
    Lesson {
        key: "ne",
        title: "Editor",
        summary: "create and edit a file with ne",
        steps: &[
            LessonStep {
                goal: "Create `draft.md` here with the editor's create flag, without opening the editor yet.",
                command: "ne --init draft.md",
                hint: "`--init` creates the file without opening the editor.",
                also_try: "view task.txt",
                check: Check::Typed {
                    cmd: "ne",
                    contains: "--init",
                },
            },
            LessonStep {
                goal: "Open `draft.md`, type a line, save and quit the editor.",
                command: "ne draft.md",
                hint: "Inside the editor: `i` to insert, Esc, then `:wq` to save and quit.",
                also_try: "head 5 draft.md",
                check: Check::Typed {
                    cmd: "ne",
                    contains: "draft.md",
                },
            },
            LessonStep {
                goal: "Confirm from the shell that the file now has content (no editor).",
                command: "view draft.md",
                hint: "`view` shows the saved lines from the shell.",
                also_try: "help editor",
                check: Check::Typed {
                    cmd: "view",
                    contains: "draft.md",
                },
            },
        ],
    },
    Lesson {
        key: "ral",
        title: "Aliases",
        summary: "create, run and remove a ral alias",
        steps: &[
            LessonStep {
                goal: "Create an alias named `tuthello` that prints a greeting.",
                command: "ral add tuthello \"echo hello from my alias\"",
                hint: "Quote the command so it stays one argument.",
                also_try: "ral list",
                check: Check::Typed {
                    cmd: "ral",
                    contains: "tuthello",
                },
            },
            LessonStep {
                goal: "Run your alias by typing its name.",
                command: "tuthello",
                hint: "An alias is just a command of your own.",
                also_try: "help alias",
                check: Check::InputStarts("tuthello"),
            },
            LessonStep {
                goal: "Remove the alias so the tutorial leaves nothing behind.",
                command: "ral rm tuthello",
                hint: "`ral rm` removes the alias again.",
                also_try: "ral list",
                check: Check::Typed {
                    cmd: "ral",
                    contains: "tuthello",
                },
            },
        ],
    },
    Lesson {
        key: "ran",
        title: "Task runner",
        summary: "list and run targets from NTCRANFILE.toml",
        steps: &[
            LessonStep {
                goal: "List the tasks defined in this lesson's `NTCRANFILE.toml`.",
                command: "ran list",
                hint: "`ran list` reads NTCRANFILE.toml in this folder.",
                also_try: "ran check",
                check: Check::Typed {
                    cmd: "ran",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Run the task named `hello`.",
                command: "ran hello",
                hint: "Put the target name right after `ran`.",
                also_try: "ran standalone",
                check: Check::Typed {
                    cmd: "ran",
                    contains: "hello",
                },
            },
            LessonStep {
                goal: "Read how a ranfile can be converted to JSON or YAML.",
                command: "help ran",
                hint: "`ran fmt json` (or yaml) converts it; `ran check` validates any format.",
                also_try: "help scr",
                check: Check::Typed {
                    cmd: "help",
                    contains: "ran",
                },
            },
        ],
    },
    Lesson {
        key: "nav",
        title: "Navigation extras",
        summary: "fgo, back, and the gos / gosc / godrive pickers",
        steps: &[
            LessonStep {
                goal: "Find `task.txt` by name without leaving this folder.",
                command: "fgo task.txt",
                hint: "`fgo` searches file names below the current folder.",
                also_try: "help nav",
                check: Check::Typed {
                    cmd: "fgo",
                    contains: "task",
                },
            },
            LessonStep {
                goal: "Step back up one folder in the history.",
                command: "back",
                hint: "`back` is the parent jump; `back 2` goes two up.",
                also_try: "where",
                check: Check::TypedCmd("back"),
            },
            LessonStep {
                goal: "Read the help for the numbered folder pickers.",
                command: "help nav",
                hint: "`gos` picks a subfolder, `gosc` keeps picking, `godrive` lists drives.",
                also_try: "view task.txt",
                check: Check::Typed {
                    cmd: "help",
                    contains: "nav",
                },
            },
        ],
    },
    Lesson {
        key: "views",
        title: "Views & text files",
        summary: "viewf, viewd, views, txtf, txtc",
        steps: &[
            LessonStep {
                goal: "List only the files here, not the folders.",
                command: "viewf .",
                hint: "`viewf` keeps files; `viewd` keeps only folders.",
                also_try: "view -d 1 .",
                check: Check::Typed {
                    cmd: "viewf",
                    contains: ".",
                },
            },
            LessonStep {
                goal: "List only the folders here.",
                command: "viewd .",
                hint: "`viewd` is the folder-only version of `view`.",
                also_try: "help view",
                check: Check::Typed {
                    cmd: "viewd",
                    contains: ".",
                },
            },
            LessonStep {
                goal: "List this folder with the views listing (sizes and sorting).",
                command: "views .",
                hint: "`views` lists a folder; add --size or --sort to change it.",
                also_try: "help view",
                check: Check::Typed {
                    cmd: "views",
                    contains: ".",
                },
            },
            LessonStep {
                goal: "Show `notes.txt` from the shell with line numbers.",
                command: "txtf notes.txt",
                hint: "`txtf` displays a file; `--no-line` hides the numbers.",
                also_try: "head 5 notes.txt",
                check: Check::Typed {
                    cmd: "txtf",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Copy `notes.txt` to the clipboard instead of printing it.",
                command: "txtc notes.txt",
                hint: "`txtc` puts the file content on the clipboard.",
                also_try: "help report",
                check: Check::Typed {
                    cmd: "txtc",
                    contains: "notes.txt",
                },
            },
        ],
    },
    Lesson {
        key: "text",
        title: "Text filters",
        summary: "head, tail, betw, sort, rev, count, trim",
        steps: &[
            LessonStep {
                goal: "Print the first 5 lines of `notes.txt`.",
                command: "head 5 notes.txt",
                hint: "`head N file` keeps the first N lines.",
                also_try: "view notes.txt",
                check: Check::Typed {
                    cmd: "head",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Print the last 5 lines of `notes.txt`.",
                command: "tail 5 notes.txt",
                hint: "`tail N file` keeps the last N lines.",
                also_try: "help pipe",
                check: Check::Typed {
                    cmd: "tail",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Print lines 2 to 4 of `notes.txt`.",
                command: "betw 2:4 notes.txt",
                hint: "`betw A:B` prints that line range, 1-based.",
                also_try: "head 4 notes.txt",
                check: Check::Typed {
                    cmd: "betw",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Sort the lines of `notes.txt`.",
                command: "sort notes.txt",
                hint: "`sort` sorts lines; -r reverses, -u keeps uniques.",
                also_try: "help pipe",
                check: Check::Typed {
                    cmd: "sort",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Reverse the characters of every line in `notes.txt`.",
                command: "rev notes.txt",
                hint: "`rev` flips each line character by character.",
                also_try: "sort -r notes.txt",
                check: Check::Typed {
                    cmd: "rev",
                    contains: "notes.txt",
                },
            },
            LessonStep {
                goal: "Count how often `ntc` appears, by piping the file into `count`.",
                command: "view notes.txt | count ntc",
                hint: "A pipe feeds one command's output into the next.",
                also_try: "help pipe",
                check: Check::InputStarts("view notes.txt |"),
            },
            LessonStep {
                goal: "Remove the word `Tutorial ` from the same piped text.",
                command: "view notes.txt | trim \"Tutorial \"",
                hint: "`trim` deletes every occurrence of the pattern.",
                also_try: "view notes.txt | sort -u",
                check: Check::InputStarts("view notes.txt |"),
            },
        ],
    },
    Lesson {
        key: "files",
        title: "File operations",
        summary: "mkf, mkd, cp, rnmf, rnmd, mv, rmf, rmd",
        steps: &[
            LessonStep {
                goal: "Create an empty file named `note.txt`.",
                command: "mkf note.txt",
                hint: "`mkf` creates files, `mkd` creates folders.",
                also_try: "view task.txt",
                check: Check::Typed {
                    cmd: "mkf",
                    contains: "note.txt",
                },
            },
            LessonStep {
                goal: "Create a folder named `scratch`.",
                command: "mkd scratch",
                hint: "`mkd` makes a directory in the current folder.",
                also_try: "help fileops",
                check: Check::Typed {
                    cmd: "mkd",
                    contains: "scratch",
                },
            },
            LessonStep {
                goal: "Copy `note.txt` to `copy.txt`.",
                command: "cp note.txt copy.txt",
                hint: "`cp <src> <dst>` copies a file or folder.",
                also_try: "viewf .",
                check: Check::Typed {
                    cmd: "cp",
                    contains: "copy.txt",
                },
            },
            LessonStep {
                goal: "Rename `copy.txt` to `renamed.txt`.",
                command: "rnmf copy.txt renamed.txt",
                hint: "`rnmf <old> <new>` renames a file.",
                also_try: "help fileops",
                check: Check::Typed {
                    cmd: "rnmf",
                    contains: "renamed.txt",
                },
            },
            LessonStep {
                goal: "Rename the folder `scratch` to `tmp`.",
                command: "rnmd scratch tmp",
                hint: "`rnmd <old> <new>` renames a directory.",
                also_try: "viewd .",
                check: Check::Typed {
                    cmd: "rnmd",
                    contains: "tmp",
                },
            },
            LessonStep {
                goal: "Move `renamed.txt` into the `tmp` folder.",
                command: "mv renamed.txt tmp",
                hint: "`mv` moves; with a folder destination it keeps the name.",
                also_try: "help fileops",
                check: Check::Typed {
                    cmd: "mv",
                    contains: "tmp",
                },
            },
            LessonStep {
                goal: "Delete `tmp/renamed.txt` (answer the confirmation).",
                command: "rmf tmp/renamed.txt",
                hint: "`rmf` deletes a file and asks before doing it.",
                also_try: "viewd tmp",
                check: Check::Typed {
                    cmd: "rmf",
                    contains: "renamed.txt",
                },
            },
            LessonStep {
                goal: "Delete the now empty `tmp` folder.",
                command: "rmd tmp",
                hint: "`rmd` removes a directory after confirmation.",
                also_try: "help fileops",
                check: Check::Typed {
                    cmd: "rmd",
                    contains: "tmp",
                },
            },
        ],
    },
    Lesson {
        key: "search",
        title: "Search",
        summary: "gs, fs, ds, locate, fsc, qur",
        steps: &[
            LessonStep {
                goal: "Search the contents of the files here for the word `ntc`.",
                command: "gs ntc",
                hint: "`gs <pattern>` greps file contents below this folder.",
                also_try: "help search",
                check: Check::Typed {
                    cmd: "gs",
                    contains: "ntc",
                },
            },
            LessonStep {
                goal: "Find files named `notes.txt`.",
                command: "fs notes.txt",
                hint: "`fs` searches file names.",
                also_try: "help search",
                check: Check::Typed {
                    cmd: "fs",
                    contains: "notes",
                },
            },
            LessonStep {
                goal: "Find directories by name.",
                command: "ds .",
                hint: "`ds` is the folder-only search.",
                also_try: "viewd .",
                check: Check::Typed {
                    cmd: "ds",
                    contains: ".",
                },
            },
            LessonStep {
                goal: "Search files and folders at once for `notes`.",
                command: "locate notes",
                hint: "`locate` matches both kinds of entry.",
                also_try: "help search",
                check: Check::Typed {
                    cmd: "locate",
                    contains: "notes",
                },
            },
            LessonStep {
                goal: "Find `task.txt` and pick it to display.",
                command: "fsc task.txt",
                hint: "`fsc` finds a file, then shows the one you pick.",
                also_try: "help search",
                check: Check::Typed {
                    cmd: "fsc",
                    contains: "task",
                },
            },
            LessonStep {
                goal: "Query every text file for `ntc` and display the matches.",
                command: "qur *.txt -s ntc -dp",
                hint: "`-s` selects a pattern, `-dp` displays matches, `-rp` replaces.",
                also_try: "help search",
                check: Check::Typed {
                    cmd: "qur",
                    contains: "ntc",
                },
            },
        ],
    },
    Lesson {
        key: "config",
        title: "Configuration",
        summary: "config, showcg, opencg, resetcg, restorecg, local",
        steps: &[
            LessonStep {
                goal: "Show every current setting at once.",
                command: "showcg",
                hint: "`showcg` prints all settings; `config <name>` prints one.",
                also_try: "help config",
                check: Check::TypedCmd("showcg"),
            },
            LessonStep {
                goal: "Ask the `output` setting what its current value is.",
                command: "config output",
                hint: "A setting name alone reports its value; add a value to change it.",
                also_try: "help config",
                check: Check::Typed {
                    cmd: "config",
                    contains: "output",
                },
            },
            LessonStep {
                goal: "Create a project-only config file in this lesson folder.",
                command: "config local init",
                hint: "`config local` writes ntconfig.toml for this folder only.",
                also_try: "showcg",
                check: Check::Typed {
                    cmd: "config",
                    contains: "local",
                },
            },
            LessonStep {
                goal: "Remove that project-only config again.",
                command: "config local deinit",
                hint: "`config local deinit` deletes the local override.",
                also_try: "showcg",
                check: Check::Typed {
                    cmd: "config",
                    contains: "deinit",
                },
            },
            LessonStep {
                goal: "Read the help for the config file commands (opencg / resetcg / restorecg).",
                command: "help config",
                hint: "`opencg` edits it, `resetcg` resets, `restorecg` brings back the copy.",
                also_try: "view task.txt",
                check: Check::Typed {
                    cmd: "help",
                    contains: "config",
                },
            },
        ],
    },
    Lesson {
        key: "project",
        title: "Project files",
        summary: "init and deinit a workspace",
        steps: &[
            LessonStep {
                goal: "Initialize this folder as an ntc project.",
                command: "init",
                hint: "`init` writes NTCRANFILE.toml and ntconfig.toml here.",
                also_try: "view NTCRANFILE.toml",
                check: Check::TypedCmd("init"),
            },
            LessonStep {
                goal: "Remove the project files again.",
                command: "deinit --local",
                hint: "`deinit` drops only what `init` created.",
                also_try: "help other",
                check: Check::Typed {
                    cmd: "deinit",
                    contains: "local",
                },
            },
        ],
    },
    Lesson {
        key: "theme",
        title: "Themes",
        summary: "list and inspect shell themes",
        steps: &[
            LessonStep {
                goal: "List the installed themes.",
                command: "theme list",
                hint: "`theme list` shows what you can switch to.",
                also_try: "help theme",
                check: Check::Typed {
                    cmd: "theme",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Show which theme is active.",
                command: "theme current",
                hint: "`theme current` prints the active theme's name.",
                also_try: "help theme",
                check: Check::Typed {
                    cmd: "theme",
                    contains: "current",
                },
            },
        ],
    },
    Lesson {
        key: "tp",
        title: "Teleports",
        summary: "save a path and jump back to it",
        steps: &[
            LessonStep {
                goal: "Save this lesson folder as the teleport `tut_saved`.",
                command: "tp add tut_saved",
                hint: "`tp add <name>` stores the current path under that name.",
                also_try: "help teleport",
                check: Check::Typed {
                    cmd: "tp",
                    contains: "tut_saved",
                },
            },
            LessonStep {
                goal: "List every saved teleport.",
                command: "tp list",
                hint: "`tp list` shows names and their paths.",
                also_try: "help teleport",
                check: Check::Typed {
                    cmd: "tp",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Show where you jumped before (the teleport history).",
                command: "tpb history",
                hint: "`tpb` is the back-jump stack for teleports.",
                also_try: "help teleport",
                check: Check::Typed {
                    cmd: "tpb",
                    contains: "history",
                },
            },
            LessonStep {
                goal: "Delete the teleport you just created.",
                command: "tp rm tut_saved",
                hint: "`tp rm <name>` removes a savepoint.",
                also_try: "tp list",
                check: Check::Typed {
                    cmd: "tp",
                    contains: "rm",
                },
            },
        ],
    },
    Lesson {
        key: "scr",
        title: "Scripts",
        summary: "create and run a reusable .ntc script",
        steps: &[
            LessonStep {
                goal: "Create a starter script named `demo`.",
                command: "scr init demo",
                hint: "`scr init` writes the script skeleton into this folder.",
                also_try: "viewf .",
                check: Check::Typed {
                    cmd: "scr",
                    contains: "demo",
                },
            },
            LessonStep {
                goal: "List the scripts available here.",
                command: "scr ls",
                hint: "`scr ls` shows every script it can run.",
                also_try: "help scr",
                check: Check::Typed {
                    cmd: "scr",
                    contains: "ls",
                },
            },
            LessonStep {
                goal: "Run your script.",
                command: "scr run demo",
                hint: "`scr run <name>` executes the script line by line.",
                also_try: "help scr",
                check: Check::Typed {
                    cmd: "scr",
                    contains: "run",
                },
            },
        ],
    },
    Lesson {
        key: "tl",
        title: "Line counts",
        summary: "tl file / tl dir",
        steps: &[
            LessonStep {
                goal: "Count the lines in `task.txt`.",
                command: "tl file task.txt",
                hint: "`tl file` counts one file.",
                also_try: "count task.txt",
                check: Check::Typed {
                    cmd: "tl",
                    contains: "file",
                },
            },
            LessonStep {
                goal: "Count the lines of every supported file in this folder.",
                command: "tl dir .",
                hint: "`tl dir` totals the folder by extension.",
                also_try: "help tl",
                check: Check::Typed {
                    cmd: "tl",
                    contains: "dir",
                },
            },
        ],
    },
    Lesson {
        key: "size",
        title: "Folder size",
        summary: "size with path and timing",
        steps: &[
            LessonStep {
                goal: "Measure the size of this folder.",
                command: "size",
                hint: "`size` totals the current folder.",
                also_try: "help size",
                check: Check::TypedCmd("size"),
            },
            LessonStep {
                goal: "Measure it again and show how long the scan took.",
                command: "size -t",
                hint: "`-t` adds the elapsed time.",
                also_try: "help size",
                check: Check::Typed {
                    cmd: "size",
                    contains: "-t",
                },
            },
        ],
    },
    Lesson {
        key: "report",
        title: "Reports",
        summary: "report project txt / report project md",
        steps: &[
            LessonStep {
                goal: "Generate a plain-text project report here.",
                command: "report project txt",
                hint: "`report project txt` writes a .txt summary into this folder.",
                also_try: "viewf .",
                check: Check::Typed {
                    cmd: "report",
                    contains: "txt",
                },
            },
            LessonStep {
                goal: "Generate a Markdown report instead.",
                command: "report project md",
                hint: "`report project md` writes the same summary as Markdown.",
                also_try: "help report",
                check: Check::Typed {
                    cmd: "report",
                    contains: "md",
                },
            },
        ],
    },
    Lesson {
        key: "backup",
        title: "Backups",
        summary: "snapshot the folder and list snapshots",
        steps: &[
            LessonStep {
                goal: "Take a backup snapshot of this folder.",
                command: "backup create",
                hint: "`backup create` saves the current state before risky edits.",
                also_try: "help backup",
                check: Check::Typed {
                    cmd: "backup",
                    contains: "create",
                },
            },
            LessonStep {
                goal: "List the backups that exist.",
                command: "backup list",
                hint: "`backup list` shows each snapshot with its number.",
                also_try: "help backup",
                check: Check::Typed {
                    cmd: "backup",
                    contains: "list",
                },
            },
        ],
    },
    Lesson {
        key: "ignore",
        title: "Ignore & care",
        summary: "skip folders in scans, then take them back",
        steps: &[
            LessonStep {
                goal: "Tell ntc to skip the folder `tmp` in scans.",
                command: "ignore dir tmp",
                hint: "`ignore dir <name>` drops a folder from search and size.",
                also_try: "help igcare",
                check: Check::Typed {
                    cmd: "ignore",
                    contains: "tmp",
                },
            },
            LessonStep {
                goal: "Take the folder back with `care`.",
                command: "care dir tmp",
                hint: "`care` is the opposite of `ignore`.",
                also_try: "help igcare",
                check: Check::Typed {
                    cmd: "care",
                    contains: "tmp",
                },
            },
            LessonStep {
                goal: "Show every ignore and care rule in effect.",
                command: "igcared",
                hint: "`igcared` lists the rules for this folder and globally.",
                also_try: "help igcare",
                check: Check::TypedCmd("igcared"),
            },
            LessonStep {
                goal: "Run the combined ignore/care viewer.",
                command: "igcare",
                hint: "`igcare` is the read-only view of both lists.",
                also_try: "help igcare",
                check: Check::TypedCmd("igcare"),
            },
        ],
    },
    Lesson {
        key: "arc",
        title: "Archives",
        summary: "create, list and extract an archive",
        steps: &[
            LessonStep {
                goal: "Pack `task.txt` into `pack.zip`.",
                command: "arc create pack.zip task.txt",
                hint: "`arc create` writes a zip/tar.gz from the files you name.",
                also_try: "viewf .",
                check: Check::Typed {
                    cmd: "arc",
                    contains: "pack.zip",
                },
            },
            LessonStep {
                goal: "Look inside the archive without extracting it.",
                command: "arc list pack.zip",
                hint: "`arc list` prints the entries and sizes.",
                also_try: "help arc",
                check: Check::Typed {
                    cmd: "arc",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Extract the archive into the folder `out`.",
                command: "arc extract pack.zip out",
                hint: "`arc extract <archive> [dest]` unpacks it.",
                also_try: "viewd .",
                check: Check::Typed {
                    cmd: "arc",
                    contains: "extract",
                },
            },
        ],
    },
    Lesson {
        key: "scan",
        title: "Scanning",
        summary: "scan by extension and by name",
        steps: &[
            LessonStep {
                goal: "List every .txt file in this folder.",
                command: "scan ext txt",
                hint: "`scan ext` filters by extension.",
                also_try: "help scan",
                check: Check::Typed {
                    cmd: "scan",
                    contains: "ext",
                },
            },
            LessonStep {
                goal: "List every file name here.",
                command: "scan files *",
                hint: "`*` means every name.",
                also_try: "help scan",
                check: Check::Typed {
                    cmd: "scan",
                    contains: "files",
                },
            },
        ],
    },
    Lesson {
        key: "ntpkg",
        title: "Runtimes",
        summary: "ntpkg list / help and ntpx",
        steps: &[
            LessonStep {
                goal: "List the runtimes installed through ntc.",
                command: "ntpkg list",
                hint: "`ntpkg list` shows what is installed and where.",
                also_try: "help ntpkg",
                check: Check::Typed {
                    cmd: "ntpkg",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Read the ntpkg help for install and ntpx.",
                command: "ntpkg help",
                hint: "`ntpx` runs a runtime without changing your PATH.",
                also_try: "view task.txt",
                check: Check::Typed {
                    cmd: "ntpkg",
                    contains: "help",
                },
            },
        ],
    },
    Lesson {
        key: "qr",
        title: "QR codes",
        summary: "print a QR code for text or a URL",
        steps: &[LessonStep {
            goal: "Print a QR code for the word `hello`.",
            command: "qr hello",
            hint: "`qr <text>` renders it in the terminal; scan it with a phone.",
            also_try: "help qr",
            check: Check::Typed {
                cmd: "qr",
                contains: "hello",
            },
        }],
    },
    Lesson {
        key: "math",
        title: "Math",
        summary: "evaluate expressions and manage functions",
        steps: &[
            LessonStep {
                goal: "Evaluate `3+4*5`.",
                command: "math 3+4*5",
                hint: "`math` parses expressions with the usual precedence.",
                also_try: "help math",
                check: Check::Typed {
                    cmd: "math",
                    contains: "3+4",
                },
            },
            LessonStep {
                goal: "List the custom math functions.",
                command: "math fun ls",
                hint: "`math fun add` defines your own functions.",
                also_try: "help math",
                check: Check::Typed {
                    cmd: "math",
                    contains: "fun",
                },
            },
        ],
    },
    Lesson {
        key: "tui",
        title: "Full-screen tools",
        summary: "sys, nfe, wi, ndb, nsheet",
        steps: &[
            LessonStep {
                goal: "Open the live system monitor, then leave it.",
                command: "sys",
                hint: "Inside the monitor, `q` returns to the shell.",
                also_try: "help syscmd",
                check: Check::TypedCmd("sys"),
            },
            LessonStep {
                goal: "Browse this folder in the file explorer, then quit it.",
                command: "nfe",
                hint: "In the explorer `q` quits and `f` searches files.",
                also_try: "help explorer",
                check: Check::TypedCmd("nfe"),
            },
            LessonStep {
                goal: "Watch this folder for changes, then stop watching.",
                command: "wi .",
                hint: "`wi` repaints when files change; `q` leaves.",
                also_try: "help other",
                check: Check::Typed {
                    cmd: "wi",
                    contains: ".",
                },
            },
            LessonStep {
                goal: "Open the database shell, then leave it.",
                command: "ndb",
                hint: "There is no .db here; `exit` closes ndb.",
                also_try: "help ndb",
                check: Check::TypedCmd("ndb"),
            },
            LessonStep {
                goal: "Print the CSV sheet here as a table.",
                command: "nsheet view data.csv",
                hint: "`nsheet view` prints rows; `--sort` and `--filter` refine it.",
                also_try: "help nsheet",
                check: Check::Typed {
                    cmd: "nsheet",
                    contains: "data.csv",
                },
            },
            LessonStep {
                goal: "Browse the same sheet fullscreen, then quit.",
                command: "nsheet open data.csv",
                hint: "`q` leaves the sheet; `/` searches rows.",
                also_try: "help nsheet",
                check: Check::Typed {
                    cmd: "nsheet",
                    contains: "open",
                },
            },
        ],
    },
    Lesson {
        key: "net",
        title: "Network tools",
        summary: "http, dl, up, dns, smtp, serve",
        steps: &[
            LessonStep {
                goal: "Read the HTTP client usage.",
                command: "http --help",
                hint: "`http <method> <url>` sends requests with headers and bodies.",
                also_try: "help network",
                check: Check::TypedCmd("http"),
            },
            LessonStep {
                goal: "Read the downloader usage.",
                command: "dl --help",
                hint: "`dl <url>` downloads with resume and history.",
                also_try: "help network",
                check: Check::TypedCmd("dl"),
            },
            LessonStep {
                goal: "Read the upload usage.",
                command: "up --help",
                hint: "`up <file> <url>` posts a file.",
                also_try: "help network",
                check: Check::TypedCmd("up"),
            },
            LessonStep {
                goal: "Read the DNS lookup usage.",
                command: "dns --help",
                hint: "`dns <host> -t MX` asks for specific record types.",
                also_try: "help network",
                check: Check::TypedCmd("dns"),
            },
            LessonStep {
                goal: "Read the SMTP mail usage.",
                command: "smtp --help",
                hint: "Passwords come from the `deadlock` vault.",
                also_try: "help network",
                check: Check::TypedCmd("smtp"),
            },
            LessonStep {
                goal: "Read the serve usage (file server, HTTP API, chess).",
                command: "serve --help",
                hint: "`serve files` shares the current folder over HTTP.",
                also_try: "help network",
                check: Check::TypedCmd("serve"),
            },
        ],
    },
    Lesson {
        key: "netdiag",
        title: "Connectivity",
        summary: "ping, trace, ipcfg",
        steps: &[
            LessonStep {
                goal: "Ping your own machine.",
                command: "ping 127.0.0.1",
                hint: "Loopback always answers, so this is safe to try.",
                also_try: "help network",
                check: Check::Typed {
                    cmd: "ping",
                    contains: "127.0.0.1",
                },
            },
            LessonStep {
                goal: "Trace the route to your own machine.",
                command: "trace 127.0.0.1",
                hint: "`trace` walks the hops one at a time.",
                also_try: "help network",
                check: Check::Typed {
                    cmd: "trace",
                    contains: "127.0.0.1",
                },
            },
            LessonStep {
                goal: "Show this machine's network adapters.",
                command: "ipcfg",
                hint: "`ipcfg` is the ntc ipconfig / ifconfig.",
                also_try: "help network",
                check: Check::TypedCmd("ipcfg"),
            },
        ],
    },
    Lesson {
        key: "system",
        title: "Machine info",
        summary: "spec, tmprt, qsc, clip",
        steps: &[
            LessonStep {
                goal: "Print this computer's hardware summary.",
                command: "spec",
                hint: "`spec` lists CPU, GPU, RAM, disks and OS.",
                also_try: "help network",
                check: Check::TypedCmd("spec"),
            },
            LessonStep {
                goal: "Show the default city used by the weather report.",
                command: "tmprt city",
                hint: "`tmprt com` reads sensors, `tmprt weather` the forecast.",
                also_try: "help tmprt",
                check: Check::Typed {
                    cmd: "tmprt",
                    contains: "city",
                },
            },
            LessonStep {
                goal: "Check whether the global hotkey daemon is running.",
                command: "qsc status",
                hint: "`qsc listen` starts it, `qsc stop` stops it.",
                also_try: "help qsc",
                check: Check::Typed {
                    cmd: "qsc",
                    contains: "status",
                },
            },
            LessonStep {
                goal: "Show the clipboard slots.",
                command: "clip",
                hint: "`clip 2` recalls slot 2, `clip clear` empties them.",
                also_try: "help clipboard",
                check: Check::TypedCmd("clip"),
            },
        ],
    },
    Lesson {
        key: "shellcfg",
        title: "Shell & PATH",
        summary: "autoup, usrpath, syspath, mode, ui, version, say, pathcp",
        steps: &[
            LessonStep {
                goal: "List the programs that start with this machine.",
                command: "autoup list",
                hint: "`autoup add/rm` edits that list (Windows).",
                also_try: "help autoup",
                check: Check::Typed {
                    cmd: "autoup",
                    contains: "list",
                },
            },
            LessonStep {
                goal: "Show the user PATH entries.",
                command: "usrpath show",
                hint: "`usrpath add <dir>` appends a folder.",
                also_try: "help pathman",
                check: Check::Typed {
                    cmd: "usrpath",
                    contains: "show",
                },
            },
            LessonStep {
                goal: "Show the system PATH entries.",
                command: "syspath show",
                hint: "`syspath` edits the machine-wide PATH (Windows).",
                also_try: "help pathman",
                check: Check::Typed {
                    cmd: "syspath",
                    contains: "show",
                },
            },
            LessonStep {
                goal: "Print the current admin / normal mode.",
                command: "mode",
                hint: "`mode admin` relaunches elevated; `mode normal` drops it.",
                also_try: "help mode",
                check: Check::TypedCmd("mode"),
            },
            LessonStep {
                goal: "Show the UI mode.",
                command: "ui",
                hint: "`ui` reports the UI mode; `version` prints the build.",
                also_try: "help other",
                check: Check::TypedCmd("ui"),
            },
            LessonStep {
                goal: "Print the ntc version.",
                command: "version",
                hint: "`version` prints the release you are running.",
                also_try: "help other",
                check: Check::TypedCmd("version"),
            },
            LessonStep {
                goal: "Print a line of text from the shell.",
                command: "say hello",
                hint: "`say <text>` prints; `print` is the same command.",
                also_try: "help other",
                check: Check::Typed {
                    cmd: "say",
                    contains: "hello",
                },
            },
            LessonStep {
                goal: "Copy the current path to the clipboard.",
                command: "pathcp",
                hint: "`pathcp` copies the folder you are in.",
                also_try: "help clipboard",
                check: Check::TypedCmd("pathcp"),
            },
        ],
    },
    Lesson {
        key: "python",
        title: "Python env",
        summary: "showenv, py_activate, py_deactivate",
        steps: &[
            LessonStep {
                goal: "Show the session environment variables.",
                command: "showenv",
                hint: "`showenv --all` adds the OS ones (secrets masked).",
                also_try: "help python",
                check: Check::TypedCmd("showenv"),
            },
            LessonStep {
                goal: "Turn the current virtualenv off (nothing is active here).",
                command: "py_deactivate",
                hint: "`py_activate` is the one that turns a venv on.",
                also_try: "help python",
                check: Check::TypedCmd("py_deactivate"),
            },
        ],
    },
    Lesson {
        key: "encode",
        title: "Encode & decode",
        summary: "encode, decode, cvcode",
        steps: &[
            LessonStep {
                goal: "Base64-encode the word `hello`.",
                command: "encode b64 \"hello\"",
                hint: "`encode` also does b2, b10, hex and url.",
                also_try: "help encdec",
                check: Check::Typed {
                    cmd: "encode",
                    contains: "b64",
                },
            },
            LessonStep {
                goal: "Decode `aGVsbG8=` back to text.",
                command: "decode b64 \"aGVsbG8=\"",
                hint: "`decode` is the inverse of `encode`.",
                also_try: "help encdec",
                check: Check::Typed {
                    cmd: "decode",
                    contains: "aGVsbG8=",
                },
            },
            LessonStep {
                goal: "Convert that base64 value to hex directly.",
                command: "cvcode b64 \"aGVsbG8=\" to hex",
                hint: "`cvcode <from> <data> to <to>` skips the two steps.",
                also_try: "help encdec",
                check: Check::Typed {
                    cmd: "cvcode",
                    contains: "hex",
                },
            },
        ],
    },
    Lesson {
        key: "fmt",
        title: "Format conversion",
        summary: "fmt between data formats",
        steps: &[LessonStep {
            goal: "Convert `data.toml` here to JSON.",
            command: "fmt data.toml to json",
            hint: "`fmt <file> to <fmt>` handles json, yaml, toml, csv, md and more.",
            also_try: "view data.toml",
            check: Check::Typed {
                cmd: "fmt",
                contains: "json",
            },
        }],
    },
    Lesson {
        key: "image",
        title: "Images",
        summary: "show images inline in the terminal",
        steps: &[LessonStep {
            goal: "Show an image inside the terminal.",
            command: "image --help",
            hint: "`image <path>` or `image display <path>` renders inline; `--sixel` forces sixel.",
            also_try: "help image",
            check: Check::TypedCmd("image"),
        }],
    },
    Lesson {
        key: "ssh",
        title: "SSH",
        summary: "nssh profiles, shell, copy, scan",
        steps: &[LessonStep {
            goal: "List the saved SSH profiles.",
            command: "nssh ls",
            hint: "`nssh add` saves one; `nssh ssh <name>` uses it.",
            also_try: "help nssh",
            check: Check::Typed {
                cmd: "nssh",
                contains: "ls",
            },
        }],
    },
    Lesson {
        key: "nms",
        title: "Music & media",
        summary: "nms scan and play",
        steps: &[LessonStep {
            goal: "Scan this folder for playable media.",
            command: "nms scan .",
            hint: "`nms play <file>` plays with a countdown; Ctrl+C stops, `-sf` starts later.",
            also_try: "help nms",
            check: Check::Typed {
                cmd: "nms",
                contains: "scan",
            },
        }],
    },
    Lesson {
        key: "fake",
        title: "Fake data",
        summary: "generate test rows",
        steps: &[
            LessonStep {
                goal: "List every data type `fake` knows.",
                command: "fake types",
                hint: "`fake types -s <pattern>` searches the list.",
                also_try: "help fake",
                check: Check::Typed {
                    cmd: "fake",
                    contains: "types",
                },
            },
            LessonStep {
                goal: "Generate 3 rows with a name and an email.",
                command: "fake gen name,email -n 3",
                hint: "`-f json` changes the output format.",
                also_try: "help fake",
                check: Check::Typed {
                    cmd: "fake",
                    contains: "gen",
                },
            },
        ],
    },
    Lesson {
        key: "vault",
        title: "Password vault",
        summary: "deadlock keeps pass-env secrets",
        steps: &[LessonStep {
            goal: "Open the encrypted vault, then close it.",
            command: "deadlock",
            hint: "First run asks for a master password; `exit` leaves the vault.",
            also_try: "help deadlock",
            check: Check::TypedCmd("deadlock"),
        }],
    },
    Lesson {
        key: "git",
        title: "Git",
        summary: "status, branch, log, diff through ntc",
        steps: &[LessonStep {
            goal: "Ask git for the status of this folder.",
            command: "git status",
            hint: "Outside a repository it simply says so; any git command passes through.",
            also_try: "help git",
            check: Check::Typed {
                cmd: "git",
                contains: "status",
            },
        }],
    },
    Lesson {
        key: "todo",
        title: "Todo list",
        summary: "add, show and complete tasks",
        steps: &[
            LessonStep {
                goal: "Add the task `first task` to this folder's list.",
                command: "todo add \"first task\"",
                hint: "`todo` writes future_todo.<fmt> in the current folder.",
                also_try: "view future_todo.md",
                check: Check::Typed {
                    cmd: "todo",
                    contains: "add",
                },
            },
            LessonStep {
                goal: "Show the list.",
                command: "todo",
                hint: "Bare `todo` prints every task with its number.",
                also_try: "help todo",
                check: Check::TypedCmd("todo"),
            },
            LessonStep {
                goal: "Mark task 1 as done.",
                command: "todo done 1",
                hint: "`todo undo 1` reopens it.",
                also_try: "help todo",
                check: Check::Typed {
                    cmd: "todo",
                    contains: "done",
                },
            },
        ],
    },
    Lesson {
        key: "time",
        title: "Timing",
        summary: "time run / bench / compare / count",
        steps: &[LessonStep {
            goal: "Benchmark `say hi` over 3 runs.",
            command: "time bench --runs 3 say hi",
            hint: "`time run <cmd>` times one run, `time compare` two commands.",
            also_try: "help time",
            check: Check::Typed {
                cmd: "time",
                contains: "bench",
            },
        }],
    },
];

pub(crate) fn lesson_by_token(token: &str) -> Option<usize> {
    let token = token.trim().to_lowercase();
    if token.is_empty() {
        return None;
    }
    if let Ok(n) = token.parse::<usize>() {
        if n >= 1 && n <= LESSONS.len() {
            return Some(n - 1);
        }
    }
    LESSONS.iter().position(|l| l.key == token)
}

pub(crate) fn lesson_keys() -> String {
    LESSONS.iter().map(|l| l.key).collect::<Vec<_>>().join(", ")
}

/// `<config>/ntc/tutorial`, honoring `NTC_CONFIG_HOME`.
pub(crate) fn tutorial_root() -> PathBuf {
    crate::utils::paths::config_home()
        .join("ntc")
        .join("tutorial")
}

const README: &str = "\
NTC tutorial sandbox
====================

This folder is a disposable playground. Each lesson has its own lab folder
and its own task sheet (task.txt):

  tutorial list            see every lesson and its key
  tutorial <lesson>        start that lesson (key or number)
  tutorial hint            one nudge for the current step
  tutorial status          which lesson and step you are on
  tutorial skip            move past a step
  tutorial restart         start the current lesson again
  tutorial exit            leave tutorial mode

Nothing here is precious — delete the folder at any time to reset it.
";

const SAMPLE: &str = "\
You are inside the inner folder.
This file only exists for the cd lesson.
";

const NOTES: &str = "\
Tutorial notes
==============

- ntc can show text files without an editor
- view also renders folders as a tree
- add -d 2 to look two levels deep
";

const VIEW_TODO: &str = "# Todo\n\n- try `view notes.txt`\n- try `view -d 2 ..`\n";

const SHEET: &str = "name,age,city\nada,36,london\ngrace,45,new york\nlin,52,taipei\n";

const DATA_TOML: &str = "[app]\nname = \"ntc\"\nport = 8080\n";

const TODO_FILE: &str = "- [ ] learn ntc\n";

const RANFILE: &str = "\
# Tutorial task runner file (used by the `ran` lesson).

[targets.standalone]
cmd = \"echo standalone target\"

[targets.hello]
cmd = \"echo hello from ran\"

[targets.build]
cmd = \"echo pretending to build...\"
";

fn write(path: &Path, body: &str) -> Result<()> {
    fs::write(path, body)?;
    Ok(())
}

fn remove_path(path: &Path) {
    if path.is_dir() {
        let _ = fs::remove_dir_all(path);
    } else if path.exists() {
        let _ = fs::remove_file(path);
    }
}

/// Create (or refresh) the sandbox in its real config location.
pub(crate) fn ensure_workspace() -> Result<PathBuf> {
    let root = tutorial_root();
    ensure_workspace_at(&root)?;
    Ok(root)
}

/// [`ensure_workspace`] against an explicit folder (also used by tests).
pub(crate) fn ensure_workspace_at(root: &Path) -> Result<()> {
    // One lab folder per lesson, plus the nested folder the cd lesson walks into.
    for lesson in LESSONS {
        fs::create_dir_all(root.join(lesson.key))?;
    }
    fs::create_dir_all(root.join("cd").join("inner"))?;

    write(&root.join("README.md"), README)?;

    write(&root.join("cd").join("inner").join("sample.txt"), SAMPLE)?;
    for key in ["view", "views", "text", "search"] {
        write(&root.join(key).join("notes.txt"), NOTES)?;
    }
    write(&root.join("view").join("todo.md"), VIEW_TODO)?;
    write(&root.join("ran").join("NTCRANFILE.toml"), RANFILE)?;
    write(&root.join("tui").join("data.csv"), SHEET)?;
    write(&root.join("fmt").join("data.toml"), DATA_TOML)?;
    write(&root.join("todo").join("future_todo.md"), TODO_FILE)?;

    // One generated task sheet per lesson: each goal plus the command to type.
    for lesson in LESSONS {
        let mut body = format!(
            "{}\n{}\n\n{}\n\n",
            lesson.title,
            "=".repeat(lesson.title.len()),
            lesson.summary
        );
        for (i, step) in lesson.steps.iter().enumerate() {
            body.push_str(&format!(
                "{}. {}\n   command: {}\n",
                i + 1,
                step.goal,
                step.command
            ));
        }
        body.push_str("\nStuck? Run `tutorial hint` in the ntc shell.\n");
        write(&root.join(lesson.key).join("task.txt"), &body)?;
    }

    Ok(())
}

/// Reset a lesson's fixtures so a re-run behaves like the first time.
pub(crate) fn reset_lesson(root: &Path, index: usize) -> Result<()> {
    let Some(lesson) = LESSONS.get(index) else {
        return Ok(());
    };
    match lesson.key {
        "cd" => {
            fs::create_dir_all(root.join("cd").join("inner"))?;
            write(&root.join("cd").join("inner").join("sample.txt"), SAMPLE)?;
        }
        "view" | "views" | "text" | "search" => {
            fs::create_dir_all(root.join(lesson.key))?;
            write(&root.join(lesson.key).join("notes.txt"), NOTES)?;
        }
        "ne" => {
            // Only ever touches the lesson's own file.
            remove_path(&root.join("ne").join("draft.md"));
        }
        "ran" => {
            fs::create_dir_all(root.join("ran"))?;
            write(&root.join("ran").join("NTCRANFILE.toml"), RANFILE)?;
        }
        "files" => {
            for name in [
                "note.txt",
                "copy.txt",
                "renamed.txt",
                "scratch",
                "tmp",
                "final.txt",
            ] {
                remove_path(&root.join("files").join(name));
            }
        }
        "arc" => {
            remove_path(&root.join("arc").join("pack.zip"));
            remove_path(&root.join("arc").join("out"));
        }
        "config" | "project" => {
            remove_path(&root.join(lesson.key).join("ntconfig.toml"));
            remove_path(&root.join(lesson.key).join("NTCRANFILE.toml"));
        }
        "scr" => {
            remove_path(&root.join("scr").join("demo.ntc"));
        }
        "tui" => {
            write(&root.join("tui").join("data.csv"), SHEET)?;
        }
        "fmt" => {
            write(&root.join("fmt").join("data.toml"), DATA_TOML)?;
        }
        "todo" => {
            write(&root.join("todo").join("future_todo.md"), TODO_FILE)?;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_lesson_has_steps_and_a_unique_key() {
        let mut keys = std::collections::HashSet::new();
        for lesson in LESSONS {
            assert!(!lesson.key.is_empty());
            assert!(!lesson.steps.is_empty());
            assert!(keys.insert(lesson.key), "duplicate key {}", lesson.key);
        }
    }

    /// The tutorial shows each command up front, so every step must carry a
    /// runnable one-liner and a separate hint (never the same text).
    #[test]
    fn every_step_shows_a_command_and_a_distinct_hint() {
        for lesson in LESSONS {
            for (i, step) in lesson.steps.iter().enumerate() {
                assert!(
                    !step.command.trim().is_empty(),
                    "{} step {}: empty command",
                    lesson.key,
                    i + 1
                );
                assert!(
                    !step.command.contains('\n'),
                    "{} step {}: command spans lines",
                    lesson.key,
                    i + 1
                );
                assert_ne!(
                    step.command,
                    step.hint,
                    "{} step {}: hint repeats the command",
                    lesson.key,
                    i + 1
                );
            }
        }
    }

    #[test]
    fn workspace_gets_every_lesson_folder_and_task_sheet() {
        // Absolute so a concurrently running test cannot break the path by
        // changing the process working directory.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("tutorial_fixture_test")
            .join("tutorial");
        let _ = fs::remove_dir_all(&root);
        ensure_workspace_at(&root).expect("workspace");
        assert!(root.join("README.md").is_file());
        assert!(root.join("cd/inner/sample.txt").is_file());
        assert!(root.join("view/notes.txt").is_file());
        assert!(root.join("ran/NTCRANFILE.toml").is_file());
        assert!(root.join("tui/data.csv").is_file());
        assert!(root.join("fmt/data.toml").is_file());
        assert!(root.join("todo/future_todo.md").is_file());
        for lesson in LESSONS {
            assert!(
                root.join(lesson.key).join("task.txt").is_file(),
                "{} missing task.txt",
                lesson.key
            );
        }
        let _ = fs::remove_dir_all(root.parent().unwrap());
    }

    #[test]
    fn lessons_can_be_picked_by_key_or_number() {
        assert_eq!(lesson_by_token("cd"), Some(0));
        assert_eq!(lesson_by_token("1"), Some(0));
        assert_eq!(lesson_by_token("RAL"), Some(3));
        assert_eq!(lesson_by_token("99"), None);
        assert_eq!(lesson_by_token("nope"), None);
    }
}
