use crate::airmass::*;

#[test]
fn test_airmass_consistency() {
    // All formulas should give consistent results for moderate altitudes
    let altitudes = [30.0, 45.0, 60.0, 75.0];
    
    for alt in altitudes {
        let am_pp = airmass_plane_parallel(alt).unwrap();
        let am_young = airmass_young(alt).unwrap();
        let am_pickering = airmass_pickering(alt).unwrap();
        let am_ky = airmass_kasten_young(alt).unwrap();
        
        // At moderate altitudes, all should agree within ~5%
        assert!((am_young - am_pp).abs() / am_pp < 0.05);
        assert!((am_pickering - am_pp).abs() / am_pp < 0.05);
        assert!((am_ky - am_pp).abs() / am_pp < 0.05);
    }
}

#[test]
fn test_airmass_increases_with_zenith_angle() {
    // Airmass should increase as altitude decreases
    let altitudes = [90.0, 60.0, 45.0, 30.0, 15.0, 5.0];
    
    for window in altitudes.windows(2) {
        let am1 = airmass_pickering(window[0]).unwrap();
        let am2 = airmass_pickering(window[1]).unwrap();
        assert!(am2 > am1, "Airmass should increase as altitude decreases");
    }
}

#[test]
fn test_extinction_wavelength_dependence() {
    // Test atmospheric extinction at different wavelengths
    let wavelengths = [
        (380.0, "UV"),
        (450.0, "Blue"),
        (550.0, "Green"),
        (650.0, "Red"),
        (800.0, "Near-IR"),
    ];
    
    for window in wavelengths.windows(2) {
        let k1 = extinction_coefficient_estimate(window[0].0).unwrap();
        let k2 = extinction_coefficient_estimate(window[1].0).unwrap();
        assert!(k1 > k2, "Extinction should decrease with wavelength");
    }
}