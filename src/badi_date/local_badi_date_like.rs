use chrono_tz::Tz;

use crate::{BadiDateLike, Coordinates};

/// Provides localization attributes for [`LocalBadiDate`][`crate::LocalBadiDate`]
pub trait LocalBadiDateLike: BadiDateLike {
    /// The time zone used for conversion to/from Gregorian dates/times
    fn timezone(&self) -> Tz;

    /// The WGS84 GPS coordinates from which sunset is calculated
    /// These MUST match the time zone or sunset times will be wrong!
    fn coordinates(&self) -> Option<Coordinates>;
}
