use crate::erfa::*;
use crate::time::julian_date;
use chrono::{TimeZone, Utc};

#[test]
fn test_precession_matrix() {
    // Test at J2000.0 - should be nearly identity with small frame bias
    let matrix = precession_matrix(2451545.0, 0.0);
    
    // Diagonal elements should be very close to 1
    assert!((matrix[0][0] - 1.0).abs() < 1e-10);
    assert!((matrix[1][1] - 1.0).abs() < 1e-10);
    assert!((matrix[2][2] - 1.0).abs() < 1e-10);
    
    // Off-diagonal elements should be tiny (frame bias)
    assert!(matrix[0][1].abs() < 1e-6);
    assert!(matrix[0][2].abs() < 1e-6);
    assert!(matrix[1][0].abs() < 1e-6);
}

#[test]
fn test_bias_precession_nutation_matrix() {
    // Test bias-precession-nutation matrix at J2000.0
    let matrix = bias_precession_nutation_matrix(2451545.0, 0.0);
    
    // Should be close to identity but includes frame bias and nutation
    assert!((matrix[0][0] - 1.0).abs() < 1e-6);
    assert!((matrix[1][1] - 1.0).abs() < 1e-6);
    assert!((matrix[2][2] - 1.0).abs() < 1e-6);
}

#[test]
fn test_greenwich_mean_sidereal_time() {
    // Test Greenwich Mean Sidereal Time
    // Example from Meeus
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let gmst_rad = greenwich_mean_sidereal_time(jd, 0.0, jd + 69.184/86400.0, 0.0);
    let gmst_hours = gmst_rad * 12.0 / std::f64::consts::PI;
    let gmst_normalized = if gmst_hours < 0.0 { gmst_hours + 24.0 } else { gmst_hours % 24.0 };
    
    // Should be around 13.18 hours for this date
    assert!((gmst_normalized - 13.18).abs() < 0.1);
}

#[test]
fn test_greenwich_apparent_sidereal_time() {
    // Test Greenwich Apparent Sidereal Time (includes nutation)
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let gast_rad = greenwich_apparent_sidereal_time(jd, 0.0, jd + 69.184/86400.0, 0.0);
    let gmst_rad = greenwich_mean_sidereal_time(jd, 0.0, jd + 69.184/86400.0, 0.0);
    
    // GAST should differ from GMST by equation of equinoxes (small)
    let diff_rad = (gast_rad - gmst_rad).abs();
    let diff_arcsec = diff_rad * 206264.806;
    
    // Equation of equinoxes can be up to ~17 arcseconds at maximum nutation
    // Typical values are 0-2 arcsec, but can be larger
    assert!(diff_arcsec < 20.0, "Equation of equinoxes too large: {} arcsec", diff_arcsec);
}

#[test]
fn test_earth_rotation_angle() {
    // Test Earth Rotation Angle
    let jd = 2451545.0; // J2000.0
    
    let era_rad = earth_rotation_angle(jd, 0.0);
    
    // ERA should be between 0 and 2π
    assert!(era_rad >= 0.0 && era_rad < 2.0 * std::f64::consts::PI);
}

#[test]
fn test_icrs_to_cirs() {
    // Test ICRS to CIRS transformation
    let ra_icrs = 100.0_f64.to_radians();
    let dec_icrs = 25.0_f64.to_radians();
    
    let jd = 2451545.0; // J2000.0
    
    let result = icrs_to_cirs(ra_icrs, dec_icrs, 0.0, 0.0, 0.0, 0.0, jd, 0.0);
    assert!(result.is_ok());
    
    let (ra_cirs, dec_cirs, _eo) = result.unwrap();
    
    // At J2000, transformation should be small (mainly frame bias)
    assert!((ra_cirs - ra_icrs).abs() < 0.001); // < ~3 arcmin
    assert!((dec_cirs - dec_icrs).abs() < 0.001);
}

#[test]
fn test_icrs_to_observed_basic() {
    // Basic test of full ICRS to observed transformation
    let ra_icrs = 0.0_f64.to_radians();
    let dec_icrs = 0.0_f64.to_radians();
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let result = icrs_to_observed(
        ra_icrs, dec_icrs,
        0.0, 0.0, 0.0, 0.0,  // no proper motion, parallax, rv
        jd, 0.0,             // UTC
        0.0,                 // dut1
        0.0, 0.0, 0.0,       // observer at 0,0,0
        0.0, 0.0,            // no polar motion
        1013.25, 15.0, 0.5, 0.55, // standard atmosphere
    );
    
    assert!(result.is_ok());
    let (az, zd, _ha, _dec, _ra, _eo) = result.unwrap();
    
    // Results should be valid angles
    assert!(az >= 0.0 && az <= 2.0 * std::f64::consts::PI);
    assert!(zd >= 0.0 && zd <= std::f64::consts::PI);
}

