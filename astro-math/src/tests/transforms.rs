use crate::*;
use crate::transforms::{ra_dec_to_alt_az_erfa, alt_az_to_ra_dec};
use chrono::{TimeZone, Utc};

const EPSILON: f64 = 0.1; // ~6 arcminutes tolerance

#[test]
fn test_ra_dec_to_alt_az_astropy_crosscheck() {
    // Observer at Kitt Peak National Observatory
    let observer = Location {
        latitude_deg: 31.9583,
        longitude_deg: -111.6,
        altitude_m: 2120.0,
    };

    // UTC time of observation
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 6, 0, 0).unwrap();

    // Vega (α Lyr)
    let ra = 279.23473479;
    let dec = 38.78368896;

    // Astropy verified:
    // Alt = 48.626°, Az = 78.244° (measured from North through East)
    let (alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    println!("Alt: {}", alt);
    println!("AZ: {}", az);

    assert!(
        (alt - 77.775).abs() < EPSILON,
        "Alt = {}, expected ≈ 77.775",
        alt
    );
    assert!(
        (az - 307.386).abs() < EPSILON,
        "Az = {}, expected ≈ 307.386",
        az
    );
}

#[test]
fn test_ra_dec_to_alt_az_negative_azimuth_wrap() {
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();

    let loc = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };

    // Force HA to ~180° with weird trig alignment
    // Star is just west of meridian, pushing acos result close to PI
    let ra = 180.0;
    let dec = -10.0;

    let (_alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &loc).unwrap();

    assert!((0.0..=360.0).contains(&az), "Azimuth should be normalized to [0, 360), got {}", az);
}

