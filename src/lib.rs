//! Dates for the Badí' (Bahá’í) calendar and conversions between Badí' and Gregorian dates.
//!
//! See [The Bahá’í Calendar at bahai.org](https://www.bahai.org/action/devotional-life/calendar).
//!
//! # Example: create [`BadiDate`] from local [`chrono::DateTime<Tz>`] and geo [`Coordinates`]
//!
//! ```
//! use badi_date::{BadiDate, BadiMonth, Coordinates, FromDateTime};
//! use chrono::TimeZone;
//! let denver: chrono_tz::Tz = "America/Denver".parse().unwrap();
//! let coords = Some(Coordinates::new(39.613319, -105.016647).unwrap());
//! let date = denver.with_ymd_and_hms(2024, 3, 19, 18, 0, 0).unwrap();
//! let badi_date = BadiDate::from_local(date, coords).unwrap();
//! assert_eq!(
//!     BadiDate::new(180, BadiMonth::Month(19), 19, denver, coords).unwrap(),
//!     badi_date,
//! );
//! ```
//!
//! # Example: create [`BadiDate`] from local [`chrono::DateTime<Tz>`] without [`Coordinates`]
//!
//! ```
//! use badi_date::{BadiDate, BadiMonth, Coordinates, FromDateTime};
//! use chrono::TimeZone;
//! let denver: chrono_tz::Tz = "America/Denver".parse().unwrap();
//! let date = denver.with_ymd_and_hms(2024, 3, 19, 18, 0, 0).unwrap();
//! let badi_date = BadiDate::from_local(date, None).unwrap();
//! assert_eq!(
//!     BadiDate::new(181, BadiMonth::Month(1), 1, denver, None).unwrap(),
//!     badi_date,
//! );
//! ```

#![warn(missing_docs)]
#![macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

mod statics;
use statics::*;

mod error;
pub use error::*;

mod badi_date;
pub use badi_date::*;
