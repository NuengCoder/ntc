use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.fake.title".into(), "fake - ສ້າງຂໍ້ມູນຈຳລອງ/ສຸ່ມ".into());

    m.insert(
        "help.fake.gen".into(),
        "  fake gen <ຊະນິດ> [-n N] [-f ຮູບແບບ] [-s ຊີດ] [-o ໄຟລ]   ສ້າງແຖວ".into(),
    );

    m.insert(
        "help.fake.gen_options".into(),
        "  ຮູບແບບ: json | txt | sql | csv | xlsx (ຄ່າເລີ່ມຕົ້ນ: ຕາຕະລາງ txt)".into(),
    );

    m.insert(
        "help.fake.gen_group".into(),
        "  fake gen -g <ກຸ່ມ> ...                          ສ້າງຈາກກຸ່ມທີ່ບັນທຶກໄວ້".into(),
    );

    m.insert(
        "help.fake.gen_unique".into(),
        "  -u, --unique [ຖັນ]                              ແຖວ ຫຼື ຖັນທີ່ບໍ່ຊ້ຳ".into(),
    );

    m.insert("help.fake.gen_locale".into(), "  -l, --locale <ລະຫັດ>                            ຂໍ້ມູນຕາມພາສາ (en_US en_GB zh_CN th_TH lo_LA)".into());

    m.insert(
        "help.fake.types".into(),
        "  fake types                                     ສະແດງຊະນິດທັງໝົດ".into(),
    );

    m.insert(
        "help.fake.types_search".into(),
        "  fake types -s <ຮູບແບບ>                          ຄົ້ນຫາຊະນິດແບບ fuzzy".into(),
    );

    m.insert(
        "help.fake.locales".into(),
        "  fake locales                                    ສະແດງລະຫັດ --locale".into(),
    );

    m.insert(
        "help.fake.group_add".into(),
        "  fake group add <ຊື່> [ນາມແຝງ as ຊະນິດ, ...]     ສ້າງກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_edit".into(),
        "  fake group edit <ຊື່> [ນາມແຝງ as ຊະນິດ, ...]    ແທນຖັນຂອງກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_rm".into(),
        "  fake group rm <ຊື່>                            ລຶບກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_rnm".into(),
        "  fake group rnm <ເກົ່າ> <ໃໝ່>                    ປ່ຽນຊື່ກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_ls".into(),
        "  fake group ls [ຮູບແບບ]                          ສະແດງກຸ່ມ (wildcard ເຊັ່ນ u*)".into(),
    );

    m.insert(
        "help.fake.group_info".into(),
        "  fake group info <ຊື່>                          ສະແດງຖັນຂອງກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_clear".into(),
        "  fake group clear [-f]                           ລຶບທຸກກຸ່ມ".into(),
    );

    m.insert(
        "help.fake.group_export".into(),
        "  fake group export <ຊື່>|--all [-o ໄຟລ]        ສົ່ງອອກກຸ່ມເປັນ .ntc_fake".into(),
    );

    m.insert(
        "help.fake.group_import".into(),
        "  fake group import <ໄຟລ> [--force] [--prefix p]  ໂຫຼດກຸ່ມຈາກໄຟລ".into(),
    );

    m.insert(
        "help.fake.group_ref".into(),
        "  ຖັນ: 'ນາມແຝງ as ຊະນິດ', 'ຊະນິດ !' = ບໍ່ຊ້ຳ, 'ນາມແຝງ as ref:ກຸ່ມ.ຖັນ'".into(),
    );

    m.insert(
        "help.fake.help".into(),
        "  fake help                                      ສະແດງຄວາມຊ່ວຍເຫຼືອນີ້".into(),
    );

    m.insert(
        "help.fake.example1".into(),
        "fake gen name,email,city -n 5 ## ຕາຕະລາງ txt".into(),
    );

    m.insert(
        "help.fake.example2".into(),
        "fake gen uuid,age -f json -o rows.json".into(),
    );

    m.insert(
        "help.fake.example3".into(),
        "fake group add users [username as name, year as age]".into(),
    );

    m.insert(
        "help.fake.example4".into(),
        "fake gen -g users -f sql -n 10 ## INSERT INTO users ...".into(),
    );
}
