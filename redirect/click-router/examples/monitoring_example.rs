use click_router::{
    core::{
        metrics::{FlowRouterMetrics, METRICS},
        metrics_endpoint::create_metrics_router,
    },
    adapters::{RequestType, ResponseType},
};
use salvo::{prelude::*, Server};
use std::time::Duration;
use tokio::time::sleep;

/// Example demonstrating how to use the monitoring capabilities
#[tokio::main]
async fn main() {
    println!("🔍 Click Router Monitoring Example");
    println!("==================================");
    println!();
    
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();
    
    println!("📊 Available Monitoring Endpoints:");
    println!("   • GET /metrics       - Prometheus metrics (text format)");
    println!("   • GET /health        - Health check with basic metrics");
    println!("   • GET /metrics/info  - Detailed metrics information (JSON)");
    println!();
    
    // Create metrics router
    let metrics_router = create_metrics_router();
    
    // Start metrics server in background
    let metrics_service = Service::new(metrics_router);
    let metrics_server = Server::new(TcpListener::new("127.0.0.1:9090").bind().await);
    
    println!("🚀 Starting metrics server on http://127.0.0.1:9090");
    println!("   Try these URLs:");
    println!("   • http://127.0.0.1:9090/health");
    println!("   • http://127.0.0.1:9090/metrics/info");
    println!("   • http://127.0.0.1:9090/metrics");
    println!();
    
    // Start server in background
    tokio::spawn(async move {
        metrics_server.serve(metrics_service).await;
    });
    
    // Give server time to start
    sleep(Duration::from_millis(100)).await;
    
    println!("📈 Demonstrating metrics collection...");
    
    // Simulate some metrics
    demonstrate_metrics().await;
    
    println!();
    println!("✅ Monitoring demonstration complete!");
    println!("   The metrics server is running at http://127.0.0.1:9090");
    println!("   Press Ctrl+C to stop the server");
    
    // Keep the server running
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

async fn demonstrate_metrics() {
    println!("   Simulating request processing...");
    
    // Simulate various metrics
    for i in 1..=10 {
        // Simulate request processing
        METRICS.requests_total.inc();
        METRICS.active_requests.inc();
        
        // Simulate cache operations
        if i % 3 == 0 {
            METRICS.route_cache_hits.inc();
        } else {
            METRICS.route_cache_misses.inc();
            METRICS.route_db_queries.inc();
        }
        
        // Simulate user settings cache
        if i % 4 == 0 {
            METRICS.user_settings_cache_hits.inc();
        } else {
            METRICS.user_settings_cache_misses.inc();
        }
        
        // Simulate flow processing (optimized)
        METRICS.iterative_flow_usage.inc();
        
        // Simulate timing
        let processing_time = 0.001 + (i as f64 * 0.0005); // Increasing processing time
        METRICS.request_duration.observe(processing_time);
        METRICS.flow_processing_duration.observe(processing_time * 0.3); // Flow is 30% of total
        
        if i % 3 != 0 {
            METRICS.db_query_duration.observe(processing_time * 0.4); // DB is 40% when cache miss
        }
        
        METRICS.cache_lookup_duration.observe(0.0001); // Cache lookup is very fast
        
        // Simulate memory allocations
        let allocations = 10.0 + (i as f64 * 2.0);
        METRICS.memory_allocations_per_request.observe(allocations);
        
        // Simulate hit registration
        if i % 2 == 0 {
            METRICS.hits_registered.inc();
        }
        
        // Mark request as successful
        METRICS.requests_success.inc();
        METRICS.active_requests.dec();
        
        // Small delay to simulate processing
        sleep(Duration::from_millis(10)).await;
        
        if i % 3 == 0 {
            println!("     Processed {} requests...", i);
        }
    }
    
    // Simulate one error
    METRICS.requests_total.inc();
    METRICS.requests_error.inc();
    
    println!("   ✅ Simulated 10 successful requests + 1 error");
    
    // Display current metrics
    display_current_metrics();
}

fn display_current_metrics() {
    println!();
    println!("📊 Current Metrics Summary:");
    println!("   Total Requests:     {}", METRICS.requests_total.get());
    println!("   Successful:         {}", METRICS.requests_success.get());
    println!("   Errors:             {}", METRICS.requests_error.get());
    println!("   Active:             {}", METRICS.active_requests.get());
    println!();
    
    let cache_hits = METRICS.route_cache_hits.get();
    let cache_misses = METRICS.route_cache_misses.get();
    let cache_total = cache_hits + cache_misses;
    let hit_rate = if cache_total > 0 {
        (cache_hits as f64 / cache_total as f64) * 100.0
    } else {
        0.0
    };
    
    println!("🎯 Cache Performance:");
    println!("   Route Cache Hits:   {}", cache_hits);
    println!("   Route Cache Misses: {}", cache_misses);
    println!("   Hit Rate:           {:.1}%", hit_rate);
    println!();
    
    println!("⚡ Optimization Impact:");
    println!("   Iterative Flow:     {}", METRICS.iterative_flow_usage.get());
    println!("   Recursive Flow:     {}", METRICS.recursive_flow_usage.get());
    println!("   DB Queries:         {}", METRICS.route_db_queries.get());
    println!("   Hits Registered:    {}", METRICS.hits_registered.get());
    println!();
    
    println!("⏱️  Timing Metrics:");
    println!("   Request Samples:    {}", METRICS.request_duration.get_sample_count());
    println!("   Flow Samples:       {}", METRICS.flow_processing_duration.get_sample_count());
    println!("   DB Query Samples:   {}", METRICS.db_query_duration.get_sample_count());
    println!("   Cache Samples:      {}", METRICS.cache_lookup_duration.get_sample_count());
}

/// Example of how to integrate metrics into your main application
pub fn integrate_metrics_into_main_app() -> Router {
    // Your main application routes
    let app_router = Router::new()
        .push(Router::with_path("/").get(hello_handler))
        .push(Router::with_path("/redirect/{path}").get(redirect_handler));
    
    // Metrics routes (typically on a separate port or path)
    let metrics_router = create_metrics_router();
    
    // Combine them or serve on different ports
    Router::new()
        .push(Router::with_path("/api").push(app_router))
        .push(Router::with_path("/monitoring").push(metrics_router))
}

#[handler]
async fn hello_handler() -> &'static str {
    "Hello from Click Router!"
}

