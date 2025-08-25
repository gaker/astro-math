Installation
============

From PyPI
---------

.. code-block:: bash

   pip install astro-math

From git
--------

If you want the latest version:

.. code-block:: bash

   pip install git+https://github.com/gaker/astro-math.git

Building from source
--------------------

You'll need:
* Rust (1.70+)
* Python 3.8+
* NumPy

.. code-block:: bash

   git clone https://github.com/gaker/astro-math.git
   cd astro-math/astro-math-py
   pip install maturin
   maturin develop

Testing it
----------

.. code-block:: python

   from astro_math.time import julian  
   from datetime import datetime
   
   print(julian(datetime.utcnow()))

Should print something around 2460000. If it works, you're good.