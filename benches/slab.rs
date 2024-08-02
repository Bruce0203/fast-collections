use std::hint::black_box;

use divan::{bench, Bencher};
use fast_collections::Vec;
use rand::Rng;

#[bench]
fn benchmark(bencher: Bencher) {
    let mut vec: Vec<usize, 100> = Vec::uninit();
    black_box(vec.push(123).unwrap());
    bencher.bench_local(|| {
        black_box(unsafe { vec.swap_remove_unchecked(0) });
    });
}

fn main() {
    divan::main();
}
