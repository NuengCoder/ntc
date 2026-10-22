// src/fake/locale.rs
//! Locale-aware pools for `fake gen --locale`.
//!
//! Culture-sensitive display types (`first_name`, `last_name`, `name`,
//! `city`, `state`, `street`, `address`, `phone_number`,
//! `phone_country_code`, `postal_code`, `id_card`) consult this; every other
//! type is locale-independent.
//!
//! Names are stored as `(native, roman)` pairs: `native` is what the display
//! types emit (may be non-Latin), `roman` is the ASCII form the machine-ish
//! types (`email`, `username`) derive from.

use super::data;

/// A name in its native script plus an ASCII/roman rendering.
#[derive(Clone, Copy)]
pub struct NamePair {
    pub native: &'static str,
    pub roman: &'static str,
}

/// Latin-script name: the roman form is the native form (lower-cased later).
pub const fn np(native: &'static str) -> NamePair {
    NamePair {
        native,
        roman: native,
    }
}

/// Non-Latin name with an explicit roman rendering.
pub const fn np2(native: &'static str, roman: &'static str) -> NamePair {
    NamePair { native, roman }
}

/// A supported locale / region.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locale {
    EnUs,
    EnGb,
    ZhCn,
    ThTh,
    LoLa,
}

/// Pools + patterns backing one locale. Patterns use `#` (digit),
/// `A` (uppercase letter) and `a` (lowercase letter); everything else is
/// literal.
pub struct LocaleData {
    pub first_names: &'static [NamePair],
    pub last_names: &'static [NamePair],
    pub cities: &'static [&'static str],
    pub states: &'static [&'static str],
    pub street_names: &'static [&'static str],
    pub street_suffixes: &'static [&'static str],
    /// `true` when `name` renders family-name first (e.g. 王伟).
    pub last_first: bool,
    pub phone_cc: &'static str,
    pub phone_pattern: &'static str,
    pub postal_pattern: &'static str,
}

impl Locale {
    /// Parse a locale code (`en_US`, `en-GB`, `TH`, …), case-insensitive.
    pub fn parse(code: &str) -> Option<Locale> {
        let c = code.trim().to_lowercase().replace('-', "_");
        Some(match c.as_str() {
            "en_us" | "en" | "us" | "" => Locale::EnUs,
            "en_gb" | "gb" | "uk" => Locale::EnGb,
            "zh_cn" | "zh" | "cn" => Locale::ZhCn,
            "th_th" | "th" => Locale::ThTh,
            "lo_la" | "lo" | "la" => Locale::LoLa,
            _ => return None,
        })
    }

    /// Canonical `xx_YY` code.
    pub fn code(self) -> &'static str {
        match self {
            Locale::EnUs => "en_US",
            Locale::EnGb => "en_GB",
            Locale::ZhCn => "zh_CN",
            Locale::ThTh => "th_TH",
            Locale::LoLa => "lo_LA",
        }
    }

    /// Every supported locale, in listing order.
    pub fn all() -> &'static [Locale] {
        &[
            Locale::EnUs,
            Locale::EnGb,
            Locale::ZhCn,
            Locale::ThTh,
            Locale::LoLa,
        ]
    }

    pub fn data(self) -> &'static LocaleData {
        match self {
            Locale::EnUs => &EN_US,
            Locale::EnGb => &EN_GB,
            Locale::ZhCn => &ZH_CN,
            Locale::ThTh => &TH_TH,
            Locale::LoLa => &LO_LA,
        }
    }
}

