use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "cmd.mode.admin_relaunch".into(),
        "ກຳລັງຣີເລົ່ນ ntc ເປັນ Administrator...".into(),
    );

    m.insert(
        "cmd.mode.elevate_fail".into(),
        "ລົ້ມເຫຼວໃນການເປີດ process ທີ່ elevate: {}".into(),
    );

    m.insert(
        "cmd.mode.exe_fail".into(),
        "ໄດ້ path executable ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.mode.launch_fail".into(),
        "ເປີດ process ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "cmd.mode.normal_relaunch".into(),
        "ກຳລັງຣີເລົ່ນ ntc ໃນ normal mode...".into(),
    );

    m.insert(
        "cmd.mode.windows_only".into(),
        "mode ແມ່ນຄຸນສົມບັດສະເພາະ Windows ເທົ່ານັ້ນ.".into(),
    );
}
