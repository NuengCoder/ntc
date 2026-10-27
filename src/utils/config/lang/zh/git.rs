use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "git.no_git_binary".into(),
        "未安装 Git。请运行 `ntpkg install git` 安装。".into(),
    );

    m.insert(
        "git.not_repo".into(),
        "不是 git 仓库（或其父目录）。".into(),
    );

    m.insert("git.executing".into(), "正在执行：{}".into());

    m.insert("git.completed".into(), "git 命令已完成。".into());

    m.insert("git.exit_code".into(), "命令退出码：{}".into());

    m.insert("git.terminated".into(), "命令已被终止。".into());

    m.insert("git.failed".into(), "执行命令失败：{}".into());

    m.insert("git.checking_status".into(), "正在检查状态".into());

    m.insert("git.status_failed".into(), "无法读取 git 状态。".into());

    m.insert("git.branch_header".into(), "当前分支".into());

    m.insert("git.upstream".into(), "跟踪 {}".into());

    m.insert("git.ahead".into(), "领先 {}".into());

    m.insert("git.behind".into(), "落后 {}".into());

    m.insert("git.detached_header".into(), "分离的 HEAD".into());

    m.insert("git.section_conflicts".into(), "未合并的路径：".into());

    m.insert("git.section_staged".into(), "待提交的更改：".into());

    m.insert("git.section_unstaged".into(), "尚未暂存的更改：".into());

    m.insert("git.section_untracked".into(), "未跟踪的文件：".into());

    m.insert("git.clean".into(), "工作区干净。".into());

    m.insert("git.branch_failed".into(), "无法列出分支。".into());

    m.insert("git.log_failed".into(), "无法读取 git 日志。".into());

    m.insert("git.log_empty".into(), "未找到提交。".into());

    m.insert("git.help_title".into(), "GIT 集成：".into());

    m.insert(
        "git.help_status".into(),
        "git status ## 显示分支和暂存/未暂存/未跟踪".into(),
    );

    m.insert("git.help_branch".into(), "git branch ## 列出分支".into());

    m.insert("git.help_log".into(), "git log ## 最近的彩色提交".into());

    m.insert("git.help_diff".into(), "git diff ## 工作区差异".into());

    m.insert(
        "git.help_passthrough".into(),
        "git <任意> ## 其他都透传给真实 git".into(),
    );

    m.insert("git.examples_header".into(), "示例：".into());

    m.insert("git.help_example1".into(), "git status".into());

    m.insert("git.help_example2".into(), "git checkout main".into());
}