static EN_US: LocaleData = LocaleData {
    first_names: &[
        np("Emily"),
        np("Michael"),
        np("Jessica"),
        np("David"),
        np("Sarah"),
        np("James"),
        np("Ashley"),
        np("Daniel"),
        np("Amanda"),
        np("Christopher"),
        np("Melissa"),
        np("Matthew"),
        np("Nicole"),
        np("Joshua"),
        np("Elizabeth"),
        np("Andrew"),
        np("Stephanie"),
        np("Ryan"),
        np("Rebecca"),
        np("Brandon"),
        np("Lauren"),
        np("Tyler"),
        np("Rachel"),
        np("Jacob"),
        np("Megan"),
        np("Ethan"),
        np("Hannah"),
        np("Noah"),
        np("Olivia"),
        np("Sophia"),
        np("Liam"),
        np("Emma"),
        np("Ava"),
        np("Mason"),
        np("Isabella"),
        np("Lucas"),
    ],
    last_names: &[
        np("Smith"),
        np("Johnson"),
        np("Williams"),
        np("Brown"),
        np("Jones"),
        np("Garcia"),
        np("Miller"),
        np("Davis"),
        np("Rodriguez"),
        np("Martinez"),
        np("Hernandez"),
        np("Lopez"),
        np("Gonzalez"),
        np("Wilson"),
        np("Anderson"),
        np("Thomas"),
        np("Taylor"),
        np("Moore"),
        np("Jackson"),
        np("Martin"),
        np("Lee"),
        np("Perez"),
        np("Thompson"),
        np("White"),
        np("Harris"),
        np("Sanchez"),
        np("Clark"),
        np("Ramirez"),
        np("Lewis"),
        np("Robinson"),
        np("Walker"),
        np("Young"),
        np("Allen"),
        np("King"),
        np("Wright"),
        np("Scott"),
    ],
    cities: data::CITIES,
    states: data::STATES,
    street_names: data::STREET_NAMES,
    street_suffixes: data::STREET_SUFFIXES,
    last_first: false,
    phone_cc: "+1",
    phone_pattern: "+1 (###) ###-####",
    postal_pattern: "#####",
};

static EN_GB: LocaleData = LocaleData {
    first_names: &[
        np("Oliver"),
        np("Amelia"),
        np("Harry"),
        np("Isla"),
        np("Jack"),
        np("Emily"),
        np("George"),
        np("Sophie"),
        np("Noah"),
        np("Grace"),
        np("Alfie"),
        np("Freya"),
        np("Charlie"),
        np("Poppy"),
        np("Thomas"),
        np("Evie"),
        np("Oscar"),
        np("Florence"),
        np("Archie"),
        np("Willow"),
        np("Henry"),
        np("Daisy"),
        np("Leo"),
        np("Millie"),
        np("Arthur"),
        np("Ella"),
        np("Freddie"),
        np("Mia"),
        np("Theodore"),
        np("Rosie"),
    ],
    last_names: &[
        np("Smith"),
        np("Jones"),
        np("Taylor"),
        np("Brown"),
        np("Wilson"),
        np("Evans"),
        np("Thomas"),
        np("Roberts"),
        np("Walker"),
        np("Wright"),
        np("Robinson"),
        np("Thompson"),
        np("White"),
        np("Hughes"),
        np("Edwards"),
        np("Green"),
        np("Hall"),
        np("Wood"),
        np("Harris"),
        np("Martin"),
        np("Jackson"),
        np("Clarke"),
        np("Turner"),
        np("Hill"),
        np("Scott"),
        np("Cooper"),
        np("Ward"),
        np("Morris"),
        np("Bell"),
        np("Watson"),
    ],
    cities: &[
        "London",
        "Manchester",
        "Birmingham",
        "Leeds",
        "Glasgow",
        "Liverpool",
        "Bristol",
        "Edinburgh",
        "Cardiff",
        "Sheffield",
        "Newcastle",
        "Nottingham",
        "Leicester",
        "Southampton",
        "Brighton",
        "Aberdeen",
    ],
    states: &[
        "England",
        "Scotland",
        "Wales",
        "Northern Ireland",
        "Greater London",
        "Merseyside",
        "Yorkshire",
        "Kent",
        "Lancashire",
        "Essex",
        "Hampshire",
        "Surrey",
    ],
    street_names: &[
        "High",
        "Station",
        "Church",
        "Victoria",
        "Kings",
        "Queens",
        "Mill",
        "Park",
        "Albert",
        "Alexandra",
        "Windsor",
        "Oak",
    ],
    street_suffixes: &[
        "Street", "Road", "Lane", "Avenue", "Close", "Way", "Drive", "Court", "Crescent", "Grove",
    ],
    last_first: false,
    phone_cc: "+44",
    phone_pattern: "+44 7### ######",
    postal_pattern: "AA# #AA",
};

