use chrono::{DateTime, Datelike};
use chrono_tz::Tz;

use crate::{statics::*, BadiDateError, Coordinates, LocalBadiDate};

use super::util::*;

/// Provides methods to create a [`LocalBadiDate`] from a Gregorian [`DateTime`]
pub trait FromDateTime {
    /// Create a new LocalBadiDate given a local time-zoned date and coordinates
    fn from_datetime(
        date: DateTime<Tz>,
        coordinates: Option<Coordinates>,
    ) -> Result<LocalBadiDate, BadiDateError>;
}

impl FromDateTime for LocalBadiDate {
    // Bahá’í Calendar 2024: https://www.bahai.org/action/devotional-life/calendar/pdf-calendar
    // https://www.bahai.org/action/devotional-life/calendar
    // adapted from https://github.com/Soroosh/badi_date/blob/main/lib/badi_date.dart
    // and https://github.com/janrg/badiDate/blob/master/src/badiDate.ts
    fn from_datetime(
        date: DateTime<Tz>,
        coordinates: Option<Coordinates>,
    ) -> Result<Self, BadiDateError> {
        if date < *FIRST_GREGORIAN_DATE_SUPPORTED || date > *LAST_GREGORIAN_DATE_SUPPORTED {
            return Err(BadiDateError::DateNotSupported);
        }
        let last_sunset = get_last_sunset(&coordinates, date);
        // let next_sunset = get_next_sunset(&coordinates, date);
        let last_naw_ruz = get_sunset_of_last_naw_ruz(&coordinates, date);
        let year = (last_naw_ruz.year() - YEAR_ZERO_IN_GREGORIAN) as u8;
        let day_of_year: u16 =
            1 + (last_sunset.date_naive() - last_naw_ruz.date_naive()).num_days() as u16;
        let (month, day) = month_and_day_from_doy(year, day_of_year)?;
        Self::new(year, month, day, date.timezone(), coordinates)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone};
    use chrono_tz::Tz;

    use crate::{BadiMonth, Coordinates, FromDateTime, LocalBadiDate, ToDateTime};

    #[test]
    fn badi_date_from_datetime() {
        let denver: Tz = "America/Denver".parse().unwrap();
        let coords = Some(Coordinates::new(39.613319, -105.016647).unwrap());
        let test_dates: Vec<(String, DateTime<Tz>, LocalBadiDate)> = vec![
            (
                "2024 Naw Ruz before sunset".to_string(),
                denver.with_ymd_and_hms(2024, 3, 19, 12, 0, 0).unwrap(),
                LocalBadiDate::new(180, BadiMonth::Month(19), 19, denver, coords).unwrap(),
            ),
            (
                "2024 Naw Ruz after sunset".to_string(),
                denver.with_ymd_and_hms(2024, 3, 19, 20, 0, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(1), 1, denver, coords).unwrap(),
            ),
            (
                "2024 day after Naw Ruz before sunset".to_string(),
                denver.with_ymd_and_hms(2024, 3, 20, 0, 0, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(1), 1, denver, coords).unwrap(),
            ),
            (
                "2024 day after Naw Ruz after sunset".to_string(),
                denver.with_ymd_and_hms(2024, 3, 20, 20, 0, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(1), 2, denver, coords).unwrap(),
            ),
            (
                "2024 Jalal before sunset".to_string(),
                denver.with_ymd_and_hms(2024, 4, 7, 10, 32, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(1), 19, denver, coords).unwrap(),
            ),
            (
                "2024 Jalal after sunset".to_string(),
                denver.with_ymd_and_hms(2024, 4, 7, 19, 32, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(2), 1, denver, coords).unwrap(),
            ),
            (
                "Feast of ‘Izzat (Might) 181 B.E. before sunset".to_string(),
                denver.with_ymd_and_hms(2024, 9, 6, 10, 24, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(9), 19, denver, coords).unwrap(),
            ),
            (
                "Feast of ‘Izzat (Might) 181 B.E. after sunset".to_string(),
                denver.with_ymd_and_hms(2024, 9, 6, 19, 24, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(10), 1, denver, coords).unwrap(),
            ),
            (
                "Feast of ‘Izzat (Might) 181 B.E. after midnight".to_string(),
                denver.with_ymd_and_hms(2024, 9, 7, 0, 0, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(10), 1, denver, coords).unwrap(),
            ),
            (
                "Feast of Sharaf (Honour) 181 B.E. before sunset".to_string(),
                denver.with_ymd_and_hms(2024, 12, 29, 15, 45, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(15), 19, denver, coords).unwrap(),
            ),
            (
                "Feast of Sharaf (Honour) 181 B.E.".to_string(),
                denver.with_ymd_and_hms(2024, 12, 29, 16, 46, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(16), 1, denver, coords).unwrap(),
            ),
            (
                "some day".to_string(),
                denver.with_ymd_and_hms(2024, 6, 4, 0, 15, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(5), 1, denver, coords).unwrap(),
            ),
            (
                "Feast of Jalál (Glory) 182 B.E.".to_string(),
                denver.with_ymd_and_hms(2025, 4, 7, 19, 30, 0).unwrap(),
                LocalBadiDate::new(182, BadiMonth::Month(2), 1, denver, coords).unwrap(),
            ),
            (
                "Feast day".to_string(),
                denver.with_ymd_and_hms(2024, 6, 3, 20, 30, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(5), 1, denver, coords).unwrap(),
            ),
            (
                "Mulk day 19 2025".to_string(),
                denver.with_ymd_and_hms(2025, 2, 24, 16, 30, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(18), 19, denver, coords).unwrap(),
            ),
            (
                "Ayyám-i-Há day 1 2025".to_string(),
                denver.with_ymd_and_hms(2025, 2, 24, 18, 30, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::AyyamIHa, 1, denver, coords).unwrap(),
            ),
            (
                "Ayyám-i-Há day 4 2025".to_string(),
                denver.with_ymd_and_hms(2025, 2, 28, 16, 30, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::AyyamIHa, 4, denver, coords).unwrap(),
            ),
            (
                "Alá day 1 2025".to_string(),
                denver.with_ymd_and_hms(2025, 2, 28, 18, 30, 0).unwrap(),
                LocalBadiDate::new(181, BadiMonth::Month(19), 1, denver, coords).unwrap(),
            ),
        ];

        for (description, date, expected_badi) in test_dates {
            println!(
                "\n==================================\nTEST: {}, date: {}",
                description, date
            );
            let badi_date = LocalBadiDate::from_datetime(date, coords).unwrap();
            println!("date = {}", date);
            println!("badi_date = {}", badi_date);
            println!(
                "serde = {}",
                serde_json::to_string_pretty(&badi_date).unwrap()
            );
            assert_eq!(badi_date, expected_badi, "{}", description);
            println!(
                "date: {}, start: {}, end: {}",
                date,
                badi_date.start(),
                badi_date.end()
            );
            assert!(badi_date.start() <= date && date <= badi_date.end());
        }
    }
}
