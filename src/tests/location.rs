use crate::location::{Location, ParseError};
use chrono::{TimeZone, Utc};

const EPSILON: f64 = 1e-3; // ~3.6s sidereal time

#[test]
fn test_parse_error_display() {
    use crate::location::ParseError;

    let err = ParseError::InvalidFormat;
    assert_eq!(err.to_string(), "Invalid DMS format");

    let err = ParseError::InvalidNumber;
    assert_eq!(err.to_string(), "Invalid number in DMS string");
}

#[test]
fn test_formatting_real_error_output() {
    let result = Location::from_dms("not valid", "still bad", 0.0);
    let err_str = format!("{}", result.unwrap_err());
    assert!(err_str.contains("Invalid"));
}

#[test]
fn test_local_sidereal_time_known_case() {
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    let loc = Location {
        latitude_deg: 32.0,   // doesn't affect LST, but needed for full Alt/Az later
        longitude_deg: -64.0, // Meeus example
        altitude_m: 200.0,
    };
    let lst = loc.local_sidereal_time(dt);

    assert!(
        (lst - 4.3157).abs() < EPSILON,
        "LST = {}, expected ≈ 4.3157",
        lst
    );
}

#[test]
fn test_mean_local_sidereal_time_known_case() {
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    let loc = Location {
        latitude_deg: 32.0,   // doesn't affect LST, but needed for full Alt/Az later
        longitude_deg: -64.0, // Meeus example
        altitude_m: 200.0,
    };
    let lst = loc.local_mean_sidereal_time(dt);

    assert!(
        (lst - 4.3157).abs() < EPSILON,
        "LST = {}, expected ≈ 4.3157",
        lst
    );
}

#[test]
fn test_latitude_dms_positive() {
    let loc = Location {
        latitude_deg: 38.889722,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let dms = loc.latitude_dms();
    assert_eq!(dms, "38° 53′ 22.999″");
}

#[test]
fn test_latitude_dms_negative() {
    let loc = Location {
        latitude_deg: -45.5123,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let dms = loc.latitude_dms();
    assert_eq!(dms, "-45° 30′ 44.280″");
}

#[test]
fn test_longitude_dms_padding_and_sign() {
    let loc = Location {
        latitude_deg: 0.0,
        longitude_deg: -122.4194,
        altitude_m: 0.0,
    };
    let dms = loc.longitude_dms();
    assert_eq!(dms, "-122° 25′ 09.840″");
}


#[test]
fn test_parse_invalid_from_dms() {
    let mut result = Location::from_dms("foo", "bar", 0.0);
    assert!(result.is_err());

    result = Location::from_dms("+xx 00 00", "-92 18 03.2", 0.0);
    assert!(matches!(result, Err(ParseError::InvalidNumber)));

    let cases = [
        ["39 0a 01.7", "-92 18 03.a"],
        ["39:a0:01.7", "-a2:18:a3.2"],
        ["39°0b'01.7\"", "-92°1c'03.2\""],
    ];

    for case in cases {
        let result = Location::from_dms(case[0], case[1], 0.0);
        assert!(result.is_err());
    }
}

#[test]
fn test_parse_valid_dms_strings() {
    let cases = [
        ["39 00 01.7", "-92 18 03.2"],
        ["39:00:01.7", "-92:18:03.2"],
        ["39°00'01.7\"", "-92°18'03.2\""],
    ];

    for case in cases {
        let result = Location::from_dms(case[0], case[1], 0.0);
        assert!(!result.is_err());
        let loc = result.unwrap();
        assert!((loc.latitude_deg - 39.0004722).abs() < 1e-6);

        let lat = loc.latitude_dms_string();
        assert_eq!(lat, "39° 00′ 01.700″");

        let long = loc.longitude_dms_string();
        assert_eq!(long, "-092° 18′ 03.200″");
    }
}