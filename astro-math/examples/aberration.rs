//! Example demonstrating stellar aberration calculations.

use astro_math::aberration::{apply_aberration, aberration_magnitude, remove_aberration};
use chrono::{TimeZone, Utc};

fn main() {
    println!("Stellar Aberration Example\n");
    
    // Vega's J2000.0 coordinates
    let ra_j2000 = 279.23473479;
    let dec_j2000 = 38.78368896;
    println!("Vega J2000.0 position: RA = {:.6}°, Dec = {:.6}°", ra_j2000, dec_j2000);
    
    // Calculate aberration for different times of year
    println!("\nAberration throughout the year:");
    println!("Date          | Apparent RA    | Apparent Dec   | Aberration");
    println!("--------------|----------------|----------------|------------");
    
    for month in [1, 4, 7, 10] {
        let dt = Utc.with_ymd_and_hms(2024, month, 15, 0, 0, 0).unwrap();
        let (ra_app, dec_app) = apply_aberration(ra_j2000, dec_j2000, dt).unwrap();
        let mag = aberration_magnitude(ra_j2000, dec_j2000, dt).unwrap();
        
        println!("{:13} | {:14.6}° | {:14.6}° | {:6.2}\"",
            dt.format("%Y-%m-%d"), ra_app, dec_app, mag);
    }
    
    // Demonstrate the inverse operation
    println!("\nInverse aberration (converting apparent to mean position):");
    let observation_date = Utc.with_ymd_and_hms(2024, 8, 15, 22, 30, 0).unwrap();
    let (ra_apparent, dec_apparent) = apply_aberration(ra_j2000, dec_j2000, observation_date).unwrap();
    
    println!("Observation time: {}", observation_date);
    println!("Apparent position: RA = {:.6}°, Dec = {:.6}°", ra_apparent, dec_apparent);
    
    let (ra_recovered, dec_recovered) = remove_aberration(ra_apparent, dec_apparent, observation_date).unwrap();
    println!("Recovered J2000: RA = {:.6}°, Dec = {:.6}°", ra_recovered, dec_recovered);
    println!("Difference: ΔRA = {:.3}\", ΔDec = {:.3}\"", 
        (ra_recovered - ra_j2000) * 3600.0, 
        (dec_recovered - dec_j2000) * 3600.0);
    
    // Show maximum aberration
    println!("\nMaximum aberration for stars at different declinations:");
    println!("Declination | Max Aberration");
    println!("------------|---------------");
    
    for dec in [0.0, 30.0, 60.0, 85.0] {
        let mut max_aberration = 0.0;
        
        // Check throughout the year
        for day in 0..365 {
            let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap() + chrono::Duration::days(day);
            // Test a star at RA that will be 90° from sun
            for ra in [0.0, 90.0, 180.0, 270.0] {
                if let Ok(mag) = aberration_magnitude(ra, dec, dt) {
                    if mag > max_aberration {
                        max_aberration = mag;
                    }
                }
            }
        }
        
        println!("{:11.0}° | {:13.1}\"", dec, max_aberration);
    }
}