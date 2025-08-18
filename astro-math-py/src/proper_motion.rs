use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDateAccess, PyTimeAccess};
use astro_math::proper_motion as rust_proper_motion;
use chrono::{DateTime, TimeZone, Utc};

/// Apply linear proper motion to stellar coordinates.
///
/// Corrects star positions from J2000.0 epoch to target epoch using
/// proper motion measurements in milliarcseconds per year.
#[pyfunction]
#[pyo3(signature = (ra_j2000, dec_j2000, pm_ra_cosdec, pm_dec, target_epoch))]
fn apply_proper_motion(
    ra_j2000: f64,
    dec_j2000: f64,
    pm_ra_cosdec: f64,
    pm_dec: f64,
    target_epoch: &Bound<'_, PyDateTime>,
) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(target_epoch)?;
    
    rust_proper_motion::apply_proper_motion(ra_j2000, dec_j2000, pm_ra_cosdec, pm_dec, dt)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Apply rigorous proper motion with space velocity corrections.
///
/// Accounts for changing perspective as a star moves through space.
/// Important for nearby stars with high proper motion.
#[pyfunction]
#[pyo3(signature = (ra_j2000, dec_j2000, pm_ra_cosdec, pm_dec, parallax, radial_velocity, target_epoch))]
fn apply_proper_motion_rigorous(
    ra_j2000: f64,
    dec_j2000: f64,
    pm_ra_cosdec: f64,
    pm_dec: f64,
    parallax: f64,
    radial_velocity: f64,
    target_epoch: &Bound<'_, PyDateTime>,
) -> PyResult<(f64, f64, f64)> {
    let dt = datetime_from_py(target_epoch)?;
    
    rust_proper_motion::apply_proper_motion_rigorous(
        ra_j2000, dec_j2000, pm_ra_cosdec, pm_dec, parallax, radial_velocity, dt
    ).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate total proper motion magnitude from components.
///
/// Returns the magnitude of the proper motion vector in mas/yr.
#[pyfunction]
#[pyo3(signature = (pm_ra_cosdec, pm_dec))]
fn total_proper_motion(pm_ra_cosdec: f64, pm_dec: f64) -> f64 {
    rust_proper_motion::total_proper_motion(pm_ra_cosdec, pm_dec)
}

/// Calculate position angle of proper motion vector.
///
/// Returns the direction of motion in degrees (0° = North, 90° = East).
#[pyfunction]
#[pyo3(signature = (pm_ra_cosdec, pm_dec))]
fn proper_motion_position_angle(pm_ra_cosdec: f64, pm_dec: f64) -> f64 {
    rust_proper_motion::proper_motion_position_angle(pm_ra_cosdec, pm_dec)
}

/// Convert proper motion in RA to RA × cos(dec) form.
///
/// Transforms between different proper motion conventions.
#[pyfunction]
#[pyo3(signature = (pm_ra, dec))]
fn pm_ra_to_pm_ra_cosdec(pm_ra: f64, dec: f64) -> f64 {
    rust_proper_motion::pm_ra_to_pm_ra_cosdec(pm_ra, dec)
}

/// Convert proper motion from RA × cos(dec) to RA form.
///
/// Transforms between different proper motion conventions.
#[pyfunction]
#[pyo3(signature = (pm_ra_cosdec, dec))]
fn pm_ra_cosdec_to_pm_ra(pm_ra_cosdec: f64, dec: f64) -> f64 {
    rust_proper_motion::pm_ra_cosdec_to_pm_ra(pm_ra_cosdec, dec)
}

/// Batch apply proper motion to arrays of stars.
///
/// Efficiently processes multiple stars using parallel computation.
#[pyfunction]
#[pyo3(signature = (ra_array, dec_array, pm_ra_array, pm_dec_array, target_epoch))]
fn batch_apply_proper_motion<'py>(
    py: Python<'py>,
    ra_array: PyReadonlyArray1<'_, f64>,
    dec_array: PyReadonlyArray1<'_, f64>,
    pm_ra_array: PyReadonlyArray1<'_, f64>,
    pm_dec_array: PyReadonlyArray1<'_, f64>,
    target_epoch: &Bound<'_, PyDateTime>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let dt = datetime_from_py(target_epoch)?;
    
    let ra_slice = ra_array.as_slice()?;
    let dec_slice = dec_array.as_slice()?;
    let pm_ra_slice = pm_ra_array.as_slice()?;
    let pm_dec_slice = pm_dec_array.as_slice()?;
    
    if ra_slice.len() != dec_slice.len() 
        || ra_slice.len() != pm_ra_slice.len() 
        || ra_slice.len() != pm_dec_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "All arrays must have the same length"
        ));
    }
    
    let mut ra_out = Vec::with_capacity(ra_slice.len());
    let mut dec_out = Vec::with_capacity(dec_slice.len());
    
    // Use parallel processing for large arrays
    if ra_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = ra_slice.par_iter()
            .zip(dec_slice.par_iter())
            .zip(pm_ra_slice.par_iter())
            .zip(pm_dec_slice.par_iter())
            .map(|(((ra, dec), pm_ra), pm_dec)| {
                rust_proper_motion::apply_proper_motion(*ra, *dec, *pm_ra, *pm_dec, dt)
                    .unwrap_or((*ra, *dec))
            })
            .collect();
        
        for (ra, dec) in results {
            ra_out.push(ra);
            dec_out.push(dec);
        }
    } else {
        for (((ra, dec), pm_ra), pm_dec) in ra_slice.iter()
            .zip(dec_slice.iter())
            .zip(pm_ra_slice.iter())
            .zip(pm_dec_slice.iter()) {
            match rust_proper_motion::apply_proper_motion(*ra, *dec, *pm_ra, *pm_dec, dt) {
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

// Helper function to parse datetime from Python
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

/// Register the proper motion module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_proper_motion, m)?)?;
    m.add_function(wrap_pyfunction!(apply_proper_motion_rigorous, m)?)?;
    m.add_function(wrap_pyfunction!(total_proper_motion, m)?)?;
    m.add_function(wrap_pyfunction!(proper_motion_position_angle, m)?)?;
    m.add_function(wrap_pyfunction!(pm_ra_to_pm_ra_cosdec, m)?)?;
    m.add_function(wrap_pyfunction!(pm_ra_cosdec_to_pm_ra, m)?)?;
    m.add_function(wrap_pyfunction!(batch_apply_proper_motion, m)?)?;
    Ok(())
}