#![feature(test)]
extern crate test;

use lpn::oracle::*;
use test::Bencher;

const LARGE_K: u32 = (MAX_K - 10) as u32;

#[bench]
fn bench_get_samples(b: &mut Bencher) {
    let oracle = LpnOracle::new(LARGE_K, 1.0 / 8.0);
    let count = 1_000_000;
    b.bytes = count as u64;
    b.iter(|| oracle.clone().get_samples_drop(count, 6))
}