#[handler]
async fn redirect_handler() -> &'static str {
    // This would normally use the FlowRouter
    "Redirect logic would go here"
}

/// Example Prometheus configuration for scraping metrics
pub fn example_prometheus_config() -> &'static str {
    r#"
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'click-router'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: '/metrics'
    scrape_interval: 5s
    
rule_files:
  - "click_router_alerts.yml"

# Example alerting rules (click_router_alerts.yml)
groups:
  - name: click_router
    rules:
      - alert: HighErrorRate
        expr: rate(flow_router_requests_error_total[5m]) / rate(flow_router_requests_total[5m]) > 0.05
        for: 2m
        labels:
          severity: warning
        annotations:
          summary: "High error rate in click router"
          
      - alert: LowCacheHitRate
        expr: rate(flow_router_route_cache_hits_total[5m]) / (rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) < 0.8
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Low cache hit rate"
          
      - alert: HighLatency
        expr: histogram_quantile(0.95, rate(flow_router_request_duration_seconds_bucket[5m])) > 0.1
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "High request latency"
"#
}

/// Example Grafana dashboard queries
pub fn example_grafana_queries() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Request Rate", "rate(flow_router_requests_total[5m])"),
        ("Error Rate", "rate(flow_router_requests_error_total[5m])"),
        ("Success Rate %", "rate(flow_router_requests_success_total[5m]) / rate(flow_router_requests_total[5m]) * 100"),
        ("Cache Hit Rate %", "rate(flow_router_route_cache_hits_total[5m]) / (rate(flow_router_route_cache_hits_total[5m]) + rate(flow_router_route_cache_misses_total[5m])) * 100"),
        ("P95 Latency", "histogram_quantile(0.95, rate(flow_router_request_duration_seconds_bucket[5m]))"),
        ("P99 Latency", "histogram_quantile(0.99, rate(flow_router_request_duration_seconds_bucket[5m]))"),
        ("Active Requests", "flow_router_active_requests"),
        ("Optimization Usage %", "rate(flow_router_iterative_flow_usage_total[5m]) / (rate(flow_router_iterative_flow_usage_total[5m]) + rate(flow_router_recursive_flow_usage_total[5m])) * 100"),
        ("Memory Allocations", "rate(flow_router_memory_allocations_per_request_sum[5m]) / rate(flow_router_memory_allocations_per_request_count[5m])"),
    ]
}
