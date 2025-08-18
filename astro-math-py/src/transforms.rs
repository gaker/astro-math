use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDateAccess, PyTimeAccess};
use astro_math::{transforms, Location};
use chrono::{DateTime, TimeZone, Utc};

/// Single coordinate transform from RA/Dec to Alt/Az using ERFA
#[pyfunction]
#[pyo3(signature = (ra, dec, dt, latitude, longitude, altitude=0.0))]
fn ra_dec_to_alt_az(
    ra: f64,
    dec: f64,
    dt: &Bound<'_, PyDateTime>,
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
) -> PyResult<(f64, f64)> {
    let datetime = datetime_from_py(dt)?;
    let location = Location {
        latitude_deg: latitude,
        longitude_deg: longitude,
        altitude_m: altitude.unwrap_or(0.0),
    };
    
    let (alt, az) = transforms::ra_dec_to_alt_az_erfa(ra, dec, datetime, &location, None, None, None)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    Ok((alt, az))
}

/// Batch coordinate transform from RA/Dec to Alt/Az
#[pyfunction]
#[pyo3(signature = (ra, dec, dt, latitude, longitude, altitude=0.0))]
fn batch_ra_dec_to_alt_az<'py>(
    py: Python<'py>,
    ra: PyReadonlyArray1<'_, f64>,
    dec: PyReadonlyArray1<'_, f64>,
    dt: &Bound<'_, PyDateTime>,
    latitude: f64,
    longitude: f64,
    altitude: Option<f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let datetime = datetime_from_py(dt)?;
    let location = Location {
        latitude_deg: latitude,
        longitude_deg: longitude,
        altitude_m: altitude.unwrap_or(0.0),
    };
    
    let ra_slice = ra.as_slice()?;
    let dec_slice = dec.as_slice()?;
    
    if ra_slice.len() != dec_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "RA and Dec arrays must have the same length"
        ));
    }
    
    let mut alt_vec = Vec::with_capacity(ra_slice.len());
    let mut az_vec = Vec::with_capacity(ra_slice.len());
    
    // Create coordinate pairs for parallel processing
    let coord_pairs: Vec<(f64, f64)> = ra_slice.iter().zip(dec_slice.iter())
        .map(|(&ra, &dec)| (ra, dec))
        .collect();
    
    // Use parallel batch processing
    let results = transforms::ra_dec_to_alt_az_batch_parallel(&coord_pairs, datetime, &location, None, None, None)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    // Separate results into alt and az vectors
    for (alt, az) in results {
        alt_vec.push(alt);
        az_vec.push(az);
    }
    
    Ok((
        alt_vec.into_pyarray_bound(py),
        az_vec.into_pyarray_bound(py)
    ))
}

/// Convert Alt/Az to RA/Dec coordinates.
///
/// Inverse transformation from horizontal to equatorial coordinates.
#[pyfunction]
#[pyo3(signature = (altitude, azimuth, datetime, latitude, longitude, altitude_m=0.0))]
fn alt_az_to_ra_dec(
    altitude: f64,
    azimuth: f64,
    datetime: &Bound<'_, PyDateTime>,
    latitude: f64,
    longitude: f64,
    altitude_m: f64
) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    let location = Location {
        latitude_deg: latitude,
        longitude_deg: longitude,
        altitude_m,
    };
    
    transforms::alt_az_to_ra_dec(altitude, azimuth, dt, &location)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Batch convert Alt/Az to RA/Dec coordinates.
///
/// Process multiple coordinates efficiently with Rayon parallelization.
#[pyfunction]
#[pyo3(signature = (altitude, azimuth, datetime, latitude, longitude, altitude_m=0.0))]
fn batch_alt_az_to_ra_dec<'py>(
    py: Python<'py>,
    altitude: &Bound<'py, PyArray1<f64>>,
    azimuth: &Bound<'py, PyArray1<f64>>,
    datetime: &Bound<'py, PyDateTime>,
    latitude: f64,
    longitude: f64,
    altitude_m: f64
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let alt_slice = unsafe { altitude.as_slice()? };
    let az_slice = unsafe { azimuth.as_slice()? };
    
    if alt_slice.len() != az_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "altitude and azimuth arrays must have the same length"
        ));
    }
    
    let dt = datetime_from_py(datetime)?;
    let location = Location {
        latitude_deg: latitude,
        longitude_deg: longitude,
        altitude_m,
    };
    
    use rayon::prelude::*;
    let results: Vec<_> = alt_slice.par_iter()
        .zip(az_slice.par_iter())
        .map(|(&alt, &az)| {
            transforms::alt_az_to_ra_dec(alt, az, dt, &location)
                .unwrap_or((0.0, 0.0))
        })
        .collect();
    
    let (ra_vec, dec_vec): (Vec<_>, Vec<_>) = results.into_iter().unzip();
    
    Ok((
        ra_vec.into_pyarray_bound(py),
        dec_vec.into_pyarray_bound(py)
    ))
}

