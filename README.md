# Astro Math for Rust

[![codecov](https://codecov.io/gh/gaker/astro-math/graph/badge.svg?token=ML01cRa3zB)](https://codecov.io/gh/gaker/astro-math)

![workflow](https://github.com/gaker/astro-math/actions/workflows/test.yml/badge.svg)

Collection of astronomy-based algorithms based on the Jean Meeus book.


Currently considting of:
- `time` – Julian Date, J2000, epoch helpers
- `sidereal` – GMST, LMST, and Apparent Sidereal Time
- `location` – Observer lat/lon/alt + LST calculation
- `transforms` – RA/DEC ↔ Alt/Az conversion
- `timestamp` – FITS-style UTC timestamp abstraction


## Installation

Add to your `Cargo.toml`:


```toml
astro-math = "0.1"
```

Examples:

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

Produces:

```
JD: 2460526.75000
LST: 19.44655 h
Vega Alt: 77.776°, Az: 307.388°
```


## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.