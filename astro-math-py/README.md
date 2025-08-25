# astro-math

Astronomy calculations in Python with Rust backend.

## Install

```bash
pip install astro-math
```

## What it does

- Time conversions (Julian dates, epochs)
- Coordinate transforms (RA/Dec ↔ Alt/Az)
- Location parsing (many coordinate formats)
- Astrometric corrections (precession, nutation, aberration, proper motion)
- Atmospheric refraction and airmass
- Sun/Moon positions
- Galactic coordinates

Built on ERFA algorithms. Works with NumPy arrays.

## Quick example

```python
from astro_math.transforms import ra_dec_to_alt_az
from astro_math.location import Location
from datetime import datetime
import numpy as np

# Single coordinate
location = Location.parse("33.3563", "-116.8650", 1712.0)  # Palomar
alt, az = ra_dec_to_alt_az(
    ra=88.793, dec=7.407,  # Betelgeuse
    dt=datetime.utcnow(),
    latitude=location.latitude_deg,
    longitude=location.longitude_deg
)

# Arrays
ra_array = np.array([88.793, 279.234, 213.915])  # Betelgeuse, Vega, Arcturus
dec_array = np.array([7.407, 38.784, 19.182])

alt_array, az_array = ra_dec_to_alt_az_batch(
    ra_array, dec_array,
    dt=datetime.utcnow(),
    latitude=location.latitude_deg, 
    longitude=location.longitude_deg
)
```

## More examples

### Time conversions

```python
from astro_math.time import julian, j2000
from datetime import datetime

dt = datetime(2024, 12, 21, 6, 30, 0)
jd = julian(dt)
days_since_j2000 = j2000(dt)
```

### Proper motion

```python
from astro_math.proper_motion import apply_proper_motion

# Barnard's Star
ra_j2000, dec_j2000 = 269.45, 4.66
pm_ra, pm_dec = -798.6, 10328.1  # mas/year

ra_2024, dec_2024 = apply_proper_motion(
    ra_j2000, dec_j2000, pm_ra, pm_dec,
    epoch_from=2000.0, epoch_to=2024.5
)
```

### Atmospheric corrections

```python
from astro_math.refraction import bennett, saemundsson
from astro_math.airmass import young

true_alt = 30.0
apparent_alt = bennett(true_alt)
refraction = apparent_alt - true_alt

# With atmospheric conditions
precise_alt = saemundsson(true_alt, pressure_hpa=617.0, temperature_c=-5.0)
airmass = young(apparent_alt)
```

### Galactic coordinates

```python
from astro_math.galactic import equatorial_to_galactic

gal_l, gal_b = equatorial_to_galactic(266.405, -28.936)  # Galactic center
```

### Catalog processing

```python
import numpy as np

# Process whole catalog at once
ra_catalog = np.array([88.793, 279.234, 213.915, 310.358])
dec_catalog = np.array([7.407, 38.784, 19.182, 45.280])

alt_catalog, az_catalog = ra_dec_to_alt_az_batch(
    ra_catalog, dec_catalog,
    dt=datetime.utcnow(),
    latitude=40.7, longitude=-74.0
)

# Find visible objects
visible = alt_catalog > 20.0
names = ["Betelgeuse", "Vega", "Arcturus", "Deneb"]
for i, name in enumerate(names):
    if visible[i]:
        print(f"{name}: {alt_catalog[i]:.1f}°, {az_catalog[i]:.1f}°")
```

## Documentation

Full API docs: https://astro-math.readthedocs.io/

## License

MIT or Apache-2.0

at your option.