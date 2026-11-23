use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.pipe.title".into(),
        "PIPE ແລະ ການປ່ຽນເສັ້ນທາງ (PIPES & REDIRECTION) [NON-COMMAND]:".into(),
    );

    m.insert(
        "help.pipe.pipe".into(),
        "a | b ## ສົ່ງ output ຂອງ a ເປັນ input ຂອງ b".into(),
    );

    m.insert(
        "help.pipe.and".into(),
        "a && b ## ແລ່ນ b ຖ້າ a ສຳເລັດເທົ່ານັ້ນ".into(),
    );

    m.insert(
        "help.pipe.or".into(),
        "a || b ## ແລ່ນ b ຖ້າ a ລົ້ມເຫຼວເທົ່ານັ້ນ".into(),
    );

    m.insert(
        "help.pipe.seq".into(),
        "a ; b ## ແລ່ນ b ເຖິງແມ່ນ a ລົ້ມເຫຼວ (ຕ້ອງມີຊ່ອງຫວ່າງສອງຂ້າງ)".into(),
    );

    m.insert(
        "help.pipe.in".into(),
        "cmd < file ## ອ່ານ input ຈາກໄຟລ໌".into(),
    );

    m.insert(
        "help.pipe.out".into(),
        "cmd > file ## ຂຽນ output ລົງໄຟລ໌ (ຂຽນທັບ)".into(),
    );

    m.insert(
        "help.pipe.append".into(),
        "cmd >> file ## ຕໍ່ທ້າຍ output ລົງໄຟລ໌".into(),
    );

    m.insert(
        "help.pipe.heredoc".into(),
        "cmd <<EOF ... EOF ## ອ່ານ input ຈົນຮອດແຖວ EOF".into(),
    );

    m.insert(
        "help.pipe.herestring".into(),
        "cmd <<< text ## ອ່ານ input ຈາກຂໍ້ຄວາມໂດຍກົງ".into(),
    );

    m.insert(
        "help.pipe.filters".into(),
        "cmd | <filter> ## ຕົວກອງ pipe: head, tail, betw, skip, gs, sort, rev, count, trim, encode, qur, clip (ເຕີມ flag ໄດ້)"
            .into(),
    );

    m.insert(
        "help.pipe.note".into(),
        "ຕົວດຳເນີນການຕ້ອງມີຊ່ອງຫວ່າງສອງຂ້າງ; ໃຊ້ - ເພື່ອອ່ານ stdin".into(),
    );

    m.insert(
        "help.pipe.hint".into(),
        "ຄຳແນະນຳ: | ສະແດງຕົວກອງ pipe; >> ແລະ < ສະແດງສະເພາະໄຟລ໌ຂໍ້ຄວາມ; > ບໍ່ສະແດງ; << ສະແດງສະເພາະ EOF"
            .into(),
    );

    m.insert(
        "help.pipe.example1".into(),
        "gs main > results.txt ## ບັນທຶກຜົນການຄົ້ນຫາລົງໄຟລ໌".into(),
    );

    m.insert(
        "help.pipe.example2".into(),
        "view readme.md | encode b64 ## ເຂົ້າລະຫັດເນື້ອໃນໄຟລ໌".into(),
    );

    m.insert(
        "help.pipe.example3".into(),
        "gs main | say ## ເບິ່ງຜົນການຄົ້ນຫາຜ່ານ say".into(),
    );

    m.insert(
        "help.pipe.example4".into(),
        "head 20 app.log ## 20 ແຖວທຳອິດຂອງໄຟລ໌ (ຫຼື input ທີ່ pipe ເຂົ້າມາ)".into(),
    );

    m.insert(
        "help.pipe.example5".into(),
        "gs error | sort -u | head 10 ## ຈັດລຳດັບຜົນທີ່ບໍ່ຊ້ຳ ແລ້ວເອົາ 10 ແຖວທຳອິດ".into(),
    );

    m.insert(
        "help.pipe.example6".into(),
        "echo nice ... bro ... | trim \" ...\" ## ຕັດ pattern ອອກຈາກຂໍ້ຄວາມທີ່ສົ່ງຜ່ານ pipe (ແບບ inline: trim \" ...\" \"nice ... bro ...\")".into(),
    );
}
