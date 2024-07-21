mod badi_month;
pub use badi_month::BadiMonth;

mod local_badi_date;
pub use local_badi_date::*;

mod badi_date_like;
pub use badi_date_like::*;

mod local_badi_date_like;
pub use local_badi_date_like::*;

mod coordinates;
pub use coordinates::*;

mod from_datetime;
pub use from_datetime::*;

mod badi_date_ops;
pub use badi_date_ops::*;

mod to_datetime;
use serde::{Deserialize, Serialize};
pub use to_datetime::*;

mod util;

use crate::{BadiDateError, HolyDayProviding};
use util::*;

use std::fmt;

/// A structure that holds a date in the Badí‘ (Bahá’í) calendar without time zone or location info
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialOrd, PartialEq, Serialize)]
pub struct BadiDate {
    year: u8,
    month: BadiMonth,
    day: u16,
    #[serde(skip)]
    day_of_year: u16,
}

impl BadiDate {
    /// Create a new [`BadiDate`] given day, [`BadiMonth`], and year; checks for validity
    pub fn new(year: u8, month: BadiMonth, day: u16) -> Result<Self, BadiDateError> {
        validate(year, month, day)?;
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

    fn with_year_and_doy(&self, year: u8, day_of_year: u16) -> Result<Self, BadiDateError> {
        let (month, day) = match month_and_day_from_doy(year, day_of_year) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        Self::new(year, month, day)
    }
}

impl HolyDayProviding for BadiDate {}

impl fmt::Display for BadiDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0>3}-{:0>2}-{:0>2}",
            self.year,
            match self.month {
                BadiMonth::Month(month) => month,
                BadiMonth::AyyamIHa => 0,
            },
            self.day,
        )
    }
}
