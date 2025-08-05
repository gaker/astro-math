use crate::time::{j2000_days, julian_date};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

const EPSILON: f64 = 1e-6;

fn parse_iso_or_bce(iso: &str) -> DateTime<Utc> {
    if iso.starts_with('-') {
        // Manually parse BCE date
        let split: Vec<&str> = iso
            .trim_start_matches('-')
            .split(['T', '-', ':'].as_ref())
            .collect();
        let year: i32 = -split[0].parse::<i32>().unwrap();
        let month: u32 = split[1].parse().unwrap();
        let day: u32 = split[2].parse().unwrap();
        let hour: u32 = split[3].parse().unwrap();
        let min: u32 = split[4].parse().unwrap();
        let sec: u32 = split[5].trim_end_matches('Z').parse().unwrap();

        let naive = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap();

        Utc.from_utc_datetime(&naive)
    } else {
        DateTime::parse_from_rfc3339(iso)
            .unwrap()
            .with_timezone(&Utc)
    }
}

#[test]
fn test_julian_dates() {
    let cases = [
        ("2024-09-23T19:12:00Z", 2460577.3),
        ("2020-01-01T12:00:00Z", 2458850.0),
        ("1987-01-27T00:00:00Z", 2446822.5),
        ("1987-06-19T12:00:00Z", 2446966.0),
        ("1988-02-29T00:00:00Z", 2447220.5),
        ("1988-01-27T00:00:00Z", 2447187.5),
        ("1988-06-19T12:00:00Z", 2447332.0),
        ("1900-01-01T00:00:00Z", 2415020.5),
        ("1600-01-01T00:00:00Z", 2305447.5),
        ("0837-04-10T15:45:00Z", 2026872.15625),
        ("-1000-07-12T12:00:00Z", 1356001.0),
        ("-1000-03-01T00:00:00Z", 1355867.5),
        ("-1001-08-17T21:36:00Z", 1355671.4),
        ("-4712-01-01T12:00:00Z", 0.0),
    ];

    for (iso, expected) in cases {
        let dt = parse_iso_or_bce(iso);
        let actual = julian_date(dt);
        assert!(
            (actual - expected).abs() < EPSILON,
            "FAIL: {} → got {}, expected {}",
            iso,
            actual,
            expected
        );
    }
}

#[test]
fn test_j2000_days() {
    let cases = [
        ("2024-09-23T00:00:00Z", 9031.5),
        ("2020-01-01T12:00:00Z", 7305.0),
        ("1987-01-27T00:00:00Z", -4722.5),
        ("1987-06-19T12:00:00Z", -4579.0),
        ("1988-02-29T00:00:00Z", -4324.5),
        ("1988-01-27T00:00:00Z", -4357.5),
        ("1988-06-19T12:00:00Z", -4213.0),
        ("1900-01-01T00:00:00Z", -36524.5),
        ("1600-01-01T00:00:00Z", -146097.5),
        ("0837-04-10T15:45:00Z", -424672.84375),
        ("-1000-07-12T12:00:00Z", -1095544.0),
        ("-1000-03-01T00:00:00Z", -1095677.5),
        ("-1001-08-17T21:36:00Z", -1095873.6),
        ("-4712-01-01T12:00:00Z", -2451545.0),
    ];

    for (iso, expected) in cases {
        let dt = parse_iso_or_bce(iso);
        let actual = j2000_days(dt);
        assert!(
            (actual - expected).abs() < EPSILON,
            "FAIL: {} → got {}, expected {}",
            iso,
            actual,
            expected
        );
    }
}

#[test]
fn test_julian_date_gregorian_transition() {
    // Test dates around the Gregorian calendar transition
    // October 4, 1582 (Julian) was followed by October 15, 1582 (Gregorian)
    
    // Last day of Julian calendar
    let dt_julian = Utc.with_ymd_and_hms(1582, 10, 4, 12, 0, 0).unwrap();
    let jd_julian = julian_date(dt_julian);
    assert!((jd_julian - 2299160.0).abs() < EPSILON, 
        "Oct 4, 1582 noon should be JD 2299160.0, got {}", jd_julian);
    
    // First day of Gregorian calendar
    let dt_gregorian = Utc.with_ymd_and_hms(1582, 10, 15, 12, 0, 0).unwrap();
    let jd_gregorian = julian_date(dt_gregorian);
    assert!((jd_gregorian - 2299161.0).abs() < EPSILON,
        "Oct 15, 1582 noon should be JD 2299161.0, got {}", jd_gregorian);
    
    // Day before transition (Julian calendar rules apply)
    let dt_before = Utc.with_ymd_and_hms(1582, 10, 3, 12, 0, 0).unwrap();
    let jd_before = julian_date(dt_before);
    assert!((jd_before - 2299159.0).abs() < EPSILON,
        "Oct 3, 1582 noon should be JD 2299159.0, got {}", jd_before);
    
    // Day after transition (Gregorian calendar rules apply)
    let dt_after = Utc.with_ymd_and_hms(1582, 10, 16, 12, 0, 0).unwrap();
    let jd_after = julian_date(dt_after);
    assert!((jd_after - 2299162.0).abs() < EPSILON,
        "Oct 16, 1582 noon should be JD 2299162.0, got {}", jd_after);
}
