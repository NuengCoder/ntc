use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "tutorial.complete_msg".into(),
        "You now know the essentials of NTC:".into(),
    );

    m.insert(
        "tutorial.complete_title".into(),
        " Tutorial Complete! ".into(),
    );

    m.insert("tutorial.demonstrating".into(), "Demonstrating:".into());

    m.insert(
        "tutorial.exit_hint".into(),
        "Type 'exit' to quit NTC.".into(),
    );

    m.insert(
        "tutorial.help_hint".into(),
        "Type 'help' for the full command reference.".into(),
    );

    m.insert(
        "tutorial.prompt_continue".into(),
        "Press Enter to continue".into(),
    );

    m.insert("tutorial.prompt_exit_label".into(), "exit".into());

    m.insert("tutorial.prompt_skip_label".into(), "skip".into());

    m.insert(
        "tutorial.return_hint".into(),
        "Press Enter to return to the shell".into(),
    );

    m.insert(
        "tutorial.step_title".into(),
        " NTC Tutorial \u{2014} Step {}/{} ".into(),
    );

    m.insert("tutorial.tip_label".into(), "\u{1F4A1} Tip:".into());
}
