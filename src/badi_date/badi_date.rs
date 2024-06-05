use super::util::*;
use crate::{BadiDateError, BadiDateLike, BadiMonth};

/// A structure that holds a date in the Badí‘ (Bahá’í) calendar without time zone or location info
#[derive(Debug, Clone, PartialEq)]
pub struct BadiDate {
    year: u8,
    month: BadiMonth,
    day: u16,
    day_of_year: u16,
}

impl BadiDate {
    /// Create a new [`BadiDate`] given day, [`BadiMonth`], and year; checks for validity
    pub fn new(year: u8, month: BadiMonth, day: u16) -> Result<Self, BadiDateError> {
        if let Err(err) = validate(year, month, day) {
            return Err(err);
        }
        let day_of_year = day_of_year(year, &month, day);
        Ok(Self {
            year,
            month,
            day,
            day_of_year,
        })
    }
}

impl BadiDateLike for BadiDate {
    fn year(&self) -> u8 {
        self.year
    }

    fn month(&self) -> BadiMonth {
        self.month
    }

    fn day(&self) -> u16 {
        self.day
    }

    fn day_of_year(&self) -> u16 {
        self.day_of_year
    }

    fn with_day(&self, day: u16) -> Result<BadiDate, BadiDateError> {
        Self::new(self.year, self.month, day)
    }

    fn with_month(&self, month: BadiMonth) -> Result<BadiDate, BadiDateError> {
        Self::new(self.year, month, self.day)
    }

    fn with_year(&self, year: u8) -> Result<Self, BadiDateError> {
        Self::new(year, self.month, self.day)
    }

    fn with_ymd(&self, year: u8, month: BadiMonth, day: u16) -> Result<BadiDate, BadiDateError> {
        Self::new(year, month, day)
    }
}
