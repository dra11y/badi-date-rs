use std::fmt::Display;

use crate::LAST_YEAR_SUPPORTED;

#[derive(Debug)]
pub enum BadiDateError {
    DayInvalid,
    MonthInvalid,
    YearInvalid,
    DateNotSupported,
}

impl BadiDateError {
    pub fn message(&self) -> String {
        match self {
            BadiDateError::DayInvalid => "Day must be in the range [1-19]".to_string(),
            BadiDateError::MonthInvalid => "Month must be in the range [1-19]".to_string(),
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
