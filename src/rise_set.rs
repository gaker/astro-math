//! Rise, set, and transit time calculations for celestial objects.

use crate::{Location, julian_date, ra_dec_to_alt_az};
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};

/// Standard altitude for rise/set calculations (accounting for refraction and semi-diameter)
pub const RISE_SET_ALTITUDE: f64 = -0.5667; // -34 arcminutes

/// Sun's semi-diameter in degrees
pub const SUN_SEMI_DIAMETER: f64 = 0.2667; // 16 arcminutes

/// Calculates rise, transit, and set times for an object.
///
/// # Arguments
/// * `ra` - Right ascension in degrees
/// * `dec` - Declination in degrees
/// * `date` - Date to calculate for (uses noon UTC as reference)
/// * `location` - Observer's location
/// * `altitude_deg` - Altitude for rise/set (default: -0.5667° for refraction)
///
/// # Returns
/// Option of (rise_time, transit_time, set_time) or None if object is circumpolar/never rises
pub fn rise_transit_set(
    ra: f64,
    dec: f64,
    date: DateTime<Utc>,
    location: &Location,
    altitude_deg: Option<f64>,
) -> Option<(DateTime<Utc>, DateTime<Utc>, DateTime<Utc>)> {
    let target_alt = altitude_deg.unwrap_or(RISE_SET_ALTITUDE);
    let lat_rad = location.latitude_deg.to_radians();
    let dec_rad = dec.to_radians();
    
    // Calculate hour angle at rise/set
    let cos_h = -(target_alt.to_radians().sin() - lat_rad.sin() * dec_rad.sin()) 
        / (lat_rad.cos() * dec_rad.cos());
    
    // Check if object is circumpolar or never rises
    if cos_h < -1.0 {
        // Circumpolar (always above horizon)
        return None;
    } else if cos_h > 1.0 {
        // Never rises
        return None;
    }
    
    let h = cos_h.acos();
    let h_hours = h.to_degrees() / 15.0;
    
    // Calculate transit time (when object crosses meridian)
    let noon = Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 12, 0, 0).unwrap();
    let lst_noon = location.local_sidereal_time(noon);
    let ra_hours = ra / 15.0;
    
    // Time difference from noon to transit
    let mut transit_offset = ra_hours - lst_noon;
    if transit_offset < -12.0 {
        transit_offset += 24.0;
    } else if transit_offset > 12.0 {
        transit_offset -= 24.0;
    }
    
    // Convert sidereal hours to solar hours
    let transit_offset_solar = transit_offset * 0.99726956;
    let transit_time = noon + Duration::seconds((transit_offset_solar * 3600.0) as i64);
    
    // Calculate rise and set times
    let rise_offset = transit_offset_solar - h_hours * 0.99726956;
    let set_offset = transit_offset_solar + h_hours * 0.99726956;
    
    let rise_time = noon + Duration::seconds((rise_offset * 3600.0) as i64);
    let set_time = noon + Duration::seconds((set_offset * 3600.0) as i64);
    
    Some((rise_time, transit_time, set_time))
}

/// Calculates next rise time for an object.
///
/// # Arguments
/// * `ra` - Right ascension in degrees
/// * `dec` - Declination in degrees
/// * `start_time` - Time to start searching from
/// * `location` - Observer's location
/// * `altitude_deg` - Altitude for rise (default: -0.5667° for refraction)
///
/// # Returns
/// Option of next rise time, or None if object never rises
pub fn next_rise(
    ra: f64,
    dec: f64,
    start_time: DateTime<Utc>,
    location: &Location,
    altitude_deg: Option<f64>,
) -> Option<DateTime<Utc>> {
    // Check current altitude
    let (_current_alt, _) = ra_dec_to_alt_az(ra, dec, start_time, location);
    let _target_alt = altitude_deg.unwrap_or(RISE_SET_ALTITUDE);
    
    // Search for rise time over next 2 days
    let mut check_date = start_time.date_naive();
    for _ in 0..2 {
        let noon = Utc.from_utc_datetime(&check_date.and_hms_opt(12, 0, 0).unwrap());
        if let Some((rise, _, _)) = rise_transit_set(ra, dec, noon, location, altitude_deg) {
            if rise > start_time {
                return Some(rise);
            }
        }
        check_date = check_date.succ_opt().unwrap();
    }
    
    None
}

