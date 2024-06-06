use rust_i18n::t;

use crate::BadiDateError;

use super::util::*;

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq)]
/// Represents one of the 19 Bahá’í months or Ayyám-i-Há
pub enum BadiMonth {
    /// One of the 19 Badi/Bahá’í months (parameter is 1-based month number)
    Month(u8),
    /// The intercalary days of Ayyám-i-Há
    AyyamIHa,
}

impl PartialOrd for BadiMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            BadiMonth::Month(sm) => match other {
                BadiMonth::Month(om) => sm.partial_cmp(om),
                BadiMonth::AyyamIHa => {
                    if *sm == 19 {
                        Some(std::cmp::Ordering::Greater)
                    } else {
                        Some(std::cmp::Ordering::Less)
                    }
                }
            },
            BadiMonth::AyyamIHa => match other {
                BadiMonth::Month(om) => {
                    if *om == 19 {
                        Some(std::cmp::Ordering::Less)
                    } else {
                        Some(std::cmp::Ordering::Greater)
                    }
                }
                BadiMonth::AyyamIHa => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

impl BadiMonth {
    /// First month of the year (Bahá)
    pub fn first() -> Self {
        BadiMonth::Month(1)
    }

    /// Last month of the year (ʻAláʼ)
    pub fn last() -> Self {
        BadiMonth::Month(19)
    }

    /// Next month of the year (None if `self` is ʻAláʼ)
    pub fn next(&self) -> Option<Self> {
        match *self {
            BadiMonth::Month(month) => {
                if month < 18 {
                    Some(BadiMonth::Month(month + 1))
                } else if month == 18 {
                    Some(BadiMonth::AyyamIHa)
                } else {
                    None
                }
            }
            BadiMonth::AyyamIHa => Some(BadiMonth::Month(19)),
        }
    }

    /// Previous month of the year (None if `self` is Bahá)
    pub fn previous(&self) -> Option<Self> {
        match *self {
            BadiMonth::Month(month) => {
                if month == 1 {
                    None
                } else if month == 19 {
                    Some(BadiMonth::AyyamIHa)
                } else if month < 19 {
                    Some(BadiMonth::Month(month - 1))
                } else {
                    None
                }
            }
            BadiMonth::AyyamIHa => Some(BadiMonth::Month(18)),
        }
    }

    /// Return `self` if month is valid, otherwise [`BadiDateError::MonthInvalid`]
    pub fn validate(&self) -> Result<Self, BadiDateError> {
        if match *self {
            BadiMonth::Month(month) => month >= 1 && month <= 19,
            BadiMonth::AyyamIHa => true,
        } {
            return Ok(*self);
        }
        Err(BadiDateError::MonthInvalid(*self))
    }

    /// Max number of days in the month (year required to compute Ayyám-i-Há days)
    pub fn number_of_days(&self, year: u8) -> u16 {
        match *self {
            BadiMonth::Month(_) => 19,
            BadiMonth::AyyamIHa => get_number_of_ayyamiha_days(year),
        }
    }

    /// Arabic name of the month in the Arabic charater set
    pub fn arabic(&self) -> String {
        self.name("ar")
    }

    /// English name of the month (as opposed to transliteration)
    pub fn english(&self) -> String {
        self.name("en")
    }

    /// English transliteration of Arabic name of the month
    pub fn transliteration(&self) -> String {
        self.name("tl")
    }

    /// Additional meanings in authorized English translations of Baháʼí scripture
    /// <https://en.wikipedia.org/wiki/Bah%C3%A1%CA%BC%C3%AD_calendar>
    pub fn additional_meanings(&self) -> String {
        self.name("extra")
    }

    /// Get name of the month in a locale (see /locales/app.yaml):
    /// en: English
    /// ar: Arabic
    /// tl: English transliteration
    /// extra: Additional meanings (English)
    pub fn name(&self, locale: &str) -> String {
        match self {
            BadiMonth::Month(month) => {
                t!(format!("month.{}", month).as_str(), locale = locale).to_string()
            }
            BadiMonth::AyyamIHa => t!("ayyamiha", locale = locale).to_string(),
        }
    }

    /// Get a (debug) description of the month
    pub fn description(&self) -> String {
        match self {
            BadiMonth::Month(_) => format!("the month of {}", self.name("tl")),
            BadiMonth::AyyamIHa => self.name("tl"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn names() {
        assert_eq!(
            HashSet::<&str>::from_iter(vec!["ar", "en", "tl", "extra"]),
            HashSet::<&str>::from_iter(rust_i18n::available_locales!())
        );
        assert_eq!("Bahá", BadiMonth::first().transliteration());
        assert_eq!("كلمات", BadiMonth::Month(7).arabic());
        assert_eq!("Words", BadiMonth::Month(7).english());
        assert_eq!("Ayyám-i-Há", BadiMonth::AyyamIHa.transliteration());
    }
}
