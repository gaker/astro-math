use astro_math::{Location, diurnal_parallax, annual_parallax, geocentric_distance};
use chrono::{TimeZone, Utc};

fn main() {
    println!("=== Parallax Corrections Example ===\n");

    // Example location: Mauna Kea Observatory, Hawaii
    let location = Location {
        latitude_deg: 19.8207,
        longitude_deg: -155.4681,
        altitude_m: 4207.0,
    };

    println!("Observer Location: Mauna Kea Observatory");
    println!("  Latitude:  {:.4}°", location.latitude_deg);
    println!("  Longitude: {:.4}°", location.longitude_deg);
    println!("  Altitude:  {:.0} m", location.altitude_m);
    println!("  Geocentric distance: {:.6} Earth radii\n", geocentric_distance(&location));

    // Example 1: Moon's diurnal parallax
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 10, 0, 0).unwrap();
    let moon_ra = 45.0;
    let moon_dec = 20.0;
    let moon_distance_au = 0.00257; // ~384,400 km
    
    let (ra_topo, dec_topo) = diurnal_parallax(moon_ra, moon_dec, moon_distance_au, dt, &location);
    
    println!("Moon's Diurnal Parallax:");
    println!("  Geocentric: RA = {:.4}°, Dec = {:.4}°", moon_ra, moon_dec);
    println!("  Topocentric: RA = {:.4}°, Dec = {:.4}°", ra_topo, dec_topo);
    println!("  Parallax: ΔRA = {:.4}°, ΔDec = {:.4}°", ra_topo - moon_ra, dec_topo - moon_dec);
    println!("  Max parallax: ~{:.1}' (at horizon)\n", 8.794 * 3600.0 / (moon_distance_au * 149597870.7 / 6378.137) / 60.0);

    // Example 2: Near-Earth asteroid parallax
    let asteroid_ra = 120.0;
    let asteroid_dec = -15.0;
    let asteroid_distance_au = 0.1; // Very close approach
    
    let (ra_ast_topo, dec_ast_topo) = diurnal_parallax(asteroid_ra, asteroid_dec, asteroid_distance_au, dt, &location);
    
    println!("Near-Earth Asteroid Diurnal Parallax (0.1 AU):");
    println!("  Geocentric: RA = {:.4}°, Dec = {:.4}°", asteroid_ra, asteroid_dec);
    println!("  Topocentric: RA = {:.4}°, Dec = {:.4}°", ra_ast_topo, dec_ast_topo);
    println!("  Parallax: ΔRA = {:.4}°, ΔDec = {:.4}°\n", ra_ast_topo - asteroid_ra, dec_ast_topo - asteroid_dec);

    // Example 3: Annual parallax for nearby stars
    println!("Annual Parallax for Nearby Stars:");
    
    // Proxima Centauri
    let proxima_ra = 217.42894;
    let proxima_dec = -62.67948;
    let proxima_parallax = 768.5; // milliarcseconds
    
    let (ra_jan, dec_jan) = annual_parallax(proxima_ra, proxima_dec, proxima_parallax, 
        Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap());
    let (ra_jul, dec_jul) = annual_parallax(proxima_ra, proxima_dec, proxima_parallax, 
        Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap());
    
    println!("  Proxima Centauri (π = {:.1} mas):", proxima_parallax);
    println!("    January:  ΔRA = {:.1} mas, ΔDec = {:.1} mas", 
        (ra_jan - proxima_ra) * 3600000.0, (dec_jan - proxima_dec) * 3600000.0);
    println!("    July:     ΔRA = {:.1} mas, ΔDec = {:.1} mas", 
        (ra_jul - proxima_ra) * 3600000.0, (dec_jul - proxima_dec) * 3600000.0);
    
    // Barnard's Star
    let barnard_ra = 269.452;
    let barnard_dec = 4.693;
    let barnard_parallax = 546.0;
    
    let (ra_jan_b, dec_jan_b) = annual_parallax(barnard_ra, barnard_dec, barnard_parallax, 
        Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap());
    let (ra_jul_b, dec_jul_b) = annual_parallax(barnard_ra, barnard_dec, barnard_parallax, 
        Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap());
    
    println!("\n  Barnard's Star (π = {:.1} mas):", barnard_parallax);
    println!("    January:  ΔRA = {:.1} mas, ΔDec = {:.1} mas", 
        (ra_jan_b - barnard_ra) * 3600000.0, (dec_jan_b - barnard_dec) * 3600000.0);
    println!("    July:     ΔRA = {:.1} mas, ΔDec = {:.1} mas", 
        (ra_jul_b - barnard_ra) * 3600000.0, (dec_jul_b - barnard_dec) * 3600000.0);
}