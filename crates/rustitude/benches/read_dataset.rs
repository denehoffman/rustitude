use criterion::{criterion_group, criterion_main, Criterion};
use rustitude::prelude::*;

pub fn criterion_read_dataset(c: &mut Criterion) {
    c.bench_function("read_dataset", |b| {
        b.iter(|| criterion::black_box(Dataset::from_parquet("benches/test_data.parquet")))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = criterion_read_dataset
}
criterion_main!(benches);
