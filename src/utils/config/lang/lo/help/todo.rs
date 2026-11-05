use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.todo.add".into(),
        "todo add \"name\" ## ເພີ່ມວຽກໃໝ່".into(),
    );

    m.insert(
        "help.todo.add_at".into(),
        "todo add \"name\" -> N ## ໃສ່ວຽກທີ່ຕຳແໜ່ງ N".into(),
    );

    m.insert(
        "help.todo.clear".into(),
        "todo clear ## ລຶບວຽກທັງໝົດ (ພ້ອມຢືນຢັນ)".into(),
    );

    m.insert(
        "help.todo.info".into(),
        "todo info [N] ## ສະແດງວຽກທັງໝົດ ຫຼື ລາຍລະອຽດວຽກ N".into(),
    );

    m.insert("help.todo.rm".into(), "todo rm <N> ## ລຶບວຽກ N".into());

    m.insert(
        "help.todo.show".into(),
        "todo ## ສະແດງວຽກທັງໝົດ (ຈາກ future_todo.txt ໃນ CWD)".into(),
    );

    m.insert(
        "help.todo.move".into(),
        "todo move <N> <M> ## ຍ້າຍລາຍການທີ່ N ໄປຕຳແໜ່ງ M".into(),
    );

    m.insert(
        "help.todo.replace".into(),
        "todo replace <N> <M> ## ສະຫຼັບລາຍການທີ່ N ກັບ M".into(),
    );

    m.insert(
        "help.todo.example4".into(),
        "todo mv 5 1 ## ຍ້າຍລາຍການທີ່ 5 ຂຶ້ນເທິງສຸດ".into(),
    );

    m.insert(
        "help.todo.example5".into(),
        "todo rp 5 1 ## ສະຫຼັບລາຍການທີ່ 5 ກັບ 1".into(),
    );

    m.insert("help.todo.title".into(), "ຄຳສັ່ງ TODO:".into());

    m.insert(
        "help.todo.done".into(),
        "todo done <N> ## ໝາຍວຽກ N ວ່າສຳເລັດ".into(),
    );

    m.insert(
        "help.todo.undo".into(),
        "todo undo <N> ## ເປີດວຽກ N ໃໝ່".into(),
    );

    m.insert(
        "help.todo.edit".into(),
        "todo edit <N> \"name\" ## ປ່ຽນຊื่อວຽກ N".into(),
    );

    m.insert("help.todo.example1".into(), "todo ## ສະແດງວຽກທັງໝົດ".into());

    m.insert(
        "help.todo.example2".into(),
        "todo add \"Fix bug\" ## ເພີ່ມວຽກໃໝ່".into(),
    );

    m.insert("help.todo.example3".into(), "todo rm 1 ## ລຶບວຽກ #1".into());
}
