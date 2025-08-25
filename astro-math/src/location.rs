//! Geographic location representation and utilities.
//!
//! This module provides the [`Location`] struct for representing observer
//! positions on Earth, with flexible coordinate parsing for various formats.
//!
//! # Supported Coordinate Formats
//!
//! The parsing system handles an extensive range of formats:
//!
//! ## Decimal Degrees
//! - `"40.7128"` or `"-74.0060"`
//! - `"40.7128N"` or `"74.0060W"`
//! - `"N40.7128"` or `"W74.0060"`
//!
//! ## DMS (Degrees Minutes Seconds)
//! - `"40 42 46"` or `"40° 42' 46\""`
//! - `"40d42m46s"` or `"40deg42min46sec"`
//! - `"40:42:46"` or `"40-42-46"`
//! - `"40°42'46.08\"N"` (with decimals and direction)
//!
//! ## HMS (Hours Minutes Seconds) for longitude
//! - `"4h 56m 27s"` or `"4:56:27"`
//! - `"4h56m27s"` or `"4 hours 56 minutes 27 seconds"`
//!
//! ## Special handling
//! - Unicode symbols: `"40°42′46″"` (with proper Unicode prime symbols)
//! - Mixed formats: `"40° 42.767'"` (degrees and decimal minutes)
//! - Fuzzy matching: handles typos, extra spaces, mixed separators
//! - Case insensitive: `"40D42M46S"` or `"n40.7128"`
//!
//! # Error Handling
//!
//! Parsing returns `Result<Location>` with detailed error messages:
//! - `AstroError::InvalidDmsFormat` with suggestions for fixing common issues

use crate::time::julian_date;
use crate::{local_mean_sidereal_time, sidereal::apparent_sidereal_time};
use crate::error::{AstroError, Result};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use regex::{Regex, RegexBuilder};
use lazy_static::lazy_static;

// Pre-compiled regex patterns for performance
lazy_static! {
    /// HMS pattern with DoS protection
    static ref HMS_REGEX: Regex = RegexBuilder::new(
        r"(\d{1,3}(?:\.\d{1,10})?)\s*h\s*(\d{1,2}(?:\.\d{1,10})?)\s*m?\s*(\d{1,2}(?:\.\d{1,10})?)\s*s?"
    )
    .size_limit(1024 * 1024)  // 1MB regex size limit
    .dfa_size_limit(10 * 1024 * 1024) // 10MB DFA size limit
    .build()
    .expect("HMS regex compilation failed");
    
    /// DMS pattern with DoS protection
    static ref DMS_REGEX: Regex = RegexBuilder::new(
        r#"([+-]?\d{1,3}(?:\.\d{1,10})?)\s*[°d]?\s*(\d{1,2}(?:\.\d{1,10})?)\s*['′m]?\s*(\d{1,2}(?:\.\d{1,10})?)\s*["″s]?"#
    )
    .size_limit(1024 * 1024)
    .dfa_size_limit(10 * 1024 * 1024)
    .build()
    .expect("DMS regex compilation failed");
    
    /// Decimal degrees pattern with size limits
    static ref DECIMAL_REGEX: Regex = RegexBuilder::new(
        r"^[+-]?\d{1,3}(?:\.\d{1,15})?[NSEW]?$"
    )
    .case_insensitive(true)
    .size_limit(1024 * 1024)
    .build()
    .expect("Decimal regex compilation failed");
    
    /// Compact format pattern (DDMM.mmm or DDMMSS) with validation
    static ref COMPACT_REGEX: Regex = RegexBuilder::new(
        r"^([+-]?)(\d{2,3})(\d{2})(?:(\d{2})(?:\.(\d{1,6}))?)?$"
    )
    .size_limit(1024 * 1024)
    .build()
    .expect("Compact regex compilation failed");
}

