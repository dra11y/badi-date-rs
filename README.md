# badi-date

A Rust crate that provides types and conversions between the Gregorian and Badi (Bahá’í) calendars for the Rust language.

## CHANGELOG

- [ ] TODO: Add CHANGELOG.md file

This crate is a work-in-progress and seems to be working correctly with initial tests. Edge cases have not been tested yet.

- 0.1.0 (initial release)
  - `BadiDate` and `BadiMonth` types
  - Creation of `BadiDate`s via `BadiDate::new`
  - Conversion from local Gregorian to `BadiDate` via `BadiDate::from_local`
  - Conversion from `BadiDate` to local Gregorian via `BadiDate::start()`, `::midnight()`, and `::end()`

## Installation

```
cargo add badi-date@0.1
```

## Usage

(See `example/` folder.)

```bash
cargo new test_badi_date
cd test_badi_date
cargo add badi_date
cargo add chrono@0.4
cargo add chrono-tz@0.9
cargo add now@0.1
```

`main.rs`
```rust
use badi_date::{BadiDate, BadiMonth, Coordinates, FromLocal, ToGregorian};
use chrono::TimeZone;
use chrono_tz::Tz;
use now::TimeZoneNow;

fn main() {
    // Replace with your timezone / WGS84 GPS coordinates
    let denver: Tz = "America/Denver".parse().unwrap();
    // WARNING! Setting `coordinates` to `None` will return fallback time `badi_date::statics::START_OF_DAY_FALLBACK`
    let coords = Some(Coordinates::new(39.613319, -105.016647).unwrap());

    // Test a specific date/time before actual sunset
    let date = denver.with_ymd_and_hms(2024, 3, 19, 18, 0, 0).unwrap();
    let badi_date = BadiDate::from_local(date, coords).unwrap();
    assert_eq!(
        BadiDate::new(19, BadiMonth::Month(19), 180, coords, Some(denver)).unwrap(),
        badi_date,
    );
    println!("date: {:?}\nbadi_date: {:?}", date, badi_date);

    // Test a dynamic date/time
    let now = denver.now();
    let badi_now = BadiDate::from_local(now, coords).unwrap();
    assert!(badi_now.start() <= now && badi_now.end() >= now);
    println!(
        "now: {:?}\nbadi_now: {:?}\nstart: {:?}\nend: {:?}",
        now,
        badi_now,
        badi_now.start(),
        badi_now.end()
    );

    // Test fallback conversion (no coordinates)
    let badi_fallback = BadiDate::from_local(date, None).unwrap();
    assert_eq!(
        BadiDate::new(1, BadiMonth::Month(1), 181, None, Some(denver)).unwrap(),
        badi_fallback,
    );
    println!("date: {:?}\nbadi_fallback: {:?}", date, badi_fallback);
}
```

## Contributions / Feedback Welcome

- [ ] TODO: Add CONTRIBUTING.md file

Contributions / feedback / **test cases** in interesting locations are very welcome. Please also let me know if any documentation is not clear.

This crate has an MIT license so, by opening an issue or pull request, you agree that your contribution also carries this license and becomes a public part of this project.

## Background

The Bahá’í calendar, known as the Badí‘ calendar, is a **solar** calendar that simplifies the year with 19 months of 19 days each, with 4 or 5 intercalary days known as Ayyám-i-Há between months 18 and 19, and starts with year 1 on sunset, Naw-Rúz, 21 March, 1844.

Each Bahá’í / Badi day starts at sunset in the local area (thus, like other calendars, the start of the day is “rolling” across the world from east to west). In the case of extreme north and south latitudes, the start of day time is instead fixed by clocks (usually 18:00 **). This crate includes calculation of local sunset times for a given latitude, longitude, and time zone.

- [ ] TODO: ** check guidance on fixed clock time and update `statics::START_OF_DAY_FALLBACK` as necessary

The current Bahá’í Era (B.E.) year can be calculated by subtracting 1844 from the current Gregorian year prior to Naw-Rúz, or 1843 from the current Gregorian year after Naw-Rúz. Another way:
```
bahai_year = current_gregorian_year - 1844
if current_date >= naw_ruz_of_current_gregorian_year
    bahai_year = bahai_year + 1
```

Ayyám-i-Há (the intercalary days) is fixed to start on the 342nd day of each Badi year, thus the month and day of the month prior to Ayyám-i-Há are calculated as follows:
```
badi_month = 1 + (day_of_badi_year - 1) / 19
badi_day = 1 + (day_of_badi_year - 1) % 19
```

and after Ayyám-i-Há as follows:
```
badi_month = 19
badi_day = day_of_badi_year - (342 + number_of_days_in_ayyamiha)
```


> “The Festival of Naw-Rúz falleth on the day that the sun entereth the sign of Aries, even should this occur no more than one minute before sunset.” – Bahá’u’lláh

Naw-Rúz falls on the date the vernal (spring) equinox occurs between sunrise and sunset in Ṭihrán, Iran (the birthplace of Bahá’u’lláh). Due to the Gregorian calendar not being a true solar calendar, Naw-Rúz shifts between 20 and 21 March each year.

> “The adoption of a new calendar in each dispensation is a symbol of the power of Divine Revelation to reshape human perception of material, social, and spiritual reality. Through it, sacred moments are distinguished, humanity’s place in time and space reimagined, and the rhythm of life recast.” — <cite>[The Universal House of Justice, 10 July 2014](https://www.bahai.org/library/authoritative-texts/the-universal-house-of-justice/messages/20140710_001/1)</cite>

### Equivalent packages in other languages

Thanks to the following authors for their work that inspired / contributed to this crate:

| Language      | Package       | Author       |
| ------------- | ------------- | ------------ |
| JavaScript / TypeScript | [badidate](https://github.com/janrg/badiDate) | @janrg |
| Dart | [badi_date](https://github.com/Soroosh/badi_date) | @Soroosh |

### Links

- [The Bahá’í Calendar (Official site of the Bahá’í World)](https://www.bahai.org/action/devotional-life/calendar)

- [Baháʼí calendar (Wikipedia)](https://en.wikipedia.org/wiki/Bah%C3%A1%CA%BC%C3%AD_calendar)

- [Live Badi Calendar at badi-calendar.com (Location permission required)](https://www.badi-calendar.com/today)
