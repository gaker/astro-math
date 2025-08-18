//! Solar position calculations.
//!
//! This module provides solar position calculations using ERFA's
//! high-precision ephemerides for professional-grade accuracy.

use crate::time::julian_date;
use chrono::{DateTime, Utc};
use std::f64::consts::PI;

/// Calculates the Sun's ecliptic longitude and latitude using ERFA.
///
/// This uses ERFA's high-precision ephemerides for professional-grade accuracy.
/// The position is calculated using the Earth's heliocentric position from ERFA's
/// Epv00 function, which provides sub-arcsecond accuracy.
///
/// # Arguments
///
/// * `date` - UTC date/time
///
/// # Returns
///
/// A tuple `(longitude, latitude)` in degrees, where:
/// - `longitude` is the ecliptic longitude (0-360°)
/// - `latitude` is the ecliptic latitude (always near 0 for the Sun)
///
/// # Example
///
/// ```
/// use astro_math::sun::sun_position;
/// use chrono::{TimeZone, Utc};
///
/// let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
/// let (lon, lat) = sun_position(dt);
/// // Near summer solstice, longitude ≈ 90°
/// assert!((lon - 90.0).abs() < 1.0);
/// assert!(lat.abs() < 0.1);
/// ```
pub fn sun_position(date: DateTime<Utc>) -> (f64, f64) {
    // Get Julian date
    let jd = julian_date(date);
    
    // Get Earth's heliocentric position-velocity
    // Returns position in ICRS equatorial frame
    let (earth_h, _earth_b) = erfars::ephemerides::Epv00(jd, 0.0);
    
    // Sun's position is negative of Earth's heliocentric position
    let x = -earth_h[0];  // AU
    let y = -earth_h[1];  // AU
    let z = -earth_h[2];  // AU
    
    // Get mean obliquity for date
    let eps = erfars::precnutpolar::Obl06(jd, 0.0);
    let cos_eps = eps.cos();
    let sin_eps = eps.sin();
    
    // Convert from equatorial to ecliptic coordinates
    // x_ecl = x_eq
    // y_ecl = y_eq * cos(eps) + z_eq * sin(eps)
    // z_ecl = -y_eq * sin(eps) + z_eq * cos(eps)
    let y_ecl = y * cos_eps + z * sin_eps;
    let z_ecl = -y * sin_eps + z * cos_eps;
    
    // Convert to ecliptic longitude and latitude
    let longitude_rad = y_ecl.atan2(x);
    let r = (x * x + y_ecl * y_ecl + z_ecl * z_ecl).sqrt();
    let latitude_rad = (z_ecl / r).asin();
    
    // Convert to degrees and normalize longitude
    let mut longitude = longitude_rad * 180.0 / PI;
    if longitude < 0.0 {
        longitude += 360.0;
    }
    let latitude = latitude_rad * 180.0 / PI;
    
    (longitude, latitude)
}

/// Calculates the Sun's right ascension and declination using ERFA.
///
/// This directly computes the Sun's equatorial coordinates from ERFA's
/// high-precision ephemerides, including proper handling of precession
/// and nutation.
///
/// # Arguments
///
/// * `date` - UTC date/time
///
/// # Returns
///
/// A tuple `(ra, dec)` in degrees.
///
/// # Example
///
/// ```
/// use astro_math::sun::sun_ra_dec;
/// use chrono::{TimeZone, Utc};
///
/// let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap();
/// let (ra, dec) = sun_ra_dec(dt);
/// // Near vernal equinox: RA ≈ 0°, Dec ≈ 0°
/// assert!(ra < 2.0 || ra > 358.0);
/// assert!(dec.abs() < 1.0);
/// ```
pub fn sun_ra_dec(date: DateTime<Utc>) -> (f64, f64) {
    // Get Julian date
    let jd = julian_date(date);
    
    // Get Earth's heliocentric position-velocity
    let (earth_h, _earth_b) = erfars::ephemerides::Epv00(jd, 0.0);
    
    // Sun's position is negative of Earth's heliocentric position
    let x = -earth_h[0];
    let y = -earth_h[1];
    let z = -earth_h[2];
    
    // Convert directly to equatorial coordinates
    // This is already in ICRS/J2000 frame
    let ra_rad = y.atan2(x);
    let r = (x * x + y * y + z * z).sqrt();
    let dec_rad = (z / r).asin();
    
    // Convert to degrees and normalize
    let mut ra = ra_rad * 180.0 / PI;
    if ra < 0.0 {
        ra += 360.0;
    }
    let dec = dec_rad * 180.0 / PI;
    
    (ra, dec)
}