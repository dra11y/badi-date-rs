# badi-date CHANGELOG

## 0.2.6
- FIX: fix some dropped value errors
- FIX: remove `println!`s
- FIX: `BadiDateOps::previous_feast()` no longer returns 1 Ayyám-i-Há if `self` during Ayyám-i-Há
- ADD: `BadiDateLike::is_feast()`
- ADD: `BadiDateOps::previous_or_current_feast()` and tests/more tests for `previous_feast()`
- CHORE: bump chrono-tz
- CHORE: add sensible `#[derive(...`s to enums

## 0.2.5
- FIX: corrected the incorrect calculations around/during Ayyám-i-Há
- ADD: impl std::fmt::Display and serde::Serialize/Deserialize

## 0.2.4
- minor clippy linter fixes for Rust best practices

## 0.2.3
- update CHANGELOG

## 0.2.2
- holy day names
- fix README example

## 0.2.1
- Add holy day support:
  - `BahaiHolyDay` enum
  - `HolyDayProviding` trait
    - next, previous, current holy day info for `BadiDateLike`

## 0.2.0

### Breaking changes
- BadiDate renamed to LocalBadiDate; BadiDate is now without timezone/coordinates
- parameters order: LocalBadiDate::new(year, month, day, timezone, coordinates) to be consistent with other date/time libraries (ymd)
- parameter types (day: u16, year: u8) - to help avoid mixing up order of variables
- LocalBadiDate timezone no longer optional
- rename `LocalBadiDate::from_local` to `LocalBadiDate::from_datetime`
- BadiDate is now "generic" without timezone/coordinates

### Minor changes
- rename `ToGregorian` and `FromLocal` traits to `ToDateTime` and `FromDateTime`
- fix documentation so it shows up in docs.rs

## 0.1.0

### Initial release
- `BadiDate` and `BadiMonth` types
- Creation of `BadiDate`s via `BadiDate::new`
- Conversion from local Gregorian to `BadiDate` via `BadiDate::from_local`
- Conversion from `BadiDate` to local Gregorian via `BadiDate::start()`, `::midnight()`, and `::end()`
