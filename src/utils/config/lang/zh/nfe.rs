use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("nfe.binary_file".into(), "（二进制/不支持的格式）".into());

    m.insert("nfe.binary_file_preview".into(), "（二进制文件）".into());

    m.insert("nfe.empty_dir".into(), "（空目录）".into());

    m.insert("nfe.file_truncated".into(), "...（文件已截断）".into());

    m.insert(
        "nfe.hint_bar".into(),
        "h 帮助 | b 返回 | f 查找 | / 过滤 | q 退出".into(),
    );

    m.insert("nfe.no_matches".into(), "（无匹配）".into());

    m.insert("nfe.preview_empty".into(), "（空）".into());

    m.insert("nfe.preview_error".into(), "（无法读取）".into());

    m.insert("nfe.preview_more".into(), "...（更多）".into());

    m.insert("nfe.sort_date".into(), "日期".into());

    m.insert("nfe.sort_name".into(), "名称".into());

    m.insert("nfe.sort_size".into(), "大小".into());

    m.insert("nfe.sort_type".into(), "类型".into());

    m.insert(
        "nfe.status_bar".into(),
        "{} 个项目 | {} 个目录 | {} 个文件 | 排序：{} {}".into(),
    );
}
