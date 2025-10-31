use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use click_tracker::core::UserAgentDetector;
use click_tracker::adapters::uaparser::user_agent_detector::UAParserUserAgentDetector;

// Sample user agent strings representing common browsers and devices
const USER_AGENTS: &[(&str, &str)] = &[
    ("Chrome Desktop", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    ("Firefox Desktop", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0"),
    ("Safari Desktop", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15"),
    ("Chrome Mobile", "Mozilla/5.0 (Linux; Android 13; SM-S901B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36"),
    ("Safari Mobile", "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1"),
    ("Edge", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0"),
    ("Bot", "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"),
];

fn get_detector() -> UAParserUserAgentDetector {
    // Use the bundled regexes.yaml from uaparser crate
    let yaml_path = std::env::var("UA_PARSER_YAML")
        .unwrap_or_else(|_| "regexes.yaml".to_string());

    // For testing, we'll create a minimal detector
    // In actual benchmarks, you'd want to use the real detector
    UAParserUserAgentDetector::new(&yaml_path)
}

/// Benchmark the OLD approach: parsing user agent, device, and OS separately (3 calls)
fn bench_triple_parse(c: &mut Criterion) {
    let detector = get_detector();

    let mut group = c.benchmark_group("user_agent_triple_parse");

    for (name, ua_string) in USER_AGENTS.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), ua_string, |b, &ua_str| {
            b.iter(|| {
                // OLD approach: 3 separate parsing calls
                let user_agent = detector.parse_user_agent(black_box(ua_str));
                let os = detector.parse_os(black_box(ua_str));
                let device = detector.parse_device(black_box(ua_str));

                // Return to prevent optimization
                black_box((user_agent, os, device))
            });
        });
    }

    group.finish();
}

/// Benchmark the NEW approach: single parse_client() call
fn bench_single_parse(c: &mut Criterion) {
    let detector = get_detector();

    let mut group = c.benchmark_group("user_agent_single_parse");

    for (name, ua_string) in USER_AGENTS.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(name), ua_string, |b, &ua_str| {
            b.iter(|| {
                // NEW approach: single parse_client() call
                let client = detector.parse_client(black_box(ua_str));
                black_box(client)
            });
        });
    }

    group.finish();
}

/// Compare both approaches directly
fn bench_comparison(c: &mut Criterion) {
    let detector = get_detector();
    let ua_string = USER_AGENTS[0].1; // Use Chrome Desktop as representative

    let mut group = c.benchmark_group("user_agent_comparison");

    group.bench_function("triple_parse", |b| {
        b.iter(|| {
            let user_agent = detector.parse_user_agent(black_box(ua_string));
            let os = detector.parse_os(black_box(ua_string));
            let device = detector.parse_device(black_box(ua_string));
            black_box((user_agent, os, device))
        });
    });

    group.bench_function("single_parse_client", |b| {
        b.iter(|| {
            let client = detector.parse_client(black_box(ua_string));
            black_box(client)
        });
    });

    group.finish();
}

/// Benchmark throughput: how many user agents can be parsed per second
fn bench_throughput(c: &mut Criterion) {
    let detector = get_detector();

    let mut group = c.benchmark_group("user_agent_throughput");
    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("triple_parse_throughput", |b| {
        let mut idx = 0;
        b.iter(|| {
            let ua_string = USER_AGENTS[idx % USER_AGENTS.len()].1;
            idx += 1;

            let user_agent = detector.parse_user_agent(black_box(ua_string));
            let os = detector.parse_os(black_box(ua_string));
            let device = detector.parse_device(black_box(ua_string));
            black_box((user_agent, os, device))
        });
    });

    group.bench_function("single_parse_throughput", |b| {
        let mut idx = 0;
        b.iter(|| {
            let ua_string = USER_AGENTS[idx % USER_AGENTS.len()].1;
            idx += 1;

            let client = detector.parse_client(black_box(ua_string));
            black_box(client)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_triple_parse,
    bench_single_parse,
    bench_comparison,
    bench_throughput
);
criterion_main!(benches);
