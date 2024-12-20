use crate::{BadiDateError, BadiMonth};

/// Common trait for [`BadiDate`][`crate::BadiDate`] and [`LocalBadiDate`][`crate::LocalBadiDate`]
pub trait BadiDateLike: Clone {
    /// The Bahá’í Era/Badi year [1 - 221 supported] (year 1 starts 21 March 1844)
    fn year(&self) -> u8;

    /// The Badi month [1 - 19] or Ayyám-i-Há
    fn month(&self) -> BadiMonth;

    /// The Badi day [1 - min(19, Ayyám-i-Há days for the year)]
    fn day(&self) -> u16;

    /// Whether the current date is a Feast day
    fn is_feast(&self) -> bool {
        self.day() == 1 && self.month() != BadiMonth::AyyamIHa
    }

    /// The day of the current year (starting with 1 on Naw-Rúz)
    fn day_of_year(&self) -> u16;

    /// Returns new [`BadiDateLike`] with the given `year` (checks input for validity)
    fn with_year(&self, year: u8) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with the given `month` (checks input for validity)
    fn with_month(&self, month: BadiMonth) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with the given `day` (checks input for validity)
    fn with_day(&self, day: u16) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with the given `year`, `month`, and `day` (checks input for validity)
    fn with_ymd(&self, year: u8, month: BadiMonth, day: u16) -> Result<Self, BadiDateError>;

    /// Returns new [`BadiDateLike`] with the given `year` and **1-based** `day_of_year` (checks input for validity)
    fn with_year_and_doy(&self, year: u8, day_of_year: u16) -> Result<Self, BadiDateError>;
}

#[cfg(test)]
mod tests {
    use crate::{BadiDate, BadiDateLike, BadiMonth};

    #[test]
    fn test_is_feast() {
        let badi = BadiDate::new(181, BadiMonth::Month(19), 2).unwrap();
        assert!(!badi.is_feast());

        let badi = BadiDate::new(181, BadiMonth::Month(19), 1).unwrap();
        assert!(badi.is_feast());

        let badi = BadiDate::new(181, BadiMonth::AyyamIHa, 1).unwrap();
        assert!(!badi.is_feast());
    }
}
