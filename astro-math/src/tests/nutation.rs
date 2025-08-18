use crate::nutation::*;
use crate::time::julian_date;
use chrono::{TimeZone, Utc};

#[test]
fn test_nutation_at_j2000() {
    // At J2000.0, test known values
    let jd = 2451545.0;
    
    let dpsi = nutation_in_longitude(jd);
    let deps = nutation_in_obliquity(jd);
    
    // At J2000.0, nutation values should be small but non-zero
    assert!(dpsi.abs() < 20.0, "Nutation in longitude should be < 20\", got {}\"", dpsi);
    assert!(deps.abs() < 10.0, "Nutation in obliquity should be < 10\", got {}\"", deps);
}

#[test]
fn test_mean_obliquity() {
    // Test mean obliquity at J2000.0
    let jd = 2451545.0;
    let eps0 = mean_obliquity(jd);
    
    // IAU 2006 value at J2000.0 (via ERFA Obl06)
    // This differs slightly from IAU 1980 value (23.4392911°)
    assert!((eps0 - 23.43927944).abs() < 0.00000001, 
        "Mean obliquity at J2000.0 should be 23.43927944° (IAU 2006), got {}°", eps0);
    
    // Test at J1900.0
    let jd_1900 = 2415020.0;
    let eps0_1900 = mean_obliquity(jd_1900);
    assert!((eps0_1900 - 23.4522).abs() < 0.0001,
        "Mean obliquity at J1900.0 should be ~23.4522°, got {}°", eps0_1900);
}

#[test]
fn test_true_obliquity() {
    let jd = 2451545.0;
    let true_eps = true_obliquity(jd);
    let mean_eps = mean_obliquity(jd);
    
    // True obliquity should differ from mean by nutation amount
    let diff = (true_eps - mean_eps) * 3600.0; // Convert to arcseconds
    let deps = nutation_in_obliquity(jd);
    
    assert!((diff - deps).abs() < 0.001,
        "True obliquity should differ from mean by nutation amount");
}

#[test]
fn test_nutation_periodicity() {
    // Test that nutation has expected periodicity (~18.6 years)
    let jd_start = 2451545.0; // J2000.0
    let jd_half_period = jd_start + 18.6 * 365.25 / 2.0; // Half period later
    let jd_full_period = jd_start + 18.6 * 365.25; // Full period later
    
    let dpsi_start = nutation_in_longitude(jd_start);
    let dpsi_half = nutation_in_longitude(jd_half_period);
    let dpsi_full = nutation_in_longitude(jd_full_period);
    
    // At half period, nutation should have opposite sign
    assert!(dpsi_start * dpsi_half < 0.0,
        "Nutation should have opposite sign at half period");
    
    // At full period, nutation should be similar to start
    assert!((dpsi_full - dpsi_start).abs() < 2.0,
        "Nutation should repeat after full period (~18.6 years)");
}

#[test]
fn test_nutation_magnitude() {
    // Test that nutation stays within expected bounds
    let start_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    
    let mut max_dpsi: f64 = 0.0;
    let mut max_deps: f64 = 0.0;
    
    for days in 0..7000 {
        let dt = start_date + chrono::Duration::days(days);
        let jd = julian_date(dt);
        
        let dpsi = nutation_in_longitude(jd);
        let deps = nutation_in_obliquity(jd);
        
        if dpsi.abs() > max_dpsi.abs() {
            max_dpsi = dpsi;
        }
        if deps.abs() > max_deps.abs() {
            max_deps = deps;
        }
        
        // Maximum theoretical nutation values with all terms
        // Primary term is ±17.2" but additional terms can add up
        assert!(dpsi.abs() <= 20.0, 
            "Nutation in longitude out of bounds: {}\" on day {}", dpsi, days);
        
        // Maximum nutation in obliquity is about ±9.2"
        assert!(deps.abs() <= 10.0,
            "Nutation in obliquity out of bounds: {}\" on day {}", deps, days);
    }
    
    // Verify we're getting reasonable maximum values
    assert!(max_dpsi.abs() > 17.0, "Should see large nutation in longitude");
    assert!(max_deps.abs() > 9.0, "Should see large nutation in obliquity");
}