/// Calculates next set time for an object.
///
/// # Arguments
/// * `ra` - Right ascension in degrees
/// * `dec` - Declination in degrees
/// * `start_time` - Time to start searching from
/// * `location` - Observer's location
/// * `altitude_deg` - Altitude for set (default: -0.5667° for refraction)
///
/// # Returns
/// Option of next set time, or None if object never sets
pub fn next_set(
    ra: f64,
    dec: f64,
    start_time: DateTime<Utc>,
    location: &Location,
    altitude_deg: Option<f64>,
) -> Option<DateTime<Utc>> {
    // Search for set time over next 2 days
    let mut check_date = start_time.date_naive();
    for _ in 0..2 {
        let noon = Utc.from_utc_datetime(&check_date.and_hms_opt(12, 0, 0).unwrap());
        if let Some((_, _, set)) = rise_transit_set(ra, dec, noon, location, altitude_deg) {
            if set > start_time {
                return Some(set);
            }
        }
        check_date = check_date.succ_opt().unwrap();
    }
    
    None
}

/// Calculates sunrise and sunset times.
///
/// # Arguments
/// * `date` - Date to calculate for
/// * `location` - Observer's location
///
/// # Returns
/// Option of (sunrise, sunset) times
pub fn sun_rise_set(
    date: DateTime<Utc>,
    location: &Location,
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    // Approximate sun position (low precision)
    let jd = julian_date(date);
    let n = jd - 2451545.0;
    let l = (280.460 + 0.9856474 * n) % 360.0;
    let g = ((357.528 + 0.9856003 * n) % 360.0).to_radians();
    let lambda = l + 1.915 * g.sin() + 0.020 * (2.0 * g).sin();
    
    // Sun's RA and Dec
    let lambda_rad = lambda.to_radians();
    let epsilon = 23.439_f64.to_radians();
    let ra = lambda_rad.cos().atan2(epsilon.cos() * lambda_rad.sin()).to_degrees();
    let dec = (epsilon.sin() * lambda_rad.sin()).asin().to_degrees();
    
    // Account for sun's semi-diameter
    let sun_altitude = RISE_SET_ALTITUDE;
    
    if let Some((rise, _, set)) = rise_transit_set(ra, dec, date, location, Some(sun_altitude)) {
        Some((rise, set))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Location;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_circumpolar_star() {
        // Polaris from mid-northern latitude
        let location = Location {
            latitude_deg: 45.0,
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        let date = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
        let result = rise_transit_set(37.95, 89.26, date, &location, None);
        
        // Should be circumpolar (None)
        assert!(result.is_none());
    }

    #[test]
    fn test_never_rises() {
        // Southern star from northern latitude
        let location = Location {
            latitude_deg: 45.0,
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        let date = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
        let result = rise_transit_set(83.0, -70.0, date, &location, None);
        
        // Should never rise
        assert!(result.is_none());
    }

    #[test]
    fn test_normal_star() {
        // Vega from mid-latitude
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        let date = Utc.with_ymd_and_hms(2024, 8, 4, 12, 0, 0).unwrap();
        let result = rise_transit_set(279.23, 38.78, date, &location, None);
        
        assert!(result.is_some());
        let (rise, transit, set) = result.unwrap();
        
        // Basic sanity checks
        assert!(rise < transit);
        assert!(transit < set);
        assert!((set - rise).num_hours() > 5); // Vega should be up for several hours
    }

    #[test]
    fn test_sun_rise_set() {
        // Summer day at mid-latitude
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        let date = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
        let result = sun_rise_set(date, &location);
        
        assert!(result.is_some());
        let (sunrise, sunset) = result.unwrap();
        
        // Should have reasonable daylight hours
        let daylight_hours = (sunset - sunrise).num_hours();
        assert!(daylight_hours > 8 && daylight_hours < 18);
    }
}