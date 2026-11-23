use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.tpb.title".into(), "TPB 命令：".into());

    m.insert(
        "help.tpb.usage".into(),
        "tpb ## 传送到上一个位置（撤销传送）".into(),
    );

    m.insert(
        "help.tpb.history".into(),
        "tpb history ## 显示传送导航历史".into(),
    );

    m.insert("help.tpb.clear".into(), "tpb clear ## 清除传送历史".into());

    m.insert("help.tpb.example1".into(), "tpb ## 返回上一个位置".into());
}
