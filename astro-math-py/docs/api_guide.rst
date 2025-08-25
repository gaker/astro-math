API Reference
=============

Modules
-------

.. code-block:: python

   # Time
   from astro_math.time import julian, j2000
   from astro_math.timescales import utc_to_tt_jd, tai_utc_offset
   
   # Coordinates
   from astro_math.transforms import ra_dec_to_alt_az, batch_ra_dec_to_alt_az
   from astro_math.location import Location
   
   # Corrections
   from astro_math.precession import j2000_to_date, to_j2000
   from astro_math.nutation import nutation, in_longitude, in_obliquity
   from astro_math.aberration import apply, remove, magnitude
   from astro_math.proper_motion import apply_proper_motion, apply_proper_motion_rigorous
   
   # Atmosphere
   from astro_math.refraction import bennett, saemundsson, radio
   from astro_math.airmass import young, pickering, plane_parallel
   
   # Other coordinate systems
   from astro_math.galactic import equatorial_to_galactic, galactic_to_equatorial
   
   # Sun/Moon
   from astro_math.sun_moon import sun_position, moon_position, moon_phase_angle
   
   # Sidereal time
   from astro_math.sidereal import gmst, local_mean_sidereal_time

Time conversions
----------------

.. code-block:: python

   from astro_math.time import julian, j2000
   from astro_math.timescales import utc_to_tt_jd, tt_utc_offset_seconds
   from datetime import datetime
   
   dt = datetime(2024, 8, 4, 6, 0, 0)
   
   jd_utc = julian(dt)
   days_j2000 = j2000(dt)
   
   # UTC → TT
   jd_tt = utc_to_tt_jd(jd_utc)
   offset = tt_utc_offset_seconds()  # 69.184 seconds currently

Coordinate transforms
---------------------

.. code-block:: python

   from astro_math.transforms import ra_dec_to_alt_az, alt_az_to_ra_dec
   from astro_math.location import Location
   from datetime import datetime
   
   location = Location.parse("40°42'46\"N", "74°0'21.6\"W", 10.0)
   
   # 61 Cygni A coordinates
   ra, dec = 316.62, 38.73
   
   alt, az = ra_dec_to_alt_az(
       ra=ra, dec=dec,
       dt=datetime.utcnow(),
       latitude=location.latitude_deg,
       longitude=location.longitude_deg
   )
   
   # Inverse
   ra_back, dec_back = alt_az_to_ra_dec(
       alt=alt, az=az,
       dt=datetime.utcnow(),
       latitude=location.latitude_deg,
       longitude=location.longitude_deg
   )

Astrometric corrections
-----------------------

.. code-block:: python

   from astro_math.precession import j2000_to_date
   from astro_math.nutation import nutation
   from astro_math.aberration import apply
   from astro_math.proper_motion import apply_proper_motion
   from datetime import datetime
   
   # Wolf 359 (nearby red dwarf)
   ra_j2000, dec_j2000 = 164.1, 7.0
   pm_ra, pm_dec = -3842.0, -2725.0  # mas/year
   current_time = datetime.utcnow()
   
   # Proper motion
   ra_pm, dec_pm = apply_proper_motion(
       ra_j2000, dec_j2000, pm_ra, pm_dec,
       epoch_from=2000.0, epoch_to=2024.5
   )
   
   # Precession
   ra_prec, dec_prec = j2000_to_date(ra_pm, dec_pm, current_time)
   
   # Nutation
   jd = julian(current_time)
   nut_lon, nut_obl = nutation(jd)
   
   # Aberration
   ra_final, dec_final = apply(ra_prec, dec_prec, current_time)

Atmosphere
----------

.. code-block:: python

   from astro_math.refraction import bennett, saemundsson
   from astro_math.airmass import young, pickering
   
   true_alt = 30.0
   apparent_alt = bennett(true_alt)
   refraction = apparent_alt - true_alt
   
   # Conditions at Mauna Kea
   precise_alt = saemundsson(
       altitude_deg=true_alt,
       pressure_hpa=617.0,
       temperature_c=-5.0
   )
   
   airmass_std = young(apparent_alt)
   airmass_precise = pickering(apparent_alt)
   
   # V-band extinction
   extinction_coeff = 0.11  # mag/airmass at MK
   dimming = airmass_std * extinction_coeff

Arrays
------

.. code-block:: python

   import numpy as np
   from astro_math.transforms import batch_ra_dec_to_alt_az
   from astro_math.airmass import batch_airmass_pickering
   
   # Some Cepheids: delta Cep, eta Aql, zeta Gem, beta Dor
   ra_array = np.array([337.29, 297.70, 113.65, 84.41])
   dec_array = np.array([58.21, 1.01, 31.89, -65.74])
   
   alt_array, az_array = batch_ra_dec_to_alt_az(
       ra_array, dec_array,
       dt=datetime.utcnow(),
       latitude=40.7, longitude=-74.0
   )
   
   airmass_array = batch_airmass_pickering(alt_array)
   
   # Observable targets
   observable = airmass_array < 2.0
   good_ra = ra_array[observable]

Errors
------

.. code-block:: python

   from astro_math.transforms import ra_dec_to_alt_az
   from datetime import datetime
   
   try:
       alt, az = ra_dec_to_alt_az(
           ra=400.0,  # Invalid
           dec=38.78,
           dt=datetime.utcnow(),
           latitude=40.7, longitude=-74.0
       )
   except ValueError as e:
       print(f"Error: {e}")

Performance
-----------

Use batch operations for multiple objects. Create Location objects once. Cache time conversions when processing at the same epoch.

.. code-block:: python

   location = Location.parse("40.7128", "-74.0060", 0.0)
   jd = julian(datetime.utcnow())
   
   # Process whole catalog
   alt_array, az_array = batch_ra_dec_to_alt_az(
       ra_catalog, dec_catalog, 
       dt=datetime.utcnow(),
       latitude=location.latitude_deg,
       longitude=location.longitude_deg
   )