use chrono::{DateTime, Datelike, Days, TimeZone};
use chrono_tz::Tz;
use now::DateTimeNow;

use crate::{statics::*, BadiDateError, BadiMonth, Coordinates};

/// Validate a Badi year, month, and day
pub(crate) fn validate(year: u8, month: BadiMonth, day: u16) -> Result<(), BadiDateError> {
    if !(1..=LAST_YEAR_SUPPORTED).contains(&year) {
        return Err(BadiDateError::DateNotSupported);
    }
    month.validate()?;
    let max_day = month.number_of_days(year);
    if day < 1 || day > max_day {
        let err = BadiDateError::DayInvalid(month, day, max_day);
        return Err(err);
    }
    Ok(())
}

/// Computes the sunset time of the current Badi year exactly at or before the given local datetime.
pub(crate) fn get_sunset_of_last_naw_ruz(
    coordinates: &Option<Coordinates>,
    date: DateTime<Tz>,
) -> DateTime<Tz> {
    let badi_year = (date.year() - YEAR_ZERO_IN_GREGORIAN) as u8;
    let specifics = YEAR_SPECIFICS.get(&badi_year);
    let day = if let Some(specifics) = specifics {
        if specifics.naw_ruz_on_march_21 {
            21
        } else {
            20
        }
    } else {
        21
    };
    let naw_ruz_date = date
        .with_month(3)
        .unwrap()
        .with_day(day)
        .unwrap()
        .beginning_of_day();
    let naw_ruz_sunset = get_last_sunset(coordinates, naw_ruz_date);
    if naw_ruz_sunset <= date {
        naw_ruz_sunset
    } else {
        get_sunset_of_last_naw_ruz(coordinates, naw_ruz_date - Days::new(364))
    }
}

// Computes the number of days in Ayyám-i-Há of the given Badi (B.E.) year
pub(crate) fn get_number_of_ayyamiha_days(year: u8) -> u16 {
    let specifics = YEAR_SPECIFICS.get(&year);
    if let Some(specifics) = specifics {
        if specifics.leapday {
            5
        } else {
            4
        }
    } else {
        let greg_year: i32 = i32::from(year) + YEAR_ONE_IN_GREGORIAN;
        let is_leap_year = greg_year % 4 == 0 && greg_year % 100 != 0 || greg_year % 400 == 0;
        if is_leap_year {
            5
        } else {
            4
        }
    }
}

/// Computes the sunset occuring on the date of the passed local DateTime
/// Passing `coordinates` as `None` will return `START_OF_DAY_FALLBACK` in `Tz` timezone
pub(crate) fn get_sunset(coordinates: &Option<Coordinates>, date: DateTime<Tz>) -> DateTime<Tz> {
    let fallback = date.with_time(*START_OF_DAY_FALLBACK).unwrap();
    let Some(coordinates) = coordinates else {
        return fallback;
    };
    let &Coordinates {
        latitude,
        longitude,
    } = coordinates;
    if !(-66. ..=66.).contains(&latitude) {
        return fallback;
    }
    let (_, sunset_timestamp) =
        sunrise::sunrise_sunset(latitude, longitude, date.year(), date.month(), date.day());
    date.timezone().from_utc_datetime(
        &DateTime::from_timestamp(sunset_timestamp, 0)
            .unwrap()
            .naive_utc(),
    )
}

// Computes the next sunset exactly at or after the passed local DateTime
pub(crate) fn get_next_sunset(
    coordinates: &Option<Coordinates>,
    date: DateTime<Tz>,
) -> DateTime<Tz> {
    let sunset = get_sunset(coordinates, date);
    if date >= sunset {
        return get_sunset(coordinates, date + Days::new(1));
    }
    sunset
}

/// Computes the previous sunset before the passed local DateTime
pub(crate) fn get_last_sunset(
    coordinates: &Option<Coordinates>,
    date: DateTime<Tz>,
) -> DateTime<Tz> {
    let sunset = get_sunset(coordinates, date);
    if date < sunset {
        return get_sunset(coordinates, date - Days::new(1));
    }
    sunset
}

pub(crate) fn month_and_day_from_doy(
    year: u8,
    doy: u16,
) -> Result<(BadiMonth, u16), BadiDateError> {
    if !(1..=LAST_YEAR_SUPPORTED).contains(&year) {
        return Err(BadiDateError::DateNotSupported);
    }
    let ayyamiha_days = get_number_of_ayyamiha_days(year);
    let doy_0 = doy - 1;
    if doy < AYYAMIHA_DAY_1 {
        let month = (doy_0 / 19 + 1) as u8;
        let day = doy_0 % 19 + 1;
        Ok((BadiMonth::Month(month), day))
    } else if doy < AYYAMIHA_DAY_1 + ayyamiha_days {
        Ok((BadiMonth::AyyamIHa, doy - AYYAMIHA_DAY_0))
    } else {
        let day: u16 = doy - (AYYAMIHA_DAY_0 + ayyamiha_days);
        Ok((BadiMonth::Month(19), day))
    }
}

// Computes the absolute 1-based day of the year given Badi year/month/day
pub(crate) fn day_of_year(year: u8, month: &BadiMonth, day: u16) -> u16 {
    match *month {
        BadiMonth::Month(month) => match month.cmp(&19) {
            std::cmp::Ordering::Less => 19 * (month - 1) as u16 + day,
            _ => {
                let ayyamiha_days = get_number_of_ayyamiha_days(year);
                AYYAMIHA_DAY_0 + ayyamiha_days + day
            }
        },
        BadiMonth::AyyamIHa => AYYAMIHA_DAY_0 + day,
    }
}
