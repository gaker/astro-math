# Astro Math

[![CI](https://github.com/gaker/astro-math/actions/workflows/test.yml/badge.svg)](https://github.com/gaker/astro-math/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/gaker/astro-math/graph/badge.svg?token=ML01cRa3zB)](https://codecov.io/gh/gaker/astro-math)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/astro-math.svg)](https://crates.io/crates/astro-math)
[![PyPI](https://img.shields.io/pypi/v/astro-math.svg)](https://pypi.org/project/astro-math/)

Fast astronomical calculations in Rust with Python bindings. Built for telescope control, sky surveys, and astronomical data processing.

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-math = "0.2"
chrono = "0.4"
```

### Python

```bash
pip install astro-math
```

Or build from source with maturin:

```bash
cd astro-math-py
maturin develop
```

## Quick Example

```rust
use astro_math::{Location, ra_dec_to_alt_az};
use chrono::Utc;

// Parse observer location from various formats
let location = Location::parse("40.7128 N", "74.0060 W", 10.0)?;

// Convert celestial coordinates to horizon coordinates
let (alt, az) = ra_dec_to_alt_az(279.23, 38.78, Utc::now(), &location)?;
println!("Altitude: {:.2}°, Azimuth: {:.2}°", alt, az);
```

## Examples

The `examples/` directory contains working code for common tasks:

- **`basic_transforms.rs`** - Convert between coordinate systems (RA/Dec ↔ Alt/Az)
- **`location_parsing.rs`** - Parse coordinates from strings in 25+ formats
- **`proper_motion.rs`** - Apply stellar proper motion over time
- **`rise_set_times.rs`** - Calculate when objects rise and set
- **`moon_phases.rs`** - Moon position, phase, and illumination
- **`precession.rs`** - Precess coordinates between epochs
- **`galactic_coords.rs`** - Convert between equatorial and galactic coordinates
- **`telescope_pointing.rs`** - Real-time telescope coordinate conversion
- **`batch_processing.rs`** - Process large coordinate datasets efficiently

Run any example:

```bash
cargo run --example basic_transforms
```

## Features

- **Coordinate Transformations**: RA/Dec ↔ Alt/Az, Galactic, Ecliptic
- **Location Parsing**: Handles DMS, HMS, decimal degrees, and mixed formats
- **Time Systems**: Proper UTC/TT conversion with leap second tables
- **Stellar Motion**: Proper motion, precession, nutation, aberration
- **Solar System**: Sun/Moon positions, phases, rise/set times
- **Performance**: Parallel batch processing, optimized algorithms

## Validation

Tested against AstroPy to ensure accuracy:

- **Coordinate transformations**: Sub-arcsecond agreement
- **Time calculations**: Microsecond precision
- **Performance benchmarks**: See `benchmarks/` notebooks for detailed comparisons

The Jupyter notebooks in `benchmarks/` show comprehensive testing against AstroPy across thousands of test cases.

## Architecture

- **`astro-math/`** - Core Rust library
- **`astro-math-py/`** - Python bindings via PyO3
- **`examples/`** - Rust usage examples
- **`benchmarks/`** - Performance analysis notebooks

## Contributing

Built using the IAU SOFA algorithms and Jean Meeus formulations. See individual module documentation for implementation details and references.

## License

MIT OR Apache-2.0