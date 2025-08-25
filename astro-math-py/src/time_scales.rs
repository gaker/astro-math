//! Python bindings for time scale conversions

use pyo3::prelude::*;
use astro_math::time_scales;

/// Get the current TAI-UTC offset in seconds.
#[pyfunction]
fn tai_utc_offset() -> f64 {
    time_scales::tai_utc_offset()
}

/// Get the TT-UTC offset in seconds.
#[pyfunction]
fn tt_utc_offset_seconds() -> f64 {
    time_scales::tt_utc_offset_seconds()
}

/// Convert UTC Julian Date to TT Julian Date.
#[pyfunction]
#[pyo3(signature = (jd_utc))]
fn utc_to_tt_jd(jd_utc: f64) -> f64 {
    time_scales::utc_to_tt_jd(jd_utc)
}

/// Convert TT Julian Date to UTC Julian Date.
#[pyfunction] 
#[pyo3(signature = (jd_tt))]
fn tt_to_utc_jd(jd_tt: f64) -> f64 {
    time_scales::tt_to_utc_jd(jd_tt)
}

/// Check if hardcoded time offset needs updating.
#[pyfunction]
#[pyo3(signature = (hardcoded_seconds))]
fn check_time_offset_accuracy(hardcoded_seconds: f64) -> f64 {
    time_scales::check_time_offset_accuracy(hardcoded_seconds)
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tai_utc_offset, m)?)?;
    m.add_function(wrap_pyfunction!(tt_utc_offset_seconds, m)?)?;
    m.add_function(wrap_pyfunction!(utc_to_tt_jd, m)?)?;
    m.add_function(wrap_pyfunction!(tt_to_utc_jd, m)?)?;
    m.add_function(wrap_pyfunction!(check_time_offset_accuracy, m)?)?;
    Ok(())
}