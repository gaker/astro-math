/// Nutation in longitude Δψ (arcseconds)
pub fn nutation_in_longitude_arcsec(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let d = (297.85036 + 445267.111480 * t).to_radians(); // Moon's mean elongation
    let m = (357.52772 + 35999.050340 * t).to_radians(); // Sun's mean anomaly
    let n = -17.20 * d.sin() - 1.32 * m.sin(); // Simplified series
    n
}

/// Mean obliquity of the ecliptic ε (arcseconds)
pub fn mean_obliquity_arcsec(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let seconds = 84381.448 - 46.8150 * t - 0.00059 * t.powi(2) + 0.001813 * t.powi(3);
    seconds
}
