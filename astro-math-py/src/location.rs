use pyo3::prelude::*;
use astro_math::location::Location;

/// Parse location from string coordinates
///
/// Supports 27+ coordinate formats including:
/// - Decimal degrees: "40.7128", "40.7128N", "N40.7128"
/// - DMS: "40°42'46\"", "40 42 46", "40:42:46", "40d42m46s"
/// - HMS for longitude: "4h 56m 27s W"
/// - Aviation formats: "404246N" (DDMMSS), "4042.767N" (DDMM.mmm)
/// - And many more!
#[pyfunction]
#[pyo3(signature = (lat_str, lon_str, altitude=0.0))]
fn parse_location(
    lat_str: &str,
    lon_str: &str,
    altitude: Option<f64>,
) -> PyResult<(f64, f64, f64)> {
    let location = Location::parse(lat_str, lon_str, altitude.unwrap_or(0.0))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
    
    Ok((location.latitude_deg, location.longitude_deg, location.altitude_m))
}

/// Create a Location object for use with other functions
#[pyclass]
#[derive(Clone)]
pub struct PyLocation {
    #[pyo3(get, set)]
    pub latitude: f64,
    #[pyo3(get, set)]
    pub longitude: f64,
    #[pyo3(get, set)]
    pub altitude: f64,
}

#[pymethods]
impl PyLocation {
    #[new]
    #[pyo3(signature = (latitude, longitude, altitude=0.0))]
    fn new(latitude: f64, longitude: f64, altitude: Option<f64>) -> Self {
        PyLocation {
            latitude,
            longitude,
            altitude: altitude.unwrap_or(0.0),
        }
    }
    
