use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use click_tracker::core::{ClickStreamItem, HitRoute, Country, Device, OS, UserAgent};
use chrono::Utc;

/// Simulate the OLD approach with excessive cloning
fn build_stream_item_with_cloning(
    hit_id: &str,
    route: &Option<HitRoute>,
    country: &Option<Country>,
    user_agent: &Option<UserAgent>,
    os: &Option<OS>,
    device: &Option<Device>,
) -> ClickStreamItem {
    let mut stream_item = ClickStreamItem {
        id: hit_id.to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    // OLD approach: clone everything
    if let Some(route) = route {
        stream_item.route_id = route.id.clone();
        stream_item.creator_id = route.creator_id.clone();
        stream_item.owner_id = route.owner_id.clone();
        stream_item.workspace_id = route.workspace_id.clone();
    }

    if let Some(country) = country {
        stream_item.country = Some(country.iso_code.clone());
    }

    if let Some(user_agent) = user_agent.clone() {
        stream_item.user_agent_family = Some(user_agent.family);
        stream_item.user_agent_version = user_agent.major;
    }

    if let Some(os) = os.clone() {
        stream_item.os_family = Some(os.family);
        stream_item.os_version = os.major;
    }

    if let Some(device) = device.clone() {
        stream_item.device_brand = device.brand;
        stream_item.device_family = Some(device.family);
        stream_item.device_model = device.model;
    }

    stream_item
}

/// Simulate the NEW optimized approach with minimal cloning
fn build_stream_item_optimized(
    hit_id: &str,
    route: &Option<HitRoute>,
    country: &Option<Country>,
    user_agent: Option<UserAgent>,
    os: Option<OS>,
    device: Option<Device>,
) -> ClickStreamItem {
    let mut stream_item = ClickStreamItem {
        id: hit_id.to_string(),
        created: Utc::now(),
        ..Default::default()
    };

    // NEW approach: use as_ref().map() or take() to avoid unnecessary clones
    if let Some(route) = route {
        stream_item.route_id = route.id.as_ref().map(|s| s.to_owned());
        stream_item.creator_id = route.creator_id.as_ref().map(|s| s.to_owned());
        stream_item.owner_id = route.owner_id.as_ref().map(|s| s.to_owned());
        stream_item.workspace_id = route.workspace_id.as_ref().map(|s| s.to_owned());
    }

    if let Some(country) = country {
        stream_item.country = Some(country.iso_code.clone());
    }

    // Take ownership since we don't need these after
    if let Some(user_agent) = user_agent {
        stream_item.user_agent_family = Some(user_agent.family);
        stream_item.user_agent_version = user_agent.major;
    }

    if let Some(os) = os {
        stream_item.os_family = Some(os.family);
        stream_item.os_version = os.major;
    }

    if let Some(device) = device {
        stream_item.device_brand = device.brand;
        stream_item.device_family = Some(device.family);
        stream_item.device_model = device.model;
    }

    stream_item
}

fn create_test_data() -> (
    String,
    Option<HitRoute>,
    Option<Country>,
    Option<UserAgent>,
    Option<OS>,
    Option<Device>,
) {
    let hit_id = "01HF8X9YQ7ZQXJ8P4WTXP6J8Q2".to_string();

    let route = Some(HitRoute {
        id: Some("route_123".to_string()),
        creator_id: Some("creator_456".to_string()),
        owner_id: Some("owner_789".to_string()),
        workspace_id: Some("workspace_abc".to_string()),
    });

    let country = Some(Country {
        iso_code: "US".to_string(),
    });

    let user_agent = Some(UserAgent {
        family: "Chrome".to_string(),
        major: Some("120".to_string()),
        minor: Some("0".to_string()),
        patch: Some("0".to_string()),
    });

    let os = Some(OS {
        family: "Windows".to_string(),
        major: Some("10".to_string()),
        minor: None,
        patch: None,
        patch_minor: None,
    });

    let device = Some(Device {
        family: "Desktop".to_string(),
        brand: None,
        model: None,
    });

    (hit_id, route, country, user_agent, os, device)
}

fn bench_aggregate_cloning(c: &mut Criterion) {
    let (hit_id, route, country, user_agent, os, device) = create_test_data();

    c.bench_function("aggregate_with_cloning", |b| {
        b.iter(|| {
            build_stream_item_with_cloning(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(&user_agent),
                black_box(&os),
                black_box(&device),
            )
        });
    });
}

fn bench_aggregate_optimized(c: &mut Criterion) {
    c.bench_function("aggregate_optimized", |b| {
        b.iter(|| {
            let (hit_id, route, country, user_agent, os, device) = create_test_data();

            build_stream_item_optimized(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(user_agent),
                black_box(os),
                black_box(device),
            )
        });
    });
}

fn bench_aggregate_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_comparison");

    group.bench_function("with_cloning", |b| {
        let (hit_id, route, country, user_agent, os, device) = create_test_data();
        b.iter(|| {
            build_stream_item_with_cloning(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(&user_agent),
                black_box(&os),
                black_box(&device),
            )
        });
    });

    group.bench_function("optimized", |b| {
        b.iter(|| {
            let (hit_id, route, country, user_agent, os, device) = create_test_data();
            build_stream_item_optimized(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(user_agent),
                black_box(os),
                black_box(device),
            )
        });
    });

    group.finish();
}

fn bench_aggregate_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_throughput");
    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("with_cloning_throughput", |b| {
        b.iter(|| {
            let (hit_id, route, country, user_agent, os, device) = create_test_data();
            build_stream_item_with_cloning(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(&user_agent),
                black_box(&os),
                black_box(&device),
            )
        });
    });

    group.bench_function("optimized_throughput", |b| {
        b.iter(|| {
            let (hit_id, route, country, user_agent, os, device) = create_test_data();
            build_stream_item_optimized(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(user_agent),
                black_box(os),
                black_box(device),
            )
        });
    });

    group.finish();
}

/// Benchmark different data sizes
fn bench_aggregate_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("aggregate_scaling");

    // Test with minimal data
    group.bench_function("minimal_data_cloning", |b| {
        let hit_id = "id".to_string();
        b.iter(|| {
            build_stream_item_with_cloning(
                black_box(&hit_id),
                black_box(&None),
                black_box(&None),
                black_box(&None),
                black_box(&None),
                black_box(&None),
            )
        });
    });

    group.bench_function("minimal_data_optimized", |b| {
        b.iter(|| {
            let hit_id = "id".to_string();
            build_stream_item_optimized(
                black_box(&hit_id),
                black_box(&None),
                black_box(&None),
                black_box(None),
                black_box(None),
                black_box(None),
            )
        });
    });

    // Test with full data
    let (hit_id, route, country, user_agent, os, device) = create_test_data();

    group.bench_function("full_data_cloning", |b| {
        b.iter(|| {
            build_stream_item_with_cloning(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(&user_agent),
                black_box(&os),
                black_box(&device),
            )
        });
    });

    group.bench_function("full_data_optimized", |b| {
        b.iter(|| {
            let (hit_id, route, country, user_agent, os, device) = create_test_data();
            build_stream_item_optimized(
                black_box(&hit_id),
                black_box(&route),
                black_box(&country),
                black_box(user_agent),
                black_box(os),
                black_box(device),
            )
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_aggregate_cloning,
    bench_aggregate_optimized,
    bench_aggregate_comparison,
    bench_aggregate_throughput,
    bench_aggregate_scaling
);
criterion_main!(benches);
