//! Proper motion calculations for stellar positions.
//!
//! This module handles the correction of stellar coordinates due to proper motion,
//! which is the apparent angular motion of stars across the celestial sphere due
//! to their actual motion through space relative to the Solar System.
//!
//! # Overview
//!
//! Proper motion is typically given in milliarcseconds per year (mas/yr) in two components:
//! - μα* (mu alpha star): Motion in right ascension, already multiplied by cos(δ)
//! - μδ (mu delta): Motion in declination
//!
//! The total proper motion is: μ = √(μα*² + μδ²)
//!
//! # Space Motion
//!
//! For nearby stars with known parallax and radial velocity, we can calculate
//! full 3D space motion and apply more accurate corrections.
//!
//! # Examples
//!
//! ```
//! use astro_math::proper_motion::apply_proper_motion;
//! use chrono::{TimeZone, Utc};
//!
//! // Barnard's Star - highest proper motion of any star
//! let ra_j2000 = 269.454;  // degrees
//! let dec_j2000 = 4.668;   // degrees
//! let pm_ra = -797.84;     // mas/yr (already includes cos(dec))
//! let pm_dec = 10326.93;   // mas/yr
//! 
//! // Calculate position at 2024.0
//! let epoch_2024 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
//! let (ra_2024, dec_2024) = apply_proper_motion(
//!     ra_j2000, dec_j2000, pm_ra, pm_dec, epoch_2024
//! ).unwrap();
//! ```

use crate::error::{Result, validate_ra, validate_dec};
use crate::time::j2000_days;
use chrono::{DateTime, Utc};

/// Applies proper motion to stellar coordinates.
///
/// # Arguments
/// * `ra_j2000` - Right ascension at J2000.0 epoch (degrees)
/// * `dec_j2000` - Declination at J2000.0 epoch (degrees)
/// * `pm_ra_cosdec` - Proper motion in RA × cos(dec) (mas/yr)
/// * `pm_dec` - Proper motion in declination (mas/yr)
/// * `target_epoch` - Date to calculate position for
///
/// # Returns
/// * `(ra, dec)` - Updated coordinates at target epoch (degrees)
///
/// # Errors
/// Returns `Err(AstroError::InvalidCoordinate)` if coordinates are invalid.
///
/// # Note
/// This is a linear approximation suitable for time spans < 100 years
/// and proper motions < 1000 mas/yr. For high proper motion stars
/// or long time spans, use `apply_proper_motion_rigorous`.
pub fn apply_proper_motion(
    ra_j2000: f64,
    dec_j2000: f64,
    pm_ra_cosdec: f64,  // mas/yr, already multiplied by cos(dec)
    pm_dec: f64,        // mas/yr
    target_epoch: DateTime<Utc>,
) -> Result<(f64, f64)> {
    validate_ra(ra_j2000)?;
    validate_dec(dec_j2000)?;
    
    // Time elapsed since J2000.0 in years
    let dt_years = j2000_days(target_epoch) / 365.25;
    
    // Convert proper motion from mas/yr to degrees/yr
    let pm_ra_deg = pm_ra_cosdec / 3_600_000.0;  // mas to degrees
    let pm_dec_deg = pm_dec / 3_600_000.0;
    
    // Apply linear proper motion
    let mut ra = ra_j2000 + pm_ra_deg * dt_years;
    let dec = dec_j2000 + pm_dec_deg * dt_years;
    
    // Normalize RA to [0, 360)
    while ra < 0.0 {
        ra += 360.0;
    }
    while ra >= 360.0 {
        ra -= 360.0;
    }
    
    // Validate declination hasn't exceeded poles
    validate_dec(dec)?;
    
    Ok((ra, dec))
}

