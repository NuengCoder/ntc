use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nprintf.title".into(), r#"NPRINTF COMMAND:"#.into());
    m.insert(
        "help.nprintf.usage".into(),
        r#"nprintf <format> [arguments...] ## Bash-style formatted output"#.into(),
    );
    m.insert("help.nprintf.conversions".into(), r#"  CONVERSIONS"#.into());
    m.insert(
        "help.nprintf.specs".into(),
        r#"  %s text · %d/%i signed · %u unsigned · %o octal · %x/%X hex · %c first character"#
            .into(),
    );
    m.insert(
        "help.nprintf.specs2".into(),
        r#"  %b escapes in the argument · %q shell-quoted · %f/%e/%g/%a floats · %% literal %"#
            .into(),
    );
    m.insert(
        "help.nprintf.flags".into(),
        r#"  FLAGS, WIDTH, PRECISION"#.into(),
    );
    m.insert(
        "help.nprintf.flags_detail".into(),
        r#"  %-8s left · %05d zero-pad · %+d sign · %.2f precision · %*s width from an argument"#
            .into(),
    );
    m.insert(
        "help.nprintf.escapes".into(),
        r#"  ESCAPES (a backslash starts each one)"#.into(),
    );
    m.insert("help.nprintf.escapes_detail".into(), r#"  \n newline · \t tab · \r return · \\ backslash · \" quote · \xHH hex · \0NNN octal · \c stop output"#.into());
    m.insert("help.nprintf.newline".into(), r#"  WHY \n?"#.into());
    m.insert("help.nprintf.newline_detail".into(), r#"  nprintf adds nothing after the format. End with \n when you want a line break; without it, output continues on the same line."#.into());
    m.insert("help.nprintf.quoting".into(), r#"  QUOTING"#.into());
    m.insert(
        "help.nprintf.quoting_detail".into(),
        r#"  Wrap the format in double quotes so spaces and escapes stay in one argument."#.into(),
    );
    m.insert("help.nprintf.reuse".into(), r#"  The format repeats while arguments remain; a missing %s is empty and a missing number is 0."#.into());
    m.insert(
        "help.nprintf.example1".into(),
        r#"nprintf "hello %s\n" world ## One line; \n ends it"#.into(),
    );
    m.insert(
        "help.nprintf.example2".into(),
        r#"nprintf "%s has %d items\n" cart 3 ## Text and integer"#.into(),
    );
    m.insert(
        "help.nprintf.example3".into(),
        r#"nprintf "%-8s|%05d|%.2f\n" left 42 3.14159 ## Left, zero-pad, precision"#.into(),
    );
    m.insert(
        "help.nprintf.example4".into(),
        r#"nprintf "%b\n" "a\tb" ## %b applies escapes from the argument"#.into(),
    );
    m.insert(
        "help.nprintf.example5".into(),
        r#"nprintf "no newline" ## Stays on the same line"#.into(),
    );
}