/// Represents a physical observer location on Earth.
///
/// Used for computing local sidereal time, converting celestial coordinates,
/// and modeling telescope geometry.
#[derive(Debug, Clone, Copy)]
pub struct Location {
    /// Latitude in degrees (+N, -S)
    pub latitude_deg: f64,
    /// Longitude in degrees (+E, -W, Greenwich = 0)
    pub longitude_deg: f64,
    /// Altitude above sea level in meters
    pub altitude_m: f64,
}

impl Location {
    /// Parses a location from flexible coordinate strings.
    ///
    /// Automatically detects the coordinate format and applies appropriate parsing.
    ///
    /// # Supported Formats
    ///
    /// ## Decimal degrees
    /// - `"40.7128"` or `"-74.0060"`
    /// - `"40.7128N"` or `"74.0060W"`
    /// - `"N40.7128"` or `"W 74.0060"`
    /// - `"40.7128 N"` or `"74.0060 West"`
    /// - `"north 40.7128"` or `"west 74.0060"`
    ///
    /// ## DMS (Degrees Minutes Seconds)
    /// - `"40 42 46"` or `"40° 42' 46\""`
    /// - `"40d42m46s"` or `"40deg42min46sec"`
    /// - `"40:42:46"` or `"40-42-46"`
    /// - `"40°42'46.08\"N"` (with decimals and direction)
    /// - `"40d 42' 46.08\" N"` (mixed separators)
    /// - `"40 degrees 42 minutes 46 seconds"`
    ///
    /// ## DM (Degrees Decimal Minutes)
    /// - `"40° 42.767'"` or `"40d 42.767m"`
    /// - `"40 42.767"` (assumed DM if only 2 parts)
    ///
    /// ## HMS (Hours Minutes Seconds) for longitude
    /// - `"4h 56m 27s"` or `"4:56:27"`
    /// - `"4h56m27.5s"` or `"4 hours 56 minutes 27.5 seconds"`
    /// - `"4h 56' 27\""` (using arcminute/arcsecond symbols)
    ///
    /// ## Special handling
    /// - Unicode: `"40°42′46″"` (proper Unicode prime/double-prime)
    /// - Compact: `"404246N"` or `"0740060W"` (DDMMSS format)
    /// - Aviation: `"4042.767N"` (DDMM.mmm format)
    /// - Fuzzy: Handles extra spaces, mixed case, common typos
    ///
    /// # Arguments
    /// - `lat_str`: Latitude string in any supported format
    /// - `lon_str`: Longitude string in any supported format  
    /// - `alt_m`: Altitude in meters
    ///
    /// # Returns
    /// `Ok(Location)` if parsing succeeds
    ///
    /// # Errors
    /// Returns `Err(AstroError::InvalidDmsFormat)` with helpful error messages
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_math::location::Location;
    /// 
    /// // Decimal degrees with compass directions
    /// let loc = Location::parse("40.7128 N", "74.0060 W", 10.0).unwrap();
    /// assert!((loc.latitude_deg - 40.7128).abs() < 1e-6);
    /// assert!((loc.longitude_deg + 74.0060).abs() < 1e-6);
    ///
    /// // DMS with symbols
    /// let loc = Location::parse("40°42'46.08\"N", "74°0'21.6\"W", 10.0).unwrap();
    /// assert!((loc.latitude_deg - 40.7128).abs() < 1e-4);
    ///
    /// // HMS for longitude
    /// let loc = Location::parse("51.5074 N", "0h 7m 39.84s W", 0.0).unwrap();
    /// assert!((loc.longitude_deg + 1.9166).abs() < 1e-3);
    ///
    /// // Mixed formats and fuzzy matching
    /// let loc = Location::parse("40d 42m 46s North", "74 deg 0 min 21.6 sec west", 10.0).unwrap();
    /// assert!((loc.latitude_deg - 40.7128).abs() < 1e-4);
    /// ```
    pub fn parse(lat_str: &str, lon_str: &str, alt_m: f64) -> Result<Self> {
        let lat = parse_coordinate(lat_str, true)?;
        let lon = parse_coordinate(lon_str, false)?;
        Ok(Location {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: alt_m,
        })
    }

