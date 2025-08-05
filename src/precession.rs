//! Precession calculations for converting coordinates between different epochs.
//!
//! Based on IAU 2000/2006 precession model from Meeus Chapter 21.

use chrono::{DateTime, Utc};

/// Calculates precession angles (ζ, z, θ) in degrees for converting from J2000.0 to a given date.
///
/// Uses the IAU 2000 precession model with improved coefficients.
///
/// # Arguments
/// * `jd` - Julian Date of the target epoch
///
/// # Returns
/// Tuple of (zeta, z, theta) in degrees
pub fn precession_angles(jd: f64) -> (f64, f64, f64) {
    let t = (jd - 2451545.0) / 36525.0; // Julian centuries from J2000.0
    let t2 = t * t;
    let t3 = t2 * t;

    // IAU 2000 precession angles in arcseconds
    let zeta_as = 2306.2181 * t + 0.30188 * t2 + 0.017998 * t3;
    let z_as = 2306.2181 * t + 1.09468 * t2 + 0.018203 * t3;
    let theta_as = 2004.3109 * t - 0.42665 * t2 - 0.041833 * t3;

    // Convert to degrees
    (zeta_as / 3600.0, z_as / 3600.0, theta_as / 3600.0)
}

/// Applies precession from J2000.0 to a given date.
///
/// # Arguments
/// * `ra_j2000` - Right ascension at J2000.0 in degrees
/// * `dec_j2000` - Declination at J2000.0 in degrees
/// * `datetime` - Target date/time
///
/// # Returns
/// Tuple of (ra, dec) at the target epoch in degrees
///
/// # Example
/// ```
/// use chrono::{TimeZone, Utc};
/// use astro_math::precess_j2000_to_date;
///
/// let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let (ra, dec) = precess_j2000_to_date(0.0, 0.0, dt);
/// println!("Precessed coordinates: RA={:.4}°, Dec={:.4}°", ra, dec);
/// ```
pub fn precess_j2000_to_date(ra_j2000: f64, dec_j2000: f64, datetime: DateTime<Utc>) -> (f64, f64) {
    let jd = crate::julian_date(datetime);
    let (zeta, z, theta) = precession_angles(jd);

    // Convert to radians
    let ra_rad = ra_j2000.to_radians();
    let dec_rad = dec_j2000.to_radians();
    let zeta_rad = zeta.to_radians();
    let z_rad = z.to_radians();
    let theta_rad = theta.to_radians();

    // Apply precession matrix
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    let cos_ra_plus_zeta = (ra_rad + zeta_rad).cos();
    let sin_ra_plus_zeta = (ra_rad + zeta_rad).sin();

    let a = cos_dec * sin_ra_plus_zeta;
    let b = theta_rad.cos() * cos_dec * cos_ra_plus_zeta - theta_rad.sin() * sin_dec;
    let c = theta_rad.sin() * cos_dec * cos_ra_plus_zeta + theta_rad.cos() * sin_dec;

    let ra_new = a.atan2(b) + z_rad;
    let dec_new = c.asin();

    // Normalize RA to [0, 360)
    let ra_deg = ra_new.to_degrees();
    let ra_normalized = if ra_deg < 0.0 { ra_deg + 360.0 } else { ra_deg };

    (ra_normalized, dec_new.to_degrees())
}

/// Applies precession from a given date back to J2000.0.
///
/// # Arguments
/// * `ra` - Right ascension at the given date in degrees
/// * `dec` - Declination at the given date in degrees
/// * `datetime` - Current date/time
///
/// # Returns
/// Tuple of (ra, dec) at J2000.0 in degrees
pub fn precess_date_to_j2000(ra: f64, dec: f64, datetime: DateTime<Utc>) -> (f64, f64) {
    let jd = crate::julian_date(datetime);
    let (zeta, z, theta) = precession_angles(jd);

    // For inverse transformation, we use the transpose of the rotation matrix
    let ra_rad = ra.to_radians();
    let dec_rad = dec.to_radians();
    let zeta_rad = zeta.to_radians();
    let z_rad = z.to_radians();
    let theta_rad = theta.to_radians();

    // Apply inverse precession matrix (transpose of forward matrix)
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    let cos_ra_minus_z = (ra_rad - z_rad).cos();
    let sin_ra_minus_z = (ra_rad - z_rad).sin();

    let a = cos_dec * sin_ra_minus_z;
    let b = theta_rad.cos() * cos_dec * cos_ra_minus_z + theta_rad.sin() * sin_dec;
    let c = -theta_rad.sin() * cos_dec * cos_ra_minus_z + theta_rad.cos() * sin_dec;

    let ra_j2000 = a.atan2(b) - zeta_rad;
    let dec_j2000 = c.asin();

    // Normalize RA to [0, 360)
    let ra_deg = ra_j2000.to_degrees();
    let ra_normalized = if ra_deg < 0.0 { 
        ra_deg + 360.0 
    } else if ra_deg >= 360.0 {
        ra_deg - 360.0
    } else { 
        ra_deg 
    };

    (ra_normalized, dec_j2000.to_degrees())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_precession_angles() {
        // Test for J2000.0 (should be zero)
        let (zeta, z, theta) = precession_angles(2451545.0);
        assert!((zeta).abs() < 1e-10);
        assert!((z).abs() < 1e-10);
        assert!((theta).abs() < 1e-10);

        // Test for J2050.0
        let (zeta, z, theta) = precession_angles(2469807.5);
        assert!((zeta - 0.3203297).abs() < 0.0000001);
        assert!((z - 0.3203847).abs() < 0.0000001);
        assert!((theta - 0.2783454).abs() < 0.0000001);
    }

    #[test]
    fn test_precess_j2000_to_date() {
        // Test precession of Polaris from J2000 to J2050
        let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = precess_j2000_to_date(37.95456067, 89.26410897, dt);
        
        // Polaris should show significant RA change due to proximity to pole
        assert!(ra > 50.0 && ra < 60.0); // Based on actual calculation
        // Dec should remain very close to pole
        assert!((dec - 89.45).abs() < 0.01);
    }

    #[test]
    fn test_precession_roundtrip() {
        // Test that precessing forward and back gives original coordinates
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
        let ra_original = 83.633;
        let dec_original = 22.0145;

        let (ra_precessed, dec_precessed) = precess_j2000_to_date(ra_original, dec_original, dt);
        let (ra_back, dec_back) = precess_date_to_j2000(ra_precessed, dec_precessed, dt);

        assert!((ra_back - ra_original).abs() < 0.001); // Allow small error
        assert!((dec_back - dec_original).abs() < 0.001);
    }

    #[test]
    fn test_precess_vega() {
        // Test Vega's precession over 25 years
        let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = precess_j2000_to_date(279.23473479, 38.78368896, dt);
        
        // Vega should precess slightly over 25 years
        assert!((ra - 279.23473479).abs() < 0.5); // Small change in RA
        assert!((dec - 38.78368896).abs() < 0.05); // Small change in Dec
    }
}