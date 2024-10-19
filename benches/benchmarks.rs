use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noorm::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Create Parser Builder", |b| {
        b.iter(|| {
            //greet(black_box("Alice"))
            let config = ParserConfig::new();
            let mut parser = Parser::new()
                .set_config(config)
                .statement("CREATE TABLE person ( Id INTEGER NOT NULL, name VARCHAR(255) )");
            _ = parser.parse();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
