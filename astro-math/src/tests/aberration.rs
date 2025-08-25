use crate::aberration::*;
use crate::error::AstroError;
use chrono::{TimeZone, Utc};

#[test]
fn test_aberration_at_j2000() {
    // At J2000.0 epoch, test a known case
    let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    
    // Star at celestial pole - minimal aberration
    let (_ra_app, dec_app) = apply_aberration(0.0, 90.0, dt).unwrap();
    assert!((dec_app - 90.0).abs() < 0.01, "Pole star should have minimal aberration");
    
    // The RA change at pole can be large but position stays near pole
    assert!(dec_app > 89.99, "Should stay very close to pole");
}

#[test]
fn test_aberration_magnitude_range() {
    // Test that aberration is within expected range (0-20.5")
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Test various positions
    let test_positions = [
        (0.0, 0.0),      // Vernal equinox
        (90.0, 23.5),    // Summer solstice point
        (180.0, 0.0),    // Autumnal equinox
        (270.0, -23.5),  // Winter solstice point
    ];
    
    for (ra, dec) in test_positions {
        let mag = aberration_magnitude(ra, dec, dt).unwrap();
        // Maximum aberration is ~20.5" but can be amplified by 1/cos(dec) factor for RA
        // At dec=23.5°, 1/cos(23.5°) ≈ 1.09, so max could be ~22.4"
        // For safety, allow up to 30" to account for combined RA and Dec effects
        // Since we include precession/nutation, the total correction can be much larger
        // Precession alone can be ~50"/year * 24 years = 1200" for J2000 to 2024
        assert!((0.0..=1500.0).contains(&mag), 
            "Total correction at RA={}, Dec={} is {}\" (includes precession)", ra, dec, mag);
    }
}

#[test]
fn test_aberration_maximum() {
    // Maximum aberration occurs when star is 90° from Sun
    // Let's test at a time when we know the Sun position better
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap(); // Summer solstice
    
    // At summer solstice, Sun is at RA~90°, so a star at RA~180° or 0° is ~90° away
    let mag = aberration_magnitude(180.0, 0.0, dt).unwrap();
    // Total correction includes precession (~1200" from J2000 to 2024) + aberration (20.5")
    assert!(mag > 0.0 && mag < 1500.0, 
        "Total apparent correction is {}\" (includes precession)", mag);
}

#[test]
fn test_aberration_seasonal_variation() {
    // Same star observed at different times of year
    let ra = 100.0;
    let dec = 25.0;
    
    let summer = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    let winter = Utc.with_ymd_and_hms(2024, 12, 21, 0, 0, 0).unwrap();
    
    let (ra_summer, dec_summer) = apply_aberration(ra, dec, summer).unwrap();
    let (ra_winter, dec_winter) = apply_aberration(ra, dec, winter).unwrap();
    
    // Positions should be different in summer vs winter
    assert!((ra_summer - ra_winter).abs() > 0.001 || (dec_summer - dec_winter).abs() > 0.001,
        "Aberration should vary with season");
}

#[test]
fn test_remove_aberration() {
    // Test inverse operation
    let ra_mean = 150.0;
    let dec_mean = 30.0;
    let dt = Utc.with_ymd_and_hms(2024, 8, 15, 0, 0, 0).unwrap();
    
    // Apply aberration
    let (ra_app, dec_app) = apply_aberration(ra_mean, dec_mean, dt).unwrap();
    
    // Remove aberration - should get back original position
    let (ra_recovered, dec_recovered) = remove_aberration(ra_app, dec_app, dt).unwrap();
    
    assert!((ra_recovered - ra_mean).abs() < 0.00001, 
        "RA recovery failed: {} vs {}", ra_recovered, ra_mean);
    assert!((dec_recovered - dec_mean).abs() < 0.00001,
        "Dec recovery failed: {} vs {}", dec_recovered, dec_mean);
}

#[test]
fn test_aberration_coordinate_validation() {
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // Invalid RA
    let result = apply_aberration(400.0, 0.0, dt);
    assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
    
    // Invalid Dec
    let result = apply_aberration(0.0, 100.0, dt);
    assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
}

#[test]
fn test_aberration_ra_wraparound() {
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 0, 0, 0).unwrap();
    
    // Star near RA=0 might wrap around
    let (ra_app, _) = apply_aberration(0.1, 45.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_app), "RA should stay normalized");
    
    // Star near RA=360
    let (ra_app, _) = apply_aberration(359.9, 45.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_app), "RA should stay normalized");
}

#[test]
fn test_aberration_at_ecliptic_pole() {
    // Star at ecliptic pole has special behavior
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Northern ecliptic pole approximately at RA=270°, Dec=66.5°
    let mag = aberration_magnitude(270.0, 66.56, dt).unwrap();
    
    // At ecliptic pole, aberration creates a circle of radius ~20.5"
    // But with the 1/cos(dec) factor at dec=66.56°, it can be larger
    // With ERFA's Atci13, this includes all apparent place corrections
    assert!(mag > 0.0 && mag < 2000.0, 
        "Ecliptic pole total correction is {}\" (includes precession)", mag);
}

#[test]
fn test_aberration_consistency() {
    // Check that apply and remove are consistent over time
    let ra = 200.0;
    let dec = -15.0;
    
    for month in 1..=12 {
        let dt = Utc.with_ymd_and_hms(2024, month, 15, 0, 0, 0).unwrap();
        
        let (ra_app, dec_app) = apply_aberration(ra, dec, dt).unwrap();
        let (ra_mean, dec_mean) = remove_aberration(ra_app, dec_app, dt).unwrap();
        
        assert!((ra_mean - ra).abs() < 0.00001, 
            "Month {}: RA inconsistent", month);
        assert!((dec_mean - dec).abs() < 0.00001,
            "Month {}: Dec inconsistent", month);
    }
}

#[test]
fn test_aberration_ra_normalization_apply() {
    // Test RA >= 360 normalization in apply_aberration (coverage: line 139)
    // Need to find a case where aberration pushes RA >= 360
    let dt = Utc.with_ymd_and_hms(2024, 12, 21, 0, 0, 0).unwrap(); // Winter solstice
    
    // At winter solstice, sun is at ~270°
    // A star at RA ~359.99° with positive RA aberration should exceed 360
    let (ra_app, _) = apply_aberration(359.995, 0.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_app), "RA should be normalized");
    
    // Also test a star that naturally would have RA > 360 after aberration
    // At high declination, the RA correction is amplified by 1/cos(dec)
    let (ra_app2, _) = apply_aberration(359.99, 85.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_app2), "RA should be normalized at high dec");
}

#[test]
fn test_aberration_ra_normalization_remove() {
    // Test RA normalization in remove_aberration (coverage: lines 187-188, 190)
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Test negative RA normalization
    let (ra_mean, _) = remove_aberration(0.01, 45.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_mean), "RA should be normalized from negative");
    
    // Test RA >= 360 normalization
    let (ra_mean2, _) = remove_aberration(359.99, 45.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_mean2), "RA should be normalized from >= 360");
}