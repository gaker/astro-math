//! Galactic coordinate system conversions.
//!
//! Converts between equatorial (RA/Dec) and galactic (l, b) coordinates.
//! Uses the IAU standard transformation.

use std::f64::consts::PI;
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
/// # Example
/// ```
/// use astro_math::equatorial_to_galactic;
///
/// // Galactic center should be at l=0, b=0
/// let (l, b) = equatorial_to_galactic(266.405, -28.936).unwrap();
/// assert!((l - 0.0).abs() < 0.1);
/// assert!((b - 0.0).abs() < 0.1);
/// ```
pub fn equatorial_to_galactic(ra: f64, dec: f64) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra)?;
    validate_dec(dec)?;
    // Convert to radians
    let ra_rad = ra * PI / 180.0;
    let dec_rad = dec * PI / 180.0;
    
    // Standard transformation matrix elements (Reid & Brunthaler 2004)
    // These implement the IAU standard galactic coordinate system
    let cos_dec = dec_rad.cos();
    let sin_dec = dec_rad.sin();
    let cos_ra = ra_rad.cos();
    let sin_ra = ra_rad.sin();
    
    // Convert to Cartesian coordinates
    let x = cos_dec * cos_ra;
    let y = cos_dec * sin_ra;
    let z = sin_dec;
    
    // Rotation matrix to galactic coordinates
    // T11 = -0.054875539390, T12 = -0.873437104725, T13 = -0.483834991775
    // T21 = +0.494109453633, T22 = -0.444829594298, T23 = +0.746982248696
    // T31 = -0.867666135681, T32 = -0.198076389622, T33 = +0.455983794523
    
    let xg = -0.054875539390 * x - 0.873437104725 * y - 0.483834991775 * z;
    let yg =  0.494109453633 * x - 0.444829594298 * y + 0.746982248696 * z;
    let zg = -0.867666135681 * x - 0.198076389622 * y + 0.455983794523 * z;
    
    // Convert back to spherical coordinates
    let l = yg.atan2(xg);
    let b = zg.atan2((xg * xg + yg * yg).sqrt());
    
    // Convert to degrees and normalize longitude
    let mut l_deg = l * 180.0 / PI;
    let b_deg = b * 180.0 / PI;
    
    if l_deg < 0.0 {
        l_deg += 360.0;
    }
    
    Ok((l_deg, b_deg))
}

/// Converts galactic coordinates to equatorial coordinates.
///
/// # Arguments
/// * `l` - Galactic longitude in degrees
/// * `b` - Galactic latitude in degrees
///
/// # Returns
/// Tuple of (ra, dec) in degrees (J2000.0)
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
pub fn galactic_to_equatorial(l: f64, b: f64) -> Result<(f64, f64)> {
    // Validate galactic latitude
    if b < -90.0 || b > 90.0 {
        return Err(crate::error::AstroError::InvalidCoordinate {
            coord_type: "Galactic latitude",
            value: b,
            valid_range: "[-90, 90]",
        });
    }
    // Convert to radians
    let l_rad = l * PI / 180.0;
    let b_rad = b * PI / 180.0;
    
    // Convert to Cartesian galactic coordinates
    let cos_b = b_rad.cos();
    let sin_b = b_rad.sin();
    let cos_l = l_rad.cos();
    let sin_l = l_rad.sin();
    
    let xg = cos_b * cos_l;
    let yg = cos_b * sin_l;
    let zg = sin_b;
    
    // Inverse rotation matrix (transpose of forward matrix)
    // T11 = -0.054875539390, T21 = +0.494109453633, T31 = -0.867666135681
    // T12 = -0.873437104725, T22 = -0.444829594298, T32 = -0.198076389622
    // T13 = -0.483834991775, T23 = +0.746982248696, T33 = +0.455983794523
    
    let x = -0.054875539390 * xg + 0.494109453633 * yg - 0.867666135681 * zg;
    let y = -0.873437104725 * xg - 0.444829594298 * yg - 0.198076389622 * zg;
    let z = -0.483834991775 * xg + 0.746982248696 * yg + 0.455983794523 * zg;
    
    // Convert back to spherical equatorial coordinates
    let ra = y.atan2(x);
    let dec = z.atan2((x * x + y * y).sqrt());
    
    // Convert to degrees and normalize RA
    let mut ra_deg = ra * 180.0 / PI;
    let dec_deg = dec * 180.0 / PI;
    
    if ra_deg < 0.0 {
        ra_deg += 360.0;
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
    fn test_round_trip() {
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