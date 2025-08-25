Troubleshooting
===============

Common problems and solutions.

Building 
--------

No Rust toolchain
~~~~~~~~~~~~~~~~~

Error: ``error: Microsoft Visual Studio C++ Build tools`` or ``rustc not found``

Install Rust:

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustc --version

Missing maturin
~~~~~~~~~~~~~~~

Error: ``maturin not found`` or ``failed to build wheels``

.. code-block:: bash

   pip install maturin
   
   # Ubuntu/Debian
   sudo apt-get install python3-dev
   
   # CentOS/RHEL
   sudo yum install python3-devel
   
   maturin develop

Import problems
---------------

Module not found
~~~~~~~~~~~~~~~~

Error: ``ModuleNotFoundError: No module named 'astro_math'``

.. code-block:: bash

   python -c "import astro_math; print('OK')"
   
   # If that fails:
   cd astro-math-py
   maturin develop

Submodule problems
~~~~~~~~~~~~~~~~~~

Error: ``ImportError: cannot import name 'julian' from 'astro_math.time'``

Extension didn't build right:

.. code-block:: bash

   rm -rf target/
   maturin develop --release
   
   find . -name "*.pyc" -delete
   find . -name "__pycache__" -delete

Runtime errors
--------------

Bad coordinates
~~~~~~~~~~~~~~~

Error: ``ValueError: RA must be in range [0, 360) degrees``

Valid ranges:
- RA: 0° ≤ RA < 360°
- Dec: -90° ≤ Dec ≤ 90° 
- Lat: -90° ≤ lat ≤ 90°
- Lon: -180° ≤ lon < 180°

.. code-block:: python

   ra = ra % 360.0
   dec = max(-90.0, min(90.0, dec))

Below horizon
~~~~~~~~~~~~~

Error: ``ProjectionError: Point is on opposite side of sky``

Object is below horizon. Check altitude:

.. code-block:: python

   from astro_math.transforms import ra_dec_to_alt_az
   
   alt, az = ra_dec_to_alt_az(ra, dec, dt, lat, lon)
   if alt > 0:
       # Above horizon
       pass
   else:
       print(f"Object is {abs(alt):.1f}° below horizon")

NaN results
~~~~~~~~~~~

Results contain ``nan`` or ``inf``.

Check inputs:

.. code-block:: python

   import numpy as np
   
   assert np.isfinite(ra) and np.isfinite(dec)
   assert -90 <= dec <= 90
   assert 0 <= ra < 360
   
   # Use pickering() for low altitudes
   from astro_math.airmass import pickering
   airmass = pickering(altitude)

Performance
-----------

Slow transforms
~~~~~~~~~~~~~~~

Don't loop. Use batch operations:

.. code-block:: python

   # Slow
   results = []
   for ra, dec in zip(ra_array, dec_array):
       alt, az = ra_dec_to_alt_az(ra, dec, dt, lat, lon)
       results.append((alt, az))
   
   # Fast
   import numpy as np
   alt_array, az_array = batch_ra_dec_to_alt_az(
       np.array(ra_array), np.array(dec_array),
       dt, lat, lon
   )

Memory issues
~~~~~~~~~~~~~

Process in chunks:

.. code-block:: python

   import numpy as np
   
   def process_catalog_chunks(ra_array, dec_array, chunk_size=10000):
       n_objects = len(ra_array)
       results_alt = np.empty(n_objects)
       results_az = np.empty(n_objects)
       
       for i in range(0, n_objects, chunk_size):
           end_idx = min(i + chunk_size, n_objects)
           chunk_alt, chunk_az = batch_ra_dec_to_alt_az(
               ra_array[i:end_idx], 
               dec_array[i:end_idx],
               dt, lat, lon
           )
           results_alt[i:end_idx] = chunk_alt
           results_az[i:end_idx] = chunk_az
       
       return results_alt, results_az

Documentation Build Issues
--------------------------

Sphinx build fails
~~~~~~~~~~~~~~~~~~~

**Error**: Various Sphinx-related errors during ``make html``

**Solutions**:

.. code-block:: bash

   # Install documentation dependencies
   pip install -e ".[docs]"
   
   # Build the Rust extension first
   maturin develop
   
   # Clean and rebuild docs
   cd docs
   make clean
   make html

AutoAPI warnings
~~~~~~~~~~~~~~~~

**Warning**: ``Cannot resolve import of unknown module astro_math.astro_math``

**Cause**: AutoAPI tries to import the extension before it's built.

**Solution**: This is expected and can be ignored. The warning is suppressed in the configuration.

Getting Help
------------

If you encounter issues not covered here:

1. **Check the GitHub issues**: https://github.com/gaker/astro-math/issues
2. **Search existing discussions** for similar problems
3. **Create a new issue** with:

   - Your operating system and Python version
   - Complete error message and traceback
   - Minimal code example that reproduces the problem
   - Steps you've already tried

**Example issue template**:

.. code-block:: text

   **Environment**
   - OS: macOS 14.0
   - Python: 3.11.5
   - astro-math version: 0.1.0
   
   **Problem**
   Getting ValueError when trying to transform coordinates
   
   **Code to reproduce**
   ```python
   from astro_math.transforms import ra_dec_to_alt_az
   # ... minimal example
   ```
   
   **Error message**
   ```
   ValueError: ...
   ```
   
   **What I tried**
   - Verified coordinates are in range
   - Reinstalled the package

**Performance expectations**:

- Single coordinate transform: ~1-10 μs
- Batch operations (1000 objects): ~100-500 μs  
- Large catalogs (100k objects): ~10-50 ms

If performance is significantly slower, there may be an installation or environment issue.