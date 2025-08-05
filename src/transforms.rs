use crate::location::Location;
use crate::error::{Result, validate_ra, validate_dec};
use chrono::{DateTime, Utc};
use std::f64::consts::PI;

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