static ZH_CN: LocaleData = LocaleData {
    first_names: &[
        np2("伟", "wei"),
        np2("芳", "fang"),
        np2("娜", "na"),
        np2("敏", "min"),
        np2("静", "jing"),
        np2("磊", "lei"),
        np2("军", "jun"),
        np2("洋", "yang"),
        np2("勇", "yong"),
        np2("艳", "yan"),
        np2("杰", "jie"),
        np2("娟", "juan"),
        np2("涛", "tao"),
        np2("明", "ming"),
        np2("超", "chao"),
        np2("霞", "xia"),
        np2("平", "ping"),
        np2("刚", "gang"),
        np2("辉", "hui"),
        np2("玲", "ling"),
        np2("强", "qiang"),
        np2("雪", "xue"),
        np2("婷", "ting"),
        np2("鹏", "peng"),
        np2("丹", "dan"),
        np2("华", "hua"),
        np2("建", "jian"),
        np2("文", "wen"),
        np2("斌", "bin"),
        np2("颖", "ying"),
    ],
    last_names: &[
        np2("王", "wang"),
        np2("李", "li"),
        np2("张", "zhang"),
        np2("刘", "liu"),
        np2("陈", "chen"),
        np2("杨", "yang"),
        np2("赵", "zhao"),
        np2("黄", "huang"),
        np2("周", "zhou"),
        np2("吴", "wu"),
        np2("徐", "xu"),
        np2("孙", "sun"),
        np2("马", "ma"),
        np2("朱", "zhu"),
        np2("胡", "hu"),
        np2("林", "lin"),
        np2("郭", "guo"),
        np2("何", "he"),
        np2("高", "gao"),
        np2("罗", "luo"),
        np2("郑", "zheng"),
        np2("梁", "liang"),
        np2("谢", "xie"),
        np2("宋", "song"),
        np2("唐", "tang"),
        np2("许", "xu"),
        np2("韩", "han"),
        np2("冯", "feng"),
        np2("邓", "deng"),
        np2("曹", "cao"),
    ],
    cities: &[
        "北京", "上海", "广州", "深圳", "成都", "杭州", "武汉", "西安", "南京", "重庆", "天津",
        "苏州", "青岛", "长沙", "郑州", "厦门",
    ],
    states: &[
        "北京市",
        "上海市",
        "广东省",
        "浙江省",
        "江苏省",
        "四川省",
        "湖北省",
        "陕西省",
        "山东省",
        "福建省",
        "湖南省",
        "河南省",
    ],
    street_names: &[
        "人民", "中山", "解放", "建设", "和平", "新华", "光明", "长江", "黄河", "文化", "幸福",
        "团结",
    ],
    street_suffixes: &["路", "街", "大道", "巷", "胡同"],
    last_first: true,
    phone_cc: "+86",
    phone_pattern: "+86 1## #### ####",
    postal_pattern: "######",
};

