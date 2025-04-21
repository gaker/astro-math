use crate::time::julian_date;
use crate::{local_mean_sidereal_time, sidereal::apparent_sidereal_time};
use chrono::{DateTime, Utc};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid DMS format"),
            ParseError::InvalidNumber => write!(f, "Invalid number in DMS string"),
        }
    }
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
    /// `Ok(Location)` if parsing succeeds, or a `ParseError` if the input is invalid.
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
    /// let result = Location::from_dms("foo", "bar", 100.0);
    /// assert!(result.is_err());
    /// ```
    pub fn from_dms(lat_str: &str, lon_str: &str, alt_m: f64) -> Result<Self, ParseError> {
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

//
fn parse_dms(s: &str) -> Result<f64, ParseError> {
    // Accepts: "+39 00 01.7", "-92 18 03.2", "39:00:01.7"
    let s = s
        .trim()
        .replace('°', " ")
        .replace('\'', " ")
        .replace(':', " ")
        .replace('"', " ");

    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidFormat);
    }

    let sign = if parts[0].starts_with('-') || s.starts_with('-') {
        -1.0
    } else {
        1.0
    };

    let d = f64::from_str(parts[0].trim_start_matches(|c| c == '+' || c == '-'))
        .map_err(|_| ParseError::InvalidNumber)?;
    let m = f64::from_str(parts.get(1).unwrap_or(&"0")).map_err(|_| ParseError::InvalidNumber)?;
    let s = f64::from_str(parts.get(2).unwrap_or(&"0")).map_err(|_| ParseError::InvalidNumber)?;

    Ok(sign * (d + m / 60.0 + s / 3600.0))
}
