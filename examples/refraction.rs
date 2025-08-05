use astro_math::{
    refraction_bennett, refraction_saemundsson, refraction_radio,
    apparent_to_true_altitude, true_to_apparent_altitude
};

fn main() {
    println!("=== Atmospheric Refraction Example ===\n");

    // Example 1: Refraction at various altitudes
    println!("Refraction vs Altitude (Standard Conditions):");
    println!("Altitude | Bennett | Saemundsson | Difference");
    println!("---------|---------|-------------|------------");
    
    for alt in [0.0, 5.0, 10.0, 20.0, 30.0, 45.0, 60.0, 90.0] {
        let r_bennett = refraction_bennett(alt).unwrap();
        let r_saem = refraction_saemundsson(alt, 1013.25, 10.0).unwrap();
        println!("{:7.1}° | {:7.3}' | {:11.3}' | {:10.3}'",
            alt, 
            r_bennett * 60.0, 
            r_saem * 60.0,
            (r_bennett - r_saem).abs() * 60.0
        );
    }

    // Example 2: Effect of atmospheric conditions
    println!("\nEffect of Atmospheric Conditions (10° altitude):");
    println!("Conditions                    | Refraction");
    println!("------------------------------|------------");
    
    let conditions = [
        ("Standard (1013 hPa, 10°C)", 1013.25, 10.0),
        ("High pressure (1030 hPa)", 1030.0, 10.0),
        ("Low pressure (980 hPa)", 980.0, 10.0),
        ("Cold (-20°C)", 1013.25, -20.0),
        ("Hot (40°C)", 1013.25, 40.0),
        ("Mt. Everest (350 hPa, -40°C)", 350.0, -40.0),
    ];
    
    for (desc, pressure, temp) in conditions {
        let r = refraction_saemundsson(10.0, pressure, temp).unwrap();
        println!("{:<29} | {:7.3}' ({:+.3}')", 
            desc, 
            r * 60.0,
            (r - refraction_saemundsson(10.0, 1013.25, 10.0).unwrap()) * 60.0
        );
    }

    // Example 3: Radio vs optical refraction
    println!("\nRadio vs Optical Refraction (20°C, 1013.25 hPa):");
    println!("Altitude | Humidity | Radio    | Optical  | Difference");
    println!("---------|----------|----------|----------|------------");
    
    for alt in [1.0, 5.0, 10.0, 30.0] {
        for humidity in [0.0, 50.0, 100.0] {
            let r_radio = refraction_radio(alt, 1013.25, 20.0, humidity).unwrap();
            let r_optical = refraction_saemundsson(alt, 1013.25, 20.0).unwrap();
            println!("{:7.1}° | {:7.0}% | {:7.3}' | {:7.3}' | {:+9.3}\"",
                alt, humidity, 
                r_radio * 60.0, 
                r_optical * 60.0,
                (r_radio - r_optical) * 3600.0
            );
        }
    }

    // Example 4: True vs apparent altitude
    println!("\nTrue vs Apparent Altitude Conversion:");
    let true_altitudes = [0.0, 5.0, 10.0, 30.0, 45.0];
    
    println!("True Alt | Apparent | Refraction | Back to True | Error");
    println!("---------|----------|------------|--------------|--------");
    
    for true_alt in true_altitudes {
        let apparent = true_to_apparent_altitude(true_alt, 1013.25, 10.0).unwrap();
        let refr = apparent - true_alt;
        let back_to_true = apparent_to_true_altitude(apparent, 1013.25, 10.0).unwrap();
        let error = (back_to_true - true_alt).abs();
        
        println!("{:7.3}° | {:7.3}° | {:9.3}' | {:11.3}° | {:.1e}°",
            true_alt, apparent, refr * 60.0, back_to_true, error);
    }

    // Example 5: Practical observation scenario
    println!("\nPractical Example - Observing object at true altitude 2°:");
    let true_alt = 2.0;
    let apparent = true_to_apparent_altitude(true_alt, 1013.25, 15.0).unwrap();
    let refr = refraction_saemundsson(apparent, 1013.25, 15.0).unwrap();
    
    println!("  True altitude:     {:.3}°", true_alt);
    println!("  Apparent altitude: {:.3}°", apparent);
    println!("  Refraction:        {:.1}' ({:.3}°)", refr * 60.0, refr);
    println!("  Object appears {:.1}' higher than it really is", refr * 60.0);
}