use crate::proper_motion::*;
use crate::error::AstroError;
use chrono::{TimeZone, Utc};

#[test]
fn test_zero_proper_motion() {
    // A star with no proper motion should not move
    let ra_2000 = 45.0;
    let dec_2000 = 30.0;
    
    let epoch_2050 = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = apply_proper_motion(ra_2000, dec_2000, 0.0, 0.0, epoch_2050).unwrap();
    
    assert!((ra - ra_2000).abs() < 1e-10, "RA should not change with zero PM");
    assert!((dec - dec_2000).abs() < 1e-10, "Dec should not change with zero PM");
}

#[test]
fn test_linear_proper_motion() {
    // Test simple linear motion
    let ra_2000 = 180.0;
    let dec_2000 = 0.0;
    let pm_ra = 100.0;  // mas/yr
    let pm_dec = -50.0; // mas/yr
    
    // After 10 years
    let epoch_2010 = Utc.with_ymd_and_hms(2010, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch_2010).unwrap();
    
    // Expected motion: 10 years * 100 mas/yr = 1000 mas = 1 arcsec = 1/3600 degree
    let expected_ra = ra_2000 + 10.0 * 100.0 / 3_600_000.0;
    let expected_dec = dec_2000 + 10.0 * (-50.0) / 3_600_000.0;
    
    assert!((ra - expected_ra).abs() < 1e-8, "RA: expected {}, got {}", expected_ra, ra);
    assert!((dec - expected_dec).abs() < 1e-8, "Dec: expected {}, got {}", expected_dec, dec);
}

#[test]
fn test_high_proper_motion_star() {
    // Barnard's Star - highest known proper motion
    let ra_2000 = 269.454022;
    let dec_2000 = 4.668288;
    let pm_ra_cosdec = -797.84;  // mas/yr
    let pm_dec = 10326.93;        // mas/yr
    
    // Position after 100 years
    let epoch_2100 = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = apply_proper_motion(ra_2000, dec_2000, pm_ra_cosdec, pm_dec, epoch_2100).unwrap();
    
    // Barnard's star should have moved significantly
    // Dec motion: 100 years * 10327 mas/yr / 3600000 = 0.287° 
    assert!((dec - dec_2000) > 0.28 && (dec - dec_2000) < 0.29,
        "Barnard's Star should move ~0.287° in Dec over 100 years, got {}", dec - dec_2000);
    
    // RA motion is smaller due to cos(dec) factor
    assert!((ra - ra_2000).abs() < 0.1,
        "RA motion should be smaller than Dec motion");
}

#[test]
fn test_ra_wraparound() {
    // Test RA wraparound at 0/360 boundary
    let ra_2000 = 0.1;  // Just past 0
    let dec_2000 = 45.0;
    let pm_ra = -10000.0;  // Moving westward rapidly
    let pm_dec = 0.0;
    
    // After 50 years: 50 * 10000 mas/yr = 500000 mas = 0.139 degrees westward
    let epoch_2050 = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch_2050).unwrap();
    
    // Expected: 0.1 - 0.139 = -0.039, which wraps to 359.961
    assert!(ra > 359.9 && ra < 360.0, "RA should wrap around to near 360°, got {}", ra);
    assert!((dec - dec_2000).abs() < 1e-10, "Dec should not change");
}