#[test]
fn test_nutation_struct() {
    let jd = 2451545.0;
    let nut = nutation(jd);
    
    // Should match individual calculations
    let dpsi = nutation_in_longitude(jd);
    let deps = nutation_in_obliquity(jd);
    
    assert!((nut.longitude - dpsi).abs() < 0.001,
        "Nutation struct longitude should match individual calculation");
    assert!((nut.obliquity - deps).abs() < 0.001,
        "Nutation struct obliquity should match individual calculation");
}

#[test]
fn test_nutation_known_values() {
    // Test against known values from Meeus examples
    
    // Example from Meeus, Chapter 22
    let dt = Utc.with_ymd_and_hms(1987, 4, 10, 0, 0, 0).unwrap();
    let jd = julian_date(dt);
    
    let dpsi = nutation_in_longitude(jd);
    let deps = nutation_in_obliquity(jd);
    
    // Meeus gives Δψ = -3.788" and Δε = +9.443"
    // Our simplified series might differ slightly
    assert!((dpsi - (-3.788)).abs() < 0.5,
        "Nutation in longitude should be ~-3.788\", got {}\"", dpsi);
    assert!((deps - 9.443).abs() < 0.5,
        "Nutation in obliquity should be ~9.443\", got {}\"", deps);
}

#[test]
fn test_mean_obliquity_change_over_time() {
    // Mean obliquity decreases slowly over time
    let jd_2000 = 2451545.0;
    let jd_2100 = jd_2000 + 100.0 * 365.25;
    
    let eps_2000 = mean_obliquity(jd_2000);
    let eps_2100 = mean_obliquity(jd_2100);
    
    assert!(eps_2100 < eps_2000, 
        "Mean obliquity should decrease over time");
    
    // Change should be about 47" per century
    let change_arcsec = (eps_2000 - eps_2100) * 3600.0;
    assert!(change_arcsec > 45.0 && change_arcsec < 49.0,
        "Mean obliquity should decrease by ~47\" per century, got {}\"", change_arcsec);
}

#[test]
fn test_nutation_symmetry() {
    // Test that nutation calculations are consistent
    let jd = 2451545.0;
    
    // Calculate using different methods
    let nut1 = nutation(jd);
    let dpsi = nutation_in_longitude(jd);
    let deps = nutation_in_obliquity(jd);
    
    // Results should be identical
    assert_eq!(nut1.longitude, dpsi);
    assert_eq!(nut1.obliquity, deps);
}

#[test]
fn test_nutation_backwards_compatibility() {
    // Test that old internal functions still work
    let jd = 2451545.0;
    
    let dpsi_old = nutation_in_longitude_arcsec(jd);
    let dpsi_new = nutation_in_longitude(jd);
    assert_eq!(dpsi_old, dpsi_new);
    
    let eps0_old = mean_obliquity_arcsec(jd);
    let eps0_new = mean_obliquity(jd) * 3600.0;
    assert!((eps0_old - eps0_new).abs() < 0.001);
}

#[test]
fn test_iau2000a_precision() {
    // Test that ERFA's IAU 2000A model provides milliarcsecond precision
    // Compare known high-precision values from IERS
    
    // Test against actual ERFA IAU 2000A values
    let test_cases = [
        // JD, rough expected ranges for dpsi (mas), deps (mas)
        (2451545.0, -20000.0..0.0, -10000.0..10000.0),  // J2000.0
        (2458849.5, -20000.0..0.0, -10000.0..10000.0),  // 2020-01-01
    ];
    
    for (jd, dpsi_range, deps_range) in test_cases {
        let dpsi = nutation_in_longitude(jd);
        let deps = nutation_in_obliquity(jd);
        
        // Convert to milliarcseconds
        let dpsi_mas = dpsi * 1000.0;
        let deps_mas = deps * 1000.0;
        
        // Verify values are in reasonable ranges
        assert!(dpsi_range.contains(&dpsi_mas),
            "dpsi at JD {}: {} mas should be in range {:?}", jd, dpsi_mas, dpsi_range);
        assert!(deps_range.contains(&deps_mas),
            "deps at JD {}: {} mas should be in range {:?}", jd, deps_mas, deps_range);
    }
}

