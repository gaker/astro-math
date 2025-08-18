//! Error types for astro-math calculations.
//!
//! Handles validation and error reporting for coordinate conversions,
//! time calculations, and astronomical computations.
//!
//! # Error Types
//!
//! The main error type is [`AstroError`], which covers all possible errors in the crate:
//!
//! - **Coordinate errors**: Invalid RA, Dec, latitude, or longitude values
//! - **Range errors**: Values outside acceptable ranges for calculations
//! - **Format errors**: Invalid string formats (e.g., DMS parsing)
//! - **Calculation errors**: Mathematical failures or edge cases
//! - **Projection errors**: Points that cannot be projected
//!
//! # Examples
//!
//! ```
//! use astro_math::error::{AstroError, validate_ra, validate_dec};
//!
//! // Validate coordinates before use
//! match validate_ra(400.0) {
//!     Ok(_) => println!("Valid RA"),
//!     Err(e) => println!("Error: {}", e), // "Invalid RA: 400 (valid range: [0, 360))"
//! }
//!
//! // Functions return Result types
//! use astro_math::{ra_dec_to_alt_az, Location};
//! use chrono::Utc;
//!
//! let location = Location { latitude_deg: 40.0, longitude_deg: -74.0, altitude_m: 0.0 };
//! let result = ra_dec_to_alt_az(400.0, 45.0, Utc::now(), &location);
//! 
//! match result {
//!     Ok((alt, az)) => println!("Alt: {}, Az: {}", alt, az),
//!     Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
//!         println!("Invalid {}: {}", coord_type, value);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```

use thiserror::Error;

/// Main error type for astro-math operations.
///
/// This enum represents all possible errors that can occur during astronomical
/// calculations. Each variant provides specific information about what went wrong.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum AstroError {
    /// Invalid coordinate value
    #[error("Invalid {coord_type}: {value} (valid range: {valid_range})")]
    InvalidCoordinate {
        /// Type of coordinate (e.g., "RA", "Dec", "Latitude")
        coord_type: &'static str,
        /// The invalid value
        value: f64,
        /// Valid range description
        valid_range: &'static str,
    },
    
    /// Invalid time/date
    #[error("Invalid date/time: {reason}")]
    InvalidDateTime {
        /// Description of the issue
        reason: String,
    },
    
    /// Calculation failed
    #[error("Calculation error in {calculation}: {reason}")]
    CalculationError {
        /// What calculation failed
        calculation: &'static str,
        /// Why it failed
        reason: String,
    },
    
    /// Object never rises or sets
    #[error("{}", if *.always_above { "Object is circumpolar (never sets)" } else { "Object never rises above horizon" })]
    NeverRisesOrSets {
        /// Whether it's always above or below horizon
        always_above: bool,
    },
    
    /// Invalid DMS string format
    #[error("Invalid DMS format '{input}' (expected: {expected})")]
    InvalidDmsFormat {
        /// The invalid string
        input: String,
        /// Expected format
        expected: &'static str,
    },
    
    /// Value out of valid range
    #[error("{parameter} value {value} is out of range [{min}, {max}]")]
    OutOfRange {
        /// Parameter name
        parameter: &'static str,
        /// The invalid value
        value: f64,
        /// Min value (inclusive)
        min: f64,
        /// Max value (inclusive)
        max: f64,
    },
    
    /// Projection error (e.g., point on opposite side of sky)
    #[error("Projection error: {reason}")]
    ProjectionError {
        /// Description of the issue
        reason: String,
    },
}

/// Type alias for Results in this crate.
/// 
/// All fallible operations in astro-math return this Result type.
pub type Result<T> = std::result::Result<T, AstroError>;

