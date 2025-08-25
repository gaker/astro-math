//! Moon position and phase calculations.
//!
//! Uses ERFA's high-precision Moon98 function based on the ELP2000-82 lunar theory
//! for professional-grade accuracy.

use crate::julian_date;
use chrono::{DateTime, Utc};

/// Calculates the Moon's ecliptic longitude and latitude using ERFA's high-precision Moon98.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Tuple of (longitude, latitude) in degrees
pub fn moon_position(datetime: DateTime<Utc>) -> (f64, f64) {
    let jd = julian_date(datetime);
    
    // Approximate TT from UTC (ignoring leap seconds for now)
    use crate::time_scales::utc_to_tt_jd;
    let tt = utc_to_tt_jd(jd);
    
    // Get Moon position-velocity using ERFA Moon98 (GCRS coordinates)
    let pv = erfars::ephemerides::Moon98(tt, 0.0);
    
    // Extract position (AU)
    let x = pv[0];
    let y = pv[1];
    let z = pv[2];
    
    // Convert to ecliptic coordinates
    // First get obliquity of ecliptic
    let eps_rad = erfars::precnutpolar::Obl06(tt, 0.0);
    let cos_eps = eps_rad.cos();
    let sin_eps = eps_rad.sin();
    
    // Rotate from equatorial to ecliptic
    let x_ecl = x;
    let y_ecl = cos_eps * y + sin_eps * z;
    let z_ecl = -sin_eps * y + cos_eps * z;
    
    // Convert to spherical ecliptic coordinates
    let lon_rad = y_ecl.atan2(x_ecl);
    let lat_rad = z_ecl.atan2((x_ecl * x_ecl + y_ecl * y_ecl).sqrt());
    
    // Convert to degrees and normalize longitude
    let mut longitude = lon_rad.to_degrees();
    let latitude = lat_rad.to_degrees();
    
    // Normalize longitude to [0, 360)
    if longitude < 0.0 {
        longitude += 360.0;
    } else if longitude >= 360.0 {
        longitude -= 360.0;
    }
    
    (longitude, latitude)
}

/// Calculates the Moon's phase angle using ERFA's high-precision ephemerides.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Phase angle in degrees (0° = New Moon, 180° = Full Moon)
pub fn moon_phase_angle(datetime: DateTime<Utc>) -> f64 {
    // Get Moon's ecliptic longitude
    let (moon_lon, _) = moon_position(datetime);
    
    // Get Sun's ecliptic longitude
    let jd = julian_date(datetime);
    use crate::time_scales::utc_to_tt_jd;
    let tt = utc_to_tt_jd(jd);
    
    // Get Earth position relative to Sun (heliocentric)
    let (earth_h, _earth_b) = erfars::ephemerides::Epv00(tt, 0.0);
    // Sun position relative to Earth is negative of Earth's heliocentric position
    let sun_x = -earth_h[0];
    let sun_y = -earth_h[1];
    let sun_z = -earth_h[2];
    
    // Convert Sun position to ecliptic longitude
    // First get obliquity
    let eps_rad = erfars::precnutpolar::Obl06(tt, 0.0);
    let cos_eps = eps_rad.cos();
    let sin_eps = eps_rad.sin();
    
    // Rotate from equatorial to ecliptic
    let sun_x_ecl = sun_x;
    let sun_y_ecl = cos_eps * sun_y + sin_eps * sun_z;
    
    // Get Sun's ecliptic longitude
    let sun_lon_rad = sun_y_ecl.atan2(sun_x_ecl);
    let mut sun_lon = sun_lon_rad.to_degrees();
    if sun_lon < 0.0 {
        sun_lon += 360.0;
    }
    
    // Phase angle is the difference in ecliptic longitudes
    let mut phase = moon_lon - sun_lon;
    
    // Normalize to 0-360
    if phase < 0.0 {
        phase += 360.0;
    } else if phase >= 360.0 {
        phase -= 360.0;
    }
    
    phase
}

/// Calculates the Moon's illumination percentage.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Illumination percentage (0-100)
pub fn moon_illumination(datetime: DateTime<Utc>) -> f64 {
    let phase_angle = moon_phase_angle(datetime);
    let phase_rad = phase_angle.to_radians();
    
    // Calculate illumination using phase angle
    let illumination = 50.0 * (1.0 - phase_rad.cos());
    
    illumination.clamp(0.0, 100.0)
}

