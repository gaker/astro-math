use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use astro_math::{*, nutation::*};
use chrono::{Utc, TimeZone};
use std::collections::HashMap;

/// Benchmark coordinate transformation functions
fn bench_coordinate_transforms(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_transforms");
    
    let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    // Single coordinate transformation
    group.bench_function("ra_dec_to_alt_az_single", |b| {
        b.iter(|| {
            ra_dec_to_alt_az(black_box(279.23), black_box(38.78), datetime, &location)
        })
    });
    
    // ERFA-based transformation  
    group.bench_function("ra_dec_to_alt_az_erfa_single", |b| {
        b.iter(|| {
            ra_dec_to_alt_az_erfa(black_box(279.23), black_box(38.78), datetime, &location, None, None, None)
        })
    });
    
    // Batch processing - test scalability
    for size in [10, 100, 1000, 10000].iter() {
        let coords: Vec<(f64, f64)> = (0..*size)
            .map(|i| ((i as f64 * 360.0 / *size as f64), (i as f64 * 180.0 / *size as f64) - 90.0))
            .collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(BenchmarkId::new("batch_parallel", size), &coords, |b, coords| {
            b.iter(|| {
                ra_dec_to_alt_az_batch_parallel(black_box(coords), datetime, &location, None, None, None)
            })
        });
    }
    
    group.finish();
}

/// Benchmark string parsing performance (location.rs bottleneck)
fn bench_location_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("location_parsing");
    
    // Test various parsing complexity levels
    let test_cases = HashMap::from([
        ("decimal_simple", "40.7128"),
        ("decimal_with_direction", "40.7128 N"),
        ("dms_spaced", "40 42 46"),
        ("dms_symbols", "40°42'46\""),
        ("dms_complex", "40 degrees 42 minutes 46.08 seconds North"),
        ("hms", "4h 56m 27s"),
        ("compact_aviation", "4042.767N"),
        ("fuzzy_mixed", "40d 42' 46.08\" N"),
    ]);
    
    for (name, input) in test_cases.iter() {
        group.bench_function(*name, |b| {
            b.iter(|| {
                Location::parse(black_box(input), "0 0 0", 0.0)
            })
        });
    }
    
    // Batch parsing performance  
    let coordinates = vec![
        "40.7128", "41.8781", "34.0522", "39.9526", "29.7604",
        "40°42'46\"", "41°52'37\"", "34°03'08\"", "39°57'09\"", "29°45'38\"",
        "4h 56m 27s", "5h 35m 12s", "11h 40m 20s", "8h 00m 00s", "6h 18m 35s",
    ];
    
    group.throughput(Throughput::Elements(coordinates.len() as u64));
    group.bench_function("batch_mixed_formats", |b| {
        b.iter(|| {
            for coord in &coordinates {
                let _ = Location::parse(black_box(coord), "0 0 0", 0.0);
            }
        })
    });
    
    group.finish();
}

/// Benchmark time calculations  
fn bench_time_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_calculations");
    
    let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    group.bench_function("julian_date", |b| {
        b.iter(|| {
            julian_date(black_box(datetime))
        })
    });
    
    group.bench_function("j2000_days", |b| {
        b.iter(|| {
            j2000_days(black_box(datetime))
        })
    });
    
    // Test time scale conversions
    let jd = julian_date(datetime);
    
    group.bench_function("utc_to_tt_jd", |b| {
        b.iter(|| {
            utc_to_tt_jd(black_box(jd))
        })
    });
    
    // Local sidereal time computation
    let location = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    group.bench_function("local_sidereal_time", |b| {
        b.iter(|| {
            location.local_sidereal_time(black_box(datetime))
        })
    });
    
    group.finish();
}

