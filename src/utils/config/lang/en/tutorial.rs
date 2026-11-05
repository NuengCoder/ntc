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

    m.insert("tutorial.tip_label".into(), "Tip:".into());

    // ── hands-on tutorial shell ─────────────────────────────────────────
    m.insert("tutorial.ws_ready".into(), "Tutorial workspace: {}".into());

    m.insert(
        "tutorial.mode_on".into(),
        "Tutorial mode is ON \u{2014} you type the commands; ntc only checks the goal.".into(),
    );

    m.insert(
        "tutorial.menu_title".into(),
        "NTC TUTORIAL \u{2014} CHOOSE A LESSON".into(),
    );

    m.insert(
        "tutorial.menu_goal".into(),
        "Type `tutorial <lesson>` once, then do the goal yourself.".into(),
    );

    m.insert(
        "tutorial.menu_hint".into(),
        "Stuck: tutorial hint \u{b7} skip: tutorial skip \u{b7} status: tutorial status \u{b7} leave: tutorial exit"
            .into(),
    );

    m.insert("tutorial.step_label".into(), "Step {}/{}".into());

    m.insert("tutorial.goal_label".into(), "Goal:".into());

    m.insert("tutorial.hint_label".into(), "Hint:".into());

    m.insert("tutorial.command_label".into(), "Command:".into());

    m.insert(
        "tutorial.command_usage".into(),
        "Type it yourself and press Enter \u{2014} ntc will not run it for you.".into(),
    );

    m.insert("tutorial.step_done".into(), "Step complete.".into());

    m.insert("tutorial.also_try_fmt".into(), "Also try: {}".into());

    m.insert(
        "tutorial.lesson_done".into(),
        "Lesson complete: {} \u{2014} pick the next one with `tutorial`.".into(),
    );

    m.insert(
        "tutorial.not_active".into(),
        "Tutorial mode is not active. Run `tutorial` to start.".into(),
    );

    m.insert(
        "tutorial.not_in_lesson".into(),
        "No lesson selected. Pick one with `tutorial <lesson>`.".into(),
    );

    m.insert(
        "tutorial.unknown_lesson".into(),
        "Unknown lesson '{}'. Available: {}".into(),
    );

    m.insert(
        "tutorial.blocked_cmd".into(),
        "`{}` is blocked in tutorial mode. Use one of the lesson commands or `tutorial`.".into(),
    );

    m.insert(
        "tutorial.blocked_outside".into(),
        "You are outside the tutorial folder. `cd {}` to return, or `tutorial exit` to leave tutorial mode."
            .into(),
    );

    m.insert(
        "tutorial.mode_off".into(),
        "Tutorial mode off. Back to {}.".into(),
    );

    m.insert(
        "tutorial.already_done".into(),
        "This lesson is complete. `tutorial` for the menu, or `tutorial restart` to repeat it."
            .into(),
    );

    m.insert("tutorial.skipped".into(), "Skipped.".into());

    m.insert(
        "tutorial.status_fmt".into(),
        "Lesson {} \u{b7} step {}/{}".into(),
    );

    m.insert("tutorial.prompt_fmt".into(), "tutorial [{} {}/{}] ".into());

    m.insert("tutorial.prompt_none".into(), "tutorial ".into());

    m.insert(
        "tutorial.usage".into(),
        "USAGE:\n  tutorial                 Start the tutorial sandbox and show the lesson menu\n  tutorial <lesson>        Start a lesson (key or number)\n  tutorial list            Show the lesson menu\n  tutorial hint            One nudge for the current step\n  tutorial skip            Skip the current step\n  tutorial restart         Restart the current lesson\n  tutorial status          Current lesson and step\n  tutorial exit            Leave tutorial mode\n\nLessons: run `tutorial list` to see the full menu (every ntc command has one).\nIn tutorial mode only lab commands run; leaving the tutorial folder\nblocks everything except cd, tutorial, help and exit."
            .into(),
    );
}
