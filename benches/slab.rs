use std::{hint::black_box, rc::Weak};

use criterion::{Criterion, Throughput};
use fast_collections::{Cursor, CursorReadTransmute, GetUnchecked, Slab};
use rand::Rng;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("pointer");
    group.throughput(Throughput::Bytes(1000));
    {
        let mut slice: [u8; 100] = [rand::thread_rng().gen(); 100];
        let index: usize = rand::thread_rng().gen::<u8>() as usize;
        group.bench_function("get with relative pointer", |b| {
            b.iter(|| {
                black_box(*unsafe { slice.get_unchecked_mut(index) });
            });
        });
    }
    {
        let mut value = rand::thread_rng().gen::<u8>();
        let value: &mut u8 = black_box(&mut value);
        group.bench_function("get with static pointer", |b| {
            b.iter(|| {
                black_box(*value);
            });
        });
    }
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