    /// Parses a `Location` from sexagesimal (DMS) strings for latitude and longitude.
    ///
    /// Supports a wide range of common DMS formats:
    /// - `"39 00 01.7"`
    /// - `"39:00:01.7"`
    /// - `"39°00'01.7\""`
    ///
    /// # Arguments
    /// - `lat_str`: Latitude string in sexagesimal format
    /// - `lon_str`: Longitude string in sexagesimal format
    /// - `alt_m`: Altitude in meters
    ///
    /// # Returns
    /// `Ok(Location)` if parsing succeeds
    ///
    /// # Errors
    /// Returns `Err(AstroError::InvalidDmsFormat)` if:
    /// - String doesn't match any supported DMS format
    /// - Degrees, minutes, or seconds are out of valid ranges
    ///
    /// # Examples
    ///
    /// ## DMS with spaces
    /// ```
    /// use astro_math::location::Location;
    /// let loc = Location::from_dms("+39 00 01.7", "-92 18 03.2", 250.0).unwrap();
    /// assert!((loc.latitude_deg - 39.0004722).abs() < 1e-6);
    /// assert!((loc.longitude_deg + 92.3008888).abs() < 1e-6);
    /// ```
    ///
    /// ## DMS with colons
    /// ```
    /// use astro_math::location::Location;
    /// let loc = Location::from_dms("+39:00:01.7", "-92:18:03.2", 250.0).unwrap();
    /// assert!((loc.latitude_deg - 39.0004722).abs() < 1e-6);
    /// ```
    ///
    /// ## ASCII punctuation
    /// ```
    /// use astro_math::location::Location;
    /// let loc = Location::from_dms("+39°00'01.7\"", "-92°18'03.2\"", 250.0).unwrap();
    /// assert!((loc.longitude_deg + 92.3008888).abs() < 1e-6);
    /// ```
    ///
    /// ## Invalid input
    /// ```
    /// use astro_math::location::Location;
    /// use astro_math::error::AstroError;
    /// 
    /// match Location::from_dms("foo", "bar", 100.0) {
    ///     Err(AstroError::InvalidDmsFormat { input, .. }) => {
    ///         assert_eq!(input, "foo");
    ///     }
    ///     _ => panic!("Expected InvalidDmsFormat error"),
    /// }
    /// ```
    pub fn from_dms(lat_str: &str, lon_str: &str, alt_m: f64) -> Result<Self> {
        let lat = parse_dms(lat_str)?;
        let lon = parse_dms(lon_str)?;
        Ok(Location {
            latitude_deg: lat,
            longitude_deg: lon,
            altitude_m: alt_m,
        })
    }

    pub fn latitude_dms_string(&self) -> String {
        format_dms(self.latitude_deg, true)
    }

    pub fn longitude_dms_string(&self) -> String {
        format_dms(self.longitude_deg, false)
    }

    /// Computes the Local Sidereal Time (LST) at this location for a given UTC timestamp.
    ///
    /// # Arguments
    /// - `datetime`: UTC datetime
    ///
    /// # Returns
    /// Local Sidereal Time in fractional hours
    ///
    /// # Example
    /// ```
    /// use chrono::{Utc, TimeZone};
    /// use astro_math::location::Location;
    ///
    /// let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    /// let loc = Location {
    ///     latitude_deg: 32.0,
    ///     longitude_deg: -64.0,
    ///     altitude_m: 200.0,
    /// };
    /// let lst = loc.local_sidereal_time(dt);
    /// assert!((lst - 4.3157).abs() < 1e-3);
    /// ```
    pub fn local_sidereal_time(&self, datetime: DateTime<Utc>) -> f64 {
        let jd = julian_date(datetime);
        apparent_sidereal_time(jd, self.longitude_deg)
    }

