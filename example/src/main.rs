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
