use crate::galactic::*;

#[test]
fn test_vernal_equinox() {
    // Vernal equinox point (RA=0, Dec=0)
    let (l, b) = equatorial_to_galactic(0.0, 0.0).unwrap();
    
    // Should be approximately l=96.3°, b=-60.2°
    assert!((l - 96.3).abs() < 1.0);
    assert!((b - (-60.2)).abs() < 1.0);
}

#[test]
fn test_galactic_plane_objects() {
    // Objects in the galactic plane should have b ≈ 0
    let galactic_objects = [
        (266.405, -28.936),  // Sgr A*
        (284.0, -1.0),       // Approximate galactic plane object
    ];
    
    for (ra, dec) in galactic_objects {
        let (_, b) = equatorial_to_galactic(ra, dec).unwrap();
        assert!(b.abs() < 5.0); // Within 5 degrees of galactic plane
    }
}

#[test]
fn test_high_galactic_latitude() {
    // North celestial pole
    let (_l, b) = equatorial_to_galactic(0.0, 90.0).unwrap();
    assert!((b - 27.13).abs() < 0.5); // Should be at galactic latitude ~27°
    
    // South celestial pole
    let (_l, b) = equatorial_to_galactic(0.0, -90.0).unwrap();
    assert!((b - (-27.13)).abs() < 0.5);
}

#[test]
fn test_coordinate_ranges() {
    // Test various coordinates to ensure output is in valid ranges
    let test_coords = [
        (0.0, 0.0),
        (180.0, 0.0),
        (359.9, 0.0),
        (45.0, 45.0),
        (270.0, -45.0),
    ];
    
    for (ra, dec) in test_coords {
        let (l, b) = equatorial_to_galactic(ra, dec).unwrap();
        assert!(l >= 0.0 && l < 360.0, "l out of range: {}", l);
        assert!(b >= -90.0 && b <= 90.0, "b out of range: {}", b);
        
        // Test reverse conversion
        let (ra2, dec2) = galactic_to_equatorial(l, b).unwrap();
        assert!(ra2 >= 0.0 && ra2 < 360.0, "RA out of range: {}", ra2);
        assert!(dec2 >= -90.0 && dec2 <= 90.0, "Dec out of range: {}", dec2);
    }
}

#[test]
fn test_galactic_landmarks() {
    // Test the galactic_landmarks function
    let landmarks = galactic_landmarks();
    assert!(!landmarks.is_empty(), "Should have galactic landmarks");
    assert!(landmarks.len() >= 5, "Should have at least 5 major landmarks, got {}", landmarks.len());
    
    // Check that galactic center is included
    let gc = landmarks.iter().find(|(name, _, _)| *name == "Galactic Center");
    assert!(gc.is_some(), "Must include Galactic Center in landmarks");
    let (_, l, b) = gc.unwrap();
    assert_eq!(*l, 0.0, "Galactic Center should be at l=0");
    assert_eq!(*b, 0.0, "Galactic Center should be at b=0");
    
    // Check for other expected landmarks
    let has_anticenter = landmarks.iter().any(|(name, _, _)| name.contains("Anticenter"));
    let has_ngp = landmarks.iter().any(|(name, _, _)| name.contains("Galactic North Pole"));
    assert!(has_anticenter, "Should include Galactic Anticenter");
    assert!(has_ngp, "Should include Galactic North Pole");
    
    // Check all landmarks have valid coordinates and meaningful names
    for (name, l, b) in landmarks {
        assert!(!name.is_empty(), "Landmark name should not be empty");
        assert!(name.len() > 5, "Landmark name should be descriptive, got: {}", name);
        assert!(l >= 0.0 && l < 360.0, "Galactic longitude should be [0,360), got {}", l);
        assert!(b >= -90.0 && b <= 90.0, "Galactic latitude should be [-90,90], got {}", b);
    }
}

#[test]
fn test_galactic_constants() {
    // Test galactic constants have correct values
    assert!((NGP_RA - 192.86).abs() < 0.01, "NGP RA should be ~192.86°, got {}", NGP_RA);
    assert!((NGP_DEC - 27.13).abs() < 0.01, "NGP Dec should be ~27.13°, got {}", NGP_DEC);
    assert!((GC_RA - 266.405).abs() < 0.01, "GC RA should be ~266.405°, got {}", GC_RA);
    assert!((GC_DEC - (-28.936)).abs() < 0.01, "GC Dec should be ~-28.936°, got {}", GC_DEC);
}