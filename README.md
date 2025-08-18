# Astro Math

[![CI](https://github.com/gaker/astro-math/actions/workflows/test.yml/badge.svg)](https://github.com/gaker/astro-math/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/gaker/astro-math/graph/badge.svg?token=ML01cRa3zB)](https://codecov.io/gh/gaker/astro-math)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/astro-math.svg)](https://crates.io/crates/astro-math)
[![PyPI](https://img.shields.io/pypi/v/astro-math.svg)](https://pypi.org/project/astro-math/)

A comprehensive astronomy library implementing algorithms from Jean Meeus and other standard references for telescope control, observation planning, and celestial mechanics.

**Perfect for:** Telescope control software, planetarium applications, observation planning tools, and any astronomy-related project requiring accurate celestial calculations.

**Features:**
- Professional-grade accuracy (sub-arcsecond precision)
- Comprehensive coordinate parsing (27+ formats!)
- High-performance parallel processing with Rayon
- ERFA integration for maximum accuracy
- Extensive documentation with examples

## Project Structure

This is a Rust workspace containing:

- **`astro-math/`** - The core Rust library with all astronomical calculations
- **`astro-math-py/`** - Python bindings using PyO3 and maturin

## Why Choose Astro Math?

- **Accuracy First**: Results match professional software (Astropy, Stellarium) to sub-arcsecond precision  
- **Developer Friendly**: Comprehensive documentation, examples, and error handling  
- **Battle Tested**: Extensive test suite with real-world validation  
- **Performance**: Parallel processing and optimized algorithms  
- **Standards Compliant**: Implements IAU/SOFA standards via ERFA

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
astro-math = "0.1"
chrono = "0.4"  # For date/time handling
```

```rust
use astro_math::{Location, ra_dec_to_alt_az};
use chrono::Utc;

// Create observer location (supports 27+ coordinate formats!)
let location = Location::parse("40.7128 N", "74.0060 W", 10.0).unwrap();

// Convert star coordinates to local alt/az
let (alt, az) = ra_dec_to_alt_az(279.23, 38.78, Utc::now(), &location).unwrap();
println!("Vega is at altitude {:.1}°, azimuth {:.1}°", alt, az);
```

## Getting Started

### Rust Library

```bash
cd astro-math
cargo build
cargo test
cargo run --example moon
```

See [astro-math/README.md](astro-math/README.md) for detailed documentation.

### Python Bindings

```bash
cd astro-math-py
maturin develop  # For development
maturin build    # For distribution
```

## Development

To work on both the Rust library and Python bindings:

```bash
# Run all tests
cargo test --workspace

# Build everything
cargo build --workspace

# Format code
cargo fmt --all
```

## License

Licensed under either of:
- MIT license
- Apache License, Version 2.0

at your option.