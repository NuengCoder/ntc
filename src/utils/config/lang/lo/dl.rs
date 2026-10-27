use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert(
        "dl.cancel_hint".into(),
        "ໃຊ້ 'dl --cancel <number|url>' ເພື່ອລຶບບັນທຶກ.".into(),
    );

    m.insert("dl.completed".into(), "ດາວໂຫຼດ {} ແລ້ວ ({} ໃນ {}, {})".into());

    m.insert(
        "dl.dns_host_required".into(),
        "ຕ້ອງການ Host. ວິທີໃຊ້: dns <host>".into(),
    );

    m.insert(
        "dl.dns_looking_up".into(),
        "ກຳລັງຄົ້ນຫາ {} {} records ຜ່ານ system tools...".into(),
    );

    m.insert(
        "dl.dns_lookup_failed".into(),
        "ການຄົ້ນຫາ DNS ລົ້ມເຫຼວ (exit code: {})".into(),
    );

    m.insert("dl.dns_missing_type".into(), "ຂາດ record type ຫຼັງ -t".into());

    m.insert("dl.dns_no_records".into(), " {} ບໍ່ພົບ {} records".into());

    m.insert(
        "dl.dns_tool_failed".into(),
        "ເຮັດວຽກ DNS tool ບໍ່ສຳເລັດ: {}. ຕິດຕັ້ງ dig ຫຼື nslookup ສຳລັບການຮອງຮັບ {} records.".into(),
    );

    m.insert("dl.downloading".into(), "ກຳລັງດາວໂຫຼດ {}".into());

    m.insert(
        "dl.failed_append".into(),
        "ເປີດ {} ສຳລັບ append ບໍ່ສຳເລັດ".into(),
    );

    m.insert("dl.failed_create".into(), "ສ້າງ {} ບໍ່ສຳເລັດ".into());

    m.insert("dl.failed_download".into(), "ດາວໂຫຼດ {} ລົ້ມເຫຼວ".into());

    m.insert("dl.failed_read".into(), "ອ່ານ {} ບໍ່ສຳເລັດ".into());

    m.insert(
        "dl.file_still_exists".into(),
        "ໄຟລ໌ '{}' ຍັງມີຢູ່. ລຶບດ້ວຍມືຖ້າຕ້ອງການ.".into(),
    );

    m.insert("dl.help.examples".into(), "ຕົວຢ່າງ:".into());

    m.insert(
        "dl.help.opt_cancel".into(),
        " --cancel <number|url> ລຶບບັນທຶກການດາວໂຫຼດ".into(),
    );

    m.insert("dl.help.opt_help".into(), " -h, --help ສະແດງ help ນີ້".into());

    m.insert(
        "dl.help.opt_o".into(),
        " -o, --output <file> ບັນທຶກເປັນຊື່ໄຟລ໌".into(),
    );

    m.insert(
        "dl.help.opt_progress".into(),
        " --progress ສະແດງແຖບ progress ການດາວໂຫຼດ".into(),
    );

    m.insert(
        "dl.help.opt_resume".into(),
        " --resume ດາວໂຫຼດຕໍ່ຈາກສ່ວນທີ່ດາວໂຫຼດໄວ້".into(),
    );

    m.insert(
        "dl.help.opt_stop".into(),
        " --stop ສະແດງ history ການດາວໂຫຼດ".into(),
    );

    m.insert("dl.help.options".into(), "ຕົວເລືອກ:".into());

    m.insert("dl.help.title".into(), "Download (dl)".into());

    m.insert("dl.help.usage".into(), "ວິທີໃຊ້:".into());

    m.insert("dl.help.usage_line".into(), " dl <url> [options]".into());

    m.insert("dl.history_title".into(), "Download History:".into());

    m.insert("dl.http_error".into(), "HTTP {}: {}".into());

    m.insert(
        "dl.invalid_header".into(),
        "ຮູບແບບ header ບໍ່ຖືກຕ້ອງ, ຄາດຫວັງ \"Key: Value\", ໄດ້ຮັບ: {}".into(),
    );

    m.insert("dl.invalid_index".into(), "index ບໍ່ຖືກຕ້ອງ".into());

    m.insert("dl.invalid_request".into(), "Request ບໍ່ຖືກຕ້ອງ: {}".into());

    m.insert(
        "dl.missing_cancel".into(),
        "ຂາດ URL/name ຫຼັງ --cancel".into(),
    );

    m.insert("dl.missing_data".into(), "ຂາດ data ຫຼັງ -d".into());

    m.insert("dl.missing_header".into(), "ຂາດ header value ຫຼັງ -H".into());

    m.insert("dl.missing_o".into(), "ຂາດຊື່ໄຟລ໌ ຫຼັງ -o".into());

    m.insert("dl.no_records".into(), "ຍັງບໍ່ມີການບັນທຶກການດາວໂຫຼດ.".into());

    m.insert("dl.not_found".into(), "ບໍ່ພົບບັນທຶກການດາວໂຫຼດສຳລັບ: {}".into());

    m.insert("dl.not_found_404".into(), "ບໍ່ພົບ URL (404): {}".into());

    m.insert("dl.ping_exit_code".into(), "ping ອອກດ້ວຍ code: {}".into());

    m.insert("dl.ping_failed".into(), "ເຮັດວຽກ ping ບໍ່ສຳເລັດ: {}".into());

    m.insert(
        "dl.ping_host_required".into(),
        "ຕ້ອງການ Host. ວິທີໃຊ້: ping <host>".into(),
    );

    m.insert("dl.ping_invalid_count".into(), "count ບໍ່ຖືກຕ້ອງ".into());

    m.insert("dl.ping_missing_count".into(), "ຂາດ count ຫຼັງ -c".into());

    m.insert("dl.removed_record".into(), "ລຶບບັນທຶກການດາວໂຫຼດແລ້ວ: {}".into());

    m.insert("dl.request_failed".into(), "ຄຳຂໍໄປຍັງ {} ລົ້ມເຫຼວ".into());

    m.insert(
        "dl.up_url_required".into(),
        "ຕ້ອງການ URL. ວິທີໃຊ້: up <file> <url>".into(),
    );

    m.insert("dl.upload_failed".into(), "upload ໄປຍັງ {} ລົ້ມເຫຼວ".into());

    m.insert("dl.uploading".into(), "ກຳລັງ upload {} ({})".into());

    m.insert(
        "dl.url_required".into(),
        "ຕ້ອງການ URL. ວິທີໃຊ້: dl <url>".into(),
    );
}
