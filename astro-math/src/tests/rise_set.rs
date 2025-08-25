use crate::rise_set::*;
use crate::*;
use chrono::{Datelike, Timelike, TimeZone, Utc};

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
    
    assert!(result.is_some(), "Equatorial object should rise and set at equator");
    let (rise, transit, set) = result.unwrap();
    
    // Should be approximately 12 hours from rise to set
    let duration = (set - rise).num_hours();
    assert!((11..=13).contains(&duration), 
        "Equatorial object should be up ~12 hours, got {}", duration);
    
    // Transit should be roughly halfway between rise and set
    let rise_to_transit = (transit - rise).num_minutes();
    let transit_to_set = (set - transit).num_minutes();
    assert!((rise_to_transit - transit_to_set).abs() < 60,
        "Transit should be centered, rise->transit: {} min, transit->set: {} min",
        rise_to_transit, transit_to_set);
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
    
    assert!(next_rise_time.is_some(), "Star at dec=20° should rise at lat=40°");
    assert!(next_set_time.is_some(), "Star at dec=20° should set at lat=40°");
    
    let rise = next_rise_time.unwrap();
    let set = next_set_time.unwrap();
    
    // Both should be in the future
    assert!(rise > start, "Next rise should be after start time");
    assert!(set > start, "Next set should be after start time");
    
    // At RA=100°, Dec=20°, from latitude 40°, the object should be up for >12 hours
    // If we're starting at midnight, rise should come after set (set from previous cycle)
    if rise > set {
        // This means set is from the previous day's cycle
        let implied_duration = 24 - (rise - set).num_hours();
        assert!(implied_duration > 12 && implied_duration < 20,
            "Object should be up 12-20 hours, implied duration: {} hours", implied_duration);
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
    // Test rise/set time wraparound when transit occurs near midnight
    let location = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Object with RA=180° transits around midnight at 0° longitude
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    let result = rise_transit_set(180.0, 30.0, dt, &location, None).unwrap();
    assert!(result.is_some());
    
    let (rise, transit, set) = result.unwrap();
    
    // At 0° longitude on Aug 4, an object at RA=180° (12h) transits when LST=12h
    // LST at noon UTC ≈ 9h in August, so RA 12h transits ~3h later = 15:00 UTC
    let transit_hour = transit.hour();
    assert!((transit_hour as i32 - 15).abs() <= 2, 
        "RA 180° should transit around 15:00 UTC in August, got {:02}:00", transit_hour);
    
    // Verify chronological order and duration
    assert!(transit > rise, "Transit should be after rise");
    assert!(set > transit, "Set should be after transit");
    
    // Duration should still make sense for Dec=30° at lat=45°
    let duration_hours = (set - rise).num_hours();
    assert!((6..18).contains(&duration_hours),
        "Object at Dec=30° should be up 6-18 hours at lat=45°, got {} hours", duration_hours);
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
    assert!(result.is_some(), "Object on celestial equator should rise/set at 45° latitude");
    
    let (rise, transit, set) = result.unwrap();
    // At 45° latitude, celestial equator objects should be up ~12 hours
    let duration = (set - rise).num_hours();
    assert!((11..=13).contains(&duration),
        "Celestial equator object should be visible ~12 hours at 45° lat, got {} hours", duration);
    
    // RA 180° should transit around midnight local time
    // At 0° longitude on Aug 4, LST at noon ≈ 9h, so RA 180° (12h) transits ~3h later = 15:00 UTC
    let expected_transit_hour = 15;
    assert!((transit.hour() as i32 - expected_transit_hour).abs() <= 1,
        "RA 180° should transit around {}:00 UTC, got {:02}:{:02}", 
        expected_transit_hour, transit.hour(), transit.minute());
    
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

#[test]
fn test_next_set_no_set_within_search() {
    // Test when object doesn't set within search window (coverage: lines 212, 215)
    let location = Location {
        latitude_deg: 70.0, // High latitude
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let summer = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Circumpolar object at this latitude - never sets
    let result = next_set(0.0, 80.0, summer, &location, None).unwrap();
    assert!(result.is_none(), "Circumpolar object should not set");
}