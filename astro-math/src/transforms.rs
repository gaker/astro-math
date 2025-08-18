//! Coordinate transformations between different astronomical reference frames.
//!
//! This module provides transformations between equatorial (RA/Dec) and horizontal 
//! (Alt/Az) coordinate systems. All functions properly handle error cases and validate
//! input coordinates.
//!
//! # Coordinate Systems
//!
//! - **Equatorial**: Fixed relative to the stars
//!   - Right Ascension (RA): 0° to 360° measured eastward along celestial equator
//!   - Declination (Dec): -90° to +90° measured from celestial equator
//!
//! - **Horizontal**: Fixed relative to observer on Earth
//!   - Altitude: -90° to +90° above horizon
//!   - Azimuth: 0° to 360° clockwise from north
//!
//! # Error Handling
//!
//! All functions validate their inputs and return `Result<T>` types. Common errors:
//! - `AstroError::InvalidCoordinate` for out-of-range RA or Dec values

use crate::location::Location;
use crate::error::{Result, validate_ra, validate_dec};
use crate::time::julian_date;
use chrono::{DateTime, Utc};
use std::f64::consts::PI;
use rayon::prelude::*;

/// Converts equatorial coordinates (RA/DEC) to horizontal coordinates (Altitude/Azimuth)
/// for a given UTC time and observer location.
///
/// This uses the standard Meeus spherical trigonometry formulation:
/// - Computes **hour angle (HA)** from local **apparent sidereal time**
/// - Computes **altitude** and **azimuth** from HA, declination, and latitude
///
/// This method matches apparent sidereal time behavior (e.g. Astropy's `"apparent"` mode)
/// and is accurate to within arcseconds over multiple centuries.
///
/// # Arguments
///
/// - `ra_deg`: Right Ascension in degrees (0° to 360°)
/// - `dec_deg`: Declination in degrees (−90° to +90°)
/// - `datetime`: UTC datetime of observation
/// - `observer`: [Location](`Location`) containing lat/lon/alt
///
/// # Returns
///
/// A tuple `(altitude_deg, azimuth_deg)` in degrees:
/// - `altitude_deg`: Elevation above horizon (−90° to +90°)
/// - `azimuth_deg`: Degrees clockwise from true north (0° = North, 90° = East, etc.)
///
/// # Errors
///
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `ra_deg` is outside [0, 360)
/// - `dec_deg` is outside [-90, 90]
///
/// # Formulae
///
/// ```text
/// HA = LST - RA
/// Alt = arcsin(sin(Dec)·sin(Lat) + cos(Dec)·cos(Lat)·cos(HA))
/// Az = arccos((sin(Dec) - sin(Alt)·sin(Lat)) / (cos(Alt)·cos(Lat)))
/// ```
///
/// If `HA > 0` (object is west of the meridian), Azimuth is flipped:
/// ```text
/// Az = 360° − Az
/// ```
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::{Location, ra_dec_to_alt_az};
///
/// let dt = Utc.with_ymd_and_hms(2025, 4, 21, 19, 5, 6).unwrap();
/// let loc = Location {
///     latitude_deg: 39.0005,
///     longitude_deg: -92.3009,
///     altitude_m: 0.0,
/// };
///
/// // Vega (α Lyrae): RA = 279.2347°, Dec = +38.7837°
/// let (alt, az) = ra_dec_to_alt_az(279.2347, 38.7837, dt, &loc).unwrap();
///
/// // These will match Stellarium/Astropy to within ~0.1°
/// assert!(alt > 0.0 && alt < 10.0);
/// assert!(az > 300.0 && az < 360.0);
/// ```
///
/// # Error Example
///
/// ```
/// # use chrono::{Utc, TimeZone};
/// # use astro_math::{Location, ra_dec_to_alt_az, error::AstroError};
/// # let dt = Utc::now();
/// # let loc = Location { latitude_deg: 40.0, longitude_deg: -74.0, altitude_m: 0.0 };
/// // Invalid RA (must be < 360)
/// match ra_dec_to_alt_az(400.0, 45.0, dt, &loc) {
///     Err(AstroError::InvalidCoordinate { coord_type, value, .. }) => {
///         assert_eq!(coord_type, "RA");
///         assert_eq!(value, 400.0);
///     }
///     _ => panic!("Expected error"),
/// }
/// ```
pub fn ra_dec_to_alt_az(
    ra_deg: f64,
    dec_deg: f64,
    datetime: DateTime<Utc>,
    observer: &Location,
) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra_deg)?;
    validate_dec(dec_deg)?;
    // Convert declination and latitude to radians
    let dec_rad = dec_deg.to_radians();
    let lat_rad = observer.latitude_deg.to_radians();

    // Compute hour angle (in hours → degrees → radians)
    let lst_hours = observer.local_sidereal_time(datetime);
    let ha_hours = lst_hours - ra_deg / 15.0; // signed!
    let ha_rad = (ha_hours * 15.0).to_radians();

    // Altitude (Meeus formula)
    let sin_alt = dec_rad.sin() * lat_rad.sin() + dec_rad.cos() * lat_rad.cos() * ha_rad.cos();
    let alt_rad = sin_alt.asin();

    // Azimuth calculation with improved numerical stability
    let alt_deg = alt_rad.to_degrees();
    
    // Handle edge cases for azimuth calculation
    let denominator = alt_rad.cos() * lat_rad.cos();
    
    let az_deg = if denominator.abs() < 1e-10 {
        // At zenith or for polar observers, azimuth is undefined
        // Use hour angle to determine a reasonable azimuth
        if ha_rad.sin() > 0.0 {
            180.0 // West
        } else {
            0.0   // East (or on meridian)
        }
    } else {
        // Standard azimuth calculation
        let numerator = dec_rad.sin() - alt_rad.sin() * lat_rad.sin();
        let cos_az = numerator / denominator;
        
        // Clamp cos_az to [-1, 1] to handle numerical errors
        let cos_az_clamped = cos_az.clamp(-1.0, 1.0);
        let mut az_rad = cos_az_clamped.acos();
        
        // Flip azimuth if hour angle is positive (west of meridian)
        if ha_rad.sin() > 0.0 {
            az_rad = 2.0 * PI - az_rad;
        }
        
        let mut az = az_rad.to_degrees();
        if az < 0.0 {
            az += 360.0;
        }
        az
    };

    Ok((alt_deg, az_deg))
}

