//! Example: Proper motion calculations
//!
//! Demonstrates how to apply proper motion corrections to stellar coordinates,
//! both using the simple linear method and the rigorous 3D space motion method.

use astro_math::{
    apply_proper_motion, apply_proper_motion_rigorous, 
    total_proper_motion, proper_motion_position_angle
};
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Proper Motion Examples ===\n");
    
    // Example 1: Barnard's Star - highest proper motion
    println!("1. Barnard's Star (highest proper motion):");
    let ra_j2000 = 269.454022;
    let dec_j2000 = 4.668288;
    let pm_ra_cosdec = -797.84;  // mas/yr (already includes cos(dec))
    let pm_dec = 10326.93;       // mas/yr
    
    // Calculate total proper motion and direction
    let total_pm = total_proper_motion(pm_ra_cosdec, pm_dec);
    let pa = proper_motion_position_angle(pm_ra_cosdec, pm_dec);
    
    println!("   J2000 position: RA = {:.3}°, Dec = {:.3}°", ra_j2000, dec_j2000);
    println!("   Proper motion: μα* = {:.1} mas/yr, μδ = {:.1} mas/yr", pm_ra_cosdec, pm_dec);
    println!("   Total: {:.1} mas/yr at PA = {:.1}°", total_pm, pa);
    
    // Calculate positions at different epochs
    for year in [2024, 2050, 2100] {
        let epoch = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
        let (ra, dec) = apply_proper_motion(ra_j2000, dec_j2000, pm_ra_cosdec, pm_dec, epoch).unwrap();
        let _years_elapsed = year - 2000;
        println!("   {:4}: RA = {:.3}°, Dec = {:.3}° (moved {:.3}°)", 
            year, ra, dec, ((ra - ra_j2000).powi(2) + (dec - dec_j2000).powi(2)).sqrt());
    }
    
    // Example 2: Proxima Centauri with rigorous method
    println!("\n2. Proxima Centauri (nearest star, with space velocity):");
    let ra_j2000_prox = 217.428953;
    let dec_j2000_prox = -62.679484;
    let pm_ra_cosdec_prox = -3775.40;
    let pm_dec_prox = 769.33;
    let parallax_prox = 768.5;  // mas
    let rv_prox = -22.4;        // km/s (approaching)
    
    println!("   J2000 position: RA = {:.3}°, Dec = {:.3}°", ra_j2000_prox, dec_j2000_prox);
    println!("   Proper motion: μα* = {:.1} mas/yr, μδ = {:.1} mas/yr", pm_ra_cosdec_prox, pm_dec_prox);
    println!("   Parallax: {:.1} mas (distance = {:.2} pc)", parallax_prox, 1000.0/parallax_prox);
    println!("   Radial velocity: {:.1} km/s", rv_prox);
    
    // Compare simple vs rigorous methods
    let epoch_2050 = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    let (ra_simple, dec_simple) = apply_proper_motion(
        ra_j2000_prox, dec_j2000_prox, pm_ra_cosdec_prox, pm_dec_prox, epoch_2050
    ).unwrap();
    
    let (ra_rigorous, dec_rigorous, plx_new) = apply_proper_motion_rigorous(
        ra_j2000_prox, dec_j2000_prox, pm_ra_cosdec_prox, pm_dec_prox, 
        parallax_prox, rv_prox, epoch_2050
    ).unwrap();
    
    println!("\n   Position at 2050:");
    println!("   Simple method:    RA = {:.3}°, Dec = {:.3}°", ra_simple, dec_simple);
    println!("   Rigorous method:  RA = {:.3}°, Dec = {:.3}°", ra_rigorous, dec_rigorous);
    println!("   Difference:       ΔRA = {:.3}°, ΔDec = {:.3}°", 
        ra_rigorous - ra_simple, dec_rigorous - dec_simple);
    println!("   New parallax:     {:.1} mas (distance = {:.2} pc)", plx_new, 1000.0/plx_new);
    
    // Example 3: Various high proper motion stars
    println!("\n3. Other high proper motion stars:");
    
    let stars = [
        ("Kapteyn's Star", 77.287, -45.017, -5723.17, -3478.16),
        ("Lacaille 9352", 348.326, -35.853, -6766.63, 1327.99),
        ("Wolf 359", 164.120, 7.016, -3842.0, -2725.0),
        ("61 Cygni A", 316.736, 38.750, -5281.58, -3585.71),
    ];
    
    for (name, ra, dec, pm_ra, pm_dec) in stars {
        let total = total_proper_motion(pm_ra, pm_dec);
        let pa = proper_motion_position_angle(pm_ra, pm_dec);
        println!("   {:<15} μ = {:6.1} mas/yr at PA = {:5.1}°", name, total, pa);
        
        // Position in 2024
        let epoch_2024 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let (ra_2024, dec_2024) = apply_proper_motion(ra, dec, pm_ra, pm_dec, epoch_2024).unwrap();
        println!("                   2024: RA = {:.3}°, Dec = {:.3}°", ra_2024, dec_2024);
    }
    
    // Example 4: Effect of proper motion on observations
    println!("\n4. Effect on telescope pointing:");
    let vega_ra = 279.234735;
    let vega_dec = 38.783689;
    let vega_pm_ra = 200.94;  // mas/yr
    let vega_pm_dec = 286.23;  // mas/yr
    
    println!("   Vega J2000: RA = {:.6}°, Dec = {:.6}°", vega_ra, vega_dec);
    
    // Calculate current position
    let now = Utc::now();
    let (ra_now, dec_now) = apply_proper_motion(vega_ra, vega_dec, vega_pm_ra, vega_pm_dec, now).unwrap();
    
    println!("   Vega now:   RA = {:.6}°, Dec = {:.6}°", ra_now, dec_now);
    println!("   Difference: ΔRA = {:.1} arcsec, ΔDec = {:.1} arcsec", 
        (ra_now - vega_ra) * 3600.0, (dec_now - vega_dec) * 3600.0);
    
    // For a telescope with 10 arcsec field of view
    let fov_arcsec = 10.0;
    let motion_arcsec = ((ra_now - vega_ra).powi(2) + (dec_now - vega_dec).powi(2)).sqrt() * 3600.0;
    
    if motion_arcsec > fov_arcsec / 2.0 {
        println!("    Vega has moved {:.1}\" - outside a {}\" FOV!", motion_arcsec, fov_arcsec);
    } else {
        println!("    Vega has moved {:.1}\" - still within a {}\" FOV", motion_arcsec, fov_arcsec);
    }
}