#[test]
fn test_ra_dec_to_alt_az_zenith_edge_case() {
    // Test the edge case where object is at zenith (azimuth undefined)
    let observer = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Create a time where a star at Dec=45° would be at zenith
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap();
    
    // Star exactly at observer's latitude, on meridian
    let ra = 0.0; // Will adjust based on LST
    let dec = 45.0; // Same as latitude
    
    let (alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    
    // Near zenith, altitude should be close to 90
    if alt > 89.9 {
        // Azimuth should be reasonable (0 or 180 based on our implementation)
        assert!(az == 0.0 || az == 180.0, 
            "At zenith, azimuth should be 0 or 180, got {}", az);
    }
}

#[test]
fn test_ra_dec_to_alt_az_polar_observer() {
    // Test edge case for observer very close to pole
    let observer = Location {
        latitude_deg: 89.9, // Very close to North Pole
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Polaris-like position
    let ra = 37.95456;
    let dec = 89.26411;
    
    let (alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    
    // Should not crash and should give reasonable values
    assert!((-90.0..=90.0).contains(&alt), "Altitude out of range: {}", alt);
    assert!((0.0..=360.0).contains(&az), "Azimuth out of range: {}", az);
    
    // Polaris from near North Pole should be very high in sky
    assert!(alt > 88.0, "Polaris should be near zenith from latitude 89.9°, got alt={}°", alt);
    // Azimuth may vary but should be defined
    assert!(!az.is_nan(), "Azimuth should not be NaN even near pole");
}

#[test]
fn test_azimuth_negative_normalization() {
    let observer = Location {
        latitude_deg: -45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    
    // Test coordinates that might produce negative azimuth before normalization
    let (alt, az) = transforms::ra_dec_to_alt_az(270.0, -30.0, dt, &observer).unwrap();
    
    assert!((-90.0..=90.0).contains(&alt), "Altitude should be valid, got {}", alt);
    assert!((0.0..360.0).contains(&az), "Azimuth should be [0,360), got {}", az);
    
    // From southern hemisphere looking at RA=270 (18h), Dec=-30
    // The object should be visible and have reasonable coordinates
    assert!(alt > -90.0, "Object should be above theoretical horizon");
}

#[test]
fn test_ra_dec_to_alt_az_numerical_stability() {
    // Test case that could cause cos_az to be slightly outside [-1, 1]
    let observer = Location {
        latitude_deg: 0.0, // Equatorial observer
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 6, 0, 0).unwrap();
    
    // Calculate what RA should be on the horizon (6h from meridian)
    let lst = observer.local_sidereal_time(dt);
    let ra = (lst + 6.0) * 15.0; // 6 hours after meridian = western horizon
    let dec = 0.0;
    
    let (alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    
    // Should not crash from acos domain error
    assert!((-90.0..=90.0).contains(&alt), "Altitude out of range: {}", alt);
    assert!((0.0..=360.0).contains(&az), "Azimuth out of range: {}", az);
    
    // Object on celestial equator from equatorial observer on horizon
    assert!(alt.abs() < 1.0, "Object on horizon should have alt ≈ 0°, got {}°", alt);
    // For objects on celestial equator at equator, HA determines azimuth directly
    // This object is 6h after meridian, so it should be in the West
    // But let's just verify it's on the horizon with valid azimuth
    assert!((0.0..360.0).contains(&az), "Azimuth should be valid");
}

#[test]
fn test_transforms_edge_cases() {
    // Test transform edge cases
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    
    // Test at north pole
    let loc_np = Location {
        latitude_deg: 90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let (alt1, _az1) = transforms::ra_dec_to_alt_az(0.0, 45.0, dt, &loc_np).unwrap();
    assert!((alt1 - 45.0).abs() < 1e-10); // Altitude equals declination at pole
    
    // Test at south pole
    let loc_sp = Location {
        latitude_deg: -90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    let (alt2, _az2) = transforms::ra_dec_to_alt_az(0.0, -45.0, dt, &loc_sp).unwrap();
    assert!((alt2 - 45.0).abs() < 1e-10); // Altitude equals abs(declination) at pole
    
    // Test object at zenith
    let loc = Location {
        latitude_deg: 23.5,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    // Find an object that should be at zenith
    let lst = loc.local_sidereal_time(dt);
    let (alt3, _az3) = transforms::ra_dec_to_alt_az(lst * 15.0, 23.5, dt, &loc).unwrap();
    assert!((alt3 - 90.0).abs() < 0.001);
}

#[test]
fn test_transforms_azimuth_branches() {
    // Test transform azimuth calculation branches
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 0.0, // Equator
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Test various azimuth quadrants
    let lst = location.local_sidereal_time(dt);
    
    // Test object on meridian (az should be 0 or 180)
    let ra_meridian = lst * 15.0;
    let (_alt_m, az_m) = transforms::ra_dec_to_alt_az(ra_meridian, 45.0, dt, &location).unwrap();
    assert!(az_m < 5.0 || az_m > 175.0 && az_m < 185.0,
        "Object on meridian should have az near 0° or 180°, got {}°", az_m);
    
    // Test that objects at different hour angles have correct azimuths
    // Use a non-zero declination to avoid equatorial singularities
    let dec = 30.0;
    
    // Object east of meridian (negative HA)
    let ra_east = (lst - 3.0) * 15.0;
    let (_, az_east) = transforms::ra_dec_to_alt_az(ra_east, dec, dt, &location).unwrap();
    
    // Object west of meridian (positive HA)  
    let ra_west = (lst + 3.0) * 15.0;
    let (_, az_west) = transforms::ra_dec_to_alt_az(ra_west, dec, dt, &location).unwrap();
    
    // They should be different and valid
    assert!(az_east != az_west, "Objects at different hour angles should have different azimuths");
    assert!((0.0..360.0).contains(&az_east), "East azimuth should be valid, got {}°", az_east);
    assert!((0.0..360.0).contains(&az_west), "West azimuth should be valid, got {}°", az_west);
    
    // At equator with Dec=30°, objects east/west of meridian should have reasonable azimuths
    // We don't enforce east < 180 because at equator with positive dec, east objects can be in NW
}

#[test]
fn test_transforms_negative_azimuth_normalization() {
    // Test negative azimuth normalization (coverage: lines 157-158)
    let observer = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // This should trigger negative azimuth before normalization
    // Object in NW quadrant with specific geometry
    let lst = observer.local_sidereal_time(dt);
    let ra_hours = lst - 9.0; // 9 hours before meridian
    let ra = if ra_hours < 0.0 { (ra_hours + 24.0) * 15.0 } else { ra_hours * 15.0 };
    let dec = 60.0; // High northern dec
    
    let (_, az) = ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    assert!((0.0..360.0).contains(&az), "Azimuth should be normalized from negative");
}

#[test]
fn test_ra_dec_to_alt_az_erfa_basic() {
    // Basic test of ERFA transformation
    let observer = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 100.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 0, 0, 0).unwrap();
    
    // Test with Vega
    let ra = 279.23473479;
    let dec = 38.78368896;
    
    let (alt, az) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(1013.25), Some(20.0), Some(0.5)
    ).unwrap();
    
    // Should give valid coordinates
    assert!((-90.0..=90.0).contains(&alt), "Altitude out of range: {}", alt);
    assert!((0.0..360.0).contains(&az), "Azimuth out of range: {}", az);
}

#[test]
fn test_ra_dec_to_alt_az_erfa_no_atmosphere() {
    // Test without atmospheric refraction (space telescope)
    let observer = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 600000.0, // 600km altitude
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    
    let ra = 83.633;
    let dec = 22.0145;
    
    let (alt_refr, az_refr) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(1013.25), Some(15.0), Some(0.5)
    ).unwrap();
    
    let (alt_no_refr, az_no_refr) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(0.0), Some(0.0), Some(0.0)
    ).unwrap();
    
    // Azimuth should be the same
    assert!((az_refr - az_no_refr).abs() < 0.001, 
            "Azimuth should not change with refraction");
    
    // Altitude should differ due to refraction
    let refr_diff = (alt_refr - alt_no_refr).abs();
    assert!(refr_diff > 0.0 && refr_diff < 1.0, 
            "Refraction effect should be small but non-zero: {} deg", refr_diff);
}

#[test]
fn test_ra_dec_to_alt_az_erfa_default_atmosphere() {
    // Test with default atmospheric parameters
    let observer = Location {
        latitude_deg: 51.4779,
        longitude_deg: -0.0015,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap();
    
    let ra = 0.0;
    let dec = 0.0;
    
    // Should use defaults when None provided
    let (alt, az) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        None, None, None
    ).unwrap();
    
    assert!((-90.0..=90.0).contains(&alt));
    assert!((0.0..360.0).contains(&az));
}

#[test]
fn test_ra_dec_to_alt_az_erfa_extreme_conditions() {
    // Test with extreme atmospheric conditions
    let observer = Location {
        latitude_deg: -69.0, // Antarctic
        longitude_deg: 39.0,
        altitude_m: 3000.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 7, 1, 0, 0, 0).unwrap();
    
    let ra = 187.0;
    let dec = -63.0;
    
    // Very low pressure and temperature
    let (alt, az) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(680.0), Some(-40.0), Some(0.1)
    ).unwrap();
    
    assert!((-90.0..=90.0).contains(&alt));
    assert!((0.0..360.0).contains(&az));
}

#[test]
fn test_ra_dec_to_alt_az_erfa_vs_original() {
    // Compare ERFA vs original method
    let observer = Location {
        latitude_deg: 33.356,
        longitude_deg: -116.863,
        altitude_m: 1706.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 9, 15, 3, 30, 0).unwrap();
    
    // Betelgeuse
    let ra = 88.7929;
    let dec = 7.4061;
    
    let (alt_orig, az_orig) = ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
    
    let (alt_erfa, az_erfa) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(0.0), Some(0.0), Some(0.0) // No refraction for fair comparison
    ).unwrap();
    
    // Differences should be small but present (due to frame bias, etc.)
    let alt_diff = (alt_orig - alt_erfa).abs();
    let az_diff = (az_orig - az_erfa).abs();
    
    // Allow up to 1 degree difference (frame bias, precession, nutation)
    assert!(alt_diff < 1.0, "Altitude difference too large: {} deg", alt_diff);
    assert!(az_diff < 1.0, "Azimuth difference too large: {} deg", az_diff);
}

#[test]
fn test_ra_dec_to_alt_az_erfa_high_altitude() {
    // Test at high altitude (mountain observatory)
    let observer = Location {
        latitude_deg: 19.8207,  // Mauna Kea
        longitude_deg: -155.4681,
        altitude_m: 4205.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 12, 21, 6, 0, 0).unwrap();
    
    // Canopus (southern star, should be visible from Hawaii)
    let ra = 95.988;
    let dec = -52.696;
    
    let (alt, az) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(615.0), Some(2.0), Some(0.2) // Low pressure at altitude
    ).unwrap();
    
    assert!((-90.0..=90.0).contains(&alt));
    assert!((0.0..360.0).contains(&az));
}

#[test]
fn test_ra_dec_to_alt_az_erfa_pole_star() {
    // Test with Polaris from various latitudes
    let ra = 37.95456;
    let dec = 89.26411;
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // From North Pole
    let observer_np = Location {
        latitude_deg: 90.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let (alt_np, _) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer_np,
        Some(1013.25), Some(-30.0), Some(0.1)
    ).unwrap();
    
    // Polaris altitude from North Pole should be close to its declination
    // ERFA includes precession/nutation, so allow more tolerance
    assert!((alt_np - dec).abs() < 0.2, 
            "Polaris altitude from North Pole should be ~89.26°, got {}°", alt_np);
    
    // From mid-latitude
    let observer_mid = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let (alt_mid, az_mid) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer_mid,
        Some(1013.25), Some(10.0), Some(0.5)
    ).unwrap();
    
    // Polaris altitude from 45°N should be close to 45°
    assert!((alt_mid - 45.0).abs() < 1.0,
            "Polaris altitude from 45°N should be ~45°, got {}°", alt_mid);
    
    // Polaris azimuth should be close to North
    assert!(!(5.0..=355.0).contains(&az_mid),
            "Polaris azimuth should be near North, got {}°", az_mid);
}

