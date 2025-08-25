//! Tests for error handling paths to improve code coverage

use crate::*;
use crate::error::{AstroError, validate_range, validate_latitude, validate_longitude};
use chrono::{Datelike, TimeZone, Utc};

#[test]
fn test_airmass_error_paths() {
    // Test out of range altitudes for all airmass functions
    let invalid_altitudes = [-100.0, 100.0, -91.0, 91.0];
    
    for alt in invalid_altitudes {
        // Plane parallel
        match airmass_plane_parallel(alt) {
            Err(AstroError::OutOfRange { parameter, value, min, max }) => {
                assert_eq!(parameter, "altitude");
                assert_eq!(value, alt);
                assert_eq!(min, -90.0);
                assert_eq!(max, 90.0);
            }
            _ => panic!("Expected OutOfRange error for altitude {}", alt),
        }
        
        // Young
        match airmass_young(alt) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
        
        // Pickering
        match airmass_pickering(alt) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
        
        // Kasten-Young
        match airmass_kasten_young(alt) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
    }
    
    // Test extinction coefficient estimate with invalid wavelength
    match extinction_coefficient_estimate(0.0) {
        Err(AstroError::OutOfRange { parameter, value, .. }) => {
            assert_eq!(parameter, "wavelength_nm");
            assert_eq!(value, 0.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match extinction_coefficient_estimate(-100.0) {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "wavelength_nm");
        }
        _ => panic!("Expected OutOfRange error"),
    }
}

#[test]
fn test_error_validation_functions() {
    // Test validate_range
    assert!(validate_range(5.0, 0.0, 10.0, "test").is_ok());
    assert!(validate_range(0.0, 0.0, 10.0, "test").is_ok());
    assert!(validate_range(10.0, 0.0, 10.0, "test").is_ok());
    
    match validate_range(-1.0, 0.0, 10.0, "test") {
        Err(AstroError::OutOfRange { parameter, value, min, max }) => {
            assert_eq!(parameter, "test");
            assert_eq!(value, -1.0);
            assert_eq!(min, 0.0);
            assert_eq!(max, 10.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match validate_range(11.0, 0.0, 10.0, "test") {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "test");
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    // Test validate_latitude
    assert!(validate_latitude(0.0).is_ok());
    assert!(validate_latitude(90.0).is_ok());
    assert!(validate_latitude(-90.0).is_ok());
    
    match validate_latitude(91.0) {
        Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
            assert_eq!(coord_type, "Latitude");
            assert_eq!(value, 91.0);
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
    
    match validate_latitude(-91.0) {
        Err(AstroError::InvalidCoordinate { coord_type, .. }) => {
            assert_eq!(coord_type, "Latitude");
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
    
    // Test validate_longitude
    assert!(validate_longitude(0.0).is_ok());
    assert!(validate_longitude(180.0).is_ok());
    assert!(validate_longitude(-180.0).is_ok());
    
    match validate_longitude(181.0) {
        Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
            assert_eq!(coord_type, "Longitude");
            assert_eq!(value, 181.0);
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
    
    match validate_longitude(-181.0) {
        Err(AstroError::InvalidCoordinate { coord_type, .. }) => {
            assert_eq!(coord_type, "Longitude");
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
}

#[test]
fn test_galactic_error_paths() {
    // Test invalid galactic latitude in galactic_to_equatorial
    match galactic_to_equatorial(0.0, 91.0) {
        Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
            assert_eq!(coord_type, "Galactic latitude");
            assert_eq!(value, 91.0);
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
    
    match galactic_to_equatorial(0.0, -91.0) {
        Err(AstroError::InvalidCoordinate { coord_type, .. }) => {
            assert_eq!(coord_type, "Galactic latitude");
        }
        _ => panic!("Expected InvalidCoordinate error"),
    }
}

#[test]
fn test_parallax_error_paths() {
    let location = Location { latitude_deg: 40.0, longitude_deg: -74.0, altitude_m: 0.0 };
    let dt = Utc::now();
    
    // Test invalid distance in diurnal_parallax
    match diurnal_parallax(45.0, 20.0, 0.0, dt, &location) {
        Err(AstroError::OutOfRange { parameter, value, .. }) => {
            assert_eq!(parameter, "distance_au");
            assert_eq!(value, 0.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match diurnal_parallax(45.0, 20.0, -1.0, dt, &location) {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "distance_au");
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    // Test invalid parallax in annual_parallax
    match annual_parallax(180.0, 0.0, 0.0, dt) {
        Err(AstroError::OutOfRange { parameter, value, .. }) => {
            assert_eq!(parameter, "parallax_mas");
            assert_eq!(value, 0.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match annual_parallax(180.0, 0.0, -10.0, dt) {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "parallax_mas");
        }
        _ => panic!("Expected OutOfRange error"),
    }
}

#[test]
fn test_refraction_error_paths() {
    // Test out of range altitudes for refraction functions
    let invalid_altitudes = [-100.0, 100.0];
    
    for alt in invalid_altitudes {
        // Bennett
        match refraction_bennett(alt) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
        
        // Saemundsson
        match refraction_saemundsson(alt, 1013.25, 10.0) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
        
        // Radio refraction
        match refraction_radio(alt, 1013.25, 10.0, 50.0) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
        
        // True to apparent altitude
        match true_to_apparent_altitude(alt, 1013.25, 10.0) {
            Err(AstroError::OutOfRange { parameter, .. }) => {
                assert_eq!(parameter, "altitude");
            }
            _ => panic!("Expected OutOfRange error"),
        }
    }
    
    // Test invalid humidity for radio refraction
    match refraction_radio(45.0, 1013.25, 10.0, -1.0) {
        Err(AstroError::OutOfRange { parameter, value, .. }) => {
            assert_eq!(parameter, "humidity_percent");
            assert_eq!(value, -1.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match refraction_radio(45.0, 1013.25, 10.0, 101.0) {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "humidity_percent");
        }
        _ => panic!("Expected OutOfRange error"),
    }
}

#[test]
fn test_projection_error_paths() {
    // Test invalid scale in TangentPlane::new
    match TangentPlane::new(180.0, 0.0, 0.0) {
        Err(AstroError::OutOfRange { parameter, value, .. }) => {
            assert_eq!(parameter, "scale");
            assert_eq!(value, 0.0);
        }
        _ => panic!("Expected OutOfRange error"),
    }
    
    match TangentPlane::new(180.0, 0.0, -1.0) {
        Err(AstroError::OutOfRange { parameter, .. }) => {
            assert_eq!(parameter, "scale");
        }
        _ => panic!("Expected OutOfRange error"),
    }
}

#[test]
fn test_location_error_paths() {
    // Test formatting for edge cases
    let loc1 = Location { latitude_deg: -0.0001, longitude_deg: -0.0001, altitude_m: 0.0 };
    let lat_str = loc1.latitude_dms_string();
    let lon_str = loc1.longitude_dms_string();
    // These should handle the negative zero case
    assert!(lat_str.starts_with('-'));
    assert!(lon_str.starts_with('-'));
}

#[test]
fn test_rise_set_error_edge_cases() {
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    
    // Test object that should trigger the no-rise condition in next_rise
    // Use a southern circumpolar star from a northern location
    let north_location = Location { latitude_deg: 45.0, longitude_deg: 0.0, altitude_m: 0.0 };
    let result = next_rise(0.0, -89.0, dt, &north_location, None).unwrap();
    assert!(result.is_none());
    
    // Test sun_rise_set with edge case at extreme polar latitudes
    let polar_location = Location { latitude_deg: 89.9, longitude_deg: 0.0, altitude_m: 0.0 };
    let winter = Utc.with_ymd_and_hms(2024, 12, 21, 12, 0, 0).unwrap();
    let result = sun_rise_set(winter, &polar_location).unwrap();
    
    // At extreme polar latitudes during winter, sun may never rise
    match result {
        None => {
            // Sun never rises (polar night) - this is expected at 89.9° in winter
        },
        Some((sunrise, sunset)) => {
            // If sun does rise/set, times should be valid
            assert!(sunrise.year() == 2024);
            assert!(sunset.year() == 2024);
            assert!(sunset > sunrise);
        }
    }
}

#[test]
fn test_transforms_error_edge_cases() {
    let location = Location { latitude_deg: 90.0, longitude_deg: 0.0, altitude_m: 0.0 };
    let dt = Utc::now();
    
    // Test polar observer edge case where denominator is very small
    // This should trigger the edge case handling in ra_dec_to_alt_az
    let (alt, az) = ra_dec_to_alt_az(0.0, 89.9999, dt, &location).unwrap();
    assert!(alt.is_finite());
    assert!(az.is_finite());
}

#[test]
fn test_precession_edge_case() {
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    
    // Test RA near 360 that might wrap to negative in inverse precession
    let (ra, dec) = precess_to_j2000(359.9, 0.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra));
    assert!((-90.0..=90.0).contains(&dec));
}