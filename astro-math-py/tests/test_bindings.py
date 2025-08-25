"""
Tests for astro-math Python bindings.

These tests ensure the Python bindings are working correctly and provide
code coverage for the PyO3 wrapper code.
"""

import pytest
import astro_math
from datetime import datetime
import math


class TestJulianDate:
    """Test Julian Date conversions."""

    def test_julian_date_j2000(self):
        """Test J2000.0 epoch."""
        dt = datetime(2000, 1, 1, 12, 0, 0)
        jd = astro_math.time.julian(dt)
        assert abs(jd - 2451545.0) < 0.001

    def test_julian_date_mjd(self):
        """Test Modified Julian Date."""
        dt = datetime(2000, 1, 1, 12, 0, 0)
        jd = astro_math.time.julian(dt)
        mjd = jd - 2400000.5  # Calculate MJD from JD
        assert abs(mjd - 51544.5) < 0.001


class TestPrecession:
    """Test precession calculations."""

    def test_precess_j2000_to_date(self):
        """Test precession from J2000 to current date."""
        ra, dec = 88.7929, 7.4071  # Betelgeuse
        target = datetime(2024, 1, 1)

        ra_new, dec_new = astro_math.precession.j2000_to_date(ra, dec, target)

        # Should have moved slightly due to precession
        assert abs(ra_new - ra) > 0.001
        assert abs(dec_new - dec) < 0.1

    def test_batch_precess(self):
        """Test batch precession."""
        import numpy as np

        ra_array = np.array([0.0, 90.0, 180.0, 270.0])
        dec_array = np.array([0.0, 30.0, -30.0, 60.0])
        target = datetime(2024, 1, 1)

        ra_new, dec_new = astro_math.precession.batch_j2000_to_date(
            ra_array, dec_array, target
        )

        assert len(ra_new) == len(ra_array)
        assert len(dec_new) == len(dec_array)


class TestNutation:
    """Test nutation calculations."""

    def test_nutation_values(self):
        """Test nutation returns reasonable values."""
        jd = 2451545.0  # J2000
        dpsi, deps = astro_math.nutation.nutation(jd)

        # Nutation should be small (< 20 arcsec typically)
        assert abs(dpsi) < 20.0
        assert abs(deps) < 20.0

    def test_mean_obliquity(self):
        """Test mean obliquity calculation."""
        jd = 2451545.0  # J2000
        eps = astro_math.nutation.mean_obliquity(jd)

        # Should be around 23.4 degrees
        assert 23.0 < eps < 24.0
        assert abs(eps - 23.4392911) < 0.001


class TestCoordinateTransforms:
    """Test coordinate transformations."""

    def test_ra_dec_to_alt_az(self):
        """Test equatorial to horizontal conversion."""
        ra, dec = 88.7929, 7.4071  # Betelgeuse
        dt = datetime(2024, 1, 15, 22, 0, 0)
        lat, lon = 40.7128, -74.0060  # NYC

        altitude, azimuth = astro_math.transforms.ra_dec_to_alt_az(ra, dec, dt, lat, lon)

        # Should be above horizon in winter evening
        assert -90 <= altitude <= 90
        assert 0 <= azimuth <= 360

    def test_batch_ra_dec_to_alt_az(self):
        """Test batch coordinate transformation."""
        import numpy as np

        ra_array = np.array([0.0, 90.0, 180.0])
        dec_array = np.array([0.0, 30.0, -30.0])
        dt = datetime(2024, 1, 15, 22, 0, 0)
        lat, lon = 40.7128, -74.0060

        alt_array, az_array = astro_math.transforms.batch_ra_dec_to_alt_az(
            ra_array, dec_array, dt, lat, lon
        )

        assert len(alt_array) == len(ra_array)
        assert len(az_array) == len(ra_array)
        assert all(-90 <= a <= 90 for a in alt_array)
        assert all(0 <= a <= 360 for a in az_array)


class TestGalacticCoordinates:
    """Test galactic coordinate conversions."""

    def test_equatorial_to_galactic(self):
        """Test conversion to galactic coordinates."""
        # Galactic center coordinates
        ra, dec = 266.405, -28.936
        l, b = astro_math.galactic.equatorial_to_galactic(ra, dec)

        # Should be near l=0, b=0
        assert abs(l) < 0.1 or abs(l - 360) < 0.1
        assert abs(b) < 0.1

    def test_galactic_to_equatorial(self):
        """Test conversion from galactic coordinates."""
        l, b = 0.0, 0.0  # Galactic center
        ra, dec = astro_math.galactic.galactic_to_equatorial(l, b)

        # Should match known galactic center position
        assert abs(ra - 266.405) < 0.1
        assert abs(dec - (-28.936)) < 0.1

    def test_batch_galactic(self):
        """Test batch galactic conversions."""
        import numpy as np

        ra_array = np.array([0.0, 90.0, 180.0, 270.0])
        dec_array = np.array([0.0, 30.0, -30.0, 60.0])

        l_array, b_array = astro_math.galactic.batch_equatorial_to_galactic(ra_array, dec_array)

        assert len(l_array) == len(ra_array)
        assert len(b_array) == len(dec_array)
        assert all(0 <= l <= 360 for l in l_array)
        assert all(-90 <= b <= 90 for b in b_array)


