use crate::parallax::*;
use crate::*;
use chrono::{TimeZone, Utc};

#[test]
fn test_geocentric_distance_poles() {
    // Test at north pole
    let loc_north = Location {
        latitude_deg: 90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let dist_north = geocentric_distance(&loc_north);
    // Poles are closer to Earth's center due to flattening
    assert!(dist_north < 0.997);
    
    // Test at south pole
    let loc_south = Location {
        latitude_deg: -90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let dist_south = geocentric_distance(&loc_south);
    assert!((dist_south - dist_north).abs() < 0.0001);
}

#[test]
fn test_moon_parallax_horizon() {
    // Moon near horizon should show maximum parallax
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 22, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Calculate for Moon near horizon
    let lst_hours = location.local_sidereal_time(dt);
    let ra_horizon = lst_hours * 15.0 - 90.0; // 90 degrees from meridian
    
    let (ra_topo, _) = diurnal_parallax(ra_horizon, 0.0, 0.00257, dt, &location).unwrap();
    
    // Parallax effect should be detectable
    let parallax = (ra_topo - ra_horizon).abs();
    assert!(parallax > 0.00001); // Should be detectable
}

#[test]
fn test_annual_parallax_maximum() {
    // Test when Earth is at maximum distance from star's direction
    let dt1 = Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(); // Near perihelion
    let dt2 = Utc.with_ymd_and_hms(2024, 7, 4, 0, 0, 0).unwrap(); // Near aphelion
    
    // Barnard's Star - high proper motion star
    let ra = 269.452;
    let dec = 4.693;
    let parallax = 546.0; // mas
    
    let (ra1, dec1) = annual_parallax(ra, dec, parallax, dt1).unwrap();
    let (ra2, dec2) = annual_parallax(ra, dec, parallax, dt2).unwrap();
    
    // Should see variation between the two dates
    assert!((ra1 - ra2).abs() > 0.0 || (dec1 - dec2).abs() > 0.0);
    // The variation should be measurable
    let total_variation = ((ra1 - ra2).powi(2) + (dec1 - dec2).powi(2)).sqrt();
    assert!(total_variation > 0.00001);
}

#[test]
fn test_parallax_normalization() {
    // Test RA normalization in parallax
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 22, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 89.0, // Near pole to trigger edge cases
        longitude_deg: -180.0,
        altitude_m: 0.0,
    };
    
    // Test with RA near 0/360 boundary
    let (ra1, _) = diurnal_parallax(359.9, 45.0, 0.1, dt, &location).unwrap();
    assert!(ra1 >= 0.0 && ra1 < 360.0);
    
    let (ra2, _) = diurnal_parallax(0.1, 45.0, 0.1, dt, &location).unwrap();
    assert!(ra2 >= 0.0 && ra2 < 360.0);
}

#[test]
fn test_parallax_ra_else_branch() {
    // Test the else branch in parallax RA normalization
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Test with RA that doesn't need normalization
    let (ra, _) = diurnal_parallax(180.0, 0.0, 1.0, dt, &location).unwrap();
    assert!(ra >= 0.0 && ra < 360.0);
}

#[test]
fn test_annual_parallax_normalization() {
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // Test RA values near boundaries that could trigger normalization
    let (ra_norm, _) = annual_parallax(359.99, 0.0, 0.001, dt).unwrap();
    assert!(ra_norm >= 0.0 && ra_norm < 360.0);
    
    let (ra_norm, _) = annual_parallax(0.01, 0.0, 0.001, dt).unwrap();
    assert!(ra_norm >= 0.0 && ra_norm < 360.0);
}

#[test]
fn test_annual_parallax_wraparound_branches() {
    // Test both RA wraparound branches in annual parallax (coverage: lines 155, 157)
    let dt = Utc.with_ymd_and_hms(2024, 3, 21, 0, 0, 0).unwrap(); // Spring equinox
    
    // Test case that results in negative RA needing wrap to positive
    let (ra, _) = annual_parallax(0.001, 0.0, 100.0, dt).unwrap();
    assert!(ra >= 0.0 && ra < 360.0, "RA should be normalized after negative correction");
    
    // Test case that results in RA > 360 needing wrap
    let (ra, _) = annual_parallax(359.999, 0.0, 100.0, dt).unwrap();
    assert!(ra >= 0.0 && ra < 360.0, "RA should be normalized after exceeding 360");
}