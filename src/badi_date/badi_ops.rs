use super::util::*;
use crate::{BadiDate, BadiDateError, BadiMonth};

// Determines resulting day in a BadiDate when adding/subtracting Badi months
#[derive(Debug)]
pub enum DayChangeAction {
    // Take the minimum of self.day and number of days in resulting Badi month
    Keep,
    // Change to first day of the resulting Badi month
    FirstInMonth,
    // Change to last day of the resulting Badi month
    LastInMonth,
}

pub trait DateOps {
    // Copy of the current BadiDate with a different day (checks input for validity)
    fn with_day(&self, day: u8) -> Result<BadiDate, BadiDateError>;
    // BadiDate of the next Feast (day 1 of next Badi month, skipping Ayyám-i-Há)
    fn next_feast(&self) -> Self;
    // BadiDate of the previous Feast (day 1 of this -- if self.day > 1 -- or previous Badi month, skipping Ayyám-i-Há)
    fn previous_feast(&self) -> Self;
    // Add (subtract if days negative) number of days to BadiDate (increments month and year accordingly; does NOT skip Ayyám-i-Há)
    fn add_days(&self, days: i32) -> Self;
    /// Add (subtract if months negative) number of months to BadiDate
    /// day_change_action: determines the resulting day (see DayChangeAction)
    /// skip_ayyamiha: whether to skip over the "month" of Ayyám-i-Há
    fn add_months(
        &self,
        months: i32,
        day_change_action: DayChangeAction,
        skip_ayyamiha: bool,
    ) -> Self;
    // Convenience method to goto previous Badi month
    fn previous_month(&self, skip_ayyamiha: bool) -> Self;
    // Convenience method to goto next Badi month
    fn next_month(&self, skip_ayyamiha: bool) -> Self;
    // Convenience method to goto previous Badi day
    fn previous_day(&self) -> Self;
    // Convenience method to goto next Badi day
    fn next_day(&self) -> Self;
}

impl DateOps for BadiDate {
    fn next_feast(&self) -> Self {
        self.add_months(1, DayChangeAction::FirstInMonth, true)
    }

    fn previous_feast(&self) -> Self {
        if self.day == 1 {
            self.add_months(-1, DayChangeAction::FirstInMonth, true)
        } else {
            self.with_day(1).unwrap()
        }
    }

    fn with_day(&self, day: u8) -> Result<BadiDate, BadiDateError> {
        let max_day = self.month.number_of_days(self.year);
        if day < 1 || day > max_day {
            return Err(BadiDateError::DayInvalid(max_day, self.month));
        }
        Ok(Self {
            day,
            month: self.month,
            year: self.year,
            day_of_year: day_of_year(self.year, &self.month, day),
            coordinates: self.coordinates,
            timezone: self.timezone,
        })
    }

    fn next_day(&self) -> Self {
        self.add_days(1)
    }

    fn previous_day(&self) -> Self {
        self.add_days(-1)
    }

    fn next_month(&self, skip_ayyamiha: bool) -> Self {
        self.add_months(1, DayChangeAction::Keep, skip_ayyamiha)
    }

    fn previous_month(&self, skip_ayyamiha: bool) -> Self {
        self.add_months(-1, DayChangeAction::Keep, skip_ayyamiha)
    }

    fn add_months(
        &self,
        months: i32,
        day_change_action: DayChangeAction,
        skip_ayyamiha: bool,
    ) -> Self {
        if months == 0 {
            return self.clone();
        }
        let abs_months = months.abs();
        let positive = months > 0;
        let mut day = self.day;
        let mut month = self.month;
        let mut year = self.year;
        for _ in 0..abs_months {
            if positive {
                if let Some(m) = month.next() {
                    if skip_ayyamiha && m == BadiMonth::AyyamIHa {
                        month = m.next().unwrap();
                    } else {
                        month = m;
                    }
                } else {
                    month = BadiMonth::first();
                    year += 1;
                }
            } else {
                if let Some(m) = month.previous() {
                    if skip_ayyamiha && m == BadiMonth::AyyamIHa {
                        month = m.previous().unwrap();
                    } else {
                        month = m;
                    }
                } else {
                    month = BadiMonth::last();
                    year -= 1;
                }
            }
            match day_change_action {
                DayChangeAction::Keep => day = day.min(month.number_of_days(year)),
                DayChangeAction::FirstInMonth => day = 1,
                DayChangeAction::LastInMonth => day = month.number_of_days(year),
            };
        }
        Self {
            day,
            month,
            year,
            day_of_year: day_of_year(year, &month, day),
            coordinates: self.coordinates,
            timezone: self.timezone,
        }
    }

    fn add_days(&self, days: i32) -> Self {
        if days == 0 {
            return self.clone();
        }
        let abs_days = days.abs();
        let positive = days > 0;
        let mut day = self.day;
        let mut month = self.month;
        let mut year = self.year;
        for _ in 0..abs_days {
            if positive {
                day += 1;
                let max_day = month.number_of_days(year);
                if day > max_day {
                    day = 1;
                    if let Some(m) = month.next() {
                        month = m;
                    } else {
                        month = BadiMonth::first();
                        year += 1;
                    }
                }
            } else {
                day -= 1;
                if day < 1 {
                    if let Some(m) = month.previous() {
                        month = m;
                        day = m.number_of_days(year);
                    } else {
                        month = BadiMonth::last();
                        year -= 1;
                        day = 19;
                    }
                }
            }
        }
        Self {
            day,
            month,
            year,
            day_of_year: day_of_year(year, &month, day),
            coordinates: self.coordinates,
            timezone: self.timezone,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BadiDate, BadiMonth, DateOps, DayChangeAction};

    #[test]
    fn add_subtract_next_previous() {
        let badi = BadiDate::new(2, BadiMonth::Month(1), 181, None, None).unwrap();
        assert_eq!(
            badi.next_day(),
            BadiDate::new(3, BadiMonth::Month(1), 181, None, None).unwrap()
        );
        assert_eq!(
            badi.add_days(-2),
            BadiDate::new(19, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(badi.add_days(-2).add_days(2), badi);
        assert_eq!(
            badi.add_days(-21),
            BadiDate::new(4, BadiMonth::AyyamIHa, 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(4, BadiMonth::AyyamIHa, 180, None, None)
                .unwrap()
                .add_days(1),
            BadiDate::new(1, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(19, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .add_days(5),
            BadiDate::new(1, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            badi.next_month(true),
            BadiDate::new(2, BadiMonth::Month(2), 181, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .next_month(true),
            BadiDate::new(17, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .next_month(false),
            BadiDate::new(4, BadiMonth::AyyamIHa, 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .add_months(2, DayChangeAction::Keep, false),
            BadiDate::new(4, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .add_months(2, DayChangeAction::Keep, true),
            BadiDate::new(17, BadiMonth::Month(1), 181, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .next_feast(),
            BadiDate::new(1, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(1, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .next_feast(),
            BadiDate::new(1, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .previous_feast(),
            BadiDate::new(1, BadiMonth::Month(18), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .previous_feast()
                .previous_feast(),
            BadiDate::new(1, BadiMonth::Month(17), 180, None, None).unwrap()
        );
    }
}
