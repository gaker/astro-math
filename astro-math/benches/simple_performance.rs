use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use astro_math::{Location, ra_dec_to_alt_az, ra_dec_to_alt_az_batch_parallel};
use chrono::{Utc, TimeZone};

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
    
    // Batch processing performance
    for size in [10, 100, 1000].iter() {
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

/// Benchmark location parsing performance (location.rs optimizations)
fn bench_location_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("location_parsing");
    
    // Test various parsing formats to show the optimization benefits
    let test_cases = [
        ("decimal_simple", "40.7128"),
        ("decimal_with_direction", "40.7128N"),
        ("dms_basic", "40 42 46"),
        ("dms_symbols", "40° 42' 46\""),
        ("dms_with_direction", "40° 42' 46\" N"),
        ("hms_format", "4h 56m 27s"),
        ("complex_unicode", "40°42′46.08″N"),
    ];
    
    for (name, input) in test_cases.iter() {
        group.bench_function(*name, |b| {
            b.iter(|| {
                let _ = Location::parse(black_box(input), "0.0", 0.0);
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_coordinate_transforms, bench_location_parsing);
criterion_main!(benches);