/// Applies proper motion with space velocity (rigorous method).
///
/// This method accounts for the changing perspective as a star moves
/// through space, important for nearby stars with high proper motion.
///
/// # Arguments
/// * `ra_j2000` - Right ascension at J2000.0 (degrees)
/// * `dec_j2000` - Declination at J2000.0 (degrees)
/// * `pm_ra_cosdec` - Proper motion in RA × cos(dec) (mas/yr)
/// * `pm_dec` - Proper motion in declination (mas/yr)
/// * `parallax` - Annual parallax (mas)
/// * `radial_velocity` - Radial velocity (km/s, positive = receding)
/// * `target_epoch` - Date to calculate position for
///
/// # Returns
/// * `(ra, dec, parallax)` - Updated position and parallax (degrees, degrees, mas)
///
/// # Errors
/// Returns error if coordinates are invalid or parallax ≤ 0.
pub fn apply_proper_motion_rigorous(
    ra_j2000: f64,
    dec_j2000: f64,
    pm_ra_cosdec: f64,
    pm_dec: f64,
    parallax: f64,
    radial_velocity: f64,
    target_epoch: DateTime<Utc>,
) -> Result<(f64, f64, f64)> {
    use crate::error::AstroError;
    
    validate_ra(ra_j2000)?;
    validate_dec(dec_j2000)?;
    
    if parallax <= 0.0 {
        return Err(AstroError::OutOfRange {
            parameter: "parallax",
            value: parallax,
            min: 0.0,
            max: f64::INFINITY,
        });
    }
    
    // Time since J2000.0 in years
    let t = j2000_days(target_epoch) / 365.25;
    
    // Convert to radians
    let ra_rad = ra_j2000.to_radians();
    let dec_rad = dec_j2000.to_radians();
    
    // Distance in parsecs
    let dist_pc = 1000.0 / parallax;
    
    // Convert proper motions to radians/yr
    let _pm_ra_rad = pm_ra_cosdec * std::f64::consts::PI / (180.0 * 3_600_000.0);
    let _pm_dec_rad = pm_dec * std::f64::consts::PI / (180.0 * 3_600_000.0);
    
    // Velocity components in km/s
    // 4.74047 converts (mas/yr) * (pc) to km/s
    let vt_ra = 4.74047 * pm_ra_cosdec * dist_pc / 1000.0;
    let vt_dec = 4.74047 * pm_dec * dist_pc / 1000.0;
    
    // Cartesian position at J2000 (in parsecs)
    let x0 = dist_pc * dec_rad.cos() * ra_rad.cos();
    let y0 = dist_pc * dec_rad.cos() * ra_rad.sin();
    let z0 = dist_pc * dec_rad.sin();
    
    // Cartesian velocity components (km/s)
    let vx = -vt_ra * ra_rad.sin() - vt_dec * dec_rad.sin() * ra_rad.cos() + radial_velocity * dec_rad.cos() * ra_rad.cos();
    let vy = vt_ra * ra_rad.cos() - vt_dec * dec_rad.sin() * ra_rad.sin() + radial_velocity * dec_rad.cos() * ra_rad.sin();
    let vz = vt_dec * dec_rad.cos() + radial_velocity * dec_rad.sin();
    
    // Convert velocity to pc/yr: 1 km/s = 0.977792 pc/Myr = 0.000977792 pc/yr
    let k = 0.000977792;
    
    // Position at target epoch
    let x = x0 + vx * k * t;
    let y = y0 + vy * k * t;
    let z = z0 + vz * k * t;
    
    // Convert back to spherical coordinates
    let dist_new = (x*x + y*y + z*z).sqrt();
    let ra_new = y.atan2(x);
    let dec_new = (z / dist_new).asin();
    
    // New parallax
    let parallax_new = 1000.0 / dist_new;
    
    // Convert to degrees and normalize
    let mut ra_deg = ra_new.to_degrees();
    if ra_deg < 0.0 {
        ra_deg += 360.0;
    }
    let dec_deg = dec_new.to_degrees();
    
    Ok((ra_deg, dec_deg, parallax_new))
}