    /// Local Mean Sidreal Time (LMST) is calculated using the
    /// "mean equinox," a theoretical reference point in space that
    /// moves at a constant rate.
    /// # Arguments
    /// - `datetime`: UTC datetime
    ///
    /// # Returns
    /// Local Sidereal Time in fractional hours
    ///
    /// # Example
    /// ```
    /// use chrono::{Utc, TimeZone};
    /// use astro_math::location::Location;
    ///
    /// let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
    /// let loc = Location {
    ///     latitude_deg: 32.0,
    ///     longitude_deg: -64.0,
    ///     altitude_m: 200.0,
    /// };
    /// let lst = loc.local_mean_sidereal_time(dt);
    /// assert!((lst - 4.315).abs() < 1e-3);
    /// ```
    pub fn local_mean_sidereal_time(&self, datetime: DateTime<Utc>) -> f64 {
        let jd = julian_date(datetime);
        local_mean_sidereal_time(jd, self.longitude_deg)
    }

    /// Returns latitude formatted as ±DD° MM′ SS.sss″ (DMS)
    pub fn latitude_dms(&self) -> String {
        format_dms(self.latitude_deg, true)
    }

    /// Returns longitude formatted as ±DDD° MM′ SS.sss″ (DMS)
    pub fn longitude_dms(&self) -> String {
        format_dms(self.longitude_deg, false)
    }
}

/// Converts decimal degrees to DMS string format:
/// - `±DD° MM′ SS.sss″` for latitude
/// - `±DDD° MM′ SS.sss″` for longitude
fn format_dms(deg: f64, is_lat: bool) -> String {
    let sign = if deg < 0.0 { "-" } else { "" };
    let abs = deg.abs();
    let d = abs.trunc();
    let m = ((abs - d) * 60.0).trunc();
    let s = ((abs - d) * 60.0 - m) * 60.0;

    if is_lat {
        format!("{sign}{:02.0}° {:02.0}′ {:06.3}″", d, m, s)
    } else {
        format!("{sign}{:03.0}° {:02.0}′ {:06.3}″", d, m, s)
    }
}

// Legacy DMS parser for backward compatibility
fn parse_dms(s: &str) -> Result<f64> {
    // Accepts: "+39 00 01.7", "-92 18 03.2", "39:00:01.7", "-00 30 00"
    let original = s.trim();
    
    // Check for negative sign at the beginning
    let is_negative = original.starts_with('-');
    
    let cleaned = original
        .replace(['°', '\'', ':', '"'], " ");

    let parts: Vec<&str> = cleaned.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "DD MM SS.s or DD:MM:SS.s or DD°MM'SS.s\"",
        });
    }

    let d = f64::from_str(parts[0].trim_start_matches(['+', '-']))
        .map_err(|_| AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "DD MM SS.s or DD:MM:SS.s or DD°MM'SS.s\"",
        })?;
    let m = f64::from_str(parts.get(1).unwrap_or(&"0")).map_err(|_| AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "DD MM SS.s or DD:MM:SS.s or DD°MM'SS.s\"",
    })?;
    let s = f64::from_str(parts.get(2).unwrap_or(&"0")).map_err(|_| AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "DD MM SS.s or DD:MM:SS.s or DD°MM'SS.s\"",
    })?;

    // Calculate the absolute value first, then apply sign
    let abs_value = d.abs() + m / 60.0 + s / 3600.0;
    
    // Apply negative sign if original string started with -
    Ok(if is_negative { -abs_value } else { abs_value })
}

/// Parse coordinate from various input formats
fn parse_coordinate(input: &str, is_latitude: bool) -> Result<f64> {
    let s = input.trim();
    
    // Extract compass direction if present
    let (value_str, compass_dir) = extract_compass_direction(s);
    
    // Try various parsing strategies in order of likelihood
    
    // 1. Try compact formats first (specific patterns)
    if let Ok(deg) = try_parse_compact(&value_str) {
        return apply_compass_direction(deg, compass_dir, is_latitude);
    }
    
    // 2. Try decimal degrees (most common)
    if let Ok(deg) = try_parse_decimal_degrees(&value_str) {
        return apply_compass_direction(deg, compass_dir, is_latitude);
    }
    
    // 3. Try HMS format (for longitude)
    if !is_latitude {
        if let Ok(deg) = try_parse_hms(&value_str) {
            return apply_compass_direction(deg, compass_dir, is_latitude);
        }
    }
    
    // 4. Try DMS format
    if let Ok(deg) = try_parse_dms(&value_str) {
        return apply_compass_direction(deg, compass_dir, is_latitude);
    }
    
    // 5. Try degrees + decimal minutes
    if let Ok(deg) = try_parse_dm(&value_str) {
        return apply_compass_direction(deg, compass_dir, is_latitude);
    }
    
    // If all parsing fails, provide helpful error message
    Err(AstroError::InvalidDmsFormat {
        input: input.to_string(),
        expected: if is_latitude {
            "Examples: 40.7128, 40.7128N, N40.7128, 40°42'46\", 40 42 46, 40d42m46s"
        } else {
            "Examples: -74.0060, 74.0060W, W74.0060, 74°0'21.6\", 74 0 21.6, 4h56m27s"
        }
    })
}

