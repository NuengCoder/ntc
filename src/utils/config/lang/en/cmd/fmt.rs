use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.fmt.file_not_found".into(), "File not found: {}".into());

    m.insert(
        "cmd.fmt.binary_file".into(),
        "Binary or zip file detected. fmt only works on supported text formats.".into(),
    );

    m.insert(
        "cmd.fmt.cross_family".into(),
        "Cannot convert '{}' to '{}': different format families ({} vs {}).".into(),
    );

    m.insert(
        "cmd.fmt.doc_ser_err".into(),
        "Failed to serialize document as '{}': {}".into(),
    );

    m.insert(
        "cmd.fmt.docx_needs_output".into(),
        "DOCX output requires an output file (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_needs_output".into(),
        "PDF output requires an output file (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_needs_output".into(),
        "XLSX output requires an output file (-o <file>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_source_unsupported".into(),
        "PDF input is not supported; fmt can only write PDF output (-o <file.pdf>).".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_source_unsupported".into(),
        "XLSX input is not supported; fmt can only write XLSX output (-o <file.xlsx>).".into(),
    );

    m.insert(
        "cmd.fmt.pdf_unsupported".into(),
        "PDF output is not supported on this platform.".into(),
    );

    m.insert("cmd.fmt.invalid_json".into(), "Invalid JSON: {}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "Invalid TOML: {}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "Invalid YAML: {}".into());

    m.insert(
        "cmd.fmt.json_ser_err".into(),
        "JSON serialization error: {}".into(),
    );

    m.insert("cmd.fmt.missing_file".into(), "Missing file path".into());

    m.insert("cmd.fmt.read_fail".into(), "Failed to read file: {}".into());

    m.insert(
        "cmd.fmt.toml_conv_err".into(),
        "TOML conversion error: {}".into(),
    );

    m.insert(
        "cmd.fmt.toml_ser_err".into(),
        "TOML serialization error: {}".into(),
    );

    m.insert("cmd.fmt.unknown_format".into(), "Unsupported file type '{}' for fmt. Use a file with extension: json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv, or xlsx.".into());

    m.insert("cmd.fmt.unknown_target".into(), "Unknown target format: '{}'. Use json, yaml, toml, md, html, xml, docx, txt, pdf, csv, tsv, or xlsx.".into());

    m.insert("cmd.fmt.legacy_syntax".into(), "Syntax changed: the source format is taken from the file extension. Use: fmt <file> [to <format>] [-o <output>]".into());

    m.insert(
        "cmd.fmt.usage".into(),
        "Usage: fmt <file> [to json|yaml|toml|md|html|xml|docx|txt|pdf|csv|tsv|xlsx] [-o <output>]"
            .into(),
    );

    m.insert("cmd.fmt.written_to".into(), "Written to: {}".into());

    m.insert(
        "cmd.fmt.yaml_ser_err".into(),
        "YAML serialization error: {}".into(),
    );
}
