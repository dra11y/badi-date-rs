use crate::{BadiDateError, BadiDateLike, BahaiHolyDay};

/// Provides methods to get the current, previous, and next Bahá’í holy day
pub trait HolyDayProviding: BadiDateLike {
    /// Gets the holy day for `self.day` if it is a holy day
    fn holy_day(&self) -> Option<BahaiHolyDay> {
        BahaiHolyDay::holy_days_for_year(self.year())
            .get(&self.day_of_year())
            .cloned()
    }

    /// Gets the next holy day, if within the supported date range
    fn next_holy_day(&self) -> Result<Self, BadiDateError> {
        for year in [self.year(), self.year() + 1] {
            let after_day = if year == self.year() {
                self.day_of_year()
            } else {
                0
            };
            if let Some((day_of_year, _holy_day)) = BahaiHolyDay::holy_days_for_year(year)
                .into_iter()
                .find(|(day_of_year, _)| *day_of_year > after_day)
            {
                return self.with_year_and_doy(year, day_of_year);
            }
        }
        Err(BadiDateError::DateNotSupported)
    }

    /// Gets the previous holy day, if within the supported date range
    fn previous_holy_day(&self) -> Result<Self, BadiDateError> {
        for year in [self.year(), self.year() - 1] {
            let before_day = if year == self.year() {
                self.day_of_year()
            } else {
                366
            };
            if let Some((day_of_year, _holy_day)) = BahaiHolyDay::holy_days_for_year(year)
                .into_iter()
                .filter(|(day_of_year, _)| *day_of_year < before_day)
                .last()
            {
                return self.with_year_and_doy(year, day_of_year);
            }
        }
        Err(BadiDateError::DateNotSupported)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BadiDate, BadiDateOps, BadiMonth, BahaiHolyDay, HolyDayProviding};

    #[test]
    fn add_subtract_next_previous() {
        // 2 Nur 181 B.E. / 5 June 2024
        let badi = BadiDate::new(181, BadiMonth::Month(5), 2).unwrap();
        let next_holy_day: BadiDate = badi.next_holy_day().unwrap();
        assert_eq!(
            next_holy_day,
            // Martyrdom of the Báb
            BadiDate::new(181, BadiMonth::Month(6), 17).unwrap(),
        );
        assert_eq!(
            next_holy_day.holy_day(),
            Some(BahaiHolyDay::MartyrdomOfTheBab)
        );
        assert!(next_holy_day.holy_day().unwrap().work_suspended());
        let next_holy_day: BadiDate = next_holy_day.next_holy_day().unwrap();
        assert_eq!(
            next_holy_day,
            // Birth of the Báb
            BadiDate::new(181, BadiMonth::Month(12), 19).unwrap(),
        );
        assert_eq!(next_holy_day.holy_day(), Some(BahaiHolyDay::BirthOfTheBab));
        let next_holy_day: BadiDate = next_holy_day.next_holy_day().unwrap();
        assert_eq!(
            next_holy_day,
            // Birth of Bahá’u’lláh
            BadiDate::new(181, BadiMonth::Month(13), 1).unwrap(),
        );
        assert_eq!(
            next_holy_day.holy_day(),
            Some(BahaiHolyDay::BirthOfBahaullah)
        );
        let next_day = next_holy_day.next_day();
        assert_eq!(next_day.holy_day(), None);
        let next_holy_day: BadiDate = next_holy_day
            .next_holy_day()
            .unwrap()
            .next_holy_day()
            .unwrap();
        assert_eq!(
            next_holy_day,
            // Ascension of ‘Abdu’l-Bahá
            BadiDate::new(181, BadiMonth::Month(14), 6).unwrap(),
        );
        let prev_holy_day = badi.previous_holy_day().unwrap();
        assert_eq!(
            prev_holy_day,
            // Ascension of Bahá’u’lláh
            BadiDate::new(181, BadiMonth::Month(4), 13).unwrap(),
        );
        let prev_holy_day = prev_holy_day.previous_holy_day().unwrap();
        assert_eq!(
            prev_holy_day,
            // Declaration of the Báb
            BadiDate::new(181, BadiMonth::Month(4), 8).unwrap(),
        );

        let badi = BadiDate::new(181, BadiMonth::AyyamIHa, 2).unwrap();
        let next_holy_day: BadiDate = badi.next_holy_day().unwrap();
        assert_eq!(
            next_holy_day,
            // Naw-Rúz
            BadiDate::new(182, BadiMonth::Month(1), 1).unwrap(),
        );
        assert_eq!(next_holy_day.holy_day(), Some(BahaiHolyDay::NawRuz),);

        let previous_holy_day: BadiDate = next_holy_day.previous_holy_day().unwrap();
        assert_eq!(
            previous_holy_day,
            // Ascension of ‘Abdu’l-Bahá
            BadiDate::new(181, BadiMonth::Month(14), 6).unwrap(),
        );
        assert_eq!(
            previous_holy_day.holy_day(),
            Some(BahaiHolyDay::AscensionOfAbdulBaha),
        );
        assert!(!previous_holy_day.holy_day().unwrap().work_suspended(),);
    }
}
