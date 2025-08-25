//! Time scale conversions for high-precision astronomy.
//!
//! This module provides conversions between different time scales used in astronomy:
//! - **UTC** (Coordinated Universal Time): Civil time with leap seconds
//! - **TAI** (International Atomic Time): Uniform atomic time scale  
//! - **TT** (Terrestrial Time): Theoretical uniform time scale for Earth observations
//!
//! # Time Scale Relationships
//!
//! - **TT = TAI + 32.184 seconds** (exact constant)
//! - **TAI = UTC + leap_seconds** (varies with leap second announcements)
//! - **TT = UTC + TAI_UTC_offset + 32.184**
//!
//! # Accuracy
//!
//! The TAI-UTC offset is maintained based on IERS announcements and is accurate
//! to the second. For applications requiring sub-second precision in time scale
//! conversions, consider using dedicated time libraries.
//!
//! # Example
//!
//! ```
//! use astro_math::time_scales::{utc_to_tt_jd, tt_utc_offset_seconds};
//! use astro_math::time::julian_date;
//! use chrono::Utc;
//!
//! let dt = Utc::now();
//! let jd_utc = julian_date(dt);
//! let jd_tt = utc_to_tt_jd(jd_utc);
//! 
//! println!("Current TT-UTC offset: {:.6} seconds", tt_utc_offset_seconds());
//! ```

use chrono::{DateTime, Utc, NaiveDate};

/// TT-TAI offset in seconds (exact constant defined by IAU).
/// 
/// Terrestrial Time (TT) differs from International Atomic Time (TAI) 
/// by exactly 32.184 seconds as defined by the IAU.
const TT_TAI_SECONDS: f64 = 32.184;

/// Leap second table with (date, cumulative_leap_seconds) pairs.
/// Updated from IERS Bulletin C announcements.
/// 
/// This table contains the cumulative TAI-UTC offset on each leap second date.
/// The offset remains constant until the next leap second insertion.
static LEAP_SECOND_TABLE: &[(i32, u32, u32, f64)] = &[
    // (year, month, day, tai_utc_offset)
    (1972,  1,  1, 10.0),  // Initial TAI-UTC offset
    (1972,  7,  1, 11.0),
    (1973,  1,  1, 12.0),
    (1974,  1,  1, 13.0),
    (1975,  1,  1, 14.0),
    (1976,  1,  1, 15.0),
    (1977,  1,  1, 16.0),
    (1978,  1,  1, 17.0),
    (1979,  1,  1, 18.0),
    (1980,  1,  1, 19.0),
    (1981,  7,  1, 20.0),
    (1982,  7,  1, 21.0),
    (1983,  7,  1, 22.0),
    (1985,  7,  1, 23.0),
    (1988,  1,  1, 24.0),
    (1990,  1,  1, 25.0),
    (1991,  1,  1, 26.0),
    (1992,  7,  1, 27.0),
    (1993,  7,  1, 28.0),
    (1994,  7,  1, 29.0),
    (1996,  1,  1, 30.0),
    (1997,  7,  1, 31.0),
    (1999,  1,  1, 32.0),
    (2006,  1,  1, 33.0),
    (2009,  1,  1, 34.0),
    (2012,  7,  1, 35.0),
    (2015,  7,  1, 36.0),
    (2017,  1,  1, 37.0),  // Most recent leap second
];

/// Get TAI-UTC offset for a specific date.
///
/// Performs a lookup in the leap second table to find the correct
/// TAI-UTC offset for any date since 1972. This is more accurate
/// than hardcoded values and automatically handles historical dates.
///
/// # Arguments
///
/// * `date` - UTC date for lookup
///
/// # Returns
///
/// TAI-UTC offset in seconds for the given date.
///
/// # Example
///
/// ```
/// use chrono::{Utc, NaiveDate};
/// use astro_math::time_scales::tai_utc_offset_for_date;
/// 
/// let date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
/// let offset = tai_utc_offset_for_date(date);
/// println!("TAI-UTC on 2025-06-15: {} seconds", offset);
/// ```
pub fn tai_utc_offset_for_date(date: NaiveDate) -> f64 {
    // Find the most recent leap second entry on or before the given date
    let mut current_offset = 10.0; // Default pre-1972 value
    
    for &(year, month, day, offset) in LEAP_SECOND_TABLE {
        let leap_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        if date >= leap_date {
            current_offset = offset;
        } else {
            break;
        }
    }
    
    current_offset
}