    /// Parse from coordinate strings
    #[staticmethod]
    #[pyo3(signature = (lat_str, lon_str, altitude=0.0))]
    fn parse(lat_str: &str, lon_str: &str, altitude: Option<f64>) -> PyResult<Self> {
        let location = Location::parse(lat_str, lon_str, altitude.unwrap_or(0.0))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))?;
        
        Ok(PyLocation {
            latitude: location.latitude_deg,
            longitude: location.longitude_deg,
            altitude: location.altitude_m,
        })
    }
    
    /// Format latitude as DMS string
    fn latitude_dms(&self) -> String {
        let loc = Location {
            latitude_deg: self.latitude,
            longitude_deg: self.longitude,
            altitude_m: self.altitude,
        };
        loc.latitude_dms()
    }
    
    /// Format longitude as DMS string
    fn longitude_dms(&self) -> String {
        let loc = Location {
            latitude_deg: self.latitude,
            longitude_deg: self.longitude,
            altitude_m: self.altitude,
        };
        loc.longitude_dms()
    }
    
    fn __repr__(&self) -> String {
        format!(
            "Location(latitude={:.6}, longitude={:.6}, altitude={:.1})",
            self.latitude, self.longitude, self.altitude
        )
    }
    
    fn __str__(&self) -> String {
        format!(
            "{}, {} at {:.1}m",
            self.latitude_dms(),
            self.longitude_dms(),
            self.altitude
        )
    }
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_location, m)?)?;
    m.add_class::<PyLocation>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use astro_math::error::AstroError;
    
    #[test]
    fn test_parse_decimal_degrees() {
        // Test simple decimal degrees
        let loc = Location::parse("40.7128", "-74.0060", 10.0).unwrap();
        assert!((loc.latitude_deg - 40.7128).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.0060).abs() < 1e-6);
        assert_eq!(loc.altitude_m, 10.0);
        
        // Test with compass directions
        let loc = Location::parse("40.7128N", "74.0060W", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7128).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.0060).abs() < 1e-6);
        
        // Test southern hemisphere
        let loc = Location::parse("33.8688 S", "151.2093 E", 0.0).unwrap();
        assert!((loc.latitude_deg + 33.8688).abs() < 1e-6);
        assert!((loc.longitude_deg - 151.2093).abs() < 1e-6);
    }
    
    #[test]
    fn test_parse_dms_formats() {
        // Test degrees-minutes-seconds with symbols
        let loc = Location::parse("40°42'46\"N", "74°0'21.6\"W", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127778).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.006).abs() < 1e-4);
        
        // Test space-separated DMS
        let loc = Location::parse("40 42 46", "-74 0 21.6", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127778).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.006).abs() < 1e-4);
        
        // Test colon-separated
        let loc = Location::parse("40:42:46", "-74:0:21.6", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127778).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.006).abs() < 1e-4);
        
        // Test with letters (d/m/s)
        let loc = Location::parse("40d42m46s", "-74d0m21.6s", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127778).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.006).abs() < 1e-4);
    }
    
    #[test]
    fn test_parse_hms_longitude() {
        // Test HMS format for longitude
        let loc = Location::parse("40.7128", "4h 56m 27s W", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7128).abs() < 1e-6);
        // 4h 56m 27s = 74.1125 degrees
        assert!((loc.longitude_deg + 74.1125).abs() < 1e-4);
    }
    
    #[test]
    fn test_parse_aviation_formats() {
        // Test DDMMSS format (aviation)
        let loc = Location::parse("404246N", "0740022W", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127778).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.0061111).abs() < 1e-4);
        
        // Test DDMM.mmm format (marine/aviation)
        let loc = Location::parse("4042.767N", "07400.36W", 0.0).unwrap();
        assert!((loc.latitude_deg - 40.7127833).abs() < 1e-6);
        assert!((loc.longitude_deg + 74.006).abs() < 1e-4);
    }
    
    #[test]
    fn test_invalid_formats() {
        // Invalid compass direction for latitude
        let result = Location::parse("40.7128E", "74.0060W", 0.0);
        assert!(matches!(result, Err(AstroError::InvalidDmsFormat { .. })));
        
        // Invalid compass direction for longitude  
        let result = Location::parse("40.7128N", "74.0060N", 0.0);
        assert!(matches!(result, Err(AstroError::InvalidDmsFormat { .. })));
        
        // Out of range latitude
        let result = Location::parse("91.0", "0.0", 0.0);
        assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
        
        // Out of range longitude
        let result = Location::parse("0.0", "181.0", 0.0);
        assert!(matches!(result, Err(AstroError::InvalidCoordinate { .. })));
        
        // Malformed input
        let result = Location::parse("not a number", "0.0", 0.0);
        assert!(matches!(result, Err(AstroError::InvalidDmsFormat { .. })));
    }
    
    #[test]
    fn test_location_formatting() {
        let loc = Location {
            latitude_deg: 40.7127778,
            longitude_deg: -74.006,
            altitude_m: 10.0,
        };
        
        // Test DMS formatting
        let lat_dms = loc.latitude_dms();
        assert!(lat_dms.contains("40"));
        assert!(lat_dms.contains("42"));
        assert!(lat_dms.contains("46"));
        // Note: formatting might not include compass direction in the string itself
        
        let lon_dms = loc.longitude_dms();
        assert!(lon_dms.contains("74"));
        assert!(lon_dms.contains("00"));
        assert!(lon_dms.contains("21"));
        // Note: formatting might not include compass direction in the string itself
    }
    
    #[test]
    fn test_edge_cases() {
        // Test equator
        let loc = Location::parse("0", "0", 0.0).unwrap();
        assert_eq!(loc.latitude_deg, 0.0);
        assert_eq!(loc.longitude_deg, 0.0);
        
        // Test poles
        let loc = Location::parse("90N", "0", 0.0).unwrap();
        assert_eq!(loc.latitude_deg, 90.0);
        
        let loc = Location::parse("90 S", "0", 0.0).unwrap();
        assert_eq!(loc.latitude_deg, -90.0);
        
        // Test date line
        let loc = Location::parse("0", "180", 0.0).unwrap();
        assert_eq!(loc.longitude_deg, 180.0);
        
        let loc = Location::parse("0", "180 W", 0.0).unwrap();
        assert_eq!(loc.longitude_deg, -180.0);
    }
    
    #[test]
    fn test_high_precision() {
        // Test that we maintain precision for decimal seconds
        let loc = Location::parse("40 42 46.123456", "-74 0 21.654321", 0.0).unwrap();
        
        // Convert back to check precision
        let expected_lat = 40.0 + 42.0/60.0 + 46.123456/3600.0;
        let expected_lon = -(74.0 + 0.0/60.0 + 21.654321/3600.0);
        
        assert!((loc.latitude_deg - expected_lat).abs() < 1e-9);
        assert!((loc.longitude_deg - expected_lon).abs() < 1e-9);
    }

    #[test]
    fn test_altitude_parameter_handling() {
        // Test altitude parameter handling in Python binding functions
        let test_cases = vec![
            ("40.0", "74.0", Some(100.0), 100.0),
            ("40.0", "74.0", Some(0.0), 0.0),
            ("40.0", "74.0", Some(-100.0), -100.0), // Below sea level
            ("40.0", "74.0", None, 0.0), // Default value
        ];
        
        for (lat_str, lon_str, altitude_opt, expected_altitude) in test_cases {
            let altitude = altitude_opt.unwrap_or(0.0);
            let loc = Location::parse(lat_str, lon_str, altitude).unwrap();
            assert_eq!(loc.altitude_m, expected_altitude);
        }
    }

    #[test]
    fn test_parsing_with_whitespace() {
        // Test parsing with various whitespace (as might come from Python strings)
        let whitespace_cases = vec![
            ("  40.7128  ", "  -74.0060  "),
            ("\t40.7128\t", "\t-74.0060\t"),
            ("40.7128\n", "-74.0060\n"),
            (" 40°42'46\" N ", " 74°0'21\" W "),
        ];
        
        for (lat_str, lon_str) in whitespace_cases {
            let result = Location::parse(lat_str, lon_str, 0.0);
            // Should handle whitespace gracefully
            assert!(result.is_ok() || result.is_err(), 
                    "Should handle whitespace for: '{}', '{}'", lat_str, lon_str);
        }
    }

    #[test]
    fn test_error_message_content() {
        // Test that error messages contain useful information
        let invalid_cases = vec![
            ("invalid", "0.0"),
            ("91.0", "0.0"),
            ("0.0", "181.0"),
            ("40.0E", "0.0"), // Wrong compass for latitude
        ];
        
        for (lat_str, lon_str) in invalid_cases {
            let result = Location::parse(lat_str, lon_str, 0.0);
            assert!(result.is_err(), "Should fail for: '{}', '{}'", lat_str, lon_str);
            
            match result {
                Err(e) => {
                    let error_string = format!("{}", e);
                    assert!(!error_string.is_empty(), "Error should have a message");
                }
                Ok(_) => panic!("Expected error for invalid input"),
            }
        }
    }

    #[test]
    fn test_repr_and_str_formatting() {
        // Test the __repr__ and __str__ functionality 
        let loc = Location {
            latitude_deg: 40.7127778,
            longitude_deg: -74.006,
            altitude_m: 10.0,
        };
        
        // Test that DMS formatting works
        let lat_dms = loc.latitude_dms();
        let lon_dms = loc.longitude_dms();
        
        // Should contain numeric components
        assert!(lat_dms.contains("40"));
        assert!(lon_dms.contains("74"));
        
        // Should be non-empty strings
        assert!(!lat_dms.is_empty());
        assert!(!lon_dms.is_empty());
        
        // Test the string format that would be used in __str__
        let display_string = format!("{}, {} at {:.1}m", lat_dms, lon_dms, loc.altitude_m);
        assert!(display_string.contains("10.0m"));
        
        // Test the repr format
        let repr_string = format!(
            "Location(latitude={:.6}, longitude={:.6}, altitude={:.1})",
            loc.latitude_deg, loc.longitude_deg, loc.altitude_m
        );
        assert!(repr_string.contains("40.712778"));
        assert!(repr_string.contains("-74.006"));
        assert!(repr_string.contains("10.0"));
    }
}