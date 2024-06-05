use badi_date::{BadiMonth, Coordinates, FromDateTime, LocalBadiDate, ToDateTime};
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
    let badi_date = LocalBadiDate::from_datetime(date, coords).unwrap();
    assert_eq!(
        LocalBadiDate::new(180, BadiMonth::Month(19), 19, denver, coords).unwrap(),
        badi_date,
    );
    println!("date: {:?}\nbadi_date: {:?}", date, badi_date);

    // Test a dynamic date/time
    let now = denver.now();
    let badi_now = LocalBadiDate::from_datetime(now, coords).unwrap();
    assert!(badi_now.start() <= now && badi_now.end() >= now);
    println!(
        "now: {:?}\nbadi_now: {:?}\nstart: {:?}\nend: {:?}",
        now,
        badi_now,
        badi_now.start(),
        badi_now.end()
    );

    // Test fallback conversion (no coordinates)
    let badi_fallback = LocalBadiDate::from_datetime(date, None).unwrap();
    assert_eq!(
        LocalBadiDate::new(181, BadiMonth::Month(1), 1, denver, None).unwrap(),
        badi_fallback,
    );
    println!("date: {:?}\nbadi_fallback: {:?}", date, badi_fallback);
}