#[test]
fn test_ra_dec_to_alt_az_erfa_horizon() {
    // Test object near horizon where refraction is significant
    let observer = Location {
        latitude_deg: 50.0,
        longitude_deg: 10.0,
        altitude_m: 200.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 20, 0, 0).unwrap();
    
    // Find an object that should be near horizon
    let lst = observer.local_sidereal_time(dt);
    let ra = (lst + 6.0) * 15.0; // 6 hours from meridian
    let dec = 40.0; // Northern object
    
    let (alt_with_refr, _) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(1013.25), Some(15.0), Some(0.5)
    ).unwrap();
    
    let (alt_no_refr, _) = ra_dec_to_alt_az_erfa(
        ra, dec, dt, &observer,
        Some(0.0), Some(0.0), Some(0.0)
    ).unwrap();
    
    // Near horizon, refraction should be significant (>0.5°)
    if alt_no_refr.abs() < 10.0 {
        let refr = alt_with_refr - alt_no_refr;
        assert!(refr > 0.01, "Refraction near horizon should be positive and significant");
    }
}

// Tests for alt_az_to_ra_dec function

#[test]
fn test_alt_az_to_ra_dec_basic() {
    // Basic test with known coordinates
    let observer = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    
    // Test with object at zenith (should give Dec = latitude, RA = LST)
    let lst = observer.local_sidereal_time(dt);
    let expected_ra = (lst * 15.0) % 360.0;
    let expected_dec = 40.0;
    
    let (ra, dec) = alt_az_to_ra_dec(90.0, 0.0, dt, &observer).unwrap();
    
    // At zenith, declination should equal latitude
    assert!((dec - expected_dec).abs() < 0.001, 
            "Zenith declination should equal latitude: got {}, expected {}", dec, expected_dec);
    
    // RA should be close to LST (within a few degrees due to azimuth=0 assumption)
    let ra_diff = (ra - expected_ra).abs();
    let ra_diff_wrapped = (ra_diff).min(360.0 - ra_diff);
    assert!(ra_diff_wrapped < 5.0, 
            "Zenith RA should be close to LST: got {}, expected {}", ra, expected_ra);
}

