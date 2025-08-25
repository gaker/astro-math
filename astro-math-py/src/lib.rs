// PyO3 requires specific return types that clippy flags as "useless conversions"
// but these are actually necessary for the Python binding interface
#![allow(clippy::useless_conversion)]
#![allow(clippy::type_complexity)]

use pyo3::prelude::*;

mod time;
mod transforms;
mod location;
mod precession;
mod nutation;
mod aberration;
mod proper_motion;
mod sidereal;
mod airmass;
mod galactic;
mod sun_moon;
mod refraction;
mod time_scales;

/// High-performance astronomy calculations for Python
#[pymodule]
fn astro_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    // Create submodules
    let time_module = PyModule::new_bound(m.py(), "time")?;
    time::register(&time_module)?;
    m.add_submodule(&time_module)?;
    
    let timescales_module = PyModule::new_bound(m.py(), "timescales")?;
    time_scales::register(&timescales_module)?;
    m.add_submodule(&timescales_module)?;
    
    let transforms_module = PyModule::new_bound(m.py(), "transforms")?;
    transforms::register(&transforms_module)?;
    m.add_submodule(&transforms_module)?;
    
    let location_module = PyModule::new_bound(m.py(), "location")?;
    location::register(&location_module)?;
    m.add_submodule(&location_module)?;
    
    let precession_module = PyModule::new_bound(m.py(), "precession")?;
    precession::register(&precession_module)?;
    m.add_submodule(&precession_module)?;
    
    let nutation_module = PyModule::new_bound(m.py(), "nutation")?;
    nutation::register(&nutation_module)?;
    m.add_submodule(&nutation_module)?;
    
    let aberration_module = PyModule::new_bound(m.py(), "aberration")?;
    aberration::register(&aberration_module)?;
    m.add_submodule(&aberration_module)?;
    
    let proper_motion_module = PyModule::new_bound(m.py(), "proper_motion")?;
    proper_motion::register(&proper_motion_module)?;
    m.add_submodule(&proper_motion_module)?;
    
    let sidereal_module = PyModule::new_bound(m.py(), "sidereal")?;
    sidereal::register(&sidereal_module)?;
    m.add_submodule(&sidereal_module)?;
    
    let airmass_module = PyModule::new_bound(m.py(), "airmass")?;
    airmass::register(&airmass_module)?;
    m.add_submodule(&airmass_module)?;
    
    let galactic_module = PyModule::new_bound(m.py(), "galactic")?;
    galactic::register(&galactic_module)?;
    m.add_submodule(&galactic_module)?;
    
    let sun_moon_module = PyModule::new_bound(m.py(), "sun_moon")?;
    sun_moon::register(&sun_moon_module)?;
    m.add_submodule(&sun_moon_module)?;
    
    let refraction_module = PyModule::new_bound(m.py(), "refraction")?;
    refraction::register(&refraction_module)?;
    m.add_submodule(&refraction_module)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    
    
    #[test]
    fn test_module_imports() {
        // Just verify that all modules compile and can be imported
        // The actual Python integration tests will be in Python
        // This test passes if the module compiles successfully
    }
}
