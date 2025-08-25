use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDateAccess, PyTimeAccess};
use astro_math::precession as rust_precession;
use chrono::{DateTime, TimeZone, Utc};

/// Convert coordinates from J2000.0 epoch to a specified date.
///
/// Applies precession corrections to transform celestial coordinates
/// from the standard J2000.0 epoch to any other date.
#[pyfunction]
#[pyo3(signature = (ra_j2000, dec_j2000, datetime))]
fn j2000_to_date(
    ra_j2000: f64,
    dec_j2000: f64,
    datetime: &Bound<'_, PyDateTime>,
) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    
    rust_precession::precess_from_j2000(ra_j2000, dec_j2000, dt)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Convert coordinates from a specified date back to J2000.0 epoch.
///
/// Removes precession effects to transform celestial coordinates
/// from any date back to the standard J2000.0 epoch.
#[pyfunction]
#[pyo3(signature = (ra, dec, datetime))]
fn to_j2000(
    ra: f64,
    dec: f64,
    datetime: &Bound<'_, PyDateTime>,
) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    
    rust_precession::precess_to_j2000(ra, dec, dt)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Batch convert coordinates from J2000.0 to a specified date.
///
/// Efficiently processes multiple coordinate pairs using parallel computation.
#[pyfunction]
#[pyo3(signature = (ra_array, dec_array, datetime))]
fn batch_j2000_to_date<'py>(
    py: Python<'py>,
    ra_array: PyReadonlyArray1<'_, f64>,
    dec_array: PyReadonlyArray1<'_, f64>,
    datetime: &Bound<'_, PyDateTime>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let dt = datetime_from_py(datetime)?;
    
    let ra_slice = ra_array.as_slice()?;
    let dec_slice = dec_array.as_slice()?;
    
    if ra_slice.len() != dec_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "RA and Dec arrays must have the same length"
        ));
    }
    
    let mut ra_out = Vec::with_capacity(ra_slice.len());
    let mut dec_out = Vec::with_capacity(dec_slice.len());
    
    // Use parallel processing for large arrays
    if ra_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = ra_slice.par_iter()
            .zip(dec_slice.par_iter())
            .map(|(&ra, &dec)| {
                rust_precession::precess_from_j2000(ra, dec, dt)
                    .unwrap_or((ra, dec))
            })
            .collect();
        
        for (ra, dec) in results {
            ra_out.push(ra);
            dec_out.push(dec);
        }
    } else {
        for (ra, dec) in ra_slice.iter().zip(dec_slice.iter()) {
            match rust_precession::precess_from_j2000(*ra, *dec, dt) {
                Ok((ra_new, dec_new)) => {
                    ra_out.push(ra_new);
                    dec_out.push(dec_new);
                },
                Err(_) => {
                    ra_out.push(*ra);
                    dec_out.push(*dec);
                }
            }
        }
    }
    
    Ok((
        ra_out.into_pyarray_bound(py),
        dec_out.into_pyarray_bound(py),
    ))
}

/// Batch convert coordinates from a specified date to J2000.0.
#[pyfunction]
#[pyo3(signature = (ra_array, dec_array, datetime))]
fn batch_to_j2000<'py>(
    py: Python<'py>,
    ra_array: PyReadonlyArray1<'_, f64>,
    dec_array: PyReadonlyArray1<'_, f64>,
    datetime: &Bound<'_, PyDateTime>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let dt = datetime_from_py(datetime)?;
    
    let ra_slice = ra_array.as_slice()?;
    let dec_slice = dec_array.as_slice()?;
    
    if ra_slice.len() != dec_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "RA and Dec arrays must have the same length"
        ));
    }
    
    let mut ra_out = Vec::with_capacity(ra_slice.len());
    let mut dec_out = Vec::with_capacity(dec_slice.len());
    
    // Use parallel processing for large arrays
    if ra_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = ra_slice.par_iter()
            .zip(dec_slice.par_iter())
            .map(|(&ra, &dec)| {
                rust_precession::precess_to_j2000(ra, dec, dt)
                    .unwrap_or((ra, dec))
            })
            .collect();
        
        for (ra, dec) in results {
            ra_out.push(ra);
            dec_out.push(dec);
        }
    } else {
        for (ra, dec) in ra_slice.iter().zip(dec_slice.iter()) {
            match rust_precession::precess_to_j2000(*ra, *dec, dt) {
                Ok((ra_new, dec_new)) => {
                    ra_out.push(ra_new);
                    dec_out.push(dec_new);
                },
                Err(_) => {
                    ra_out.push(*ra);
                    dec_out.push(*dec);
                }
            }
        }
    }
    
    Ok((
        ra_out.into_pyarray_bound(py),
        dec_out.into_pyarray_bound(py),
    ))
}

// Helper function to parse datetime from Python (copied from transforms.rs)
fn datetime_from_py(dt: &Bound<'_, PyDateTime>) -> PyResult<DateTime<Utc>> {
    let year = dt.get_year();
    let month = dt.get_month();
    let day = dt.get_day();
    let hour = dt.get_hour();
    let minute = dt.get_minute();
    let second = dt.get_second();
    let microsecond = dt.get_microsecond();

    let naive_dt = chrono::NaiveDate::from_ymd_opt(year, month.into(), day.into())
        .and_then(|d| {
            d.and_hms_micro_opt(
                hour.into(),
                minute.into(),
                second.into(),
                microsecond,
            )
        })
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid datetime"))?;

    Ok(Utc.from_utc_datetime(&naive_dt))
}

/// Register the precession module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(j2000_to_date, m)?)?;
    m.add_function(wrap_pyfunction!(to_j2000, m)?)?;
    m.add_function(wrap_pyfunction!(batch_j2000_to_date, m)?)?;
    m.add_function(wrap_pyfunction!(batch_to_j2000, m)?)?;
    Ok(())
}