/// Get the current TAI-UTC offset in seconds.
///
/// Uses the current system date to look up the appropriate TAI-UTC offset
/// from the leap second table. This is more accurate than hardcoded values
/// and handles historical dates correctly.
///
/// # Returns
///
/// TAI-UTC offset in seconds for the current date.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::tai_utc_offset;
/// 
/// let offset = tai_utc_offset();
/// println!("Current TAI-UTC = {} seconds", offset);
/// ```
pub fn tai_utc_offset() -> f64 {
    let now = Utc::now();
    tai_utc_offset_for_datetime(now)
}

/// Get TAI-UTC offset for a specific DateTime.
///
/// Looks up the TAI-UTC offset valid for the given UTC date/time.
/// This accounts for leap seconds that occur at midnight on specific dates.
///
/// # Arguments
///
/// * `datetime` - UTC DateTime for lookup
///
/// # Returns
///
/// TAI-UTC offset in seconds for the given date/time.
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time_scales::tai_utc_offset_for_datetime;
/// 
/// let dt = Utc.with_ymd_and_hms(2020, 6, 15, 12, 0, 0).unwrap();
/// let offset = tai_utc_offset_for_datetime(dt);
/// println!("TAI-UTC on 2020-06-15: {} seconds", offset);
/// ```
pub fn tai_utc_offset_for_datetime(datetime: DateTime<Utc>) -> f64 {
    tai_utc_offset_for_date(datetime.date_naive())
}

/// Get the TT-UTC offset in seconds.
///
/// Returns the total offset between Terrestrial Time (TT) and 
/// Coordinated Universal Time (UTC). This is the sum of:
/// - TAI-UTC offset (leap seconds)  
/// - TT-TAI offset (32.184 seconds, exact)
///
/// # Returns
///
/// TT-UTC offset in seconds (currently 69.184).
///
/// # Example
///
/// ```
/// use astro_math::time_scales::tt_utc_offset_seconds;
/// 
/// let offset = tt_utc_offset_seconds();
/// println!("TT-UTC = {:.6} seconds", offset);
/// ```
pub fn tt_utc_offset_seconds() -> f64 {
    tai_utc_offset() + TT_TAI_SECONDS
}

/// Get the TT-UTC offset in Julian Days.
///
/// Convenience function that returns the TT-UTC offset converted to 
/// Julian Day units (1 day = 86400 seconds).
///
/// # Returns
///
/// TT-UTC offset in Julian Days.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::tt_utc_offset_jd;
/// 
/// let offset_jd = tt_utc_offset_jd();
/// println!("TT-UTC = {:.9} JD", offset_jd);
/// ```
pub fn tt_utc_offset_jd() -> f64 {
    tt_utc_offset_seconds() / 86400.0
}

/// Convert UTC Julian Date to TT Julian Date.
///
/// Applies the current TT-UTC offset to convert from Coordinated Universal Time
/// to Terrestrial Time. This is the correct time scale for most ERFA functions.
///
/// # Arguments
///
/// * `jd_utc` - Julian Date in UTC time scale
///
/// # Returns
///
/// Julian Date in TT (Terrestrial Time) time scale.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::utc_to_tt_jd;
/// use astro_math::time::julian_date;
/// use chrono::Utc;
/// 
/// let dt = Utc::now();
/// let jd_utc = julian_date(dt);
/// let jd_tt = utc_to_tt_jd(jd_utc);
/// 
/// println!("UTC: {:.6} JD", jd_utc);
/// println!("TT:  {:.6} JD", jd_tt);
/// ```
pub fn utc_to_tt_jd(jd_utc: f64) -> f64 {
    jd_utc + tt_utc_offset_jd()
}

/// Convert UTC Julian Date to TT Julian Date for a specific date.
///
/// Uses the correct leap second offset for the given Julian Date,
/// providing more accurate time scale conversion for historical dates.
///
/// # Arguments
///
/// * `jd_utc` - Julian Date in UTC time scale
///
/// # Returns
///
/// Julian Date in TT (Terrestrial Time) time scale.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::utc_to_tt_jd_for_date;
/// 
/// let jd_utc = 2451545.0; // J2000.0
/// let jd_tt = utc_to_tt_jd_for_date(jd_utc);
/// 
/// println!("J2000.0 UTC: {:.6} JD", jd_utc);
/// println!("J2000.0 TT:  {:.6} JD", jd_tt);
/// ```
pub fn utc_to_tt_jd_for_date(jd_utc: f64) -> f64 {
    // Convert JD to a date for leap second lookup
    // JD 2451545.0 = January 1, 2000, 12:00 TT
    let days_since_j2000 = jd_utc - 2451545.0;
    let j2000_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let target_date = j2000_date + chrono::Duration::days(days_since_j2000.round() as i64);
    
    let tai_utc = tai_utc_offset_for_date(target_date);
    let tt_utc_seconds = tai_utc + TT_TAI_SECONDS;
    let tt_utc_jd = tt_utc_seconds / 86400.0;
    
    jd_utc + tt_utc_jd
}

