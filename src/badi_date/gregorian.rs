use chrono::{DateTime, Days, TimeZone};
use chrono_tz::Tz;
use now::DateTimeNow;

use super::util::*;
use crate::{BadiDate, YEAR_ZERO_IN_GREGORIAN};

pub trait Gregorian {
    fn start(&self) -> DateTime<Tz>;
    fn end(&self) -> DateTime<Tz>;
}

impl Gregorian for BadiDate {
    fn start(&self) -> DateTime<Tz> {
        let tz = self.timezone.unwrap_or_default();
        let naw_ruz_year = YEAR_ZERO_IN_GREGORIAN + self.year as i32;
        let with_ymd_and_hms = tz.with_ymd_and_hms(naw_ruz_year, 3, 23, 0, 0, 0);
        let naw_ruz_date = with_ymd_and_hms.unwrap();
        let naw_ruz = get_sunset_of_last_naw_ruz(&self.coordinates, naw_ruz_date);
        let date = (naw_ruz + Days::new(self.day_of_year)).beginning_of_day();
        get_last_sunset(&self.coordinates, date)
    }

    fn end(&self) -> DateTime<Tz> {
        get_next_sunset(&self.coordinates, self.start())
    }
}
