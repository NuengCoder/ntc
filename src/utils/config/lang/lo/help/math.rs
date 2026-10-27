use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.math.eval".into(),
        "math <expr> ## ຄຳນວນພົດການຄິດໄລ່".into(),
    );

    m.insert(
        "help.math.file".into(),
        "math file.ntc.math ## ຄອມໄພ້ ແລະ ເຮັດວຽກໄຟລ໌ math".into(),
    );

    m.insert(
        "help.math.fun".into(),
        "math fun add/rm/edit/info/ls ## ຈັດການຟັງຊັນ math ທີ່ກຳນົດເອງ".into(),
    );

    m.insert(
        "help.math.timer".into(),
        "math timer [sec] ## lap timer ຫຼື ນັບຖອຍຫຼັງພ້ອມສຽງ".into(),
    );

    m.insert("help.math.title".into(), "ຄຳສັ່ງ MATH:".into());

    m.insert(
        "help.math.example1".into(),
        "math 3+4*5 ## ຄຳນວນພົດ arithmetic".into(),
    );

    m.insert(
        "help.math.example2".into(),
        "math sin(PI/2) ## ຄິດໄລ່ດ້ວຍຟັງຊັນ".into(),
    );

    m.insert(
        "help.math.example3".into(),
        "math timer 30 ## ເລີ່ມນັບຖອຍຫຼັງ 30 ວິນາທີ".into(),
    );

    m.insert(
        "help.math.example4".into(),
        "math fun add myfn \"x+1\" ## ເພີ່ມ custom function".into(),
    );

    m.insert(
        "help.math.example5".into(),
        "math fun ls ## ລາຍຊື່ custom functions".into(),
    );

    m.insert(
        "help.math.example6".into(),
        "math calc.ntc.math ## ເຮັດວຽກໄຟລ໌ math".into(),
    );
}