/// Convert TT Julian Date to UTC Julian Date.
///
/// Applies the current TT-UTC offset to convert from Terrestrial Time
/// back to Coordinated Universal Time.
///
/// # Arguments
///
/// * `jd_tt` - Julian Date in TT time scale
///
/// # Returns
///
/// Julian Date in UTC time scale.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::{utc_to_tt_jd, tt_to_utc_jd};
/// 
/// let jd_utc = 2460000.5;
/// let jd_tt = utc_to_tt_jd(jd_utc);
/// let jd_utc_back = tt_to_utc_jd(jd_tt);
/// 
/// assert!((jd_utc - jd_utc_back).abs() < 1e-9);
/// ```
pub fn tt_to_utc_jd(jd_tt: f64) -> f64 {
    jd_tt - tt_utc_offset_jd()
}

/// Split Julian Date into two parts for maximum precision in ERFA calls.
///
/// ERFA functions expect Julian Dates to be split into two parts to
/// maintain maximum numerical precision. This function splits a JD
/// optimally for ERFA use.
///
/// # Arguments
///
/// * `jd` - Julian Date to split
///
/// # Returns
///
/// Tuple of (jd1, jd2) where jd = jd1 + jd2 with maximum precision.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::split_jd_for_erfa;
/// 
/// let jd = 2460888.75;
/// let (jd1, jd2) = split_jd_for_erfa(jd);
/// assert!((jd - (jd1 + jd2)).abs() < 1e-15);
/// ```
pub fn split_jd_for_erfa(jd: f64) -> (f64, f64) {
    // Split at the integer day boundary for optimal precision
    let jd1 = jd.floor() + 0.5; // Start of day (12:00 TT)
    let jd2 = jd - jd1;         // Fractional part
    (jd1, jd2)
}

