use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "encode.binary_hint".into(),
        "Use -o <file> to save the binary output.".into(),
    );

    m.insert(
        "encode.binary_warning".into(),
        "Output is binary data. Displaying hex preview:".into(),
    );

    m.insert(
        "encode.hex_even".into(),
        "Hex string length must be even".into(),
    );

    m.insert(
        "encode.input_must_be_text".into(),
        "Input data must be text".into(),
    );

    m.insert(
        "encode.input_not_text".into(),
        "{} input must be text (not binary)".into(),
    );

    m.insert(
        "encode.invalid_b2".into(),
        "Invalid binary string: '{}'".into(),
    );

    m.insert(
        "encode.invalid_b10".into(),
        "Invalid decimal byte: '{}'".into(),
    );

    m.insert(
        "encode.invalid_b64".into(),
        "Invalid base64 input: {}".into(),
    );

    m.insert(
        "encode.invalid_hex_pct".into(),
        "Invalid hex in percent-encoding: %{}".into(),
    );

    m.insert(
        "encode.invalid_hex_pos".into(),
        "Invalid hex character at position {}".into(),
    );

    m.insert(
        "encode.invalid_pct".into(),
        "Invalid percent-encoding".into(),
    );

    m.insert(
        "encode.truncated_pct".into(),
        "Truncated percent-encoding".into(),
    );

    m.insert(
        "encode.unknown_format".into(),
        "Unknown encode format: '{}'. Use b2, b10, b64, hex, or url.".into(),
    );

    m.insert(
        "encode.unknown_format_fmt".into(),
        "(unknown format: {})".into(),
    );

    m.insert(
        "encode.unknown_format_generic".into(),
        "Unknown format: '{}'. Use b2, b10, b64, hex, or url.".into(),
    );

    m.insert(
        "encode.usage_cmd".into(),
        "Usage: encode {} <target> [-o <file>]".into(),
    );

    m.insert(
        "encode.usage_encode_b2".into(),
        " encode b2 <target> [-o <file>] Binary encode (0101...)".into(),
    );

    m.insert(
        "encode.usage_encode_b10".into(),
        " encode b10 <target> [-o <file>] Decimal encode (0-255...)".into(),
    );

    m.insert(
        "encode.usage_encode_b64".into(),
        " encode b64 <target> [-o <file>] Base64 encode".into(),
    );

    m.insert(
        "encode.usage_encode_hex".into(),
        " encode hex <target> [-o <file>] Hex encode".into(),
    );

    m.insert(
        "encode.usage_encode_url".into(),
        " encode url <target> [-o <file>] URL percent-encode".into(),
    );

    m.insert("encode.usage_examples".into(), "Examples:".into());

    m.insert("encode.usage_title".into(), "Usage:".into());

    m.insert("encode.written_to".into(), "Written to: {}".into());
}
