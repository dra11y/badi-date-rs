use std::fmt::Display;

use crate::{BadiMonth, LAST_YEAR_SUPPORTED};

#[derive(Debug)]
pub enum BadiDateError {
    DayInvalid(u8, BadiMonth),
    MonthInvalid,
    YearInvalid,
    DateNotSupported,
}

impl BadiDateError {
    pub fn message(&self) -> String {
        match self {
            BadiDateError::DayInvalid(max, month) => {
                format!(
                    "Day must be in the range [1-{}] for {}",
                    max,
                    month.description()
                )
            }
            BadiDateError::MonthInvalid => {
                "Month must be BadiMonth::AyyamIHa or BadiMonth::Month(month) in the range [1-19]"
                    .to_string()
            }
            BadiDateError::YearInvalid => {
                format!("Year must be in the range [1-{}]", LAST_YEAR_SUPPORTED)
            }
            BadiDateError::DateNotSupported => "Gregorian date is not supported".to_string(),
        }
    }
}

impl Display for BadiDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadiDateError: {}", self.message())
    }
}
