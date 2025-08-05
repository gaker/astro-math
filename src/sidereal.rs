//! Sidereal time calculations for astronomical observations.
//!
//! This module provides functions to calculate sidereal time - the time system based on
//! Earth's rotation relative to the stars rather than the Sun. Sidereal time is essential
//! for telescope pointing and celestial coordinate conversions.
//!
//! # Overview
//!
//! While solar time (UTC) is based on the Sun's position, sidereal time tracks Earth's
//! rotation relative to distant stars. A sidereal day is about 23h 56m 4s, roughly 4
//! minutes shorter than a solar day.
//!
//! # Types of Sidereal Time
//!
//! - **Greenwich Mean Sidereal Time (GMST)**: Sidereal time at Greenwich meridian,
//!   based on Earth's uniform rotation
//! - **Local Mean Sidereal Time (LMST)**: GMST adjusted for observer's longitude
//! - **Apparent Sidereal Time**: True sidereal time including nutation effects
//!
//! # Applications
//!
//! - **Telescope Pointing**: Converting RA/Dec to Alt/Az requires local sidereal time
//! - **Meridian Transit**: Objects transit when their RA equals the LST
//! - **Hour Angle**: HA = LST - RA tells you where an object is relative to meridian
//!
//! # Example
//!
//! ```
//! use chrono::{Utc, TimeZone};
//! use astro_math::{julian_date, Location};
//!
//! let location = Location { 
//!     latitude_deg: 40.0, 
//!     longitude_deg: -74.0, 
//!     altitude_m: 0.0 
//! };
//! let dt = Utc::now();
//! let lst = location.local_sidereal_time(dt);
//! 
//! // Object at RA = LST is on the meridian (highest point)
//! println!("Current LST: {:.2} hours", lst);
//! ```

use crate::nutation::{mean_obliquity_arcsec, nutation_in_longitude_arcsec};

/// Computes the Greenwich Mean Sidereal Time (GMST) in fractional hours (0.0–24.0)
/// from a Julian Date (JD).
///
/// This function uses the IAU 1982 model as published in Jean Meeus’ *Astronomical Algorithms*
/// (2nd ed., Chapter 12), which gives sufficient precision for telescope guiding, pointing, and tracking.
///
/// The formula used is:
///
/// ```text
/// θ = 280.46061837 + 360.98564736629 × D
/// ```
///
/// Where:
/// - `θ` is the **GMST at Greenwich**, in **degrees**
/// - `D` is the number of days since the J2000.0 epoch: `D = JD - 2451545.0`
///
/// The result is then converted from degrees to hours by dividing by 15,
/// and normalized to the range `[0.0, 24.0)` to represent a sidereal time value.
///
/// ### Constants:
/// - `280.46061837°`: GMST at 2000 January 1, 12:00 UT (J2000.0)
/// - `360.98564736629°`: Earth’s mean sidereal rotation rate per solar day
/// - `15.0`: Number of degrees in one sidereal hour
///
/// ### Accuracy:
/// This approximation is good to ~0.1 seconds of time over several centuries,
/// making it suitable for astronomical control systems and star charting.
///
/// # Arguments
/// * `jd` - Julian Date, typically computed using [`julian_date`](crate::time::julian_date)
///
/// # Returns
/// GMST in fractional hours (e.g. `13.781` = 13h 46m 51s)
///
/// # Example
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time::julian_date;
/// use astro_math::sidereal::gmst;
///
/// let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
/// let jd = julian_date(dt);
/// let gmst = gmst(jd);
/// assert!((gmst - 8.5825).abs() < 1e-4);  // matches Meeus Example 11.a
/// ```
pub fn gmst(jd: f64) -> f64 {
    let d = jd - 2451545.0;
    let theta = 280.46061837 + 360.98564736629 * d;
    let mut hours = (theta / 15.0) % 24.0;
    if hours < 0.0 {
        hours += 24.0;
    }
    hours
}

