//! ERFA wrapper functions for coordinate transformations and astronomical calculations.
//!
//! This module provides safe wrappers around the ERFA (Essential Routines for 
//! Fundamental Astronomy) C library functions, ensuring compatibility with
//! astropy and other professional astronomy software.
//!
//! # Function Naming
//!
//! We use descriptive names instead of ERFA's cryptic abbreviations:
//! - `greenwich_mean_sidereal_time` instead of `gmst06`
//! - `greenwich_apparent_sidereal_time` instead of `gst06a`
//! - `earth_rotation_angle` instead of `era00`
//! - `bias_precession_nutation_matrix` instead of `pnm06a`

use crate::error::{AstroError, Result};

/// Transform ICRS coordinates to observed (horizontal) coordinates.
///
/// This uses ERFA's Atco13 function which implements the full IAU 2000/2006
/// transformation pipeline including:
/// - Frame bias
/// - Precession-nutation
/// - Earth rotation
/// - Polar motion
/// - Diurnal aberration
/// - Atmospheric refraction
///
/// # Arguments
///
/// * `ra_icrs` - ICRS right ascension (radians)
/// * `dec_icrs` - ICRS declination (radians)
/// * `pr` - Proper motion in RA (radians/year)
/// * `pd` - Proper motion in Dec (radians/year)
/// * `px` - Parallax (arcsec)
/// * `rv` - Radial velocity (km/s, positive = receding)
/// * `utc1` - UTC as JD (part 1)
/// * `utc2` - UTC as JD (part 2)
/// * `dut1` - UT1-UTC (seconds)
/// * `elong` - Longitude (radians, east positive)
/// * `phi` - Latitude (radians)
/// * `hm` - Height above ellipsoid (meters)
/// * `xp` - Polar motion x (radians)
/// * `yp` - Polar motion y (radians)
/// * `phpa` - Pressure (hPa)
/// * `tc` - Temperature (Celsius)
/// * `rh` - Relative humidity (0-1)
/// * `wl` - Wavelength (micrometers)
///
/// # Returns
///
/// * `Result<(f64, f64, f64, f64, f64, f64)>` - (azimuth, zenith distance, hour angle, declination, RA, declination) all in radians
pub fn icrs_to_observed(
    ra_icrs: f64,
    dec_icrs: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    utc1: f64,
    utc2: f64,
    dut1: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    phpa: f64,
    tc: f64,
    rh: f64,
    wl: f64,
) -> Result<(f64, f64, f64, f64, f64, f64)> {
    match erfars::astrometry::Atco13(
        ra_icrs, dec_icrs, pr, pd, px, rv,
        utc1, utc2, dut1, elong, phi, hm,
        xp, yp, phpa, tc, rh, wl,
    ) {
        Ok((aob, zob, hob, dob, rob, eo)) => Ok((aob, zob, hob, dob, rob, eo)),
        Err(_) => Err(AstroError::CalculationError {
            calculation: "ERFA Atco13",
            reason: "Failed to transform ICRS to observed coordinates".to_string(),
        }),
    }
}

/// Transform ICRS to CIRS (Celestial Intermediate Reference System).
///
/// This handles proper motion, parallax, light deflection, and aberration.
///
/// # Arguments
///
/// * `ra_icrs` - ICRS right ascension (radians)
/// * `dec_icrs` - ICRS declination (radians)
/// * `pr` - Proper motion in RA (radians/year)
/// * `pd` - Proper motion in Dec (radians/year)
/// * `px` - Parallax (arcsec)
/// * `rv` - Radial velocity (km/s, positive = receding)
/// * `date1` - TDB as JD (part 1)
/// * `date2` - TDB as JD (part 2)
///
/// # Returns
///
/// * `Result<(f64, f64, f64)>` - (RA, Dec, equation of origins) in radians
pub fn icrs_to_cirs(
    ra_icrs: f64,
    dec_icrs: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    date1: f64,
    date2: f64,
) -> Result<(f64, f64, f64)> {
    let (ri, di, eo) = erfars::astrometry::Atci13(
        ra_icrs, dec_icrs, pr, pd, px, rv, date1, date2,
    );
    Ok((ri, di, eo))
}

