use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.count.title".into(), "COUNT COMMAND:".into());

    m.insert(
        "help.count.usage".into(),
        "count <needle> [text] [-l|--line] ## Count <needle> occurrences; -l|--line counts piped lines"
            .into(),
    );

    m.insert(
        "help.count.example1".into(),
        "gs TODO | count TODO ## Count matches in piped text".into(),
    );

    m.insert(
        "help.count.example2".into(),
        "gs TODO | count -l ## Count the total lines of piped output".into(),
    );
}
