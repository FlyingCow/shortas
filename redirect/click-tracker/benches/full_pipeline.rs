use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use click_tracker::core::{
    Hit, HitData, Click, HitRoute, TrackingPipeContext, ClickStreamItem,
    UserAgentDetector, Client, Device, OS, UserAgent, Country,
    session::{Session, SessionDetector},
    location::LocationDetector,
};
use chrono::Utc;
use std::net::IpAddr;
use anyhow::Result;

// ============================================================================
// Mock Implementations (No External Dependencies)
// ============================================================================

/// Mock UserAgentDetector that returns hardcoded results
#[derive(Clone, Debug)]
struct MockUserAgentDetector;

impl UserAgentDetector for MockUserAgentDetector {
    fn parse_user_agent(&self, _user_agent: &str) -> UserAgent {
        UserAgent {
            family: "Chrome".to_string(),
            major: Some("120".to_string()),
            minor: Some("0".to_string()),
            patch: Some("0".to_string()),
        }
    }

    fn parse_os(&self, _user_agent: &str) -> OS {
        OS {
            family: "Windows".to_string(),
            major: Some("10".to_string()),
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }

    fn parse_device(&self, _user_agent: &str) -> Device {
        Device {
            family: "Desktop".to_string(),
            brand: None,
            model: None,
        }
    }

    fn parse_client(&self, _user_agent: &str) -> Client {
        Client {
            user_agent: self.parse_user_agent(_user_agent),
            os: self.parse_os(_user_agent),
            device: self.parse_device(_user_agent),
        }
    }
}

/// Mock LocationDetector that returns hardcoded US location
#[derive(Clone, Debug)]
struct MockLocationDetector;

impl LocationDetector for MockLocationDetector {
    fn detect_country(&self, _ip_addr: &IpAddr) -> Option<Country> {
        Some(Country {
            iso_code: "US".to_string(),
        })
    }
}

/// Mock SessionDetector that simulates session tracking
#[derive(Clone, Debug)]
struct MockSessionDetector {
    session_count: u128,
}

impl MockSessionDetector {
    fn new() -> Self {
        Self { session_count: 1 }
    }
}

#[async_trait::async_trait]
impl SessionDetector for MockSessionDetector {
    async fn detect(
        &self,
        _route_id: &str,
        _ip_addr: &IpAddr,
        click_time: &chrono::DateTime<Utc>,
    ) -> Result<Session> {
        Ok(Session {
            first: *click_time,
            count: self.session_count,
        })
    }
}

/// Mock ClickAggsRegistrar that just simulates registration (no-op)
#[derive(Clone, Debug)]
struct MockClickAggsRegistrar;

// ============================================================================
// Test Data Generation
// ============================================================================

fn create_test_hit(id: usize) -> Hit {
    Hit {
        id: format!("01HF8X9YQ7ZQXJ8P4WTXP6J{:03}", id),
        data: HitData::Click(Click {
            dest: Some("https://example.com".to_string()),
        }),
        route: Some(HitRoute {
            id: Some(format!("route_{}", id % 10)),
            creator_id: Some("creator_123".to_string()),
            owner_id: Some("owner_456".to_string()),
            workspace_id: Some("workspace_789".to_string()),
        }),
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()),
        ip: Some("192.168.1.1".parse::<IpAddr>().unwrap()),
        utc: Utc::now(),
    }
}

// ============================================================================
// Pipeline Simulation (without actual modules)
// ============================================================================

/// Simulates the full pipeline processing manually
async fn process_event_through_pipeline(hit: Hit) -> Result<ClickStreamItem> {
    // Create context
    let mut context = TrackingPipeContext::new(hit);

    // Step 1: Init (no-op)
    // Nothing to do

    // Step 2: Enrich User Agent
    if let Some(user_agent_string) = &context.hit.user_agent {
        let detector = MockUserAgentDetector;
        let client = detector.parse_client(user_agent_string);

        // Check for spider before moving device
        if let Some(brand) = &client.device.brand {
            if brand == "Spider" {
                context.spider = true;
            }
        }

        context.client_ua = Some(client.user_agent);
        context.client_os = Some(client.os);
        context.client_device = Some(client.device);
    }

    // Step 3: Enrich Location
    if let Some(ip) = &context.hit.ip {
        let detector = MockLocationDetector;
        if let Some(country) = detector.detect_country(ip) {
            context.client_country = Some(country);
        }
    }

    // Step 4: Enrich Session
    if context.hit.ip.is_some() && context.hit.route.is_some() {
        let ip = context.hit.ip.unwrap();
        let route = context.hit.route.as_ref().unwrap();

        if let Some(route_id) = &route.id {
            let detector = MockSessionDetector::new();
            let session = detector.detect(route_id, &ip, &context.hit.utc).await?;
            context.session = Some(session);
        }
    }

    // Step 5: Aggregate (build ClickStreamItem)
    let mut stream_item = ClickStreamItem {
        id: context.hit.id.clone(),
        created: context.utc,
        ..Default::default()
    };

    if let Some(ip) = context.hit.ip {
        stream_item.ip = Some(ip.to_string());
    }

    if let Some(route) = &context.hit.route {
        stream_item.route_id = route.id.as_ref().map(|s| s.to_owned());
        stream_item.creator_id = route.creator_id.as_ref().map(|s| s.to_owned());
        stream_item.owner_id = route.owner_id.as_ref().map(|s| s.to_owned());
        stream_item.workspace_id = route.workspace_id.as_ref().map(|s| s.to_owned());
    }

    if let HitData::Click(click) = &context.hit.data {
        stream_item.dest = click.dest.as_ref().map(|s| s.to_owned());
    }

    if let Some(user_agent) = context.client_ua.take() {
        stream_item.user_agent_family = Some(user_agent.family);
        stream_item.user_agent_version = user_agent.major;
    }

    if let Some(os) = context.client_os.take() {
        stream_item.os_family = Some(os.family);
        stream_item.os_version = os.major;
    }

    if let Some(device) = context.client_device.take() {
        stream_item.device_brand = device.brand;
        stream_item.device_family = Some(device.family);
        stream_item.device_model = device.model;
        stream_item.is_bot = context.spider;
    }

    if let Some(country) = &context.client_country {
        stream_item.country = Some(country.iso_code.clone());
    }

    if let Some(session) = &context.session {
        stream_item.session_clicks = Some(session.count);
        stream_item.session_first = Some(session.first);
        stream_item.is_unique = session.count == 1;
    }

    Ok(stream_item)
}

