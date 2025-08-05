//! Airmass calculations for astronomical observations.
//!
//! Airmass represents the optical path length through Earth's atmosphere
//! relative to the zenith direction.

/// Calculates airmass using the plane-parallel atmosphere approximation.
///
/// Valid for zenith angles less than ~60-70 degrees.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees (must be positive)
///
/// # Returns
/// Airmass value (1.0 at zenith, increasing towards horizon)
pub fn airmass_plane_parallel(altitude_deg: f64) -> f64 {
    if altitude_deg <= 0.0 {
        return f64::INFINITY;
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    1.0 / zenith_angle.to_radians().cos()
}

/// Calculates airmass using Young's formula (1994).
///
/// More accurate than plane-parallel, especially at low altitudes.
/// Valid down to the horizon.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees
///
/// # Returns
/// Airmass value
pub fn airmass_young(altitude_deg: f64) -> f64 {
    if altitude_deg <= -0.5 {
        return f64::INFINITY;
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    let z_rad = zenith_angle.to_radians();
    
    // Young's formula
    let cos_z = z_rad.cos();
    1.0 / (cos_z + 0.50572 * (96.07995 - zenith_angle).powf(-1.6364))
}

/// Calculates airmass using Pickering's formula (2002).
///
/// Very accurate down to the horizon, accounting for atmospheric refraction.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees (apparent altitude)
///
/// # Returns
/// Airmass value
pub fn airmass_pickering(altitude_deg: f64) -> f64 {
    if altitude_deg <= -0.5 {
        return f64::INFINITY;
    }
    
    let h = altitude_deg.max(0.0); // Clamp to non-negative
    1.0 / (h + 244.0 / (165.0 + 47.0 * h.powf(1.1))).to_radians().sin()
}

/// Calculates airmass using Kasten & Young's formula (1989).
///
/// Standard formula used in many applications.
///
/// # Arguments
/// * `altitude_deg` - Altitude in degrees
///
/// # Returns
/// Airmass value
pub fn airmass_kasten_young(altitude_deg: f64) -> f64 {
    if altitude_deg <= 0.0 {
        return f64::INFINITY;
    }
    
    let zenith_angle = 90.0 - altitude_deg;
    let z_rad = zenith_angle.to_radians();
    
    1.0 / (z_rad.cos() + 0.50572 * (96.07995 - zenith_angle).powf(-1.6364))
}

/// Calculates the extinction in magnitudes for a given airmass.
///
/// # Arguments
/// * `airmass` - Airmass value
/// * `extinction_coefficient` - Extinction coefficient in magnitudes per airmass
///   (typical values: 0.1-0.3 for good sites)
///
/// # Returns
/// Extinction in magnitudes
pub fn extinction_magnitudes(airmass: f64, extinction_coefficient: f64) -> f64 {
    airmass * extinction_coefficient
}

/// Estimates the extinction coefficient based on wavelength.
///
/// Very approximate model for clear conditions.
///
/// # Arguments
/// * `wavelength_nm` - Wavelength in nanometers
///
/// # Returns
/// Approximate extinction coefficient in magnitudes per airmass
pub fn extinction_coefficient_estimate(wavelength_nm: f64) -> f64 {
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
    
    rayleigh + aerosol + ozone
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airmass_zenith() {
        // At zenith (90° altitude), airmass should be 1.0
        assert!((airmass_plane_parallel(90.0) - 1.0).abs() < 1e-6);
        assert!((airmass_young(90.0) - 1.0).abs() < 0.001);
        assert!((airmass_pickering(90.0) - 1.0).abs() < 0.01);
        assert!((airmass_kasten_young(90.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_airmass_45deg() {
        // At 45° altitude, plane-parallel gives sec(45°) = sqrt(2)
        let expected = 2.0_f64.sqrt();
        assert!((airmass_plane_parallel(45.0) - expected).abs() < 1e-6);
        
        // Other formulas should give similar values
        assert!((airmass_young(45.0) - expected).abs() < 0.01);
        assert!((airmass_pickering(45.0) - expected).abs() < 0.01);
    }

    #[test]
    fn test_airmass_horizon() {
        // Near horizon, airmass should be large
        let am_young = airmass_young(0.0);
        let am_pickering = airmass_pickering(0.0);
        
        assert!(am_young > 30.0 && am_young < 50.0);
        assert!(am_pickering > 30.0 && am_pickering < 50.0);
    }

    #[test]
    fn test_airmass_below_horizon() {
        // Below horizon should return infinity
        assert!(airmass_plane_parallel(-5.0).is_infinite());
        assert!(airmass_young(-5.0).is_infinite());
        assert!(airmass_pickering(-5.0).is_infinite());
        assert!(airmass_kasten_young(-5.0).is_infinite());
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
        let k_blue = extinction_coefficient_estimate(450.0);
        let k_red = extinction_coefficient_estimate(650.0);
        assert!(k_blue > k_red);
        
        // Values should be reasonable
        assert!(k_blue > 0.15 && k_blue < 0.5);
        assert!(k_red > 0.05 && k_red < 0.3);
    }
}