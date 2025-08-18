use crate::refraction::*;

#[test]
fn test_refraction_below_horizon() {
    // No refraction for objects well below horizon
    let r = refraction_bennett(-5.0).unwrap();
    assert_eq!(r, 0.0);
    
    let r2 = refraction_saemundsson(-5.0, 1013.25, 10.0).unwrap();
    assert_eq!(r2, 0.0);
}

#[test]
fn test_refraction_increases_with_lower_altitude() {
    // Refraction should increase as altitude decreases
    let r_high = refraction_bennett(45.0).unwrap();
    let r_mid = refraction_bennett(20.0).unwrap();
    let r_low = refraction_bennett(5.0).unwrap();
    let r_horizon = refraction_bennett(0.0).unwrap();
    
    assert!(r_high < r_mid);
    assert!(r_mid < r_low);
    assert!(r_low < r_horizon);
}

#[test]
fn test_extreme_conditions() {
    // Test with extreme pressure and temperature
    let r_everest = refraction_saemundsson(10.0, 350.0, -40.0).unwrap(); // Mt. Everest conditions
    let r_death_valley = refraction_saemundsson(10.0, 1030.0, 50.0).unwrap(); // Death Valley summer
    
    // At 10° altitude, refraction should be ~0.09-0.10° under normal conditions
    // Everest (low pressure) should give less refraction
    assert!(r_everest > 0.0 && r_everest < 0.08, 
        "Refraction at 10° with low pressure should be < 0.08°, got {}°", r_everest);
    // Death Valley (high pressure) should give more refraction  
    assert!(r_death_valley > 0.08 && r_death_valley < 0.15,
        "Refraction at 10° with high pressure should be 0.08-0.15°, got {}°", r_death_valley);
    // Verify pressure relationship
    assert!(r_everest < r_death_valley,
        "Lower pressure should give less refraction: {} vs {}", r_everest, r_death_valley);
}

#[test]
fn test_radio_vs_optical() {
    // Test radio refraction under various humidity conditions
    let r_radio_dry = refraction_radio(10.0, 1013.25, 20.0, 0.0).unwrap();
    let r_radio_humid = refraction_radio(10.0, 1013.25, 20.0, 100.0).unwrap();
    
    // Higher humidity should increase radio refraction
    assert!(r_radio_humid > r_radio_dry,
        "100% humidity ({}) should give more refraction than 0% ({})", r_radio_humid, r_radio_dry);
    
    // The difference should be noticeable at 10° altitude
    let humidity_effect = r_radio_humid - r_radio_dry;
    assert!(humidity_effect > 0.0,
        "Humidity should increase refraction, difference = {}°", humidity_effect);
}

#[test]
fn test_refraction_edge_case() {
    // Test refraction at exactly 0 altitude (horizon)
    let refr = refraction_radio(0.0, 1013.25, 20.0, 50.0).unwrap();
    // At horizon, cot(0) = infinity, so refraction might be very large or implementation-dependent
    assert!(refr > 0.0, "Radio refraction at horizon should be positive, got {}°", refr);
    
    // Test just above horizon where calculation is more stable
    let refr_above = refraction_radio(0.1, 1013.25, 20.0, 50.0).unwrap();
    assert!(refr_above > 0.3 && refr_above < 15.0,
        "Radio refraction near horizon should be significant, got {}°", refr_above);
    
    // Compare with optical refraction near horizon
    let optical = refraction_saemundsson(0.1, 1013.25, 20.0).unwrap();
    assert!(optical > 0.3 && optical < 1.0,
        "Optical refraction near horizon should be ~0.3-1.0°, got {}°", optical);
}

#[test]
fn test_refraction_below_limit() {
    // Test refraction functions below their altitude limits
    // Bennett: below -0.5 degrees
    let r1 = refraction_bennett(-1.0).unwrap();
    assert_eq!(r1, 0.0);
    
    // Saemundsson: below -1.0 degrees
    let r2 = refraction_saemundsson(-2.0, 1013.25, 10.0).unwrap();
    assert_eq!(r2, 0.0);
    
    // Radio: below -1.0 degrees
    let r3 = refraction_radio(-2.0, 1013.25, 10.0, 50.0).unwrap();
    assert_eq!(r3, 0.0);
}