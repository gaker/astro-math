//! Precession calculations for converting coordinates between different epochs.
//!
//! This module handles the slow wobble of Earth's axis that causes celestial
//! coordinates to change over time. It provides functions to convert coordinates
//! between J2000.0 and any other epoch.
//!
//! # Background
//!
//! Due to gravitational forces from the Sun and Moon on Earth's equatorial bulge,
//! the celestial poles trace circles on the sky with a period of ~26,000 years.
//! This means star coordinates slowly change over time and must be adjusted to
//! match observations.
//!
//! # Implementation
//!
//! This module uses ERFA (Essential Routines for Fundamental Astronomy) for
//! maximum accuracy, implementing the IAU 2006 precession model.
//!
//! # Error Handling
//!
//! All functions validate their inputs and return `Result<T>` types:
//! - `AstroError::InvalidCoordinate` for out-of-range RA or Dec values
//!
//! # References
//!
//! - IAU 2006 precession model via ERFA
//! - Capitaine et al. (2003), "Expressions for IAU 2000 precession quantities"

use chrono::{DateTime, Utc};
use crate::error::{Result, validate_ra, validate_dec};

/// Calculates precession angles (ζ, z, θ) in degrees for converting from J2000.0 to a given date.
///
/// Uses the IAU 2006 precession model via ERFA for maximum accuracy.
/// Note: These angles include frame bias corrections.
///
/// # Arguments
/// * `jd` - Julian Date of the target epoch (TT)
///
/// # Returns
/// Tuple of (zeta, z, theta) in degrees
pub fn get_precession_angles(jd: f64) -> (f64, f64, f64) {
    // Use ERFA's IAU 2006 precession angles directly
    let (_eps0, _psia, _oma, _bpa, _bqa, _pia, _bpia, 
         _epsa, _chia, za, zetaa, thetaa, _pa, _gam, _phi, _psi) = 
        erfars::precnutpolar::P06e(jd, 0.0);
    
    // Convert from radians to degrees
    // zetaa, za, and thetaa are the precession angles we need
    (zetaa.to_degrees(), za.to_degrees(), thetaa.to_degrees())
}

/// Returns the IAU 2006 precession matrix from J2000.0 to the given date.
///
/// This matrix transforms mean J2000.0 coordinates to mean coordinates of date.
/// Uses ERFA's Pmat06 function which implements the IAU 2006 precession model.
///
/// # Arguments
/// * `jd` - Julian Date of the target epoch (TT)
///
/// # Returns
/// 3x3 precession matrix as a nested array
///
/// # Example
/// ```
/// use astro_math::get_precession_matrix;
/// 
/// let jd = 2451545.0; // J2000.0
/// let matrix = get_precession_matrix(jd);
/// // At J2000.0, matrix should be close to identity (with small frame bias)
/// ```
pub fn get_precession_matrix(jd: f64) -> [[f64; 3]; 3] {
    let mut rbp = [0.0; 9];
    erfars::precnutpolar::Pmat06(jd, 0.0, &mut rbp);
    
    // Convert flat array to 3x3 matrix
    [
        [rbp[0], rbp[1], rbp[2]],
        [rbp[3], rbp[4], rbp[5]],
        [rbp[6], rbp[7], rbp[8]],
    ]
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
/// # Errors
///
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `ra_j2000` is outside [0, 360)
/// - `dec_j2000` is outside [-90, 90]
///
/// # Example
/// ```
/// use chrono::{TimeZone, Utc};
/// use astro_math::precess_from_j2000;
///
/// let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let (ra, dec) = precess_from_j2000(0.0, 0.0, dt).unwrap();
/// println!("Precessed coordinates: RA={:.4}°, Dec={:.4}°", ra, dec);
/// ```
///
/// # Error Example
/// ```
/// # use chrono::Utc;
/// # use astro_math::{precess_from_j2000, error::AstroError};
/// // Invalid RA
/// match precess_from_j2000(400.0, 0.0, Utc::now()) {
///     Err(AstroError::InvalidCoordinate { coord_type, .. }) => {
///         assert_eq!(coord_type, "RA");
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn precess_from_j2000(ra_j2000: f64, dec_j2000: f64, datetime: DateTime<Utc>) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra_j2000)?;
    validate_dec(dec_j2000)?;
    let jd = crate::julian_date(datetime);
    
    // Use ERFA for accurate precession
    let ra_rad = ra_j2000.to_radians();
    let dec_rad = dec_j2000.to_radians();
    
    // Get precession matrix from J2000 to date
    let mut rbp = [0.0; 9];
    erfars::precnutpolar::Pmat06(jd, 0.0, &mut rbp);
    
    // Convert spherical to Cartesian
    let cos_ra = ra_rad.cos();
    let sin_ra = ra_rad.sin();
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    
    let p = [
        cos_dec * cos_ra,
        cos_dec * sin_ra,
        sin_dec,
    ];
    
    // Apply precession matrix
    let p_new = [
        rbp[0] * p[0] + rbp[1] * p[1] + rbp[2] * p[2],
        rbp[3] * p[0] + rbp[4] * p[1] + rbp[5] * p[2],
        rbp[6] * p[0] + rbp[7] * p[1] + rbp[8] * p[2],
    ];
    
    // Convert back to spherical
    let ra_new = p_new[1].atan2(p_new[0]);
    let dec_new = p_new[2].asin();
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra_new.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    } else if ra_deg >= 360.0 {
        ra_deg -= 360.0;
    }
    
    Ok((ra_deg, dec_new.to_degrees()))
}

