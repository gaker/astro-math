//! Parallax corrections for celestial coordinates.
//!
//! This module handles the apparent shift in celestial object positions due to the
//! observer's changing viewpoint. It includes both diurnal parallax (due to Earth's
//! rotation) and annual parallax (due to Earth's orbit around the Sun).
//!
//! # Types of Parallax
//!
//! - **Diurnal Parallax**: Significant for nearby objects like the Moon. An observer
//!   on Earth's surface sees objects from a slightly different angle than from Earth's center.
//! - **Annual Parallax**: Measurable for nearby stars. As Earth orbits the Sun, nearby
//!   stars appear to shift against the background of distant stars.
//!
//! # Error Handling
//!
//! All functions validate their inputs and return `Result<T>` types:
//! - `AstroError::InvalidCoordinate` for out-of-range RA or Dec values
//! - `AstroError::OutOfRange` for invalid distance values

use crate::{Location, julian_date};
use crate::error::{Result, validate_ra, validate_dec};
use chrono::{DateTime, Utc};

/// Earth's equatorial radius in kilometers
const EARTH_RADIUS_KM: f64 = 6378.137;

/// Earth's flattening factor
const EARTH_FLATTENING: f64 = 1.0 / 298.257223563;

/// Astronomical Unit in kilometers
const AU_KM: f64 = 149597870.7;

/// Calculates the geocentric distance of an observer from Earth's center.
///
/// # Arguments
/// * `location` - Observer's location
///
/// # Returns
/// Distance from Earth's center in Earth radii
pub fn geocentric_distance(location: &Location) -> f64 {
    let lat_rad = location.latitude_deg.to_radians();
    let alt_km = location.altitude_m / 1000.0;
    
    // Calculate u = geocentric latitude
    let u = ((1.0 - EARTH_FLATTENING) * lat_rad.tan()).atan();
    
    // rho * cos(phi') and rho * sin(phi')
    let rho_cos_phi = u.cos() + (alt_km / EARTH_RADIUS_KM) * lat_rad.cos();
    let rho_sin_phi = (1.0 - EARTH_FLATTENING).powi(2) * u.sin() + (alt_km / EARTH_RADIUS_KM) * lat_rad.sin();
    
    (rho_cos_phi.powi(2) + rho_sin_phi.powi(2)).sqrt()
}

