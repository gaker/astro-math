use crate::moon::*;
use chrono::{TimeZone, Utc};

#[test]
fn test_moon_position_range() {
    // Test over several dates to ensure values are in expected ranges
    let dates = [
        Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 4, 15, 12, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 7, 30, 18, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 11, 11, 6, 0, 0).unwrap(),
    ];
    
    for dt in dates {
        let (lon, lat) = moon_position(dt);
        assert!(lon >= 0.0 && lon < 360.0, "Longitude out of range: {}", lon);
        assert!(lat >= -6.0 && lat <= 6.0, "Latitude out of range: {}", lat);
    }
}

#[test]
fn test_moon_phases_cycle() {
    // Test that phase angle increases monotonically over short periods
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let mut prev_phase = moon_phase_angle(start);
    
    for hours in 1..24 {
        let dt = start + chrono::Duration::hours(hours);
        let phase = moon_phase_angle(dt);
        
        // Phase should generally increase (with small possible jumps near 360/0)
        if (phase - prev_phase).abs() < 300.0 {  // Not crossing 0/360 boundary
            assert!(phase > prev_phase, "Phase not increasing: {} -> {}", prev_phase, phase);
        }
        prev_phase = phase;
    }
}

#[test]
fn test_moon_distance_extremes() {
    // Test multiple dates to find reasonable extremes
    let mut min_dist = f64::MAX;
    let mut max_dist = f64::MIN;
    
    // Check every 12 hours for a month
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    for half_days in 0..60 {
        let dt = start + chrono::Duration::hours(half_days * 12);
        let dist = moon_distance(dt);
        min_dist = min_dist.min(dist);
        max_dist = max_dist.max(dist);
    }
    
    // Perigee ~356,500 km, Apogee ~406,700 km
    assert!(min_dist > 350000.0 && min_dist < 365000.0);
    assert!(max_dist > 395000.0 && max_dist < 410000.0);
}

#[test]
fn test_phase_illumination_consistency() {
    // Test that phase angle and illumination are consistent
    let dates = [
        (0.0, 0.0),    // New moon
        (90.0, 50.0),  // First quarter
        (180.0, 100.0), // Full moon
        (270.0, 50.0),  // Last quarter
    ];
    
    for (expected_phase, expected_illum) in dates {
        // Create a date with approximately the right phase
        let base = Utc.with_ymd_and_hms(2024, 1, 11, 12, 0, 0).unwrap(); // Near new moon
        let offset_days = (expected_phase / 360.0 * 29.53) as i64; // Lunar month
        let dt = base + chrono::Duration::days(offset_days);
        
        let illum = moon_illumination(dt);
        
        // Check illumination is in reasonable range
        // (won't be exact due to simplified calculation)
        if expected_illum == 0.0 {
            assert!(illum < 10.0);
        } else if expected_illum == 100.0 {
            assert!(illum > 90.0);
        } else {
            assert!(illum > 30.0 && illum < 70.0);
        }
    }
}

#[test]
fn test_moon_edge_cases() {
    // Test moon phase angle wraparound
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let phase = moon_phase_angle(dt);
    assert!(phase >= 0.0 && phase < 360.0);
}

#[test]
fn test_moon_distance_formula() {
    // Additional moon distance test
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let dist = moon_distance(dt);
    
    // Distance should be reasonable
    assert!(dist > 350000.0); // Minimum possible distance
    assert!(dist < 410000.0); // Maximum possible distance
}

#[test]
fn test_moon_negative_normalizations() {
    // Test negative longitude normalization
    let dt = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();
    let (lon, _) = moon_position(dt);
    assert!(lon >= 0.0 && lon < 360.0);
    
    // Test negative phase normalization
    let phase = moon_phase_angle(dt);
    assert!(phase >= 0.0 && phase < 360.0);
    
    // Test negative RA normalization
    let (ra, _) = moon_equatorial(dt);
    assert!(ra >= 0.0 && ra < 360.0);
}

#[test]
fn test_moon_phase_edge_cases() {
    // Test all phase name branches
    let base_dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // Since we can't easily control the exact phase, at least ensure
    // the function works for various dates throughout a month
    for i in 0..30 {
        let dt = base_dt + chrono::Duration::days(i);
        let name = moon_phase_name(dt);
        assert!(["New Moon", "Waxing Crescent", "First Quarter", "Waxing Gibbous",
                 "Full Moon", "Waning Gibbous", "Last Quarter", "Waning Crescent"]
                .contains(&name));
    }
}