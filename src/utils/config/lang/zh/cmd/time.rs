use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.time.bench_header".into(),
        "正在基准测试 {} 次运行：{}".into(),
    );
    m.insert("cmd.time.bench_max".into(), "  最大：  {}".into());
    m.insert("cmd.time.bench_mean".into(), "  平均：  {}".into());
    m.insert("cmd.time.bench_median".into(), "  中位数：{}".into());
    m.insert("cmd.time.bench_min".into(), "  最小：  {}".into());
    m.insert(
        "cmd.time.bench_note".into(),
        "重复运行可能改变文件或环境；副作用不会回滚。".into(),
    );
    m.insert("cmd.time.bench_run".into(), "  第 {} 次：{}（{}）".into());
    m.insert(
        "cmd.time.bench_summary".into(),
        "摘要（{} 次运行）：".into(),
    );
    m.insert(
        "cmd.time.bench_usage".into(),
        "用法：time --bench <n> <command>".into(),
    );
    m.insert("cmd.time.cancelled".into(), "计时已取消。".into());
    m.insert(
        "cmd.time.compare_usage".into(),
        "用法：time <命令 A> --compare <命令 B> [-w]".into(),
    );
    m.insert(
        "cmd.time.count_usage".into(),
        "用法：time --count <时长>".into(),
    );
    m.insert("cmd.time.elapsed".into(), "耗时：{}".into());
    m.insert("cmd.time.executing".into(), "正在执行：{}".into());
    m.insert("cmd.time.exit_status".into(), "退出状态：{}".into());
    m.insert("cmd.time.finished".into(), "时间到！".into());
    m.insert(
        "cmd.time.first_command".into(),
        "正在运行第一条命令：{}".into(),
    );
    m.insert(
        "cmd.time.first_result".into(),
        "第一条：{}（退出 {}）".into(),
    );
    m.insert(
        "cmd.time.ignoring_whitespace".into(),
        "忽略仅空白字符的差异。".into(),
    );
    m.insert(
        "cmd.time.invalid_duration".into(),
        "无效时长“{}”。请使用秒数或 90s、5m、1h30m 等格式。".into(),
    );
    m.insert(
        "cmd.time.invalid_runs".into(),
        "time --bench：无效运行次数“{}”。请使用正整数。".into(),
    );
    m.insert(
        "cmd.time.large_diff".into(),
        "差异过大；以下分别显示删除和新增的行，不做对齐：".into(),
    );
    m.insert("cmd.time.outputs_differ".into(), "命令输出不同：".into());
    m.insert("cmd.time.outputs_match".into(), "命令输出相同。".into());
    m.insert("cmd.time.remaining".into(), "剩余 {} 秒 ".into());
    m.insert(
        "cmd.time.run_usage".into(),
        "用法：time <command>".into(),
    );
    m.insert(
        "cmd.time.second_command".into(),
        "正在运行第二条命令：{}".into(),
    );
    m.insert(
        "cmd.time.second_result".into(),
        "第二条：{}（退出 {}）".into(),
    );
    m.insert("cmd.time.status_failed".into(), "失败".into());
    m.insert("cmd.time.status_ok".into(), "成功".into());
    m.insert("cmd.time.terminated".into(), "命令已终止".into());
}
