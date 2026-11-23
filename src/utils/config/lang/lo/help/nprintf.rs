use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nprintf.title".into(), r#"ຄຳສັ່ງ NPRINTF:"#.into());
    m.insert(
        "help.nprintf.usage".into(),
        r#"nprintf <ຮູບແບບ> [ອາກິວເມັນ...] ## ຜົນອອກແບບຈັດຮູບແບບສະຕາຍ Bash"#.into(),
    );
    m.insert("help.nprintf.conversions".into(), r#"  ຕົວແປງ"#.into());
    m.insert("help.nprintf.specs".into(), r#"  %s ຂໍ້ຄວາມ · %d/%i ຈຳນວນເຕັມມີເຄື່ອງໝາຍ · %u ບໍ່ມີເຄື່ອງໝາຍ · %o ຖານແປດ · %x/%X ຖານສິບຫົກ · %c ຕົວອັກສອນທຳອິດ"#.into());
    m.insert("help.nprintf.specs2".into(), r#"  %b ແປ escape ໃນອາກິວເມັນ · %q ໃສ່ quote ແບບ shell · %f/%e/%g/%a ທົດສະນິຍົມ · %% ເຄື່ອງໝາຍເປີເຊັນ"#.into());
    m.insert(
        "help.nprintf.flags".into(),
        r#"  ແຟລັກ, ຄວາມກວ້າງ, ຄວາມແມ່ນຍຳ"#.into(),
    );
    m.insert(
        "help.nprintf.flags_detail".into(),
        r#"  %-8s ຊິດຊ້າຍ · %05d ເຕີມສູນ · %+d ສະແດງເຄື່ອງໝາຍ · %.2f ຄວາມແມ່ນຍຳ · %*s ຄວາມກວ້າງຈາກອາກິວເມັນ"#
            .into(),
    );
    m.insert(
        "help.nprintf.escapes".into(),
        r#"  ESCAPE (ແຕ່ລະຕົວເລີ່ມດ້ວຍ backslash)"#.into(),
    );
    m.insert("help.nprintf.escapes_detail".into(), r#"  \n ຂຶ້ນແຖວໃໝ່ · \t tab · \r carriage return · \\ backslash · \" ເຄື່ອງໝາຍຄຳເວົ້າ · \xHH ຖານສິບຫົກ · \0NNN ຖານແປດ · \c ຢຸດຜົນອອກ"#.into());
    m.insert("help.nprintf.newline".into(), r#"  ເປັນຫຍັງຕ້ອງ \n?"#.into());
    m.insert(
        "help.nprintf.newline_detail".into(),
        r#"  nprintf ບໍ່ເພີ່ມຫຍັງຫຼັງຮູບແບບ. ໃສ່ \n ຕອນທ້າຍເມື່ອຕ້ອງການຂຶ້ນແຖວໃໝ່; ຖ້າບໍ່ໃສ່ ຜົນອອກຈະຕໍ່ໃນແຖວເດີມ."#.into(),
    );
    m.insert("help.nprintf.quoting".into(), r#"  ການໃສ່ quote"#.into());
    m.insert(
        "help.nprintf.quoting_detail".into(),
        r#"  ໃສ່ຮູບແບບໃນເຄື່ອງໝາຍຄຳເວົ້າເປີ້ນຄູ່ ເພື່ອໃຫ້ຊ່ອງວ່າງ ແລະ escape ຢູ່ໃນອາກິວເມັນດຽວ."#.into(),
    );
    m.insert(
        "help.nprintf.reuse".into(),
        r#"  ຮູບແບບຈະຖືກໃຊ້ຄືນຕອນຍັງມີອາກິວເມັນເຫຼືອ; %s ທີ່ຂາດຈະເປັນຄ່າຫວ່າງ ແລະ ຕົວເລກທີ່ຂາດຈະເປັນ 0."#.into(),
    );
    m.insert(
        "help.nprintf.example1".into(),
        r#"nprintf "hello %s\n" world ## ໜຶ່ງແຖວ; \n ຈົບແຖວ"#.into(),
    );
    m.insert(
        "help.nprintf.example2".into(),
        r#"nprintf "%s has %d items\n" cart 3 ## ຂໍ້ຄວາມ ແລະ ຈຳນວນເຕັມ"#.into(),
    );
    m.insert(
        "help.nprintf.example3".into(),
        r#"nprintf "%-8s|%05d|%.2f\n" left 42 3.14159 ## ຊິດຊ້າຍ, ເຕີມສູນ, ຄວາມແມ່ນຍຳ"#.into(),
    );
    m.insert(
        "help.nprintf.example4".into(),
        r#"nprintf "%b\n" "a\tb" ## %b ແປ escape ຈາກອາກິວເມັນ"#.into(),
    );
    m.insert(
        "help.nprintf.example5".into(),
        r#"nprintf "no newline" ## ຢູ່ໃນແຖວເດີມ"#.into(),
    );
}
