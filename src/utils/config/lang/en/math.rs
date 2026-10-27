use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "math.builtin_agg".into(),
        "  agg:  sum, min, max, avg".into(),
    );

    m.insert(
        "math.builtin_constants".into(),
        "  constants: PI, E, PHI, TAU".into(),
    );

    m.insert(
        "math.builtin_conv".into(),
        "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into(),
    );

    m.insert("math.builtin_header".into(), "Built-in functions:".into());

    m.insert(
        "math.builtin_inv_trig".into(),
        "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc"
            .into(),
    );

    m.insert(
        "math.builtin_math".into(),
        "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into(),
    );

    m.insert(
        "math.builtin_print".into(),
        "  print: print values (like Python)".into(),
    );

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert(
        "math.builtin_trig".into(),
        "  trig: sin, cos, tan, cot, sec, csc".into(),
    );

    m.insert("math.compiled_header".into(), "Compiled: {}".into());

    m.insert("math.countdown".into(), "Countdown: {} seconds".into());

    m.insert("math.error".into(), "  {} at line {}: {}".into());

    m.insert(
        "math.example_add".into(),
        "  math 3+4*5                           → 23".into(),
    );

    m.insert(
        "math.example_fun_call".into(),
        "  math square(5)                        → 25".into(),
    );

    m.insert(
        "math.example_fun_def".into(),
        "  math fun add square(x) = x^2".into(),
    );

    m.insert(
        "math.example_hexbin".into(),
        "  math 0xFF + 0b1010                   → 265".into(),
    );

    m.insert(
        "math.example_rand".into(),
        "  math rand(1,100)                      → 42".into(),
    );

    m.insert(
        "math.example_sin".into(),
        "  math sin(PI/2)                       → 1".into(),
    );

    m.insert(
        "math.example_sqrt".into(),
        "  math sqrt(144)                        → 12".into(),
    );

    m.insert(
        "math.example_timer".into(),
        "  math timer 10                        → countdown 10s".into(),
    );

    m.insert(
        "math.example_tobinary".into(),
        "  math toBinary(255)                    → 11111111".into(),
    );

    m.insert(
        "math.example_tohb".into(),
        "  math toHB(1048576)                    → 1.00 MB".into(),
    );

    m.insert(
        "math.example_tohex".into(),
        "  math toHex(255)                       → FF".into(),
    );

    m.insert("math.examples_header".into(), "Examples:".into());

    m.insert(
        "math.exec_end".into(),
        "\u{255A}\u{2550} End: {} \u{2550}\u{255D}".into(),
    );

    m.insert("math.failed_read".into(), "Failed to read '{}': {}".into());

    m.insert("math.file_not_found".into(), "File not found: '{}'".into());

    m.insert(
        "math.fun_commands_header".into(),
        "math fun commands:".into(),
    );

    m.insert(
        "math.fun_list_header".into(),
        "User-defined math functions:".into(),
    );

    m.insert(
        "math.fun_none".into(),
        " No user-defined functions. Use 'math fun add <name>(<params>) = <body>'".into(),
    );

    m.insert("math.fun_removed".into(), " Removed: {}".into());

    m.insert(
        "math.help_expr".into(),
        "  math <expression>       Evaluate a math expression".into(),
    );

    m.insert(
        "math.help_file".into(),
        "  math <file>.ntc.math    Compile and run a math file".into(),
    );

    m.insert(
        "math.help_fun_add".into(),
        "  math fun add <name>(<params>) = <body>   Define a function".into(),
    );

    m.insert(
        "math.help_fun_edit".into(),
        "  math fun edit <name> = <body>  Update a function".into(),
    );

    m.insert(
        "math.help_fun_info".into(),
        "  math fun info <name>    Show function details".into(),
    );

    m.insert(
        "math.help_fun_ls".into(),
        "  math fun ls             List all user-defined functions".into(),
    );

    m.insert(
        "math.help_fun_rm".into(),
        "  math fun rm <name>      Remove a function".into(),
    );

    m.insert(
        "math.help_func_call".into(),
        "  math <func>(<args>)     Call a built-in or user-defined function".into(),
    );

    m.insert(
        "math.help_timer".into(),
        "  math timer [seconds]    Lap timer or countdown with alarm".into(),
    );

    m.insert("math.help_title".into(), "Math Module - Usage:".into());

    m.insert(
        "math.is_directory".into(),
        "'{}' is a directory, not a file".into(),
    );

    m.insert("math.lap_time".into(), "Lap: {} seconds".into());

    m.insert("math.time_remaining".into(), "Time remaining: {}".into());

    m.insert("math.timer_started".into(), "Timer started...".into());

    m.insert(
        "math.timer_usage".into(),
        "Usage: math timer <seconds>".into(),
    );

    m.insert("math.times_up".into(), "Time's up!".into());
}
