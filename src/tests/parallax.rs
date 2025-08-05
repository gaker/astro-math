use crate::parallax::*;
use crate::Location;
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
    
    let (ra_topo, _) = diurnal_parallax(ra_horizon, 0.0, 0.00257, dt, &location);
    
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
    
    let (ra1, dec1) = annual_parallax(ra, dec, parallax, dt1);
    let (ra2, dec2) = annual_parallax(ra, dec, parallax, dt2);
    
    // Should see variation between the two dates
    assert!((ra1 - ra2).abs() > 0.0 || (dec1 - dec2).abs() > 0.0);
    // The variation should be measurable
    let total_variation = ((ra1 - ra2).powi(2) + (dec1 - dec2).powi(2)).sqrt();
    assert!(total_variation > 0.00001);
}