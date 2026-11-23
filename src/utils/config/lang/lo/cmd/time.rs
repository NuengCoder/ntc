use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("cmd.time.bench_header".into(), "ກຳລັງວັດແທກ {} ຄັ້ງ: {}".into());
    m.insert("cmd.time.bench_max".into(), "  ສູງສຸດ:   {}".into());
    m.insert("cmd.time.bench_mean".into(), "  ສະເລ່ຍ:   {}".into());
    m.insert("cmd.time.bench_median".into(), "  ມັດທະຍົມ: {}".into());
    m.insert("cmd.time.bench_min".into(), "  ຕ່ຳສຸດ:   {}".into());
    m.insert(
        "cmd.time.bench_note".into(),
        "ການຣັນຊ້ຳອາດປ່ຽນໄຟລ໌ ຫຼື ສະພາບແວດລ້ອມ; ຜົນຂ້າງຄຽງຈະບໍ່ຖືກຍົກເລີກ.".into(),
    );
    m.insert("cmd.time.bench_run".into(), "  ຄັ້ງທີ {}: {} ({})".into());
    m.insert("cmd.time.bench_summary".into(), "ສະຫຼຸບ ({} ຄັ້ງ):".into());
    m.insert(
        "cmd.time.bench_usage".into(),
        "ວິທີໃຊ້: time --bench <n> <command>".into(),
    );
    m.insert("cmd.time.cancelled".into(), "ຍົກເລີກການຈັບເວລາແລ້ວ.".into());
    m.insert(
        "cmd.time.compare_usage".into(),
        "ວິທີໃຊ້: time <command A> --compare <command B> [-w]".into(),
    );
    m.insert(
        "cmd.time.count_usage".into(),
        "ວິທີໃຊ້: time --count <duration>".into(),
    );
    m.insert("cmd.time.elapsed".into(), "ໃຊ້ເວລາ: {}".into());
    m.insert("cmd.time.executing".into(), "ກຳລັງປະຕິບັດ: {}".into());
    m.insert("cmd.time.exit_status".into(), "ສະຖານະອອກ: {}".into());
    m.insert("cmd.time.finished".into(), "ໝົດເວລາແລ້ວ!".into());
    m.insert("cmd.time.first_command".into(), "ກຳລັງຣັນຄຳສັ່ງທຳອິດ: {}".into());
    m.insert("cmd.time.first_result".into(), "ທຳອິດ: {} (ອອກ {})".into());
    m.insert(
        "cmd.time.ignoring_whitespace".into(),
        "ກຳລັງຂ້າມຄວາມແຕກຕ່າງທີ່ເປັນຊ່ອງຫວ່າງເທົ່ານັ້ນ.".into(),
    );
    m.insert(
        "cmd.time.invalid_duration".into(),
        "ໄລຍະເວລາ '{}' ບໍ່ຖືກຕ້ອງ. ໃຊ້ຈຳນວນວິນາທີ ຫຼື ຮູບແບບ 90s, 5m, ຫຼື 1h30m.".into(),
    );
    m.insert(
        "cmd.time.invalid_runs".into(),
        "time --bench: ຈຳນວນຄັ້ງ '{}' ບໍ່ຖືກຕ້ອງ. ໃຊ້ຈຳນວນເຕັມບວກ.".into(),
    );
    m.insert(
        "cmd.time.large_diff".into(),
        "ຄວາມແຕກຕ່າງໃຫຍ່; ສະແດງແຖວທີ່ຖືກລຶບ ແລະ ເພີ່ມໂດຍບໍ່ຈັດຮຽງ:".into(),
    );
    m.insert(
        "cmd.time.outputs_differ".into(),
        "ຜົນລັບຂອງຄຳສັ່ງແຕກຕ່າງກັນ:".into(),
    );
    m.insert("cmd.time.outputs_match".into(), "ຜົນລັບຂອງຄຳສັ່ງຄືກັນ.".into());
    m.insert("cmd.time.remaining".into(), "ເຫຼືອ {} ວິນາທີ ".into());
    m.insert(
        "cmd.time.run_usage".into(),
        "ວິທີໃຊ້: time <command>".into(),
    );
    m.insert(
        "cmd.time.second_command".into(),
        "ກຳລັງຣັນຄຳສັ່ງທີສອງ: {}".into(),
    );
    m.insert("cmd.time.second_result".into(), "ທີສອງ: {} (ອອກ {})".into());
    m.insert("cmd.time.status_failed".into(), "ລົ້ມເຫຼວ".into());
    m.insert("cmd.time.status_ok".into(), "ສຳເລັດ".into());
    m.insert("cmd.time.terminated".into(), "ຄຳສັ່ງຖືກຢຸດ".into());
}
