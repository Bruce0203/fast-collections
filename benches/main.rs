use std::hint::black_box;

use criterion::Criterion;
use fast_collections::{Cursor, ReadTransmute};
use typenum::U1000000;

fn bench(c: &mut Criterion) {
    let mut cur = Cursor::<u8, U1000000>::new();
    c.bench_function("push vec", |b| {
        b.iter(|| {
            black_box(unsafe { cur.read_transmute_unchecked::<u8>() });
        });
    });
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