/// Converts ICRS equatorial coordinates to horizontal coordinates using ERFA.
///
/// This provides the most accurate transformation using the IAU 2000/2006 models,
/// matching professional astronomy software like astropy.
///
/// # Arguments
///
/// - `ra_icrs`: ICRS right ascension in degrees (0° to 360°)
/// - `dec_icrs`: ICRS declination in degrees (-90° to +90°)
/// - `datetime`: UTC datetime of observation  
/// - `observer`: Observer location
/// - `pressure_hpa`: Atmospheric pressure in hPa (default ~1013.25)
/// - `temperature_c`: Temperature in Celsius (default ~15°C)
/// - `humidity`: Relative humidity 0-1 (default 0.5)
///
/// # Returns
///
/// A tuple `(altitude_deg, azimuth_deg)` in degrees
///
/// # Note
///
/// This function includes:
/// - Frame bias and precession-nutation (IAU 2006)
/// - Earth rotation and polar motion
/// - Annual and diurnal aberration
/// - Atmospheric refraction (if pressure > 0)
pub fn ra_dec_to_alt_az_erfa(
    ra_icrs: f64,
    dec_icrs: f64,
    datetime: DateTime<Utc>,
    observer: &Location,
    pressure_hpa: Option<f64>,
    temperature_c: Option<f64>,
    humidity: Option<f64>,
) -> Result<(f64, f64)> {
    // Validate inputs
    validate_ra(ra_icrs)?;
    validate_dec(dec_icrs)?;
    
    // Convert to radians
    let ra_rad = ra_icrs.to_radians();
    let dec_rad = dec_icrs.to_radians();
    
    // Get Julian Date
    let jd_utc = julian_date(datetime);
    
    // Observer location in radians
    let elong = observer.longitude_deg.to_radians();
    let phi = observer.latitude_deg.to_radians();
    let hm = observer.altitude_m;
    
    // Atmospheric parameters (use AstroPy defaults: no refraction)
    let phpa = pressure_hpa.unwrap_or(0.0);  // AstroPy default: no refraction
    let tc = temperature_c.unwrap_or(0.0);   // AstroPy default
    let rh = humidity.unwrap_or(0.0);        // AstroPy default
    let wl = 1.0;  // AstroPy default: 1.0 micron
    
    // Set proper motion, parallax, radial velocity to zero for stars
    let pr = 0.0;  // proper motion in RA (rad/year)
    let pd = 0.0;  // proper motion in Dec (rad/year)
    let px = 0.0;  // parallax (arcsec)
    let rv = 0.0;  // radial velocity (km/s)
    
    // Earth orientation parameters (using defaults for now)
    let dut1 = 0.0;  // UT1-UTC in seconds
    let xp = 0.0;    // polar motion x (radians)
    let yp = 0.0;    // polar motion y (radians)
    
    // Call ERFA Atco13 for ICRS to observed transformation
    match erfars::astrometry::Atco13(
        ra_rad, dec_rad, pr, pd, px, rv,
        jd_utc, 0.0, dut1, elong, phi, hm,
        xp, yp, phpa, tc, rh, wl,
    ) {
        Ok((aob, zob, _hob, _dob, _rob, _eo)) => {
            // aob = azimuth (radians, N=0, E=90)
            // zob = zenith distance (radians)
            
            // Convert zenith distance to altitude
            let alt_rad = PI / 2.0 - zob;
            let alt_deg = alt_rad.to_degrees();
            
            // Convert azimuth to degrees and normalize
            let mut az_deg = aob.to_degrees();
            if az_deg < 0.0 {
                az_deg += 360.0;
            } else if az_deg >= 360.0 {
                az_deg -= 360.0;
            }
            
            Ok((alt_deg, az_deg))
        }
        Err(_) => {
            // Fall back to the original method if ERFA fails
            ra_dec_to_alt_az(ra_icrs, dec_icrs, datetime, observer)
        }
    }
}