// ============================================================================
// Benchmarks
// ============================================================================

/// Benchmark single event processing through entire pipeline
fn bench_single_event(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    c.bench_function("pipeline_single_event", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let hit = create_test_hit(0);
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });
}

/// Benchmark batch event processing
fn bench_batch_events(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("pipeline_batch");

    for batch_size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*batch_size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    runtime.block_on(async {
                        for i in 0..size {
                            let hit = create_test_hit(i);
                            black_box(process_event_through_pipeline(hit).await.unwrap());
                        }
                    })
                });
            },
        );
    }

    group.finish();
}

/// Benchmark throughput - events per second
fn bench_throughput(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("pipeline_throughput");
    group.throughput(Throughput::Elements(1));

    group.bench_function("events_per_second", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let hit = create_test_hit(0);
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });

    group.finish();
}

/// Benchmark each step individually to identify bottlenecks
fn bench_individual_steps(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("pipeline_steps");

    // Step 1: Context creation
    group.bench_function("01_context_creation", |b| {
        b.iter(|| {
            let hit = create_test_hit(0);
            black_box(TrackingPipeContext::new(hit))
        });
    });

    // Step 2: User agent parsing
    group.bench_function("02_user_agent_parsing", |b| {
        b.iter(|| {
            let detector = MockUserAgentDetector;
            let ua_string = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
            black_box(detector.parse_client(ua_string))
        });
    });

    // Step 3: GeoIP lookup
    group.bench_function("03_geo_lookup", |b| {
        b.iter(|| {
            let detector = MockLocationDetector;
            let ip: IpAddr = "192.168.1.1".parse().unwrap();
            black_box(detector.detect_country(&ip))
        });
    });

    // Step 4: Session detection
    group.bench_function("04_session_detection", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let detector = MockSessionDetector::new();
                let ip: IpAddr = "192.168.1.1".parse().unwrap();
                let time = Utc::now();
                black_box(detector.detect("route_123", &ip, &time).await.unwrap())
            })
        });
    });

    // Step 5: ClickStreamItem building
    group.bench_function("05_stream_item_building", |b| {
        let hit = create_test_hit(0);
        let mut context = TrackingPipeContext::new(hit);

        // Populate context
        context.client_ua = Some(UserAgent {
            family: "Chrome".to_string(),
            major: Some("120".to_string()),
            minor: Some("0".to_string()),
            patch: Some("0".to_string()),
        });
        context.client_os = Some(OS {
            family: "Windows".to_string(),
            major: Some("10".to_string()),
            minor: None,
            patch: None,
            patch_minor: None,
        });
        context.client_device = Some(Device {
            family: "Desktop".to_string(),
            brand: None,
            model: None,
        });
        context.client_country = Some(Country {
            iso_code: "US".to_string(),
        });
        context.session = Some(Session {
            first: Utc::now(),
            count: 1,
        });

        b.iter(|| {
            // Simulate building the ClickStreamItem
            let mut stream_item = ClickStreamItem {
                id: context.hit.id.clone(),
                created: context.utc,
                ..Default::default()
            };

            if let Some(route) = &context.hit.route {
                stream_item.route_id = route.id.as_ref().map(|s| s.to_owned());
                stream_item.creator_id = route.creator_id.as_ref().map(|s| s.to_owned());
                stream_item.owner_id = route.owner_id.as_ref().map(|s| s.to_owned());
                stream_item.workspace_id = route.workspace_id.as_ref().map(|s| s.to_owned());
            }

            black_box(stream_item)
        });
    });

    group.finish();
}

/// Benchmark with varying hit patterns
fn bench_different_patterns(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("pipeline_patterns");

    // Pattern 1: All fields populated
    group.bench_function("full_data", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let hit = create_test_hit(0);
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });

    // Pattern 2: Minimal data (no user agent)
    group.bench_function("minimal_data", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut hit = create_test_hit(0);
                hit.user_agent = None;
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });

    // Pattern 3: No IP (skips location and session)
    group.bench_function("no_ip", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut hit = create_test_hit(0);
                hit.ip = None;
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });

    // Pattern 4: No route (skips session)
    group.bench_function("no_route", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut hit = create_test_hit(0);
                hit.route = None;
                black_box(process_event_through_pipeline(hit).await.unwrap())
            })
        });
    });

    group.finish();
}

/// Benchmark memory allocations by processing many events
fn bench_memory_pressure(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let mut group = c.benchmark_group("pipeline_memory");
    group.sample_size(10); // Fewer samples for memory-intensive test

    group.bench_function("10k_events", |b| {
        b.iter(|| {
            runtime.block_on(async {
                for i in 0..10_000 {
                    let hit = create_test_hit(i);
                    black_box(process_event_through_pipeline(hit).await.unwrap());
                }
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_single_event,
    bench_batch_events,
    bench_throughput,
    bench_individual_steps,
    bench_different_patterns,
    bench_memory_pressure
);
criterion_main!(benches);
