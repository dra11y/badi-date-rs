#![macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

mod statics;
use statics::*;

mod error;
pub use error::*;

mod badi_date;
pub use badi_date::*;