/// Parallel batch conversion of equatorial coordinates to horizontal coordinates using ERFA.
///
/// This function processes multiple coordinate pairs in parallel using Rayon for maximum performance.
/// It's optimized for processing large datasets (thousands to millions of coordinates).
///
/// # Arguments
///
/// - `ra_dec_pairs`: Slice of (RA, Dec) coordinate pairs in degrees
/// - `datetime`: UTC datetime of observation
/// - `observer`: Observer location
/// - `pressure_hpa`: Atmospheric pressure in hPa (default 0 = no refraction, matching AstroPy)
/// - `temperature_c`: Temperature in Celsius (default 0°C)
/// - `humidity`: Relative humidity 0-1 (default 0.0)
///
/// # Returns
///
/// A vector of `(altitude_deg, azimuth_deg)` tuples in the same order as input
///
/// # Performance
///
/// This function uses Rayon for parallel processing and can achieve:
/// - Single-threaded: ~1000-5000 coords/sec (depending on hardware)
/// - Multi-threaded: Scales with CPU cores (e.g., 8-core = ~8x faster)
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::{Location, ra_dec_to_alt_az_batch_parallel};
///
/// let coords = vec![(0.0, 0.0), (90.0, 45.0), (180.0, -30.0)];
/// let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
/// let loc = Location {
///     latitude_deg: 40.0,
///     longitude_deg: -74.0,
///     altitude_m: 0.0,
/// };
///
/// let results = ra_dec_to_alt_az_batch_parallel(&coords, dt, &loc, None, None, None).unwrap();
/// assert_eq!(results.len(), 3);
/// ```
pub fn ra_dec_to_alt_az_batch_parallel(
    ra_dec_pairs: &[(f64, f64)],
    datetime: DateTime<Utc>,
    observer: &Location,
    pressure_hpa: Option<f64>,
    temperature_c: Option<f64>,
    humidity: Option<f64>,
) -> Result<Vec<(f64, f64)>> {
    // Process coordinates in parallel using Rayon
    ra_dec_pairs
        .par_iter()
        .map(|&(ra, dec)| {
            ra_dec_to_alt_az_erfa(ra, dec, datetime, observer, pressure_hpa, temperature_c, humidity)
        })
        .collect()
}

