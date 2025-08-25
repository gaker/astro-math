use pyo3::prelude::*;
use astro_math::refraction as rust_refraction;

/// Calculate atmospheric refraction using Bennett's formula.
///
/// Applies a simple, commonly-used formula for atmospheric refraction 
/// corrections suitable for most observational astronomy applications.
///
/// Parameters
/// ----------
/// altitude_deg : float
///     True altitude in degrees above horizon (0-90°)
///
/// Returns
/// -------
/// float
///     Atmospheric refraction correction in degrees
///
/// Examples
/// --------
/// >>> from astro_math.refraction import bennett
/// >>> # Object at 45° altitude
/// >>> refraction = bennett(45.0)
/// >>> print(f"Refraction: {refraction:.4f}°")
/// Refraction: 0.0062°
///
/// Notes
/// -----
/// This formula assumes standard atmospheric conditions (15°C, 1013.25 hPa).
/// For more accurate corrections with custom conditions, use saemundsson().
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn bennett(altitude_deg: f64) -> PyResult<f64> {
    rust_refraction::refraction_bennett(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate atmospheric refraction using Saemundsson's formula.
///
/// High-accuracy refraction formula that accounts for actual atmospheric
/// conditions including temperature and pressure variations.
///
/// Parameters
/// ----------
/// altitude_deg : float
///     True altitude in degrees above horizon (0-90°)
/// pressure_hpa : float
///     Atmospheric pressure in hectopascals (hPa)
/// temperature_c : float
///     Air temperature in degrees Celsius
///
/// Returns
/// -------
/// float
///     Atmospheric refraction correction in degrees
///
/// Examples
/// --------
/// >>> from astro_math.refraction import saemundsson
/// >>> # Object at 30° altitude in cold, low pressure conditions
/// >>> refraction = saemundsson(30.0, 980.0, -10.0)
/// >>> print(f"Refraction: {refraction:.4f}°")
/// Refraction: 0.0098°
///
/// Notes
/// -----
/// This formula provides higher accuracy than Bennett's formula,
/// especially at low altitudes and non-standard conditions.
#[pyfunction]
#[pyo3(signature = (altitude_deg, pressure_hpa, temperature_c))]
fn saemundsson(altitude_deg: f64, pressure_hpa: f64, temperature_c: f64) -> PyResult<f64> {
    rust_refraction::refraction_saemundsson(altitude_deg, pressure_hpa, temperature_c)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate atmospheric refraction at radio wavelengths.
///
/// Radio refraction depends strongly on atmospheric water vapor content,
/// requiring humidity measurements for accurate corrections.
///
/// Parameters
/// ----------
/// altitude_deg : float
///     True altitude in degrees above horizon (0-90°)
/// pressure_hpa : float
///     Atmospheric pressure in hectopascals (hPa)
/// temperature_c : float
///     Air temperature in degrees Celsius
/// humidity_percent : float
///     Relative humidity as percentage (0-100%)
///
/// Returns
/// -------
/// float
///     Radio refraction correction in degrees
///
/// Examples
/// --------
/// >>> from astro_math.refraction import radio
/// >>> # Radio telescope observation at 20° elevation
/// >>> refraction = radio(20.0, 1013.25, 15.0, 65.0)
/// >>> print(f"Radio refraction: {refraction:.4f}°")
/// Radio refraction: 0.0183°
///
/// Notes
/// -----
/// Radio refraction is typically larger than optical refraction due to
/// the strong dependence on atmospheric water vapor.
#[pyfunction]
#[pyo3(signature = (altitude_deg, pressure_hpa, temperature_c, humidity_percent))]
fn radio(
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
    m.add_function(wrap_pyfunction!(bennett, m)?)?;
    m.add_function(wrap_pyfunction!(saemundsson, m)?)?;
    m.add_function(wrap_pyfunction!(radio, m)?)?;
    m.add_function(wrap_pyfunction!(apparent_to_true_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(true_to_apparent_altitude, m)?)?;
    Ok(())
}