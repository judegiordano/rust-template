use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum(num: u64) -> u64 {
    num + 1
}

fn integer_benchmarks(c: &mut Criterion) {
    let num: u64 = 1;
    c.bench_function("sum", |b| b.iter(|| sum(black_box(num))));
}

criterion_group!(benches, integer_benchmarks);
criterion_main!(benches);
