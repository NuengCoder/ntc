use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "navigator.already_root".into(),
        "ຢູ່ root ແລ້ວ, ບໍ່ສາມາດກັບຄືນໄປອີກ".into(),
    );

    m.insert(
        "navigator.cannot_navigate_to".into(),
        "ບໍ່ສາມາດນຳທາງໄປ: {}".into(),
    );

    m.insert(
        "navigator.cannot_read_dir".into(),
        "ອ່ານໄດເຣັກທໍຣີບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "navigator.drive_windows_only".into(),
        "ການນຳທາງ drive ຮອງຮັບພຽງ Windows".into(),
    );

    m.insert("navigator.not_a_directory".into(), "{} ບໍ່ແມ່ນໄດເຣັກທໍຣີ".into());

    m.insert(
        "navigator.venv_activate_unix".into(),
        "   Run: source ./venv/bin/activate".into(),
    );

    m.insert(
        "navigator.venv_activate_win".into(),
        "   Run: .\\\\\\\\venv\\\\\\\\Scripts\\\\\\\\activate".into(),
    );

    m.insert(
        "navigator.venv_tip".into(),
        "   Run: source ./venv/bin/activate".into(),
    );
}