/// Transform CIRS to observed coordinates.
///
/// Applies Earth rotation, polar motion, diurnal aberration, and refraction.
///
/// # Arguments
///
/// * `ri` - CIRS right ascension (radians)
/// * `di` - CIRS declination (radians)
/// * `utc1` - UTC as JD (part 1)
/// * `utc2` - UTC as JD (part 2)
/// * `dut1` - UT1-UTC (seconds)
/// * `elong` - Longitude (radians, east positive)
/// * `phi` - Latitude (radians)
/// * `hm` - Height above ellipsoid (meters)
/// * `xp` - Polar motion x (radians)
/// * `yp` - Polar motion y (radians)
/// * `phpa` - Pressure (hPa)
/// * `tc` - Temperature (Celsius)
/// * `rh` - Relative humidity (0-1)
/// * `wl` - Wavelength (micrometers)
///
/// # Returns
///
/// * `Result<(f64, f64, f64, f64, f64, f64)>` - (azimuth, zenith distance, hour angle, declination, RA, declination) in radians
pub fn cirs_to_observed(
    ri: f64,
    di: f64,
    utc1: f64,
    utc2: f64,
    dut1: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    phpa: f64,
    tc: f64,
    rh: f64,
    wl: f64,
) -> Result<(f64, f64, f64, f64, f64, f64)> {
    match erfars::astrometry::Atio13(
        ri, di, utc1, utc2, dut1, elong, phi, hm,
        xp, yp, phpa, tc, rh, wl,
    ) {
        Ok((aob, zob, hob, dob, rob)) => Ok((aob, zob, hob, dob, rob, 0.0)),
        Err(_) => Err(AstroError::CalculationError {
            calculation: "ERFA Atio13",
            reason: "Failed to transform CIRS to observed coordinates".to_string(),
        }),
    }
}

/// Calculate Greenwich Mean Sidereal Time using ERFA IAU 2006 model.
///
/// # Arguments
///
/// * `ut11` - UT1 as JD (part 1)
/// * `ut12` - UT1 as JD (part 2)
/// * `tt1` - TT as JD (part 1)
/// * `tt2` - TT as JD (part 2)
///
/// # Returns
///
/// GMST in radians
pub fn greenwich_mean_sidereal_time(ut11: f64, ut12: f64, tt1: f64, tt2: f64) -> f64 {
    erfars::rotationtime::Gmst06(ut11, ut12, tt1, tt2)
}

/// Calculate Greenwich Apparent Sidereal Time using ERFA IAU 2006 model.
/// 
/// This includes nutation corrections for the true sidereal time.
///
/// # Arguments
///
/// * `ut11` - UT1 as JD (part 1)
/// * `ut12` - UT1 as JD (part 2)
/// * `tt1` - TT as JD (part 1)
/// * `tt2` - TT as JD (part 2)
///
/// # Returns
///
/// GAST in radians
pub fn greenwich_apparent_sidereal_time(ut11: f64, ut12: f64, tt1: f64, tt2: f64) -> f64 {
    erfars::rotationtime::Gst06a(ut11, ut12, tt1, tt2)
}

/// Calculate Earth Rotation Angle.
///
/// The angle through which Earth has rotated since the J2000.0 epoch.
///
/// # Arguments
///
/// * `ut11` - UT1 as JD (part 1)
/// * `ut12` - UT1 as JD (part 2)
///
/// # Returns
///
/// ERA in radians
pub fn earth_rotation_angle(ut11: f64, ut12: f64) -> f64 {
    erfars::rotationtime::Era00(ut11, ut12)
}

/// Get precession matrix at a given epoch.
///
/// # Arguments
///
/// * `date1` - TT as JD (part 1)
/// * `date2` - TT as JD (part 2)
///
/// # Returns
///
/// 3x3 precession matrix from J2000 to date
pub fn precession_matrix(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    let mut rbp = [0.0; 9];
    erfars::precnutpolar::Pmat06(date1, date2, &mut rbp);
    
    // Convert from flat array to 3x3 matrix
    [
        [rbp[0], rbp[1], rbp[2]],
        [rbp[3], rbp[4], rbp[5]],
        [rbp[6], rbp[7], rbp[8]],
    ]
}

/// Get bias-precession-nutation matrix.
///
/// # Arguments
///
/// * `date1` - TT as JD (part 1)
/// * `date2` - TT as JD (part 2)
///
/// # Returns
///
/// 3x3 BPN matrix
pub fn bias_precession_nutation_matrix(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    let mut rbpn = [0.0; 9];
    erfars::precnutpolar::Pnm06a(date1, date2, &mut rbpn);
    
    // Convert from flat array to 3x3 matrix
    [
        [rbpn[0], rbpn[1], rbpn[2]],
        [rbpn[3], rbpn[4], rbpn[5]],
        [rbpn[6], rbpn[7], rbpn[8]],
    ]
}