use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.fmt.file_not_found".into(), "文件未找到：{}".into());

    m.insert(
        "cmd.fmt.binary_file".into(),
        "检测到二进制或压缩文件。fmt 只支持文本格式。".into(),
    );

    m.insert(
        "cmd.fmt.cross_family".into(),
        "无法将 '{}' 转换为 '{}'：属于不同的格式家族（{} 对 {}）。".into(),
    );

    m.insert(
        "cmd.fmt.doc_ser_err".into(),
        "以 '{}' 序列化文档失败：{}".into(),
    );

    m.insert(
        "cmd.fmt.docx_needs_output".into(),
        "DOCX 输出需要指定输出文件（-o <文件>）。".into(),
    );

    m.insert(
        "cmd.fmt.pdf_needs_output".into(),
        "PDF 输出需要指定输出文件（-o <文件>）。".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_needs_output".into(),
        "XLSX 输出需要指定输出文件（-o <文件>）。".into(),
    );

    m.insert(
        "cmd.fmt.pdf_source_unsupported".into(),
        "不支持 PDF 输入；fmt 只能输出 PDF（-o <文件.pdf>）。".into(),
    );

    m.insert(
        "cmd.fmt.xlsx_source_unsupported".into(),
        "不支持 XLSX 输入；fmt 只能输出 XLSX（-o <文件.xlsx>）。".into(),
    );

    m.insert(
        "cmd.fmt.pdf_unsupported".into(),
        "当前平台不支持 PDF 输出。".into(),
    );

    m.insert("cmd.fmt.invalid_json".into(), "无效的 JSON：{}".into());

    m.insert("cmd.fmt.invalid_toml".into(), "无效的 TOML：{}".into());

    m.insert("cmd.fmt.invalid_yaml".into(), "无效的 YAML：{}".into());

    m.insert("cmd.fmt.json_ser_err".into(), "JSON 序列化错误：{}".into());

    m.insert("cmd.fmt.missing_file".into(), "缺少文件路径".into());

    m.insert("cmd.fmt.read_fail".into(), "读取文件失败：{}".into());

    m.insert("cmd.fmt.toml_conv_err".into(), "TOML 转换错误：{}".into());

    m.insert("cmd.fmt.toml_ser_err".into(), "TOML 序列化错误：{}".into());

    m.insert("cmd.fmt.unknown_format".into(), "不支持的文件类型：'{}'。fmt 需要以下扩展名的文件：json、yaml、toml、md、html、xml、docx、txt、pdf、csv、tsv 或 xlsx。".into());

    m.insert("cmd.fmt.unknown_target".into(), "未知的目标格式：'{}'。请使用 json、yaml、toml、md、html、xml、docx、txt、pdf、csv、tsv 或 xlsx。".into());

    m.insert(
        "cmd.fmt.legacy_syntax".into(),
        "语法已更改：源格式取自文件扩展名。请使用：fmt <文件> [to <格式>] [-o <输出>]".into(),
    );

    m.insert(
        "cmd.fmt.usage".into(),
        "用法：fmt <文件> [to json|yaml|toml|md|html|xml|docx|txt|pdf|csv|tsv|xlsx] [-o <输出>]"
            .into(),
    );

    m.insert("cmd.fmt.written_to".into(), "已写入：{}".into());

    m.insert("cmd.fmt.yaml_ser_err".into(), "YAML 序列化错误：{}".into());
}
