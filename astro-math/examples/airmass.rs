use astro_math::{
    airmass_plane_parallel, airmass_young, airmass_pickering, airmass_kasten_young,
    extinction_magnitudes, extinction_coefficient_estimate
};

fn main() {
    println!("=== Airmass Calculations Example ===\n");

    // Example 1: Compare different airmass formulas
    println!("Airmass vs Altitude (comparison of formulas):");
    println!("Altitude | Zenith Angle | Plane-Parallel | Young | Pickering | Kasten-Young");
    println!("---------|--------------|----------------|-------|-----------|-------------");
    
    for alt in [0.0, 5.0, 10.0, 20.0, 30.0, 45.0, 60.0, 75.0, 90.0] {
        let z = 90.0 - alt;
        let am_pp = if alt > 0.0 { airmass_plane_parallel(alt).unwrap() } else { f64::NAN };
        let am_young = airmass_young(alt).unwrap();
        let am_pickering = airmass_pickering(alt).unwrap();
        let am_ky = if alt > 0.0 { airmass_kasten_young(alt).unwrap() } else { f64::NAN };
        
        println!("{:7.1}° | {:11.1}° | {:14.3} | {:5.3} | {:9.3} | {:11.3}",
            alt, z, am_pp, am_young, am_pickering, am_ky);
    }

    // Example 2: Extinction calculation
    println!("\nExtinction at Different Airmasses:");
    println!("Airmass | Extinction (mag)");
    println!("--------|------------------");
    println!("        | k=0.10 | k=0.15 | k=0.25");
    println!("--------|--------|--------|--------");
    
    for am in [1.0, 1.5, 2.0, 3.0, 5.0] {
        println!("{:7.1} | {:6.3} | {:6.3} | {:6.3}",
            am,
            extinction_magnitudes(am, 0.10),
            extinction_magnitudes(am, 0.15),
            extinction_magnitudes(am, 0.25)
        );
    }

    // Example 3: Wavelength-dependent extinction
    println!("\nExtinction Coefficient vs Wavelength:");
    println!("Wavelength | Band    | k (mag/airmass) | Relative to V");
    println!("-----------|---------|-----------------|---------------");
    
    let k_v = extinction_coefficient_estimate(550.0).unwrap();
    for (wavelength, band) in [
        (380.0, "U"),
        (450.0, "B"),
        (550.0, "V"),
        (650.0, "R"),
        (800.0, "I"),
        (1250.0, "J"),
        (2200.0, "K"),
    ] {
        let k = extinction_coefficient_estimate(wavelength).unwrap();
        println!("{:9.0} nm | {:7} | {:15.3} | {:13.2}",
            wavelength, band, k, k / k_v);
    }

    // Example 4: Observing scenario
    println!("\nObserving Scenario: Object tracking from rise to set");
    println!("Time | Altitude | Airmass | V-band Extinction | Limiting Mag Loss");
    println!("-----|----------|---------|-------------------|------------------");
    
    // Simulate object rising from east to meridian to west
    let altitudes = [5.0, 15.0, 30.0, 45.0, 60.0, 75.0, 60.0, 45.0, 30.0, 15.0, 5.0];
    let times = ["18:00", "19:00", "20:00", "21:00", "22:00", "23:00", 
                 "00:00", "01:00", "02:00", "03:00", "04:00"];
    
    let k_v = 0.15; // Typical V-band extinction
    
    for (i, alt) in altitudes.iter().enumerate() {
        let am = airmass_pickering(*alt).unwrap();
        let extinction = extinction_magnitudes(am, k_v);
        let mag_loss = 2.5 * (10.0_f64.powf(0.4 * extinction)).log10();
        
        println!("{} | {:7.1}° | {:7.3} | {:16.3} mag | {:16.3} mag",
            times[i], alt, am, extinction, mag_loss);
    }

    // Example 5: Planning observations
    println!("\nPlanning Guide - Maximum Airmass for Different Science Cases:");
    println!("Science Case          | Max Airmass | Min Altitude | Notes");
    println!("----------------------|-------------|--------------|-------");
    println!("Photometry (precise)  | 1.5         | 42°          | Minimize extinction");
    println!("Spectroscopy (blue)   | 1.3         | 50°          | Blue sensitive");
    println!("Spectroscopy (red)    | 2.0         | 30°          | Less extinction");
    println!("Survey/discovery      | 2.5         | 24°          | Max sky coverage");
    println!("Near-horizon monitor  | 3.0         | 20°          | Special programs");
}