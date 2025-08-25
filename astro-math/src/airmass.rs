//! Airmass calculations for astronomical observations.
//!
//! Airmass quantifies how much atmosphere light must pass through to reach
//! an observer. It's crucial for understanding atmospheric extinction and
//! planning observations.
//!
//! # Overview
//!
//! - Airmass = 1.0 at zenith (directly overhead)
//! - Increases with zenith angle (lower altitude)
//! - Approaches infinity at the horizon
//! - Different formulas provide varying accuracy at low altitudes
//!
//! # Applications
//!
//! - Calculating atmospheric extinction
//! - Planning photometric observations
//! - Correcting for differential atmospheric effects
//!
//! # Error Handling
//!
//! All functions validate altitude inputs and return `Result<T>` types:
//! - `AstroError::OutOfRange` for altitudes outside [-90, 90] degrees

use crate::error::{Result, AstroError};

/// Calculates airmass using the plane-parallel atmosphere approximation.
///
/// Simplest model assuming flat, parallel atmospheric layers. Accurate for
/// high altitudes but underestimates airmass near the horizon.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees
///
/// # Returns
/// Airmass value (1.0 at zenith, increasing towards horizon)
///
/// # Errors
/// Returns `Err(AstroError::OutOfRange)` if altitude is outside [-90, 90] degrees.
///
/// # Notes
/// - Returns infinity for altitude ≤ 0°
/// - Best for altitude > 30°
/// - Formula: X = sec(z) where z is zenith angle
pub fn airmass_plane_parallel(altitude_deg: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&altitude_deg) {
        return Err(AstroError::OutOfRange {
            parameter: "altitude",
            value: altitude_deg,
            min: -90.0,
            max: 90.0,
        });
    }
    
    if altitude_deg <= 0.0 {
        return Ok(f64::INFINITY);
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    Ok(1.0 / zenith_angle.to_radians().cos())
}

/// Calculates airmass using Young's formula (1994).
///
/// Improved model that accounts for Earth's curvature and atmospheric
/// refraction. More accurate than plane-parallel at low altitudes.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees
///
/// # Returns
/// Airmass value
///
/// # Errors
/// Returns `Err(AstroError::OutOfRange)` if altitude is outside [-90, 90] degrees.
///
/// # Example
/// ```
/// # use astro_math::airmass_young;
/// // Airmass at 30° altitude
/// let airmass = airmass_young(30.0).unwrap();
/// assert!((airmass - 2.0).abs() < 0.1);
/// ```
pub fn airmass_young(altitude_deg: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&altitude_deg) {
        return Err(AstroError::OutOfRange {
            parameter: "altitude",
            value: altitude_deg,
            min: -90.0,
            max: 90.0,
        });
    }
    
    if altitude_deg <= -0.5 {
        return Ok(f64::INFINITY);
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    let z_rad = zenith_angle.to_radians();
    
    // Young's formula
    let cos_z = z_rad.cos();
    Ok(1.0 / (cos_z + 0.50572 * (96.07995 - zenith_angle).powf(-1.6364)))
}

/// Calculates airmass using Pickering's formula (2002).
///
/// Most accurate formula, especially near the horizon. Properly accounts
/// for atmospheric refraction and Earth's curvature.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees (apparent altitude)
///
/// # Returns
/// Airmass value
///
/// # Errors
/// Returns `Err(AstroError::OutOfRange)` if altitude is outside [-90, 90] degrees.
///
/// # Recommendation
/// Use this formula for the most accurate results, especially for
/// altitude < 15° where other formulas become less reliable.
pub fn airmass_pickering(altitude_deg: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&altitude_deg) {
        return Err(AstroError::OutOfRange {
            parameter: "altitude",
            value: altitude_deg,
            min: -90.0,
            max: 90.0,
        });
    }
    
    if altitude_deg <= -0.5 {
        return Ok(f64::INFINITY);
    }
    
    let h = altitude_deg.max(0.0); // Clamp to non-negative
    Ok(1.0 / (h + 244.0 / (165.0 + 47.0 * h.powf(1.1))).to_radians().sin())
}

/// Calculates airmass using Kasten & Young's formula (1989).
///
/// Standard formula widely used in astronomy. Good balance between
/// accuracy and simplicity.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees
///
/// # Returns
/// Airmass value
///
/// # Errors
/// Returns `Err(AstroError::OutOfRange)` if altitude is outside [-90, 90] degrees.
///
pub fn airmass_kasten_young(altitude_deg: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&altitude_deg) {
        return Err(AstroError::OutOfRange {
            parameter: "altitude",
            value: altitude_deg,
            min: -90.0,
            max: 90.0,
        });
    }
    
    if altitude_deg <= 0.0 {
        return Ok(f64::INFINITY);
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    let z_rad = zenith_angle.to_radians();
    
    Ok(1.0 / (z_rad.cos() + 0.50572 * (96.07995 - zenith_angle).powf(-1.6364)))
}

