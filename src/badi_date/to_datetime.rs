use chrono::{DateTime, Days, TimeZone};
use chrono_tz::Tz;
use now::DateTimeNow;

use super::util::*;
use crate::{statics::*, BadiDateLike, LocalBadiDate, LocalBadiDateLike};

/// Provides methods to convert [`LocalBadiDate`] to a local Gregorian [`DateTime<Tz>`]
pub trait ToDateTime {
    /// The moment of sunset of this LocalBadiDate in local time
    fn start(&self) -> DateTime<Tz>;
    /// The moment of sunset of the end of this / start of next LocalBadiDate in local time
    fn end(&self) -> DateTime<Tz>;
    /// Midnight (in local time) of this BadiDate
    fn midnight(&self) -> DateTime<Tz>;
}

impl ToDateTime for LocalBadiDate {
    fn start(&self) -> DateTime<Tz> {
        get_last_sunset(&self.coordinates(), self.midnight())
    }

    fn end(&self) -> DateTime<Tz> {
        get_next_sunset(&self.coordinates(), self.midnight())
    }

    fn midnight(&self) -> DateTime<Tz> {
        let tz = self.timezone();
        let naw_ruz_year = YEAR_ZERO_IN_GREGORIAN + self.year() as i32;
        let with_ymd_and_hms = tz.with_ymd_and_hms(naw_ruz_year, 3, 23, 0, 0, 0);
        let naw_ruz_date = with_ymd_and_hms.unwrap();
        let naw_ruz = get_sunset_of_last_naw_ruz(&self.coordinates(), naw_ruz_date);
        (naw_ruz + Days::new(self.day_of_year() as u64)).beginning_of_day()
    }
}
