//! Galactic coordinate system conversions.
//!
//! This module provides transformations between equatorial (ICRS) and galactic (l, b) 
//! coordinate systems using ERFA's IAU standard transformation functions.
//!
//! # Coordinate Systems
//!
//! - **Equatorial (J2000.0)**:
//!   - Right Ascension (RA): 0° to 360°
//!   - Declination (Dec): -90° to +90°
//!
//! - **Galactic**:
//!   - Longitude (l): 0° to 360° (measured from galactic center)
//!   - Latitude (b): -90° to +90° (measured from galactic plane)
//!
//! # Error Handling
//!
//! All functions validate their inputs and return `Result<T>` types:
//! - `AstroError::InvalidCoordinate` for out-of-range values
//!
//! # References
//!
//! - IAU standard galactic coordinate system definition
//! - ERFA (Essential Routines for Fundamental Astronomy) library

use crate::error::{Result, validate_ra, validate_dec};

/// Converts equatorial coordinates to galactic coordinates.
///
/// # Arguments
/// * `ra` - Right ascension in degrees (J2000.0)
/// * `dec` - Declination in degrees (J2000.0)
///
/// # Returns
/// Tuple of (l, b) in degrees where:
/// * l = Galactic longitude (0-360°)
/// * b = Galactic latitude (-90° to +90°)
///
/// # Errors
/// 
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `ra` is outside [0, 360)
/// - `dec` is outside [-90, 90]
///
/// # Example
/// ```
/// use astro_math::equatorial_to_galactic;
///
/// // Galactic center should be at l=0, b=0
/// let (l, b) = equatorial_to_galactic(266.405, -28.936).unwrap();
/// assert!((l - 0.0).abs() < 0.1);
/// assert!((b - 0.0).abs() < 0.1);
/// ```
///
/// # Error Example
/// ```
/// # use astro_math::{equatorial_to_galactic, error::AstroError};
/// // Invalid declination
/// match equatorial_to_galactic(180.0, 95.0) {
///     Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
///         assert_eq!(coord_type, "Declination");
///         assert_eq!(value, 95.0);
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn equatorial_to_galactic(ra: f64, dec: f64) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra)?;
    validate_dec(dec)?;
    
    // Convert to radians for ERFA
    let ra_rad = ra.to_radians();
    let dec_rad = dec.to_radians();
    
    // Use ERFA's ICRS to Galactic transformation
    // This implements the IAU standard galactic coordinate system
    let (l_rad, b_rad) = erfars::galacticcoordinates::Icrs2g(ra_rad, dec_rad);
    
    // Convert to degrees and normalize longitude
    let mut l_deg = l_rad.to_degrees();
    let b_deg = b_rad.to_degrees();
    
    // Normalize longitude to [0, 360)
    if l_deg < 0.0 {
        l_deg += 360.0;
    } else if l_deg >= 360.0 {
        l_deg -= 360.0;
    }
    
    Ok((l_deg, b_deg))
}

/// Converts galactic coordinates to equatorial coordinates.
///
/// # Arguments
/// * `l` - Galactic longitude in degrees (any value, will be normalized to [0, 360))
/// * `b` - Galactic latitude in degrees
///
/// # Returns
/// Tuple of (ra, dec) in degrees (J2000.0)
///
/// # Errors
///
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `b` is outside [-90, 90]
///
/// Note: Galactic longitude `l` is automatically normalized to [0, 360) and does not produce errors.
///
/// # Example
/// ```
/// use astro_math::galactic_to_equatorial;
///
/// // Convert galactic center back to equatorial
/// let (ra, dec) = galactic_to_equatorial(0.0, 0.0).unwrap();
/// assert!((ra - 266.405).abs() < 0.1);
/// assert!((dec - (-28.936)).abs() < 0.1);
/// ```
///
/// # Error Example
/// ```
/// # use astro_math::{galactic_to_equatorial, error::AstroError};
/// // Invalid galactic latitude
/// match galactic_to_equatorial(180.0, 100.0) {
///     Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
///         assert_eq!(coord_type, "Galactic latitude");
///         assert_eq!(value, 100.0);
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn galactic_to_equatorial(l: f64, b: f64) -> Result<(f64, f64)> {
    // Validate galactic latitude
    if b < -90.0 || b > 90.0 {
        return Err(crate::error::AstroError::InvalidCoordinate {
            coord_type: "Galactic latitude",
            value: b,
            valid_range: "[-90, 90]",
        });
    }
    
    // Convert to radians for ERFA
    let l_rad = l.to_radians();
    let b_rad = b.to_radians();
    
    // Use ERFA's Galactic to ICRS transformation
    let (ra_rad, dec_rad) = erfars::galacticcoordinates::G2icrs(l_rad, b_rad);
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra_rad.to_degrees();
    let dec_deg = dec_rad.to_degrees();
    
    // Normalize RA to [0, 360)
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    } else if ra_deg >= 360.0 {
        ra_deg -= 360.0;
    }
    
    Ok((ra_deg, dec_deg))
}

