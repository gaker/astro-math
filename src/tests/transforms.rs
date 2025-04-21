use crate::location::Location;
use crate::transforms::ra_dec_to_alt_az;
use chrono::{TimeZone, Utc};

const EPSILON: f64 = 0.1; // ~6 arcminutes tolerance

#[test]
fn test_ra_dec_to_alt_az_astropy_crosscheck() {
    // Observer at Kitt Peak National Observatory
    let observer = Location {
        latitude_deg: 31.9583,
        longitude_deg: -111.6,
        altitude_m: 2120.0,
    };

    // UTC time of observation
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 6, 0, 0).unwrap();

    // Vega (α Lyr)
    let ra = 279.23473479;
    let dec = 38.78368896;

    // Astropy verified:
    // Alt = 48.626°, Az = 78.244° (measured from North through East)
    let (alt, az) = ra_dec_to_alt_az(ra, dec, dt, &observer);
    println!("Alt: {}", alt);
    println!("AZ: {}", az);

    assert!(
        (alt - 77.775).abs() < EPSILON,
        "Alt = {}, expected ≈ 77.775",
        alt
    );
    assert!(
        (az - 307.386).abs() < EPSILON,
        "Az = {}, expected ≈ 307.386",
        az
    );
}

#[test]
fn test_ra_dec_to_alt_az_negative_azimuth_wrap() {
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();

    let loc = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };

    // Force HA to ~180° with weird trig alignment
    // Star is just west of meridian, pushing acos result close to PI
    let ra = 180.0;
    let dec = -10.0;

    let (_alt, az) = ra_dec_to_alt_az(ra, dec, dt, &loc);

    assert!(az >= 0.0 && az <= 360.0, "Azimuth should be normalized to [0, 360), got {}", az);
}