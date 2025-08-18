use astro_math::{
    moon_position, moon_phase_angle, moon_phase_name, moon_illumination,
    moon_distance, moon_equatorial, Location, ra_dec_to_alt_az
};
use chrono::{Datelike, TimeZone, Utc};

fn main() {
    println!("=== Moon Calculations Example ===\n");

    // Example observation time
    let dt = Utc::now();
    println!("Current time: {}", dt.format("%Y-%m-%d %H:%M:%S UTC"));

    // Example 1: Moon phase information
    let phase_angle = moon_phase_angle(dt);
    let phase_name = moon_phase_name(dt);
    let illumination = moon_illumination(dt);
    
    println!("\nMoon Phase Information:");
    println!("  Phase angle: {:.1}°", phase_angle);
    println!("  Phase name: {}", phase_name);
    println!("  Illumination: {:.1}%", illumination);

    // Example 2: Moon position
    let (ecl_lon, ecl_lat) = moon_position(dt);
    let (ra, dec) = moon_equatorial(dt);
    let distance = moon_distance(dt);
    
    println!("\nMoon Position:");
    println!("  Ecliptic: λ = {:.2}°, β = {:.2}°", ecl_lon, ecl_lat);
    println!("  Equatorial: RA = {:.2}°, Dec = {:.2}°", ra, dec);
    println!("  Distance: {:.0} km ({:.2} Earth radii)", distance, distance / 6378.137);

    // Example 3: Moon altitude and azimuth for a specific location
    let location = Location {
        latitude_deg: 40.7128,   // New York City
        longitude_deg: -74.0060,
        altitude_m: 10.0,
    };
    
    let (alt, az) = ra_dec_to_alt_az(ra, dec, dt, &location).unwrap();
    
    println!("\nMoon from New York City:");
    println!("  Altitude: {:.1}°", alt);
    println!("  Azimuth: {:.1}°", az);
    if alt > 0.0 {
        println!("  Status: Above horizon");
    } else {
        println!("  Status: Below horizon");
    }

    // Example 4: Moon phase calendar for the month
    println!("\nMoon Phases This Month:");
    println!("Date       | Phase Angle | Phase Name      | Illumination");
    println!("-----------|-------------|-----------------|-------------");
    
    let start_of_month = Utc.with_ymd_and_hms(
        dt.year(), dt.month(), 1, 0, 0, 0
    ).unwrap();
    
    for day in 0..30 {
        let date = start_of_month + chrono::Duration::days(day);
        let phase = moon_phase_angle(date);
        let name = moon_phase_name(date);
        let illum = moon_illumination(date);
        
        println!("{} | {:10.1}° | {:<15} | {:10.1}%",
            date.format("%Y-%m-%d"),
            phase,
            name,
            illum
        );
        
        // Highlight special phases
        if day > 0 {
            let prev_date = start_of_month + chrono::Duration::days(day - 1);
            let prev_phase = moon_phase_angle(prev_date);
            
            // Check for phase transitions
            if prev_phase > 350.0 && phase < 10.0 {
                println!("           >>> New Moon <<<");
            } else if prev_phase < 90.0 && phase >= 90.0 {
                println!("           >>> First Quarter <<<");
            } else if prev_phase < 180.0 && phase >= 180.0 {
                println!("           >>> Full Moon <<<");
            } else if prev_phase < 270.0 && phase >= 270.0 {
                println!("           >>> Last Quarter <<<");
            }
        }
    }

    // Example 5: Perigee/Apogee detection
    println!("\nSearching for lunar perigee/apogee in next 30 days...");
    let mut min_dist = f64::MAX;
    let mut max_dist = f64::MIN;
    let mut min_date = dt;
    let mut max_date = dt;
    
    for hours in 0..720 { // 30 days
        let check_date = dt + chrono::Duration::hours(hours);
        let dist = moon_distance(check_date);
        
        if dist < min_dist {
            min_dist = dist;
            min_date = check_date;
        }
        if dist > max_dist {
            max_dist = dist;
            max_date = check_date;
        }
    }
    
    println!("  Perigee: {} at {:.0} km", min_date.format("%Y-%m-%d"), min_dist);
    println!("  Apogee:  {} at {:.0} km", max_date.format("%Y-%m-%d"), max_dist);
}