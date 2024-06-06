use std::collections::BTreeMap;

use crate::{HOLY_DAYS_FALLBACK, YEAR_SPECIFICS};

/// List of the 11 Bahá’í Holy Days (9 on which work is to be suspended)
/// See <https://www.bahai.org/action/devotional-life/calendar>
// Taken from: https://github.com/Soroosh/badi_date/blob/main/lib/bahai_holyday.dart
#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BahaiHolyDay {
    /// Naw-Rúz
    NawRuz,

    /// 1st day of Riḍván
    Ridvan1st,

    /// 9th day of Riḍván
    Ridvan9th,

    /// 12th day of Riḍván
    Ridvan12th,

    /// Declaration of the Báb
    DeclarationOfTheBab,

    /// Ascension of Bahá’u’lláh
    AscensionOfBahaullah,

    /// Martyrdom of the Báb
    MartyrdomOfTheBab,

    /// Birth of the Báb
    BirthOfTheBab,

    /// Birth of Bahá’u’lláh
    BirthOfBahaullah,

    /// Day of the Covenant (work not suspended)
    DayOfTheCovenant,

    /// Ascension of ‘Abdu’l-Bahá (work not suspended)
    AscensionOfAbdulBaha,
}

impl BahaiHolyDay {
    pub(crate) fn holy_days_for_year(year: u8) -> BTreeMap<u16, BahaiHolyDay> {
        let specifics: Option<&crate::YearSpecifics> = YEAR_SPECIFICS.get(&year);
        HOLY_DAYS_FALLBACK
            .iter()
            .map(|(k, v)| {
                let key = match specifics {
                    Some(specifics) => match k {
                        BahaiHolyDay::BirthOfTheBab => specifics.birth_of_bab,
                        BahaiHolyDay::BirthOfBahaullah => specifics.birth_of_bab + 1,
                        _ => *v,
                    },
                    None => *v,
                };
                (key, k.clone())
            })
            .collect()
    }

    /// Get the 1-based Badi day of the Badi year on which the holy day occurs in the given Badi `year`
    pub fn day_of_year(&self, year: u8) -> u16 {
        let day = *HOLY_DAYS_FALLBACK.get(self).unwrap();
        if [BahaiHolyDay::BirthOfTheBab, BahaiHolyDay::BirthOfBahaullah].contains(self) {
            return match YEAR_SPECIFICS.get(&year) {
                Some(specifics) => {
                    if self == &BahaiHolyDay::BirthOfTheBab {
                        specifics.birth_of_bab
                    } else {
                        specifics.birth_of_bab + 1
                    }
                }
                None => day,
            };
        }
        day
    }

    /// Whether work is to be suspended on this holy day
    pub fn work_suspended(&self) -> bool {
        ![
            BahaiHolyDay::DayOfTheCovenant,
            BahaiHolyDay::AscensionOfAbdulBaha,
        ]
        .contains(self)
    }
}
