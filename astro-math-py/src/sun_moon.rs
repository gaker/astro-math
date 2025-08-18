use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDateAccess, PyTimeAccess};
use astro_math::{sun, moon};
use chrono::{DateTime, TimeZone, Utc};

/// Calculate the Sun's equatorial position (RA, Dec).
///
/// Returns the Sun's position in ICRS J2000.0 coordinates.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn sun_position(datetime: &Bound<'_, PyDateTime>) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    Ok(sun::sun_position(dt))
}

/// Calculate the Sun's right ascension and declination.
///
/// Alias for sun_position for compatibility.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn sun_ra_dec(datetime: &Bound<'_, PyDateTime>) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    Ok(sun::sun_ra_dec(dt))
}

/// Calculate the Moon's equatorial position (RA, Dec).
///
/// Returns the Moon's position in ICRS J2000.0 coordinates.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_position(datetime: &Bound<'_, PyDateTime>) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_position(dt))
}

/// Calculate the Moon's phase angle.
///
/// Returns the phase angle in degrees (0° = new moon, 180° = full moon).
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_phase_angle(datetime: &Bound<'_, PyDateTime>) -> PyResult<f64> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_phase_angle(dt))
}

/// Calculate the Moon's illumination fraction.
///
/// Returns the fraction of the Moon's disk that is illuminated (0.0 to 1.0).
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_illumination(datetime: &Bound<'_, PyDateTime>) -> PyResult<f64> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_illumination(dt))
}

/// Get the Moon's phase name.
///
/// Returns a string describing the current lunar phase.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_phase_name(datetime: &Bound<'_, PyDateTime>) -> PyResult<String> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_phase_name(dt).to_string())
}

/// Calculate the Moon's distance from Earth.
///
/// Returns the distance in kilometers.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_distance(datetime: &Bound<'_, PyDateTime>) -> PyResult<f64> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_distance(dt))
}

/// Calculate the Moon's equatorial coordinates.
///
/// Alias for moon_position for compatibility.
#[pyfunction]
#[pyo3(signature = (datetime))]
fn moon_equatorial(datetime: &Bound<'_, PyDateTime>) -> PyResult<(f64, f64)> {
    let dt = datetime_from_py(datetime)?;
    Ok(moon::moon_equatorial(dt))
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

/// Register the sun/moon module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sun_position, m)?)?;
    m.add_function(wrap_pyfunction!(sun_ra_dec, m)?)?;
    m.add_function(wrap_pyfunction!(moon_position, m)?)?;
    m.add_function(wrap_pyfunction!(moon_phase_angle, m)?)?;
    m.add_function(wrap_pyfunction!(moon_illumination, m)?)?;
    m.add_function(wrap_pyfunction!(moon_phase_name, m)?)?;
    m.add_function(wrap_pyfunction!(moon_distance, m)?)?;
    m.add_function(wrap_pyfunction!(moon_equatorial, m)?)?;
    Ok(())
}