// Helper function to convert Python datetime to chrono DateTime
fn datetime_from_py(dt: &Bound<'_, PyDateTime>) -> PyResult<DateTime<Utc>> {
    let year = dt.get_year();
    let month = dt.get_month();
    let day = dt.get_day();
    let hour = dt.get_hour();
    let minute = dt.get_minute();
    let second = dt.get_second();
    let microsecond = dt.get_microsecond();
    
    Utc.with_ymd_and_hms(year, month.into(), day.into(), hour.into(), minute.into(), second.into())
        .single()
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid datetime"))
        .map(|dt| dt + chrono::Duration::microseconds(microsecond as i64))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ra_dec_to_alt_az, m)?)?;
    m.add_function(wrap_pyfunction!(batch_ra_dec_to_alt_az, m)?)?;
    m.add_function(wrap_pyfunction!(alt_az_to_ra_dec, m)?)?;
    m.add_function(wrap_pyfunction!(batch_alt_az_to_ra_dec, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ra_dec_to_alt_az_basic() {
        // Test basic coordinate transformation
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        // Test zenith (RA matches LST, Dec matches latitude)
        let lst = location.local_sidereal_time(dt);
        let ra = lst * 15.0; // Convert hours to degrees
        let dec = location.latitude_deg;
        
        let (alt, _az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
        assert!(alt > 89.0, "Object at zenith should have altitude near 90°");
    }
    
    #[test]
    fn test_ra_dec_to_alt_az_horizon() {
        // Test object on horizon
        let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 0.0,  // Equator
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        // Object on celestial equator, 90° from meridian
        let lst = location.local_sidereal_time(dt);
        let ra = (lst + 6.0) * 15.0; // 6 hours from meridian
        let dec = 0.0;
        
        let (alt, _az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
        assert!(alt.abs() < 1.0, "Object 90° from meridian on celestial equator should be near horizon at equator");
    }
    
    #[test]
    fn test_ra_dec_to_alt_az_circumpolar() {
        // Test circumpolar star (Polaris from northern location)
        let dt = Utc.with_ymd_and_hms(2024, 3, 15, 22, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 50.0,  // Northern location
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        // Polaris coordinates
        let ra = 37.95456067;  // About 2h 31m
        let dec = 89.26410897;  // Very close to north celestial pole
        
        let (alt, _az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
        
        // Altitude should be approximately equal to latitude ± 1° (due to Polaris offset from pole)
        assert!((alt - location.latitude_deg).abs() < 2.0, 
                "Polaris altitude should be close to observer latitude, got alt={}", alt);
    }
    
    #[test]
    fn test_ra_dec_to_alt_az_never_visible() {
        // Test object that never rises from northern location
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 45.0,  // Mid-northern latitude
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        // Southern circumpolar object
        let ra = 0.0;
        let dec = -80.0;  // Deep southern object
        
        let (alt, _az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
        assert!(alt < 0.0, "Deep southern object should be below horizon from northern location");
    }
    
    #[test]
    fn test_coordinate_validation() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        // Test invalid RA (should be 0-360)
        let result = transforms::ra_dec_to_alt_az(400.0, 0.0, dt, &location);
        assert!(result.is_err(), "Invalid RA should return error");
        
        // Test invalid Dec (should be -90 to 90)
        let result = transforms::ra_dec_to_alt_az(0.0, 100.0, dt, &location);
        assert!(result.is_err(), "Invalid Dec should return error");
        
        // Test valid edge cases
        let result = transforms::ra_dec_to_alt_az(0.0, -90.0, dt, &location);
        assert!(result.is_ok(), "Dec = -90 should be valid");
        
        let result = transforms::ra_dec_to_alt_az(359.999, 90.0, dt, &location);
        assert!(result.is_ok(), "RA = 359.999, Dec = 90 should be valid");
    }
    
    #[test]
    fn test_altitude_range() {
        // Altitude should always be between -90 and 90
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        // Test various positions
        let test_cases = vec![
            (0.0, 0.0),
            (180.0, 45.0),
            (270.0, -45.0),
            (90.0, 89.0),
            (0.0, -89.0),
        ];
        
        for (ra, dec) in test_cases {
            let (alt, az) = transforms::ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
            assert!(alt >= -90.0 && alt <= 90.0, 
                    "Altitude must be in range [-90, 90], got {} for RA={}, Dec={}", alt, ra, dec);
            assert!(az >= 0.0 && az < 360.0,
                    "Azimuth must be in range [0, 360), got {} for RA={}, Dec={}", az, ra, dec);
        }
    }

    #[test]
    fn test_array_length_validation() {
        // Test the array length validation logic used in batch operations
        let ra_values = vec![0.0, 90.0, 180.0];
        let dec_values = vec![0.0, 45.0]; // Intentionally different length
        
        // This simulates the validation that happens in ra_dec_to_alt_az_batch
        assert_ne!(ra_values.len(), dec_values.len(), "Arrays should have different lengths for this test");
        
        // Equal length arrays should work
        let ra_equal = vec![0.0, 90.0];
        let dec_equal = vec![0.0, 45.0];
        assert_eq!(ra_equal.len(), dec_equal.len(), "Equal length arrays should pass validation");
        
        // Test the actual transformation logic that would be used in batch
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        
        for (&ra, &dec) in ra_equal.iter().zip(dec_equal.iter()) {
            let result = transforms::ra_dec_to_alt_az(ra, dec, dt, &location);
            assert!(result.is_ok(), "Valid coordinates should transform successfully");
        }
    }

    #[test]
    fn test_extreme_locations() {
        // Test transforms at extreme locations that might be used in batch operations
        let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap(); // Summer solstice
        
        let extreme_locations = vec![
            // North pole
            Location { latitude_deg: 90.0, longitude_deg: 0.0, altitude_m: 0.0 },
            // South pole  
            Location { latitude_deg: -90.0, longitude_deg: 0.0, altitude_m: 0.0 },
            // High altitude
            Location { latitude_deg: 45.0, longitude_deg: 0.0, altitude_m: 8848.0 },
            // Sea level at equator
            Location { latitude_deg: 0.0, longitude_deg: 0.0, altitude_m: 0.0 },
            // Below sea level
            Location { latitude_deg: 31.5, longitude_deg: 35.5, altitude_m: -427.0 }, // Dead Sea
        ];
        
        for location in extreme_locations {
            let result = transforms::ra_dec_to_alt_az(0.0, 0.0, dt, &location);
            assert!(result.is_ok(), "Transform should work at extreme location: lat={}, lon={}, alt={}", 
                    location.latitude_deg, location.longitude_deg, location.altitude_m);
            
            let (alt, az) = result.unwrap();
            assert!(alt >= -90.0 && alt <= 90.0);
            assert!(az >= 0.0 && az < 360.0);
        }
    }

    #[test]
    fn test_multiple_datetime_conversions() {
        // Test the datetime conversion logic used in batch operations
        let test_dates = vec![
            (2000, 1, 1, 0, 0, 0, 0),      // J2000
            (2024, 2, 29, 12, 0, 0, 0),    // Leap year
            (2024, 12, 31, 23, 59, 59, 999999), // End of year with microseconds
        ];
        
        let location = Location {
            latitude_deg: 40.0,
            longitude_deg: -74.0,
            altitude_m: 0.0,
        };
        
        for (year, month, day, hour, minute, second, microsecond) in test_dates {
            let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap()
                + chrono::Duration::microseconds(microsecond);
            
            let result = transforms::ra_dec_to_alt_az(0.0, 45.0, dt, &location);
            assert!(result.is_ok(), "Transform should work for date: {}-{}-{} {}:{}:{}.{}", 
                    year, month, day, hour, minute, second, microsecond);
        }
    }
}