/// Applies precession from a given date back to J2000.0.
///
/// This is the inverse of [`precess_from_j2000`] and is useful for converting
/// current epoch coordinates to the standard J2000.0 reference frame.
///
/// # Arguments
/// * `ra` - Right ascension at the given date in degrees
/// * `dec` - Declination at the given date in degrees
/// * `datetime` - Current date/time
///
/// # Returns
/// Tuple of (ra, dec) at J2000.0 in degrees
///
/// # Errors
///
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `ra` is outside [0, 360)
/// - `dec` is outside [-90, 90]
///
/// # Example
/// ```
/// # use chrono::{TimeZone, Utc};
/// # use astro_math::precess_to_j2000;
/// let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// // Convert current epoch coordinates back to J2000.0
/// let (ra_j2000, dec_j2000) = precess_to_j2000(10.0, 20.0, dt).unwrap();
/// ```
pub fn precess_to_j2000(ra: f64, dec: f64, datetime: DateTime<Utc>) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra)?;
    validate_dec(dec)?;
    let jd = crate::julian_date(datetime);
    
    // Use ERFA for accurate precession
    let ra_rad = ra.to_radians();
    let dec_rad = dec.to_radians();
    
    // Get precession matrix from J2000 to date
    let mut rbp = [0.0; 9];
    erfars::precnutpolar::Pmat06(jd, 0.0, &mut rbp);
    
    // For inverse, we need the transpose of the matrix
    let rbp_t = [
        rbp[0], rbp[3], rbp[6],
        rbp[1], rbp[4], rbp[7],
        rbp[2], rbp[5], rbp[8],
    ];
    
    // Convert spherical to Cartesian
    let cos_ra = ra_rad.cos();
    let sin_ra = ra_rad.sin();
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    
    let p = [
        cos_dec * cos_ra,
        cos_dec * sin_ra,
        sin_dec,
    ];
    
    // Apply inverse precession matrix (transpose)
    let p_j2000 = [
        rbp_t[0] * p[0] + rbp_t[1] * p[1] + rbp_t[2] * p[2],
        rbp_t[3] * p[0] + rbp_t[4] * p[1] + rbp_t[5] * p[2],
        rbp_t[6] * p[0] + rbp_t[7] * p[1] + rbp_t[8] * p[2],
    ];
    
    // Convert back to spherical
    let ra_j2000 = p_j2000[1].atan2(p_j2000[0]);
    let dec_j2000 = p_j2000[2].asin();
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra_j2000.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    } else if ra_deg >= 360.0 {
        ra_deg -= 360.0;
    }
    
    Ok((ra_deg, dec_j2000.to_degrees()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_get_precession_angles() {
        // Test for J2000.0 - ERFA's P06e returns angles that include frame bias
        let (zeta, z, theta) = get_precession_angles(2451545.0);
        // At J2000.0, angles include small frame bias corrections
        assert!((zeta - 0.0007362625).abs() < 1e-9, "zeta at J2000.0: {}", zeta);
        assert!((z - (-0.0007362625)).abs() < 1e-9, "z at J2000.0: {}", z);  
        assert!(theta.abs() < 1e-10, "theta at J2000.0: {}", theta);

        // Test for J2050.0 using ERFA's exact values
        let (zeta, z, theta) = get_precession_angles(2469807.5);
        // These are the correct IAU 2006 precession angles from ERFA
        assert!((zeta - 0.3210).abs() < 0.0001, "zeta: {}", zeta);
        assert!((z - 0.3196).abs() < 0.0001, "z: {}", z);
        assert!((theta - 0.2783).abs() < 0.0001, "theta: {}", theta);
    }
    
    #[test]
    fn test_get_precession_matrix() {
        // Test that precession matrix at J2000.0 includes frame bias
        let matrix = get_precession_matrix(2451545.0);
        
        // Diagonal elements should be very close to 1
        assert!((matrix[0][0] - 1.0).abs() < 1e-7);
        assert!((matrix[1][1] - 1.0).abs() < 1e-7);
        assert!((matrix[2][2] - 1.0).abs() < 1e-7);
        
        // Off-diagonal elements represent frame bias (tiny)
        assert!(matrix[0][1].abs() < 1e-7);
        assert!(matrix[0][2].abs() < 1e-7);
        
        // Test matrix is orthogonal (determinant = 1)
        let det = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
                - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
                + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);
        assert!((det - 1.0).abs() < 1e-10, "Determinant should be 1, got {}", det);
    }

    #[test]
    fn test_precess_from_j2000() {
        // Test precession of Polaris from J2000 to J2050
        let dt = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = precess_from_j2000(37.95456067, 89.26410897, dt).unwrap();
        
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

        let (ra_precessed, dec_precessed) = precess_from_j2000(ra_original, dec_original, dt).unwrap();
        let (ra_back, dec_back) = precess_to_j2000(ra_precessed, dec_precessed, dt).unwrap();

        assert!((ra_back - ra_original).abs() < 0.001); // Allow small error
        assert!((dec_back - dec_original).abs() < 0.001);
    }

    #[test]
    fn test_precess_vega() {
        // Test Vega's precession over 25 years
        let dt = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = precess_from_j2000(279.23473479, 38.78368896, dt).unwrap();
        
        // Vega should precess slightly over 25 years
        assert!((ra - 279.23473479).abs() < 0.5); // Small change in RA
        assert!((dec - 38.78368896).abs() < 0.05); // Small change in Dec
    }
}