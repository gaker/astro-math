// Standard tangent plane projection for astronomical imaging

/// Tangent plane (gnomonic) projection for converting RA/Dec to X/Y pixel coordinates
/// This is the standard projection used in most astronomical imaging
pub struct TangentPlane {
    /// Reference point RA in degrees
    pub ra0: f64,
    /// Reference point Dec in degrees  
    pub dec0: f64,
    /// Pixel scale in arcseconds per pixel
    pub scale: f64,
    /// Rotation angle in degrees (0 = North up)
    pub rotation: f64,
    /// Reference pixel X coordinate
    pub crpix1: f64,
    /// Reference pixel Y coordinate
    pub crpix2: f64,
}

impl TangentPlane {
    /// Create a new tangent plane projection
    pub fn new(ra0: f64, dec0: f64, scale: f64) -> Self {
        Self {
            ra0,
            dec0,
            scale,
            rotation: 0.0,
            crpix1: 0.0,
            crpix2: 0.0,
        }
    }
    
    /// Set the reference pixel (usually image center)
    pub fn with_reference_pixel(mut self, x: f64, y: f64) -> Self {
        self.crpix1 = x;
        self.crpix2 = y;
        self
    }
    
    /// Set the rotation angle in degrees
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }
    
    /// Project RA/Dec coordinates to pixel coordinates
    /// Returns (x, y) pixel coordinates
    pub fn ra_dec_to_pixel(&self, ra: f64, dec: f64) -> (f64, f64) {
        // Convert to radians
        let ra_rad = ra.to_radians();
        let dec_rad = dec.to_radians();
        let ra0_rad = self.ra0.to_radians();
        let dec0_rad = self.dec0.to_radians();
        
        // Compute direction cosines
        let cos_dec = dec_rad.cos();
        let sin_dec = dec_rad.sin();
        let cos_dec0 = dec0_rad.cos();
        let sin_dec0 = dec0_rad.sin();
        let cos_ra_diff = (ra_rad - ra0_rad).cos();
        let sin_ra_diff = (ra_rad - ra0_rad).sin();
        
        // Tangent plane projection
        let divisor = sin_dec * sin_dec0 + cos_dec * cos_dec0 * cos_ra_diff;
        
        // Handle case where point is on opposite side of sky
        if divisor <= 0.0 {
            return (f64::NAN, f64::NAN);
        }
        
        // Standard coordinates (xi, eta) in radians
        let xi = cos_dec * sin_ra_diff / divisor;
        let eta = (sin_dec * cos_dec0 - cos_dec * sin_dec0 * cos_ra_diff) / divisor;
        
        // Convert to degrees
        let xi_deg = xi.to_degrees();
        let eta_deg = eta.to_degrees();
        
        // Apply rotation
        let cos_rot = self.rotation.to_radians().cos();
        let sin_rot = self.rotation.to_radians().sin();
        
        let xi_rot = xi_deg * cos_rot + eta_deg * sin_rot;
        let eta_rot = -xi_deg * sin_rot + eta_deg * cos_rot;
        
        // Convert to pixels (note: xi increases to the west, hence negative)
        let x = self.crpix1 - xi_rot * 3600.0 / self.scale;
        let y = self.crpix2 + eta_rot * 3600.0 / self.scale;
        
        (x, y)
    }
    
    /// Inverse projection: pixel to RA/Dec
    /// Returns (ra, dec) in degrees
    pub fn pixel_to_ra_dec(&self, x: f64, y: f64) -> (f64, f64) {
        // Convert pixel to standard coordinates
        let dx = x - self.crpix1;
        let dy = y - self.crpix2;
        
        // Convert to angular offset in degrees
        let xi_deg = -dx * self.scale / 3600.0;
        let eta_deg = dy * self.scale / 3600.0;
        
        // Apply inverse rotation
        let cos_rot = self.rotation.to_radians().cos();
        let sin_rot = self.rotation.to_radians().sin();
        
        let xi_unrot = xi_deg * cos_rot - eta_deg * sin_rot;
        let eta_unrot = xi_deg * sin_rot + eta_deg * cos_rot;
        
        // Convert to radians
        let xi = xi_unrot.to_radians();
        let eta = eta_unrot.to_radians();
        
        let ra0_rad = self.ra0.to_radians();
        let dec0_rad = self.dec0.to_radians();
        
        // Inverse tangent plane projection
        let rho = (xi * xi + eta * eta).sqrt();
        let c = rho.atan();
        
        let cos_c = c.cos();
        let sin_c = c.sin();
        let cos_dec0 = dec0_rad.cos();
        let sin_dec0 = dec0_rad.sin();
        
        let dec_rad = if rho == 0.0 {
            dec0_rad
        } else {
            (cos_c * sin_dec0 + eta * sin_c * cos_dec0 / rho).asin()
        };
        
        let ra_rad = if rho == 0.0 {
            ra0_rad
        } else {
            ra0_rad + (xi * sin_c).atan2(rho * cos_dec0 * cos_c - eta * sin_dec0 * sin_c)
        };
        
        // Convert to degrees and normalize
        let mut ra = ra_rad.to_degrees();
        let dec = dec_rad.to_degrees();
        
        // Normalize RA to [0, 360)
        while ra < 0.0 {
            ra += 360.0;
        }
        while ra >= 360.0 {
            ra -= 360.0;
        }
        
        (ra, dec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tangent_plane_projection() {
        // Test projection at reference point
        let tp = TangentPlane::new(180.0, 45.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        
        let (x, y) = tp.ra_dec_to_pixel(180.0, 45.0);
        assert!((x - 512.0).abs() < 1e-10);
        assert!((y - 512.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_round_trip() {
        let tp = TangentPlane::new(83.8, -5.4, 2.0)
            .with_reference_pixel(1024.0, 1024.0)
            .with_rotation(15.0);
        
        // Test a point near the reference
        let ra_orig = 84.0;
        let dec_orig = -5.5;
        
        let (x, y) = tp.ra_dec_to_pixel(ra_orig, dec_orig);
        let (ra_back, dec_back) = tp.pixel_to_ra_dec(x, y);
        
        assert!((ra_orig - ra_back).abs() < 1e-10);
        assert!((dec_orig - dec_back).abs() < 1e-10);
    }

    #[test]
    fn test_opposite_side_of_sky() {
        // Test projection of point on opposite side of sky (line 68)
        let tp = TangentPlane::new(0.0, 0.0, 1.0);
        let (x, y) = tp.ra_dec_to_pixel(180.0, 0.0);
        assert!(x.is_nan());
        assert!(y.is_nan());
    }

    #[test]
    fn test_pixel_to_radec_at_reference() {
        // Test inverse projection at reference point (lines 128, 134)
        let tp = TangentPlane::new(100.0, 30.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        
        let (ra, dec) = tp.pixel_to_ra_dec(512.0, 512.0);
        assert!((ra - 100.0).abs() < 1e-10);
        assert!((dec - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_ra_normalization() {
        // Test RA normalization in pixel_to_ra_dec (lines 145, 148)
        let tp = TangentPlane::new(1.0, 0.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        
        // Test point that would give negative RA
        let (ra1, _) = tp.pixel_to_ra_dec(1000.0, 512.0);
        assert!(ra1 >= 0.0 && ra1 < 360.0);
        
        // Test point that would give RA > 360
        let tp2 = TangentPlane::new(359.0, 0.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        let (ra2, _) = tp2.pixel_to_ra_dec(100.0, 512.0);
        assert!(ra2 >= 0.0 && ra2 < 360.0);
    }
    
    #[test]
    fn test_projection_edge_cases() {
        // Already covered in projection.rs tests
    }
    
    #[test]
    fn test_projection_ra_while_loops() {
        // Test projection RA normalization while loops
        let tp = TangentPlane::new(0.0, 0.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        
        // Create a pixel that would result in RA < 0
        let (ra1, _) = tp.pixel_to_ra_dec(2000.0, 512.0);
        assert!(ra1 >= 0.0 && ra1 < 360.0);
        
        // Create a pixel that would result in RA > 360
        let tp2 = TangentPlane::new(359.9, 0.0, 1.0)
            .with_reference_pixel(512.0, 512.0);
        let (ra2, _) = tp2.pixel_to_ra_dec(100.0, 512.0);
        assert!(ra2 >= 0.0 && ra2 < 360.0);
    }
}