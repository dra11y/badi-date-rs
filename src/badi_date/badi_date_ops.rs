use crate::{BadiDateLike, BadiMonth};

/// Determines resulting day in a returned [`BadiDate`] copy when adding/subtracting [`BadiMonth`]s
#[derive(Debug)]
pub enum DayChangeAction {
    /// Take the minimum of `self.day` and number of days in **resulting** [`BadiMonth`]
    Keep,
    /// Change to first day of the **resulting** [`BadiMonth`]
    FirstInMonth,
    /// Change to last day of the **resulting** [`BadiMonth`]
    LastInMonth,
}

impl Default for DayChangeAction {
    fn default() -> Self {
        DayChangeAction::Keep
    }
}

/// Provides methods to return a modified copy of a [`BadiDateLike`]
pub trait BadiDateOps<T>
where
    T: BadiDateLike,
{
    /// Returns new [`BadiDateLike`] of the next Feast (day 1 of next [`BadiMonth`]; **skips** [`BadiMonth::AyyamIHa`])
    fn next_feast(&self) -> T;
    /// Returns new [`BadiDateLike`] of the previous Feast (day 1 of [`BadiMonth::Month`] -- `self.month` is kept if `self.day` > 1), **skips** [`BadiMonth::AyyamIHa`])
    fn previous_feast(&self) -> T;
    /// Returns new [`BadiDateLike`] with number of days added (subtracted if negative) (increments `month` and `year` accordingly; **includes** [`BadiMonth::AyyamIHa`])
    fn add_days(&self, days: i32) -> T;
    /// Add (subtract if months negative) number of `months` to [`BadiDateLike`]
    /// * `day_change_action` - determines the resulting day (see [`DayChangeAction`])
    /// * `skip_ayyamiha` - whether to skip over the "month" of Ayyám-i-Há
    fn add_months(
        &self,
        months: i32,
        day_change_action: DayChangeAction,
        skip_ayyamiha: bool,
    ) -> Self;
    /// Convenience method to goto previous Badi month
    fn previous_month(&self, skip_ayyamiha: bool) -> T;
    /// Convenience method to goto next Badi month
    fn next_month(&self, skip_ayyamiha: bool) -> T;
    /// Convenience method to goto previous Badi day
    fn previous_day(&self) -> T;
    /// Convenience method to goto next Badi day
    fn next_day(&self) -> T;
}

impl<T> BadiDateOps<T> for T
where
    T: BadiDateLike,
{
    fn next_feast(&self) -> T {
        self.add_months(1, DayChangeAction::FirstInMonth, true)
    }

    fn previous_feast(&self) -> T {
        if self.day() == 1 {
            self.add_months(-1, DayChangeAction::FirstInMonth, true)
        } else {
            self.with_day(1).unwrap()
        }
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
        let mut day = self.day();
        let mut month = self.month();
        let mut year = self.year();
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
        Self::with_ymd(&self, year, month, day).unwrap()
    }

    fn add_days(&self, days: i32) -> Self {
        if days == 0 {
            return self.clone();
        }
        let abs_days = days.abs();
        let positive = days > 0;
        let mut day = self.day();
        let mut month = self.month();
        let mut year = self.year();
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
        Self::with_ymd(&self, year, month, day).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BadiDateOps, BadiMonth, DayChangeAction, NaiveBadiDate};

    #[test]
    fn add_subtract_next_previous() {
        let badi = NaiveBadiDate::new(181, BadiMonth::Month(1), 2).unwrap();
        assert_eq!(
            badi.next_day(),
            NaiveBadiDate::new(181, BadiMonth::Month(1), 3).unwrap()
        );
        assert_eq!(
            badi.add_days(-2),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 19).unwrap()
        );
        assert_eq!(badi.add_days(-2).add_days(2), badi);
        assert_eq!(
            badi.add_days(-21),
            NaiveBadiDate::new(180, BadiMonth::AyyamIHa, 4).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::AyyamIHa, 4)
                .unwrap()
                .add_days(1),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 1).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 19)
                .unwrap()
                .add_days(5),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 1).unwrap()
        );
        assert_eq!(
            badi.next_month(true),
            NaiveBadiDate::new(181, BadiMonth::Month(2), 2).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .next_month(true),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 17).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .next_month(false),
            NaiveBadiDate::new(180, BadiMonth::AyyamIHa, 4).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .add_months(2, DayChangeAction::Keep, false),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 4).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .add_months(2, DayChangeAction::Keep, true),
            NaiveBadiDate::new(181, BadiMonth::Month(1), 17).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .next_feast(),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 1).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 1)
                .unwrap()
                .next_feast(),
            NaiveBadiDate::new(180, BadiMonth::Month(19), 1).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .previous_feast(),
            NaiveBadiDate::new(180, BadiMonth::Month(18), 1).unwrap()
        );
        assert_eq!(
            NaiveBadiDate::new(180, BadiMonth::Month(18), 17)
                .unwrap()
                .previous_feast()
                .previous_feast(),
            NaiveBadiDate::new(180, BadiMonth::Month(17), 1).unwrap()
        );
    }
}