/// Returns a descriptive name for the Moon's phase.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Phase name as a string
pub fn moon_phase_name(datetime: DateTime<Utc>) -> &'static str {
    let phase = moon_phase_angle(datetime);
    
    match phase {
        p if p < 22.5 => "New Moon",
        p if p < 67.5 => "Waxing Crescent",
        p if p < 112.5 => "First Quarter",
        p if p < 157.5 => "Waxing Gibbous",
        p if p < 202.5 => "Full Moon",
        p if p < 247.5 => "Waning Gibbous",
        p if p < 292.5 => "Last Quarter",
        p if p < 337.5 => "Waning Crescent",
        _ => "New Moon",
    }
}

/// Calculates the Moon's distance from Earth using ERFA's high-precision Moon98.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Distance in kilometers
pub fn moon_distance(datetime: DateTime<Utc>) -> f64 {
    let jd = julian_date(datetime);
    
    // Approximate TT from UTC
    use crate::time_scales::utc_to_tt_jd;
    let tt = utc_to_tt_jd(jd);
    
    // Get Moon position-velocity using ERFA Moon98
    let pv = erfars::ephemerides::Moon98(tt, 0.0);
    
    // Calculate distance from position vector (in AU)
    let x = pv[0];
    let y = pv[1];
    let z = pv[2];
    let distance_au = (x * x + y * y + z * z).sqrt();
    
    // Convert AU to kilometers (1 AU = 149,597,870.7 km)
    distance_au * 149_597_870.7
}

/// Calculates the Moon's equatorial coordinates using ERFA's high-precision Moon98.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Tuple of (right_ascension, declination) in degrees (GCRS)
pub fn moon_equatorial(datetime: DateTime<Utc>) -> (f64, f64) {
    let jd = julian_date(datetime);
    
    // Approximate TT from UTC
    use crate::time_scales::utc_to_tt_jd;
    let tt = utc_to_tt_jd(jd);
    
    // Get Moon position-velocity using ERFA Moon98 (already in GCRS equatorial)
    let pv = erfars::ephemerides::Moon98(tt, 0.0);
    
    // Extract position and convert to spherical coordinates
    let x = pv[0];
    let y = pv[1];
    let z = pv[2];
    
    // Convert to spherical coordinates
    let ra_rad = y.atan2(x);
    let dec_rad = z.atan2((x * x + y * y).sqrt());
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra_rad.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    } else if ra_deg >= 360.0 {
        ra_deg -= 360.0;
    }
    
    (ra_deg, dec_rad.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_moon_phase_angle() {
        // Test known new moon
        let new_moon = Utc.with_ymd_and_hms(2024, 1, 11, 11, 57, 0).unwrap();
        let phase = moon_phase_angle(new_moon);
        assert!(!(10.0..=350.0).contains(&phase)); // Near 0 or 360
        
        // Test known full moon
        let full_moon = Utc.with_ymd_and_hms(2024, 1, 25, 17, 54, 0).unwrap();
        let phase = moon_phase_angle(full_moon);
        assert!(phase > 170.0 && phase < 190.0); // Near 180
    }

    #[test]
    fn test_moon_illumination() {
        // New moon should be ~0% illuminated
        let new_moon = Utc.with_ymd_and_hms(2024, 1, 11, 11, 57, 0).unwrap();
        let illum = moon_illumination(new_moon);
        assert!(illum < 5.0);
        
        // Full moon should be ~100% illuminated
        let full_moon = Utc.with_ymd_and_hms(2024, 1, 25, 17, 54, 0).unwrap();
        let illum = moon_illumination(full_moon);
        assert!(illum > 95.0);
    }

    #[test]
    fn test_moon_phase_names() {
        let new_moon = Utc.with_ymd_and_hms(2024, 1, 11, 12, 0, 0).unwrap();
        assert_eq!(moon_phase_name(new_moon), "New Moon");
        
        let first_quarter = Utc.with_ymd_and_hms(2024, 1, 18, 3, 0, 0).unwrap();
        let phase_name = moon_phase_name(first_quarter);
        assert!(phase_name == "First Quarter" || phase_name == "Waxing Crescent");
    }

    #[test]
    fn test_moon_distance() {
        // Check that distance is reasonable (356,500 - 406,700 km)
        let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
        let distance = moon_distance(dt);
        assert!(distance > 356000.0 && distance < 407000.0);
    }

    #[test]
    fn test_moon_equatorial() {
        // Test that coordinates are in valid ranges
        let dt = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
        let (ra, dec) = moon_equatorial(dt);
        assert!((0.0..360.0).contains(&ra));
        assert!((-90.0..=90.0).contains(&dec)); // Valid declination range
    }
}