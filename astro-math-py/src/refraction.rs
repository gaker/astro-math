use pyo3::prelude::*;
use astro_math::refraction as rust_refraction;

/// Calculate atmospheric refraction using Bennett's formula.
///
/// Simple formula for atmospheric refraction corrections.
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn refraction_bennett(altitude_deg: f64) -> PyResult<f64> {
    rust_refraction::refraction_bennett(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate atmospheric refraction using Saemundsson's formula.
///
/// More accurate formula including temperature and pressure corrections.
#[pyfunction]
#[pyo3(signature = (altitude_deg, pressure_hpa, temperature_c))]
fn refraction_saemundsson(altitude_deg: f64, pressure_hpa: f64, temperature_c: f64) -> PyResult<f64> {
    rust_refraction::refraction_saemundsson(altitude_deg, pressure_hpa, temperature_c)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate radio refraction.
///
/// Refraction varies with atmospheric conditions at radio wavelengths.
#[pyfunction]
#[pyo3(signature = (altitude_deg, pressure_hpa, temperature_c, humidity_percent))]
fn refraction_radio(
    altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
    humidity_percent: f64,
) -> PyResult<f64> {
    rust_refraction::refraction_radio(
        altitude_deg, pressure_hpa, temperature_c, humidity_percent
    ).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Convert apparent altitude to true altitude.
///
/// Removes atmospheric refraction to get geometric altitude.
#[pyfunction]
#[pyo3(signature = (apparent_altitude_deg, pressure_hpa, temperature_c))]
fn apparent_to_true_altitude(
    apparent_altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
) -> PyResult<f64> {
    rust_refraction::apparent_to_true_altitude(apparent_altitude_deg, pressure_hpa, temperature_c)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Convert true altitude to apparent altitude.
///
/// Adds atmospheric refraction to get observed altitude.
#[pyfunction]
#[pyo3(signature = (true_altitude_deg, pressure_hpa, temperature_c))]
fn true_to_apparent_altitude(
    true_altitude_deg: f64,
    pressure_hpa: f64,
    temperature_c: f64,
) -> PyResult<f64> {
    rust_refraction::true_to_apparent_altitude(true_altitude_deg, pressure_hpa, temperature_c)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Register the refraction module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(refraction_bennett, m)?)?;
    m.add_function(wrap_pyfunction!(refraction_saemundsson, m)?)?;
    m.add_function(wrap_pyfunction!(refraction_radio, m)?)?;
    m.add_function(wrap_pyfunction!(apparent_to_true_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(true_to_apparent_altitude, m)?)?;
    Ok(())
}