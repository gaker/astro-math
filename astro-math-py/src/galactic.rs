use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use astro_math::galactic as rust_galactic;

/// Convert equatorial coordinates to galactic coordinates.
///
/// Transforms from ICRS J2000.0 equatorial coordinates (RA, Dec) to
/// galactic coordinates (longitude, latitude) using IAU standard definitions.
#[pyfunction]
#[pyo3(signature = (ra, dec))]
fn equatorial_to_galactic(ra: f64, dec: f64) -> PyResult<(f64, f64)> {
    rust_galactic::equatorial_to_galactic(ra, dec)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Convert galactic coordinates to equatorial coordinates.
///
/// Transforms from galactic coordinates (longitude, latitude) to
/// ICRS J2000.0 equatorial coordinates (RA, Dec).
#[pyfunction]
#[pyo3(signature = (l, b))]
fn galactic_to_equatorial(l: f64, b: f64) -> PyResult<(f64, f64)> {
    rust_galactic::galactic_to_equatorial(l, b)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

/// Get galactic coordinates of common astronomical landmarks.
///
/// Returns a list of (name, longitude, latitude) tuples for reference objects.
#[pyfunction]
fn galactic_landmarks() -> Vec<(String, f64, f64)> {
    rust_galactic::galactic_landmarks()
        .into_iter()
        .map(|(name, l, b)| (name.to_string(), l, b))
        .collect()
}

/// Batch convert equatorial to galactic coordinates.
///
/// Efficiently processes arrays of coordinates using parallel computation.
#[pyfunction]
#[pyo3(signature = (ra_array, dec_array))]
fn batch_equatorial_to_galactic<'py>(
    py: Python<'py>,
    ra_array: PyReadonlyArray1<'_, f64>,
    dec_array: PyReadonlyArray1<'_, f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let ra_slice = ra_array.as_slice()?;
    let dec_slice = dec_array.as_slice()?;
    
    if ra_slice.len() != dec_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "RA and Dec arrays must have the same length"
        ));
    }
    
    let mut l_out = Vec::with_capacity(ra_slice.len());
    let mut b_out = Vec::with_capacity(dec_slice.len());
    
    // Use parallel processing for large arrays
    if ra_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = ra_slice.par_iter()
            .zip(dec_slice.par_iter())
            .map(|(&ra, &dec)| {
                rust_galactic::equatorial_to_galactic(ra, dec)
                    .unwrap_or((ra, dec)) // fallback to original coords on error
            })
            .collect();
        
        for (l, b) in results {
            l_out.push(l);
            b_out.push(b);
        }
    } else {
        for (&ra, &dec) in ra_slice.iter().zip(dec_slice.iter()) {
            match rust_galactic::equatorial_to_galactic(ra, dec) {
                Ok((l, b)) => {
                    l_out.push(l);
                    b_out.push(b);
                },
                Err(_) => {
                    l_out.push(ra); // fallback
                    b_out.push(dec);
                }
            }
        }
    }
    
    Ok((
        l_out.into_pyarray_bound(py),
        b_out.into_pyarray_bound(py),
    ))
}

/// Batch convert galactic to equatorial coordinates.
///
/// Efficiently processes arrays of coordinates using parallel computation.
#[pyfunction]
#[pyo3(signature = (l_array, b_array))]
fn batch_galactic_to_equatorial<'py>(
    py: Python<'py>,
    l_array: PyReadonlyArray1<'_, f64>,
    b_array: PyReadonlyArray1<'_, f64>,
) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
    let l_slice = l_array.as_slice()?;
    let b_slice = b_array.as_slice()?;
    
    if l_slice.len() != b_slice.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "L and B arrays must have the same length"
        ));
    }
    
    let mut ra_out = Vec::with_capacity(l_slice.len());
    let mut dec_out = Vec::with_capacity(b_slice.len());
    
    // Use parallel processing for large arrays
    if l_slice.len() > 1000 {
        use rayon::prelude::*;
        let results: Vec<_> = l_slice.par_iter()
            .zip(b_slice.par_iter())
            .map(|(&l, &b)| {
                rust_galactic::galactic_to_equatorial(l, b)
                    .unwrap_or((l, b)) // fallback to original coords on error
            })
            .collect();
        
        for (ra, dec) in results {
            ra_out.push(ra);
            dec_out.push(dec);
        }
    } else {
        for (&l, &b) in l_slice.iter().zip(b_slice.iter()) {
            match rust_galactic::galactic_to_equatorial(l, b) {
                Ok((ra, dec)) => {
                    ra_out.push(ra);
                    dec_out.push(dec);
                },
                Err(_) => {
                    ra_out.push(l); // fallback
                    dec_out.push(b);
                }
            }
        }
    }
    
    Ok((
        ra_out.into_pyarray_bound(py),
        dec_out.into_pyarray_bound(py),
    ))
}

/// Register the galactic coordinates module with Python
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(equatorial_to_galactic, m)?)?;
    m.add_function(wrap_pyfunction!(galactic_to_equatorial, m)?)?;
    m.add_function(wrap_pyfunction!(galactic_landmarks, m)?)?;
    m.add_function(wrap_pyfunction!(batch_equatorial_to_galactic, m)?)?;
    m.add_function(wrap_pyfunction!(batch_galactic_to_equatorial, m)?)?;
    
    // Add constants
    m.add("NGP_RA", rust_galactic::NGP_RA)?;
    m.add("NGP_DEC", rust_galactic::NGP_DEC)?;
    m.add("GC_RA", rust_galactic::GC_RA)?;
    m.add("GC_DEC", rust_galactic::GC_DEC)?;
    
    Ok(())
}