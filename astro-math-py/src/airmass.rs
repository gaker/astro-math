use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use astro_math::airmass as rust_airmass;

/// Calculate airmass using the plane-parallel atmosphere approximation.
///
/// Simplest model assuming flat atmospheric layers. Accurate at high altitudes
/// but underestimates airmass near the horizon.
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn airmass_plane_parallel(altitude_deg: f64) -> PyResult<f64> {
    rust_airmass::airmass_plane_parallel(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate airmass using Young's formula (1994).
///
/// Improved model accounting for Earth's curvature and atmospheric refraction.
/// More accurate than plane-parallel at low altitudes.
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn airmass_young(altitude_deg: f64) -> PyResult<f64> {
    rust_airmass::airmass_young(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate airmass using Pickering's formula (2002).
///
/// Most accurate formula, especially near the horizon. Properly accounts
/// for atmospheric refraction and Earth's curvature.
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn airmass_pickering(altitude_deg: f64) -> PyResult<f64> {
    rust_airmass::airmass_pickering(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate airmass using Kasten & Young's formula (1989).
///
/// Standard formula widely used in astronomy. Good balance between
/// accuracy and simplicity.
#[pyfunction]
#[pyo3(signature = (altitude_deg))]
fn airmass_kasten_young(altitude_deg: f64) -> PyResult<f64> {
    rust_airmass::airmass_kasten_young(altitude_deg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Calculate extinction in magnitudes for a given airmass.
///
/// Atmospheric extinction reduces apparent brightness of celestial objects.
/// Returns the dimming in magnitudes.
#[pyfunction]
#[pyo3(signature = (airmass, extinction_coefficient))]
fn extinction_magnitudes(airmass: f64, extinction_coefficient: f64) -> f64 {
    rust_airmass::extinction_magnitudes(airmass, extinction_coefficient)
}

/// Estimate extinction coefficient based on wavelength.
///
/// Provides rough estimate for clear atmospheric conditions.
/// Real extinction varies with atmospheric conditions and location.
#[pyfunction]
#[pyo3(signature = (wavelength_nm))]
fn extinction_coefficient_estimate(wavelength_nm: f64) -> PyResult<f64> {
    rust_airmass::extinction_coefficient_estimate(wavelength_nm)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Batch calculate airmass for arrays of altitudes using Pickering's formula.
///
/// Most accurate batch calculation for observational planning.
#[pyfunction]
#[pyo3(signature = (altitude_array))]
fn batch_airmass_pickering<'py>(
    py: Python<'py>,
    altitude_array: PyReadonlyArray1<'_, f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let altitude_slice = altitude_array.as_slice()?;
    let mut airmass_out = Vec::with_capacity(altitude_slice.len());
    
    // Use parallel processing for large arrays
    if altitude_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = altitude_slice.par_iter()
            .map(|&alt| {
                rust_airmass::airmass_pickering(alt).unwrap_or(f64::INFINITY)
            })
            .collect();
        airmass_out.extend(results);
    } else {
        for &alt in altitude_slice {
            let airmass = rust_airmass::airmass_pickering(alt).unwrap_or(f64::INFINITY);
            airmass_out.push(airmass);
        }
    }
    
    Ok(airmass_out.into_pyarray_bound(py))
}

/// Batch calculate extinction for arrays of airmass values.
///
/// Efficiently calculates atmospheric extinction for multiple observations.
#[pyfunction]
#[pyo3(signature = (airmass_array, extinction_coefficient))]
fn batch_extinction<'py>(
    py: Python<'py>,
    airmass_array: PyReadonlyArray1<'_, f64>,
    extinction_coefficient: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let airmass_slice = airmass_array.as_slice()?;
    let mut extinction_out = Vec::with_capacity(airmass_slice.len());
    
    for &airmass in airmass_slice {
        extinction_out.push(rust_airmass::extinction_magnitudes(airmass, extinction_coefficient));
    }
    
    Ok(extinction_out.into_pyarray_bound(py))
}

/// Register the airmass module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(airmass_plane_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(airmass_young, m)?)?;
    m.add_function(wrap_pyfunction!(airmass_pickering, m)?)?;
    m.add_function(wrap_pyfunction!(airmass_kasten_young, m)?)?;
    m.add_function(wrap_pyfunction!(extinction_magnitudes, m)?)?;
    m.add_function(wrap_pyfunction!(extinction_coefficient_estimate, m)?)?;
    m.add_function(wrap_pyfunction!(batch_airmass_pickering, m)?)?;
    m.add_function(wrap_pyfunction!(batch_extinction, m)?)?;
    Ok(())
}