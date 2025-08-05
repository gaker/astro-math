use crate::refraction::*;

#[test]
fn test_refraction_below_horizon() {
    // No refraction for objects well below horizon
    let r = refraction_bennett(-5.0);
    assert_eq!(r, 0.0);
    
    let r2 = refraction_saemundsson(-5.0, 1013.25, 10.0);
    assert_eq!(r2, 0.0);
}

#[test]
fn test_refraction_increases_with_lower_altitude() {
    // Refraction should increase as altitude decreases
    let r_high = refraction_bennett(45.0);
    let r_mid = refraction_bennett(20.0);
    let r_low = refraction_bennett(5.0);
    let r_horizon = refraction_bennett(0.0);
    
    assert!(r_high < r_mid);
    assert!(r_mid < r_low);
    assert!(r_low < r_horizon);
}

#[test]
fn test_extreme_conditions() {
    // Test with extreme pressure and temperature
    let r_everest = refraction_saemundsson(10.0, 350.0, -40.0); // Mt. Everest conditions
    let r_death_valley = refraction_saemundsson(10.0, 1030.0, 50.0); // Death Valley summer
    
    // Both should give reasonable values
    assert!(r_everest > 0.0 && r_everest < 0.2);
    assert!(r_death_valley > 0.0 && r_death_valley < 0.2);
}

#[test]
fn test_radio_vs_optical() {
    // Test radio refraction under various humidity conditions
    let r_radio_dry = refraction_radio(10.0, 1013.25, 20.0, 0.0);
    let r_radio_humid = refraction_radio(10.0, 1013.25, 20.0, 100.0);
    
    // Higher humidity should increase radio refraction
    assert!(r_radio_humid > r_radio_dry);
}