/// Extract compass direction from string and return cleaned value
fn extract_compass_direction(s: &str) -> (String, Option<char>) {
    let upper = s.to_uppercase();
    
    // Check for direction at the beginning
    if let Some(first_char) = upper.chars().next() {
        if matches!(first_char, 'N' | 'S' | 'E' | 'W') {
            // Handle cases like "N40.7" or "N 40.7"
            let remainder = s[1..].trim_start();
            return (remainder.to_string(), Some(first_char));
        }
    }
    
    // Check for direction at the end - but only if it's a standalone letter or at the end of a word boundary
    if let Some(last_char) = upper.chars().last() {
        if matches!(last_char, 'N' | 'S' | 'E' | 'W') {
            // Check if this is likely a compass direction vs part of a word
            // Look at the character before the last one
            let chars: Vec<char> = upper.chars().collect();
            #[allow(clippy::comparison_chain)]
            if chars.len() == 1 {
                // Single character, definitely a direction
                let value = s[..s.len()-1].trim_end();
                return (value.to_string(), Some(last_char));
            } else if chars.len() > 1 {
                let second_to_last = chars[chars.len()-2];
                // If preceded by space, digit, or punctuation, it's likely a direction
                // If preceded by a letter, it's probably part of a word like "seconds"
                // BUT: watch out for patterns like "40d42m46s" where 's' is seconds, not South
                if !second_to_last.is_alphabetic() {
                    // Special case: detect "seconds" vs "South" direction
                    if last_char == 'S' && chars.len() >= 3 {
                        // Look for patterns that indicate "seconds" vs "South"
                        // "46s" or "27s" = seconds (digit immediately before 's', no other indicators)
                        // "8\"S" or "33.8688 S" = South (has space, quotes, or other separators)
                        let has_separators = s.contains(' ') || s.contains('"') || s.contains('\'') || s.contains('°');
                        if !has_separators && second_to_last.is_ascii_digit() {
                            // Pattern like "46s" - likely seconds
                        } else {
                            // Pattern like "8\"S" or "33.8688 S" - likely South direction
                            let value = s[..s.len()-1].trim_end();
                            return (value.to_string(), Some(last_char));
                        }
                    } else {
                        let value = s[..s.len()-1].trim_end();
                        return (value.to_string(), Some(last_char));
                    }
                }
            }
        }
    }
    
    // Check for spelled out directions
    let words: Vec<&str> = upper.split_whitespace().collect();
    let s_upper = s.to_uppercase();
    for word in &words {
        match *word {
            "NORTH" => return (s_upper.replace("NORTH", "").trim().to_string(), Some('N')),
            "SOUTH" => return (s_upper.replace("SOUTH", "").trim().to_string(), Some('S')),
            "EAST" => return (s_upper.replace("EAST", "").trim().to_string(), Some('E')),
            "WEST" => return (s_upper.replace("WEST", "").trim().to_string(), Some('W')),
            _ => {}
        }
    }
    
    (s.to_string(), None)
}

