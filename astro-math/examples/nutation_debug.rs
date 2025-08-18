//! Debug nutation calculations to understand the values.

use astro_math::nutation::nutation_in_longitude;
use astro_math::time::julian_date;
use chrono::{TimeZone, Utc};

fn main() {
    // Find the maximum nutation values over a period
    let start_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    
    let mut max_dpsi = 0.0;
    let mut max_date = start_date;
    let mut min_dpsi = 0.0;
    let mut min_date = start_date;
    
    // Check over ~20 years (more than one nutation period)
    for days in 0..7300 {
        let dt = start_date + chrono::Duration::days(days);
        let jd = julian_date(dt);
        let dpsi = nutation_in_longitude(jd);
        
        if dpsi > max_dpsi {
            max_dpsi = dpsi;
            max_date = dt;
        }
        if dpsi < min_dpsi {
            min_dpsi = dpsi;
            min_date = dt;
        }
    }
    
    println!("Maximum nutation in longitude: {:.6}\" on {}", max_dpsi, max_date.format("%Y-%m-%d"));
    println!("Minimum nutation in longitude: {:.6}\" on {}", min_dpsi, min_date.format("%Y-%m-%d"));
    println!("Total range: {:.6}\"", max_dpsi - min_dpsi);
    
    // Now let's break down the contributions at the maximum
    println!("\nBreaking down contributions at maximum:");
    let jd_max = julian_date(max_date);
    let t = (jd_max - 2451545.0) / 36525.0;
    
    // Fundamental arguments
    let d = 297.85036 + 445267.111480 * t;
    let m = 357.52772 + 35999.050340 * t;
    let m_prime = 134.96298 + 477198.867398 * t;
    let f = 93.27191 + 483202.017538 * t;
    let omega = 125.04452 - 1934.136261 * t;
    
    let d_rad = d.to_radians();
    let m_rad = m.to_radians();
    let m_prime_rad = m_prime.to_radians();
    let f_rad = f.to_radians();
    let omega_rad = omega.to_radians();
    
    println!("Fundamental arguments (degrees):");
    println!("D (Moon's elongation): {:.2}°", d % 360.0);
    println!("M (Sun's anomaly): {:.2}°", m % 360.0);
    println!("M' (Moon's anomaly): {:.2}°", m_prime % 360.0);
    println!("F (Moon's latitude arg): {:.2}°", f % 360.0);
    println!("Ω (Moon's node): {:.2}°", omega % 360.0);
    
    // Calculate each term
    let terms = [
        ([0, 0, 0, 0, 1], -171996.0, -174.2, "Primary (Ω)"),
        ([-2, 0, 0, 2, 2], -13187.0, -1.6, "-2D+2F+2Ω"),
        ([0, 0, 0, 2, 2], -2274.0, -0.2, "2F+2Ω"),
        ([2, 0, 0, -2, 0], 2062.0, 0.2, "2D-2F"),
        ([0, 0, 0, 0, 2], 1426.0, -3.4, "2Ω"),
        ([0, 1, 0, 0, 0], 712.0, 0.1, "M"),
        ([-2, 1, 0, 2, 2], -517.0, 1.2, "-2D+M+2F+2Ω"),
        ([0, 0, 0, 2, 1], -386.0, -0.4, "2F+Ω"),
        ([0, 0, 1, 0, 0], -301.0, 0.0, "M'"),
        ([-2, 0, 0, 2, 1], 217.0, -0.5, "-2D+2F+Ω"),
    ];
    
    println!("\nIndividual term contributions:");
    let mut total = 0.0;
    
    for (args, coeff_sin, coeff_sin_t, name) in terms.iter() {
        let argument = args[0] as f64 * d_rad 
                     + args[1] as f64 * m_rad 
                     + args[2] as f64 * m_prime_rad 
                     + args[3] as f64 * f_rad 
                     + args[4] as f64 * omega_rad;
        
        let contribution = (coeff_sin + coeff_sin_t * t) * argument.sin() / 10000.0;
        total += contribution;
        
        println!("{:20} : {:+8.3}\" (arg = {:.1}°)", 
            name, contribution, argument.to_degrees() % 360.0);
    }
    
    println!("\nTotal: {:.3}\"", total);
}