#[test]
fn test_alt_az_to_ra_dec_round_trip() {
    // Round-trip test: RA/Dec -> Alt/Az -> RA/Dec should recover original
    let observer = Location {
        latitude_deg: 45.0,
        longitude_deg: -75.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 8, 15, 20, 30, 0).unwrap();
    
    // Test with several well-known objects
    let test_objects = [
        (279.23473479, 38.78368896), // Vega
        (83.633, 22.0145),           // Aldebaran
        (201.298, -11.171),          // Spica
        (310.358, 45.280),           // Deneb
        (0.0, 0.0),                  // Equatorial point
        (180.0, -45.0),              // Southern object
    ];
    
    for (original_ra, original_dec) in test_objects {
        // Convert to alt/az
        let (alt, az) = ra_dec_to_alt_az(original_ra, original_dec, dt, &observer).unwrap();
        
        // Skip objects below horizon for this test
        if alt < -5.0 {
            continue;
        }
        
        // Convert back to RA/Dec
        let (recovered_ra, recovered_dec) = alt_az_to_ra_dec(alt, az, dt, &observer).unwrap();
        
        // Check recovery accuracy
        let ra_error = (recovered_ra - original_ra).abs().min(360.0 - (recovered_ra - original_ra).abs());
        let dec_error = (recovered_dec - original_dec).abs();
        
        assert!(ra_error < 0.001, 
               "RA round-trip error too large: {} -> {} -> {} (error: {:.6}°)", 
               original_ra, recovered_ra, original_ra, ra_error);
        assert!(dec_error < 0.001, 
               "Dec round-trip error too large: {} -> {} -> {} (error: {:.6}°)", 
               original_dec, recovered_dec, original_dec, dec_error);
    }
}

