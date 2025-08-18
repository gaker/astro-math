# astro-math Python Bindings

High-performance astronomy calculations for Python, powered by Rust.

## Installation

```bash
pip install astro-math
```

## Features

- **Fast batch operations** using NumPy arrays
- **27+ coordinate format parsing** - handles any format users throw at it
- **Zero-copy array operations** for maximum performance
- **Type hints** for better IDE support

## Quick Start

```python
import astro_math
import numpy as np
from datetime import datetime

# Single coordinate transformation
alt, az = astro_math.ra_dec_to_alt_az(
    ra=279.23,      # Vega
    dec=38.78,
    dt=datetime.utcnow(),
    latitude=40.7,
    longitude=-74.0
)

# Batch operations with NumPy arrays (100-1000x faster than loops!)
ra_array = np.array([279.23, 88.79, 213.91])  # Vega, Betelgeuse, Arcturus
dec_array = np.array([38.78, 7.41, 19.18])

alt_array, az_array = astro_math.ra_dec_to_alt_az_batch(
    ra_array, dec_array,
    dt=datetime.utcnow(),
    latitude=40.7,
    longitude=-74.0
)

# Parse any coordinate format
lat, lon, alt = astro_math.parse_location(
    "40°42'46\"N",     # or "40.7128", "40:42:46", "404246N", etc.
    "74°0'21.6\"W"     # or "-74.006", "74:00:21.6W", etc.
)

# Use Location objects
location = astro_math.Location.parse(
    "40°42'46\"N",
    "74°0'21.6\"W",
    altitude=10.0
)
print(location.latitude_dms())   # "40°42'46.0\"N"
print(location.longitude_dms())  # "74°00'21.6\"W"
```

## Performance

Batch operations are typically 100-1000x faster than Python loops:

```python
# Slow: Python loop
alts = []
azs = []
for ra, dec in zip(ra_list, dec_list):
    alt, az = astro_math.ra_dec_to_alt_az(ra, dec, dt, lat, lon)
    alts.append(alt)
    azs.append(az)

# Fast: Batch operation
alt_array, az_array = astro_math.ra_dec_to_alt_az_batch(
    np.array(ra_list), np.array(dec_list), dt, lat, lon
)
```

## API Reference

See the [main astro-math documentation](https://github.com/gaker/astro-math) for detailed API information.

## License

Licensed under either of:
- MIT license
- Apache License, Version 2.0

at your option.