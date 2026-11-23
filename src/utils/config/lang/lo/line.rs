use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "misc.line.usage".into(),
        "ວິທີໃຊ້: line [-A <ຄຳນຳໜ້າ>] [-B <ຕົວຄັ່ນ>] [file]  (ໃສ່ເລກແຕ່ລະແຖວ; ອ່ານຈາກ pipe ເມື່ອບໍ່ມີໄຟລ໌)".into(),
    );

    m.insert(
        "misc.line.bad_arg".into(),
        "line: ອາກິວເມັນທີ່ບໍ່ຄາດຄິດ '{}' (ວິທີໃຊ້: line [-A <ຄຳນຳໜ້າ>] [-B <ຕົວຄັ່ນ>] [file])".into(),
    );
}