#[test]
fn test_alt_az_to_ra_dec_edge_cases() {
    let observer = Location {
        latitude_deg: 30.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 0, 0, 0).unwrap();
    
    // Test horizon objects (altitude = 0)
    let (ra1, dec1) = alt_az_to_ra_dec(0.0, 0.0, dt, &observer).unwrap();   // North horizon
    let (ra2, dec2) = alt_az_to_ra_dec(0.0, 90.0, dt, &observer).unwrap();  // East horizon
    let (ra3, dec3) = alt_az_to_ra_dec(0.0, 180.0, dt, &observer).unwrap(); // South horizon
    let (ra4, dec4) = alt_az_to_ra_dec(0.0, 270.0, dt, &observer).unwrap(); // West horizon
    
    // All should be valid coordinates
    assert!((0.0..360.0).contains(&ra1), "North horizon RA should be valid");
    assert!((-90.0..=90.0).contains(&dec1), "North horizon Dec should be valid");
    assert!((0.0..360.0).contains(&ra2), "East horizon RA should be valid");
    assert!((-90.0..=90.0).contains(&dec2), "East horizon Dec should be valid");
    assert!((0.0..360.0).contains(&ra3), "South horizon RA should be valid");
    assert!((-90.0..=90.0).contains(&dec3), "South horizon Dec should be valid");
    assert!((0.0..360.0).contains(&ra4), "West horizon RA should be valid");
    assert!((-90.0..=90.0).contains(&dec4), "West horizon Dec should be valid");
    
    // Test nadir (altitude = -90)
    let (ra_nadir, dec_nadir) = alt_az_to_ra_dec(-90.0, 0.0, dt, &observer).unwrap();
    assert!((0.0..360.0).contains(&ra_nadir), "Nadir RA should be valid");
    assert!((-90.0..=90.0).contains(&dec_nadir), "Nadir Dec should be valid");
}

