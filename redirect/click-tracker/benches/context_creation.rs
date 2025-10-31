use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use click_tracker::core::{Hit, HitData, Click, TrackingPipeContext};
use chrono::Utc;
use std::net::IpAddr;
use std::collections::HashMap;

/// Simulate OLD approach: always allocate HashMap
fn create_context_with_hashmap(hit: Hit) -> TrackingPipeContextOld {
    TrackingPipeContextOld {
        id: ulid::Ulid::new().to_string(),
        utc: Utc::now(),
        hit,
        data: HashMap::new(), // Always allocate
        spider: false,
    }
}

/// Simulate the actual OLD TrackingPipeContext structure
struct TrackingPipeContextOld {
    id: String,
    utc: chrono::DateTime<Utc>,
    hit: Hit,
    data: HashMap<&'static str, bool>, // Simplified for benchmark
    spider: bool,
}

fn create_test_hit() -> Hit {
    Hit {
        id: ulid::Ulid::new().to_string(),
        data: HitData::Click(Click {
            dest: Some("https://example.com".to_string()),
        }),
        route: None,
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()),
        ip: Some("192.168.1.1".parse::<IpAddr>().unwrap()),
        utc: Utc::now(),
    }
}

/// Benchmark OLD approach: always allocate HashMap
fn bench_context_with_hashmap(c: &mut Criterion) {
    c.bench_function("context_with_hashmap", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            create_context_with_hashmap(black_box(hit))
        });
    });
}

/// Benchmark NEW approach: lazy HashMap allocation
fn bench_context_lazy_hashmap(c: &mut Criterion) {
    c.bench_function("context_lazy_hashmap", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            TrackingPipeContext::new(black_box(hit))
        });
    });
}

/// Compare both approaches
fn bench_context_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_comparison");

    group.bench_function("with_hashmap_allocation", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            create_context_with_hashmap(black_box(hit))
        });
    });

    group.bench_function("lazy_hashmap", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            TrackingPipeContext::new(black_box(hit))
        });
    });

    group.finish();
}

/// Benchmark throughput: contexts created per second
fn bench_context_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_throughput");
    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("with_hashmap_throughput", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            create_context_with_hashmap(black_box(hit))
        });
    });

    group.bench_function("lazy_hashmap_throughput", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            TrackingPipeContext::new(black_box(hit))
        });
    });

    group.finish();
}

/// Benchmark batch creation (simulating real workload)
fn bench_context_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_batch");

    for batch_size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("with_hashmap", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let contexts: Vec<_> = (0..size)
                        .map(|_| {
                            let hit = create_test_hit();
                            create_context_with_hashmap(hit)
                        })
                        .collect();
                    black_box(contexts)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("lazy_hashmap", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let contexts: Vec<_> = (0..size)
                        .map(|_| {
                            let hit = create_test_hit();
                            TrackingPipeContext::new(hit)
                        })
                        .collect();
                    black_box(contexts)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory footprint (indirectly via allocation count)
fn bench_context_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_memory");

    // Simulate creating many contexts (as in high-throughput scenario)
    group.bench_function("with_hashmap_memory_pressure", |b| {
        b.iter(|| {
            let contexts: Vec<_> = (0..1000)
                .map(|_| {
                    let hit = create_test_hit();
                    create_context_with_hashmap(hit)
                })
                .collect();
            black_box(contexts)
        });
    });

    group.bench_function("lazy_hashmap_memory_pressure", |b| {
        b.iter(|| {
            let contexts: Vec<_> = (0..1000)
                .map(|_| {
                    let hit = create_test_hit();
                    TrackingPipeContext::new(hit)
                })
                .collect();
            black_box(contexts)
        });
    });

    group.finish();
}

/// Test accessing data (to ensure lazy init doesn't hurt when actually needed)
fn bench_context_data_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("context_data_access");

    // Test adding data with old approach
    group.bench_function("with_hashmap_add_data", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            let mut context = create_context_with_hashmap(hit);
            context.data.insert("test_key", true);
            black_box(context)
        });
    });

    // Test adding data with new lazy approach
    group.bench_function("lazy_hashmap_add_data", |b| {
        b.iter(|| {
            let hit = create_test_hit();
            let mut context = TrackingPipeContext::new(hit);
            context.add_bool("test_key", true);
            black_box(context)
        });
    });

    // Test reading data
    group.bench_function("with_hashmap_read_data", |b| {
        let hit = create_test_hit();
        let mut context = create_context_with_hashmap(hit);
        context.data.insert("test_key", true);

        b.iter(|| {
            let value = context.data.get("test_key");
            black_box(value)
        });
    });

    group.bench_function("lazy_hashmap_read_data", |b| {
        let hit = create_test_hit();
        let mut context = TrackingPipeContext::new(hit);
        context.add_bool("test_key", true);

        b.iter(|| {
            let value = context.is_data_true("test_key");
            black_box(value)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_context_with_hashmap,
    bench_context_lazy_hashmap,
    bench_context_comparison,
    bench_context_throughput,
    bench_context_batch,
    bench_context_memory,
    bench_context_data_access
);
criterion_main!(benches);