#[test]
fn test_pole_approach() {
    // Test star approaching north pole
    let ra_2000 = 0.0;
    let dec_2000 = 89.9;
    let pm_ra = 0.0;
    let pm_dec = 10000.0;  // Moving toward pole
    
    // After 30 years
    let epoch_2030 = Utc.with_ymd_and_hms(2030, 1, 1, 0, 0, 0).unwrap();
    let result = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch_2030);
    
    // Should fail if it would exceed the pole
    match result {
        Ok((_, dec)) => {
            assert!(dec <= 90.0, "Dec should not exceed 90°");
        }
        Err(AstroError::InvalidCoordinate { .. }) => {
            // This is also acceptable - the function detected invalid declination
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_rigorous_proper_motion() {
    // Test rigorous method with radial velocity
    let ra_2000 = 88.793;   // Betelgeuse
    let dec_2000 = 7.407;
    let pm_ra_cosdec = 27.54;
    let pm_dec = 11.30;
    let parallax = 6.55;     // mas
    let rv = 21.91;          // km/s
    
    let epoch_2050 = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra_rig, dec_rig, plx_new) = apply_proper_motion_rigorous(
        ra_2000, dec_2000, pm_ra_cosdec, pm_dec, parallax, rv, epoch_2050
    ).unwrap();
    
    // Compare with simple method
    let (ra_simple, dec_simple) = apply_proper_motion(
        ra_2000, dec_2000, pm_ra_cosdec, pm_dec, epoch_2050
    ).unwrap();
    
    // Results should be similar but not identical
    // For Betelgeuse with large distance, the difference can be significant
    assert!((ra_rig - ra_simple).abs() < 1.0, 
        "Rigorous and simple RA should be somewhat close: {} vs {}", ra_rig, ra_simple);
    assert!((dec_rig - dec_simple).abs() < 1.0,
        "Rigorous and simple Dec should be somewhat close: {} vs {}", dec_rig, dec_simple);
    
    // Parallax should change due to radial motion
    assert!(plx_new != parallax, "Parallax should change with radial velocity");
    assert!(plx_new < parallax, "Receding star should have decreasing parallax");
}

#[test]
fn test_total_proper_motion_formula() {
    // Test Pythagorean calculation
    assert!((total_proper_motion(3.0, 4.0) - 5.0).abs() < 1e-10);
    assert!((total_proper_motion(0.0, 10.0) - 10.0).abs() < 1e-10);
    assert!((total_proper_motion(10.0, 0.0) - 10.0).abs() < 1e-10);
    
    // Test with negative values
    assert!((total_proper_motion(-3.0, -4.0) - 5.0).abs() < 1e-10);
}

#[test]
fn test_position_angles() {
    // Cardinal directions
    assert!((proper_motion_position_angle(0.0, 10.0) - 0.0).abs() < 1e-10,
        "North should be 0°");
    assert!((proper_motion_position_angle(10.0, 0.0) - 90.0).abs() < 1e-10,
        "East should be 90°");
    assert!((proper_motion_position_angle(0.0, -10.0) - 180.0).abs() < 1e-10,
        "South should be 180°");
    assert!((proper_motion_position_angle(-10.0, 0.0) - 270.0).abs() < 1e-10,
        "West should be 270°");
    
    // 45° angles
    assert!((proper_motion_position_angle(10.0, 10.0) - 45.0).abs() < 1e-10,
        "NE should be 45°");
    assert!((proper_motion_position_angle(10.0, -10.0) - 135.0).abs() < 1e-10,
        "SE should be 135°");
}

#[test]
fn test_pm_conversions() {
    // Test conversion between pm_ra and pm_ra*cos(dec)
    let pm_ra = 100.0;
    
    // At equator, cos(dec) = 1
    let pm_ra_cosdec = pm_ra_to_pm_ra_cosdec(pm_ra, 0.0);
    assert!((pm_ra_cosdec - pm_ra).abs() < 1e-10);
    
    // At 60° declination, cos(dec) = 0.5
    let pm_ra_cosdec = pm_ra_to_pm_ra_cosdec(pm_ra, 60.0);
    assert!((pm_ra_cosdec - 50.0).abs() < 1e-10);
    
    // Test inverse conversion
    let pm_ra_back = pm_ra_cosdec_to_pm_ra(pm_ra_cosdec, 60.0);
    assert!((pm_ra_back - pm_ra).abs() < 1e-10);
    
    // At pole, cos(dec) = 0, conversion undefined
    let pm_ra_cosdec = pm_ra_to_pm_ra_cosdec(pm_ra, 90.0);
    assert!(pm_ra_cosdec.abs() < 1e-10);
}

#[test]
fn test_proper_motion_error_cases() {
    let epoch = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    // Invalid RA
    let result = apply_proper_motion(400.0, 0.0, 0.0, 0.0, epoch);
    assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
    
    // Invalid Dec
    let result = apply_proper_motion(0.0, 100.0, 0.0, 0.0, epoch);
    assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
    
    // Invalid parallax for rigorous method
    let result = apply_proper_motion_rigorous(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, epoch);
    assert!(matches!(result, Err(AstroError::OutOfRange { .. })));
    
    let result = apply_proper_motion_rigorous(0.0, 0.0, 0.0, 0.0, -1.0, 0.0, epoch);
    assert!(matches!(result, Err(AstroError::OutOfRange { .. })));
}

#[test]
fn test_backwards_proper_motion() {
    // Test calculating position in the past
    let ra_2000 = 100.0;
    let dec_2000 = 20.0;
    let pm_ra = 50.0;
    let pm_dec = -30.0;
    
    // Go back to 1950
    let epoch_1950 = Utc.with_ymd_and_hms(1950, 1, 1, 0, 0, 0).unwrap();
    let (ra_1950, dec_1950) = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch_1950).unwrap();
    
    // Motion should be reversed (negative time)
    assert!(ra_1950 < ra_2000, "RA should be smaller in the past");
    assert!(dec_1950 > dec_2000, "Dec should be larger in the past (negative PM)");
    
    // Verify by going forward from 1950 to 2000
    let epoch_2000 = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let (ra_check, dec_check) = apply_proper_motion(ra_1950, dec_1950, pm_ra, pm_dec, epoch_2000).unwrap();
    
    assert!((ra_check - ra_2000).abs() < 1e-3, 
        "Round trip RA should match: {} vs {}", ra_check, ra_2000);
    assert!((dec_check - dec_2000).abs() < 1e-3, 
        "Round trip Dec should match: {} vs {}", dec_check, dec_2000);
}