#[test]
fn test_cirs_to_observed_basic() {
    // Test CIRS to observed transformation
    let ri = 0.0_f64.to_radians();
    let di = 0.0_f64.to_radians();
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let result = cirs_to_observed(
        ri, di,
        jd, 0.0,             // UTC
        0.0,                 // dut1
        0.0, 0.0, 0.0,       // observer at equator/prime meridian
        0.0, 0.0,            // no polar motion
        1013.25, 15.0, 0.5, 0.55, // standard atmosphere
    );
    
    assert!(result.is_ok());
    let (az, zd, _ha, _dec, _ra, _eo) = result.unwrap();
    
    // Results should be valid angles
    assert!(az >= 0.0 && az <= 2.0 * std::f64::consts::PI);
    assert!(zd >= 0.0 && zd <= std::f64::consts::PI);
}

#[test]
fn test_icrs_to_observed_with_proper_motion() {
    // Test with proper motion
    let ra_icrs = 279.23473479_f64.to_radians(); // Vega
    let dec_icrs = 38.78368896_f64.to_radians();
    
    // Vega's proper motion (milliarcsec/year to radians/year)
    let pr = 200.94 * std::f64::consts::PI / (180.0 * 3600.0 * 1000.0);
    let pd = 286.23 * std::f64::consts::PI / (180.0 * 3600.0 * 1000.0);
    
    let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let result = icrs_to_observed(
        ra_icrs, dec_icrs,
        pr, pd, 0.130, 0.0,  // proper motion, parallax, no rv
        jd, 0.0,
        0.0,
        0.0, 0.0, 0.0,
        0.0, 0.0,
        0.0, 0.0, 0.0, 0.55, // no atmosphere (space telescope)
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_icrs_to_observed_high_latitude() {
    // Test at high observer latitude
    let ra_icrs = 0.0_f64.to_radians();
    let dec_icrs = 80.0_f64.to_radians(); // Near pole
    
    let dt = Utc.with_ymd_and_hms(2024, 6, 21, 12, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let result = icrs_to_observed(
        ra_icrs, dec_icrs,
        0.0, 0.0, 0.0, 0.0,
        jd, 0.0,
        0.0,
        0.0, 70.0_f64.to_radians(), 0.0, // Arctic observer
        0.0, 0.0,
        1013.25, -20.0, 0.3, 0.55, // Cold arctic conditions
    );
    
    assert!(result.is_ok());
}

#[test]
fn test_precession_matrix_orthogonal() {
    // Test that precession matrix is orthogonal (rotation matrix)
    let matrix = precession_matrix(2460000.0, 0.0);
    
    // Compute transpose times original
    let mut product = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                product[i][j] += matrix[k][i] * matrix[k][j];
            }
        }
    }
    
    // Should be identity
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((product[i][j] - expected).abs() < 1e-10,
                    "Matrix not orthogonal at [{},{}]: {}", i, j, product[i][j]);
        }
    }
}

#[test]
fn test_bias_precession_nutation_matrix_determinant() {
    // Test that bias-precession-nutation matrix has determinant = 1
    let matrix = bias_precession_nutation_matrix(2460000.0, 0.0);
    
    // Calculate determinant
    let det = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1])
            - matrix[0][1] * (matrix[1][0] * matrix[2][2] - matrix[1][2] * matrix[2][0])
            + matrix[0][2] * (matrix[1][0] * matrix[2][1] - matrix[1][1] * matrix[2][0]);
    
    assert!((det - 1.0).abs() < 1e-10, "Determinant = {}, expected 1", det);
}

#[test]
fn test_friendly_function_names() {
    // Verify our friendly function names work correctly
    let jd = 2451545.0;
    
    // Test that we can call functions with friendly names
    let gmst = greenwich_mean_sidereal_time(jd, 0.0, jd, 0.0);
    assert!(!gmst.is_nan());
    
    let gast = greenwich_apparent_sidereal_time(jd, 0.0, jd, 0.0);
    assert!(!gast.is_nan());
    
    let era = earth_rotation_angle(jd, 0.0);
    assert!(!era.is_nan());
    
    let matrix = bias_precession_nutation_matrix(jd, 0.0);
    assert_eq!(matrix.len(), 3);
    assert_eq!(matrix[0].len(), 3);
}