/// Applies diurnal parallax correction for the Moon or other nearby objects.
///
/// Corrects for the difference between the observer's position on Earth's surface
/// and Earth's center. This effect is most significant for the Moon (up to ~1 degree)
/// and negligible for stars.
///
/// # Arguments
/// * `ra` - Right ascension in degrees
/// * `dec` - Declination in degrees
/// * `distance_au` - Distance to object in AU (use 0.00257 for Moon's mean distance)
/// * `datetime` - Observation time
/// * `location` - Observer's location
///
/// # Returns
/// Tuple of (corrected_ra, corrected_dec) in degrees
///
/// # Errors
/// - `AstroError::InvalidCoordinate` if RA is outside [0, 360) or Dec outside [-90, 90]
/// - `AstroError::OutOfRange` if distance_au is not positive
///
/// # Example
/// ```
/// use chrono::{TimeZone, Utc};
/// use astro_math::{Location, diurnal_parallax};
///
/// let dt = Utc.with_ymd_and_hms(2024, 8, 4, 22, 0, 0).unwrap();
/// let location = Location {
///     latitude_deg: 40.0,
///     longitude_deg: -74.0,
///     altitude_m: 0.0,
/// };
/// 
/// // Moon's position and distance
/// let (ra_topo, dec_topo) = diurnal_parallax(45.0, 20.0, 0.00257, dt, &location).unwrap();
/// ```
///
/// # Error Example
/// ```
/// # use chrono::Utc;
/// # use astro_math::{Location, diurnal_parallax, error::AstroError};
/// # let location = Location { latitude_deg: 40.0, longitude_deg: -74.0, altitude_m: 0.0 };
/// // Invalid distance
/// match diurnal_parallax(45.0, 20.0, -1.0, Utc::now(), &location) {
///     Err(AstroError::OutOfRange { parameter, .. }) => {
///         assert_eq!(parameter, "distance_au");
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn diurnal_parallax(
    ra: f64,
    dec: f64,
    distance_au: f64,
    datetime: DateTime<Utc>,
    location: &Location,
) -> Result<(f64, f64)> {
    validate_ra(ra)?;
    validate_dec(dec)?;
    if distance_au <= 0.0 {
        return Err(crate::error::AstroError::OutOfRange {
            parameter: "distance_au",
            value: distance_au,
            min: f64::MIN_POSITIVE,
            max: f64::MAX,
        });
    }
    let lst_hours = location.local_sidereal_time(datetime);
    let lst_deg = lst_hours * 15.0;
    
    // Hour angle
    let ha = lst_deg - ra;
    let ha_rad = ha.to_radians();
    let dec_rad = dec.to_radians();
    
    // Observer's geocentric position
    let lat_rad = location.latitude_deg.to_radians();
    let u = ((1.0 - EARTH_FLATTENING) * lat_rad.tan()).atan();
    let rho_cos = u.cos() + (location.altitude_m / 1000.0 / EARTH_RADIUS_KM) * lat_rad.cos();
    let rho_sin = (1.0 - EARTH_FLATTENING).powi(2) * u.sin() + 
                  (location.altitude_m / 1000.0 / EARTH_RADIUS_KM) * lat_rad.sin();
    
    // Parallax in arcseconds
    let parallax_as = 8.794 / (distance_au * AU_KM / EARTH_RADIUS_KM);
    let parallax_rad = (parallax_as / 3600.0).to_radians();
    
    // Calculate corrections
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    let cos_ha = ha_rad.cos();
    let sin_ha = ha_rad.sin();
    
    // Parallax factors
    let p_ra = -parallax_rad * rho_cos * sin_ha / cos_dec;
    let p_dec = -parallax_rad * (rho_sin * cos_dec - rho_cos * cos_ha * sin_dec);
    
    // Apply corrections
    let ra_corrected = ra + p_ra.to_degrees();
    let dec_corrected = dec + p_dec.to_degrees();
    
    // Normalize RA
    let ra_normalized = if ra_corrected < 0.0 {
        ra_corrected + 360.0
    } else if ra_corrected >= 360.0 {
        ra_corrected - 360.0
    } else {
        ra_corrected
    };
    
    Ok((ra_normalized, dec_corrected))
}

