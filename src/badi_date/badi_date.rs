use chrono_tz::Tz;

use super::util::*;
use crate::{
    BadiDateError, BadiDateLike, BadiMonth, Coordinates, LocalBadiDateLike, NaiveBadiDate,
};

/// A structure that holds a date in the Badí‘ (Bahá’í) calendar with associated time zone and optional coordinates
#[derive(Debug, Clone, PartialEq)]
pub struct BadiDate {
    year: u8,
    month: BadiMonth,
    day: u16,
    day_of_year: u16,
    timezone: Tz,
    coordinates: Option<Coordinates>,
}

impl BadiDate {
    /// Create a "naive" [`BadiDate`] (somewhat like a [`chrono::NaiveDateTime`] but simpler)
    /// without time zone or location info.
    pub fn naive(year: u8, month: BadiMonth, day: u16) -> Result<NaiveBadiDate, BadiDateError> {
        NaiveBadiDate::new(year, month, day)
    }

    /// Create a new [`BadiDate`] given day, [`BadiMonth`], year,
    /// with optionals [`Coordinates`], [`chrono::Tz`]; checks for validity
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
        Self::new(self.year, self.month, day, self.timezone, self.coordinates)
    }

    fn with_ymd(&self, year: u8, month: BadiMonth, day: u16) -> Result<BadiDate, BadiDateError> {
        Self::new(year, month, day, self.timezone, self.coordinates)
    }

    fn with_month(&self, month: BadiMonth) -> Result<BadiDate, BadiDateError> {
        Self::new(self.year, month, self.day, self.timezone, self.coordinates)
    }

    fn with_year(&self, year: u8) -> Result<BadiDate, BadiDateError> {
        Self::new(year, self.month, self.day, self.timezone, self.coordinates)
    }
}

impl LocalBadiDateLike for BadiDate {
    fn timezone(&self) -> Tz {
        self.timezone
    }

    fn coordinates(&self) -> Option<Coordinates> {
        self.coordinates
    }
}
