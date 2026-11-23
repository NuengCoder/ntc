use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.rmd.title".into(), "RMD COMMAND:".into());

    m.insert(
        "help.rmd.usage".into(),
        "rmd <dir>[, ...] ## Delete directories; wildcards delete every match after confirmation"
            .into(),
    );

    m.insert(
        "help.rmd.example1".into(),
        "rmd build ## Delete the build directory".into(),
    );
}
