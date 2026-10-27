use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "nfe.binary_file".into(),
        "(binary / unsupported format)".into(),
    );

    m.insert("nfe.binary_file_preview".into(), "(binary file)".into());

    m.insert("nfe.empty_dir".into(), "(empty directory)".into());

    m.insert("nfe.file_truncated".into(), "... (file truncated)".into());

    m.insert(
        "nfe.hint_bar".into(),
        "h help | b back | f find | / filter | q quit".into(),
    );

    m.insert("nfe.no_matches".into(), "(no matches)".into());

    m.insert("nfe.preview_empty".into(), "(empty)".into());

    m.insert("nfe.preview_error".into(), "(cannot read)".into());

    m.insert("nfe.preview_more".into(), "... (more)".into());

    m.insert("nfe.sort_date".into(), "Date".into());

    m.insert("nfe.sort_name".into(), "Name".into());

    m.insert("nfe.sort_size".into(), "Size".into());

    m.insert("nfe.sort_type".into(), "Type".into());

    m.insert(
        "nfe.status_bar".into(),
        "{} items | {} dirs | {} files | Sort: {} {}".into(),
    );
}
