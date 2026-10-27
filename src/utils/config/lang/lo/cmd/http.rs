use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.http.authorization_label".into(), "ການອະນຸຍາດ:".into());

    m.insert(
        "cmd.http.basic_placeholder".into(),
        "Basic <encoded>".into(),
    );

    m.insert(
        "cmd.http.bearer_placeholder".into(),
        "Bearer <token>".into(),
    );

    m.insert("cmd.http.byte_count".into(), "{} bytes".into());

    m.insert(
        "cmd.http.help_auth".into(),
        "  -a, --auth <user:pass>         Basic authentication".into(),
    );

    m.insert(
        "cmd.http.help_bearer".into(),
        "  --bearer <token>               Bearer token authentication".into(),
    );

    m.insert(
        "cmd.http.help_body".into(),
        "  -d, --data <body>              Request body".into(),
    );

    m.insert(
        "cmd.http.help_example1".into(),
        "  http GET https://api.github.com/users/octocat".into(),
    );

    m.insert("cmd.http.help_example2".into(), "  http POST https://httpbin.org/post -H \"Content-Type: application/json\" --json '{\"name\":\"test\"}'".into());

    m.insert(
        "cmd.http.help_example3".into(),
        "  http GET https://api.example.com/search -q q=hello -q page=1 -s > results.json".into(),
    );

    m.insert(
        "cmd.http.help_example4".into(),
        "  http GET https://api.example.com/data --bearer mytoken -o response.json".into(),
    );

    m.insert(
        "cmd.http.help_example5".into(),
        "  http GET https://api.example.com/health -s -t 5".into(),
    );

    m.insert(
        "cmd.http.help_example6".into(),
        "  http GET https://api.example.com/users -a admin:secret123".into(),
    );

    m.insert("cmd.http.help_examples_title".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "cmd.http.help_header".into(),
        "  -H, --header <\"Key: Value\">     ເພີ່ມ request header (repeatable)".into(),
    );

    m.insert(
        "cmd.http.help_help".into(),
        "  -h, --help                     ສະແດງ help ນີ້".into(),
    );

    m.insert(
        "cmd.http.help_json".into(),
        "  --json <data>                  ສົ່ງເປັນ JSON (auto-set Content-Type)".into(),
    );

    m.insert(
        "cmd.http.help_methods".into(),
        "  GET, POST, PUT, DELETE, PATCH, HEAD".into(),
    );

    m.insert("cmd.http.help_methods_title".into(), "Methods:".into());

    m.insert(
        "cmd.http.help_no_pretty".into(),
        "  --no-pretty                    ປິດ JSON pretty-printing".into(),
    );

    m.insert("cmd.http.help_options_title".into(), "Options:".into());

    m.insert(
        "cmd.http.help_output".into(),
        "  -o, --output <file>            ບັນທຶກ body ໃສ່ໄຟລ໌ (raw)".into(),
    );

    m.insert(
        "cmd.http.help_query".into(),
        "  -q, --query <key=value>        URL query parameter (repeatable)".into(),
    );

    m.insert(
        "cmd.http.help_silent".into(),
        "  -s, --silent                   ສະແດງ body ຢ່າງດຽວ (ບໍ່ມີ status/headers)".into(),
    );

    m.insert(
        "cmd.http.help_timeout".into(),
        "  -t, --timeout <secs>           Request timeout (default 30)".into(),
    );

    m.insert("cmd.http.help_title".into(), "HTTP API Client".into());

    m.insert(
        "cmd.http.help_usage_default".into(),
        "  <url>                          (GET ໂດຍຄ່າເລີ່ມຕົ້ນ ຖ້າ URL ເປັນ argument ທຳອິດ)".into(),
    );

    m.insert(
        "cmd.http.help_usage_line".into(),
        "  http <method> <url> [options]".into(),
    );

    m.insert("cmd.http.help_usage_title".into(), "Usage:".into());

    m.insert(
        "cmd.http.help_verbose".into(),
        "  -v, --verbose                  ສະແດງລາຍລະອຽດ request ທີ່ອອກ".into(),
    );

    m.insert("cmd.http.invalid_body".into(), "Body ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "cmd.http.invalid_header".into(),
        "ຮູບແບບ header ບໍ່ຖືກຕ້ອງ, ຄາດຫວັງ \"Key: Value\", ໄດ້ຮັບ: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_query".into(),
        "ຮູບແບບ query ບໍ່ຖືກຕ້ອງ, ຄາດຫວັງ key=value, ໄດ້ຮັບ: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_request".into(),
        "Request ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert(
        "cmd.http.invalid_timeout".into(),
        "Timeout ບໍ່ຖືກຕ້ອງ: {}".into(),
    );

    m.insert("cmd.http.missing_auth".into(), "ຂາດ user:pass ຫຼັງ -a".into());

    m.insert(
        "cmd.http.missing_bearer".into(),
        "ຂາດ token ຫຼັງ --bearer".into(),
    );

    m.insert(
        "cmd.http.missing_body_data".into(),
        "ຂາດ body data ຫຼັງ -d".into(),
    );

    m.insert("cmd.http.missing_filename".into(), "ຂາດຊື່ໄຟລ໌ ຫຼັງ -o".into());

    m.insert(
        "cmd.http.missing_header".into(),
        "ຂາດ header value ຫຼັງ -H".into(),
    );

    m.insert(
        "cmd.http.missing_json".into(),
        "ຂາດ JSON data ຫຼັງ --json".into(),
    );

    m.insert(
        "cmd.http.missing_query".into(),
        "ຂາດ key=value ຫຼັງ -q".into(),
    );

    m.insert(
        "cmd.http.missing_timeout".into(),
        "ຂາດ seconds ຫຼັງ -t".into(),
    );

    m.insert("cmd.http.request_failed".into(), "{} {} ລົ້ມເຫຼວ".into());

    m.insert("cmd.http.saved_bytes".into(), "ບັນທຶກ {} bytes ໄປທີ່ {}".into());

    m.insert("cmd.http.status_line".into(), "{} {} {}  ".into());

    m.insert("cmd.http.unknown_status".into(), "ບໍ່ຮູ້ຈັກ".into());

    m.insert(
        "cmd.http.url_required".into(),
        "ຕ້ອງການ URL. ວິທີໃຊ້: http <method> <url>".into(),
    );

    m.insert("cmd.http.verbose_prefix".into(), ">".into());

    m.insert("cmd.http.write_fail".into(), "ຂຽນໄປທີ່ {} ບໍ່ສຳເລັດ".into());
}
