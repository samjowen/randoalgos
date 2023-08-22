use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maths::mean;

fn mean_benchmark(c: &mut Criterion) {
    c.bench_function("Mean, Large List (1x10^6 elements)", |b| {
        b.iter(|| mean(black_box(&vec![1.0; 1000000])))
    });

    c.bench_function("Mean, Small List (10 elements)", |b| {
        b.iter(|| mean(black_box(&vec![1.0; 10])))
    });

    c.bench_function("Mean, Empty List", |b| {
        b.iter(|| mean(black_box(&Vec::new())))
    });
}

criterion_group!(benches, mean_benchmark);
criterion_main!(benches);
