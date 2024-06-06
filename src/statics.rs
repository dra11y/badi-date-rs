use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, NaiveTime, TimeZone};
use chrono_tz::{Asia::Tehran, Tz};
use lazy_static::lazy_static;

use crate::BahaiHolyDay;

pub(crate) const YEAR_ONE_IN_GREGORIAN: i32 = 1844;
pub(crate) const YEAR_ZERO_IN_GREGORIAN: i32 = YEAR_ONE_IN_GREGORIAN - 1;
pub(crate) const LAST_YEAR_SUPPORTED: u8 = 221;
pub(crate) const LAST_GREGORIAN_YEAR_SUPPORTED: i32 =
    YEAR_ONE_IN_GREGORIAN + LAST_YEAR_SUPPORTED as i32;
pub(crate) const AYYAMIHA_DAY_1: u16 = 342;
pub(crate) const AYYAMIHA_DAY_0: u16 = AYYAMIHA_DAY_1 - 1;

#[derive(Debug)]
pub(crate) struct YearSpecifics {
    pub(crate) leapday: bool,
    pub(crate) naw_ruz_on_march_21: bool,
    #[allow(dead_code)]
    pub(crate) birth_of_bab: u16,
}

impl YearSpecifics {
    fn new(birth_of_bab: u16, leapday: bool, naw_ruz_on_march_21: bool) -> YearSpecifics {
        YearSpecifics {
            leapday,
            naw_ruz_on_march_21,
            birth_of_bab,
        }
    }
}

