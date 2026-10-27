use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.alias.add".into(),
        "ral add <name> \"<cmd>\" ## ສ້າງ alias".into(),
    );

    m.insert(
        "help.alias.add_array".into(),
        "ral add <name>(x[],y) \"<cmd>\" ## alias ແບບມີ array params".into(),
    );

    m.insert(
        "help.alias.add_rest".into(),
        "ral add <name>(x) \"echo $*{x}\" ## ຈັບໜຶ່ງອາກິວເມັນ (ເກັບຊ່ອງວ່າງ, ຕັດຄຳເວົ້າ)".into(),
    );

    m.insert(
        "help.alias.add_defaults".into(),
        "ral add <name>({x=a,e=txt}) \"<cmd>\" ## alias ແບບມີຄ່າເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.alias.add_param".into(),
        "ral add <name>(x) \"<cmd>\" ## ສ້າງ parameterised alias ດ້ວຍ $x".into(),
    );

    m.insert(
        "help.alias.cls".into(),
        "ral cls ## ລຶບ alias ທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.alias.edit".into(),
        "ral edit <name> \"<cmd>\" ## ອັບເດດ alias".into(),
    );

    m.insert(
        "help.alias.export_all".into(),
        "ral export --all <name> ## ສົ່ງອອກ aliases ທັງໝົດໄປຍັງ <name>.ntc.ral".into(),
    );

    m.insert(
        "help.alias.export_select".into(),
        "ral export --select <name> ## ເລືອກ aliases ເພື່ອສົ່ງອອກ".into(),
    );

    m.insert(
        "help.alias.import".into(),
        "ral import <file> ## ນຳເຂົ້າ aliases ຈາກໄຟລ໌ .ntc.ral".into(),
    );

    m.insert(
        "help.alias.info".into(),
        "ral info <name> [--edit] ## ສະແດງລາຍລະອຽດ alias (--edit: ສຳເນົາຄຳສັ່ງແກ້ໄຂໃສ່ clipboard)"
            .into(),
    );

    m.insert(
        "help.alias.list".into(),
        "ral list ## ສະແດງ aliases ທັງໝົດ".into(),
    );

    m.insert(
        "help.alias.make".into(),
        "ral make [name] ## ສ້າງ alias template ໃນ CWD".into(),
    );

    m.insert(
        "help.alias.hintn".into(),
        "ral hintn add|rm|rnm|info|ls ## ຈັດການລາຍການຄ່າທີ່ຕັ້ງຊື່ໄວ້".into(),
    );

    m.insert(
        "help.alias.rm".into(),
        "ral rm <name>[, ...] ## ລຶບ alias".into(),
    );

    m.insert(
        "help.alias.rnm".into(),
        "ral rnm <old> to <new> ## ປ່ຽນຊື່ alias".into(),
    );

    m.insert(
        "help.alias.run".into(),
        "run <alias> ## ເຮັດວຽກ alias ດ້ວຍ 'run'".into(),
    );

    m.insert(
        "help.alias.run_direct".into(),
        "<alias> ## ເຮັດວຽກ alias ໂດຍກົງ".into(),
    );

    m.insert(
        "help.alias.run_param".into(),
        "<alias>(<arg>) ## ເຮັດວຽກ parameterised alias".into(),
    );

    m.insert("help.alias.title".into(), "ຄຳສັ່ງ RUN ALIAS:".into());

    m.insert(
        "help.alias.example1".into(),
        "ral add build \"cargo build\" ## ສ້າງ alias ງ່າຍໆ".into(),
    );

    m.insert(
        "help.alias.example2".into(),
        "ral add greet(x) \"echo Hello $x\" ## alias ແບບ parameterised".into(),
    );

    m.insert(
        "help.alias.example3".into(),
        "ral add v \"view --care -t -d ${x:-1}\" ## alias ພ້ອມຄ່າເລີ່ມຕົ້ນ".into(),
    );

    m.insert(
        "help.alias.example4".into(),
        "ral list ## ສະແດງ aliases ທັງໝົດ".into(),
    );

    m.insert(
        "help.alias.example5".into(),
        "ral export --all myaliases ## ສົ່ງອອກ aliases".into(),
    );

    m.insert(
        "help.alias.example6".into(),
        "build ## ເຮັດວຽກ alias ໂດຍກົງ".into(),
    );

    m.insert(
        "help.alias.example7".into(),
        "ral hintn add port [8080,5050] ## ລາຍການທີ່ຕັ້ງຊື່ໄວ້".into(),
    );
}
