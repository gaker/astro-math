"""
High-performance astronomy calculations for Python.

This package provides fast astronomical calculations organized into logical modules:

- astro_math.time: Julian dates and time conversions
- astro_math.transforms: Coordinate transformations (RA/Dec ↔ Alt/Az)  
- astro_math.location: Earth coordinates and parsing
- astro_math.precession: Epoch conversions
- astro_math.nutation: Earth's axis corrections
- astro_math.aberration: Annual stellar aberration
- astro_math.refraction: Atmospheric effects
- astro_math.airmass: Extinction calculations
- astro_math.galactic: Galactic coordinates
- astro_math.sun_moon: Solar system objects
- astro_math.proper_motion: Stellar motions
- astro_math.sidereal: Sidereal time

Examples:
    >>> from astro_math.time import julian
    >>> from astro_math.transforms import ra_dec_to_alt_az
    >>> from astro_math.location import Location
    >>> from datetime import datetime
    >>> 
    >>> # Julian date conversion
    >>> jd = julian(datetime.utcnow())
    >>> 
    >>> # Coordinate transformation
    >>> alt, az = ra_dec_to_alt_az(
    ...     ra=279.23, dec=38.78, 
    ...     dt=datetime.utcnow(),
    ...     latitude=40.7, longitude=-74.0
    ... )
    >>> 
    >>> # Parse coordinates
    >>> location = Location.parse("40°42'46\"N", "74°0'21.6\"W", 0.0)
"""

# Import version info
from astro_math.astro_math import __version__

__all__ = ["__version__"]