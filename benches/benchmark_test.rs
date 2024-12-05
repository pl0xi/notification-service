use criterion::{criterion_group, criterion_main, Criterion};

#[allow(unused_variables)]
fn benchmarks(c: &mut Criterion) {}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