/// Calculates total proper motion from components.
///
/// # Arguments
/// * `pm_ra_cosdec` - Proper motion in RA × cos(dec) (mas/yr)
/// * `pm_dec` - Proper motion in declination (mas/yr)
///
/// # Returns
/// Total proper motion in mas/yr
pub fn total_proper_motion(pm_ra_cosdec: f64, pm_dec: f64) -> f64 {
    (pm_ra_cosdec * pm_ra_cosdec + pm_dec * pm_dec).sqrt()
}

/// Calculates position angle of proper motion.
///
/// # Arguments
/// * `pm_ra_cosdec` - Proper motion in RA × cos(dec) (mas/yr)
/// * `pm_dec` - Proper motion in declination (mas/yr)
///
/// # Returns
/// Position angle in degrees (0° = North, 90° = East)
pub fn proper_motion_position_angle(pm_ra_cosdec: f64, pm_dec: f64) -> f64 {
    let pa_rad = pm_ra_cosdec.atan2(pm_dec);
    let mut pa_deg = pa_rad.to_degrees();
    if pa_deg < 0.0 {
        pa_deg += 360.0;
    }
    pa_deg
}

/// Converts proper motion between different forms.
///
/// # Arguments
/// * `pm_ra` - Proper motion in RA (mas/yr, NOT multiplied by cos(dec))
/// * `dec` - Declination (degrees)
///
/// # Returns
/// Proper motion in RA × cos(dec) (mas/yr)
pub fn pm_ra_to_pm_ra_cosdec(pm_ra: f64, dec: f64) -> f64 {
    pm_ra * dec.to_radians().cos()
}

/// Converts proper motion between different forms.
///
/// # Arguments
/// * `pm_ra_cosdec` - Proper motion in RA × cos(dec) (mas/yr)
/// * `dec` - Declination (degrees)
///
/// # Returns
/// Proper motion in RA (mas/yr, NOT multiplied by cos(dec))
pub fn pm_ra_cosdec_to_pm_ra(pm_ra_cosdec: f64, dec: f64) -> f64 {
    pm_ra_cosdec / dec.to_radians().cos()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_no_proper_motion() {
        // Star with no proper motion should not move
        let epoch = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = apply_proper_motion(100.0, 25.0, 0.0, 0.0, epoch).unwrap();
        
        assert!((ra - 100.0).abs() < 1e-10);
        assert!((dec - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_barnards_star() {
        // Barnard's Star has the highest proper motion
        let ra_2000 = 269.454;
        let dec_2000 = 4.668;
        let pm_ra = -797.84;   // mas/yr
        let pm_dec = 10326.93; // mas/yr
        
        // Calculate position 50 years later
        let epoch = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
        let (_ra, dec) = apply_proper_motion(ra_2000, dec_2000, pm_ra, pm_dec, epoch).unwrap();
        
        // Rough calculation: 50 years * 10327 mas/yr / 3600000 mas/deg ≈ 0.14°
        assert!((dec - dec_2000) > 0.14 && (dec - dec_2000) < 0.15);
    }

    #[test]
    fn test_total_proper_motion_calculation() {
        // 3-4-5 triangle
        let total = total_proper_motion(3.0, 4.0);
        assert!((total - 5.0).abs() < 1e-10);
        
        // Barnard's Star
        let total_barnards = total_proper_motion(-797.84, 10326.93);
        assert!((total_barnards - 10357.72).abs() < 0.1);
    }

    #[test]
    fn test_position_angle() {
        // Due North
        let pa = proper_motion_position_angle(0.0, 1.0);
        assert!((pa - 0.0).abs() < 1e-10);
        
        // Due East
        let pa = proper_motion_position_angle(1.0, 0.0);
        assert!((pa - 90.0).abs() < 1e-10);
        
        // Due South
        let pa = proper_motion_position_angle(0.0, -1.0);
        assert!((pa - 180.0).abs() < 1e-10);
        
        // Due West
        let pa = proper_motion_position_angle(-1.0, 0.0);
        assert!((pa - 270.0).abs() < 1e-10);
    }
}