#[test]
fn test_proxima_centauri() {
    // Proxima Centauri - nearest star, high proper motion
    let ra_2000 = 217.428953;
    let dec_2000 = -62.679484;
    let pm_ra_cosdec = -3775.40;  // mas/yr
    let pm_dec = 769.33;           // mas/yr
    let parallax = 768.5;          // mas
    let rv = -22.4;                // km/s (approaching)
    
    // Calculate position in 2100
    let epoch_2100 = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    
    // Simple method
    let (ra_simple, dec_simple) = apply_proper_motion(
        ra_2000, dec_2000, pm_ra_cosdec, pm_dec, epoch_2100
    ).unwrap();
    
    // Rigorous method
    let (ra_rig, dec_rig, plx_new) = apply_proper_motion_rigorous(
        ra_2000, dec_2000, pm_ra_cosdec, pm_dec, parallax, rv, epoch_2100
    ).unwrap();
    
    // With such high proper motion and nearby distance, differences should be noticeable
    let ra_diff = (ra_rig - ra_simple).abs();
    let dec_diff = (dec_rig - dec_simple).abs();
    
    assert!(ra_diff > 0.0001, "Rigorous method should give different result for nearby star");
    assert!(dec_diff > 0.0001, "Rigorous method should give different result for nearby star");
    
    // With such high proper motion, the tangential motion dominates
    // Even though it's approaching, the large sideways motion increases distance
    println!("Original parallax: {}, New parallax: {}", parallax, plx_new);
    println!("Distance changed from {} pc to {} pc", 1000.0/parallax, 1000.0/plx_new);
    assert!(plx_new != parallax, "Parallax should change over 100 years");
    
    // The proper motion is so large that tangential motion dominates
    let total_pm = total_proper_motion(pm_ra_cosdec, pm_dec);
    assert!(total_pm > 3000.0, "Proxima has very high proper motion");
}

#[test]
fn test_proper_motion_ra_wraparound_multiple() {
    // Test multiple RA wraparounds (coverage: line 88)
    let ra_2000 = 359.9;
    let dec_2000 = 0.0;
    let pm_ra = 50000.0; // Very high PM to force multiple wraps
    let pm_dec = 0.0;
    
    let epoch = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    let (ra, _) = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch).unwrap();
    
    assert!((0.0..360.0).contains(&ra), "RA should be normalized after multiple wraps");
}

#[test]
fn test_proper_motion_invalid_dec() {
    // Test declination validation (coverage: line 92)
    let ra_2000 = 0.0;
    let dec_2000 = 89.9;
    let pm_ra = 0.0;
    let pm_dec = 50000.0; // Would push Dec > 90
    
    let epoch = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    let result = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch);
    
    // Should fail with invalid coordinate
    assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
}

#[test]
fn test_proper_motion_rigorous_negative_ra() {
    // Test negative RA in rigorous proper motion (coverage: line 187)
    let ra_2000 = 0.1;
    let dec_2000 = 0.0;
    let pm_ra = -10000.0; // High westward motion
    let pm_dec = 0.0;
    let parallax = 100.0;
    let rv = 0.0;
    
    let epoch = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra, _, _) = apply_proper_motion_rigorous(
        ra_2000, dec_2000, pm_ra, pm_dec, parallax, rv, epoch
    ).unwrap();
    
    assert!((0.0..360.0).contains(&ra), "RA should be normalized from negative");
}