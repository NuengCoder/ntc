use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "help.views.title".into(),
        "ຄຳສັ່ງ VIEWS (ລາຍການແບນສັ້ນ):".into(),
    );

    m.insert(
        "help.views.usage".into(),
        "views [<dir|@tp|@tp/dir/>] ## ສະແດງໜຶ່ງລະດັບຂອງ directory ແບບແບນຄື ls".into(),
    );

    m.insert(
        "help.views.size".into(),
        "views -s, --size [ຂະໜາດຂັ້ນຕ່ຳ] ## ສະແດງຂະໜາດ; ໃສ່ຄ່າຈະເຊື່ອງລາຍການທີ່ນ້ອຍກວ່າ (1024, 10KB, 5MB)"
            .into(),
    );

    m.insert(
        "help.views.sort".into(),
        "views --sort <name|ext|size> ## ຈັດລຳດັບລາຍການ (ຄ່າເລີ່ມຕົ້ນ: name)".into(),
    );

    m.insert(
        "help.views.newline".into(),
        "views -n ## ໜຶ່ງລາຍການຕໍ່ແຖວ ແທນການຈັດກຸ່ມແບບໄຫຼຕໍ່ເນື່ອງ".into(),
    );

    m.insert(
        "help.views.note".into(),
        "ບໍ່ມີແທັກ [Directory]/[ignored] - ສີບອກຊະນິດ ຄວາມເລິກຄົງທີ່ທີ່ 1 ແລະ ຄິດຂະໜາດດ້ວຍຄວາມໝາຍຂອງ --care ສະເໝີ ໃຊ້ --size ເພື່ອສະແດງຂະໜາດ".into(),
    );

    m.insert(
        "help.views.example1".into(),
        "views ## ສະແດງລາຍການແບນຂອງ directory ປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.views.example2".into(),
        "views src --size 1KB --sort size ## ລາຍການ >= 1KB ໃຫຍ່ສຸດກ່ອນ ສະແດງຂະໜາດ".into(),
    );
}