/// Validate that a value is within a range.
///
/// # Arguments
/// * `value` - The value to check
/// * `min` - Minimum valid value (inclusive)
/// * `max` - Maximum valid value (inclusive)
/// * `parameter` - Name of the parameter for error messages
///
/// # Errors
/// Returns `AstroError::OutOfRange` if the value is outside [min, max].
///
/// # Example
/// ```
/// use astro_math::error::validate_range;
/// 
/// assert!(validate_range(45.0, 0.0, 90.0, "altitude").is_ok());
/// assert!(validate_range(100.0, 0.0, 90.0, "altitude").is_err());
/// ```
#[inline]
pub fn validate_range(value: f64, min: f64, max: f64, parameter: &'static str) -> Result<()> {
    if value < min || value > max {
        Err(AstroError::OutOfRange { parameter, value, min, max })
    } else {
        Ok(())
    }
}

/// Validate right ascension (0 <= RA < 360).
///
/// Right ascension must be in the range [0, 360) degrees.
///
/// # Errors
/// Returns `AstroError::InvalidCoordinate` if RA is outside the valid range.
///
/// # Example
/// ```
/// use astro_math::error::validate_ra;
/// 
/// assert!(validate_ra(0.0).is_ok());
/// assert!(validate_ra(359.9).is_ok());
/// assert!(validate_ra(360.0).is_err()); // 360 is invalid (use 0 instead)
/// assert!(validate_ra(-1.0).is_err());
/// ```
#[inline]
pub fn validate_ra(ra: f64) -> Result<()> {
    if ra < 0.0 || ra >= 360.0 {
        Err(AstroError::InvalidCoordinate {
            coord_type: "RA",
            value: ra,
            valid_range: "[0, 360)",
        })
    } else {
        Ok(())
    }
}

/// Validate declination (-90 <= Dec <= 90).
///
/// Declination must be in the range [-90, 90] degrees.
///
/// # Errors
/// Returns `AstroError::InvalidCoordinate` if Dec is outside the valid range.
///
/// # Example
/// ```
/// use astro_math::error::validate_dec;
/// 
/// assert!(validate_dec(0.0).is_ok());
/// assert!(validate_dec(90.0).is_ok());
/// assert!(validate_dec(-90.0).is_ok());
/// assert!(validate_dec(91.0).is_err());
/// assert!(validate_dec(-91.0).is_err());
/// ```
#[inline]
pub fn validate_dec(dec: f64) -> Result<()> {
    if dec < -90.0 || dec > 90.0 {
        Err(AstroError::InvalidCoordinate {
            coord_type: "Declination",
            value: dec,
            valid_range: "[-90, 90]",
        })
    } else {
        Ok(())
    }
}

/// Validate latitude (-90 <= lat <= 90)
#[inline]
pub fn validate_latitude(lat: f64) -> Result<()> {
    if lat < -90.0 || lat > 90.0 {
        Err(AstroError::InvalidCoordinate {
            coord_type: "Latitude",
            value: lat,
            valid_range: "[-90, 90]",
        })
    } else {
        Ok(())
    }
}

/// Validate longitude (-180 <= lon <= 180)
#[inline]
pub fn validate_longitude(lon: f64) -> Result<()> {
    if lon < -180.0 || lon > 180.0 {
        Err(AstroError::InvalidCoordinate {
            coord_type: "Longitude",
            value: lon,
            valid_range: "[-180, 180]",
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let err = AstroError::InvalidCoordinate {
            coord_type: "RA",
            value: 400.0,
            valid_range: "[0, 360)",
        };
        assert_eq!(err.to_string(), "Invalid RA: 400 (valid range: [0, 360))");
    }
    
    #[test]
    fn test_validate_ra() {
        assert!(validate_ra(0.0).is_ok());
        assert!(validate_ra(359.9).is_ok());
        assert!(validate_ra(-1.0).is_err());
        assert!(validate_ra(360.0).is_err());
    }
    
    #[test]
    fn test_validate_dec() {
        assert!(validate_dec(0.0).is_ok());
        assert!(validate_dec(90.0).is_ok());
        assert!(validate_dec(-90.0).is_ok());
        assert!(validate_dec(91.0).is_err());
        assert!(validate_dec(-91.0).is_err());
    }
}