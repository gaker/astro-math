//! Standard tangent plane projection for astronomical imaging.
//!
//! This module provides the tangent plane (gnomonic) projection commonly used in
//! astronomical imaging to convert between celestial coordinates (RA/Dec) and 
//! pixel coordinates (X/Y).
//!
//! # Overview
//!
//! The tangent plane projection is ideal for small fields of view where distortion
//! is minimal. It's the standard projection used in FITS files and most astronomical
//! CCD images.
//!
//! # Error Handling
//!
//! Functions return `Result<T>` types with these possible errors:
//! - `AstroError::InvalidCoordinate` for out-of-range RA or Dec values
//! - `AstroError::ProjectionError` when a point cannot be projected (e.g., on opposite side of sky)
//! - `AstroError::OutOfRange` for invalid scale values

use crate::error::{Result, AstroError, validate_ra, validate_dec};

/// Tangent plane (gnomonic) projection for converting RA/Dec to X/Y pixel coordinates.
///
/// This is the standard projection used in most astronomical imaging. It provides
/// accurate representation of small fields of view with minimal distortion.
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
    /// Create a new tangent plane projection.
    ///
    /// # Arguments
    /// * `ra0` - Reference RA in degrees (projection center)
    /// * `dec0` - Reference Dec in degrees (projection center)
    /// * `scale` - Pixel scale in arcseconds per pixel (must be positive)
    ///
    /// # Errors
    /// - `AstroError::InvalidCoordinate` if RA is outside [0, 360) or Dec outside [-90, 90]
    /// - `AstroError::OutOfRange` if scale is not positive
    ///
    /// # Example
    /// ```
    /// use astro_math::projection::TangentPlane;
    /// 
    /// let tp = TangentPlane::new(180.0, 45.0, 1.0).unwrap();
    /// ```
    pub fn new(ra0: f64, dec0: f64, scale: f64) -> Result<Self> {
        validate_ra(ra0)?;
        validate_dec(dec0)?;
        if scale <= 0.0 {
            return Err(AstroError::OutOfRange {
                parameter: "scale",
                value: scale,
                min: f64::MIN_POSITIVE,
                max: f64::MAX,
            });
        }
        Ok(Self {
            ra0,
            dec0,
            scale,
            rotation: 0.0,
            crpix1: 0.0,
            crpix2: 0.0,
        })
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
    
    /// Project RA/Dec coordinates to pixel coordinates.
    ///
    /// # Arguments
    /// * `ra` - Right ascension in degrees
    /// * `dec` - Declination in degrees
    ///
    /// # Returns
    /// (x, y) pixel coordinates
    ///
    /// # Errors
    /// - `AstroError::InvalidCoordinate` if RA or Dec is out of range
    /// - `AstroError::ProjectionError` if point is on opposite side of sky
    ///
    /// # Example
    /// ```
    /// # use astro_math::projection::TangentPlane;
    /// let tp = TangentPlane::new(180.0, 0.0, 1.0).unwrap()
    ///     .with_reference_pixel(1024.0, 1024.0);
    /// 
    /// // Project a point near the reference
    /// let (x, y) = tp.ra_dec_to_pixel(180.1, 0.1).unwrap();
    /// assert!((x - 1024.0).abs() < 500.0); // Should be near center
    /// ```
    pub fn ra_dec_to_pixel(&self, ra: f64, dec: f64) -> Result<(f64, f64)> {
        validate_ra(ra)?;
        validate_dec(dec)?;
        
        // Convert to radians for ERFA
        let ra_rad = ra.to_radians();
        let dec_rad = dec.to_radians();
        let ra0_rad = self.ra0.to_radians();
        let dec0_rad = self.dec0.to_radians();
        
        // Use ERFA's Tpxes for tangent plane projection
        // Returns standard coordinates (xi, eta) in radians
        let result = erfars::gnomonic::Tpxes(ra_rad, dec_rad, ra0_rad, dec0_rad);
        
        let (xi, eta) = match result {
            Ok((xi, eta)) => (xi, eta),
            Err(_) => {
                return Err(AstroError::ProjectionError {
                    reason: "Point is on opposite side of sky from projection center".to_string(),
                });
            }
        };
        
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
        
        Ok((x, y))
    }
    
    /// Inverse projection: pixel to RA/Dec.
    ///
    /// # Arguments
    /// * `x` - X pixel coordinate
    /// * `y` - Y pixel coordinate
    ///
    /// # Returns
    /// (ra, dec) in degrees
    ///
    /// # Example
    /// ```
    /// # use astro_math::projection::TangentPlane;
    /// let tp = TangentPlane::new(180.0, 0.0, 1.0).unwrap()
    ///     .with_reference_pixel(1024.0, 1024.0);
    /// 
    /// // Convert center pixel back to sky coordinates
    /// let (ra, dec) = tp.pixel_to_ra_dec(1024.0, 1024.0).unwrap();
    /// assert!((ra - 180.0).abs() < 0.001);
    /// assert!(dec.abs() < 0.001);
    /// ```
    pub fn pixel_to_ra_dec(&self, x: f64, y: f64) -> Result<(f64, f64)> {
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
        
        // Convert to radians for ERFA
        let xi = xi_unrot.to_radians();
        let eta = eta_unrot.to_radians();
        
        let ra0_rad = self.ra0.to_radians();
        let dec0_rad = self.dec0.to_radians();
        
        // Use ERFA's Tpsts for inverse tangent plane projection
        let (ra_rad, dec_rad) = erfars::gnomonic::Tpsts(xi, eta, ra0_rad, dec0_rad);
        
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
        
        Ok((ra, dec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tangent_plane_projection() {
        // Test projection at reference point
        let tp = TangentPlane::new(180.0, 45.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        
        let (x, y) = tp.ra_dec_to_pixel(180.0, 45.0).unwrap();
        assert!((x - 512.0).abs() < 1e-10);
        assert!((y - 512.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_projection_round_trip() {
        let tp = TangentPlane::new(83.8, -5.4, 2.0).unwrap()
            .with_reference_pixel(1024.0, 1024.0)
            .with_rotation(15.0);
        
        // Test a point near the reference
        let ra_orig = 84.0;
        let dec_orig = -5.5;
        
        let (x, y) = tp.ra_dec_to_pixel(ra_orig, dec_orig).unwrap();
        let (ra_back, dec_back) = tp.pixel_to_ra_dec(x, y).unwrap();
        
        assert!((ra_orig - ra_back).abs() < 1e-10);
        assert!((dec_orig - dec_back).abs() < 1e-10);
    }

    #[test]
    fn test_opposite_side_of_sky() {
        // Test projection of point on opposite side of sky (line 68)
        let tp = TangentPlane::new(0.0, 0.0, 1.0).unwrap();
        let result = tp.ra_dec_to_pixel(180.0, 0.0);
        assert!(result.is_err());
        assert!(matches!(result, Err(AstroError::ProjectionError { .. })));
    }

    #[test]
    fn test_pixel_to_radec_at_reference() {
        // Test inverse projection at reference point (lines 128, 134)
        let tp = TangentPlane::new(100.0, 30.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        
        let (ra, dec) = tp.pixel_to_ra_dec(512.0, 512.0).unwrap();
        assert!((ra - 100.0).abs() < 1e-10);
        assert!((dec - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_ra_normalization() {
        // Test RA normalization in pixel_to_ra_dec (lines 145, 148)
        let tp = TangentPlane::new(1.0, 0.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        
        // Test point that would give negative RA
        let (ra1, _) = tp.pixel_to_ra_dec(1000.0, 512.0).unwrap();
        assert!((0.0..360.0).contains(&ra1));
        
        // Test point that would give RA > 360
        let tp2 = TangentPlane::new(359.0, 0.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        let (ra2, _) = tp2.pixel_to_ra_dec(100.0, 512.0).unwrap();
        assert!((0.0..360.0).contains(&ra2));
    }
    
    #[test]
    fn test_projection_ra_while_loops() {
        // Test projection RA normalization while loops
        let tp = TangentPlane::new(0.0, 0.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        
        // Create a pixel that would result in RA < 0
        let (ra1, _) = tp.pixel_to_ra_dec(2000.0, 512.0).unwrap();
        assert!((0.0..360.0).contains(&ra1));
        
        // Create a pixel that would result in RA > 360
        let tp2 = TangentPlane::new(359.9, 0.0, 1.0).unwrap()
            .with_reference_pixel(512.0, 512.0);
        let (ra2, _) = tp2.pixel_to_ra_dec(100.0, 512.0).unwrap();
        assert!((0.0..360.0).contains(&ra2));
    }
}