/// Benchmark nutation calculations
fn bench_nutation(c: &mut Criterion) {
    let mut group = c.benchmark_group("nutation");
    
    let jd = 2451545.0; // J2000.0
    
    group.bench_function("nutation_in_longitude", |b| {
        b.iter(|| {
            nutation_in_longitude(black_box(jd))
        })
    });
    
    group.bench_function("nutation_in_obliquity", |b| {
        b.iter(|| {
            nutation_in_obliquity(black_box(jd))
        })
    });
    
    group.bench_function("nutation_combined", |b| {
        b.iter(|| {
            nutation(black_box(jd))
        })
    });
    
    group.bench_function("mean_obliquity", |b| {
        b.iter(|| {
            mean_obliquity(black_box(jd))
        })
    });
    
    // Test over time range (performance over different epochs)
    let jd_range: Vec<f64> = (0..100).map(|i| jd + i as f64 * 365.25).collect();
    
    group.throughput(Throughput::Elements(jd_range.len() as u64));
    group.bench_function("nutation_time_series", |b| {
        b.iter(|| {
            for jd in &jd_range {
                let _ = nutation(black_box(*jd));
            }
        })
    });
    
    group.finish();
}

/// Benchmark mathematical operations that could benefit from SIMD
fn bench_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");
    
    // Test coordinate transformations that could be vectorized
    let coords: Vec<(f64, f64)> = (0..1000)
        .map(|i| ((i as f64 * 360.0 / 1000.0), (i as f64 * 180.0 / 1000.0) - 90.0))
        .collect();
    
    let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    // Sequential processing (current implementation)
    group.throughput(Throughput::Elements(coords.len() as u64));
    group.bench_function("sequential_transforms", |b| {
        b.iter(|| {
            let results: Vec<_> = coords.iter().map(|(ra, dec)| {
                ra_dec_to_alt_az(*ra, *dec, datetime, &location).unwrap()
            }).collect();
            black_box(results)
        })
    });
    
    // Parallel processing (using Rayon)
    group.bench_function("parallel_transforms", |b| {
        b.iter(|| {
            ra_dec_to_alt_az_batch_parallel(black_box(&coords), datetime, &location, None, None, None)
        })
    });
    
    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Test Vec allocations in string parsing
    let complex_coordinate = "40 degrees 42 minutes 46.08 seconds North";
    
    group.bench_function("string_parsing_allocations", |b| {
        b.iter(|| {
            // This will internally create multiple Vec<> allocations
            Location::parse(black_box(complex_coordinate), "0 0 0", 0.0)
        })
    });
    
    // Test repeated calculations without reallocation
    let coords: Vec<(f64, f64)> = vec![(279.23, 38.78); 100];
    let datetime = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let location = Location {
        latitude_deg: 40.0,
        longitude_deg: -74.0,
        altitude_m: 0.0,
    };
    
    group.bench_function("repeated_transforms_with_alloc", |b| {
        b.iter(|| {
            // This creates new Vec on each iteration
            let results: Vec<_> = coords.iter().map(|(ra, dec)| {
                ra_dec_to_alt_az(*ra, *dec, datetime, &location).unwrap()
            }).collect();
            black_box(results)
        })
    });
    
    group.bench_function("repeated_transforms_preallocated", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(coords.len());
            for (ra, dec) in &coords {
                results.push(ra_dec_to_alt_az(*ra, *dec, datetime, &location).unwrap());
            }
            black_box(results)
        })
    });
    
    group.finish();
}

/// Benchmark regex compilation performance in location parsing  
fn bench_regex_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_patterns");
    
    let test_inputs = vec![
        "4h 56m 27s",
        "40°42'46\"N", 
        "123 45 67.89",
        "40d42m46s",
        "4 hours 56 minutes 27 seconds",
    ];
    
    // Test regex compilation overhead
    group.bench_function("regex_heavy_parsing", |b| {
        b.iter(|| {
            for input in &test_inputs {
                let _ = Location::parse(black_box(input), "0 0 0", 0.0);
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_coordinate_transforms,
    bench_location_parsing,
    bench_time_calculations,
    bench_nutation,
    bench_vector_operations,
    bench_memory_patterns,
    bench_regex_patterns,
);

criterion_main!(benches);