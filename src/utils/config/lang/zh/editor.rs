use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("editor.already_root".into(), "已在根目录".into());

    m.insert("editor.auto_save_off".into(), "退出时自动保存：关".into());

    m.insert("editor.auto_save_on".into(), "退出时自动保存：开".into());

    m.insert("editor.cleared_cursors".into(), "已清除额外光标".into());

    m.insert("editor.clip_copied_chars".into(), "已复制 {} 个字符".into());

    m.insert("editor.clip_copied_line".into(), "已复制行".into());

    m.insert("editor.clip_cut_chars".into(), "已剪切 {} 个字符".into());

    m.insert("editor.clip_cut_line".into(), "已剪切行".into());

    m.insert("editor.clip_empty".into(), "剪贴板为空".into());

    m.insert("editor.clip_yanked".into(), "已拉出行".into());

    m.insert("editor.clipboard_empty".into(), "剪贴板为空".into());

    m.insert("editor.close_btn".into(), "\u{2715}".into());

    m.insert("editor.cmd_already_at_root".into(), "已在根目录".into());

    m.insert("editor.cmd_auto_off".into(), "退出时自动保存：关".into());

    m.insert("editor.cmd_auto_on".into(), "退出时自动保存：开".into());

    m.insert("editor.cmd_cd_prefix".into(), "cd：".into());

    m.insert("editor.cmd_color_off".into(), "颜色：关".into());

    m.insert("editor.cmd_color_on".into(), "颜色：开".into());

    m.insert("editor.cmd_created_dir".into(), "已创建目录：".into());

    m.insert("editor.cmd_current_theme".into(), "当前主题：".into());

    m.insert(
        "editor.cmd_exited_hex".into(),
        "已退出十六进制模式（更改已丢弃）".into(),
    );

    m.insert("editor.cmd_gos_only".into(), "gos：只有 {} 个子目录".into());

    m.insert("editor.cmd_gos_usage".into(), "gos：用法 gos <N>".into());

    m.insert(
        "editor.cmd_goto_usage".into(),
        "goto：用法 goto <地址> 或 goto 0x<十六进制>".into(),
    );

    m.insert(
        "editor.cmd_hex_save_failed".into(),
        "十六进制保存失败：".into(),
    );

    m.insert("editor.cmd_hex_saved".into(), "十六进制已保存".into());

    m.insert("editor.cmd_jumped_to_line".into(), "已跳转到行".into());

    m.insert("editor.cmd_jumped_to_offset".into(), "已跳转到偏移".into());

    m.insert("editor.cmd_mkd_failed".into(), "mkd：".into());

    m.insert("editor.cmd_mkd_missing".into(), "mkd：缺少操作数".into());

    m.insert("editor.cmd_mkdir_failed".into(), "mkdir：".into());

    m.insert(
        "editor.cmd_mkdir_missing".into(),
        "mkdir：缺少操作数".into(),
    );

    m.insert("editor.cmd_mkf_failed".into(), "mkf：".into());

    m.insert("editor.cmd_mkf_missing".into(), "mkf：缺少操作数".into());

    m.insert("editor.cmd_no_cwd".into(), "无法获取当前目录".into());

    m.insert("editor.cmd_no_subdirs".into(), "没有子目录".into());

    m.insert("editor.cmd_no_tp".into(), "没有传送点".into());

    m.insert("editor.cmd_not_dir".into(), "不是目录".into());

    m.insert("editor.cmd_not_exist".into(), "不存在".into());

    m.insert("editor.cmd_removed".into(), "已删除：".into());

    m.insert("editor.cmd_removed_dir".into(), "已删除目录：".into());

    m.insert("editor.cmd_rmd".into(), "rmd：".into());

    m.insert("editor.cmd_rmd_missing".into(), "rmd：缺少操作数".into());

    m.insert("editor.cmd_rmf".into(), "rmf：".into());

    m.insert("editor.cmd_rmf_failed".into(), "rmf：".into());

    m.insert("editor.cmd_rmf_missing".into(), "rmf：缺少操作数".into());

    m.insert("editor.cmd_run_missing".into(), "run：缺少命令".into());

    m.insert("editor.cmd_save_failed".into(), "保存失败：".into());

    m.insert("editor.cmd_saved".into(), "已保存".into());

    m.insert("editor.cmd_syntax_off".into(), "语法高亮：关".into());

    m.insert("editor.cmd_syntax_on".into(), "语法高亮：开".into());

    m.insert(
        "editor.cmd_theme_list_hint".into(),
        "使用 'theme list' 查看可用主题。".into(),
    );

    m.insert("editor.cmd_theme_not_found".into(), "未找到主题".into());

    m.insert("editor.cmd_theme_switched".into(), "已切换主题".into());

    m.insert("editor.cmd_themes".into(), "主题：".into());

    m.insert("editor.cmd_touch_failed".into(), "touch：".into());

    m.insert(
        "editor.cmd_touch_missing".into(),
        "touch：缺少操作数".into(),
    );

    m.insert("editor.cmd_tp_add_error".into(), "tp-add：".into());

    m.insert(
        "editor.cmd_tp_add_missing".into(),
        "tp-add：缺少名称".into(),
    );

    m.insert("editor.cmd_tp_added".into(), "已创建传送点".into());

    m.insert("editor.cmd_tp_error".into(), "tp：".into());

    m.insert("editor.cmd_tp_jumped".into(), "已传送到".into());

    m.insert("editor.cmd_tp_list".into(), "传送点：".into());

    m.insert("editor.cmd_tp_not_found".into(), "未找到传送点：".into());

    m.insert("editor.cmd_unknown".into(), "未知命令：".into());

    m.insert(
        "editor.cmd_visual_hint".into(),
        "可视模式 方向键选择，Alt+hjkl/方向键移动所选，Esc 退出".into(),
    );

    m.insert(
        "editor.visual_dedent_blocked".into(),
        "Alt+h 需要所选每行都有缩进".into(),
    );

    m.insert("editor.color_off".into(), "颜色：关".into());

    m.insert("editor.color_on".into(), "颜色：开".into());

    m.insert("editor.compl_builtin".into(), "[内置]".into());

    m.insert("editor.compl_math_abs".into(), "绝对值".into());

    m.insert("editor.compl_math_acos".into(), "反余弦（弧度）".into());

    m.insert("editor.compl_math_acot".into(), "反余切（弧度）".into());

    m.insert("editor.compl_math_acsc".into(), "反余割（弧度）".into());

    m.insert("editor.compl_math_asec".into(), "反正割（弧度）".into());

    m.insert("editor.compl_math_asin".into(), "反正弦（弧度）".into());

    m.insert("editor.compl_math_atan".into(), "反正切（弧度）".into());

    m.insert("editor.compl_math_avg".into(), "平均值".into());

    m.insert("editor.compl_math_ceil".into(), "向上取整".into());

    m.insert("editor.compl_math_cos".into(), "余弦（弧度）".into());

    m.insert("editor.compl_math_cot".into(), "余切（弧度）".into());

    m.insert("editor.compl_math_csc".into(), "余割（弧度）".into());

    m.insert("editor.compl_math_floor".into(), "向下取整".into());

    m.insert("editor.compl_math_ln".into(), "自然对数".into());

    m.insert("editor.compl_math_log10".into(), "以 10 为底的对数".into());

    m.insert("editor.compl_math_log2".into(), "以 2 为底的对数".into());

    m.insert("editor.compl_math_max".into(), "最大值".into());

    m.insert("editor.compl_math_min".into(), "最小值".into());

    m.insert("editor.compl_math_pow".into(), "幂（x^y）".into());

    m.insert("editor.compl_math_rand".into(), "随机数".into());

    m.insert("editor.compl_math_round".into(), "四舍五入".into());

    m.insert("editor.compl_math_sec".into(), "正割（弧度）".into());

    m.insert("editor.compl_math_sin".into(), "正弦（弧度）".into());

    m.insert("editor.compl_math_sqrt".into(), "平方根".into());

    m.insert("editor.compl_math_sum".into(), "总和".into());

    m.insert("editor.compl_math_tan".into(), "正切（弧度）".into());

    m.insert("editor.compl_math_tobinary".into(), "转换为二进制".into());

    m.insert("editor.compl_math_todecimal".into(), "转换为十进制".into());

    m.insert(
        "editor.compl_math_tohb".into(),
        "转换为人类可读字节数".into(),
    );

    m.insert("editor.compl_math_tohex".into(), "转换为十六进制".into());

    m.insert("editor.compl_math_tooctal".into(), "转换为八进制".into());

    m.insert("editor.compl_other".into(), "[其他]".into());

    m.insert("editor.compl_print_values".into(), "打印值".into());

    m.insert("editor.compl_return_value".into(), "返回值".into());

    m.insert("editor.completion_title".into(), "补全".into());

    m.insert("editor.copied_line".into(), "已复制行".into());

    m.insert("editor.cut_line".into(), "已剪切行".into());

    m.insert(
        "editor.edit_added_cursor".into(),
        "已添加光标（共 {} 个）".into(),
    );

    m.insert("editor.edit_already_selected".into(), "已选中".into());

    m.insert(
        "editor.edit_no_more_occurrences".into(),
        "没有更多匹配项".into(),
    );

    m.insert("editor.edit_wrap".into(), "换行".into());

    m.insert(
        "editor.exited_hex".into(),
        "已退出十六进制模式（更改已放弃）".into(),
    );

    m.insert(
        "editor.exited_multicursor".into(),
        "已退出多光标模式".into(),
    );

    m.insert(
        "editor.ff_prompt_find_empty".into(),
        "查找：输入搜索...".into(),
    );

    m.insert("editor.ff_prompt_find_prefix".into(), "查找：".into());

    m.insert("editor.ff_prompt_gs_prefix".into(), "gs:".into());

    m.insert(
        "editor.file_changed_reload".into(),
        "文件已外部更改 \u{2014} 已重新加载".into(),
    );

    m.insert("editor.gosc_exit_hint".into(), "0=退出 ".into());

    m.insert(
        "editor.gosc_hint".into(),
        " Enter=进入 -N=后退 N Esc=关闭 ".into(),
    );

    m.insert("editor.gosc_title".into(), " gosc — 连续导航 ".into());

    m.insert("editor.gs_loading".into(), " 正在查找... {} 请稍候".into());

    m.insert("editor.help_screen".into(), " 普通模式（vim 风格）\n i=插入 v=可视 q=查询 x=保存 n=文件 m=目录 c=复制 e=导航 f=查找 b=文件 t=终端 u=编码 g=命令 :=命令 ?=帮助 0=退出（未保存会提示）\n hjkl/awsd/方向键移动 \u{B7} Home/End 行首尾 \u{B7} PgUp/PgDn 翻页 \u{B7} ^J=跳转\n\n GMODE（按 g，输入，Enter）\n t/T=上下标签 qt=关闭标签 aqt=关闭全部 数字=跳转 q=退出 u=撤销 r=重做 n=文件 m=目录 i=插入 v=可视 e=导航 Esc=取消\n\n NFE 侧栏（按 e）\n hjkl/方向键/wasd 移动 \u{B7} Enter=目录进入/文件打开（停留） \u{B7} i=插入 v=可视 \u{B7} Esc=返回\n\n 插入模式（编辑）\n 输入即插入 \u{B7} 方向键移动 \u{B7} Esc=普通 ^S=保存 ^Z=撤销 ^R=重做 ^D=多光标 ^F=查找 ^N=文件 ^P=粘贴 ^T=终端 ^G=命令 ^E=导航 ^W=可视 ^:=命令 \u{B7} Home/End 行首尾 \u{B7} PgUp/PgDn 翻页 \u{B7} F3/Sh+F3 查找上下\n\n 可视模式（选择）\n hjkl/方向键移动（Shift 选择） \u{B7} Alt+hjkl/方向键移动所选（j/k）或缩进（h/l） \u{B7} a=全选切换 c=复制 x=剪切 y=复制行 p=粘贴 :=命令 e=导航 f=查找 g=命令 \u{B7} Esc=退出\n\n 命令模式\n :<number> 跳转到行号\n :l N 或 :line N 跳转到第 N 行\n :q 或 :quit 退出编辑器（带保存提示）\n :wq 保存并退出\n :w 或 :write 保存文件\n :auto on 启用自动保存\n :auto off 禁用自动保存\n :syntax on 启用语法高亮\n :syntax off 禁用语法高亮\n :color on|off 切换彩色输出\n :cd <路径> 更改目录（导航器）\n :back [N] 返回父目录（N 级）\n :gos <N> 导航到第 N 个子目录\n :gosc 列出编号的子目录\n :mkdir <目录> 创建目录\n :mkd <目录> 创建目录（:mkdir 别名）\n :touch <文件> 创建新文件\n :mkf <文件> 创建新文件（:touch 别名）\n :rmd <目标> 递归删除目录\n :rmf <目标> 强制删除文件或目录\n :rnmf <源> <目标> 重命名文件\n :rnmd <源> <目标> 重命名目录\n :ne <文件> 导航并编辑（不存在则创建）\n :ne . 在侧边栏显示当前目录\n :e 打开文件浏览器（nfe）\n :theme <名称> 按名称切换主题\n :theme list 列出可用主题\n :theme 显示当前主题\n :git diff 或 :gd 显示当前文件的 git 差异\n :git blame 或 :blame 显示当前行的 blame 信息\n :tp <名称> 跳转到传送点\n :tp-add <名称> 将当前目录保存为传送点\n :gs <模式> 在项目中搜索文件内容\n :run <命令> 在持久运行面板中执行命令\n Esc 取消\n\n 查询模式（普通模式 q，插入模式 Ctrl+Alt+Q，命令模式 :x — 开头 / 可省略）\n /s '文本' 选中光标后下一个匹配（循环）；空 /s 选中光标处单词\n /l N 跳转到第 N 行（光标到行首）\n /sa '文本' 选中全文所有匹配\n /sal 全选所有行\n /sl N 选中第 N 行；/sl A B 选中 A..B 行（顺序不限）；/sl A , B , … 选中离散行\n /rp <查找> <替换> 在选区内替换（需选择；多词加引号：/rp 'a b' 'c'）\n /add <文本> to #end|#start|#hm 在选中行追加/前插（需选择）\n /rm <文本> from #end|#start|#hm 在选中行移除前后缀（需选择）\n /set <模式> to #end|#start|#hm 对齐首处匹配（需选择）\n /c | /cs 清除选择\n /z [#hm|#end|#start] 在选中行添加光标（默认 #hm）\n /up 选择转大写（/upper 别名，需选择）\n /low 选择转小写（/lower 别名，需选择）\n /qal add <n> <q> 添加查询别名（如 /qal sw /rp ${1} ${2}；s/l/sa/sal/sl/rp/c 等为保留名）\n /qal rm <n> 删除别名\n /qal info <n> 显示别名\n /qal edit <n> <q> 更新别名\n /qal rnm <a> to <b> 重命名别名\n /qal list 列出别名\n <名称> [参数] 直接运行别名（无需 /）\n /undo 撤销\n /redo 重做\n /exit /quit /Esc 退出查询模式\n ; 链式执行多个命令\n ${1} ${2} 别名位置参数\n ${1:-默认值} 带默认值的别名参数\n ${1[]} 别名数组参数（所有剩余参数）\n\n 终端 / 运行面板\n Ctrl+T 切换终端面板\n :run <命令> 执行命令，打开持久运行面板\n mouse 单击面板聚焦，单击编辑器返回\n Esc（面板内） 运行：关闭 | 终端：返回编辑器\n drag \u{2500}\u{2500}\u{2500} row 调整面板高度".into());

    m.insert(
        "editor.hex_cannot_read".into(),
        "hex：无法读取文件：{}".into(),
    );

    m.insert(
        "editor.hex_mode_hint".into(),
        "Hex 模式 - 0-9/a-f 编辑字节  u=撤销  Ctrl+R=重做  :=命令  Esc=退出".into(),
    );

    m.insert("editor.hex_saved".into(), "十六进制已保存".into());

    m.insert(
        "editor.hex_unsaved".into(),
        "未保存的十六进制更改。使用 :w 保存，:q! 放弃。".into(),
    );

    m.insert(
        "editor.hex_unsaved_warning".into(),
        "Hex 更改未保存。使用 :w 保存，:q! 放弃。".into(),
    );

    m.insert(
        "editor.hint_command_actions".into(),
        "Enter=执行 Esc=取消".into(),
    );

    m.insert(
        "editor.hint_command_nav".into(),
        "cd|back|gos|gs|mkdir|touch|ne|tp|tp-add".into(),
    );

    m.insert(
        "editor.hint_file_finder_results".into(),
        "结果 ↑↓=导航 Enter=打开 Esc=关闭".into(),
    );

    m.insert("editor.hint_find".into(), "查找：{}".into());

    m.insert("editor.hint_find_prefix".into(), "查找 —".into());

    m.insert("editor.hint_gosc".into(), "转到：{}".into());

    m.insert("editor.hint_gosc_close".into(), "Esc=关闭".into());

    m.insert(
        "editor.hint_gosc_nav".into(),
        "目录 输入编号+Enter=导航 0=退出".into(),
    );

    m.insert("editor.hint_gosc_prefix".into(), "导航 —".into());

    m.insert("editor.hint_gs_close".into(), "Esc=关闭".into());

    m.insert("editor.hint_gs_done".into(), "完成：{} 个匹配".into());

    m.insert("editor.hint_gs_done_prefix".into(), "gs —".into());

    m.insert("editor.hint_gs_loading".into(), "gs — 正在搜索...".into());

    m.insert(
        "editor.hint_help".into(),
        " 帮助 — Esc/q=关闭 ↑↓=滚动".into(),
    );

    m.insert(
        "editor.hint_hex_actions".into(),
        "0-9/a-f=编辑 u=撤销 ^R=重做 :=cmd :w=保存 Esc=退出".into(),
    );

    m.insert(
        "editor.hint_hex_edit".into(),
        "Hex 编辑 - 输入十六进制字节".into(),
    );

    m.insert("editor.hint_insert".into(), " 插入模式 — Esc=普通 ^S=保存 ^Z=撤销 ^R=重做 ^D=多光标 ^F=查找 ^N=文件 ^P=粘贴 ^T=终端 ^G=命令 ^E=导航 ^W=可视 ^:=命令".into());

    m.insert("editor.hint_insert_msg".into(), " Esc=普通".into());

    m.insert(
        "editor.hint_insert_multi".into(),
        " Insert* — Esc=退出多光标 ^D=下一处".into(),
    );

    m.insert(
        "editor.hint_insert_multi_msg".into(),
        " Esc=退出多光标".into(),
    );

    m.insert("editor.hint_jump".into(), "跳转到行 —".into());

    m.insert(
        "editor.hint_jump_actions".into(),
        "Enter=跳转 Esc=取消".into(),
    );

    m.insert("editor.hint_gmode".into(), "g".into());

    m.insert("editor.hint_gmode_actions".into(), "t/T=标签 qt/aqt=关闭 数字=跳转 q=退出 u=撤销 r=重做 n=文件 m=目录 i=插入 v=可视 e=导航 Enter=运行 Esc=取消".into());

    m.insert(
        "editor.hint_create_actions".into(),
        "Enter=创建 Esc=取消".into(),
    );

    m.insert(
        "editor.gmode_unknown".into(),
        "未知 gmode 命令：'{}'".into(),
    );

    m.insert("editor.hint_normal".into(), " i=插入 v=可视 f=查找 b=文件 e=导航 t=终端 g=命令 x=保存 n=文件 m=目录 c=复制 Sh=选择 :命令 ?=帮助 0=退出".into());

    m.insert(
        "editor.term_run_usage".into(),
        "用法：! <命令>（在终端运行）".into(),
    );

    m.insert(
        "editor.hint_normal_msg".into(),
        " i=插入 v=可视 :命令 ?=帮助 0=退出".into(),
    );

    m.insert("editor.hint_query".into(), "查询：{}".into());

    m.insert("editor.hint_query_actions".into(), "操作：/s 下一个, /l 跳转, /sa 全部, /sal 所有行, /sl 行, /rp 替换, /add, /rm, /set, /z 光标, /up, /low, /qal, /c 清除".into());

    m.insert(
        "editor.query_z_usage".into(),
        "/z [#hm|#end|#start]（默认 #hm）".into(),
    );

    m.insert(
        "editor.query_cursors_added".into(),
        "已添加 {} 个光标".into(),
    );

    m.insert("editor.hint_query_prefix".into(), "/{} — {}".into());

    m.insert("editor.hint_results".into(), "结果：{}".into());

    m.insert(
        "editor.hint_results_so_far".into(),
        "结果：目前 {} 个".into(),
    );

    m.insert("editor.hint_run_empty".into(), "运行输出 —（空）".into());

    m.insert("editor.hint_run_entry".into(), "Enter=执行 Esc=取消".into());

    m.insert("editor.hint_run_output".into(), "运行输出 — 行".into());

    m.insert(
        "editor.hint_run_scroll".into(),
        ":=cmd ↑↓/j/k=滚动 Esc=关闭".into(),
    );

    m.insert("editor.hint_search".into(), "搜索：".into());

    m.insert(
        "editor.hint_search_actions".into(),
        "Enter=下一个 Shift+Enter=上一个 Esc=关闭".into(),
    );

    m.insert(
        "editor.hint_term_actions".into(),
        "行 PgUp/PgDn=滚动 Ctrl+Shift+C=复制 Ctrl+C=中断".into(),
    );

    m.insert("editor.hint_term_close".into(), "Esc/q=关闭".into());

    m.insert("editor.hint_term_exited".into(), "终端已退出".into());

    m.insert("editor.hint_term_lines".into(), "终端 —".into());

    m.insert("editor.hint_terminal".into(), "终端 - {} — {}".into());

    m.insert(
        "editor.hint_visual".into(),
        " hjkl=移动 Sh=选择 i=插入 u=撤销 r=重做 a=全选 c=复制 x=剪切 p=粘贴 Esc=退出".into(),
    );

    m.insert(
        "editor.hint_visual_no_msg".into(),
        " 可视模式 — hjkl/方向键移动 · Shift 选择 · a=全选 c=复制 x=剪切 y=复制行 p=粘贴".into(),
    );

    m.insert("editor.hover_title".into(), " 信息 ".into());

    m.insert(
        "editor.hyper_created".into(),
        "hyper 项目 '{}' 创建成功！".into(),
    );

    m.insert(
        "editor.hyper_creating".into(),
        "正在创建 hyper 项目 '{}'（C）...".into(),
    );

    m.insert(
        "editor.hyper_dir_exists".into(),
        "目录 '{}' 已存在。".into(),
    );

    m.insert(
        "editor.hyper_name_with_ext".into(),
        "项目名 '{}' 包含扩展名。请使用不含点号的名称。".into(),
    );

    m.insert(
        "editor.hyper_unsupported_lang".into(),
        "不支持的语言 '{}'。仅支持 'c'。".into(),
    );

    m.insert("editor.line_deleted".into(), "行已删除".into());

    m.insert("editor.line_selected".into(), "行已选中".into());

    m.insert("editor.line_yanked".into(), "行已复制".into());

    m.insert("editor.max_tabs".into(), "已达到最大标签页数（{}）".into());

    m.insert("editor.mkd_missing".into(), "mkd：缺少操作数".into());

    m.insert("editor.mkdir_missing".into(), "mkdir：缺少操作数".into());

    m.insert("editor.mkf_missing".into(), "mkf：缺少操作数".into());

    m.insert("editor.mode.command".into(), "[命令]".into());

    m.insert("editor.mode.finder".into(), "[查找器]".into());

    m.insert("editor.mode.gosc".into(), "[Gosc]".into());

    m.insert("editor.mode.help".into(), "[帮助]".into());

    m.insert("editor.mode.hex".into(), "[十六进制]".into());

    m.insert("editor.mode.insert".into(), "[插入]".into());

    m.insert("editor.mode.insert_star".into(), "[插入 *]".into());

    m.insert("editor.mode.jump".into(), "[跳转]".into());

    m.insert("editor.mode.normal".into(), "[普通]".into());

    m.insert("editor.mode.query".into(), "[查询]".into());

    m.insert("editor.mode.run".into(), "[运行]".into());

    m.insert("editor.mode.search".into(), "[搜索]".into());

    m.insert("editor.mode.term".into(), "[终端]".into());

    m.insert("editor.mode.visual".into(), "[可视]".into());

    m.insert(
        "editor.mode_cleared_cursors".into(),
        "已清除额外光标".into(),
    );

    m.insert("editor.mode_command".into(), "[命令]".into());

    m.insert("editor.mode_exited_multi".into(), "已退出多光标模式".into());

    m.insert("editor.mode_finder".into(), "[查找]".into());

    m.insert("editor.mode_encoding".into(), "[编码]".into());

    m.insert("editor.mode_git_diff".into(), "[Git差异]".into());

    m.insert("editor.git_no_diff".into(), "git：此文件无差异".into());

    m.insert(
        "editor.git_no_blame".into(),
        "git：此行为无 blame 信息".into(),
    );

    m.insert(
        "editor.git_disabled".into(),
        "git 集成已禁用（config git ON 开启）".into(),
    );

    m.insert("editor.git_quit_hint".into(), "Esc/q 关闭".into());

    m.insert("editor.mode_gosc".into(), "[导航]".into());

    m.insert("editor.mode_help".into(), "[帮助]".into());

    m.insert("editor.mode_hex".into(), "[十六进制]".into());

    m.insert("editor.mode_insert".into(), "[插入]".into());

    m.insert("editor.mode_insert_multi".into(), "[插入 *]".into());

    m.insert("editor.mode_jump".into(), "[跳转]".into());

    m.insert(
        "editor.mode_jump_range".into(),
        "第 {} 行超出范围（1-{}）".into(),
    );

    m.insert("editor.mode_normal".into(), "[普通]".into());

    m.insert("editor.mode_gmode".into(), "[Gmode]".into());

    m.insert("editor.mode_nfe".into(), "[Nfe]".into());

    m.insert(
        "editor.hint_nfe".into(),
        " hjkl/wasd/方向键移动 · Enter 打开 · i/v 编辑 · :=宽度 · Esc 返回".into(),
    );

    m.insert(
        "editor.hint_nfe_cmd_actions".into(),
        "Enter=应用 Esc=取消 · :i/:v 编辑".into(),
    );

    m.insert("editor.nfe_unknown".into(), "未知 nfe 命令：'{}'（试试 size full|def|N% | n <文件> | m <目录> | rmd <目录> | rmf <文件> | i | v）".into());

    m.insert(
        "editor.nfe_rm_usage".into(),
        "用法：rmd <目录> | rmf <文件>（相对侧栏根目录）".into(),
    );

    m.insert("editor.nfe_rmd_not_dir".into(), "不是目录：{}".into());

    m.insert(
        "editor.nfe_rmf_is_dir".into(),
        "是目录（请用 rmd）：{}".into(),
    );

    m.insert("editor.nfe_rm_not_exist".into(), "未找到：{}".into());

    m.insert("editor.nfe_removed".into(), "已删除：{}".into());

    m.insert("editor.nfe_rm_failed".into(), "删除失败：{}".into());

    m.insert(
        "editor.nfe_rm_has_unsaved".into(),
        "'{}' 有未保存的修改 — 请先保存".into(),
    );

    m.insert(
        "editor.nfe_new_usage".into(),
        "用法：n <文件> | m <目录>（相对侧栏根目录）".into(),
    );

    m.insert("editor.nfe_created_file".into(), "已创建文件：{}".into());

    m.insert("editor.nfe_created_dir".into(), "已创建目录：{}".into());

    m.insert("editor.nfe_create_exists".into(), "已存在：{}".into());

    m.insert("editor.nfe_create_failed".into(), "创建失败：{}".into());

    m.insert(
        "editor.nfe_size_usage".into(),
        "用法：size full|def|N%（收到 '{}'）".into(),
    );

    m.insert("editor.nfe_size_set".into(), "侧栏宽度：{}".into());

    m.insert("editor.nfe_size_default".into(), "默认（30 列）".into());

    m.insert("editor.nfe_size_full".into(), "全宽".into());

    m.insert("editor.mode_query".into(), "[查询]".into());

    m.insert("editor.mode_run".into(), "[运行]".into());

    m.insert("editor.mode_run_err".into(), "run：{}".into());

    m.insert("editor.mode_search".into(), "[搜索]".into());

    m.insert("editor.mode_term_copied".into(), "已复制 {} 个字符".into());

    m.insert("editor.mode_terminal".into(), "[终端]".into());

    m.insert("editor.mode_terminal_err".into(), "终端错误：{}".into());

    m.insert("editor.mode_visual".into(), "[可视]".into());

    m.insert(
        "editor.mode_visual_start".into(),
        "可视模式 - 用方向键选择，Esc 退出".into(),
    );

    m.insert(
        "editor.mode_zoom_hint".into(),
        "缩放：在终端模拟器中按 Ctrl++ / Ctrl+-".into(),
    );

    m.insert("editor.modified_marker".into(), " \u{25CF}".into());

    m.insert("editor.mouse_closed_all".into(), "已关闭所有标签页".into());

    m.insert("editor.nav_line_selected".into(), "已选择行".into());

    m.insert("editor.nav_selected_all".into(), "已全选".into());

    m.insert(
        "editor.nav_split_cursor".into(),
        "拆分光标（共 {} 个）".into(),
    );

    m.insert("editor.no_subdirs".into(), "没有子目录".into());

    m.insert("editor.no_teleports".into(), "没有传送保存点".into());

    m.insert("editor.nothing_redo".into(), "没有可重做的操作".into());

    m.insert("editor.nothing_undo".into(), "没有可撤销的操作".into());

    m.insert("editor.opened_file".into(), "已打开 {}".into());

    m.insert("editor.panel_executing".into(), "正在执行：".into());

    m.insert("editor.panel_lines".into(), "行".into());

    m.insert("editor.panel_run".into(), "运行：".into());

    m.insert(
        "editor.panel_run_hint".into(),
        "j/k 滚动，Esc/q 关闭".into(),
    );

    m.insert("editor.panel_run_output".into(), "运行输出".into());

    m.insert("editor.panel_terminal".into(), "终端".into());

    m.insert("editor.pattern_not_found".into(), "未找到模式：{}".into());

    m.insert(
        "editor.query_add_format".into(),
        "格式：/add <文本> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_add_position".into(),
        "位置必须是 #end、#start 或 #hm".into(),
    );

    m.insert(
        "editor.query_add_usage".into(),
        "/add <文本> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_added_text".into(),
        "已向 {} 行添加文本".into(),
    );

    m.insert(
        "editor.query_rm_usage".into(),
        "/rm <文本> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_rm_format".into(),
        "格式：/rm <文本> from #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_removed_text".into(),
        "已从 {} 行移除文本，跳过 {}".into(),
    );

    m.insert(
        "editor.query_alias_not_found".into(),
        "未找到别名：{}".into(),
    );

    m.insert("editor.query_empty_pattern".into(), "空的查找模式".into());

    m.insert("editor.query_l_usage".into(), "/l <行号> — 跳转到行".into());

    m.insert("editor.query_lowercased".into(), "已转为小写".into());

    m.insert(
        "editor.query_no_matches".into(),
        "未找到 '{}' 的匹配".into(),
    );

    m.insert(
        "editor.query_no_pattern_matches".into(),
        "没有包含 '{}' 的行".into(),
    );

    m.insert(
        "editor.query_qal_add_usage".into(),
        "/qal add <名称> <查询>".into(),
    );

    m.insert("editor.query_qal_added".into(), "已添加别名 '{}'".into());

    m.insert(
        "editor.query_qal_edit_usage".into(),
        "/qal edit <名称> <新查询>".into(),
    );

    m.insert(
        "editor.query_qal_exists".into(),
        "别名 '{}' 已存在（请用 /qal edit）".into(),
    );

    m.insert(
        "editor.query_qal_help".into(),
        "/qal：add|rm|info|edit|rnm|list|run".into(),
    );

    m.insert(
        "editor.query_qal_info_usage".into(),
        "/qal info <名称>".into(),
    );

    m.insert("editor.query_qal_list".into(), "别名：{}".into());

    m.insert("editor.query_qal_no_aliases".into(), "没有查询别名".into());

    m.insert(
        "editor.query_qal_not_found".into(),
        "未找到别名 '{}'".into(),
    );

    m.insert("editor.query_qal_removed".into(), "已移除别名 '{}'".into());

    m.insert("editor.query_qal_reserved".into(), "保留名称：'{}'".into());

    m.insert(
        "editor.query_qal_renamed".into(),
        "已将 '{}' 重命名为 '{}'".into(),
    );

    m.insert("editor.query_qal_rm_usage".into(), "/qal rm <名称>".into());

    m.insert(
        "editor.query_qal_rnm_usage".into(),
        "/qal rnm <旧名> to <新名>".into(),
    );

    m.insert("editor.query_qal_updated".into(), "已更新别名 '{}'".into());

    m.insert(
        "editor.query_replace_format".into(),
        "格式：/p <查找> to <替换>".into(),
    );

    m.insert(
        "editor.query_replace_usage".into(),
        "/p <查找> to <替换>".into(),
    );

    m.insert("editor.query_replaced_count".into(), "已替换 {} 处".into());

    m.insert(
        "editor.query_rp_usage".into(),
        "/rp <查找> <替换> — 在选区内替换".into(),
    );

    m.insert(
        "editor.query_s_usage".into(),
        "/s \"文本\" — 选中下一个匹配（空 /s = 选中光标处单词）".into(),
    );

    m.insert(
        "editor.query_sa_usage".into(),
        "/sa \"文本\" — 选中所有匹配".into(),
    );

    m.insert(
        "editor.query_selected_all_lines".into(),
        "已选择所有行".into(),
    );

    m.insert("editor.query_selected_line".into(), "已选择第 {} 行".into());

    m.insert("editor.query_selected_lines".into(), "已选择 {} 行".into());

    m.insert(
        "editor.query_selected_lines_range".into(),
        "已选择第 {}-{} 行".into(),
    );

    m.insert(
        "editor.query_selected_next".into(),
        "已选中下一个 '{}'".into(),
    );

    m.insert(
        "editor.query_selected_occurrences".into(),
        "已选中 {} 处 '{}' 的匹配".into(),
    );

    m.insert("editor.query_selected_word".into(), "已选中单词".into());

    m.insert("editor.query_selection_cleared".into(), "已清除选择".into());

    m.insert(
        "editor.query_selection_required".into(),
        "需要先选择（用 /s、/sa、/sl 或 /sal 选择）".into(),
    );

    m.insert(
        "editor.query_set_alignment".into(),
        "已设置 {} 行对齐".into(),
    );

    m.insert(
        "editor.query_set_format".into(),
        "格式：/set <模式> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_set_usage".into(),
        "/set <模式> to #end|#start|#hm".into(),
    );

    m.insert(
        "editor.query_sl_usage".into(),
        "/sl <行号> [行号] | <行号> , ... — 选择行".into(),
    );

    m.insert("editor.query_unknown".into(), "未知查询：{}".into());

    m.insert("editor.query_uppercased".into(), "已转为大写".into());

    m.insert("editor.redo".into(), "重做".into());

    m.insert("editor.rmd_missing".into(), "rmd：缺少操作数".into());

    m.insert("editor.rmf_missing".into(), "rmf：缺少操作数".into());

    m.insert("editor.run_exited".into(), "进程已退出，代码 {}".into());

    m.insert("editor.run_missing".into(), "run：缺少命令".into());

    m.insert("editor.run_terminated".into(), "进程已终止".into());

    m.insert("editor.save_auto_error".into(), "自动保存错误：{}".into());

    m.insert("editor.save_failed".into(), "保存失败：{}".into());

    m.insert("editor.encoding_changed".into(), "编码已更改".into());

    m.insert("editor.enc_title".into(), "编码".into());

    m.insert(
        "editor.enc_hint".into(),
        "↑↓ 选择 · 回车确认 · Esc 取消".into(),
    );

    m.insert("editor.hint_encoding".into(), "编码：".into());

    m.insert(
        "editor.encoding_reload_error".into(),
        "重新加载失败：{}".into(),
    );

    m.insert(
        "editor.encoding_applied_on_save".into(),
        "保存时应用".into(),
    );

    m.insert("editor.save_prompt".into(), "保存文件 [Y|N]？".into());

    m.insert("editor.saved".into(), "已保存 \u{2713}".into());

    m.insert("editor.saved_full".into(), " 已保存 ✓".into());

    m.insert("editor.saved_marker".into(), " \u{2713}".into());

    m.insert("editor.scratch".into(), "[草稿]".into());

    m.insert("editor.scratch_label".into(), "[临时]".into());

    m.insert(
        "editor.search_pattern_not_found".into(),
        "未找到模式：{}".into(),
    );

    m.insert("editor.selected_all".into(), "已全选".into());

    m.insert("editor.status_modified".into(), "已修改".into());

    m.insert("editor.status_modified_tag".into(), " \u{25cf}".into());

    m.insert("editor.status_saved".into(), "已保存".into());

    m.insert("editor.status_saved_tag".into(), " \u{2713}".into());

    m.insert("editor.syntax_off".into(), "语法高亮：关".into());

    m.insert("editor.syntax_on".into(), "语法高亮：开".into());

    m.insert("editor.tab_cannot_open".into(), "无法打开 {}".into());

    m.insert("editor.tab_error_reading".into(), "读取 {} 出错：{}".into());

    m.insert(
        "editor.tab_max_reached".into(),
        "已达到最大标签页数（{}）".into(),
    );

    m.insert("editor.tab_opened".into(), "已打开 {}".into());

    m.insert("editor.term_title".into(), "终端".into());

    m.insert("editor.tmpl_last_saved".into(), "上次保存：{}".into());

    m.insert(
        "editor.tmpl_recover_prompt".into(),
        "恢复未保存的更改？[Y/N]：".into(),
    );

    m.insert(
        "editor.tmpl_recovery_found".into(),
        "为 {} 找到崩溃恢复文件：".into(),
    );

    m.insert("editor.touch_missing".into(), "touch：缺少操作数".into());

    m.insert("editor.tp_add_missing".into(), "tp-add：缺少名称".into());

    m.insert("editor.undo".into(), "撤销".into());

    m.insert("editor.unknown_cmd".into(), "未知命令：{}".into());

    m.insert("editor.untitled".into(), "未命名".into());

    m.insert("editor.untitled_label".into(), "未命名".into());

    m.insert(
        "editor.visual_hint".into(),
        "可视模式 \u{2014} 方向键选择，Esc退出".into(),
    );

    m.insert(
        "editor.welcome.title".into(),
        "一个用Rust构建的快速、基于终端的代码编辑器。".into(),
    );

    m.insert(
        "editor.zoom_hint".into(),
        "缩放：在终端模拟器中按 Ctrl++ / Ctrl+-".into(),
    );
}
