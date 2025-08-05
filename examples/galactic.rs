use astro_math::{equatorial_to_galactic, galactic_to_equatorial, galactic_landmarks};

fn main() {
    println!("=== Galactic Coordinates Example ===\n");

    // Example 1: Convert famous objects to galactic coordinates
    println!("Famous Objects in Galactic Coordinates:");
    println!("Object                | RA (J2000) | Dec (J2000) | l (gal) | b (gal)");
    println!("----------------------|------------|-------------|---------|--------");
    
    let objects = [
        ("Galactic Center", 266.417, -29.008),
        ("Vega", 279.234, 38.784),
        ("Sirius", 101.287, -16.716),
        ("Polaris", 37.955, 89.264),
        ("Alpha Centauri", 219.901, -60.834),
        ("Betelgeuse", 88.793, 7.407),
        ("M31 (Andromeda)", 10.685, 41.269),
        ("M42 (Orion Nebula)", 83.822, -5.391),
        ("Crab Nebula", 83.633, 22.015),
        ("Vernal Equinox", 0.0, 0.0),
    ];
    
    for (name, ra, dec) in objects {
        let (l, b) = equatorial_to_galactic(ra, dec).unwrap();
        println!("{:<21} | {:10.3}° | {:11.3}° | {:7.2}° | {:7.2}°", 
            name, ra, dec, l, b);
    }

    // Example 2: Show galactic plane
    println!("\nObjects Along the Galactic Plane (b ≈ 0°):");
    println!("Galactic Longitude | RA (J2000) | Dec (J2000) | Description");
    println!("-------------------|------------|-------------|-------------");
    
    for l in (0..360).step_by(30) {
        let b = 0.0;
        let (ra, dec) = galactic_to_equatorial(l as f64, b).unwrap();
        let description = match l {
            0 => "Galactic Center",
            90 => "Direction of rotation",
            180 => "Galactic Anticenter",
            270 => "Opposite rotation",
            _ => "",
        };
        println!("{:17}° | {:10.3}° | {:11.3}° | {}", 
            l, ra, dec, description);
    }

    // Example 3: Galactic poles
    println!("\nGalactic Poles:");
    let (ra_ngp, dec_ngp) = galactic_to_equatorial(0.0, 90.0).unwrap();
    let (ra_sgp, dec_sgp) = galactic_to_equatorial(0.0, -90.0).unwrap();
    println!("North Galactic Pole: RA = {:.3}°, Dec = {:.3}°", ra_ngp, dec_ngp);
    println!("South Galactic Pole: RA = {:.3}°, Dec = {:.3}°", ra_sgp, dec_sgp);

    // Example 4: Round-trip conversion test
    println!("\nRound-trip Conversion Test:");
    let test_ra = 123.456;
    let test_dec = -45.678;
    let (l, b) = equatorial_to_galactic(test_ra, test_dec).unwrap();
    let (ra2, dec2) = galactic_to_equatorial(l, b).unwrap();
    println!("Original:  RA = {:.6}°, Dec = {:.6}°", test_ra, test_dec);
    println!("Galactic:  l = {:.6}°, b = {:.6}°", l, b);
    println!("Converted: RA = {:.6}°, Dec = {:.6}°", ra2, dec2);
    println!("Error:     ΔRA = {:.1e}°, ΔDec = {:.1e}°", 
        (ra2 - test_ra).abs(), (dec2 - test_dec).abs());

    // Example 5: Show galactic landmarks
    println!("\nGalactic Landmarks:");
    for (name, l, b) in galactic_landmarks() {
        let (ra, dec) = galactic_to_equatorial(l, b).unwrap();
        println!("{:<25} | l={:7.1}°, b={:7.1}° | RA={:7.2}°, Dec={:7.2}°",
            name, l, b, ra, dec);
    }

    // Example 6: Milky Way band visualization
    println!("\nMilky Way Band (objects within ±10° of galactic plane):");
    println!("The Milky Way appears as a band because we're inside the disk.");
    println!("Objects with |b| < 10° are within the visible Milky Way:");
    
    let bright_stars: [(&str, f64, f64); 5] = [
        ("Deneb", 84.28, 2.0),
        ("Altair", 47.74, -9.0),
        ("Antares", 351.95, 15.1),
        ("Spica", 316.12, 50.8),
        ("Arcturus", 15.18, 69.1),
    ];
    
    for (name, l, b) in bright_stars {
        let in_band = if b.abs() < 10.0 { "Yes" } else { "No" };
        println!("  {:<10} (l={:6.1}°, b={:+6.1}°) - In Milky Way band: {}", 
            name, l, b, in_band);
    }
}