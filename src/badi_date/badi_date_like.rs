use crate::{BadiDateError, BadiMonth};

/// Common trait for [`LocalBadiDate`][`crate::LocalBadiDate`] and [`BadiDate`][`crate::BadiDate`]
pub trait BadiDateLike: Clone {
    /// The Bahá’í Era/Badi year [1 - 221 supported] (year 1 starts 21 March 1844)
    fn year(&self) -> u8;

    /// The Badi month [1 - 19] or Ayyám-i-Há
    fn month(&self) -> BadiMonth;

    /// The Badi day [1 - min(19, Ayyám-i-Há days for the year)]
    fn day(&self) -> u16;

    /// The day of the current year (starting with 1 on Naw-Rúz)
    fn day_of_year(&self) -> u16;

    /// Returns new [`BadiDateLike`] with a different year (checks input for validity)
    fn with_year(&self, year: u8) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with a different month (checks input for validity)
    fn with_month(&self, month: BadiMonth) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with a different day (checks input for validity)
    fn with_day(&self, day: u16) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with a different year, month, and day (checks input for validity)
    fn with_ymd(&self, year: u8, month: BadiMonth, day: u16) -> Result<Self, BadiDateError>;
}
