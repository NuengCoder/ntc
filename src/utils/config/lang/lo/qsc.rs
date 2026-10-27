use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "qsc.macos_listen_hint".into(),
        "QSC: ເຮັດວຽກ 'qsc listen' ເມື່ອເພີ່ມການສະໜັບສະໜູນແລ້ວ.".into(),
    );

    m.insert(
        "qsc.macos_not_implemented".into(),
        "QSC: ການສະໜັບສະໜູນ macOS global hotkey ຍັງບໍ່ຖືກຈັດຕັ້ງປະຕິບັດ.".into(),
    );

    m.insert(
        "qsc.unsupported_platform".into(),
        "QSC: platform ບໍ່ຮອງຮັບ".into(),
    );
}
