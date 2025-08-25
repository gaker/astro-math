Getting Started
===============

Time conversions
----------------

.. code-block:: python

   from astro_math.time import julian, j2000
   from datetime import datetime
   
   dt = datetime(2024, 8, 4, 6, 0, 0)
   
   jd = julian(dt)
   print(f"JD: {jd:.5f}")
   
   days = j2000(dt)
   print(f"Days since J2000: {days:.2f}")

Coordinate transforms
---------------------

.. code-block:: python

   from astro_math.transforms import ra_dec_to_alt_az
   from astro_math.location import Location
   from datetime import datetime
   
   # Apache Point Observatory
   location = Location.parse("32.78", "-105.82", 2788.0)
   
   # M31 core
   ra, dec = 10.68458, 41.26917
   
   alt, az = ra_dec_to_alt_az(
       ra=ra, dec=dec,
       dt=datetime.utcnow(),
       latitude=location.latitude_deg,
       longitude=location.longitude_deg
   )
   
   print(f"M31: {alt:.1f}°, {az:.1f}°")

Parsing coordinates
-------------------

.. code-block:: python

   from astro_math.location import Location
   
   # Multiple formats supported
   coords = [
       ("40.7128", "-74.0060"),
       ("40°42'46\"N", "74°0'21.6\"W"),
       ("40:42:46", "-74:00:21.6"),
       ("40d42m46s", "-74d00m21.6s"),
   ]
   
   for lat_str, lon_str in coords:
       loc = Location.parse(lat_str, lon_str, 0.0)
       print(f"{lat_str} → {loc.latitude_deg:.4f}°")

Refraction
----------

.. code-block:: python

   from astro_math.refraction import bennett
   from astro_math.airmass import young
   
   alt = 30.0  # true altitude
   temp = 15.0  # celsius
   pressure = 1013.25  # hPa
   
   apparent = bennett(alt, temp, pressure)
   print(f"Refraction: {apparent - alt:.3f}°")
   
   airmass = young(apparent)
   print(f"Airmass: {airmass:.2f}")

Arrays
------

.. code-block:: python

   import numpy as np
   from astro_math.transforms import batch_ra_dec_to_alt_az
   from datetime import datetime
   
   # Some Messier objects
   ra_array = np.array([83.82, 210.80, 201.37])  # M42, M104, M51
   dec_array = np.array([-5.39, -11.62, 47.20])
   
   alt_array, az_array = batch_ra_dec_to_alt_az(
       ra_array, dec_array,
       dt=datetime.utcnow(),
       latitude=40.7, longitude=-74.0
   )
   
   for i, (alt, az) in enumerate(zip(alt_array, az_array)):
       print(f"Object {i+1}: {alt:.1f}°, {az:.1f}°")

See also
--------

Check out :doc:`examples` and :doc:`api_guide` for more examples.