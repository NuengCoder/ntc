use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ral.title".into(), "ຄຳສັ່ງ RUN ALIAS:".into());

    m.insert(
        "help.ral.add".into(),
        "ral add <name> \"<cmd>\" ## ສ້າງ alias".into(),
    );

    m.insert(
        "help.ral.add_param".into(),
        "ral add <name>(x) \"<cmd>\" ## ສ້າງ parameterised alias ດ້ວຍ ${x}".into(),
    );

    m.insert(
        "help.ral.env_ref".into(),
        "ral add <name> \"<cmd>\" ## $NAME ດ່ຽວແມ່ນ environment variable, ໃຊ້ ${x} ສຳລັບພາຣາມິເຕີ".into(),
    );

    m.insert(
        "help.ral.add_defaults".into(),
        "ral add <name>({x=a,e=txt}) \"<cmd>\" ## alias ແບບມີຄ່າເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.ral.add_array".into(),
        "ral add <name>(x[],y) \"<cmd>\" ## alias ແບບມີ array params".into(),
    );

    m.insert(
        "help.ral.add_rest".into(),
        "ral add <name>(x) \"echo $*{x}\" ## ຈັບໜຶ່ງອາກິວເມັນ (ເກັບຊ່ອງວ່າງ, ຕັດຄຳເວົ້າ)".into(),
    );

    m.insert(
        "help.ral.edit".into(),
        "ral edit <name> \"<cmd>\" ## ອັບເດດ alias".into(),
    );

    m.insert(
        "help.ral.rnm".into(),
        "ral rnm <old> to <new> ## ປ່ຽນຊື່ alias".into(),
    );

    m.insert(
        "help.ral.rm".into(),
        "ral rm <name>[, ...] ## ລຶບ alias".into(),
    );

    m.insert(
        "help.ral.list".into(),
        "ral list ## ສະແດງ aliases ທັງໝົດ".into(),
    );

    m.insert(
        "help.ral.info".into(),
        "ral info <name> [--edit] ## ສະແດງລາຍລະອຽດ alias (--edit: ສຳເນົາຄຳສັ່ງແກ້ໄຂໃສ່ clipboard)"
            .into(),
    );

    m.insert(
        "help.ral.cls".into(),
        "ral cls ## ລຶບ alias ທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.ral.export_all".into(),
        "ral export --all <name> ## ສົ່ງອອກ aliases ທັງໝົດໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert(
        "help.ral.export_select".into(),
        "ral export --select <name> ## ເລືອກ aliases ເພື່ອສົ່ງອອກ".into(),
    );

    m.insert(
        "help.ral.import".into(),
        "ral import <file> ## ນຳເຂົ້າ aliases ຈາກໄຟລ໌ .ntc.ral".into(),
    );

    m.insert(
        "help.ral.make".into(),
        "ral make [name] ## ສ້າງ alias template ໃນ CWD".into(),
    );

    m.insert(
        "help.ral.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## ຈັດການລາຍການຄ່າທີ່ຕັ້ງຊື່ໄວ້".into(),
    );

    m.insert(
        "help.ral.example1".into(),
        "ral add build \"cargo build\" ## ສ້າງ alias ງ່າຍໆ".into(),
    );

    m.insert(
        "help.ral.example2".into(),
        "ral add greet(x) \"echo Hello ${x}\" ## alias ແບບ parameterised".into(),
    );

    m.insert(
        "help.ral.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## alias ພ້ອມຄ່າເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.ral.example4".into(),
        "ral list ## ສະແດງ aliases ທັງໝົດ".into(),
    );

    m.insert(
        "help.ral.example5".into(),
        "ral export --all myaliases ## ສົ່ງອອກ aliases".into(),
    );

    m.insert(
        "help.ral.example6".into(),
        "ral hintn add port [8080,5050] ## ລາຍການທີ່ຕັ້ງຊື່ໄວ້".into(),
    );

    m.insert(
        "help.ral.install".into(),
        "ral install ## ຕິດຕັ້ງ run aliases ຈາກແຄັດຕາລັອກ GitHub ຂອງ ntc".into(),
    );
}
