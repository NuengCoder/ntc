use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.pipe.title".into(), "PIPES & REDIRECTION:".into());

    m.insert(
        "help.pipe.pipe".into(),
        "a | b ## Send a's output to b as input".into(),
    );

    m.insert(
        "help.pipe.and".into(),
        "a && b ## Run b only if a succeeds".into(),
    );

    m.insert(
        "help.pipe.or".into(),
        "a || b ## Run b only if a fails".into(),
    );

    m.insert(
        "help.pipe.out".into(),
        "cmd > file ## Write output to file (overwrite)".into(),
    );

    m.insert(
        "help.pipe.append".into(),
        "cmd >> file ## Append output to file".into(),
    );

    m.insert(
        "help.pipe.in".into(),
        "cmd < file ## Read input from file".into(),
    );

    m.insert(
        "help.pipe.heredoc".into(),
        "cmd <<EOF ... EOF ## Read input until the EOF line".into(),
    );

    m.insert(
        "help.pipe.herestring".into(),
        "cmd <<< text ## Read input from the literal text token".into(),
    );

    m.insert(
        "help.pipe.head".into(),
        "head [N] [file] ## Print the first N lines (default 10)".into(),
    );

    m.insert(
        "help.pipe.tail".into(),
        "tail [N] [file] ## Print the last N lines (default 10)".into(),
    );

    m.insert(
        "help.pipe.betw".into(),
        "betw A:B [file] ## Print lines A to B (1-based inclusive)".into(),
    );

    m.insert(
        "help.pipe.sort".into(),
        "sort [-r] [-n] [-u] [-f] [file] ## Sort lines (reverse/numeric/unique/ignore-case)".into(),
    );

    m.insert(
        "help.pipe.rev".into(),
        "rev [file] ## Reverse the characters of each line".into(),
    );

    m.insert(
        "help.pipe.count".into(),
        "count <needle> [text] ## Count how many times <needle> occurs (text or piped input)"
            .into(),
    );

    m.insert(
        "help.pipe.trim".into(),
        "trim <pattern> [text] ## Remove every occurrence of <pattern> (text or piped input)"
            .into(),
    );

    m.insert("help.pipe.filters".into(), "cmd | head|tail|betw|gs|sort|rev|count|trim|encode|qur|clip ## Pipe filters (their flags complete too)".into());

    m.insert(
        "help.pipe.note".into(),
        "Operators need spaces around them; use - to read stdin".into(),
    );

    m.insert(
        "help.pipe.hint".into(),
        "Hints: | lists the pipe filters; >> and < list text files only; > shows none; << only EOF"
            .into(),
    );

    m.insert(
        "help.pipe.example1".into(),
        "gs main > results.txt ## Save search results to a file".into(),
    );

    m.insert(
        "help.pipe.example2".into(),
        "view readme.md | encode b64 ## Encode a file's contents".into(),
    );

    m.insert(
        "help.pipe.example3".into(),
        "gs main | say ## Page-search results through say".into(),
    );

    m.insert(
        "help.pipe.example4".into(),
        "head 20 app.log ## First 20 lines of a file (or piped input)".into(),
    );

    m.insert(
        "help.pipe.example5".into(),
        "gs error | sort -u | head 10 ## Unique sorted matches, first 10".into(),
    );

    m.insert("help.pipe.example6".into(), "echo nice ... bro ... | trim \" ...\" ## Strip a pattern from piped text (inline: trim \" ...\" \"nice ... bro ...\")".into());
}
