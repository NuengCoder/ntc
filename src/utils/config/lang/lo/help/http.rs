use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.http.title".into(), "HTTP ຄຳສັ່ງ:".into());

    m.insert(
        "help.http.usage".into(),
        "http <method> <url> [options] ## ສົ່ງຄຳຮ້ອງຂໍ (method ເລີ່ມຕົ້ນແມ່ນ GET)".into(),
    );

    m.insert(
        "help.http.usage2".into(),
        "http GET|POST|PUT|PATCH|DELETE|HEAD|QUERY <url> ## method ທີ່ຮອງຮັບ".into(),
    );

    m.insert(
        "help.http.note".into(),
        "ຕົວເລືອກ: -H \"Key: Value\" header, -d ຂໍ້ຄວາມ body, --json body ແບບ JSON (ຕັ້ງ Content-Type ໃຫ້), -q key=value query, -a user:pass Basic auth, --bearer token, -o ໄຟລ໌ ບັນທຶກ body, -s ສະແດງສະເພາະ body, -v ສະແດງຄຳຮ້ອງຂໍ, -t ວິນາທີ timeout (ຄ່າເລີ່ມຕົ້ນ 30). ຜົນ JSON ຈະຖືກຈັດຮູບແບບອັດຕະໂນມັດ ເວັ້ນເສຍແຕ່ໃຊ້ --no-pretty.".into(),
    );

    m.insert(
        "help.http.example1".into(),
        "http GET https://api.example.com/users ## ຄຳຮ້ອງຂໍ GET ງ່າຍໆ".into(),
    );

    m.insert(
        "help.http.example2".into(),
        "http POST https://api.example.com/users --json '{\"name\":\"ada\"}' ## ສົ່ງ body ແບບ JSON".into(),
    );

    m.insert(
        "help.http.example3".into(),
        "http GET https://api.github.com/repos/rust-lang/rust -H \"Accept: application/vnd.github+json\" -q per_page=1 ## header + query".into(),
    );

    m.insert(
        "help.http.example4".into(),
        "http GET https://api.example.com/me -a user:pass -o me.json ## Basic auth ແລະ ບັນທຶກ body ລົງໄຟລ໌".into(),
    );

    m.insert(
        "help.http.example5".into(),
        "http GET https://api.example.com/users -s | qur -s name ## ສົ່ງ body ດິບໄປຍັງຄຳສັ່ງອື່ນ".into(),
    );
}