static TH_TH: LocaleData = LocaleData {
    first_names: &[
        np2("สมชาย", "somchai"),
        np2("สมศรี", "somsri"),
        np2("อนันต์", "anan"),
        np2("มาลี", "malee"),
        np2("ประเสริฐ", "prasert"),
        np2("กัญญา", "kanya"),
        np2("นิรันดร์", "niran"),
        np2("พิมพ์", "pim"),
        np2("ชัย", "chai"),
        np2("อรุณ", "arun"),
        np2("สุชาติ", "suchat"),
        np2("วันดี", "wandee"),
        np2("ธนา", "thana"),
        np2("ปรีชา", "preecha"),
        np2("ศิริ", "siri"),
        np2("จันทร์", "jan"),
        np2("พิชัย", "phichai"),
        np2("รุ่ง", "rung"),
        np2("นภา", "napha"),
        np2("กิตติ", "kitti"),
        np2("สมบัติ", "sombat"),
        np2("ณัฐ", "nat"),
        np2("วรรณ", "wan"),
        np2("ปิยะ", "piya"),
        np2("เดชา", "decha"),
        np2("สุดา", "suda"),
        np2("มานะ", "mana"),
        np2("อรทัย", "orathai"),
        np2("ธิดา", "thida"),
        np2("วีระ", "weera"),
    ],
    last_names: &[
        np2("สุขใจ", "sukjai"),
        np2("รัตนกร", "rattanakorn"),
        np2("ชัยพร", "chaiporn"),
        np2("สุวรรณ", "suwan"),
        np2("บุญมี", "bounmee"),
        np2("ศรีใส", "srisai"),
        np2("พงษ์ทอง", "pongthong"),
        np2("อินทร์แก้ว", "inkaew"),
        np2("ทองดี", "thongdee"),
        np2("จันทร์เพ็ญ", "janpen"),
        np2("วงศ์ศิลป์", "wongsilp"),
        np2("ธรรมใจ", "thammajai"),
        np2("เจริญสุข", "charoensuk"),
        np2("พร้อมใจ", "promjai"),
        np2("แสงทอง", "saengthong"),
        np2("มั่นคง", "mankhong"),
        np2("พิทักษ์", "phithak"),
        np2("รักไทย", "rakthai"),
        np2("สมบูรณ์", "somboon"),
        np2("ปัญญา", "panya"),
    ],
    cities: &[
        "กรุงเทพ",
        "เชียงใหม่",
        "ภูเก็ต",
        "พัทยา",
        "ขอนแก่น",
        "นครราชสีมา",
        "หาดใหญ่",
        "อุดรธานี",
        "เชียงราย",
        "สุราษฎร์ธานี",
        "นครปฐม",
        "ลำปาง",
    ],
    states: &[
        "กรุงเทพมหานคร",
        "เชียงใหม่",
        "ภูเก็ต",
        "ขอนแก่น",
        "ชลบุรี",
        "นครราชสีมา",
        "สงขลา",
        "อุดรธานี",
        "เชียงราย",
        "สุราษฎร์ธานี",
    ],
    street_names: &[
        "สุขุมวิท",
        "พหลโยธิน",
        "รัชดา",
        "สีลม",
        "เพชรบุรี",
        "ลาดพร้าว",
        "อโศก",
        "สาทร",
        "พระราม",
        "รามคำแหง",
    ],
    street_suffixes: &["ซอย", "ถนน", "ตรอก", "หมู่"],
    last_first: false,
    phone_cc: "+66",
    phone_pattern: "+66 ## ### ####",
    postal_pattern: "#####",
};

