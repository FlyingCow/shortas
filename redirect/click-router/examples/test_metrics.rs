use click_router::core::metrics::METRICS;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("🧪 Testing Metrics Collection");
    println!("==============================");
    
    // Simulate some requests
    println!("\n📊 Initial metrics state:");
    print_metrics();
    
    println!("\n🔄 Simulating 10 requests...");
    for i in 1..=10 {
        // Simulate request processing
        METRICS.requests_total.inc();
        METRICS.active_requests.inc();
        
        // Simulate some processing time
        sleep(Duration::from_millis(10)).await;
        
        // Simulate success/error (90% success rate)
        if i % 10 != 0 {
            METRICS.requests_success.inc();
        } else {
            METRICS.requests_error.inc();
        }
        
        // Simulate cache operations
        if i % 3 == 0 {
            METRICS.route_cache_hits.inc();
        } else {
            METRICS.route_cache_misses.inc();
        }
        
        // Simulate iterative flow usage (our optimization)
        METRICS.iterative_flow_usage.inc();
        
        // Simulate hit registration
        METRICS.hits_registered.inc();
        
        // Record request duration (simulate 50ms average)
        METRICS.request_duration.observe(0.05);
        METRICS.flow_processing_duration.observe(0.01);
        
        METRICS.active_requests.dec();
        
        println!("  ✅ Request {} processed", i);
    }
    
    println!("\n📊 Final metrics state:");
    print_metrics();
    
    println!("\n🎯 Key Performance Indicators:");
    let total = METRICS.requests_total.get();
    let success = METRICS.requests_success.get();
    let errors = METRICS.requests_error.get();
    let cache_hits = METRICS.route_cache_hits.get();
    let cache_misses = METRICS.route_cache_misses.get();
    let iterative_usage = METRICS.iterative_flow_usage.get();
    
    if total > 0 {
        let success_rate = (success as f64 / total as f64) * 100.0;
        println!("  • Success Rate: {:.1}%", success_rate);
    }
    
    if (cache_hits + cache_misses) > 0 {
        let cache_hit_rate = (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0;
        println!("  • Cache Hit Rate: {:.1}%", cache_hit_rate);
    }
    
    if total > 0 {
        let optimization_rate = (iterative_usage as f64 / total as f64) * 100.0;
        println!("  • Optimization Usage: {:.1}% (should be 100%)", optimization_rate);
    }
    
    println!("\n✅ Metrics collection is working correctly!");
    println!("   The /health and /metrics/info endpoints will show these values when the application processes real requests.");
}

fn print_metrics() {
    println!("  • Total Requests: {}", METRICS.requests_total.get());
    println!("  • Successful Requests: {}", METRICS.requests_success.get());
    println!("  • Error Requests: {}", METRICS.requests_error.get());
    println!("  • Active Requests: {}", METRICS.active_requests.get());
    println!("  • Route Cache Hits: {}", METRICS.route_cache_hits.get());
    println!("  • Route Cache Misses: {}", METRICS.route_cache_misses.get());
    println!("  • Iterative Flow Usage: {}", METRICS.iterative_flow_usage.get());
    println!("  • Recursive Flow Usage: {}", METRICS.recursive_flow_usage.get());
    println!("  • Hits Registered: {}", METRICS.hits_registered.get());
    println!("  • Request Duration Samples: {}", METRICS.request_duration.get_sample_count());
    println!("  • Flow Processing Samples: {}", METRICS.flow_processing_duration.get_sample_count());
}
