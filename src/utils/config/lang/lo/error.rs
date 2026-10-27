use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "error.failed_write_completions".into(),
        "ຂຽນ completions ບໍ່ສຳເລັດ: {}".into(),
    );

    m.insert(
        "error.unsupported_shell".into(),
        "shell ບໍ່ຮອງຮັບ: {}. ໃຊ້: bash, zsh, fish, powershell".into(),
    );
}
