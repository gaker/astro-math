//! Atmospheric refraction calculations.
//!
//! Corrects for the bending of light through Earth's atmosphere,
//! which makes objects appear higher than their true positions.

/// Calculates atmospheric refraction using Bennett's formula.
///
/// This formula is accurate for altitudes above 0 degrees.
///
/// # Arguments
/// * `altitude_deg` - Apparent altitude in degrees (must be >= -0.5)
///
/// # Returns
/// Refraction correction in degrees (always positive, subtract from apparent altitude)
///
/// # Example
/// ```
/// use astro_math::refraction_bennett;
/// 
/// // Object at 10 degrees apparent altitude
/// let refraction = refraction_bennett(10.0);
/// let true_altitude = 10.0 - refraction;
/// println!("Refraction: {:.3}°, True altitude: {:.3}°", refraction, true_altitude);
/// ```
pub fn refraction_bennett(altitude_deg: f64) -> f64 {
    if altitude_deg < -0.5 {
        return 0.0; // No refraction below horizon
    }
    
    // Bennett's formula in arcminutes
    let h = altitude_deg;
    let r_arcmin = 1.0 / ((h + 7.31 / (h + 4.4)).to_radians().tan());
    
    // Convert to degrees
    r_arcmin / 60.0
}

/// Calculates atmospheric refraction using Saemundsson's formula.
///
/// More accurate than Bennett for very low altitudes.
///
/// # Arguments
/// * `altitude_deg` - Apparent altitude in degrees
/// * `pressure_hpa` - Atmospheric pressure in hectopascals (default: 1013.25)
/// * `temperature_c` - Temperature in Celsius (default: 10.0)
///
/// # Returns
/// Refraction correction in degrees
pub fn refraction_saemundsson(altitude_deg: f64, pressure_hpa: f64, temperature_c: f64) -> f64 {
    if altitude_deg < -1.0 {
        return 0.0;
    }
    
    // Saemundsson's formula
    let h = altitude_deg;
    let r_arcmin = 1.02 / ((h + 10.3 / (h + 5.11)).to_radians().tan());
    
    // Pressure and temperature corrections
    let p_factor = pressure_hpa / 1010.0;
    let t_factor = 283.0 / (273.0 + temperature_c);
    
    r_arcmin * p_factor * t_factor / 60.0
}

/// Calculates atmospheric refraction for radio wavelengths.
///
/// Radio refraction is slightly different from optical due to atmospheric properties.
///
/// # Arguments
/// * `altitude_deg` - Apparent altitude in degrees
/// * `pressure_hpa` - Atmospheric pressure in hectopascals
/// * `temperature_c` - Temperature in Celsius
/// * `humidity_percent` - Relative humidity (0-100)
///
/// # Returns
/// Refraction correction in degrees for radio observations
pub fn refraction_radio(
    altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
    humidity_percent: f64,
) -> f64 {
    if altitude_deg < -1.0 {
        return 0.0;
    }
    
    // Calculate water vapor pressure
    let es = 6.105 * (17.27 * temperature_c / (237.7 + temperature_c)).exp();
    let e = humidity_percent / 100.0 * es;
    
    // Radio refractivity
    let n_dry = 77.6 * pressure_hpa / (273.15 + temperature_c);
    let n_wet = 3.73e5 * e / (273.15 + temperature_c).powi(2);
    let n = n_dry + n_wet;
    
    // Refraction in arcseconds
    let cot_h = 1.0 / altitude_deg.to_radians().tan();
    let r_arcsec = n * cot_h / 1e6 * 206265.0;
    
    r_arcsec / 3600.0
}

/// Converts apparent altitude to true altitude by removing refraction.
///
/// # Arguments
/// * `apparent_altitude_deg` - Observed altitude including refraction
/// * `pressure_hpa` - Atmospheric pressure (default: 1013.25)
/// * `temperature_c` - Temperature in Celsius (default: 10.0)
///
/// # Returns
/// True altitude in degrees
pub fn apparent_to_true_altitude(
    apparent_altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
) -> f64 {
    let refraction = refraction_saemundsson(apparent_altitude_deg, pressure_hpa, temperature_c);
    apparent_altitude_deg - refraction
}

/// Converts true altitude to apparent altitude by adding refraction.
///
/// Uses iteration since refraction depends on apparent altitude.
///
/// # Arguments
/// * `true_altitude_deg` - True altitude without refraction
/// * `pressure_hpa` - Atmospheric pressure (default: 1013.25)
/// * `temperature_c` - Temperature in Celsius (default: 10.0)
///
/// # Returns
/// Apparent altitude in degrees
pub fn true_to_apparent_altitude(
    true_altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
) -> f64 {
    // Initial guess
    let mut apparent = true_altitude_deg;
    
    // Iterate to convergence
    for _ in 0..5 {
        let refraction = refraction_saemundsson(apparent, pressure_hpa, temperature_c);
        apparent = true_altitude_deg + refraction;
    }
    
    apparent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refraction_bennett_horizon() {
        // At horizon, refraction should be about 34-35 arcminutes
        let r = refraction_bennett(0.0);
        assert!(r > 0.55 && r < 0.60); // 33-36 arcminutes
    }

    #[test]
    fn test_refraction_bennett_zenith() {
        // At zenith, refraction should be zero
        let r = refraction_bennett(90.0);
        assert!(r < 0.001);
    }

    #[test]
    fn test_refraction_bennett_45deg() {
        // At 45 degrees, refraction should be about 1 arcminute
        let r = refraction_bennett(45.0);
        assert!(r > 0.015 && r < 0.020); // About 1 arcminute
    }

    #[test]
    fn test_refraction_saemundsson_standard() {
        // Test with standard conditions
        let r = refraction_saemundsson(10.0, 1013.25, 10.0);
        assert!(r > 0.08 && r < 0.10); // About 5-6 arcminutes
    }

    #[test]
    fn test_refraction_pressure_effect() {
        // Higher pressure should increase refraction
        let r_low = refraction_saemundsson(10.0, 980.0, 10.0);
        let r_high = refraction_saemundsson(10.0, 1040.0, 10.0);
        assert!(r_high > r_low);
    }

    #[test]
    fn test_refraction_temperature_effect() {
        // Higher temperature should decrease refraction
        let r_cold = refraction_saemundsson(10.0, 1013.25, -10.0);
        let r_hot = refraction_saemundsson(10.0, 1013.25, 30.0);
        assert!(r_cold > r_hot);
    }

    #[test]
    fn test_altitude_conversion_roundtrip() {
        let true_alt = 15.0;
        let apparent = true_to_apparent_altitude(true_alt, 1013.25, 10.0);
        let back_to_true = apparent_to_true_altitude(apparent, 1013.25, 10.0);
        assert!((back_to_true - true_alt).abs() < 0.001);
    }

    #[test]
    fn test_radio_refraction() {
        // Radio refraction should be slightly different from optical
        let r_radio = refraction_radio(10.0, 1013.25, 20.0, 50.0);
        let r_optical = refraction_saemundsson(10.0, 1013.25, 20.0);
        
        // Both should be positive
        assert!(r_radio > 0.0);
        assert!(r_optical > 0.0);
        // Radio refraction is typically larger than optical
        assert!(r_radio > r_optical);
    }
}