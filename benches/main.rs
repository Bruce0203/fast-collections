use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{Cursor, Push};
use typenum::U1000000;

fn bench(c: &mut Criterion) {
    let mut cur = Cursor::<u8, U1000000>::new();
    c.bench_function("spares vec", |b| {
        b.iter(|| {
            unsafe { cur.push_unchecked(0) };
        });
    });
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
