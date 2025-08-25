astro-math
==========

Rust-based astronomy calculations for Python. Does the math you need for astrometry and observations.

.. toctree::
   :maxdepth: 2

   installation
   quickstart  
   examples
   api_guide
   api/astro_math/index
   troubleshooting

What it does
------------

Coordinate transforms, time conversions, astrometric corrections:

* Julian dates and epoch conversions
* RA/Dec to Alt/Az (and back)
* Parse coordinates from strings 
* Precession, nutation, aberration corrections
* Atmospheric refraction
* Sun/Moon positions
* Galactic coordinate transforms

Uses ERFA underneath.

Example
-------

.. code-block:: python

   from astro_math.time import julian
   from astro_math.transforms import ra_dec_to_alt_az
   from astro_math.location import Location
   from datetime import datetime

   # Palomar Observatory
   location = Location.parse("33°21'22\"N", "116°51'47\"W", 1712.0)
   
   # Right now
   dt = datetime.utcnow()
   
   # Where's Betelgeuse? 
   alt, az = ra_dec_to_alt_az(
       ra=88.793,   # J2000
       dec=7.407,
       dt=dt,
       latitude=location.latitude_deg,
       longitude=location.longitude_deg
   )
   
   print(f"Alt: {alt:.1f}°, Az: {az:.1f}°")

Performance
-----------

Rust backend makes batch operations fast. Single calculations are fast too. Handles arrays via NumPy.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`