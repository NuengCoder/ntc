use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.math.eval".into(),
        "math <expr> ## Evaluate a math expression".into(),
    );

    m.insert(
        "help.math.file".into(),
        "math file.ntc.math ## Compile and run a math file".into(),
    );

    m.insert(
        "help.math.fun".into(),
        "math fun add/rm/edit/info/ls ## Manage custom math functions".into(),
    );

    m.insert(
        "help.math.timer".into(),
        "math timer [sec] ## Lap timer or countdown with alarm".into(),
    );

    m.insert("help.math.title".into(), "MATH COMMANDS:".into());

    m.insert(
        "help.math.example1".into(),
        "math 3+4*5 ## Evaluate arithmetic expression".into(),
    );

    m.insert(
        "help.math.example2".into(),
        "math sin(PI/2) ## Math with functions".into(),
    );

    m.insert(
        "help.math.example3".into(),
        "math timer 30 ## Start 30-second countdown".into(),
    );

    m.insert(
        "help.math.example4".into(),
        "math fun add myfn \"x+1\" ## Add custom function".into(),
    );

    m.insert(
        "help.math.example5".into(),
        "math fun ls ## List custom functions".into(),
    );

    m.insert(
        "help.math.example6".into(),
        "math calc.ntc.math ## Run math file".into(),
    );
}