/// Apply compass direction to coordinate value
fn apply_compass_direction(mut value: f64, direction: Option<char>, is_latitude: bool) -> Result<f64> {
    if let Some(dir) = direction {
        match dir {
            'S' if is_latitude => value = -value.abs(),
            'W' if !is_latitude => value = -value.abs(),
            'N' if !is_latitude => return Err(AstroError::InvalidDmsFormat {
                input: format!("{}{}", value, dir),
                expected: "N/S for latitude, E/W for longitude"
            }),
            'E' if is_latitude => return Err(AstroError::InvalidDmsFormat {
                input: format!("{}{}", value, dir),
                expected: "N/S for latitude, E/W for longitude"
            }),
            _ => {}
        }
    }
    
    // Validate ranges
    if is_latitude {
        crate::error::validate_latitude(value)?;
    } else {
        crate::error::validate_longitude(value)?;
    }
    
    Ok(value)
}

/// Try to parse decimal degrees
fn try_parse_decimal_degrees(s: &str) -> Result<f64> {
    // Must not contain letters (except scientific notation)
    if s.chars().any(|c| c.is_alphabetic() && c != 'e' && c != 'E') {
        return Err(AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "decimal degrees"
        });
    }
    
    // Simple decimal number
    if let Ok(value) = f64::from_str(s) {
        return Ok(value);
    }
    
    // Handle leading + or - with spaces
    let cleaned = s.trim_start_matches('+').trim();
    if let Ok(value) = f64::from_str(cleaned) {
        return Ok(value);
    }
    
    Err(AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "decimal degrees"
    })
}

/// Input validation to prevent DoS attacks
fn validate_input_length(s: &str, _context: &str) -> Result<()> {
    const MAX_INPUT_LENGTH: usize = 1000; // Prevent extremely long inputs
    const MAX_UNICODE_LENGTH: usize = 500; // Unicode chars can be larger
    
    if s.len() > MAX_INPUT_LENGTH {
        return Err(AstroError::InvalidDmsFormat {
            input: format!("Input too long ({} chars)", s.len()),
            expected: "Input must be < 1000 characters",
        });
    }
    
    if s.chars().count() > MAX_UNICODE_LENGTH {
        return Err(AstroError::InvalidDmsFormat {
            input: format!("Too many Unicode characters ({} chars)", s.chars().count()),
            expected: "Input must be < 500 Unicode characters", 
        });
    }
    
    Ok(())
}

/// Try to parse HMS format (for longitude)
fn try_parse_hms(s: &str) -> Result<f64> {
    validate_input_length(s, "HMS")?;
    
    let normalized = s.to_lowercase()
        .replace("hours", "h").replace("hour", "h")
        .replace("minutes", "m").replace("minute", "m") 
        .replace("seconds", "s").replace("second", "s")
        .replace("hrs", "h").replace("hr", "h")
        .replace("mins", "m").replace("min", "m")
        .replace("secs", "s").replace("sec", "s")
        .replace('′', "'")  // Unicode prime
        .replace(['″', '"'], "\"");
    
    if let Some(caps) = HMS_REGEX.captures(&normalized) {
        let h = f64::from_str(&caps[1]).map_err(|_| AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "HMS format"
        })?;
        let m = caps.get(2).and_then(|c| f64::from_str(c.as_str()).ok()).unwrap_or(0.0);
        let s = caps.get(3).and_then(|c| f64::from_str(c.as_str()).ok()).unwrap_or(0.0);
        
        // Convert HMS to degrees (15 degrees per hour)
        return Ok((h + m/60.0 + s/3600.0) * 15.0);
    }
    
    // Try colon-separated HMS
    if s.contains('h') || s.contains('H') {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 2 {
            let h_part = parts[0].trim_end_matches(['h', 'H']);
            if let Ok(h) = f64::from_str(h_part) {
                let m = parts.get(1).and_then(|p| f64::from_str(p.trim()).ok()).unwrap_or(0.0);
                let s = parts.get(2).and_then(|p| f64::from_str(p.trim()).ok()).unwrap_or(0.0);
                return Ok((h + m/60.0 + s/3600.0) * 15.0);
            }
        }
    }
    
    Err(AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "HMS format"
    })
}

