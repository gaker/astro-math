use astro_math::{Location, julian_date, ra_dec_to_alt_az};
use chrono::Utc;
use std::io::{self, Write};

fn prompt_f64(prompt: &str) -> f64 {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number")
}

fn main() {
    println!("Alt/Az Calculator");
    println!("Enter observer location:");

    let latitude = prompt_f64("  Latitude (deg, +N/-S)");
    let longitude = prompt_f64("  Longitude (deg, +E/-W)");

    let location = Location {
        latitude_deg: latitude,
        longitude_deg: longitude,
        altitude_m: 0.0, // not needed for alt/az
    };

    println!("Enter star coordinates:");

    let ra = prompt_f64("  RA (deg, 0–360)");
    let dec = prompt_f64("  DEC (deg, -90 to +90)");

    let now = Utc::now();
    let jd = julian_date(now);
    let lst = location.local_sidereal_time(now);
    let (alt, az) = ra_dec_to_alt_az(ra, dec, now, &location).unwrap();

    println!("\nUTC Time       : {now}");
    println!(" Julian Date   : {:.5}", jd);
    println!(" Sidereal Time : {:.5} hours", lst);
    println!(" Altitude      : {:.3}°", alt);
    println!(" Azimuth       : {:.3}°", az);
}