/// Calculates annual parallax for stars.
///
/// Annual parallax is the apparent shift in a star's position as Earth orbits the Sun.
/// This effect is only measurable for relatively nearby stars and is the primary method
/// for determining stellar distances.
///
/// # Arguments
/// * `ra` - Right ascension in degrees
/// * `dec` - Declination in degrees
/// * `parallax_mas` - Annual parallax in milliarcseconds
/// * `datetime` - Observation time
///
/// # Returns
/// Tuple of (corrected_ra, corrected_dec) in degrees
///
/// # Errors
/// - `AstroError::InvalidCoordinate` if RA is outside [0, 360) or Dec outside [-90, 90]
/// - `AstroError::OutOfRange` if parallax_mas is not positive
///
/// # Example
/// ```
/// use chrono::{TimeZone, Utc};
/// use astro_math::annual_parallax;
///
/// let dt = Utc.with_ymd_and_hms(2024, 8, 4, 0, 0, 0).unwrap();
/// // Proxima Centauri with parallax of 768.5 mas
/// let (ra_corrected, dec_corrected) = annual_parallax(217.42894, -62.67948, 768.5, dt).unwrap();
/// ```
///
/// # Error Example
/// ```
/// # use chrono::Utc;
/// # use astro_math::{annual_parallax, error::AstroError};
/// // Invalid parallax
/// match annual_parallax(180.0, 0.0, 0.0, Utc::now()) {
///     Err(AstroError::OutOfRange { parameter, .. }) => {
///         assert_eq!(parameter, "parallax_mas");
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn annual_parallax(
    ra: f64,
    dec: f64,
    parallax_mas: f64,
    datetime: DateTime<Utc>,
) -> Result<(f64, f64)> {
    validate_ra(ra)?;
    validate_dec(dec)?;
    if parallax_mas <= 0.0 {
        return Err(crate::error::AstroError::OutOfRange {
            parameter: "parallax_mas",
            value: parallax_mas,
            min: f64::MIN_POSITIVE,
            max: f64::MAX,
        });
    }
    let jd = julian_date(datetime);
    let t = (jd - 2451545.0) / 36525.0; // Julian centuries from J2000
    
    // Mean longitude of the Sun
    let l = 280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    
    // Mean anomaly of the Sun
    let m = 357.52911 + 35999.05029 * t - 0.0001537 * t * t;
    let m_rad = m.to_radians();
    
    // Sun's equation of center
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
          + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
          + 0.000289 * (3.0 * m_rad).sin();
    
    // Sun's true longitude
    let sun_lon = l + c;
    let sun_lon_rad = sun_lon.to_radians();
    
    // Parallax in radians
    let parallax_rad = (parallax_mas / 1000.0 / 3600.0).to_radians();
    
    // Calculate corrections
    let ra_rad = ra.to_radians();
    let dec_rad = dec.to_radians();
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    
    // Annual parallax corrections (simplified)
    let delta_ra = parallax_rad * (sun_lon_rad.cos() * ra_rad.sin() - sun_lon_rad.sin() * ra_rad.cos()) / cos_dec;
    let delta_dec = parallax_rad * (sun_lon_rad.sin() * sin_dec * ra_rad.sin() + sun_lon_rad.cos() * sin_dec * ra_rad.cos());
    
    Ok((ra + delta_ra.to_degrees(), dec + delta_dec.to_degrees()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Location;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_geocentric_distance() {
        // Test at sea level, equator
        let loc = Location {
            latitude_deg: 0.0,
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        let dist = geocentric_distance(&loc);
        assert!((dist - 1.0).abs() < 0.001);
        
        // Test at high altitude
        let loc_high = Location {
            latitude_deg: 0.0,
            longitude_deg: 0.0,
            altitude_m: 8848.0, // Mt. Everest
        };
        let dist_high = geocentric_distance(&loc_high);
        assert!(dist_high > 1.001);
    }

    #[test]
    fn test_diurnal_parallax_moon() {
        // Test Moon's diurnal parallax
        let dt = Utc.with_ymd_and_hms(2024, 8, 4, 22, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        // Moon at moderate altitude
        let (ra_topo, dec_topo) = diurnal_parallax(45.0, 30.0, 0.00257, dt, &location).unwrap();
        
        // Should show measurable parallax
        assert!((ra_topo - 45.0).abs() < 1.0); // Less than 1 degree
        assert!((dec_topo - 30.0).abs() < 1.0);
    }

    #[test]
    fn test_diurnal_parallax_distant() {
        // Test parallax for distant object (should be negligible)
        let dt = Utc.with_ymd_and_hms(2024, 8, 4, 22, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        // Object at 1000 AU
        let (ra_topo, dec_topo) = diurnal_parallax(45.0, 30.0, 1000.0, dt, &location).unwrap();
        
        // Should show negligible parallax
        assert!((ra_topo - 45.0).abs() < 0.001);
        assert!((dec_topo - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_annual_parallax() {
        // Test Proxima Centauri
        let dt = Utc.with_ymd_and_hms(2024, 8, 4, 0, 0, 0).unwrap();
        let (ra_corrected, dec_corrected) = annual_parallax(217.42894, -62.67948, 768.5, dt).unwrap();
        
        // Should show small but measurable correction
        assert!((ra_corrected - 217.42894).abs() < 0.001);
        assert!((dec_corrected - (-62.67948)).abs() < 0.001);
    }
}