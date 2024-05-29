use criterion::criterion_main;

mod flow_router;

criterion_main!(
    flow_router::benches
    // batch::benches,
    // event::benches,
    // files::benches,
    // http::benches,
    // lua::benches,
    // metrics_snapshot::benches,
    // template::benches,
);