/// Calculates the extinction in magnitudes for a given airmass.
///
/// Extinction reduces the apparent brightness of celestial objects due to
/// atmospheric absorption and scattering.
///
/// # Arguments
/// * `airmass` - Airmass value
/// * `extinction_coefficient` - Extinction coefficient in magnitudes per airmass
///   - Excellent site: 0.05-0.10 mag/airmass
///   - Good site: 0.10-0.20 mag/airmass
///   - Average site: 0.20-0.30 mag/airmass
///   - Poor site: > 0.30 mag/airmass
///
/// # Returns
/// Extinction in magnitudes
///
/// # Example
/// ```
/// # use astro_math::extinction_magnitudes;
/// // Object at airmass 2.0 with typical extinction
/// let extinction = extinction_magnitudes(2.0, 0.15);
/// assert_eq!(extinction, 0.30); // 0.3 magnitudes dimmer
/// ```
pub fn extinction_magnitudes(airmass: f64, extinction_coefficient: f64) -> f64 {
    airmass * extinction_coefficient
}

/// Estimates the extinction coefficient based on wavelength.
///
/// Provides a rough estimate for clear atmospheric conditions. Real extinction
/// varies significantly with atmospheric conditions, altitude, and location.
///
/// # Arguments
/// * `wavelength_nm` - Wavelength in nanometers
///
/// # Returns
/// Approximate extinction coefficient in magnitudes per airmass
///
/// # Errors
/// Returns `Err(AstroError::OutOfRange)` if wavelength is not positive.
///
/// # Components
/// - Rayleigh scattering: λ⁻⁴ dependence (blue light scattered more)
/// - Aerosol scattering: λ⁻¹·³ dependence
/// - Ozone absorption: Peak around 600nm (Chappuis band)
///
/// # Example
/// ```
/// # use astro_math::extinction_coefficient_estimate;
/// // Blue light has higher extinction
/// let k_blue = extinction_coefficient_estimate(450.0).unwrap();
/// let k_red = extinction_coefficient_estimate(650.0).unwrap();
/// assert!(k_blue > k_red);
/// ```
pub fn extinction_coefficient_estimate(wavelength_nm: f64) -> Result<f64> {
    if wavelength_nm <= 0.0 {
        return Err(AstroError::OutOfRange {
            parameter: "wavelength_nm",
            value: wavelength_nm,
            min: f64::MIN_POSITIVE,
            max: f64::MAX,
        });
    }
    // Rayleigh scattering component (λ^-4)
    let rayleigh = 0.145 * (550.0 / wavelength_nm).powf(4.0);
    
    // Aerosol component (λ^-1.3)
    let aerosol = 0.10 * (550.0 / wavelength_nm).powf(1.3);
    
    // Ozone absorption (simplified)
    let ozone = if wavelength_nm > 500.0 && wavelength_nm < 700.0 {
        0.016
    } else {
        0.0
    };
    
    Ok(rayleigh + aerosol + ozone)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airmass_zenith() {
        // At zenith (90° altitude), airmass should be 1.0
        assert!((airmass_plane_parallel(90.0).unwrap() - 1.0).abs() < 1e-6);
        assert!((airmass_young(90.0).unwrap() - 1.0).abs() < 0.001);
        assert!((airmass_pickering(90.0).unwrap() - 1.0).abs() < 0.01);
        assert!((airmass_kasten_young(90.0).unwrap() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_airmass_45deg() {
        // At 45° altitude, plane-parallel gives sec(45°) = sqrt(2)
        let expected = 2.0_f64.sqrt();
        assert!((airmass_plane_parallel(45.0).unwrap() - expected).abs() < 1e-6);
        
        // Other formulas should give similar values
        assert!((airmass_young(45.0).unwrap() - expected).abs() < 0.01);
        assert!((airmass_pickering(45.0).unwrap() - expected).abs() < 0.01);
    }

    #[test]
    fn test_airmass_horizon() {
        // Near horizon, airmass should be large
        let am_young = airmass_young(0.0).unwrap();
        let am_pickering = airmass_pickering(0.0).unwrap();
        
        assert!(am_young > 30.0 && am_young < 50.0);
        assert!(am_pickering > 30.0 && am_pickering < 50.0);
    }

    #[test]
    fn test_airmass_below_horizon() {
        // Below horizon should return infinity
        assert!(airmass_plane_parallel(-5.0).unwrap().is_infinite());
        assert!(airmass_young(-5.0).unwrap().is_infinite());
        assert!(airmass_pickering(-5.0).unwrap().is_infinite());
        assert!(airmass_kasten_young(-5.0).unwrap().is_infinite());
    }

    #[test]
    fn test_extinction() {
        // Test extinction calculation
        let airmass = 2.0;
        let k = 0.15; // typical extinction coefficient
        let extinction = extinction_magnitudes(airmass, k);
        assert!((extinction - 0.3).abs() < 1e-6);
    }

    #[test]
    fn test_extinction_coefficient() {
        // Blue light should have higher extinction than red
        let k_blue = extinction_coefficient_estimate(450.0).unwrap();
        let k_red = extinction_coefficient_estimate(650.0).unwrap();
        assert!(k_blue > k_red);
        
        // Values should be reasonable
        assert!(k_blue > 0.15 && k_blue < 0.5);
        assert!(k_red > 0.05 && k_red < 0.3);
    }
}