/// Try to parse DMS format with maximum flexibility
fn try_parse_dms(s: &str) -> Result<f64> {
    // First handle verbose format like "40 degrees 42 minutes 46 seconds"
    let verbose_normalized = s.to_lowercase()
        .replace("degrees", "d")
        .replace("degree", "d") 
        .replace("deg", "d")
        .replace("minutes", "m")
        .replace("minute", "m")
        .replace("min", "m")
        .replace("seconds", "s")
        .replace("second", "s")
        .replace("sec", "s");
    
    // Try parsing the verbose-normalized version first
    if verbose_normalized != s.to_lowercase() {
        // Only try this if we actually made substitutions
        // But make sure to preserve the original negative sign detection
        if let Ok(mut result) = try_parse_dms_internal(&verbose_normalized) {
            // Check the original string for negative sign since that's what we want to preserve
            if s.starts_with('-') {
                result = -result.abs();
            }
            return Ok(result);
        }
    }
    
    // Then try the original string
    try_parse_dms_internal(s)
}

/// Internal DMS parser that handles the actual parsing logic
fn try_parse_dms_internal(s: &str) -> Result<f64> {
    validate_input_length(s, "DMS")?;
    
    if let Some(caps) = DMS_REGEX.captures(s) {
        if caps.get(2).is_some() {  // Ensure at least degrees and minutes
            let d_str = &caps[1];
            let is_negative = s.starts_with('-') || d_str.starts_with('-');
            
            let d = f64::from_str(d_str.trim_start_matches('-')).map_err(|_| AstroError::InvalidDmsFormat {
                input: s.to_string(),
                expected: "DMS format"
            })?;
            let m = caps.get(2).and_then(|c| f64::from_str(c.as_str()).ok()).unwrap_or(0.0);
            let s = caps.get(3).and_then(|c| f64::from_str(c.as_str()).ok()).unwrap_or(0.0);
            
            let abs_value = d + m/60.0 + s/3600.0;
            return Ok(if is_negative { -abs_value } else { abs_value });
        }
    }
    
    // Normalize Unicode and common symbols  
    let _normalized = s
        .replace(['°', 'º', '′', '″', '\'', '"', '"', '`'], " ")
        .replace("''", " ") // Double apostrophe as seconds
        .replace(['d', 'D', 'm', 'M', 's', 'S'], " ")
        .to_lowercase();
    
    // Try various separators
    let separators = [' ', ':', ',', ';'];
    
    // Check if the string starts with a negative sign
    let is_negative = s.starts_with('-');
    
    for sep in &separators {
        let parts: Vec<&str> = s.split(*sep).filter(|p| !p.is_empty()).collect();
        if parts.len() >= 2 {
            // Clean up parts
            let clean_parts: Vec<String> = parts.iter().enumerate().map(|(i, p)| {
                let cleaned = p.trim()
                    .trim_end_matches(|c: char| c.is_alphabetic() || "°'\"″′".contains(c));
                // For the first part, also trim leading sign
                if i == 0 {
                    cleaned.trim_start_matches(['+', '-']).to_string()
                } else {
                    cleaned.to_string()
                }
            }).collect();
            
            if let Ok(d) = f64::from_str(&clean_parts[0]) {
                if let Ok(m) = f64::from_str(&clean_parts[1]) {
                    let s = clean_parts.get(2)
                        .and_then(|p| f64::from_str(p).ok())
                        .unwrap_or(0.0);
                    
                    let abs_value = d + m/60.0 + s/3600.0;
                    return Ok(if is_negative { -abs_value } else { abs_value });
                }
            }
        }
    }
    
    // Try dash separator specially 
    if s.contains('-') {
        // For negative numbers, skip the first dash
        let dash_parts: Vec<&str> = if is_negative {
            let no_first_dash = &s[1..]; // Remove the leading -
            no_first_dash.split('-').collect()
        } else {
            s.split('-').collect()
        };
        
        let parts: Vec<&str> = dash_parts.into_iter().filter(|p| !p.is_empty()).collect();
        if parts.len() >= 2 {
            let clean_parts: Vec<String> = parts.iter().map(|p| {
                p.trim()
                    .trim_end_matches(|c: char| c.is_alphabetic() || "°'\"″′".contains(c))
                    .to_string()
            }).collect();
            
            if let Ok(d) = f64::from_str(&clean_parts[0]) {
                if let Ok(m) = f64::from_str(&clean_parts[1]) {
                    let s = clean_parts.get(2)
                        .and_then(|p| f64::from_str(p).ok())
                        .unwrap_or(0.0);
                    
                    let abs_value = d + m/60.0 + s/3600.0;
                    return Ok(if is_negative { -abs_value } else { abs_value });
                }
            }
        }
    }
    
    Err(AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "DMS format"
    })
}

