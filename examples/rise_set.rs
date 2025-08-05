use astro_math::{
    Location, moon_equatorial,
    rise_set::{rise_transit_set, next_rise, next_set, sun_rise_set}
};
use chrono::{Datelike, TimeZone, Utc};

fn main() {
    println!("=== Rise, Set, and Transit Times Example ===\n");

    // Example location: New York City
    let location = Location {
        latitude_deg: 40.7128,
        longitude_deg: -74.0060,
        altitude_m: 10.0,
    };

    println!("Location: New York City");
    println!("  Latitude:  {:.4}°", location.latitude_deg);
    println!("  Longitude: {:.4}°\n", location.longitude_deg);

    // Example 1: Sun rise and set times
    let today = Utc::now();
    let noon_today = Utc.with_ymd_and_hms(
        today.year(), today.month(), today.day(), 12, 0, 0
    ).unwrap();
    
    println!("Sun Rise/Set Times for {}:", today.format("%Y-%m-%d"));
    if let Some((sunrise, sunset)) = sun_rise_set(noon_today, &location) {
        let daylight = sunset - sunrise;
        println!("  Sunrise: {} UTC", sunrise.format("%H:%M:%S"));
        println!("  Sunset:  {} UTC", sunset.format("%H:%M:%S"));
        println!("  Daylight: {} hours {} minutes", 
            daylight.num_hours(), 
            daylight.num_minutes() % 60
        );
    } else {
        println!("  Sun does not rise or set (polar day/night)");
    }

    // Example 2: Bright star (Vega) rise/transit/set
    let vega_ra = 279.23473479;
    let vega_dec = 38.78368896;
    
    println!("\nVega Rise/Transit/Set Times:");
    if let Some((rise, transit, set)) = rise_transit_set(vega_ra, vega_dec, noon_today, &location, None) {
        let above_horizon = set - rise;
        println!("  Rise:    {} UTC", rise.format("%H:%M:%S"));
        println!("  Transit: {} UTC (altitude: ~{:.1}°)", 
            transit.format("%H:%M:%S"),
            90.0 - (location.latitude_deg - vega_dec).abs()
        );
        println!("  Set:     {} UTC", set.format("%H:%M:%S"));
        println!("  Above horizon: {} hours {} minutes",
            above_horizon.num_hours(),
            above_horizon.num_minutes() % 60
        );
    } else {
        println!("  Vega is circumpolar or never visible from this location");
    }

    // Example 3: Moon rise and set
    let (moon_ra, moon_dec) = moon_equatorial(today);
    
    println!("\nMoon Rise/Set Times:");
    println!("  Current position: RA={:.2}°, Dec={:.2}°", moon_ra, moon_dec);
    
    if let Some(rise) = next_rise(moon_ra, moon_dec, today, &location, None) {
        println!("  Next rise: {} UTC", rise.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("  Moon does not rise");
    }
    
    if let Some(set) = next_set(moon_ra, moon_dec, today, &location, None) {
        println!("  Next set:  {} UTC", set.format("%Y-%m-%d %H:%M:%S"));
    } else {
        println!("  Moon does not set");
    }

    // Example 4: Different objects at different latitudes
    println!("\nRise/Set at Different Latitudes (for RA=0°, Dec=0°):");
    println!("Latitude | Rise Time | Set Time  | Hours Up");
    println!("---------|-----------|-----------|----------");
    
    for lat in [-60.0, -30.0, 0.0, 30.0, 60.0] {
        let loc = Location {
            latitude_deg: lat,
            longitude_deg: 0.0,
            altitude_m: 0.0,
        };
        
        if let Some((rise, _, set)) = rise_transit_set(0.0, 0.0, noon_today, &loc, None) {
            let hours_up = (set - rise).num_minutes() as f64 / 60.0;
            println!("{:7.0}° | {} | {} | {:8.1}",
                lat,
                rise.format("%H:%M:%S"),
                set.format("%H:%M:%S"),
                hours_up
            );
        } else {
            println!("{:7.0}° | Circumpolar or never visible", lat);
        }
    }

    // Example 5: Custom altitude (e.g., civil twilight at -6°)
    println!("\nCivil Twilight Times (Sun at -6° altitude):");
    let civil_twilight_alt = -6.0;
    
    // Approximate sun position for today
    let jd = astro_math::julian_date(noon_today);
    let n = jd - 2451545.0;
    let l = (280.460 + 0.9856474 * n) % 360.0;
    let g = ((357.528 + 0.9856003 * n) % 360.0).to_radians();
    let lambda = l + 1.915 * g.sin() + 0.020 * (2.0 * g).sin();
    let lambda_rad = lambda.to_radians();
    let epsilon = 23.439_f64.to_radians();
    let sun_ra = lambda_rad.cos().atan2(epsilon.cos() * lambda_rad.sin()).to_degrees();
    let sun_dec = (epsilon.sin() * lambda_rad.sin()).asin().to_degrees();
    
    if let Some((dawn, _, dusk)) = rise_transit_set(sun_ra, sun_dec, noon_today, &location, Some(civil_twilight_alt)) {
        println!("  Civil dawn: {} UTC", dawn.format("%H:%M:%S"));
        println!("  Civil dusk: {} UTC", dusk.format("%H:%M:%S"));
    }
}