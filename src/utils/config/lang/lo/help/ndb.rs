use std::collections::HashMap;

pub fn fill(m: &mut HashMap<String, String>) {
    m.insert("help.ndb.title".into(), "ຄຳສັ່ງ NDB (ຖານຂໍູ້ນ):".into());

    m.insert(
        "help.ndb.launch".into(),
        "ndb ## ເປີດ ndb database shell (.db browser) ໃນໄດເຣັກທຣີປັດຈຸບັນ".into(),
    );

    m.insert(
        "help.ndb.root_help".into(),
        "help ## ສະແດງທຸກຄຳສັງ ndb (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.root_view".into(),
        "view | ls ## ສະແງໄຟ໌ .db ທີ່ໃຊ້ໄດ້ (ູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.root_interface".into(),
        "interface <file.db> ## ປີຖານຂ້ມນ: prompt ກາຍປັນ ndb [file.db]>".into(),
    );

    m.insert(
        "help.ndb.root_sql".into(),
        "sql <query> ## ເຮັດຄິວຣີແບບອານເົ່າັ້ນຫຼັງເປີດຖານຂໍ້ມູນ".into(),
    );

    m.insert(
        "help.ndb.root_exit".into(),
        "exit ## ປິດຖານຂໍ້ມູນ ແລວອກຈາກ ndb".into(),
    );

    m.insert(
        "help.ndb.db_viewf".into(),
        "viewf | v ## ສະແດງຄງສາງຕາະລາງ ແະ ຖັນ".into(),
    );

    m.insert(
        "help.ndb.db_tables".into(),
        "tables | t ## ສະແດງຊືຕາຕລາງ ແລ ວິວ".into(),
    );

    m.insert(
        "help.ndb.db_tablef".into(),
        "tablef <table> ## ະແງແຖວອງາຕະາງ (ື txtf)".into(),
    );

    m.insert(
        "help.ndb.db_schema".into(),
        "schema <table> ## ສະແດງ CREATE TABLE SQL ແລະ ລາລະອຽດຖັນ".into(),
    );

    m.insert(
        "help.ndb.db_sql".into(),
        "sql <query> ## ເຮັດຄິວຣີແບບອານເທົ່ານັ້ນ (-l N ແຖວ, --all)".into(),
    );

    m.insert(
        "help.ndb.db_write".into(),
        "write YES <stmt> ## ເຮັດການຂຽນໜຶ່ງຄຳສັງ ຕ້ອງ edit on ກ່ອນ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.db_edit".into(),
        "edit on|off ## ອະນຸຍາດຫຼືບລັອກການຂຽນ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.db_backup".into(),
        "backup ## ຄັດລອກໄຟລ .db ໄວ້ຂ້າງໆ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.more".into(),
        "more ## ສະດງຫາຖັດໄປອງຄິຣີລ່າຸດ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.indexes".into(),
        "indexes [table] ## ສະດງດັດຊະນີ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.info".into(),
        "info ## ສະດງໄຟລ ເນຈິນ ແລະ ຈຳນວນວັດຖຸ (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.dbal".into(),
        "dbal ## ຈັດການຄິຣີອ່ານເທົ່ານັ້ນທີ່ບັນທຶກໄວ້: add|run|list|rm (ຢູ່ນ ndb)".into(),
    );

    m.insert(
        "help.ndb.db_exit".into(),
        "exit ## ປິດຖານຂ້ມນ ແະ ກັບປ ndb [cwd]>".into(),
    );

    m.insert("help.ndb.readonly".into(), "່ານທົານັນ ## ບລັອກກາຂຽນ".into());

    m.insert(
        "help.ndb.example1".into(),
        "ndb ## ຈາກນັ້ນ: interface users.db".into(),
    );

    m.insert(
        "help.ndb.example2".into(),
        "ndb ## ຈາກນັນ: tablef users --all".into(),
    );
}
