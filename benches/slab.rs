use std::{hint::black_box, rc::Weak};

use criterion::{Criterion, Throughput};
use fast_collections::{Cursor, CursorReadTransmute, GetUnchecked, Slab};
use rand::Rng;

fn bench(c: &mut Criterion) {
    let mut slab: Slab<MyStruct, typenum::U500> = Slab::new();
    let value: u8 = rand::thread_rng().gen();
    let value = value as usize;
    let mut group = c.benchmark_group("pointer");
    group.throughput(Throughput::Bytes(1000));
    {
        group.bench_function("get with relative pointer", |b| {
            b.iter(|| {
                let value = unsafe { slab.get_unchecked_mut(value) };
                black_box(value.0);
            });
        });
    }
    {
        let value = unsafe { slab.get_unchecked_mut(value) };
        group.bench_function("get with static pointer", |b| {
            b.iter(|| {
                black_box(value.0);
            });
        });
    }
}

pub struct MyStruct(usize);

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
