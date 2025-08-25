use crate::sidereal::{apparent_sidereal_time, gmst, local_mean_sidereal_time};
use crate::time::julian_date;
use chrono::{DateTime, TimeZone, Utc};

const EPSILON: f64 = 1e-2; // ≈ 0.36 seconds

#[test]
fn test_gmst_known_value() {
    // 1987 Apr 10, 19h 21m 0s UT
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    let jd = julian_date(dt);

    let gmst_hours = gmst(jd);
    let expected = 8.582; // from Meeus example, p.88
    assert!(
        (gmst_hours - expected).abs() < EPSILON,
        "GMST = {}, expected = {}",
        gmst_hours,
        expected
    );
}

#[test]
fn test_local_mean_sidereal_time_known_value() {
    // Same date/time, but longitude = -64.0° (west)
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    let jd = julian_date(dt);

    let lst_hours = local_mean_sidereal_time(jd, -64.0);
    let expected = 4.317; // example from Meeus (p.88)
    assert!(
        (lst_hours - expected).abs() < EPSILON,
        "LST = {}, expected = {}",
        lst_hours,
        expected
    );
}

#[test]
fn test_local_mean_sidereal_time_wraps_positive() {
    // Create a datetime where GMST is near 0.0
    let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(); // JD = 2451545.0 → GMST ~ 18.697
    let jd = julian_date(dt);

    // Pick a longitude that shifts GMST below 0 (e.g. -285°)
    let lst_val = local_mean_sidereal_time(jd, -285.0);
    assert!(
        (0.0..24.0).contains(&lst_val),
        "LST should wrap to [0, 24), got {}",
        lst_val
    );
}

#[test]
fn test_apparent_sidereal_time_astropy_crosscheck() {
    let cases = [
        // Greenwich
        ("2024-08-04T06:00:00Z", 0.0, 2.8865),
        ("2000-01-01T12:00:00Z", 0.0, 18.69723),
        ("1987-04-10T19:21:00Z", 0.0, 8.584307),
        // Kitt Peak
        ("2024-08-04T06:00:00Z", -111.6, 19.4465),
        // Tokyo
        ("2024-08-04T06:00:00Z", 139.6917, 12.19935),
        ("2000-06-21T00:00:00Z", 139.6917, 3.27918),
    ];

    for (iso, lon, expected) in cases {
        let dt = DateTime::parse_from_rfc3339(iso).unwrap();
        let jd = julian_date(dt.to_utc());
        let actual = apparent_sidereal_time(jd, lon);

        assert!(
            (actual - expected).abs() < EPSILON,
            "FAIL: {} @ lon {}° → got {:.6}, expected {:.6}",
            iso,
            lon,
            actual,
            expected
        );
    }
}
