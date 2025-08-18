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

/// High-performance astronomy calculations for Python
#[pymodule]
fn astro_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    
    // Time functions
    time::register(m)?;
    
    // Coordinate transforms
    transforms::register(m)?;
    
    // Location parsing
    location::register(m)?;
    
    // Precession and epoch conversion
    precession::register(m)?;
    
    // Nutation corrections
    nutation::register(m)?;
    
    // Aberration corrections
    aberration::register(m)?;
    
    // Proper motion calculations
    proper_motion::register(m)?;
    
    // Sidereal time calculations
    sidereal::register(m)?;
    
    // Airmass and extinction
    airmass::register(m)?;
    
    // Galactic coordinates
    galactic::register(m)?;
    
    // Sun and Moon positions
    sun_moon::register(m)?;
    
    // Atmospheric refraction
    refraction::register(m)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_imports() {
        // Just verify that all modules compile and can be imported
        // The actual Python integration tests will be in Python
        assert!(true);
    }
}
