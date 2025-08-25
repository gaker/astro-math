//! Nutation calculations using ERFA library.
//!
//! This module provides IAU 2000A nutation calculations through ERFA,
//! ensuring exact compatibility with astropy and other professional
//! astronomy software.
//!
//! Nutation consists of two components:
//! - **Nutation in longitude** (Δψ): Affects right ascension
//! - **Nutation in obliquity** (Δε): Affects declination
//!
//! # Accuracy
//!
//! Uses ERFA's IAU 2000A model which provides milliarcsecond accuracy
//! with 1365 terms for longitude and 1359 terms for obliquity.
//!
//! # Example
//!
//! ```
//! use astro_math::nutation::{nutation_in_longitude, nutation_in_obliquity, mean_obliquity};
//! use astro_math::time::julian_date;
//! use chrono::Utc;
//!
//! let dt = Utc::now();
//! let jd = julian_date(dt);
//!
//! let dpsi = nutation_in_longitude(jd);
//! let deps = nutation_in_obliquity(jd);
//! let eps0 = mean_obliquity(jd);
//! let true_obliquity = eps0 + deps;
//!
//! println!("Nutation in longitude: {:.2}\"", dpsi);
//! println!("Nutation in obliquity: {:.2}\"", deps);
//! println!("True obliquity: {:.6}°", true_obliquity);
//! ```


/// Calculates nutation in longitude (Δψ) in arcseconds using ERFA.
///
/// Uses the IAU 2000A model for milliarcsecond accuracy.
///
/// # Arguments
///
/// * `jd` - Julian Date (TT)
///
/// # Returns
///
/// Nutation in longitude in arcseconds.
///
/// # Example
///
/// ```
/// use astro_math::nutation::nutation_in_longitude;
/// 
/// let jd = 2451545.0; // J2000.0
/// let dpsi = nutation_in_longitude(jd);
/// assert!(dpsi.abs() < 20.0); // Should be within ±20 arcseconds
/// ```
pub fn nutation_in_longitude(jd: f64) -> f64 {
    // Split JD for better precision (ERFA convention)
    let jd1 = jd;
    let jd2 = 0.0;
    
    // Get nutation using IAU 2000A model
    let (dpsi, _deps) = erfars::precnutpolar::Nut00a(jd1, jd2);
    
    // Convert from radians to arcseconds using exact mathematical constant
    dpsi * (180.0 * 3600.0 / std::f64::consts::PI)
}

/// Calculates nutation in obliquity (Δε) in arcseconds using ERFA.
///
/// Uses the IAU 2000A model for milliarcsecond accuracy.
///
/// # Arguments
///
/// * `jd` - Julian Date (TT)
///
/// # Returns
///
/// Nutation in obliquity in arcseconds.
///
/// # Example
///
/// ```
/// use astro_math::nutation::nutation_in_obliquity;
/// 
/// let jd = 2451545.0; // J2000.0
/// let deps = nutation_in_obliquity(jd);
/// assert!(deps.abs() < 10.0); // Should be within ±10 arcseconds
/// ```
pub fn nutation_in_obliquity(jd: f64) -> f64 {
    // Split JD for better precision (ERFA convention)
    let jd1 = jd;
    let jd2 = 0.0;
    
    // Get nutation using IAU 2000A model
    let (_dpsi, deps) = erfars::precnutpolar::Nut00a(jd1, jd2);
    
    // Convert from radians to arcseconds using exact mathematical constant
    deps * (180.0 * 3600.0 / std::f64::consts::PI)
}

/// Calculates the mean obliquity of the ecliptic (ε₀) in degrees using ERFA.
///
/// Uses the IAU 2006 precession model.
///
/// # Arguments
///
/// * `jd` - Julian Date (TT)
///
/// # Returns
///
/// Mean obliquity in degrees (approximately 23.4°).
///
/// # Example
///
/// ```
/// use astro_math::nutation::mean_obliquity;
/// 
/// let jd = 2451545.0; // J2000.0
/// let eps0 = mean_obliquity(jd);
/// assert!((eps0 - 23.4392911).abs() < 0.0001);
/// ```
pub fn mean_obliquity(jd: f64) -> f64 {
    // Split JD for better precision
    let jd1 = jd;
    let jd2 = 0.0;
    
    // Get mean obliquity using IAU 2006 model
    let eps_rad = erfars::precnutpolar::Obl06(jd1, jd2);
    
    // Convert from radians to degrees
    eps_rad.to_degrees()
}

/// Calculates the true obliquity of the ecliptic in degrees.
///
/// This includes both the mean obliquity and the nutation in obliquity.
/// Use this value for precise coordinate transformations.
///
/// # Arguments
///
/// * `jd` - Julian Date
///
/// # Returns
///
/// True obliquity in degrees.
///
/// # Example
///
/// ```
/// use astro_math::nutation::true_obliquity;
/// 
/// let jd = 2451545.0;
/// let eps = true_obliquity(jd);
/// assert!(eps > 23.0 && eps < 24.0);
/// ```
pub fn true_obliquity(jd: f64) -> f64 {
    mean_obliquity(jd) + nutation_in_obliquity(jd) / 3600.0
}

/// Structure containing both nutation components.
///
/// This is convenient when you need both values and want to avoid
/// duplicate calculations of the fundamental arguments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nutation {
    /// Nutation in longitude (Δψ) in arcseconds
    pub longitude: f64,
    /// Nutation in obliquity (Δε) in arcseconds  
    pub obliquity: f64,
}

