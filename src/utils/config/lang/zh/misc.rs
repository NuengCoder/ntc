use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.clear.type_help".into(),
        "输入 'help' 查看可用命令，输入 'exit' 退出。".into(),
    );

    m.insert(
        "misc.clear.welcome".into(),
        "欢迎使用 ntc {} - 导航, 工具包, 中心".into(),
    );

    m.insert(
        "misc.ds.example1".into(),
        " ds src # 使用配置深度搜索".into(),
    );

    m.insert(
        "misc.ds.example2".into(),
        " ds src -d 3 # 搜索最多3层深度".into(),
    );

    m.insert(
        "misc.ds.example3".into(),
        " ds test # 部分匹配：找到 test_utils/ 等".into(),
    );

    m.insert("misc.ds.usage".into(), "用法：ds <模式> [-d <深度>]".into());

    m.insert(
        "misc.fs.example1".into(),
        " fs main.c # 使用配置深度搜索".into(),
    );

    m.insert(
        "misc.fs.example2".into(),
        " fs main.c -d 5 # 搜索最多5层深度".into(),
    );

    m.insert(
        "misc.fs.example3".into(),
        " fs main # 部分匹配：找到 main_helper.c 等".into(),
    );

    m.insert("misc.fs.usage".into(), "用法：fs <模式> [-d <深度>]".into());

    m.insert("misc.fsc.copied".into(), "'{}' 已复制到剪贴板！".into());

    m.insert(
        "misc.fsc.copy_prompt".into(),
        "复制到剪贴板？（y/N）：".into(),
    );

    m.insert("misc.fsc.done_opt".into(), "0. 完成".into());

    m.insert("misc.fsc.done_str".into(), "完成。".into());

    m.insert(
        "misc.fsc.example1".into(),
        " fsc main.rs # 搜索然后选择文件显示".into(),
    );

    m.insert(
        "misc.fsc.example2".into(),
        " fsc main.rs -d 5 # 搜索最多5层深度".into(),
    );

    m.insert(
        "misc.fsc.found".into(),
        "\u{1F50D} 找到 {} 个文件匹配 \"{}\"：".into(),
    );

    m.insert("misc.fsc.invalid_input".into(), "无效的输入。".into());

    m.insert("misc.fsc.invalid_selection".into(), "无效的选择。".into());

    m.insert("misc.fsc.not_file".into(), "所选路径不是文件。".into());

    m.insert("misc.fsc.select_prompt".into(), "选择要显示的文件：".into());

    m.insert(
        "misc.fsc.skipped".into(),
        "已跳过（不支持的格式）：{}".into(),
    );

    m.insert(
        "misc.fsc.usage".into(),
        "用法：fsc <模式> [-d <深度>]".into(),
    );

    m.insert(
        "misc.gs.example1".into(),
        " gs main # 使用配置深度搜索文件内容".into(),
    );

    m.insert(
        "misc.gs.example2".into(),
        " gs main -d 5 # 搜索最多5层深度".into(),
    );

    m.insert(
        "misc.gs.example3".into(),
        " gs -i main # 交互模式（浏览结果，Enter打开）".into(),
    );

    m.insert(
        "misc.gs.example4".into(),
        " gs -i main -d 3 # 带深度限制的交互模式".into(),
    );

    m.insert(
        "misc.gs.example5".into(),
        " gs main -d 5 -l 10 # 仅显示前10个匹配".into(),
    );

    m.insert(
        "misc.gs.example6".into(),
        " gs main -p \"*.rs\" # 仅搜索匹配glob的文件".into(),
    );

    m.insert(
        "misc.gs.example7".into(),
        " gs main -s size # 按文件大小排序 (line|path|size, 裸-s=line)".into(),
    );

    m.insert(
        "misc.gs.example8".into(),
        " gs error\\|warning # 或搜索：匹配任意一个词 (a\\|b)".into(),
    );

    m.insert(
        "misc.gs.no_pipe_path".into(),
        "gs: -p 不能与管道输入混用 (管道传输的是文本而非文件)".into(),
    );

    m.insert(
        "misc.gs.pipe_sort_only_line".into(),
        "gs: 管道输入仅支持 -s line (path/size 需要文件)".into(),
    );

    m.insert("misc.gs.searching".into(), "正在搜索内容...".into());

    m.insert(
        "misc.gs.many_results".into(),
        "gs：找到 {} 个匹配，显示前20,000个。请缩小搜索范围。".into(),
    );

    m.insert(
        "misc.gs.usage".into(),
        "用法：gs <模式> [-d <深度>] [-l <数量>] [-i] [-p <glob>]... [-s [line|path|size]]".into(),
    );

    m.insert("misc.wi.not_dir".into(), "wi：不是目录：'{}'".into());

    m.insert("misc.wi.start_error".into(), "wi：无法监视：{}".into());

    m.insert("misc.wi.usage".into(), "用法：wi [路径]".into());

    m.insert(
        "misc.locate.example1".into(),
        " locate main.c # 搜索文件和目录（配置深度）".into(),
    );

    m.insert(
        "misc.locate.example2".into(),
        " locate main.c -d 5 # 搜索最多5层深度".into(),
    );

    m.insert(
        "misc.locate.example3".into(),
        " locate main # 部分匹配：找到 main.c, main_helper/ 等".into(),
    );

    m.insert(
        "misc.locate.header".into(),
        "\u{1F50D} {} \"{}\"（{} 个结果）：".into(),
    );

    m.insert(
        "misc.locate.label_didyoumean".into(),
        "您是不是要找？".into(),
    );

    m.insert("misc.locate.label_exact".into(), "精确匹配".into());

    m.insert("misc.locate.label_partial".into(), "部分匹配".into());

    m.insert(
        "misc.locate.no_results".into(),
        "\n\u{1F50D} 未找到 \"{}\" 的结果\n".into(),
    );

    m.insert(
        "misc.locate.usage".into(),
        "用法：locate <模式> [-d <深度>]".into(),
    );

    m.insert("misc.mkd.created".into(), "已创建目录：{}".into());

    m.insert("misc.mkd.error".into(), "mkd：{}：{}".into());

    m.insert("misc.mkd.missing".into(), "mkd：缺少操作数".into());

    m.insert("misc.mkf.created".into(), "已创建文件：{}".into());

    m.insert("misc.mkf.error".into(), "mkf：{}：{}".into());

    m.insert("misc.mkf.exists".into(), "已存在：{}".into());

    m.insert("misc.mkf.missing".into(), "mkf：缺少操作数".into());

    m.insert("misc.ne.create_failed".into(), "无法创建模板：{}".into());

    m.insert(
        "misc.ne.created_hyper".into(),
        "已创建 hyper 项目：{}".into(),
    );

    m.insert("misc.ne.created_super".into(), "已创建超级模板：{}".into());

    m.insert("misc.ne.created_template".into(), "已创建模板：{}".into());

    m.insert("misc.ne.editor_error".into(), "编辑器错误：{}".into());

    m.insert(
        "misc.ne.file_missing".into(),
        "文件不存在：{}。使用 --init <文件> 或 --super <文件> 创建它。".into(),
    );

    m.insert("misc.ne.hyper_error".into(), "Hyper 项目错误：{}".into());

    m.insert("misc.ne.missing_file".into(), "ne：缺少文件参数".into());

    m.insert("misc.ne.one_file".into(), "一次只能创建一个文件".into());

    m.insert(
        "misc.ne.open_not_created".into(),
        "-I 文件必须是正在创建的文件之一：{}".into(),
    );

    m.insert(
        "misc.ne.usage_open".into(),
        "用法：-I <文件> 必须是最后一个参数（仅限一个文件）".into(),
    );

    m.insert(
        "misc.ne.opened_welcome".into(),
        "已在 {} 中打开欢迎屏幕".into(),
    );

    m.insert(
        "misc.ne.usage_hyper".into(),
        "用法：ne --hyper <名称> --lang <语言>（名称不能包含扩展名）".into(),
    );

    m.insert("misc.pathcp.copied".into(), "路径已复制到剪贴板：{}".into());

    m.insert(
        "misc.pip.found".into(),
        "\u{2713} 在以下位置找到 pip：{}".into(),
    );

    m.insert(
        "misc.pip.not_found".into(),
        "在 venv/Scripts 目录中未找到 pip".into(),
    );

    m.insert("misc.rmd.error".into(), "rmd：{}：{}".into());

    m.insert("misc.rmd.missing".into(), "rmd：缺少操作数".into());

    m.insert("misc.rmd.not_dir".into(), "rmd：'{}' 不是目录".into());

    m.insert("misc.rmd.not_exist".into(), "rmd：'{}' 不存在".into());

    m.insert("misc.rmd.removed".into(), "已删除目录：{}".into());

    m.insert(
        "misc.rmd.no_match".into(),
        "rmd：没有匹配 '{}' 的目录".into(),
    );

    m.insert(
        "misc.rmd.pattern_matches".into(),
        "{} 个目录匹配 '{}'：".into(),
    );

    m.insert("misc.rmd.confirm_delete".into(), "删除 {} 个目录？".into());

    m.insert("misc.rmd.cancelled".into(), "已取消。".into());

    m.insert("misc.rmf.error".into(), "rmf：{}：{}".into());

    m.insert(
        "misc.rmf.is_dir".into(),
        "rmf：'{}' 是目录（请使用 rmd）".into(),
    );

    m.insert("misc.rmf.missing".into(), "rmf：缺少操作数".into());

    m.insert("misc.rmf.not_exist".into(), "rmf：'{}' 不存在".into());

    m.insert("misc.rmf.no_match".into(), "rmf：没有文件匹配 '{}'".into());

    m.insert(
        "misc.rmf.pattern_matches".into(),
        "有 {} 个文件匹配 '{}'：".into(),
    );

    m.insert(
        "misc.rmf.confirm_delete".into(),
        "删除 {} 个文件吗？".into(),
    );

    m.insert("misc.rmf.cancelled".into(), "已取消。".into());

    m.insert("misc.rmf.removed".into(), "已删除：{}".into());

    m.insert("misc.rnmd.error".into(), "rnmd：{}".into());

    m.insert("misc.rnmd.exists".into(), "rnmd：'{}' 已存在".into());

    m.insert("misc.rnmd.missing".into(), "rnmd：缺少操作数".into());

    m.insert("misc.rnmd.not_dir".into(), "rnmd：'{}' 不是目录".into());

    m.insert("misc.rnmd.not_exist".into(), "rnmd：'{}' 不存在".into());

    m.insert(
        "misc.rnmd.renamed".into(),
        "目录已重命名 '{}' -> '{}'".into(),
    );

    m.insert(
        "misc.rnmd.usage".into(),
        " 用法：rnmd <旧名称> <新名称>".into(),
    );

    m.insert("misc.rnmf.error".into(), "rnmf：{}".into());

    m.insert("misc.rnmf.exists".into(), "rnmf：'{}' 已存在".into());

    m.insert(
        "misc.rnmf.is_dir".into(),
        "rnmf：'{}' 是目录（请使用 rnmd）".into(),
    );

    m.insert("misc.rnmf.missing".into(), "rnmf：缺少操作数".into());

    m.insert("misc.rnmf.not_exist".into(), "rnmf：'{}' 不存在".into());

    m.insert("misc.rnmf.renamed".into(), "已重命名 '{}' -> '{}'".into());

    m.insert(
        "misc.rnmf.usage".into(),
        " 用法：rnmf <旧名称> <新名称>".into(),
    );

    m.insert("misc.mv.error".into(), "mv：{}".into());

    m.insert(
        "misc.mv.exists".into(),
        "mv：目标已存在：'{}'（使用 -f 覆盖）".into(),
    );

    m.insert("misc.mv.missing".into(), "mv：缺少操作数".into());

    m.insert("misc.mv.moved".into(), "已移动 '{}' -> '{}'".into());

    m.insert("misc.mv.not_exist".into(), "mv：源不存在：'{}'".into());

    m.insert("misc.mv.no_match".into(), "mv：没有匹配的文件：'{}'".into());

    m.insert("misc.mv.dst_dir".into(), "mv：目标必须是目录：'{}'".into());

    m.insert("misc.mv.wild_done".into(), "已移动 {} 个文件到 '{}'".into());

    m.insert(
        "misc.mv.usage".into(),
        "用法：mv <源|@tp|通配符> <目标|@tp> [-f]".into(),
    );

    m.insert("misc.cp.error".into(), "cp：{}".into());

    m.insert(
        "misc.cp.exists".into(),
        "cp：目标已存在：'{}'（使用 -f 覆盖）".into(),
    );

    m.insert("misc.cp.missing".into(), "cp：缺少操作数".into());

    m.insert("misc.cp.copied".into(), "已复制 '{}' -> '{}'".into());

    m.insert("misc.cp.not_exist".into(), "cp：源不存在：'{}'".into());

    m.insert("misc.cp.no_match".into(), "cp：没有匹配的文件：'{}'".into());

    m.insert("misc.cp.dst_dir".into(), "cp：目标必须是目录：'{}'".into());

    m.insert("misc.cp.wild_done".into(), "已复制 {} 个文件到 '{}'".into());

    m.insert("misc.cp.same".into(), "cp：源和目标是同一文件：'{}'".into());

    m.insert(
        "misc.cp.usage".into(),
        "用法：cp <源|@tp|通配符> <目标|@tp> [-f]".into(),
    );

    m.insert(
        "misc.say.example1".into(),
        " say \"hello world\" --loop 5 -n # 5行".into(),
    );

    m.insert(
        "misc.say.example2".into(),
        " say \"hello world\" --loop 3 -t # 1行，重复3次".into(),
    );

    m.insert(
        "misc.say.loop_exceed".into(),
        "--loop 值超过最大值（{}），已限制".into(),
    );

    m.insert(
        "misc.say.loop_invalid".into(),
        "无效的 --loop 值 '{}'，使用1".into(),
    );

    m.insert(
        "misc.say.loop_requires".into(),
        "--loop 需要一个数字，使用1".into(),
    );

    m.insert(
        "misc.say.loop_warning".into(),
        "--loop 值不能小于1，使用1".into(),
    );

    m.insert(
        "misc.say.usage".into(),
        "用法：say <文本> [--loop N] [-n|-t]".into(),
    );

    m.insert(
        "misc.show_env.activate_hint".into(),
        "运行 'python_activate' 激活虚拟环境".into(),
    );

    m.insert("misc.show_env.active".into(), "虚拟环境".into());

    m.insert("misc.show_env.and_more".into(), "... 及另外 {} 个".into());

    m.insert("misc.show_env.cwd_label".into(), "当前目录：".into());

    m.insert(
        "misc.show_env.deactivate_hint".into(),
        "运行 'deactivate' 退出虚拟环境".into(),
    );

    m.insert("misc.show_env.not_active".into(), "虚拟环境".into());

    m.insert("misc.show_env.not_set".into(), "（未设置）".into());

    m.insert("misc.show_env.os_label".into(), "操作系统：".into());

    m.insert(
        "misc.show_env.other_header".into(),
        "\u{1F527} 其他Python/Venv变量：".into(),
    );

    m.insert(
        "misc.show_env.path_full".into(),
        "\u{1F4C2} PATH（完整）：".into(),
    );

    m.insert(
        "misc.show_env.path_header".into(),
        "\u{1F4C2} PATH（前几个条目）：".into(),
    );

    m.insert("misc.show_env.shell_label".into(), "Shell：".into());

    m.insert(
        "misc.show_env.summary_header".into(),
        "\u{1F4CA} 摘要：".into(),
    );

    m.insert(
        "misc.show_env.sys_header".into(),
        "\u{1F4BB} 系统信息：".into(),
    );

    m.insert(
        "misc.show_env.venv_header".into(),
        "\u{1F40D} Python虚拟环境：".into(),
    );

    m.insert("misc.size.bytes_label".into(), "字节：".into());

    m.insert("misc.size.calculating".into(), "正在计算目录大小...".into());

    m.insert("misc.size.human_label".into(), "可读格式：".into());

    m.insert("misc.size.path_label".into(), "路径：".into());

    m.insert("misc.size.time_label".into(), "耗时：".into());

    m.insert("misc.size.title".into(), "当前目录大小".into());

    m.insert(
        "misc.tp.example_info".into(),
        "示例：tp info my_project".into(),
    );

    m.insert("misc.tp.info_canonical".into(), "规范路径".into());

    m.insert("misc.tp.info_exists".into(), "存在".into());

    m.insert("misc.tp.info_global".into(), "全局".into());

    m.insert("misc.tp.info_index".into(), "索引".into());

    m.insert("misc.tp.info_name".into(), "名称".into());

    m.insert("misc.tp.info_path".into(), "路径".into());

    m.insert("misc.tp.info_source".into(), "来源".into());

    m.insert(
        "misc.tp.info_title".into(),
        "\u{1F4CC} 传送保存点信息".into(),
    );

    m.insert(
        "misc.tp.invalid_format".into(),
        "无效的格式。使用：tp rnm <旧名称> to <新名称>".into(),
    );

    m.insert("misc.tp.not_found".into(), "未找到保存点 '{}'".into());

    m.insert(
        "misc.tp.type_help".into(),
        "输入 'tp help' 查看用法。".into(),
    );

    m.insert(
        "misc.tp.unknown_subcmd".into(),
        "未知的 tp 子命令：{}".into(),
    );

    m.insert(
        "misc.tp.usage_add".into(),
        "用法：tp add <名称> [路径]".into(),
    );

    m.insert("misc.tp.usage_info".into(), "用法：tp info <名称>".into());

    m.insert(
        "misc.tp.usage_jump".into(),
        "用法：tp jump <名称|编号>".into(),
    );

    m.insert(
        "misc.tp.usage_rm".into(),
        "用法：tp rm <名称|编号>[, <名称|编号>, ...]".into(),
    );

    m.insert(
        "misc.tp.usage_rnm".into(),
        "用法：tp rnm <旧名称> to <新名称>".into(),
    );

    m.insert("misc.tpb.unknown".into(), "未知的 tpb 选项：{}".into());

    m.insert("misc.tpb.usage".into(), "用法：".into());

    m.insert(
        "misc.tpb.usage_basic".into(),
        " tpb - 传送到上一个位置".into(),
    );

    m.insert(
        "misc.tpb.usage_clear".into(),
        " tpb clear - 清除传送历史".into(),
    );

    m.insert(
        "misc.tpb.usage_history".into(),
        " tpb history - 显示传送历史".into(),
    );

    m.insert(
        "misc.ui.removed".into(),
        "现代UI已被移除。仅经典Shell可用。".into(),
    );

    m.insert("misc.venva.activated".into(), "已激活：{}".into());

    m.insert(
        "misc.venva.activating".into(),
        "正在激活虚拟环境：{}".into(),
    );

    m.insert(
        "misc.venva.active_info".into(),
        "虚拟环境现已激活。pip和python命令将使用此环境。".into(),
    );

    m.insert(
        "misc.venva.already_in".into(),
        "已在虚拟环境中。请先使用 'deactivate' 退出。".into(),
    );

    m.insert("misc.venva.failed".into(), "激活失败：{}".into());

    m.insert(
        "misc.venva.no_script".into(),
        "未在以下位置找到激活脚本：{}".into(),
    );

    m.insert(
        "misc.venva.not_found".into(),
        "未找到虚拟环境。用法：python_activate <环境名称>".into(),
    );

    m.insert("misc.venvd.deactivated".into(), "虚拟环境已停用".into());

    m.insert("misc.venvd.not_in_venv".into(), "不在虚拟环境中".into());

    m.insert(
        "misc.qur.stdin_needs_select".into(),
        "qur：管道输入需要 -s \"<模式>\"（例如 cmd | qur -s \"foo\"）。".into(),
    );

    m.insert(
        "misc.head.usage".into(),
        "用法：head [N] [file]（N 默认为 10；未给文件时读取管道输入）".into(),
    );

    m.insert(
        "misc.tail.usage".into(),
        "用法：tail [N] [file]（N 默认为 10；未给文件时读取管道输入）".into(),
    );

    m.insert(
        "misc.betw.usage".into(),
        "用法：betw A:B [file]（A:B 从 1 开始，含两端；未给文件时读取管道输入）".into(),
    );

    m.insert("misc.window.bad_arg".into(), "{}：意外的参数 '{}'".into());

    m.insert(
        "misc.window.no_input".into(),
        "{}：没有输入（请通过管道传入或指定文件）".into(),
    );

    m.insert(
        "misc.sort.usage".into(),
        "用法：sort [-r] [-n] [-u] [-f] [file]（未指定文件时读取管道输入）".into(),
    );

    m.insert("misc.sort.bad_arg".into(), "sort：未知选项 '{}'".into());

    m.insert(
        "misc.rev.usage".into(),
        "用法：rev [file]（反转每行的字符；未指定文件时读取管道输入）".into(),
    );

    m.insert(
        "misc.rev.bad_arg".into(),
        "rev：意外的参数 '{}'（用法：rev [file]）".into(),
    );

    m.insert(
        "misc.trim.usage".into(),
        "用法：trim <pattern> [文本]   （或：<命令> | trim <pattern> [...]）".into(),
    );

    m.insert(
        "misc.trim.empty_pattern".into(),
        "trim：模式不能为空".into(),
    );
}