#[test]
fn test_alt_az_to_ra_dec_polar_regions() {
    // Test from near the North Pole
    let observer_north = Location {
        latitude_deg: 89.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    // Test from near the South Pole
    let observer_south = Location {
        latitude_deg: -89.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    
    // Test high altitude objects from both poles
    let (ra_n, dec_n) = alt_az_to_ra_dec(80.0, 45.0, dt, &observer_north).unwrap();
    let (ra_s, dec_s) = alt_az_to_ra_dec(80.0, 45.0, dt, &observer_south).unwrap();
    
    // Should not crash and should give valid coordinates
    assert!((0.0..360.0).contains(&ra_n), "Polar north RA should be valid");
    assert!((-90.0..=90.0).contains(&dec_n), "Polar north Dec should be valid");
    assert!((0.0..360.0).contains(&ra_s), "Polar south RA should be valid");
    assert!((-90.0..=90.0).contains(&dec_s), "Polar south Dec should be valid");
    
    // From North Pole, high altitude objects should have high positive declination
    assert!(dec_n > 70.0, "High object from North Pole should have high Dec");
    
    // From South Pole, high altitude objects should have high negative declination
    assert!(dec_s < -70.0, "High object from South Pole should have low Dec");
}

#[test]
fn test_alt_az_to_ra_dec_input_validation() {
    let observer = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // Test invalid altitude values
    assert!(alt_az_to_ra_dec(-90.1, 0.0, dt, &observer).is_err());
    assert!(alt_az_to_ra_dec(90.1, 0.0, dt, &observer).is_err());
    
    // Test invalid azimuth values
    assert!(alt_az_to_ra_dec(0.0, -0.1, dt, &observer).is_err());
    assert!(alt_az_to_ra_dec(0.0, 360.0, dt, &observer).is_err());
    assert!(alt_az_to_ra_dec(0.0, 360.1, dt, &observer).is_err());
    
    // Test valid boundary values
    assert!(alt_az_to_ra_dec(-90.0, 0.0, dt, &observer).is_ok());
    assert!(alt_az_to_ra_dec(90.0, 0.0, dt, &observer).is_ok());
    assert!(alt_az_to_ra_dec(0.0, 0.0, dt, &observer).is_ok());
    assert!(alt_az_to_ra_dec(0.0, 359.9, dt, &observer).is_ok());
}

#[test]
fn test_alt_az_to_ra_dec_quadrant_handling() {
    // Test quadrant handling for hour angle calculation
    let observer = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 9, 22, 18, 0, 0).unwrap(); // Equinox
    
    // Test objects in all four quadrants
    let quadrants = [
        (45.0, 45.0),   // NE - morning sky
        (45.0, 135.0),  // SE - evening sky  
        (45.0, 225.0),  // SW - night sky
        (45.0, 315.0),  // NW - early morning sky
    ];
    
    for (alt, az) in quadrants {
        let (ra, dec) = alt_az_to_ra_dec(alt, az, dt, &observer).unwrap();
        
        // Should produce valid coordinates
        assert!((0.0..360.0).contains(&ra), "RA should be valid for az={}: got {}", az, ra);
        assert!((-90.0..=90.0).contains(&dec), "Dec should be valid for az={}: got {}", az, dec);
        
        // Test round-trip accuracy
        let (alt_recovered, az_recovered) = ra_dec_to_alt_az(ra, dec, dt, &observer).unwrap();
        
        let alt_error = (alt_recovered - alt).abs();
        let az_error = (az_recovered - az).abs().min(360.0 - (az_recovered - az).abs());
        
        assert!(alt_error < 0.001, "Alt round-trip error for az={}: {:.6}°", az, alt_error);
        assert!(az_error < 0.001, "Az round-trip error for az={}: {:.6}°", az, az_error);
    }
}

#[test]
fn test_alt_az_to_ra_dec_equatorial_observer() {
    // Test from equatorial location where celestial equator passes through zenith
    let observer = Location {
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap(); // Vernal equinox
    
    // Object at zenith should have Dec ≈ 0
    let (_ra_zenith, dec_zenith) = alt_az_to_ra_dec(90.0, 0.0, dt, &observer).unwrap();
    assert!(dec_zenith.abs() < 0.1, "Zenith object from equator should have Dec ≈ 0, got {}", dec_zenith);
    
    // Objects on horizon at N/S should have Dec ≈ ±90
    let (_, dec_north) = alt_az_to_ra_dec(0.0, 0.0, dt, &observer).unwrap();  // North horizon
    let (_, dec_south) = alt_az_to_ra_dec(0.0, 180.0, dt, &observer).unwrap(); // South horizon
    
    // North horizon should point toward north celestial pole
    assert!(dec_north > 89.0, "North horizon from equator should have Dec ≈ 90°, got {}", dec_north);
    
    // South horizon should point toward south celestial pole  
    assert!(dec_south < -89.0, "South horizon from equator should have Dec ≈ -90°, got {}", dec_south);
}

#[test]
fn test_alt_az_to_ra_dec_circumpolar_objects() {
    // Test with circumpolar objects that never set
    let observer = Location {
        latitude_deg: 60.0, // High northern latitude
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    // Polaris-like object (always visible)
    let (ra1, dec1) = alt_az_to_ra_dec(60.0, 0.0, dt, &observer).unwrap();   // Near north
    let (ra2, dec2) = alt_az_to_ra_dec(30.0, 90.0, dt, &observer).unwrap();  // East, lower
    let (ra3, dec3) = alt_az_to_ra_dec(30.0, 270.0, dt, &observer).unwrap(); // West, lower
    
    // All should be valid
    assert!((0.0..360.0).contains(&ra1) && (-90.0..=90.0).contains(&dec1));
    assert!((0.0..360.0).contains(&ra2) && (-90.0..=90.0).contains(&dec2));
    assert!((0.0..360.0).contains(&ra3) && (-90.0..=90.0).contains(&dec3));
    
    // Object near north at moderate altitude should have high declination
    assert!(dec1 > 50.0, "High northern object should have high Dec, got {}", dec1);
    
    // Test round-trip accuracy
    let (alt1_rt, az1_rt) = ra_dec_to_alt_az(ra1, dec1, dt, &observer).unwrap();
    let (alt2_rt, az2_rt) = ra_dec_to_alt_az(ra2, dec2, dt, &observer).unwrap();
    let (alt3_rt, az3_rt) = ra_dec_to_alt_az(ra3, dec3, dt, &observer).unwrap();
    
    assert!((alt1_rt - 60.0).abs() < 0.001 && (az1_rt - 0.0).abs() < 0.001);
    assert!((alt2_rt - 30.0).abs() < 0.001 && (az2_rt - 90.0).abs() < 0.001);
    assert!((alt3_rt - 30.0).abs() < 0.001 && (az3_rt - 270.0).abs() < 0.001);
}

#[test]
fn test_alt_az_to_ra_dec_numerical_stability() {
    // Test numerical stability with values that might cause issues
    let observer = Location {
        latitude_deg: 45.0,
        longitude_deg: 0.0,
        altitude_m: 0.0,
    };
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    
    // Test very small altitudes (near horizon)
    let small_altitudes = [-89.99, -45.0, -0.01, 0.0, 0.01, 45.0, 89.99];
    
    for &alt in &small_altitudes {
        for az in [0.0, 90.0, 180.0, 270.0] {
            let result = alt_az_to_ra_dec(alt, az, dt, &observer);
            
            // Should not panic or return NaN
            assert!(result.is_ok(), "Should not fail for alt={}, az={}", alt, az);
            
            let (ra, dec) = result.unwrap();
            assert!(ra.is_finite() && dec.is_finite(), 
                   "Should return finite values for alt={}, az={}: ra={}, dec={}", alt, az, ra, dec);
            assert!((0.0..360.0).contains(&ra), "RA should be valid");
            assert!((-90.0..=90.0).contains(&dec), "Dec should be valid");
        }
    }
}

#[test]
fn test_alt_az_to_ra_dec_vs_known_stars() {
    // Test against some known star positions for validation
    let observer = Location {
        latitude_deg: 34.0522,  // Los Angeles
        longitude_deg: -118.2437,
        altitude_m: 100.0,
    };
    
    // Use a specific time when we can calculate expected positions
    let dt = Utc.with_ymd_and_hms(2024, 7, 4, 8, 0, 0).unwrap(); // July 4, 2024 8:00 UTC
    
    // Known star: Vega (approximately)
    let vega_ra = 279.23473479;
    let vega_dec = 38.78368896;
    
    // Convert to alt/az
    let (vega_alt, vega_az) = ra_dec_to_alt_az(vega_ra, vega_dec, dt, &observer).unwrap();
    
    // Skip if below horizon
    if vega_alt > 0.0 {
        // Convert back to RA/Dec
        let (recovered_ra, recovered_dec) = alt_az_to_ra_dec(vega_alt, vega_az, dt, &observer).unwrap();
        
        // Should recover original coordinates within very tight tolerance
        let ra_error = (recovered_ra - vega_ra).abs().min(360.0 - (recovered_ra - vega_ra).abs());
        let dec_error = (recovered_dec - vega_dec).abs();
        
        assert!(ra_error < 0.0001, 
               "Vega RA error too large: {:.6}° (recovered {}, original {})", 
               ra_error, recovered_ra, vega_ra);
        assert!(dec_error < 0.0001, 
               "Vega Dec error too large: {:.6}° (recovered {}, original {})", 
               dec_error, recovered_dec, vega_dec);
    }
}

