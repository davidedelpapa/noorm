use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noorm::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Create Parser Builder", |b| {
        b.iter(|| {
            //greet(black_box("Alice"))
            let config = ParserConfig::new();
            let _parser = Parser::new().set_config(config);
            //parser.parse();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
