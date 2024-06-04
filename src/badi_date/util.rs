use chrono::{DateTime, Datelike, Days, TimeZone};
use chrono_tz::Tz;
use now::DateTimeNow;

use crate::{statics::*, BadiMonth, Coordinates};

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
    let naw_ruz_sunset = get_last_sunset(&coordinates, naw_ruz_date);
    if naw_ruz_sunset <= date {
        naw_ruz_sunset
    } else {
        get_sunset_of_last_naw_ruz(&coordinates, naw_ruz_date - Days::new(364))
    }
}

// Computes the number of days in Ayyám-i-Há of the given Badi (B.E.) year
pub(crate) fn get_number_of_ayyamiha_days(year: u8) -> u8 {
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
    if latitude > 66. || latitude < -66. {
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

// Computes the absolute 1-based day of the year given Badi year/month/day
pub(crate) fn day_of_year(year: u8, month: &BadiMonth, day: u8) -> u64 {
    match *month {
        BadiMonth::Month(month) => {
            if month < 19 {
                19 * (month - 1) as u64 + day as u64
            } else {
                let ayyamiha_days = get_number_of_ayyamiha_days(year);
                AYYAMIHA_DAY_1 as u64 + ayyamiha_days as u64 + day as u64
            }
        }
        BadiMonth::AyyamIHa => AYYAMIHA_DAY_1 as u64 + day as u64,
    }
}