/// Try to parse compact formats like DDMMSS or DDMM.mmm
fn try_parse_compact(s: &str) -> Result<f64> {
    // Only try compact format if string has no spaces or separators
    if s.contains(' ') || s.contains(':') || s.contains('-') || s.contains('°') {
        return Err(AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "compact format"
        });
    }
    
    // Remove all non-digit characters except decimal point
    let digits_only: String = s.chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    
    // Must be mostly digits
    if digits_only.len() < s.len() / 2 {
        return Err(AstroError::InvalidDmsFormat {
            input: s.to_string(),
            expected: "compact format"
        });
    }
    
    // DDMM.mmm format (aviation/marine)
    if digits_only.contains('.') && digits_only.len() >= 6 {
        let parts: Vec<&str> = digits_only.split('.').collect();
        if parts[0].len() == 4 || parts[0].len() == 5 {  // DDMM or DDDMM
            if let Ok(ddmm) = i32::from_str(parts[0]) {
                let dd = ddmm / 100;
                let mm = ddmm % 100;
                if mm < 60 {  // Valid minutes
                    let decimal_minutes = parts.get(1)
                        .and_then(|p| f64::from_str(&format!("0.{}", p)).ok())
                        .unwrap_or(0.0);
                    
                    return Ok(dd as f64 + (mm as f64 + decimal_minutes) / 60.0);
                }
            }
        }
    }
    
    // DDMMSS format
    if !digits_only.contains('.') && (digits_only.len() == 6 || digits_only.len() == 7) {
        // DDMMSS or DDDMMSS
        let (dd_len, _is_longitude) = if digits_only.len() == 7 { (3, true) } else { (2, false) };
        
        if let Ok(dd) = i32::from_str(&digits_only[..dd_len]) {
            if let Ok(mm) = i32::from_str(&digits_only[dd_len..dd_len+2]) {
                if let Ok(ss) = i32::from_str(&digits_only[dd_len+2..]) {
                    if mm < 60 && ss < 60 {  // Valid minutes and seconds
                        return Ok(dd as f64 + mm as f64 / 60.0 + ss as f64 / 3600.0);
                    }
                }
            }
        }
    }
    
    Err(AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "compact format"
    })
}

/// Try to parse degrees and decimal minutes
fn try_parse_dm(s: &str) -> Result<f64> {
    // Normalize the string
    let normalized = s
        .replace(['°', '′', '\'', 'd', 'm'], " ")
        .to_lowercase();
    
    // Split and clean parts
    let parts: Vec<&str> = normalized.split_whitespace()
        .filter(|p| p.chars().any(|c| c.is_ascii_digit()))
        .collect();
    
    if parts.len() == 2 {
        if let Ok(d) = f64::from_str(parts[0]) {
            if let Ok(m) = f64::from_str(parts[1]) {
                // Check if minutes value makes sense (should be < 60 if integer part)
                if m < 60.0 || m.fract() != 0.0 {
                    let sign = if d < 0.0 { -1.0 } else { 1.0 };
                    return Ok(sign * (d.abs() + m / 60.0));
                }
            }
        }
    }
    
    Err(AstroError::InvalidDmsFormat {
        input: s.to_string(),
        expected: "degrees and decimal minutes"
    })
}
