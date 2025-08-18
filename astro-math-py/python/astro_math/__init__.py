"""
High-performance astronomy calculations for Python.

This package provides fast astronomical calculations including:
- Time conversions (Julian Date, J2000)
- Coordinate transformations (RA/Dec ↔ Alt/Az)
- Location parsing (27+ coordinate formats)
- Precession, nutation, and aberration corrections
- Proper motion calculations
- Sidereal time
- Galactic coordinates
- Sun and Moon positions
- Atmospheric effects (refraction, airmass, extinction)

Examples:
    >>> import astro_math
    >>> from datetime import datetime
    >>> 
    >>> # Julian date conversion
    >>> jd = astro_math.julian_date(datetime.utcnow())
    >>> 
    >>> # Coordinate transformation
    >>> alt, az = astro_math.ra_dec_to_alt_az(
    ...     ra=279.23, dec=38.78, 
    ...     dt=datetime.utcnow(),
    ...     latitude=40.7, longitude=-74.0
    ... )
    >>> 
    >>> # Parse any coordinate format
    >>> lat, lon, alt = astro_math.parse_location(
    ...     "40°42'46\"N", "74°0'21.6\"W"
    ... )
"""

from astro_math.astro_math import (
    # Time functions
    julian_date,
    julian_date_batch,
    j2000_days,
    j2000_days_batch,
    
    # Coordinate transforms
    ra_dec_to_alt_az,
    batch_ra_dec_to_alt_az,
    alt_az_to_ra_dec,
    batch_alt_az_to_ra_dec,
    
    # Location parsing
    parse_location,
    PyLocation as Location,
    
    # Precession
    precess_j2000_to_date,
    precess_to_j2000,
    batch_precess_j2000_to_date,
    batch_precess_to_j2000,
    
    # Nutation
    nutation,
    nutation_in_longitude,
    nutation_in_obliquity,
    mean_obliquity,
    true_obliquity,
    
    # Aberration
    apply_aberration,
    remove_aberration,
    aberration_magnitude,
    batch_aberration,
    
    # Proper motion
    apply_proper_motion,
    apply_proper_motion_rigorous,
    total_proper_motion,
    proper_motion_position_angle,
    pm_ra_to_pm_ra_cosdec,
    pm_ra_cosdec_to_pm_ra,
    batch_apply_proper_motion,
    
    # Sidereal time
    gmst,
    local_mean_sidereal_time,
    apparent_sidereal_time,
    
    # Airmass and extinction
    airmass_plane_parallel,
    airmass_young,
    airmass_pickering,
    airmass_kasten_young,
    extinction_magnitudes,
    extinction_coefficient_estimate,
    batch_airmass_pickering,
    batch_extinction,
    
    # Galactic coordinates
    equatorial_to_galactic,
    galactic_to_equatorial,
    galactic_landmarks,
    batch_equatorial_to_galactic,
    batch_galactic_to_equatorial,
    
    # Sun and Moon
    sun_position,
    sun_ra_dec,
    moon_position,
    moon_phase_angle,
    moon_illumination,
    moon_phase_name,
    moon_distance,
    moon_equatorial,
    
    # Refraction
    refraction_bennett,
    refraction_saemundsson,
    refraction_radio,
    apparent_to_true_altitude,
    true_to_apparent_altitude,
    
    # Version info
    __version__,
)

__all__ = [
    # Time
    "julian_date",
    "julian_date_batch",
    "j2000_days",
    "j2000_days_batch",
    
    # Transforms
    "ra_dec_to_alt_az",
    "batch_ra_dec_to_alt_az",
    "alt_az_to_ra_dec",
    "batch_alt_az_to_ra_dec",
    
    # Location
    "parse_location",
    "Location",
    
    # Precession
    "precess_j2000_to_date",
    "precess_to_j2000",
    "batch_precess_j2000_to_date",
    "batch_precess_to_j2000",
    
    # Nutation
    "nutation",
    "nutation_in_longitude",
    "nutation_in_obliquity",
    "mean_obliquity",
    "true_obliquity",
    
    # Aberration
    "apply_aberration",
    "remove_aberration",
    "aberration_magnitude",
    "batch_aberration",
    
    # Proper motion
    "apply_proper_motion",
    "apply_proper_motion_rigorous",
    "total_proper_motion",
    "proper_motion_position_angle",
    "pm_ra_to_pm_ra_cosdec",
    "pm_ra_cosdec_to_pm_ra",
    "batch_apply_proper_motion",
    
    # Sidereal time
    "gmst",
    "local_mean_sidereal_time",
    "apparent_sidereal_time",
    
    # Airmass and extinction
    "airmass_plane_parallel",
    "airmass_young",
    "airmass_pickering",
    "airmass_kasten_young",
    "extinction_magnitudes",
    "extinction_coefficient_estimate",
    "batch_airmass_pickering",
    "batch_extinction",
    
    # Galactic coordinates
    "equatorial_to_galactic",
    "galactic_to_equatorial",
    "galactic_landmarks",
    "batch_equatorial_to_galactic",
    "batch_galactic_to_equatorial",
    
    # Sun and Moon
    "sun_position",
    "sun_ra_dec",
    "moon_position",
    "moon_phase_angle",
    "moon_illumination",
    "moon_phase_name",
    "moon_distance",
    "moon_equatorial",
    
    # Refraction
    "refraction_bennett",
    "refraction_saemundsson",
    "refraction_radio",
    "apparent_to_true_altitude",
    "true_to_apparent_altitude",
    
    # Version
    "__version__",
]