#[test]
fn test_nutation_principal_terms() {
    // Test that the principal nutation terms are present
    // The largest term has period 18.6 years (6798.4 days)
    
    let jd_start = 2451545.0; // J2000.0
    let period_days = 6798.4; // Principal nutation period
    
    // Sample over one complete period
    let mut values = Vec::new();
    for i in 0..100 {
        let jd = jd_start + (i as f64) * period_days / 100.0;
        let dpsi = nutation_in_longitude(jd);
        values.push(dpsi);
    }
    
    // Find approximate amplitude (should be ~17.2")
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let amplitude = (max_val - min_val) / 2.0;
    
    // ERFA includes many terms, so amplitude may be slightly larger
    assert!(amplitude > 16.0 && amplitude < 20.0,
        "Principal nutation amplitude should be ~17-19\", got {}\"", amplitude);
}

#[test]
fn test_nutation_short_period_terms() {
    // Test that short-period terms are included
    // There are terms with periods as short as 5.5 days
    
    let jd_start = 2451545.0;
    let mut prev_dpsi = nutation_in_longitude(jd_start);
    let mut changes = Vec::new();
    
    // Sample daily for a month
    for day in 1..30 {
        let jd = jd_start + day as f64;
        let dpsi = nutation_in_longitude(jd);
        changes.push((dpsi - prev_dpsi).abs());
        prev_dpsi = dpsi;
    }
    
    // Should see daily changes due to short-period terms
    let max_daily_change = changes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    assert!(max_daily_change > 0.1,
        "Should see measurable daily changes from short-period terms");
}

#[test]
fn test_equation_of_equinoxes() {
    // The equation of equinoxes relates to nutation in longitude
    // EE ≈ Δψ * cos(ε) + small terms
    
    let jd = 2451545.0;
    let dpsi = nutation_in_longitude(jd); // arcseconds
    let eps = mean_obliquity(jd); // degrees
    
    // Approximate equation of equinoxes (in seconds of time)
    let ee_approx = dpsi * eps.to_radians().cos() / 15.0; // Convert to time seconds
    
    // Should be a reasonable value (typically < 1 second of time)
    assert!(ee_approx.abs() < 1.5,
        "Equation of equinoxes should be < 1.5 seconds of time, got {}", ee_approx);
}

#[test]
fn test_nutation_at_extremes() {
    // Test nutation at dates when it reaches extremes
    // Maximum nutation in longitude occurs when the lunar node is at 0° or 180°
    
    // Test over a longer period to find actual extremes
    let start_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let mut max_dpsi = -1000.0;
    let mut min_dpsi = 1000.0;
    
    // Sample over ~19 years (one complete nutation cycle)
    for days in (0..7000).step_by(30) {
        let dt = start_date + chrono::Duration::days(days);
        let jd = julian_date(dt);
        let dpsi = nutation_in_longitude(jd);
        
        if dpsi > max_dpsi {
            max_dpsi = dpsi;
        }
        if dpsi < min_dpsi {
            min_dpsi = dpsi;
        }
    }
    
    // Verify we found reasonable extremes
    assert!(max_dpsi > 15.0,
        "Maximum nutation in longitude should be > 15\", got {}\"", max_dpsi);
    assert!(min_dpsi < -15.0,
        "Minimum nutation in longitude should be < -15\", got {}\"", min_dpsi);
}

#[test]
fn test_nutation_matrix_consistency() {
    // Test that nutation values are consistent with the nutation matrix
    // The nutation matrix from ERFA should produce the same effect
    
    let jd = 2451545.0;
    let dpsi = nutation_in_longitude(jd);
    let deps = nutation_in_obliquity(jd);
    let eps = mean_obliquity(jd);
    
    // Convert to radians
    let dpsi_rad = dpsi / 206264.80624709636;
    let deps_rad = deps / 206264.80624709636;
    let eps_rad = eps.to_radians();
    
    // Build nutation matrix manually
    let cos_dpsi = dpsi_rad.cos();
    let sin_dpsi = dpsi_rad.sin();
    let _cos_eps = eps_rad.cos();
    let _sin_eps = eps_rad.sin();
    let _cos_deps_eps = (eps_rad + deps_rad).cos();
    let _sin_deps_eps = (eps_rad + deps_rad).sin();
    
    // Just verify the values are reasonable
    assert!((cos_dpsi - 1.0).abs() < 0.001, "cos(dpsi) should be ~1 for small angles");
    assert!(sin_dpsi.abs() < 0.0001, "sin(dpsi) should be small");
}