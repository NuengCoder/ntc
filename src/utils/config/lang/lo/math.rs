use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "math.builtin_agg".into(),
        "  agg:  sum, min, max, avg".into(),
    );

    m.insert(
        "math.builtin_constants".into(),
        "  constants: PI, E, PHI, TAU".into(),
    );

    m.insert(
        "math.builtin_conv".into(),
        "  conv: toBinary, toHex, to8/toOctal, toDecimal, toHB".into(),
    );

    m.insert("math.builtin_header".into(), "ຟັງຊັນ built-in:".into());

    m.insert(
        "math.builtin_inv_trig".into(),
        "  inv:  arcsin/asin, arccos/acos, arctan/atan, arccot/acot, arcsec/asec, arccsc/acsc"
            .into(),
    );

    m.insert(
        "math.builtin_math".into(),
        "  math: sqrt, pow(x,y), abs, floor, ceil, round, ln/log, log2, log10".into(),
    );

    m.insert(
        "math.builtin_print".into(),
        "  print: ພິມຄ່າ (ຄື Python)".into(),
    );

    m.insert("math.builtin_rand".into(), "  rand: rand(min,max)".into());

    m.insert(
        "math.builtin_trig".into(),
        "  trig: sin, cos, tan, cot, sec, csc".into(),
    );

    m.insert("math.compiled_header".into(), "ຄອມໄພ້ແລ້ວ: {}".into());

    m.insert("math.countdown".into(), "ນັບຖອຍຫຼັງ: {} ວິນາທີ".into());

    m.insert("math.error".into(), "  {} ທີ່ແຖວ {}: {}".into());

    m.insert(
        "math.example_add".into(),
        "  math 3+4*5                           → 23".into(),
    );

    m.insert(
        "math.example_fun_call".into(),
        "  math square(5)                        → 25".into(),
    );

    m.insert(
        "math.example_fun_def".into(),
        "  math fun add square(x) = x^2".into(),
    );

    m.insert(
        "math.example_hexbin".into(),
        "  math 0xFF + 0b1010                   → 265".into(),
    );

    m.insert(
        "math.example_rand".into(),
        "  math rand(1,100)                      → 42".into(),
    );

    m.insert(
        "math.example_sin".into(),
        "  math sin(PI/2)                       → 1".into(),
    );

    m.insert(
        "math.example_sqrt".into(),
        "  math sqrt(144)                        → 12".into(),
    );

    m.insert(
        "math.example_timer".into(),
        "  math timer 10                        → ນັບຖອຍຫຼັງ 10s".into(),
    );

    m.insert(
        "math.example_tobinary".into(),
        "  math toBinary(255)                    → 11111111".into(),
    );

    m.insert(
        "math.example_tohb".into(),
        "  math toHB(1048576)                    → 1.00 MB".into(),
    );

    m.insert(
        "math.example_tohex".into(),
        "  math toHex(255)                       → FF".into(),
    );

    m.insert("math.examples_header".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "math.exec_end".into(),
        "\u{255A}\u{2550} ຈົບ: {} \u{2550}\u{255D}".into(),
    );

    m.insert("math.failed_read".into(), "ອ່ານ '{}' ບໍ່ສຳເລັດ: {}".into());

    m.insert("math.file_not_found".into(), "ບໍ່ພົບໄຟລ໌: '{}'".into());

    m.insert("math.fun_commands_header".into(), "ຄຳສັ່ງ math fun:".into());

    m.insert("math.fun_list_header".into(), "ຟັງຊັນ math ທີ່ກຳນົດເອງ:".into());

    m.insert(
        "math.fun_none".into(),
        " ບໍ່ມີຟັງຊັນທີ່ກຳນົດເອງ. ໃຊ້ 'math fun add <name>(<params>) = <body>'".into(),
    );

    m.insert("math.fun_removed".into(), " ລຶບແລ້ວ: {}".into());

    m.insert(
        "math.help_expr".into(),
        "  math <expression>       ຄຳນວນພົດ".into(),
    );

    m.insert(
        "math.help_file".into(),
        "  math <file>.ntc.math    ຄອມໄພ້ ແລະ ເຮັດວຽກໄຟລ໌ math".into(),
    );

    m.insert(
        "math.help_fun_add".into(),
        "  math fun add <name>(<params>) = <body>   ກຳນົດຟັງຊັນ".into(),
    );

    m.insert(
        "math.help_fun_edit".into(),
        "  math fun edit <name> = <body>  ອັບເດດຟັງຊັນ".into(),
    );

    m.insert(
        "math.help_fun_info".into(),
        "  math fun info <name>    ສະແດງລາຍລະອຽດຟັງຊັນ".into(),
    );

    m.insert(
        "math.help_fun_ls".into(),
        "  math fun ls             ລາຍຊື່ຟັງຊັນທີ່ກຳນົດເອງທັງໝົດ".into(),
    );

    m.insert(
        "math.help_fun_rm".into(),
        "  math fun rm <name>      ລຶບຟັງຊັນ".into(),
    );

    m.insert(
        "math.help_func_call".into(),
        "  math <func>(<args>)     ເອີ້ນຟັງຊັນ built-in ຫຼື ທີ່ກຳນົດເອງ".into(),
    );

    m.insert(
        "math.help_timer".into(),
        "  math timer [seconds]    lap timer ຫຼື ນັບຖອຍຫຼັງພ້ອມສຽງ".into(),
    );

    m.insert("math.help_title".into(), "Math Module - ການໃຊ້ງານ:".into());

    m.insert(
        "math.is_directory".into(),
        "'{}' ເປັນໄດເຣັກທໍຣີ, ບໍ່ແມ່ນໄຟລ໌".into(),
    );

    m.insert("math.lap_time".into(), "Lap: {} seconds".into());

    m.insert("math.time_remaining".into(), "ເວລາທີ່ຍັງເຫຼືອ: {}".into());

    m.insert("math.timer_started".into(), "ເລີ່ມ timer...".into());

    m.insert(
        "math.timer_usage".into(),
        "Usage: math timer <seconds>".into(),
    );

    m.insert("math.times_up".into(), "ໝົດເວລາແລ້ວ!".into());
}