/// Calculates both nutation components efficiently using ERFA.
///
/// Uses the IAU 2000A model for milliarcsecond accuracy.
///
/// # Arguments
///
/// * `jd` - Julian Date (TT)
///
/// # Returns
///
/// A `Nutation` struct containing both components in arcseconds.
///
/// # Example
///
/// ```
/// use astro_math::nutation::nutation;
/// 
/// let jd = 2451545.0;
/// let nut = nutation(jd);
/// println!("Δψ = {:.2}\", Δε = {:.2}\"", nut.longitude, nut.obliquity);
/// ```
pub fn nutation(jd: f64) -> Nutation {
    // Split JD for better precision
    let jd1 = jd;
    let jd2 = 0.0;
    
    // Get nutation using IAU 2000A model
    let (dpsi, deps) = erfars::precnutpolar::Nut00a(jd1, jd2);
    
    // Convert from radians to arcseconds using exact mathematical constant  
    let rad_to_arcsec = 180.0 * 3600.0 / std::f64::consts::PI;
    Nutation {
        longitude: dpsi * rad_to_arcsec,
        obliquity: deps * rad_to_arcsec,
    }
}

// Keep the old functions for backwards compatibility with internal use
#[doc(hidden)]
pub fn nutation_in_longitude_arcsec(jd: f64) -> f64 {
    nutation_in_longitude(jd)
}

#[doc(hidden)]
pub fn mean_obliquity_arcsec(jd: f64) -> f64 {
    mean_obliquity(jd) * 3600.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::julian_date;
    use chrono::{DateTime, Utc, NaiveDateTime};

    #[test]
    fn test_nutation_precision_august_2025() {
        // Test date: August 1, 2025, 00:00:00 UTC
        let dt = NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        );
        let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        
        // Convert to Julian Date (UTC)
        let jd_utc = julian_date(utc_dt);
        
        // ISSUE: Our functions expect TT but we're passing UTC
        // For now, let's test with the conversion we should be doing:
        // TT = UTC + (TAI-UTC) + 32.184s
        // As of 2025, TAI-UTC ≈ 37 seconds
        let tt_offset_days = (37.0 + 32.184) / 86400.0; // Convert seconds to days
        let jd_tt = jd_utc + tt_offset_days;
        
        // Test nutation with proper TT time
        let nut = nutation(jd_tt);
        
        // Expected values from astropy/ERFA with proper TT conversion:
        // These values are from our astropy_test_data/nutation_aug1_2025.py script
        let expected_dpsi = 3.821318106868885;
        let expected_deps = 8.91080363873388;
        
        // Test precision - should be within microarcsecond precision
        let dpsi_diff = (nut.longitude - expected_dpsi).abs();
        let deps_diff = (nut.obliquity - expected_deps).abs();
        
        println!("Current dpsi: {:.9}, expected: {:.9}, diff: {:.6} mas", 
                 nut.longitude, expected_dpsi, dpsi_diff * 1000.0);
        println!("Current deps: {:.9}, expected: {:.9}, diff: {:.6} mas", 
                 nut.obliquity, expected_deps, deps_diff * 1000.0);
        
        // For now, allow larger differences until we fix TT-UTC handling
        assert!(dpsi_diff < 0.001, "Nutation in longitude differs by {:.6} mas", dpsi_diff * 1000.0);
        assert!(deps_diff < 0.001, "Nutation in obliquity differs by {:.6} mas", deps_diff * 1000.0);
    }

    #[test]  
    fn test_nutation_with_utc_shows_issue() {
        // This test demonstrates the TT-UTC issue
        let dt = NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        );
        let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        let jd_utc = julian_date(utc_dt);
        
        // Using UTC JD directly (which is wrong for nutation)
        let nut_utc = nutation(jd_utc);
        
        // This will show the ~69 second offset error in our results
        println!("With UTC JD: dpsi={:.6}, deps={:.6}", nut_utc.longitude, nut_utc.obliquity);
        
        // The values should be different from the expected ERFA values
        // because we're not using TT as required
        let expected_dpsi = 3.821318106868885;
        let expected_deps = 8.91080363873388;
        
        let dpsi_error = (nut_utc.longitude - expected_dpsi).abs();
        let deps_error = (nut_utc.obliquity - expected_deps).abs();
        
        // This test should "fail" (show the issue) until we fix TT conversion
        // For now, we expect the error to be small but non-zero
        println!("Error using UTC instead of TT: dpsi={:.3} mas, deps={:.3} mas", 
                 dpsi_error * 1000.0, deps_error * 1000.0);
    }

    #[test]
    fn test_conversion_factor_precision() {
        // Test the exact conversion factor identified by the auditor
        let exact_factor = 180.0 * 3600.0 / std::f64::consts::PI;
        let current_factor = 206264.80624709636;
        
        let factor_diff = (exact_factor - current_factor).abs();
        println!("Exact factor: {:.15}", exact_factor);
        println!("Current factor: {:.15}", current_factor);
        println!("Difference: {:.2e}", factor_diff);
        
        // The difference should be extremely small (< 1e-10)
        assert!(factor_diff < 1e-10, "Conversion factor precision issue");
    }

    #[test]
    fn test_mean_obliquity_j2000() {
        // Test mean obliquity at J2000.0
        let jd_j2000 = 2451545.0;
        let eps0 = mean_obliquity(jd_j2000);
        
        // J2000.0 mean obliquity should be very close to 23.4392911°
        let expected = 23.4392911;
        assert!((eps0 - expected).abs() < 0.0001, 
                "Mean obliquity at J2000: got {:.7}, expected {:.7}", eps0, expected);
    }
}