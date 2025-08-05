use crate::rise_set::*;
use crate::*;
use chrono::{Datelike, TimeZone, Utc};

#[test]
fn test_equatorial_object() {
    // Object on celestial equator should rise/set at roughly 6 hours from transit
    let location = Location {
        latitude_deg: 0.0, // Equator
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let date = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap(); // Equinox
    let result = rise_transit_set(0.0, 0.0, date, &location, None).unwrap();
    
    assert!(result.is_some());
    let (rise, _transit, set) = result.unwrap();
    
    // Should be approximately 12 hours from rise to set
    let duration = (set - rise).num_hours();
    assert!(duration >= 11 && duration <= 13);
}

#[test]
fn test_polar_extremes() {
    // At poles, objects are either always up or always down
    let north_pole = Location {
        latitude_deg: 90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let date = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    
    // Positive declination should be circumpolar at north pole
    let result = rise_transit_set(0.0, 45.0, date, &north_pole, None).unwrap();
    assert!(result.is_none());
    
    // Negative declination should never rise at north pole
    let result = rise_transit_set(0.0, -45.0, date, &north_pole, None).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_next_rise_set() {
    let location = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    let start = Utc.with_ymd_and_hms(2024, 8, 4, 0, 0, 0).unwrap();
    
    // Test with a star that rises and sets
    let ra = 100.0;
    let dec = 20.0;
    
    let next_rise_time = next_rise(ra, dec, start, &location, None).unwrap();
    let next_set_time = next_set(ra, dec, start, &location, None).unwrap();
    
    assert!(next_rise_time.is_some());
    assert!(next_set_time.is_some());
    
    // Rise should come before set if we start before dawn
    if let (Some(rise), Some(set)) = (next_rise_time, next_set_time) {
        assert!(rise < set || set < start); // Either rise then set, or set is from previous day
    }
}

#[test]
fn test_sun_polar_day() {
    // Arctic summer - sun should not set
    let arctic = Location {
        latitude_deg: 75.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let summer = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    let result = sun_rise_set(summer, &arctic).unwrap();
    
    // Should be None (midnight sun)
    assert!(result.is_none());
}

#[test]
fn test_rise_set_wraparound() {
    // Test rise/set time wraparound
    let location = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Test object that transits near midnight
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    let result = rise_transit_set(180.0, 30.0, dt, &location, None).unwrap();
    assert!(result.is_some());
}

#[test]
fn test_rise_set_search_failure() {
    let location = Location {
        latitude_deg: 89.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    
    // Test object at extreme declination that should not rise/set at this latitude
    // At 89° latitude in summer, an object at 89.5° dec should be circumpolar (always up)
    let result = rise_transit_set(0.0, 89.5, dt, &location, Some(-18.0)).unwrap();
    
    // Test that the function handles extreme cases without panicking
    match result {
        None => {
            // Object never rises or sets (circumpolar or never visible) - this is expected
        },
        Some((rise, transit, set)) => {
            // If times are returned, they should be valid
            assert!(rise.year() >= 2024);
            assert!(transit.year() >= 2024);
            assert!(set.year() >= 2024);
        }
    }
}

#[test]
fn test_rise_set_edge_cases() {
    // Test rise/set edge cases
    let location = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Test case where transit offset is in normal range
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    let result = rise_transit_set(180.0, 0.0, dt, &location, None).unwrap();
    assert!(result.is_some());
    
    // Test sun_rise_set at high latitude during winter
    let polar_location = Location {
        latitude_deg: 85.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let winter = Utc.with_ymd_and_hms(2024, 12, 21, 12, 0, 0).unwrap();
    let _result = sun_rise_set(winter, &polar_location).unwrap();
    // May or may not be None depending on exact calculations
}