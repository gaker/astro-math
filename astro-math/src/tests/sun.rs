use crate::sun::*;
use chrono::{TimeZone, Utc};

#[test]
fn test_sun_position_basic() {
    // Test sun position at known dates
    
    // Vernal equinox (approximately)
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 0, 0, 0).unwrap();
    let (lon, lat) = sun_position(dt);
    assert!(lon >= 358.0 || lon <= 2.0, "Near vernal equinox, longitude should be ~0°, got {}", lon);
    assert!(lat.abs() < 0.1, "Sun latitude should be ~0°, got {}", lat);
    
    // Summer solstice (approximately)
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    let (lon, lat) = sun_position(dt);
    assert!((lon - 90.0).abs() < 2.0, "Near summer solstice, longitude should be ~90°, got {}", lon);
    assert!(lat.abs() < 0.1, "Sun latitude should be ~0°, got {}", lat);
    
    // Autumnal equinox (approximately)
    let dt = Utc.with_ymd_and_hms(2024, 9, 22, 0, 0, 0).unwrap();
    let (lon, lat) = sun_position(dt);
    assert!((lon - 180.0).abs() < 2.0, "Near autumnal equinox, longitude should be ~180°, got {}", lon);
    assert!(lat.abs() < 0.1, "Sun latitude should be ~0°, got {}", lat);
    
    // Winter solstice (approximately)
    let dt = Utc.with_ymd_and_hms(2024, 12, 21, 0, 0, 0).unwrap();
    let (lon, lat) = sun_position(dt);
    assert!((lon - 270.0).abs() < 2.0, "Near winter solstice, longitude should be ~270°, got {}", lon);
    assert!(lat.abs() < 0.1, "Sun latitude should be ~0°, got {}", lat);
}

#[test]
fn test_sun_ra_dec() {
    // Test equatorial coordinates conversion
    
    // Vernal equinox - sun crosses celestial equator
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 0, 0, 0).unwrap();
    let (ra, dec) = sun_ra_dec(dt);
    assert!(!(2.0..=358.0).contains(&ra), "Near vernal equinox, RA should be ~0°, got {}", ra);
    assert!(dec.abs() < 1.0, "Near vernal equinox, Dec should be ~0°, got {}", dec);
    
    // Summer solstice - sun at maximum northern declination
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    let (ra, dec) = sun_ra_dec(dt);
    assert!((ra - 90.0).abs() < 3.0, "Near summer solstice, RA should be ~90°, got {}", ra);
    assert!((dec - 23.4).abs() < 0.5, "Near summer solstice, Dec should be ~23.4°, got {}", dec);
    
    // Winter solstice - sun at maximum southern declination
    let dt = Utc.with_ymd_and_hms(2024, 12, 21, 0, 0, 0).unwrap();
    let (ra, dec) = sun_ra_dec(dt);
    assert!((ra - 270.0).abs() < 3.0, "Near winter solstice, RA should be ~270°, got {}", ra);
    assert!((dec + 23.4).abs() < 0.5, "Near winter solstice, Dec should be ~-23.4°, got {}", dec);
}

#[test]
fn test_sun_position_continuity() {
    // Test that sun position changes smoothly
    let dt1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let dt2 = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap();
    
    let (lon1, _) = sun_position(dt1);
    let (lon2, _) = sun_position(dt2);
    
    // Sun moves about 1 degree per day
    let daily_motion = (lon2 - lon1).abs();
    assert!(daily_motion > 0.9 && daily_motion < 1.1, 
        "Sun should move ~1° per day, got {}°", daily_motion);
}