#![feature(test)]
extern crate test;

use lpn::{bkw::partition_reduce, lf1::xor_reduce, oracle::{LpnOracle, MAX_K}};
use test::Bencher;

const LARGE_K: u32 = (MAX_K - 10) as u32;

#[bench]
fn bench_oracle_right_size_k(b: &mut Bencher) {
    let oracle = LpnOracle::new(LARGE_K, 1.0 / 8.0);

    b.iter(|| oracle.clone().get_samples(100_000))
}

#[bench]
fn bench_oracle_undersized_k(b: &mut Bencher) {
    let oracle = LpnOracle::new(10 as u32, 1.0 / 8.0);

    b.iter(|| oracle.clone().get_samples(100_000))
}

#[bench]
fn bench_partition_reduce(b: &mut Bencher) {
    let mut oracle = LpnOracle::new(LARGE_K, 1.0 / 8.0);
    oracle.get_samples(100_000);

    b.iter(|| partition_reduce(&mut (oracle.clone()), 22));
}
#[bench]
fn bench_xor_reduce(b: &mut Bencher) {
    let mut oracle = LpnOracle::new(LARGE_K, 1.0 / 8.0);
    oracle.get_samples(10_000);

    b.iter(|| xor_reduce(&mut (oracle.clone()), 8));
}
