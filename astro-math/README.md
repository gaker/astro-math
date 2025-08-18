# Astro Math for Rust

[![codecov](https://codecov.io/gh/gaker/astro-math/graph/badge.svg?token=ML01cRa3zB)](https://codecov.io/gh/gaker/astro-math)

![workflow](https://github.com/gaker/astro-math/actions/workflows/test.yml/badge.svg)

A comprehensive astronomy library for Rust, implementing algorithms from Jean Meeus and other standard references for telescope control, observation planning, and celestial mechanics.

## Features

### Core Functionality
- **Time** – Julian Date conversions, J2000 epoch calculations
- **Sidereal Time** – GMST, LMST, and Apparent Sidereal Time
- **Location** – Observer coordinates with **ultimate coordinate parsing** and LST calculation
- **Coordinate Transforms** – RA/Dec ↔ Alt/Az conversions
- **Projection** – Gnomonic/TAN projection for astrometry

### Advanced Features
- **Precession** – Convert coordinates between different epochs (J2000 ↔ current date)
- **Nutation** – Earth's axis wobble corrections (±18.6" longitude, ±9.2" obliquity)
- **Parallax** – Diurnal and annual parallax corrections
- **Proper Motion** – Linear and rigorous space motion calculations
- **Stellar Aberration** – Annual aberration corrections (±20.5 arcseconds)
- **Atmospheric Refraction** – Multiple refraction models (Bennett, Saemundsson, radio)
- **Moon Calculations** – Position, phase, illumination, distance
- **Rise/Set/Transit** – Calculate rise, set, and meridian transit times
- **Galactic Coordinates** – Convert between equatorial and galactic coordinate systems
- **Airmass** – Multiple airmass formulas for extinction calculations

### Ultimate Coordinate Parsing
The library features the most comprehensive coordinate parsing system available, handling virtually any format users might input:

- **Decimal Degrees**: `40.7128`, `40.7128N`, `N40.7128`, `40.7128 North`
- **DMS Formats**: `40°42'46"`, `40 42 46`, `40:42:46`, `40-42-46`, `40d42m46s`
- **Unicode Symbols**: `40°42′46″` (proper Unicode prime/double-prime)
- **HMS Longitude**: `4h 56m 27s W`, `4:56:27`, `4 hours 56 minutes 27 seconds`
- **Compact Aviation**: `404246N`, `4042.767N` (DDMMSS, DDMM.mmm formats)
- **Mixed Formats**: `40d 42' 46"`, extra spaces, case-insensitive
- **Fuzzy Matching**: Handles typos, mixed separators, various symbols


## Installation

Add to your `Cargo.toml`:


```toml
astro-math = "0.1"
```

## Quick Start

### Basic Example

```rust
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
    let (alt, az) = ra_dec_to_alt_az(279.23473479, 38.78368896, dt, &loc);

    println!("JD: {:.5}", jd);
    println!("LST: {:.5} h", lst);
    println!("Vega Alt: {:.3}°, Az: {:.3}°", alt, az);
}
```

### Moon Phase and Position

```rust
use astro_math::{moon_phase_name, moon_illumination, moon_equatorial};
use chrono::Utc;

let now = Utc::now();
let phase = moon_phase_name(now);
let illumination = moon_illumination(now);
let (ra, dec) = moon_equatorial(now);

println!("Moon: {} ({:.1}% illuminated)", phase, illumination);
println!("Position: RA={:.2}°, Dec={:.2}°", ra, dec);
```

### Precession Between Epochs

```rust
use astro_math::precess_j2000_to_date;
use chrono::{TimeZone, Utc};

// Precess coordinates from J2000 to current date
let dt = Utc::now();
let (ra_j2000, dec_j2000) = (83.633, 22.0145); // Orion Nebula
let (ra_now, dec_now) = precess_j2000_to_date(ra_j2000, dec_j2000, dt);

println!("Orion Nebula J2000: RA={:.3}°, Dec={:.3}°", ra_j2000, dec_j2000);
println!("Orion Nebula now:   RA={:.3}°, Dec={:.3}°", ra_now, dec_now);
```

### Nutation Calculations

```rust
use astro_math::nutation::{nutation, mean_obliquity, true_obliquity};
use astro_math::time::julian_date;
use chrono::Utc;

let dt = Utc::now();
let jd = julian_date(dt);

// Get both nutation components
let nut = nutation(jd);
println!("Nutation in longitude: {:.2}\"", nut.longitude);
println!("Nutation in obliquity: {:.2}\"", nut.obliquity);

// Calculate true obliquity for coordinate transformations
let true_obl = true_obliquity(jd);
println!("True obliquity: {:.6}°", true_obl);
```

### Ultimate Coordinate Parsing

```rust
use astro_math::location::Location;

// The parser handles virtually any coordinate format!

// Decimal degrees with compass directions
let loc = Location::parse("40.7128 N", "74.0060 W", 10.0).unwrap();
assert_eq!(loc.latitude_deg, 40.7128);
assert_eq!(loc.longitude_deg, -74.0060);

// Traditional DMS with symbols  
let loc = Location::parse("40°42'46\"N", "74°0'21.6\"W", 10.0).unwrap();

// Unicode symbols
let loc = Location::parse("40°42′46″", "-74°00′21.6″", 10.0).unwrap();

// Various separators
let loc = Location::parse("40:42:46", "-74-00-21.6", 10.0).unwrap();

// HMS for longitude
let loc = Location::parse("51.5074 N", "0h 7m 39.84s W", 0.0).unwrap();

// Compact aviation formats
let loc = Location::parse("404246N", "0740022W", 10.0).unwrap();  // DDMMSS
let loc = Location::parse("4042.767N", "07400.360W", 10.0).unwrap();  // DDMM.mmm

// Mixed formats and fuzzy matching
let loc = Location::parse("40d 42' 46\" North", "74 deg 0 min 21.6 sec west", 10.0).unwrap();

// Handles edge cases like negative zero
let loc = Location::parse("-00 30 00", "000 00 00", 0.0).unwrap();
assert_eq!(loc.latitude_deg, -0.5);
```

### Proper Motion Calculations

```rust
use astro_math::apply_proper_motion;
use chrono::{TimeZone, Utc};

// Barnard's Star - highest proper motion
let ra_j2000 = 269.454;  // degrees
let dec_j2000 = 4.668;   // degrees
let pm_ra = -797.84;     // mas/yr (already includes cos(dec))
let pm_dec = 10326.93;   // mas/yr

// Calculate position at 2024.0
let epoch_2024 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
let (ra_2024, dec_2024) = apply_proper_motion(
    ra_j2000, dec_j2000, pm_ra, pm_dec, epoch_2024
).unwrap();

println!("Barnard's Star J2000: RA={:.3}°, Dec={:.3}°", ra_j2000, dec_j2000);
println!("Barnard's Star 2024:  RA={:.3}°, Dec={:.3}°", ra_2024, dec_2024);
```

### Stellar Aberration

```rust
use astro_math::{apply_aberration, aberration_magnitude};
use chrono::{TimeZone, Utc};

// Vega coordinates
let ra = 279.234735;
let dec = 38.783689;

// Calculate apparent position for observation date
let obs_date = Utc::now();
let (ra_apparent, dec_apparent) = apply_aberration(ra, dec, obs_date).unwrap();

// Get the magnitude of displacement
let displacement = aberration_magnitude(ra, dec, obs_date).unwrap();

println!("Mean position:     RA={:.6}°, Dec={:.6}°", ra, dec);
println!("Apparent position: RA={:.6}°, Dec={:.6}°", ra_apparent, dec_apparent);
println!("Aberration: {:.1} arcseconds", displacement);
```

### Rise, Set, and Transit Times

```rust
use astro_math::{sun_rise_set, rise_transit_set, Location};
use chrono::{TimeZone, Utc};

let location = Location {
    latitude_deg: 40.7128,
    longitude_deg: -74.0060,
    altitude_m: 10.0,
};

let today = Utc::now().date_naive().and_hms_opt(12, 0, 0).unwrap();
let today = Utc.from_utc_datetime(&today);

// Sun rise and set
if let Some((sunrise, sunset)) = sun_rise_set(today, &location) {
    println!("Sunrise: {}", sunrise.format("%H:%M UTC"));
    println!("Sunset:  {}", sunset.format("%H:%M UTC"));
}

// Star rise, transit, set
let (ra, dec) = (88.793, 7.407); // Betelgeuse
if let Some((rise, transit, set)) = rise_transit_set(ra, dec, today, &location, None) {
    println!("Betelgeuse rises:    {}", rise.format("%H:%M UTC"));
    println!("Betelgeuse transits: {}", transit.format("%H:%M UTC"));
    println!("Betelgeuse sets:     {}", set.format("%H:%M UTC"));
}
```


### More Examples

Check out the `examples/` directory for comprehensive examples:

- `coordinate_parsing.rs` - Ultimate coordinate parsing demo (27+ formats!)
- `precession.rs` - Coordinate precession between epochs
- `parallax.rs` - Diurnal and annual parallax corrections
- `proper_motion.rs` - Stellar proper motion and space velocity
- `refraction.rs` - Atmospheric refraction models
- `moon.rs` - Moon position, phase, and rise/set calculations  
- `rise_set.rs` - Rise, set, and transit time calculations
- `galactic.rs` - Galactic coordinate conversions
- `airmass.rs` - Airmass and extinction calculations

Run examples with:

```bash
cargo run --example coordinate_parsing
cargo run --example moon
cargo run --example rise_set
```

## API Documentation

### Time Functions

- `julian_date(datetime)` - Convert DateTime to Julian Date
- `j2000_days(datetime)` - Days since J2000.0 epoch

### Location and Coordinate Parsing

- `Location::parse(lat_str, lon_str, alt_m)` - Parse any coordinate format
- `Location::from_dms(lat_str, lon_str, alt_m)` - Traditional DMS parsing
- `location.local_sidereal_time(datetime)` - Calculate local sidereal time
- `location.latitude_dms()` - Format latitude as DMS string
- `location.longitude_dms()` - Format longitude as DMS string

### Coordinate Transformations

- `ra_dec_to_alt_az(ra, dec, datetime, location)` - Equatorial to horizontal
- `equatorial_to_galactic(ra, dec)` - Equatorial to galactic coordinates
- `galactic_to_equatorial(l, b)` - Galactic to equatorial coordinates

### Precession

- `precess_j2000_to_date(ra, dec, datetime)` - J2000 to current epoch
- `precess_date_to_j2000(ra, dec, datetime)` - Current epoch to J2000

### Nutation

- `nutation(jd)` - Returns both components in a struct
- `nutation_in_longitude(jd)` - Nutation in longitude (Δψ) in arcseconds
- `nutation_in_obliquity(jd)` - Nutation in obliquity (Δε) in arcseconds
- `mean_obliquity(jd)` - Mean obliquity of ecliptic in degrees
- `true_obliquity(jd)` - True obliquity including nutation

### Proper Motion

- `apply_proper_motion(ra, dec, pm_ra_cosdec, pm_dec, datetime)` - Linear proper motion
- `apply_proper_motion_rigorous(ra, dec, pm_ra_cosdec, pm_dec, parallax, rv, datetime)` - Rigorous 3D space motion
- `total_proper_motion(pm_ra_cosdec, pm_dec)` - Calculate total proper motion
- `proper_motion_position_angle(pm_ra_cosdec, pm_dec)` - Direction of motion

### Stellar Aberration

- `apply_aberration(ra, dec, datetime)` - Apply annual aberration correction
- `remove_aberration(ra_apparent, dec_apparent, datetime)` - Convert apparent to mean position
- `aberration_magnitude(ra, dec, datetime)` - Calculate aberration displacement in arcseconds

### Moon Calculations

- `moon_position(datetime)` - Returns (longitude, latitude) in ecliptic
- `moon_equatorial(datetime)` - Returns (RA, Dec)
- `moon_phase_angle(datetime)` - Phase angle in degrees
- `moon_phase_name(datetime)` - Phase name as string
- `moon_illumination(datetime)` - Illumination percentage
- `moon_distance(datetime)` - Distance in kilometers

### Refraction Models

- `refraction_bennett(altitude)` - Bennett's formula
- `refraction_saemundsson(altitude, pressure, temperature)` - With weather
- `refraction_radio(altitude, pressure, temperature, humidity)` - Radio wavelengths

### Airmass Calculations

- `airmass_plane_parallel(altitude)` - Simple secant formula
- `airmass_young(altitude)` - Young's formula (1994)
- `airmass_pickering(altitude)` - Pickering's formula (2002)
- `extinction_magnitudes(airmass, coefficient)` - Calculate extinction

## Testing

The library includes comprehensive unit tests. Run with:

```bash
cargo test
```

## References

- Meeus, J. (1998). *Astronomical Algorithms* (2nd ed.)
- IAU SOFA Library (Standards of Fundamental Astronomy)
- Reid, M. J. & Brunthaler, A. (2004). *The Proper Motion of Sagittarius A**
- Young, A. T. (1994). *Air mass and refraction*
- Pickering, K. A. (2002). *The Southern Limits of the Ancient Star Catalog*

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.