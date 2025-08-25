Examples
========

Practical examples for real observing scenarios.

Planning observations
---------------------

.. code-block:: python

   from astro_math.transforms import ra_dec_to_alt_az
   from astro_math.location import Location
   from astro_math.airmass import young
   from datetime import datetime, timedelta
   
   # Palomar 200-inch
   obs = Location.parse("33.3563", "-116.8650", 1712.0)
   
   # NGC 2419 - distant globular 
   ra, dec = 114.5, 38.88
   
   start = datetime.utcnow().replace(hour=2, minute=0)
   
   print("Time  Alt   Az   Airmass")
   print("----  ---  ---  -------")
   
   for hour in range(8):
       obs_time = start + timedelta(hours=hour)
       
       alt, az = ra_dec_to_alt_az(
           ra=ra, dec=dec, dt=obs_time,
           latitude=obs.latitude_deg, 
           longitude=obs.longitude_deg
       )
       
       if alt > 20:  # decent altitude
           airmass = young(alt)
           print(f"{obs_time.strftime('%H:%M')} {alt:5.1f} {az:5.1f}  {airmass:.2f}")

High proper motion stars
------------------------

.. code-block:: python

   from astro_math.precession import j2000_to_date
   from astro_math.proper_motion import apply_proper_motion
   from datetime import datetime
   import math
   
   # Barnard's Star
   ra_j2000, dec_j2000 = 269.45, 4.66
   pm_ra, pm_dec = -798.6, 10328.1  # mas/year
   
   current_epoch = 2024.5
   ra_pm, dec_pm = apply_proper_motion(
       ra_j2000, dec_j2000, pm_ra, pm_dec,
       epoch_from=2000.0, epoch_to=current_epoch
   )
   
   ra_now, dec_now = j2000_to_date(ra_pm, dec_pm, datetime.utcnow())
   
   print(f"J2000.0: {ra_j2000:.3f}, {dec_j2000:.3f}")
   print(f"Now:     {ra_now:.3f}, {dec_now:.3f}")
   
   # Total motion
   dra = (ra_now - ra_j2000) * 3600 * math.cos(math.radians(dec_j2000))
   ddec = (dec_now - dec_j2000) * 3600
   print(f"Motion: {dra:.1f}\", {ddec:.1f}\"")

Refraction
----------

.. code-block:: python

   from astro_math.refraction import bennett, saemundsson
   from astro_math.airmass import young, pickering
   
   altitude = 15.0  # low altitude target
   
   refr_basic = bennett(altitude)
   print(f"Bennett: {refr_basic - altitude:.3f}°")
   
   # Mauna Kea conditions
   refr_precise = saemundsson(altitude, 617.0, -5.0)  # hPa, °C
   print(f"Saemundsson: {refr_precise - altitude:.3f}°")
   
   am1 = young(altitude)
   am2 = pickering(altitude)
   print(f"Airmass: {am1:.2f} (Young), {am2:.2f} (Pickering)")

Catalogs
--------

.. code-block:: python

   import numpy as np
   from astro_math.transforms import batch_ra_dec_to_alt_az
   from datetime import datetime
   
   # Some bright variables: RR Lyr, delta Cep, W Vir, RT Aur
   ra_array = np.array([283.78, 337.29, 213.90, 95.78])
   dec_array = np.array([42.78, 58.21, -12.04, 30.33])
   
   alt_array, az_array = batch_ra_dec_to_alt_az(
       ra_array, dec_array,
       dt=datetime.utcnow(),
       latitude=40.7, longitude=-74.0
   )
   
   for i, (ra, dec, alt, az) in enumerate(zip(ra_array, dec_array, alt_array, az_array)):
       if alt > 0:
           print(f"Star {i+1}: RA={ra:.2f} Dec={dec:.2f} → Alt={alt:.1f}° Az={az:.1f}°")

Time conversions
----------------

.. code-block:: python

   from astro_math.time import julian, j2000
   from astro_math.timescales import utc_to_tt_jd
   from datetime import datetime
   
   # Observation time
   dt = datetime(2024, 12, 21, 3, 45, 30)
   
   jd_utc = julian(dt)
   jd_tt = utc_to_tt_jd(jd_utc)
   days_j2000 = j2000(dt)
   
   print(f"JD (UTC): {jd_utc:.5f}")
   print(f"JD (TT):  {jd_tt:.5f}")
   print(f"Days since J2000: {days_j2000:.2f}")