/// North Galactic Pole in J2000.0 coordinates  
pub const NGP_RA: f64 = 192.85948;  // degrees
pub const NGP_DEC: f64 = 27.12825;  // degrees

/// Galactic center in J2000.0 coordinates
pub const GC_RA: f64 = 266.405;  // degrees  
pub const GC_DEC: f64 = -28.936;  // degrees

/// Returns the galactic coordinates of common objects.
///
/// Useful for testing and reference.
pub fn galactic_landmarks() -> Vec<(&'static str, f64, f64)> {
    vec![
        ("Galactic Center", 0.0, 0.0),
        ("Galactic North Pole", 0.0, 90.0),  // Actually any l
        ("Galactic South Pole", 0.0, -90.0), // Actually any l
        ("Galactic Anticenter", 180.0, 0.0),
        ("Cygnus X-1", 71.3, 3.1),
        ("Large Magellanic Cloud", 280.5, -32.9),
        ("Small Magellanic Cloud", 302.8, -44.3),
        ("M31 (Andromeda)", 121.2, -21.6),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_galactic_center() {
        // Galactic center should be at l=0, b=0
        let (l, b) = equatorial_to_galactic(GC_RA, GC_DEC).unwrap();
        assert!((l - 0.0).abs() < 0.01 || (l - 360.0).abs() < 0.01);
        assert!((b - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_galactic_poles() {
        // North Galactic Pole
        let (_l, b) = equatorial_to_galactic(NGP_RA, NGP_DEC).unwrap();
        assert!((b - 90.0).abs() < 0.01);
        
        // South Galactic Pole (opposite of NGP)
        let sgp_ra = NGP_RA + 180.0;
        let sgp_dec = -NGP_DEC;
        let (_l2, b2) = equatorial_to_galactic(sgp_ra % 360.0, sgp_dec).unwrap();
        assert!((b2 - (-90.0)).abs() < 0.01);
    }

    #[test]
    fn test_galactic_round_trip() {
        // Test various coordinates round-trip
        let test_coords = [
            (83.633, 22.0145),   // Orion Nebula
            (279.234, 38.784),   // Vega
            (201.298, -43.019),  // Alpha Centauri
            (0.0, 0.0),          // Vernal equinox
        ];
        
        for (ra, dec) in test_coords {
            let (l, b) = equatorial_to_galactic(ra, dec).unwrap();
            let (ra2, dec2) = galactic_to_equatorial(l, b).unwrap();
            
            // Handle RA wraparound at 0/360 boundary
            let ra_diff = (ra2 - ra).abs();
            let ra_diff_wrapped = (360.0 - ra_diff).abs();
            let min_ra_diff = ra_diff.min(ra_diff_wrapped);
            
            assert!(min_ra_diff < 0.01, 
                "RA mismatch for ({}, {}): {} -> {}", ra, dec, ra, ra2);
            assert!((dec2 - dec).abs() < 0.01,
                "Dec mismatch for ({}, {}): {} -> {}", ra, dec, dec, dec2);
        }
    }

    #[test]
    fn test_known_objects() {
        // Test some known galactic coordinates
        // Sagittarius A* (galactic center)
        let (l, b) = equatorial_to_galactic(266.417, -29.008).unwrap();
        assert!(l < 0.5 || l > 359.5);
        assert!(b.abs() < 0.1);
        
        // Cygnus X-1
        let (l, b) = equatorial_to_galactic(299.590, 35.202).unwrap();
        assert!((l - 71.3).abs() < 0.5);
        assert!((b - 3.1).abs() < 0.5);
    }
}