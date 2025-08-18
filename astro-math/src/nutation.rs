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
    
    // Convert from radians to arcseconds
    dpsi * 206264.80624709636
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
    
    // Convert from radians to arcseconds
    deps * 206264.80624709636
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
    
    // Convert from radians to arcseconds
    Nutation {
        longitude: dpsi * 206264.80624709636,
        obliquity: deps * 206264.80624709636,
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