/// Computes **Local Mean Sidereal Time** (LMST) in fractional hours (0.0–24.0)
/// from a Julian Date and a geographic longitude.
///
/// This function adds the observer’s longitude to the **Greenwich Mean Sidereal Time (GMST)**
/// to calculate the **Local Mean Sidereal Time** (LMST) — a standard value used in
/// telescope pointing models and mount alignment logic.
///
/// This does **not** include nutation or the equation of the equinoxes.
/// For that, see Apparent Sidereal Time.
///
/// # Arguments
///
/// - `jd` — Julian Date (e.g. from [`julian_date`](crate::time::julian_date))
/// - `longitude_deg` — Observer’s longitude in **degrees**, positive east of Greenwich, negative west
///
/// # Returns
///
/// Local **Mean** Sidereal Time in fractional hours, normalized to the range `[0.0, 24.0)`
///
/// # Notes
///
/// The formula used is:
///
/// ```text
/// LMST = GMST + (longitude in hours)
/// ```
///
/// Since Earth rotates 15° per hour, the longitude is divided by 15
/// to convert to sidereal time. This formulation is based on Meeus’
/// *Astronomical Algorithms* (2nd ed., Chapter 12).
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time::julian_date;
/// use astro_math::sidereal::local_mean_sidereal_time;
///
/// let dt = Utc.with_ymd_and_hms(1987, 4, 10, 19, 21, 0).unwrap();
/// let jd = julian_date(dt);
/// let local_sidereal = local_mean_sidereal_time(jd, -64.0);
/// assert!((local_sidereal - 4.317).abs() < 1e-2);
/// ```
pub fn local_mean_sidereal_time(jd: f64, longitude_deg: f64) -> f64 {
    let mut h = gmst(jd) + longitude_deg / 15.0;
    h %= 24.0;
    if h < 0.0 {
        h += 24.0;
    }
    h
}

/// Computes **Local Apparent Sidereal Time (LAST)** in fractional hours (0.0–24.0)
/// from a Julian Date and geographic longitude, using Meeus Ch. 22.
///
/// Apparent sidereal time includes the effect of nutation in longitude and
/// the equation of the equinoxes, and thus is more accurate than mean sidereal time.
///
/// # Arguments
///
/// - `jd`: Julian Date (e.g. from [`julian_date`](crate::time::julian_date))
/// - `longitude_deg`: Observer's longitude (degrees, east positive)
///
/// # Returns
///
/// Local **apparent** sidereal time in fractional hours, normalized to `[0.0, 24.0)`
///
/// # Notes
///
/// ```text
/// LAST = GMST + Equation of the Equinoxes + longitude / 15
///
/// Equation of Equinoxes = Δψ × cos(ε)
///   where:
///     Δψ = nutation in longitude (arcseconds)
///     ε = mean obliquity of the ecliptic (arcseconds)
/// ```
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::time::julian_date;
/// use astro_math::sidereal::apparent_sidereal_time;
///
/// let dt = Utc.with_ymd_and_hms(2024, 8, 4, 6, 0, 0).unwrap();
/// let jd = julian_date(dt);
/// let last = apparent_sidereal_time(jd, -111.6);
/// assert!(last >= 0.0 && last < 24.0);
/// ```
pub fn apparent_sidereal_time(jd: f64, longitude_deg: f64) -> f64 {
    let gmst_h = gmst(jd);

    // Nutation and obliquity (in arcseconds)
    let delta_psi = nutation_in_longitude_arcsec(jd);
    let epsilon = mean_obliquity_arcsec(jd);

    // Equation of the Equinoxes in degrees
    let eqeq_deg = (delta_psi * epsilon.to_radians().cos()) / 3600.0;

    // Add Equation of Equinoxes and observer longitude
    let mut last = gmst_h + eqeq_deg / 15.0 + longitude_deg / 15.0;
    last %= 24.0;
    if last < 0.0 {
        last += 24.0;
    }
    last
}