static LO_LA: LocaleData = LocaleData {
    first_names: &[
        np2("ຄຳ", "kham"),
        np2("ສີ", "si"),
        np2("ບຸນ", "boun"),
        np2("ພອນ", "phon"),
        np2("ວິຊຽນ", "visien"),
        np2("ສົມພອນ", "somphon"),
        np2("ຄຳແພງ", "khamphaeng"),
        np2("ນ້ອຍ", "noi"),
        np2("ທອງ", "thong"),
        np2("ແກ້ວ", "kaew"),
        np2("ສຸລິ", "suli"),
        np2("ຈັນ", "chan"),
        np2("ວັນ", "van"),
        np2("ຄຳພອນ", "khamphon"),
        np2("ບົວ", "boua"),
        np2("ພູ", "phou"),
        np2("ສີພອນ", "siphon"),
        np2("ອານ", "an"),
        np2("ພອນສະຫວັນ", "phonhasavan"),
        np2("ດາວ", "dao"),
    ],
    last_names: &[
        np2("ສີສະຫວາດ", "sisavath"),
        np2("ພົມມະຈັນ", "phommachan"),
        np2("ໄຊຍະວົງ", "saixayavong"),
        np2("ສຸລິວົງ", "sulivong"),
        np2("ວົງສະຫວັນ", "vongsavan"),
        np2("ຈັນທະວົງ", "chanthavong"),
        np2("ສະຫວາດດີ", "savathdee"),
        np2("ພອນວົງ", "phonvong"),
        np2("ສີສົມບັດ", "sisombat"),
        np2("ແກ້ວວົງ", "kaewvong"),
        np2("ດວງຈັນ", "douangchan"),
        np2("ອິນທະວົງ", "inthavong"),
    ],
    cities: &[
        "ວຽງຈັນ",
        "ຫຼວງພະບາງ",
        "ປາກເຊ",
        "ສະຫວັນນະເຂດ",
        "ຊຳປາສັກ",
        "ອຸດົມໄຊ",
        "ຫຼວງນ້ຳທາ",
        "ຊຽງຂວາງ",
        "ຄຳມ່ວນ",
        "ສະຫວັນ",
    ],
    states: &[
        "ນະຄອນຫຼວງວຽງຈັນ",
        "ຫຼວງພະບາງ",
        "ຈຳປາສັກ",
        "ສະຫວັນນະເຂດ",
        "ອຸດົມໄຊ",
        "ບໍລິຄຳໄຊ",
        "ຊຽງຂວາງ",
        "ຄຳມ່ວນ",
    ],
    street_names: &["ສີສະຫວາດ", "ລ້ານຊ້າງ", "ເສດຖາ", "ອຸດົມສຸກ", "ໄກສອນ"],
    street_suffixes: &["ຖະໜົນ", "ຊອຍ", "ບ້ານ"],
    last_first: false,
    phone_cc: "+856",
    phone_pattern: "+856 ## ## ### ###",
    postal_pattern: "#####",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_accepts_codes_and_regions() {
        assert_eq!(Locale::parse("en_US"), Some(Locale::EnUs));
        assert_eq!(Locale::parse("en-gb"), Some(Locale::EnGb));
        assert_eq!(Locale::parse("TH"), Some(Locale::ThTh));
        assert_eq!(Locale::parse("lo_LA"), Some(Locale::LoLa));
        assert_eq!(Locale::parse("zh"), Some(Locale::ZhCn));
        assert!(Locale::parse("xx_YY").is_none());
    }

    #[test]
    fn every_locale_has_pools() {
        for l in Locale::all() {
            let d = l.data();
            assert!(d.first_names.len() >= 20, "{:?} first names", l.code());
            assert!(d.last_names.len() >= 12, "{:?} last names", l.code());
            assert!(!d.cities.is_empty(), "{:?}", l.code());
            assert!(!d.states.is_empty(), "{:?}", l.code());
            assert!(!d.postal_pattern.is_empty(), "{:?}", l.code());
            assert!(!d.phone_pattern.is_empty(), "{:?}", l.code());
            for p in d.first_names.iter().chain(d.last_names.iter()) {
                assert!(
                    p.roman.chars().all(|c| c.is_ascii_alphanumeric()),
                    "{:?} non-ascii roman '{}'",
                    l.code(),
                    p.roman
                );
                assert!(!p.native.is_empty());
            }
        }
    }

    #[test]
    fn zh_is_last_first() {
        assert!(Locale::ZhCn.data().last_first);
        assert!(!Locale::EnUs.data().last_first);
    }
}
