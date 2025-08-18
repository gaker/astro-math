use pyo3::prelude::*;
use astro_math::sidereal as rust_sidereal;

/// Calculate Greenwich Mean Sidereal Time (GMST).
///
/// Returns the mean sidereal time at Greenwich meridian in fractional hours.
/// This is the time system based on Earth's rotation relative to the stars.
#[pyfunction]
#[pyo3(signature = (jd))]
fn gmst(jd: f64) -> f64 {
    rust_sidereal::gmst(jd)
}

/// Calculate Local Mean Sidereal Time (LMST).
///
/// Returns the mean sidereal time for a given longitude in fractional hours.
/// Essential for telescope pointing and celestial coordinate conversions.
#[pyfunction]
#[pyo3(signature = (jd, longitude_deg))]
fn local_mean_sidereal_time(jd: f64, longitude_deg: f64) -> f64 {
    rust_sidereal::local_mean_sidereal_time(jd, longitude_deg)
}

/// Calculate Local Apparent Sidereal Time (LAST).
///
/// Returns the apparent sidereal time including nutation corrections.
/// Most accurate form of sidereal time for precise observations.
#[pyfunction]
#[pyo3(signature = (jd, longitude_deg))]
fn apparent_sidereal_time(jd: f64, longitude_deg: f64) -> f64 {
    rust_sidereal::apparent_sidereal_time(jd, longitude_deg)
}

/// Register the sidereal time module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gmst, m)?)?;
    m.add_function(wrap_pyfunction!(local_mean_sidereal_time, m)?)?;
    m.add_function(wrap_pyfunction!(apparent_sidereal_time, m)?)?;
    Ok(())
}