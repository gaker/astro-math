//! Time conversions and Julian Date calculations.
//!
//! This module provides essential time conversion functions for astronomical calculations,
//! primarily focused on Julian Dates - the standard continuous time system used in astronomy.
//!
//! # Julian Dates
//!
//! The Julian Date (JD) system is a continuous count of days since noon UTC on January 1, 4713 BCE
//! in the proleptic Julian calendar. It provides several advantages for astronomical calculations:
//!
//! - **Continuous**: No gaps, leap years, or calendar reforms to worry about
//! - **Decimal**: Fractional days represent time of day (0.5 = midnight UTC)
//! - **Universal**: Used by all major ephemerides and astronomical software
//! - **Precise**: Can represent any moment in recorded history
//!
//! # Key Epochs
//!
//! - **JD 0**: January 1, 4713 BCE at noon UTC (Julian calendar)
//! - **JD 2451545.0**: J2000.0 epoch (January 1, 2000 at noon TT)
//! - **Modified Julian Date (MJD)**: JD - 2400000.5 (starts at midnight)
//!
//! # Examples
//!
//! ```
//! use chrono::{Utc, TimeZone};
//! use astro_math::time::{julian_date, j2000_days};
//!
//! // Convert current time to Julian Date
//! let now = Utc::now();
//! let jd = julian_date(now);
//! println!("Current Julian Date: {:.5}", jd);
//!
//! // Days since J2000.0
//! let days = j2000_days(now);
//! println!("Days since J2000.0: {:.5}", days);
//! ```

use chrono::{DateTime, Datelike, Timelike, Utc};

/// Julian Date (JD) of the J2000.0 epoch: 2000 January 1.5 TT.
///
/// This is the standard reference epoch for modern astronomical calculations.
/// Most star catalogs, ephemerides, and orbital elements are referenced to this epoch.
pub const JD2000: f64 = 2451545.0;

/// Converts a UTC datetime to a Julian Date (JD).
///
/// Julian Dates are a continuous count of days since noon UTC on **January 1, 4713 BCE**
/// in the Julian calendar. They are the standard timekeeping format for ephemerides,
/// astronomical observations, and sidereal calculations.
///
/// This implementation is based on the algorithm from Jean Meeus’ *Astronomical Algorithms*
/// (2nd ed., Chapter 7), and is accurate for all Gregorian calendar dates (1582–present).
///
/// # Arguments
///
/// - `datetime` — A UTC [`DateTime<Utc>`] representing the moment in time to convert
///
/// # Returns
///
/// A `f64` representing the Julian Date, with fractional days included.
///
/// # Notes
///
/// The Julian Day starts at **noon**, so:
/// - `2000-01-01 12:00:00 UTC` → `2451545.0` (start of J2000.0)
/// - `2000-01-01 00:00:00 UTC` → `2451544.5`
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time::julian_date;
///
/// let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
/// let jd = julian_date(dt);
/// assert!((jd - 2451545.0).abs() < 1e-6);
/// ```
pub fn julian_date(datetime: DateTime<Utc>) -> f64 {
    let year = datetime.year();
    let month = datetime.month();
    let day = datetime.day() as f64;

    let mut y = year;
    let mut m = month as i32;

    if m <= 2 {
        y -= 1;
        m += 12;
    }

    let a = (y as f64 / 100.0).floor();
    
    // Proleptic Gregorian calendar approach (matches astropy/ERFA)
    // Always apply the Gregorian leap year correction
    let b = 2.0 - a + (a / 4.0).floor();

    let hour = datetime.hour() as f64;
    let minute = datetime.minute() as f64;
    let second = datetime.second() as f64;
    let frac_day = (hour + (minute / 60.0) + (second / 3600.0)) / 24.0;

    (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * ((m + 1) as f64)).floor()
        + day
        + frac_day
        + b
        - 1524.5
}

