use crate::galactic::*;

#[test]
fn test_vernal_equinox() {
    // Vernal equinox point (RA=0, Dec=0)
    let (l, b) = equatorial_to_galactic(0.0, 0.0);
    
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
        let (_, b) = equatorial_to_galactic(ra, dec);
        assert!(b.abs() < 5.0); // Within 5 degrees of galactic plane
    }
}

#[test]
fn test_high_galactic_latitude() {
    // North celestial pole
    let (_l, b) = equatorial_to_galactic(0.0, 90.0);
    assert!((b - 27.13).abs() < 0.5); // Should be at galactic latitude ~27°
    
    // South celestial pole
    let (_l, b) = equatorial_to_galactic(0.0, -90.0);
    assert!((b - (-27.13)).abs() < 0.5);
}

#[test]
fn test_coordinate_ranges() {
    // Test various coordinates to ensure output is in valid ranges
    let test_coords = [
        (0.0, 0.0),
        (180.0, 0.0),
        (360.0, 0.0),
        (45.0, 45.0),
        (270.0, -45.0),
    ];
    
    for (ra, dec) in test_coords {
        let (l, b) = equatorial_to_galactic(ra, dec);
        assert!(l >= 0.0 && l < 360.0, "l out of range: {}", l);
        assert!(b >= -90.0 && b <= 90.0, "b out of range: {}", b);
        
        // Test reverse conversion
        let (ra2, dec2) = galactic_to_equatorial(l, b);
        assert!(ra2 >= 0.0 && ra2 < 360.0, "RA out of range: {}", ra2);
        assert!(dec2 >= -90.0 && dec2 <= 90.0, "Dec out of range: {}", dec2);
    }
}

#[test]
fn test_galactic_landmarks() {
    // Test the galactic_landmarks function
    let landmarks = galactic_landmarks();
    assert!(!landmarks.is_empty());
    
    // Check that galactic center is included
    let gc = landmarks.iter().find(|(name, _, _)| *name == "Galactic Center");
    assert!(gc.is_some());
    let (_, l, b) = gc.unwrap();
    assert_eq!(*l, 0.0);
    assert_eq!(*b, 0.0);
    
    // Check all landmarks have valid coordinates
    for (name, l, b) in landmarks {
        assert!(!name.is_empty());
        assert!(l >= 0.0 && l < 360.0);
        assert!(b >= -90.0 && b <= 90.0);
    }
}

#[test]
fn test_galactic_constants() {
    // Test access to galactic constants
    assert!(NGP_RA > 0.0);
    assert!(NGP_DEC > 0.0);
    assert!(GC_RA > 0.0);
    assert!(GC_DEC < 0.0); // Negative declination
}