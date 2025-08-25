use astro_math::location::Location;

fn main() {
    println!("Astro-Math: Coordinate Parsing Demo\n");
    
    let examples = vec![
        // Decimal degrees
        ("40.7128", "-74.0060", "New York City - Basic decimal degrees"),
        ("+40.7128", "-74.0060", "New York City - With plus sign"),
        ("40.7128N", "74.0060W", "New York City - With compass suffixes"),
        ("N40.7128", "W74.0060", "New York City - With compass prefixes"),
        ("40.7128 North", "74.0060 West", "New York City - Spelled out directions"),
        
        // DMS formats
        ("40 42 46", "-74 00 21.6", "NYC - Space-separated DMS"),
        ("40:42:46", "-74:00:21.6", "NYC - Colon-separated DMS"),
        ("40-42-46", "-74-00-21.6", "NYC - Dash-separated DMS"),
        ("40°42'46\"", "-74°00'21.6\"", "NYC - Traditional DMS symbols"),
        ("40°42′46″", "-74°00′21.6″", "NYC - Unicode prime symbols"),
        ("40d42m46s", "-74d00m21.6s", "NYC - d/m/s notation"),
        ("40 degrees 42 minutes 46 seconds", "-74 degrees 0 minutes 21.6 seconds", "NYC - Verbose DMS"),
        ("40°42'46\"N", "74°00'21.6\"W", "NYC - DMS with compass"),
        
        // Mixed separators and formats
        ("40d 42' 46\"", "-74:00:21.6", "NYC - Mixed separators"),
        ("  40   42   46  ", " -74  0  21.6 ", "NYC - Extra spaces"),
        
        // Degrees and decimal minutes
        ("40 42.767", "-74 0.36", "NYC - Degrees decimal minutes"),
        ("40° 42.767'", "-74° 0.36'", "NYC - DM with symbols"),
        
        // Compact formats
        ("404246N", "0740022W", "NYC - DDMMSS compact"),
        ("4042.767N", "07400.360W", "NYC - DDMM.mmm aviation format"),
        
        // HMS for longitude
        ("51.5074 N", "0h 7m 39.84s W", "London - HMS longitude"),
        ("40.7128", "4h 56m 27s W", "NYC - HMS longitude equivalent"),
        
        // Edge cases
        ("-00 30 00", "000 00 00", "Negative zero degrees"),
        ("33.8688 S", "151.2093 E", "Sydney - Southern/Eastern hemisphere"),
        
        // International locations
        ("35.6762 N", "139.6503 E", "Tokyo - Decimal with compass"),
        ("51°28'38\"N", "0°0'0\"", "Greenwich Observatory"),
        ("-33°52'08\"", "151°12'30\"", "Sydney Opera House"),
        ("48°51'29.5\"N", "2°17'40.2\"E", "Eiffel Tower"),
    ];
    
    println!("Parsing {} different coordinate formats:\n", examples.len());
    
    for (i, (lat_str, lon_str, description)) in examples.iter().enumerate() {
        match Location::parse(lat_str, lon_str, 0.0) {
            Ok(location) => {
                println!("Example {}: {}", i + 1, description);
                println!("   Input:  '{}', '{}'", lat_str, lon_str);
                println!("   Result: {:.6}°, {:.6}°", location.latitude_deg, location.longitude_deg);
                println!("   DMS:    {}, {}", location.latitude_dms(), location.longitude_dms());
                println!();
            }
            Err(error) => {
                println!("Example {}: {}", i + 1, description);
                println!("   Input:  '{}', '{}'", lat_str, lon_str);
                println!("   Error:  {}", error);
                println!();
            }
        }
    }
    
    println!("Error handling:");
    
    let error_examples = vec![
        ("91.0", "0.0", "Latitude out of range"),
        ("0.0", "181.0", "Longitude out of range"),
        ("40.7128 E", "74.0060 W", "Wrong compass direction for latitude"),
        ("40.7128 N", "74.0060 N", "Wrong compass direction for longitude"),
        ("not a number", "also bad", "Unparseable input"),
    ];
    
    for (lat_str, lon_str, description) in error_examples {
        match Location::parse(lat_str, lon_str, 0.0) {
            Ok(_) => println!("Unexpected success: {}", description),
            Err(error) => {
                println!("Correctly caught error: {}", description);
                println!("   Input:  '{}', '{}'", lat_str, lon_str);
                println!("   Error:  {}", error);
                println!();
            }
        }
    }
}