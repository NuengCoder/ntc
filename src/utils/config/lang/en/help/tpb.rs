use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tpb.title".into(), "TPB COMMAND:".into());

    m.insert(
        "help.tpb.usage".into(),
        "tpb ## Teleport back to previous location (undo tp)".into(),
    );

    m.insert(
        "help.tpb.history".into(),
        "tpb history ## Show teleport navigation history".into(),
    );

    m.insert(
        "help.tpb.clear".into(),
        "tpb clear ## Clear teleport history".into(),
    );

    m.insert(
        "help.tpb.example1".into(),
        "tpb ## Return to the previous location".into(),
    );
}
