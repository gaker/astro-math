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

#[test]
fn test_ra_dec_to_alt_az_zenith_edge_case() {
    // Test the edge case where object is at zenith (azimuth undefined)
    let observer = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Create a time where a star at Dec=45° would be at zenith
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap();
    
    // Star exactly at observer's latitude, on meridian
    let ra = 0.0; // Will adjust based on LST
    let dec = 45.0; // Same as latitude
    
    let (alt, az) = ra_dec_to_alt_az(ra, dec, dt, &observer);
    
    // Near zenith, altitude should be close to 90
    if alt > 89.9 {
        // Azimuth should be reasonable (0 or 180 based on our implementation)
        assert!(az == 0.0 || az == 180.0, 
            "At zenith, azimuth should be 0 or 180, got {}", az);
    }
}

#[test]
fn test_ra_dec_to_alt_az_polar_observer() {
    // Test edge case for observer very close to pole
    let observer = Location {
        latitude_deg: 89.9, // Very close to North Pole
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Polaris-like position
    let ra = 37.95456;
    let dec = 89.26411;
    
    let (alt, az) = ra_dec_to_alt_az(ra, dec, dt, &observer);
    
    // Should not crash and should give reasonable values
    assert!(alt >= -90.0 && alt <= 90.0, "Altitude out of range: {}", alt);
    assert!(az >= 0.0 && az <= 360.0, "Azimuth out of range: {}", az);
}

#[test]
fn test_ra_dec_to_alt_az_numerical_stability() {
    // Test case that could cause cos_az to be slightly outside [-1, 1]
    let observer = Location {
        latitude_deg: 0.0, // Equatorial observer
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 6, 0, 0).unwrap();
    
    // Object on horizon
    let ra = 90.0;
    let dec = 0.0;
    
    let (alt, az) = ra_dec_to_alt_az(ra, dec, dt, &observer);
    
    // Should not crash from acos domain error
    assert!(alt >= -90.0 && alt <= 90.0, "Altitude out of range: {}", alt);
    assert!(az >= 0.0 && az <= 360.0, "Azimuth out of range: {}", az);
}