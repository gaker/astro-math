use crate::precession::*;
use chrono::{TimeZone, Utc};

#[test]
fn test_precession_j2000_epoch() {
    // At J2000.0 epoch, ERFA includes frame bias (ICRS to J2000 mean equator/equinox)
    // This is a small correction of ~0.014-0.016 arcseconds
    let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let (ra, dec) = precess_from_j2000(100.0, 25.0, dt).unwrap();
    
    // Frame bias causes tiny shifts (~0.000002° in RA, ~0.000001° in Dec)
    assert!((ra - 100.0).abs() < 0.00001);
    assert!((dec - 25.0).abs() < 0.00001);
}

#[test]
fn test_precession_century() {
    // Test precession over 100 years
    let dt = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = precess_from_j2000(0.0, 0.0, dt).unwrap();
    
    // Should show significant precession
    // Over 100 years, precession is ~1.4 degrees (50.3"/year * 100)
    assert!(ra > 0.5 && ra < 3.0, "RA precession over 100 years should be 1-2°, got {}°", ra);
    assert!((dec - 0.0).abs() < 1.0, "Dec at equator should change minimally, got {}°", dec);
}

#[test]
fn test_precession_pole() {
    // Test precession near the pole
    let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = precess_from_j2000(0.0, 89.9, dt).unwrap();
    
    // Near pole, RA changes should be large due to convergence of meridians
    // 50 years of precession near pole can cause large RA shifts
    assert!(ra > 5.0 || ra == 0.0, 
        "Near pole, RA should change significantly over 50 years or be undefined, got {}°", ra);
    // Dec should remain very close to pole
    assert!((dec - 89.9).abs() < 0.5, 
        "Declination near pole should be stable, changed from 89.9° to {}°", dec);
}

#[test]
fn test_precession_normalization() {
    // Test RA normalization in precession
    let dt = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    
    // Test with RA that might go negative after inverse precession
    let (ra1, dec1) = precess_to_j2000(5.0, 60.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra1), "RA should be normalized to [0,360), got {}", ra1);
    assert!((-90.0..=90.0).contains(&dec1), "Dec should remain valid, got {}", dec1);
    // Inverse precession should reduce RA
    assert!(!(5.0..=350.0).contains(&ra1), "Inverse precession from 2100 should reduce RA, got {}", ra1);
    
    // Test with RA that might exceed 360 after forward precession
    let (ra2, dec2) = precess_from_j2000(355.0, -60.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra2), "RA should be normalized to [0,360), got {}", ra2);
    assert!((-90.0..=90.0).contains(&dec2), "Dec should remain valid, got {}", dec2);
}

#[test]
fn test_precession_ra_else_branches() {
    // Test the else branches in precession RA normalization
    let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    // Test with RA that doesn't need normalization
    let (ra1, _) = precess_to_j2000(180.0, 0.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra1));
    
    // Test j2000_to_date with normal RA
    let (ra2, _) = precess_from_j2000(180.0, 0.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra2));
}

#[test]
fn test_precession_ra_normalization_edge_cases() {
    let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    // Test RA near 360 boundary
    let (ra_norm, _) = precess_to_j2000(359.99, 0.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra_norm));
}

#[test]
fn test_precession_ra_wraparound_360() {
    // Test RA wraparound when result >= 360 (coverage: line 181)
    let dt = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap(); // Going back in time
    
    // Test case that results in RA >= 360 after inverse precession
    let (ra, _) = precess_to_j2000(0.1, 89.0, dt).unwrap();
    assert!((0.0..360.0).contains(&ra), "RA should be normalized when >= 360");
}