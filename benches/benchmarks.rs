use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noorm::greet;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("greet Alice", |b| {
        b.iter(|| greet(black_box("Alice")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
