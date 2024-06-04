use super::util::*;
use crate::{BadiDate, BadiMonth};

pub trait DateOps {
    fn add_days(&self, days: i32) -> Self;
    fn add_months(&self, months: i32, skip_ayyamiha: bool) -> Self;
    fn previous_month(&self, skip_ayyamiha: bool) -> Self;
    fn next_month(&self, skip_ayyamiha: bool) -> Self;
    fn previous_day(&self) -> Self;
    fn next_day(&self) -> Self;
}

impl DateOps for BadiDate {
    fn next_day(&self) -> Self {
        self.add_days(1)
    }

    fn previous_day(&self) -> Self {
        self.add_days(-1)
    }

    fn next_month(&self, skip_ayyamiha: bool) -> Self {
        self.add_months(1, skip_ayyamiha)
    }

    fn previous_month(&self, skip_ayyamiha: bool) -> Self {
        self.add_months(-1, skip_ayyamiha)
    }

    fn add_months(&self, months: i32, skip_ayyamiha: bool) -> Self {
        if months == 0 {
            return self.clone();
        }
        let abs_months = months.abs();
        let positive = months > 0;
        let mut day = self.day;
        let mut month = self.month;
        let mut year = self.year;
        let ayyamiha_days = get_number_of_ayyamiha_days(year);
        for _ in 0..abs_months {
            if positive {
                match month {
                    BadiMonth::Month(m) => {
                        if m == 18 {
                            if skip_ayyamiha {
                                month = BadiMonth::Month(19);
                            } else {
                                month = BadiMonth::AyyamIHa;
                                day = day.min(ayyamiha_days);
                            }
                        } else if m == 19 {
                            month = BadiMonth::Month(1);
                            year += 1;
                        } else {
                            month = BadiMonth::Month(m + 1);
                        }
                    }
                    BadiMonth::AyyamIHa => {
                        month = BadiMonth::Month(19);
                    }
                };
            } else {
                match month {
                    BadiMonth::Month(m) => {
                        if m == 19 {
                            if skip_ayyamiha {
                                month = BadiMonth::Month(18);
                            } else {
                                month = BadiMonth::AyyamIHa;
                                day = day.min(ayyamiha_days);
                            }
                        } else if m == 1 {
                            year -= 1;
                            month = BadiMonth::Month(19);
                        } else {
                            month = BadiMonth::Month(m - 1);
                        }
                    }
                    BadiMonth::AyyamIHa => {
                        month = BadiMonth::Month(18);
                    }
                };
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

    fn add_days(&self, days: i32) -> Self {
        if days == 0 {
            return self.clone();
        }
        let abs_days = days.abs();
        let positive = days > 0;
        let mut day = self.day;
        let mut month = self.month;
        let mut year = self.year;
        let ayyamiha_days = get_number_of_ayyamiha_days(year);
        for _ in 0..abs_days {
            if positive {
                day += 1;
                match month {
                    BadiMonth::Month(m) => {
                        if day > 19 {
                            day = 1;
                            if m == 18 {
                                month = BadiMonth::AyyamIHa;
                            } else if m == 19 {
                                month = BadiMonth::Month(1);
                                year += 1;
                            } else {
                                month = BadiMonth::Month(m + 1);
                            }
                        }
                    }
                    BadiMonth::AyyamIHa => {
                        if day > ayyamiha_days {
                            day = 1;
                            month = BadiMonth::Month(19);
                        }
                    }
                };
            } else {
                day -= 1;
                if day < 1 {
                    match month {
                        BadiMonth::Month(m) => {
                            if m == 19 {
                                month = BadiMonth::AyyamIHa;
                                day = ayyamiha_days;
                            } else if m == 1 {
                                year -= 1;
                                month = BadiMonth::Month(19);
                                day = 19;
                            } else {
                                month = BadiMonth::Month(m - 1);
                                day = 19;
                            }
                        }
                        BadiMonth::AyyamIHa => {
                            month = BadiMonth::Month(18);
                            day = 19;
                        }
                    };
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
    use crate::{BadiDate, BadiMonth, DateOps};

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
                .add_months(2, false),
            BadiDate::new(4, BadiMonth::Month(19), 180, None, None).unwrap()
        );
        assert_eq!(
            BadiDate::new(17, BadiMonth::Month(18), 180, None, None)
                .unwrap()
                .add_months(2, true),
            BadiDate::new(17, BadiMonth::Month(1), 181, None, None).unwrap()
        );
    }
}
