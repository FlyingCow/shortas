use criterion::criterion_main;

// Note: flow_router benchmark requires DynamoDB/MongoDB to be running
// Uncomment the line below and run with a configured database if needed
// mod flow_router;

// Empty criterion_main for now - flow_router benchmark disabled
// To use flow_router, uncomment it above and uncomment the line below:
// criterion_main!(flow_router::benches);

// Placeholder empty benchmark group
use criterion::Criterion;
fn empty_bench(_c: &mut Criterion) {}
criterion::criterion_group!(benches, empty_bench);
criterion_main!(benches);