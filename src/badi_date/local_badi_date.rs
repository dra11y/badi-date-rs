use chrono_tz::Tz;

use super::util::*;
use crate::{BadiDateError, BadiDateLike, BadiMonth, Coordinates, LocalBadiDateLike};

/// A structure that holds a date in the Badí‘ (Bahá’í) calendar with associated time zone and optional coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct LocalBadiDate {
    year: u8,
    month: BadiMonth,
    day: u16,
    day_of_year: u16,
    timezone: Tz,
    coordinates: Option<Coordinates>,
}

impl LocalBadiDate {
    /// Create a new [`LocalBadiDate`] given day, [`BadiMonth`], year,
    /// with optionals [`Coordinates`], [`chrono_tz::Tz`]; checks for validity
    pub fn new(
        year: u8,
        month: BadiMonth,
        day: u16,
        timezone: Tz,
        coordinates: Option<Coordinates>,
    ) -> Result<Self, BadiDateError> {
        if let Err(err) = validate(year, month, day) {
            return Err(err);
        }
        let day_of_year = day_of_year(year, &month, day);
        Ok(Self {
            year,
            month,
            day,
            coordinates,
            timezone,
            day_of_year,
        })
    }
}

impl BadiDateLike for LocalBadiDate {
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

    fn with_day(&self, day: u16) -> Result<LocalBadiDate, BadiDateError> {
        Self::new(self.year, self.month, day, self.timezone, self.coordinates)
    }

    fn with_ymd(
        &self,
        year: u8,
        month: BadiMonth,
        day: u16,
    ) -> Result<LocalBadiDate, BadiDateError> {
        Self::new(year, month, day, self.timezone, self.coordinates)
    }

    fn with_month(&self, month: BadiMonth) -> Result<LocalBadiDate, BadiDateError> {
        Self::new(self.year, month, self.day, self.timezone, self.coordinates)
    }

    fn with_year(&self, year: u8) -> Result<LocalBadiDate, BadiDateError> {
        Self::new(year, self.month, self.day, self.timezone, self.coordinates)
    }
}

impl LocalBadiDateLike for LocalBadiDate {
    fn timezone(&self) -> Tz {
        self.timezone
    }

    fn coordinates(&self) -> Option<Coordinates> {
        self.coordinates
    }
}
