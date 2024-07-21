use std::fmt::Display;

use crate::{BadiMonth, LAST_YEAR_SUPPORTED};

/// Error returned from trying to construct a [`BadiDateLike`][`crate::BadiDateLike`] with invalid parameters
#[derive(Debug)]
pub enum BadiDateError {
    /// The day number passed in for a given [`BadiMonth`] is invalid for that month
    DayInvalid(BadiMonth, u16, u16),
    /// The [`BadiMonth`] itself is invalid (due to an invalid day number)
    MonthInvalid(BadiMonth),
    /// The date passed in is not in the supported range
    DateNotSupported,
}

impl BadiDateError {
    /// Message associated with the [`BadiDateError`]
    pub fn message(&self) -> String {
        match self {
            BadiDateError::DayInvalid(month, day, max_day) => {
                format!(
                    "ERROR: Invalid Badi month: day {} is not in the range [1-{}] for {}",
                    day,
                    max_day,
                    month.description()
                )
            }
            BadiDateError::MonthInvalid(month) => match month {
                BadiMonth::Month(month) => {
                    format!(
                        "ERROR: BadiMonth::Month({}) is not in the range [1-19]",
                        month
                    )
                }
                BadiMonth::AyyamIHa => String::new(),
            },
            BadiDateError::DateNotSupported => {
                format!(
                    "The given date is not supported; year must be in the range [1-{}]",
                    LAST_YEAR_SUPPORTED
                )
            }
        }
    }
}

impl Display for BadiDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadiDateError: {}", self.message())
    }
}