/// Computes the number of days since the J2000.0 epoch (`JD2000`).
///
/// This is useful as a normalized timescale for many astronomical calculations,
/// including precession, nutation, solar/lunar position modeling, and sidereal time.
///
/// # Arguments
///
/// - `datetime` — A UTC datetime to measure from J2000
///
/// # Returns
///
/// A `f64` representing the number of **days since 2000-01-01 12:00:00 UTC**
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time::{julian_date, j2000_days};
///
/// let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
/// let delta = j2000_days(dt);
///
/// let jd = julian_date(dt);
/// assert!((jd - (2451545.0 + delta)).abs() < 1e-6);
/// ```
pub fn j2000_days(datetime: DateTime<Utc>) -> f64 {
    julian_date(datetime) - JD2000
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_calendar_reform_period_1582() {
        // Critical test cases for the Gregorian calendar reform in October 1582
        // These values are verified against astropy and match the proleptic Gregorian approach
        
        let test_cases = [
            // Before the reform (Julian calendar dates)
            (1582, 10, 1, 12, 0, 0, 2299147.0, "Oct 1, 1582"),
            (1582, 10, 3, 12, 0, 0, 2299149.0, "Oct 3, 1582"),
            (1582, 10, 4, 12, 0, 0, 2299150.0, "Last Julian day"),
            
            // The historically non-existent dates (Oct 5-14, 1582)
            // In proleptic Gregorian calendar, these have sequential JDs
            (1582, 10, 5, 12, 0, 0, 2299151.0, "Invalid historical date"),
            (1582, 10, 10, 12, 0, 0, 2299156.0, "Invalid historical date"),
            (1582, 10, 14, 12, 0, 0, 2299160.0, "Invalid historical date"),
            
            // After the reform (Gregorian calendar dates)
            (1582, 10, 15, 12, 0, 0, 2299161.0, "First Gregorian day"),
            (1582, 10, 16, 12, 0, 0, 2299162.0, "Day after reform"),
            (1582, 10, 31, 12, 0, 0, 2299177.0, "Later Oct 1582"),
            
            // Additional edge cases
            (1582, 11, 1, 12, 0, 0, 2299178.0, "Nov 1582"),
            (1583, 1, 1, 12, 0, 0, 2299239.0, "Jan 1583"),
        ];
        
        for (year, month, day, hour, min, sec, expected_jd, description) in test_cases {
            let dt = Utc.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap();
            let calculated_jd = julian_date(dt);
            
            let diff_seconds = (calculated_jd - expected_jd).abs() * 86400.0;
            assert!(
                diff_seconds < 0.001,
                "Failed for {}: expected JD {}, got {}, diff = {:.6} seconds",
                description, expected_jd, calculated_jd, diff_seconds
            );
        }
    }
    
    #[test]
    fn test_calendar_transition_gap() {
        // Verify the 11-day gap between last Julian and first Gregorian dates
        let oct_4 = Utc.with_ymd_and_hms(1582, 10, 4, 12, 0, 0).unwrap();
        let oct_15 = Utc.with_ymd_and_hms(1582, 10, 15, 12, 0, 0).unwrap();
        
        let jd_4 = julian_date(oct_4);
        let jd_15 = julian_date(oct_15);
        let gap = jd_15 - jd_4;
        
        // Should be exactly 11 days difference
        assert!((gap - 11.0).abs() < 0.001, 
               "Calendar transition gap should be 11 days, got {:.3}", gap);
    }
    
    #[test]
    fn test_j2000_epoch() {
        // Verify the J2000.0 epoch is correct
        let j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let jd = julian_date(j2000);
        
        assert!((jd - JD2000).abs() < 1e-9, 
               "J2000.0 epoch should be exactly {}, got {}", JD2000, jd);
    }
    
    #[test]
    fn test_j2000_days() {
        // Test days since J2000.0 calculation
        let test_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let days = j2000_days(test_date);
        let jd = julian_date(test_date);
        
        // Should match: jd = JD2000 + days
        assert!((jd - (JD2000 + days)).abs() < 1e-9,
               "j2000_days calculation inconsistent with julian_date");
    }
}
