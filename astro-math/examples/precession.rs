use astro_math::{precess_from_j2000, precess_to_j2000, precession::get_precession_angles};
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Precession Example ===\n");

    // Test precession angles for J2050.0
    let jd_2050 = 2469807.5;
    let (zeta, z, theta) = get_precession_angles(jd_2050);
    println!("Precession angles for J2050.0:");
    println!("  ζ = {:.7}°", zeta);
    println!("  z = {:.7}°", z);
    println!("  θ = {:.7}°\n", theta);

    // Example 1: Precess Vega from J2000 to current date
    let vega_ra_j2000 = 279.23473479;
    let vega_dec_j2000 = 38.78368896;
    let now = Utc::now();
    
    let (ra_now, dec_now) = precess_from_j2000(vega_ra_j2000, vega_dec_j2000, now).unwrap();
    println!("Vega (α Lyrae) precession:");
    println!("  J2000.0: RA = {:.5}°, Dec = {:.5}°", vega_ra_j2000, vega_dec_j2000);
    println!("  Now:     RA = {:.5}°, Dec = {:.5}°", ra_now, dec_now);
    println!("  ΔRA = {:.5}°, ΔDec = {:.5}°\n", ra_now - vega_ra_j2000, dec_now - vega_dec_j2000);

    // Example 2: Precess Polaris over 50 years
    let polaris_ra_j2000 = 37.95456067;
    let polaris_dec_j2000 = 89.26410897;
    let dt_2050 = Utc.with_ymd_and_hms(2050, 1, 1, 0, 0, 0).unwrap();
    
    let (ra_2050, dec_2050) = precess_from_j2000(polaris_ra_j2000, polaris_dec_j2000, dt_2050).unwrap();
    println!("Polaris (α UMi) precession to 2050:");
    println!("  J2000.0: RA = {:.5}°, Dec = {:.5}°", polaris_ra_j2000, polaris_dec_j2000);
    println!("  2050:    RA = {:.5}°, Dec = {:.5}°", ra_2050, dec_2050);
    println!("  ΔRA = {:.5}° (large due to proximity to pole)", ra_2050 - polaris_ra_j2000);

    // Example 3: Round-trip test
    println!("\nRound-trip precession test:");
    let test_ra = 83.633;
    let test_dec = 22.0145;
    let dt_2025 = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    
    let (ra_2025, dec_2025) = precess_from_j2000(test_ra, test_dec, dt_2025).unwrap();
    let (ra_back, dec_back) = precess_to_j2000(ra_2025, dec_2025, dt_2025).unwrap();
    
    println!("  Original:  RA = {:.6}°, Dec = {:.6}°", test_ra, test_dec);
    println!("  → 2025:    RA = {:.6}°, Dec = {:.6}°", ra_2025, dec_2025);
    println!("  → J2000:   RA = {:.6}°, Dec = {:.6}°", ra_back, dec_back);
    println!("  Error:     ΔRA = {:.1e}°, ΔDec = {:.1e}°", 
        (ra_back - test_ra).abs(), (dec_back - test_dec).abs());
}