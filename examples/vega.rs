use astro_math::{Location, julian_date, ra_dec_to_alt_az};
use chrono::{TimeZone, Utc};

fn main() {
    let dt = Utc.with_ymd_and_hms(2024, 8, 4, 6, 0, 0).unwrap();
    let loc = Location {
        latitude_deg: 31.9583,
        longitude_deg: -111.6,
        altitude_m: 2120.0,
    };

    let jd = julian_date(dt);
    let lst = loc.local_sidereal_time(dt);
    let (alt, az) = ra_dec_to_alt_az(279.23473479, 38.78368896, dt, &loc).unwrap();

    println!("JD: {:.5}", jd);
    println!("LST: {:.5} h", lst);
    println!("Vega Alt: {:.3}°, Az: {:.3}°", alt, az);
}
