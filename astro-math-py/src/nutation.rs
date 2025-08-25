use pyo3::prelude::*;
use astro_math::nutation as rust_nutation;

/// Calculate nutation in longitude and obliquity.
///
/// Returns the nutation corrections for a given Julian Date.
/// These are essential for sub-arcsecond accuracy.
#[pyfunction]
#[pyo3(signature = (jd))]
fn nutation(jd: f64) -> PyResult<(f64, f64)> {
    let nutation_result = rust_nutation::nutation(jd);
    Ok((nutation_result.longitude, nutation_result.obliquity))
}

/// Calculate nutation in longitude only.
#[pyfunction]
#[pyo3(signature = (jd))]
fn in_longitude(jd: f64) -> PyResult<f64> {
    Ok(rust_nutation::nutation_in_longitude(jd))
}

/// Calculate nutation in obliquity only.
#[pyfunction]
#[pyo3(signature = (jd))]
fn in_obliquity(jd: f64) -> PyResult<f64> {
    Ok(rust_nutation::nutation_in_obliquity(jd))
}

/// Calculate mean obliquity of the ecliptic.
#[pyfunction]
#[pyo3(signature = (jd))]
fn mean_obliquity(jd: f64) -> PyResult<f64> {
    Ok(rust_nutation::mean_obliquity(jd))
}

/// Calculate true obliquity of the ecliptic (includes nutation).
#[pyfunction]
#[pyo3(signature = (jd))]
fn true_obliquity(jd: f64) -> PyResult<f64> {
    Ok(rust_nutation::true_obliquity(jd))
}

/// Register the nutation module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(nutation, m)?)?;
    m.add_function(wrap_pyfunction!(in_longitude, m)?)?;
    m.add_function(wrap_pyfunction!(in_obliquity, m)?)?;
    m.add_function(wrap_pyfunction!(mean_obliquity, m)?)?;
    m.add_function(wrap_pyfunction!(true_obliquity, m)?)?;
    Ok(())
}