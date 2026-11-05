use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.todo.add".into(),
        "todo add \"名称\" ## 添加新任务".into(),
    );

    m.insert(
        "help.todo.add_at".into(),
        "todo add \"名称\" -> N ## 在位置N插入任务".into(),
    );

    m.insert(
        "help.todo.clear".into(),
        "todo clear ## 清除所有任务（需确认）".into(),
    );

    m.insert(
        "help.todo.info".into(),
        "todo info [N] ## 显示所有任务或任务N的详情".into(),
    );

    m.insert("help.todo.rm".into(), "todo rm <N> ## 删除任务N".into());

    m.insert(
        "help.todo.show".into(),
        "todo ## 显示所有任务（来自当前目录的 future_todo.txt）".into(),
    );

    m.insert(
        "help.todo.move".into(),
        "todo move <N> <M> ## 把第 N 条移到第 M 位（其余顺移）".into(),
    );

    m.insert(
        "help.todo.replace".into(),
        "todo replace <N> <M> ## 交换第 N 条和第 M 条".into(),
    );

    m.insert(
        "help.todo.example4".into(),
        "todo mv 5 1 ## 把第 5 条移到最前".into(),
    );

    m.insert(
        "help.todo.example5".into(),
        "todo rp 5 1 ## 交换第 5 条和第 1 条".into(),
    );

    m.insert("help.todo.title".into(), "待办事项命令：".into());

    m.insert(
        "help.todo.done".into(),
        "todo done <N> ## 标记任务 N 为已完成".into(),
    );

    m.insert(
        "help.todo.undo".into(),
        "todo undo <N> ## 重新打开任务 N".into(),
    );

    m.insert(
        "help.todo.edit".into(),
        "todo edit <N> \"名称\" ## 重命名任务 N".into(),
    );

    m.insert("help.todo.example1".into(), "todo ## 显示所有任务".into());

    m.insert(
        "help.todo.example2".into(),
        "todo add \"Fix bug\" ## 添加新任务".into(),
    );

    m.insert(
        "help.todo.example3".into(),
        "todo rm 1 ## 删除任务 #1".into(),
    );
}
