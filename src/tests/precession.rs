use crate::precession::*;
use chrono::{TimeZone, Utc};

#[test]
fn test_precession_j2000_epoch() {
    // At J2000.0 epoch, there should be no precession
    let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
    let (ra, dec) = precess_j2000_to_date(100.0, 25.0, dt).unwrap();
    
    assert!((ra - 100.0).abs() < 1e-6);
    assert!((dec - 25.0).abs() < 1e-6);
}

#[test]
fn test_precession_century() {
    // Test precession over 100 years
    let dt = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = precess_j2000_to_date(0.0, 0.0, dt).unwrap();
    
    // Should show significant precession
    assert!(ra > 1.0); // RA should increase
    assert!((dec - 0.0).abs() < 1.0); // Dec near equator changes due to precession
}

#[test]
fn test_precession_pole() {
    // Test precession near the pole
    let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    let (ra, dec) = precess_j2000_to_date(0.0, 89.9, dt).unwrap();
    
    // Near pole, RA changes should be large
    assert!(ra > 10.0);
    // Dec should remain very close to pole
    assert!((dec - 89.9).abs() < 0.1);
}

#[test]
fn test_precession_normalization() {
    // Test RA normalization in precession
    let dt = Utc.with_ymd_and_hms(2100, 1, 1, 0, 0, 0).unwrap();
    
    // Test with RA that would go negative
    let (ra1, _) = precess_date_to_j2000(5.0, 60.0, dt).unwrap();
    assert!(ra1 >= 0.0 && ra1 < 360.0);
    
    // Test with RA that would exceed 360
    let (ra2, _) = precess_date_to_j2000(355.0, -60.0, dt).unwrap();
    assert!(ra2 >= 0.0 && ra2 < 360.0);
}

#[test]
fn test_precession_ra_else_branches() {
    // Test the else branches in precession RA normalization
    let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    // Test with RA that doesn't need normalization
    let (ra1, _) = precess_date_to_j2000(180.0, 0.0, dt).unwrap();
    assert!(ra1 >= 0.0 && ra1 < 360.0);
    
    // Test j2000_to_date with normal RA
    let (ra2, _) = precess_j2000_to_date(180.0, 0.0, dt).unwrap();
    assert!(ra2 >= 0.0 && ra2 < 360.0);
}