lazy_static! {
    // https://github.com/Soroosh/badi_date/blob/main/lib/bahai_holyday.dart
    pub(crate) static ref HOLY_DAYS_FALLBACK: BTreeMap<BahaiHolyDay, u16> = {
        let mut map = BTreeMap::new();
        map.insert(BahaiHolyDay::NawRuz, 1);
        map.insert(BahaiHolyDay::Ridvan1st, 32);
        map.insert(BahaiHolyDay::Ridvan9th, 40);
        map.insert(BahaiHolyDay::Ridvan12th, 43);
        map.insert(BahaiHolyDay::DeclarationOfTheBab, 65);
        map.insert(BahaiHolyDay::AscensionOfBahaullah, 70);
        map.insert(BahaiHolyDay::MartyrdomOfTheBab, 112);
        map.insert(BahaiHolyDay::BirthOfTheBab, 214);
        map.insert(BahaiHolyDay::BirthOfBahaullah, 237);
        map.insert(BahaiHolyDay::DayOfTheCovenant, 251);
        map.insert(BahaiHolyDay::AscensionOfAbdulBaha, 253);
        map
    };
    pub(crate) static ref START_OF_DAY_FALLBACK: NaiveTime =
        NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    pub(crate) static ref YEAR_SPECIFICS: HashMap<u8, YearSpecifics> = {
        let mut map = HashMap::new();
        map.insert(172, YearSpecifics::new(238, false, true));
        map.insert(173, YearSpecifics::new(227, false, false));
        map.insert(174, YearSpecifics::new(216, true, false));
        map.insert(175, YearSpecifics::new(234, false, true));
        map.insert(176, YearSpecifics::new(223, false, true));
        map.insert(177, YearSpecifics::new(213, false, false));
        map.insert(178, YearSpecifics::new(232, true, false));
        map.insert(179, YearSpecifics::new(220, false, true));
        map.insert(180, YearSpecifics::new(210, false, true));
        map.insert(181, YearSpecifics::new(228, false, false));
        map.insert(182, YearSpecifics::new(217, true, false));
        map.insert(183, YearSpecifics::new(235, false, true));
        map.insert(184, YearSpecifics::new(224, false, true));
        map.insert(185, YearSpecifics::new(214, false, false));
        map.insert(186, YearSpecifics::new(233, false, false));
        map.insert(187, YearSpecifics::new(223, true, false));
        map.insert(188, YearSpecifics::new(211, false, true));
        map.insert(189, YearSpecifics::new(230, false, false));
        map.insert(190, YearSpecifics::new(238, false, false));
        map.insert(191, YearSpecifics::new(238, true, false));
        map.insert(192, YearSpecifics::new(226, false, true));
        map.insert(193, YearSpecifics::new(215, false, false));
        map.insert(194, YearSpecifics::new(234, false, false));
        map.insert(195, YearSpecifics::new(224, true, false));
        map.insert(196, YearSpecifics::new(213, false, true));
        map.insert(197, YearSpecifics::new(232, false, false));
        map.insert(198, YearSpecifics::new(221, false, false));
        map.insert(199, YearSpecifics::new(210, true, false));
        map.insert(200, YearSpecifics::new(228, false, true));
        map.insert(201, YearSpecifics::new(217, false, false));
        map.insert(202, YearSpecifics::new(236, false, false));
        map.insert(203, YearSpecifics::new(225, true, false));
        map.insert(204, YearSpecifics::new(214, false, true));
        map.insert(205, YearSpecifics::new(233, false, false));
        map.insert(206, YearSpecifics::new(223, false, false));
        map.insert(207, YearSpecifics::new(212, true, false));
        map.insert(208, YearSpecifics::new(230, false, true));
        map.insert(209, YearSpecifics::new(219, false, false));
        map.insert(210, YearSpecifics::new(237, false, false));
        map.insert(211, YearSpecifics::new(227, true, false));
        map.insert(212, YearSpecifics::new(215, false, true));
        map.insert(213, YearSpecifics::new(234, false, false));
        map.insert(214, YearSpecifics::new(224, false, false));
        map.insert(215, YearSpecifics::new(213, false, false));
        map.insert(216, YearSpecifics::new(232, true, false));
        map.insert(217, YearSpecifics::new(220, false, false));
        map.insert(218, YearSpecifics::new(209, false, false));
        map.insert(219, YearSpecifics::new(228, false, false));
        map.insert(220, YearSpecifics::new(218, true, false));
        map.insert(221, YearSpecifics::new(236, false, false));
        map
    };
    pub(crate) static ref FIRST_GREGORIAN_DATE_SUPPORTED: DateTime<Tz> = Tehran
        .with_ymd_and_hms(YEAR_ONE_IN_GREGORIAN, 3, 21, 0, 0, 0)
        .unwrap();
    pub(crate) static ref LAST_GREGORIAN_DATE_SUPPORTED: DateTime<Tz> = Tehran
        .with_ymd_and_hms(LAST_GREGORIAN_YEAR_SUPPORTED, 3, 19, 0, 0, 0)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{YearSpecifics, YEAR_SPECIFICS};

    #[test]
    fn year_specifics() {
        let mut dart_code = String::new();

        let mut sorted_map: BTreeMap<u8, &YearSpecifics> = BTreeMap::new();
        for (year, specifics) in YEAR_SPECIFICS.iter() {
            sorted_map.insert(*year, specifics);
        }

        dart_code.push_str("const yearSpecifics = {\n");
        for (year, specifics) in sorted_map.iter() {
            dart_code.push_str(&format!(
                "  {}: YearSpecifics(year: {}, birthOfBab: {}",
                year, year, specifics.birth_of_bab
            ));
            if specifics.leapday {
                dart_code.push_str(", leapday: true")
            }
            if specifics.naw_ruz_on_march_21 {
                dart_code.push_str(", nawRuzOnMarch21: true")
            }
            dart_code.push_str("),\n");
        }
        dart_code.push_str("};\n");

        // from https://github.com/Soroosh/badi_date/blob/main/lib/years.dart
        let original_dart_code = r#"const yearSpecifics = {
  172: YearSpecifics(year: 172, birthOfBab: 238, nawRuzOnMarch21: true),
  173: YearSpecifics(year: 173, birthOfBab: 227),
  174: YearSpecifics(year: 174, birthOfBab: 216, leapday: true),
  175: YearSpecifics(year: 175, birthOfBab: 234, nawRuzOnMarch21: true),
  176: YearSpecifics(year: 176, birthOfBab: 223, nawRuzOnMarch21: true),
  177: YearSpecifics(year: 177, birthOfBab: 213),
  178: YearSpecifics(year: 178, birthOfBab: 232, leapday: true),
  179: YearSpecifics(year: 179, birthOfBab: 220, nawRuzOnMarch21: true),
  180: YearSpecifics(year: 180, birthOfBab: 210, nawRuzOnMarch21: true),
  181: YearSpecifics(year: 181, birthOfBab: 228),
  182: YearSpecifics(year: 182, birthOfBab: 217, leapday: true),
  183: YearSpecifics(year: 183, birthOfBab: 235, nawRuzOnMarch21: true),
  184: YearSpecifics(year: 184, birthOfBab: 224, nawRuzOnMarch21: true),
  185: YearSpecifics(year: 185, birthOfBab: 214),
  186: YearSpecifics(year: 186, birthOfBab: 233),
  187: YearSpecifics(year: 187, birthOfBab: 223, leapday: true),
  188: YearSpecifics(year: 188, birthOfBab: 211, nawRuzOnMarch21: true),
  189: YearSpecifics(year: 189, birthOfBab: 230),
  190: YearSpecifics(year: 190, birthOfBab: 238),
  191: YearSpecifics(year: 191, birthOfBab: 238, leapday: true),
  192: YearSpecifics(year: 192, birthOfBab: 226, nawRuzOnMarch21: true),
  193: YearSpecifics(year: 193, birthOfBab: 215),
  194: YearSpecifics(year: 194, birthOfBab: 234),
  195: YearSpecifics(year: 195, birthOfBab: 224, leapday: true),
  196: YearSpecifics(year: 196, birthOfBab: 213, nawRuzOnMarch21: true),
  197: YearSpecifics(year: 197, birthOfBab: 232),
  198: YearSpecifics(year: 198, birthOfBab: 221),
  199: YearSpecifics(year: 199, birthOfBab: 210, leapday: true),
  200: YearSpecifics(year: 200, birthOfBab: 228, nawRuzOnMarch21: true),
  201: YearSpecifics(year: 201, birthOfBab: 217),
  202: YearSpecifics(year: 202, birthOfBab: 236),
  203: YearSpecifics(year: 203, birthOfBab: 225, leapday: true),
  204: YearSpecifics(year: 204, birthOfBab: 214, nawRuzOnMarch21: true),
  205: YearSpecifics(year: 205, birthOfBab: 233),
  206: YearSpecifics(year: 206, birthOfBab: 223),
  207: YearSpecifics(year: 207, birthOfBab: 212, leapday: true),
  208: YearSpecifics(year: 208, birthOfBab: 230, nawRuzOnMarch21: true),
  209: YearSpecifics(year: 209, birthOfBab: 219),
  210: YearSpecifics(year: 210, birthOfBab: 237),
  211: YearSpecifics(year: 211, birthOfBab: 227, leapday: true),
  212: YearSpecifics(year: 212, birthOfBab: 215, nawRuzOnMarch21: true),
  213: YearSpecifics(year: 213, birthOfBab: 234),
  214: YearSpecifics(year: 214, birthOfBab: 224),
  215: YearSpecifics(year: 215, birthOfBab: 213),
  216: YearSpecifics(year: 216, birthOfBab: 232, leapday: true),
  217: YearSpecifics(year: 217, birthOfBab: 220),
  218: YearSpecifics(year: 218, birthOfBab: 209),
  219: YearSpecifics(year: 219, birthOfBab: 228),
  220: YearSpecifics(year: 220, birthOfBab: 218, leapday: true),
  221: YearSpecifics(year: 221, birthOfBab: 236),
};"#;

        for it in dart_code.lines().zip(original_dart_code.lines()) {
            let (a, b) = it;
            assert_eq!(a, b);
        }
    }
}