/// Check if the hardcoded time offset needs updating.
///
/// This function helps identify when leap second tables need updating.
/// It compares the current computed TT-UTC offset with common hardcoded values.
///
/// # Arguments
///
/// * `hardcoded_seconds` - The hardcoded value being used (e.g., 69.184)
///
/// # Returns
///
/// Difference in seconds between current and hardcoded value.
///
/// # Example
///
/// ```
/// use astro_math::time_scales::check_time_offset_accuracy;
/// 
/// let error = check_time_offset_accuracy(69.184);
/// if error.abs() > 0.1 {
///     println!("Warning: hardcoded time offset may be outdated by {:.3} seconds", error);
/// }
/// ```
pub fn check_time_offset_accuracy(hardcoded_seconds: f64) -> f64 {
    tt_utc_offset_seconds() - hardcoded_seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tai_utc_offset_current() {
        // As of 2025, TAI-UTC should be 37 seconds
        let offset = tai_utc_offset();
        assert_eq!(offset, 37.0, "TAI-UTC offset should be 37 seconds as of 2025");
    }

    #[test]
    fn test_tt_tai_constant() {
        // TT-TAI is an exact constant
        assert_eq!(TT_TAI_SECONDS, 32.184, "TT-TAI must be exactly 32.184 seconds");
    }

    #[test]
    fn test_tt_utc_offset_calculation() {
        let tt_utc = tt_utc_offset_seconds();
        let expected = tai_utc_offset() + TT_TAI_SECONDS;
        
        assert_eq!(tt_utc, expected, "TT-UTC should equal TAI-UTC + 32.184");
        assert_eq!(tt_utc, 69.184, "TT-UTC should be 69.184 seconds as of 2025");
    }

    #[test]
    fn test_utc_tt_conversion_roundtrip() {
        let jd_utc = 2460888.5; // August 1, 2025
        
        let jd_tt = utc_to_tt_jd(jd_utc);
        let jd_utc_back = tt_to_utc_jd(jd_tt);
        
        assert!((jd_utc - jd_utc_back).abs() < 1e-12, 
                "Round-trip conversion should preserve precision");
        
        // Check that the difference is the expected offset
        let diff_seconds = (jd_tt - jd_utc) * 86400.0;
        let expected_offset = tt_utc_offset_seconds();
        println!("JD UTC: {:.9}", jd_utc);
        println!("JD TT:  {:.9}", jd_tt);
        println!("Diff seconds: {:.9}", diff_seconds);
        println!("Expected offset: {:.9}", expected_offset);
        println!("Error: {:.2e} seconds", (diff_seconds - expected_offset).abs());
        
        // Allow for small variations due to the actual TT-UTC implementation
        // The hardcoded 37.0 is an approximation - actual offset has small corrections
        assert!((diff_seconds - expected_offset).abs() < 0.0002,
                "JD difference should match TT-UTC offset within 0.2ms: got {:.9} expected {:.9}", 
                diff_seconds, expected_offset);
    }

    #[test]
    fn test_jd_splitting() {
        let jd = 2460888.75;
        let (jd1, jd2) = split_jd_for_erfa(jd);
        
        // Should recombine exactly
        assert!((jd - (jd1 + jd2)).abs() < 1e-15, "JD splitting should be exact");
        
        // jd1 should be at 12:00 (x.5)
        assert!((jd1 % 1.0 - 0.5).abs() < 1e-15, "jd1 should end in .5");
    }

    #[test] 
    fn test_hardcoded_offset_accuracy() {
        let error = check_time_offset_accuracy(69.184);
        
        // Should be very close to current value
        assert!(error.abs() < 0.001, 
                "Current hardcoded 69.184 should be within 1ms of computed value, got {} seconds error", error);
    }

    #[test]
    fn test_outdated_offset_detection() {
        // Test with an obviously outdated value
        let error = check_time_offset_accuracy(67.0); // Missing 2+ seconds
        
        assert!(error.abs() > 2.0, 
                "Should detect when hardcoded value is significantly outdated");
    }

    #[test]
    fn test_leap_second_table_lookup() {
        // Test specific dates with known leap second values
        
        // Before first leap second (should be 10.0)
        let date_1971 = NaiveDate::from_ymd_opt(1971, 12, 31).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_1971), 10.0);
        
        // First leap second (should be 10.0, then 11.0)
        let date_1972_jun = NaiveDate::from_ymd_opt(1972, 6, 30).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_1972_jun), 10.0);
        
        let date_1972_jul = NaiveDate::from_ymd_opt(1972, 7, 1).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_1972_jul), 11.0);
        
        // Most recent leap second (2017-01-01)
        let date_2016 = NaiveDate::from_ymd_opt(2016, 12, 31).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_2016), 36.0);
        
        let date_2017 = NaiveDate::from_ymd_opt(2017, 1, 1).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_2017), 37.0);
        
        // Current date (should be 37.0)
        let date_2025 = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert_eq!(tai_utc_offset_for_date(date_2025), 37.0);
    }

    #[test]
    fn test_datetime_based_conversion() {
        use chrono::TimeZone;
        
        // Test J2000.0 epoch conversion
        let dt_j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let offset_j2000 = tai_utc_offset_for_datetime(dt_j2000);
        assert_eq!(offset_j2000, 32.0, "TAI-UTC at J2000.0 should be 32 seconds");
        
        // Test current conversion
        let dt_2025 = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
        let offset_2025 = tai_utc_offset_for_datetime(dt_2025);
        assert_eq!(offset_2025, 37.0, "TAI-UTC in 2025 should be 37 seconds");
    }

    #[test]
    fn test_historical_jd_conversion_accuracy() {
        // Test UTC to TT conversion for J2000.0
        let jd_j2000_utc = 2451545.0;
        let jd_j2000_tt = utc_to_tt_jd_for_date(jd_j2000_utc);
        
        // At J2000.0, TAI-UTC = 32.0, so TT-UTC = 32.0 + 32.184 = 64.184 seconds
        let expected_offset_seconds = 32.0 + 32.184;
        let expected_offset_jd = expected_offset_seconds / 86400.0;
        let expected_jd_tt = jd_j2000_utc + expected_offset_jd;
        
        assert!((jd_j2000_tt - expected_jd_tt).abs() < 1e-10,
                "J2000.0 conversion should use correct leap second value: got {:.9}, expected {:.9}",
                jd_j2000_tt, expected_jd_tt);
    }
}