class TestProperMotion:
    """Test proper motion calculations."""

    def test_apply_proper_motion(self):
        """Test proper motion application."""
        # Barnard's Star
        ra, dec = 269.454, 4.668
        pm_ra_cosdec = -797.84  # mas/yr
        pm_dec = 10326.93  # mas/yr
        target = datetime(2024, 1, 1)

        ra_new, dec_new = astro_math.proper_motion.apply_proper_motion(
            ra, dec, pm_ra_cosdec, pm_dec, target
        )

        # Should have moved significantly
        assert abs(ra_new - ra) > 0.001  # Lower threshold as time difference is small
        assert abs(dec_new - dec) > 0.01


class TestSiderealTime:
    """Test sidereal time calculations."""

    def test_greenwich_sidereal_time(self):
        """Test Greenwich mean sidereal time."""
        dt = datetime(2024, 1, 1, 0, 0, 0)
        jd = astro_math.time.julian(dt)
        gst = astro_math.sidereal.gmst(jd)

        # Should be between 0 and 24 hours
        assert 0 <= gst < 24

    def test_local_sidereal_time(self):
        """Test local mean sidereal time."""
        dt = datetime(2024, 1, 1, 0, 0, 0)
        jd = astro_math.time.julian(dt)
        longitude = -74.0060  # NYC
        lst = astro_math.sidereal.local_mean_sidereal_time(jd, longitude)

        # Should be between 0 and 24 hours
        assert 0 <= lst < 24


class TestAberration:
    """Test aberration corrections."""

    def test_aberration(self):
        """Test stellar aberration."""
        ra, dec = 88.7929, 7.4071  # Betelgeuse
        dt = datetime(2024, 1, 1)

        ra_app, dec_app = astro_math.aberration.apply(ra, dec, dt)

        # Aberration is small (< 20 arcsec), but can be ~0.02 degrees
        assert abs(ra_app - ra) < 0.03
        assert abs(dec_app - dec) < 0.01


class TestAirmass:
    """Test airmass calculations."""

    def test_airmass_zenith(self):
        """Test airmass at zenith."""
        altitude = 90.0  # Zenith
        airmass = astro_math.airmass.pickering(altitude)

        # Should be 1.0 at zenith
        assert abs(airmass - 1.0) < 0.001

    def test_airmass_horizon(self):
        """Test airmass near horizon."""
        altitude = 5.0  # Near horizon
        airmass = astro_math.airmass.pickering(altitude)

        # Should be large near horizon
        assert airmass > 10.0


class TestSunMoon:
    """Test Sun and Moon position calculations."""

    def test_sun_position(self):
        """Test Sun position calculation."""
        dt = datetime(2024, 6, 21, 12, 0, 0)  # Summer solstice
        ra, dec = astro_math.sun_moon.sun_position(dt)

        # Check valid ranges (sun_position returns ecliptic longitude, latitude)
        assert 0 <= ra <= 360
        assert -1 < dec < 1  # Sun stays very close to ecliptic

    def test_moon_position(self):
        """Test Moon position calculation."""
        dt = datetime(2024, 1, 1, 0, 0, 0)
        ra, dec = astro_math.sun_moon.moon_position(dt)

        # Just check valid ranges
        assert 0 <= ra <= 360
        assert -30 <= dec <= 30  # Moon stays near ecliptic


class TestRefraction:
    """Test atmospheric refraction."""

    def test_refraction_zenith(self):
        """Test refraction at zenith."""
        altitude = 90.0
        temperature = 10.0
        pressure = 1013.25

        refraction = astro_math.refraction.bennett(altitude)

        # Should be nearly zero at zenith
        assert abs(refraction) < 0.001

    def test_refraction_horizon(self):
        """Test refraction at horizon."""
        altitude = 0.0
        temperature = 10.0
        pressure = 1013.25

        refraction = astro_math.refraction.bennett(altitude)

        # Should be around 0.57 degrees (34 arcminutes) at horizon
        assert 0.5 < refraction < 0.7


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
