use std::fmt;

use super::util::*;
use crate::{
    BadiDateError, BadiDateLike, BadiMonth, Coordinates, HolyDayProviding, LocalBadiDateLike,
    ToDateTime,
};
use chrono_tz::{OffsetName, Tz};
use serde::{Deserialize, Serialize};

/// A structure that holds a date in the Badí‘ (Bahá’í) calendar with associated time zone and optional coordinates
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocalBadiDate {
    year: u8,
    month: BadiMonth,
    day: u16,
    #[serde(skip)]
    day_of_year: u16,
    timezone: Tz,
    coordinates: Option<Coordinates>,
}

impl Eq for LocalBadiDate {}

impl PartialEq for LocalBadiDate {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.day_of_year == other.day_of_year
            && self.timezone == other.timezone
    }
}

impl Ord for LocalBadiDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_start = self.start().timestamp();
        let other_start = other.start().timestamp();
        self_start.cmp(&other_start)
    }
}

impl PartialOrd for LocalBadiDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for LocalBadiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>3}-{:0>2}-{:0>2} {}{}",
            self.year,
            match self.month {
                BadiMonth::Month(month) => month,
                BadiMonth::AyyamIHa => 0,
            },
            self.day,
            match self.coordinates {
                Some(coords) => format!("{} ", coords),
                None => String::default(),
            },
            self.start().offset().abbreviation(),
        )
    }
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
        validate(year, month, day)?;
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

    fn with_year_and_doy(&self, year: u8, day_of_year: u16) -> Result<Self, BadiDateError> {
        let (month, day) = match month_and_day_from_doy(year, day_of_year) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        Self::new(year, month, day, self.timezone, self.coordinates)
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

impl HolyDayProviding for LocalBadiDate {}
