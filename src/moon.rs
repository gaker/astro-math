//! Moon position and phase calculations.
//!
//! Based on low-precision formulas from Meeus suitable for most observational purposes.

use crate::julian_date;
use chrono::{DateTime, Utc};

/// Calculates the Moon's ecliptic longitude and latitude.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Tuple of (longitude, latitude) in degrees
pub fn moon_position(datetime: DateTime<Utc>) -> (f64, f64) {
    let jd = julian_date(datetime);
    let t = (jd - 2451545.0) / 36525.0; // Julian centuries from J2000
    
    // Mean elements
    let l_prime = 218.3164477 + 481267.88123421 * t 
        - 0.0015786 * t * t + t * t * t / 538841.0;
    let d = 297.8501921 + 445267.1114034 * t 
        - 0.0018819 * t * t + t * t * t / 545868.0;
    let m = 357.5291092 + 35999.0502909 * t 
        - 0.0001536 * t * t + t * t * t / 24490000.0;
    let m_prime = 134.9633964 + 477198.8675055 * t 
        + 0.0087414 * t * t + t * t * t / 69699.0;
    let f = 93.2720950 + 483202.0175233 * t 
        - 0.0036539 * t * t - t * t * t / 3526000.0;
    
    // Convert to radians
    let d_rad = d.to_radians();
    let m_rad = m.to_radians();
    let m_prime_rad = m_prime.to_radians();
    let f_rad = f.to_radians();
    
    // Longitude corrections (simplified)
    let mut longitude = l_prime + 6.288774 * m_prime_rad.sin()
        + 1.274027 * (2.0 * d_rad - m_prime_rad).sin()
        + 0.658314 * (2.0 * d_rad).sin()
        + 0.213618 * (2.0 * m_prime_rad).sin()
        - 0.185116 * m_rad.sin()
        - 0.114332 * (2.0 * f_rad).sin();
    
    // Latitude corrections (simplified)
    let latitude = 5.128122 * f_rad.sin()
        + 0.280602 * (m_prime_rad + f_rad).sin()
        + 0.277693 * (m_prime_rad - f_rad).sin()
        + 0.173237 * (2.0 * d_rad - f_rad).sin()
        + 0.055413 * (2.0 * d_rad - m_prime_rad + f_rad).sin();
    
    // Normalize longitude
    longitude = longitude % 360.0;
    if longitude < 0.0 {
        longitude += 360.0;
    }
    
    (longitude, latitude)
}

/// Calculates the Moon's phase angle.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Phase angle in degrees (0° = New Moon, 180° = Full Moon)
pub fn moon_phase_angle(datetime: DateTime<Utc>) -> f64 {
    let jd = julian_date(datetime);
    let t = (jd - 2451545.0) / 36525.0;
    
    // Sun's mean longitude
    let l_sun = 280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    
    // Moon's mean longitude
    let (l_moon, _) = moon_position(datetime);
    
    // Phase angle (elongation)
    let mut phase = l_moon - l_sun;
    
    // Normalize to 0-360
    phase = phase % 360.0;
    if phase < 0.0 {
        phase += 360.0;
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

/// Calculates the Moon's distance from Earth.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Distance in kilometers
pub fn moon_distance(datetime: DateTime<Utc>) -> f64 {
    let jd = julian_date(datetime);
    let t = (jd - 2451545.0) / 36525.0;
    
    // Mean anomaly
    let m_prime = 134.9633964 + 477198.8675055 * t;
    let m_prime_rad = m_prime.to_radians();
    
    // Mean distance
    let d = 297.8501921 + 445267.1114034 * t;
    let d_rad = d.to_radians();
    
    // Calculate distance (simplified)
    let distance = 385000.56 - 20905.355 * m_prime_rad.cos()
        - 3699.111 * (2.0 * d_rad - m_prime_rad).cos()
        - 2955.968 * (2.0 * d_rad).cos()
        - 569.925 * (2.0 * m_prime_rad).cos();
    
    distance
}

/// Calculates the Moon's equatorial coordinates.
///
/// # Arguments
/// * `datetime` - Observation time
///
/// # Returns
/// Tuple of (right_ascension, declination) in degrees
pub fn moon_equatorial(datetime: DateTime<Utc>) -> (f64, f64) {
    let (lon, lat) = moon_position(datetime);
    
    // Obliquity of ecliptic
    let jd = julian_date(datetime);
    let t = (jd - 2451545.0) / 36525.0;
    let epsilon = 23.439291 - 0.0130042 * t;
    let epsilon_rad = epsilon.to_radians();
    
    // Convert ecliptic to equatorial
    let lon_rad = lon.to_radians();
    let lat_rad = lat.to_radians();
    
    let ra = (epsilon_rad.sin() * lon_rad.sin() * lat_rad.cos() + 
             epsilon_rad.cos() * lat_rad.sin()).atan2(lon_rad.cos() * lat_rad.cos());
    
    let dec = (epsilon_rad.cos() * lon_rad.sin() * lat_rad.cos() - 
              epsilon_rad.sin() * lat_rad.sin()).asin();
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    }
    
    (ra_deg, dec.to_degrees())
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
        assert!(phase < 10.0 || phase > 350.0); // Near 0 or 360
        
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
        assert!(ra >= 0.0 && ra < 360.0);
        assert!(dec >= -90.0 && dec <= 90.0); // Valid declination range
    }
}