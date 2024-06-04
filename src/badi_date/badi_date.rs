use chrono_tz::Tz;

use super::util::*;
use crate::{BadiDateError, BadiMonth, Coordinates, LAST_YEAR_SUPPORTED};

#[derive(Debug, Clone, PartialEq)]
pub struct BadiDate {
    // The Badi day [1 - min(19, Ayyám-i-Há days for the year)]
    pub day: u8,
    // The Badi month [1 - 19] or Ayyám-i-Há
    pub month: BadiMonth,
    // The Bahá’í Era/Badi year [1 - 221 supported] (current year - 1843)
    pub year: u8,
    // The day of the current year (starting with 1 on Naw-Rúz)
    pub day_of_year: u64,
    // The WGS84 GPS coordinates from which sunset is calculated
    pub coordinates: Option<Coordinates>,
    // The time zone used for conversion to/from Gregorian dates/times
    // This MUST match the coordinates or sunset times will be wrong!
    pub timezone: Option<Tz>,
}

impl BadiDate {
    // Create a new BadiDate given day, BadiMonth, year, with optionals coordinates, time zone
    pub fn new(
        day: u8,
        month: BadiMonth,
        year: u8,
        coordinates: Option<Coordinates>,
        timezone: Option<Tz>,
    ) -> Result<Self, BadiDateError> {
        if year < 1 || year > LAST_YEAR_SUPPORTED {
            return Err(BadiDateError::YearInvalid);
        }
        if !month.valid() {
            return Err(BadiDateError::MonthInvalid);
        }
        let max_day = month.number_of_days(year);
        if day < 1 || day > max_day {
            return Err(BadiDateError::DayInvalid(max_day, month));
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
