use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDateAccess, PyTimeAccess};
use astro_math::time;
use chrono::{DateTime, TimeZone, Utc};

/// Convert a datetime to Julian Date.
/// 
/// Parameters
/// ----------
/// dt : datetime
///     UTC datetime to convert
/// 
/// Returns
/// -------
/// float
///     Julian Date
/// 
/// Examples
/// --------
/// >>> from astro_math.time import julian
/// >>> from datetime import datetime
/// >>> jd = julian(datetime(2000, 1, 1, 12, 0, 0))
/// >>> print(f"{jd:.1f}")
/// 2451545.0
#[pyfunction]
#[pyo3(signature = (dt))]
fn julian(dt: &Bound<'_, PyDateTime>) -> PyResult<f64> {
    let datetime = datetime_from_py(dt)?;
    Ok(time::julian_date(datetime))
}

/// Batch convert datetimes to Julian Dates
#[pyfunction]
#[pyo3(signature = (dts))]
fn julian_batch<'py>(
    py: Python<'py>,
    dts: Vec<Bound<'py, PyDateTime>>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let jds: Vec<f64> = dts
        .into_iter()
        .map(|dt| {
            let datetime = datetime_from_py(&dt)?;
            Ok(time::julian_date(datetime))
        })
        .collect::<PyResult<Vec<f64>>>()?;
    
    Ok(jds.into_pyarray_bound(py))
}

/// Convert a datetime to days since J2000.0.
/// 
/// Parameters
/// ----------
/// dt : datetime
///     UTC datetime to convert
/// 
/// Returns
/// -------
/// float
///     Days since J2000.0 epoch (January 1, 2000, 12:00 UTC)
/// 
/// Examples
/// --------
/// >>> from astro_math.time import j2000
/// >>> from datetime import datetime
/// >>> days = j2000(datetime(2000, 1, 1, 12, 0, 0))
/// >>> print(f"{days:.1f}")
/// 0.0
#[pyfunction]
#[pyo3(signature = (dt))]
fn j2000(dt: &Bound<'_, PyDateTime>) -> PyResult<f64> {
    let datetime = datetime_from_py(dt)?;
    Ok(time::j2000_days(datetime))
}

/// Batch convert datetimes to days since J2000.0
#[pyfunction]
#[pyo3(signature = (dts))]
fn j2000_batch<'py>(
    py: Python<'py>,
    dts: Vec<Bound<'py, PyDateTime>>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let days: Vec<f64> = dts
        .into_iter()
        .map(|dt| {
            let datetime = datetime_from_py(&dt)?;
            Ok(time::j2000_days(datetime))
        })
        .collect::<PyResult<Vec<f64>>>()?;
    
    Ok(days.into_pyarray_bound(py))
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
    m.add_function(wrap_pyfunction!(julian, m)?)?;
    m.add_function(wrap_pyfunction!(julian_batch, m)?)?;
    m.add_function(wrap_pyfunction!(j2000, m)?)?;
    m.add_function(wrap_pyfunction!(j2000_batch, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_from_py_conversion() {
        // We can't directly test Python datetime conversion without Python runtime,
        // but we can test the underlying astro_math functions
        let dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let jd = time::julian_date(dt);
        assert!((jd - 2451545.0).abs() < 1e-10, "J2000.0 epoch should be JD 2451545.0");
    }

    #[test]
    fn test_julian_date_known_values() {
        // Test some known Julian dates
        let test_cases = vec![
            (2000, 1, 1, 12, 0, 0, 2451545.0),  // J2000.0
            (1858, 11, 17, 0, 0, 0, 2400000.5),  // MJD epoch
            (2024, 1, 1, 0, 0, 0, 2460310.5),    // 2024 New Year
        ];
        
        for (year, month, day, hour, minute, second, expected_jd) in test_cases {
            let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap();
            let jd = time::julian_date(dt);
            assert!(
                (jd - expected_jd).abs() < 1e-6,
                "JD for {}-{:02}-{:02} {:02}:{:02}:{:02} should be {}, got {}",
                year, month, day, hour, minute, second, expected_jd, jd
            );
        }
    }

    #[test]
    fn test_j2000_days_values() {
        // Test days since J2000.0
        let j2000 = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let days = time::j2000_days(j2000);
        assert!(days.abs() < 1e-10, "J2000.0 should be 0 days from J2000.0");
        
        // One day later
        let one_day_later = Utc.with_ymd_and_hms(2000, 1, 2, 12, 0, 0).unwrap();
        let days = time::j2000_days(one_day_later);
        assert!((days - 1.0).abs() < 1e-10, "Should be 1 day after J2000.0");
        
        // One year later (365 days in 2000, which was a leap year)
        let one_year_later = Utc.with_ymd_and_hms(2001, 1, 1, 12, 0, 0).unwrap();
        let days = time::j2000_days(one_year_later);
        assert!((days - 366.0).abs() < 1e-10, "Should be 366 days after J2000.0 (leap year)");
    }

    #[test]
    fn test_batch_operations_consistency() {
        // Test that batch operations give same results as single operations
        let dates = vec![
            Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2010, 6, 15, 18, 30, 0).unwrap(),
            Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap(),
        ];
        
        for dt in dates {
            let single_jd = time::julian_date(dt);
            let single_j2000 = time::j2000_days(dt);
            
            // Batch operations would be tested via Python interface
            // Here we just verify the underlying calculations are consistent
            assert!(single_jd > 2400000.0, "JD should be reasonable");
            assert!((single_jd - 2451545.0 - single_j2000).abs() < 1e-10, 
                    "JD and J2000 days should be consistent");
        }
    }

    #[test]
    fn test_datetime_conversion_edge_cases() {
        // Test datetime conversions that would happen in Python bindings
        let edge_cases = vec![
            // Leap year
            (2000, 2, 29, 0, 0, 0, 0),
            // End of century
            (1999, 12, 31, 23, 59, 59, 999999),
            // Start of millennium  
            (2001, 1, 1, 0, 0, 0, 0),
            // Future date
            (2100, 6, 15, 12, 30, 45, 123456),
        ];
        
        for (year, month, day, hour, minute, second, microsecond) in edge_cases {
            let dt = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap()
                + chrono::Duration::microseconds(microsecond);
            
            let jd = time::julian_date(dt);
            let j2000 = time::j2000_days(dt);
            
            assert!(jd > 1721426.0, "JD should be after calendar start");
            assert!((jd - 2451545.0 - j2000).abs() < 1e-10, "JD and J2000 should be consistent");
        }
    }

    #[test]
    fn test_time_precision() {
        // Test that time precision is maintained (as done in datetime_from_py)
        let base_dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let second_dt = Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 1).unwrap();
        
        let jd_base = time::julian_date(base_dt);
        let jd_second = time::julian_date(second_dt);
        
        // 1 second = 1/(24*3600) days in JD
        let expected_diff = 1.0 / (24.0 * 3600.0);
        let actual_diff = jd_second - jd_base;
        
        assert!((actual_diff - expected_diff).abs() < 1e-10, 
                "Second precision should be maintained: expected {}, got {}", 
                expected_diff, actual_diff);
    }
}