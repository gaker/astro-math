//! Example demonstrating nutation calculations.

use astro_math::nutation::{nutation, mean_obliquity, true_obliquity};
use astro_math::time::julian_date;
use chrono::{TimeZone, Utc};

fn main() {
    println!("Nutation Example\n");
    
    // Current date
    let now = Utc::now();
    let jd_now = julian_date(now);
    
    println!("Current nutation values:");
    let nut_now = nutation(jd_now);
    let mean_obl = mean_obliquity(jd_now);
    let true_obl = true_obliquity(jd_now);
    
    println!("Date: {}", now.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Nutation in longitude (Δψ): {:+.2}\"", nut_now.longitude);
    println!("Nutation in obliquity (Δε): {:+.2}\"", nut_now.obliquity);
    println!("Mean obliquity (ε₀): {:.6}°", mean_obl);
    println!("True obliquity (ε): {:.6}°", true_obl);
    
    // Show nutation over time
    println!("\nNutation variation over one month:");
    println!("Date       | Δψ (arcsec) | Δε (arcsec)");
    println!("-----------|-------------|------------");
    
    let start_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    for day in 0..30 {
        let dt = start_date + chrono::Duration::days(day);
        let jd = julian_date(dt);
        let nut = nutation(jd);
        
        println!("{} | {:+11.3} | {:+10.3}", 
            dt.format("%Y-%m-%d"), nut.longitude, nut.obliquity);
    }
    
    // Show long-term nutation cycle
    println!("\nNutation over 20 years (showing peaks):");
    println!("Year | Max Δψ (arcsec) | Date of Max");
    println!("-----|-----------------|------------");
    
    for year in 2020..2040 {
        let mut max_dpsi: f64 = 0.0;
        let mut max_date = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        
        // Check every 10 days
        for day in (0..365).step_by(10) {
            let dt = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap() 
                + chrono::Duration::days(day);
            let jd = julian_date(dt);
            let dpsi = nutation(jd).longitude;
            
            if dpsi.abs() > max_dpsi.abs() {
                max_dpsi = dpsi;
                max_date = dt;
            }
        }
        
        println!("{} | {:15.2} | {}", 
            year, max_dpsi, max_date.format("%Y-%m-%d"));
    }
    
    // Show effect on coordinates
    println!("\nEffect of nutation on coordinates:");
    println!("Nutation primarily affects Right Ascension through the equation of the equinoxes.");
    println!("For a star at the vernal equinox (RA=0°, Dec=0°):");
    
    let nut_effect = nutation(jd_now);
    let true_obl_rad = true_obliquity(jd_now).to_radians();
    let eq_equinoxes = nut_effect.longitude * true_obl_rad.cos() / 15.0; // Convert to time
    
    println!("Equation of equinoxes: {:.3} seconds of time", eq_equinoxes);
    println!("This shifts apparent RA by: {:.3}\"", eq_equinoxes * 15.0);
    
    // Historical example
    println!("\nHistorical example (from Meeus):");
    let historical_date = Utc.with_ymd_and_hms(1987, 4, 10, 0, 0, 0).unwrap();
    let jd_hist = julian_date(historical_date);
    let nut_hist = nutation(jd_hist);
    
    println!("Date: 1987-04-10");
    println!("Δψ = {:.3}\" (Meeus: -3.788\")", nut_hist.longitude);
    println!("Δε = {:.3}\" (Meeus: +9.443\")", nut_hist.obliquity);
}