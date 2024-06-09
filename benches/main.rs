use std::hint::black_box;

use criterion::Criterion;

fn bench(c: &mut Criterion) {
    c.bench_function("spares vec", |b| {
        b.iter(|| {
            black_box(1 + 1);
        });
    });
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