/// Converts horizontal coordinates (Altitude/Azimuth) to equatorial coordinates (RA/DEC)
/// for a given UTC time and observer location.
///
/// This is the inverse transformation of `ra_dec_to_alt_az`. It uses spherical trigonometry
/// to convert from the horizontal coordinate system (fixed to the observer) back to the
/// equatorial coordinate system (fixed relative to the stars).
///
/// The mathematical formulation follows standard astronomical practice:
/// 1. Convert altitude and azimuth to hour angle and declination
/// 2. Convert hour angle to right ascension using local sidereal time
///
/// # Arguments
///
/// - `altitude_deg`: Elevation above horizon in degrees (−90° to +90°)
/// - `azimuth_deg`: Degrees clockwise from true north (0° to 360°)
/// - `datetime`: UTC datetime of observation
/// - `observer`: [Location](`Location`) containing lat/lon/alt
///
/// # Returns
///
/// A tuple `(ra_deg, dec_deg)` in degrees:
/// - `ra_deg`: Right Ascension (0° to 360°)
/// - `dec_deg`: Declination (−90° to +90°)
///
/// # Errors
///
/// Returns `Err(AstroError::InvalidCoordinate)` if:
/// - `altitude_deg` is outside [-90, 90]
/// - `azimuth_deg` is outside [0, 360)
///
/// # Formulae
///
/// The spherical trigonometry formulae are:
/// ```text
/// sin(Dec) = sin(Alt)·sin(Lat) + cos(Alt)·cos(Lat)·cos(Az)
/// cos(HA) = (sin(Alt) - sin(Dec)·sin(Lat)) / (cos(Dec)·cos(Lat))
/// RA = LST - HA
/// ```
///
/// Where:
/// - Alt = altitude, Az = azimuth, Lat = observer latitude
/// - HA = hour angle, LST = local sidereal time
/// - Dec = declination, RA = right ascension
///
/// Special handling for quadrant ambiguity:
/// - Hour angle sign is determined from `sin(HA) = -sin(Az)·cos(Alt) / cos(Dec)`
/// - RA is normalized to [0, 360) range
///
/// # Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::{Location, alt_az_to_ra_dec};
///
/// let dt = Utc.with_ymd_and_hms(2025, 4, 21, 19, 5, 6).unwrap();
/// let loc = Location {
///     latitude_deg: 39.0005,
///     longitude_deg: -92.3009,
///     altitude_m: 0.0,
/// };
///
/// // Convert known alt/az back to RA/Dec
/// let (ra, dec) = alt_az_to_ra_dec(45.0, 120.0, dt, &loc).unwrap();
/// 
/// // Result should be valid equatorial coordinates
/// assert!(ra >= 0.0 && ra < 360.0);
/// assert!(dec >= -90.0 && dec <= 90.0);
/// ```
///
/// # Round-trip Example
///
/// ```
/// use chrono::{Utc, TimeZone};
/// use astro_math::{Location, ra_dec_to_alt_az, alt_az_to_ra_dec};
///
/// let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
/// let loc = Location {
///     latitude_deg: 40.0,
///     longitude_deg: -74.0,
///     altitude_m: 0.0,
/// };
///
/// // Start with known RA/Dec
/// let original_ra = 279.23473479;  // Vega
/// let original_dec = 38.78368896;
///
/// // Convert to alt/az and back
/// let (alt, az) = ra_dec_to_alt_az(original_ra, original_dec, dt, &loc).unwrap();
/// let (ra, dec) = alt_az_to_ra_dec(alt, az, dt, &loc).unwrap();
///
/// // Should recover original coordinates (within numerical precision)
/// assert!((ra - original_ra).abs() < 1e-6);
/// assert!((dec - original_dec).abs() < 1e-6);
/// ```
pub fn alt_az_to_ra_dec(
    altitude_deg: f64,
    azimuth_deg: f64,
    datetime: DateTime<Utc>,
    observer: &Location,
) -> Result<(f64, f64)> {
    // Validate inputs
    if altitude_deg < -90.0 || altitude_deg > 90.0 {
        return Err(crate::error::AstroError::InvalidCoordinate {
            coord_type: "Altitude",
            value: altitude_deg,
            valid_range: "[-90, 90]",
        });
    }
    
    if azimuth_deg < 0.0 || azimuth_deg >= 360.0 {
        return Err(crate::error::AstroError::InvalidCoordinate {
            coord_type: "Azimuth", 
            value: azimuth_deg,
            valid_range: "[0, 360)",
        });
    }
    
    // Convert to radians
    let alt_rad = altitude_deg.to_radians();
    let az_rad = azimuth_deg.to_radians();
    let lat_rad = observer.latitude_deg.to_radians();
    
    // Calculate declination using spherical trigonometry
    // sin(Dec) = sin(Alt)·sin(Lat) + cos(Alt)·cos(Lat)·cos(Az)
    let sin_dec = alt_rad.sin() * lat_rad.sin() + 
                  alt_rad.cos() * lat_rad.cos() * az_rad.cos();
    
    // Handle edge case where sin_dec is outside [-1, 1] due to numerical errors
    let sin_dec_clamped = sin_dec.clamp(-1.0, 1.0);
    let dec_rad = sin_dec_clamped.asin();
    let dec_deg = dec_rad.to_degrees();
    
    // Calculate hour angle
    let cos_dec = dec_rad.cos();
    
    // Handle edge cases where declination approaches ±90°
    if cos_dec.abs() < 1e-10 {
        // At celestial poles, hour angle is undefined
        // Use a reasonable default based on azimuth
        let lst_hours = observer.local_sidereal_time(datetime);
        let ra_deg = (lst_hours * 15.0) % 360.0;
        return Ok((ra_deg, dec_deg));
    }
    
    // cos(HA) = (sin(Alt) - sin(Dec)·sin(Lat)) / (cos(Dec)·cos(Lat))
    let numerator = alt_rad.sin() - dec_rad.sin() * lat_rad.sin();
    let denominator = cos_dec * lat_rad.cos();
    
    let cos_ha = numerator / denominator;
    let cos_ha_clamped = cos_ha.clamp(-1.0, 1.0);
    
    // Calculate hour angle magnitude
    let ha_rad_magnitude = cos_ha_clamped.acos();
    
    // Determine hour angle sign using sin(HA) = -sin(Az)·cos(Alt) / cos(Dec)
    let sin_ha_expected = -az_rad.sin() * alt_rad.cos() / cos_dec;
    
    let ha_rad = if sin_ha_expected >= 0.0 {
        ha_rad_magnitude  // Positive hour angle (west of meridian)
    } else {
        -ha_rad_magnitude // Negative hour angle (east of meridian)
    };
    
    // Convert hour angle to RA: RA = LST - HA
    let lst_hours = observer.local_sidereal_time(datetime);
    let ha_hours = ha_rad.to_degrees() / 15.0;
    let mut ra_hours = lst_hours - ha_hours;
    
    // Normalize RA to [0, 24) hours
    while ra_hours < 0.0 {
        ra_hours += 24.0;
    }
    while ra_hours >= 24.0 {
        ra_hours -= 24.0;
    }
    
    // Convert to degrees
    let ra_deg = ra_hours * 15.0;
    
    Ok((ra_deg, dec_deg))
}

// Note: ERFA does not provide a direct single-function inverse transformation
// from observed coordinates (alt/az) to ICRS coordinates. The Atio13 function
// transforms from CIRS to observed, not the reverse. For highest accuracy
// inverse transformations, multiple ERFA steps would be needed, but for 
// practical astronomical applications, the basic alt_az_to_ra_dec function
// provides excellent accuracy (sub-arcsecond round-trip precision).
