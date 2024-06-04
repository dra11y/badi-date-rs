use chrono_tz::Tz;

use super::util::*;
use crate::{BadiDateError, BadiMonth, Coordinates, LAST_YEAR_SUPPORTED};

#[derive(Debug, Clone, PartialEq)]
pub struct BadiDate {
    pub day: u8,
    pub month: BadiMonth,
    pub year: u8,
    pub day_of_year: u64,
    pub coordinates: Option<Coordinates>,
    pub timezone: Option<Tz>,
}

impl BadiDate {
    pub fn new(
        day: u8,
        month: BadiMonth,
        year: u8,
        coordinates: Option<Coordinates>,
        timezone: Option<Tz>,
    ) -> Result<Self, BadiDateError> {
        if day < 1 || day > 19 {
            return Err(BadiDateError::DayInvalid);
        }
        if let BadiMonth::Month(month) = month {
            if month < 1 || month > 19 {
                return Err(BadiDateError::MonthInvalid);
            }
        }
        if year < 1 || year > LAST_YEAR_SUPPORTED {
            return Err(BadiDateError::YearInvalid);
        }
        let day_of_year = day_of_year(year, &month, day);
        Ok(Self {
            day,
            month,
            year,
            coordinates,
            timezone,
            day_of_year,
        })
    }
}
