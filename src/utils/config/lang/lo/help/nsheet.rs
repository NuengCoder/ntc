use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.nsheet.title".into(), "ຄຳສັ່ງ NSHEET:".into());

    m.insert(
        "help.nsheet.open".into(),
        "nsheet open <ໄຟລ໌> [ຕົວເລືອກ] ## ເບິ່ງຕາຕະລາງແບບເຕັມຈໍ (CSV/TSV/JSON)".into(),
    );

    m.insert(
        "help.nsheet.view".into(),
        "nsheet view <ໄຟລ໌> [ຕົວເລືອກ] ## ພິມຕາຕະລາງ (ເລີ່ມຕົ້ນ 50 ແຖວ)".into(),
    );

    m.insert(
        "help.nsheet.sort".into(),
        "nsheet view <ໄຟລ໌> --sort <ຖັນ> [--desc] ## ຮຽງຕາມຖັນ".into(),
    );

    m.insert(
        "help.nsheet.filter".into(),
        "nsheet view <ໄຟລ໌> --filter <ເງື່ອນໄຂ> ## ກອງແຖວ (col~ຂໍ້ຄວາມ, col=ຄ່າ, col>ເລກ)".into(),
    );

    m.insert(
        "help.nsheet.export".into(),
        "nsheet view <ໄຟລ໌> --export <json|md> [-o <ໄຟລ໌>] ## ສົ່ງອອກ".into(),
    );

    m.insert(
        "help.nsheet.tui".into(),
        "ປຸ່ມເຕັມຈໍ: ລูกສອນ/hjkl ເລື່ອນ · / ຄົ້ນຫາ · n ຖັດໄປ · s ຮຽງ · f ກອງ · e ສົ່ງອອກ · Enter ເບິ່ງ · q ອອກ"
            .into(),
    );

    m.insert(
        "help.nsheet.example1".into(),
        "nsheet open users.csv ## ເປີດເບິ່ງຕາຕະລາງ".into(),
    );

    m.insert(
        "help.nsheet.example2".into(),
        "nsheet view data.json --filter \"age>30\" --sort age ## ກອງແລະຮຽງ".into(),
    );

    m.insert(
        "help.nsheet.example3".into(),
        "nsheet view a.csv --export md -o out.md ## ສົ່ງອອກ Markdown".into(),
    );
}
