use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.git.title".into(), "GIT 集成命令：".into());

    m.insert(
        "help.git.status".into(),
        "git status | st ## 漂亮的仓库状态输出".into(),
    );

    m.insert(
        "help.git.branch".into(),
        "git branch | b ## 列出分支（当前分支高亮）".into(),
    );

    m.insert(
        "help.git.log".into(),
        "git log | l ## 彩色最近提交日志".into(),
    );

    m.insert("help.git.diff".into(), "git diff ## 显示工作区差异".into());

    m.insert(
        "help.git.passthrough".into(),
        "git <任意> ## 其他 git 子命令直接透传".into(),
    );

    m.insert(
        "help.git.example1".into(),
        "git status ## 显示分支和暂存/未暂存/未跟踪".into(),
    );

    m.insert(
        "help.git.example2".into(),
        "git branch -a ## 列出全部分支（透传）".into(),
    );

    m.insert(
        "help.git.example3".into(),
        "git commit -am \"消息\" ## 常规 git 流程（透传）".into(),
    );
}
