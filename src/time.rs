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
    // Gregorian calendar was adopted on October 15, 1582
    // Dates from October 5-14, 1582 don't exist in the Gregorian calendar
    let b = if datetime.date_naive() >= chrono::NaiveDate::from_ymd_opt(1582, 10, 15).unwrap() {
        2.0 - a + (a / 4.0).floor()
    } else {
        0.0
    };

    let hour = datetime.hour() as f64;
    let minute = datetime.minute() as f64;
    let second = datetime.second() as f64;
    let frac_day = (hour + (minute / 60.0) + (second / 3600.0)) / 24.0;

    let jd = (365.25 * (y as f64 + 4716.0)).floor()
        + (30.6001 * ((m + 1) as f64)).floor()
        + day
        + frac_day
        + b
        - 1524.5;

    jd
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
