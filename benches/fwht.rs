#![feature(test)]

extern crate test;

use rand::prelude::*;

use test::Bencher;

use lpn::lf1::{fwht, parfwht};

const BITS: u32 = 10;

#[bench]
fn fwht_speed(b: &mut Bencher) {
    let mut majority_counter = vec![0; 2usize.pow(BITS)];
    let rng = &mut rand::thread_rng();
    majority_counter.iter_mut().for_each(|el| {
        *el = (rng).gen::<i64>() % 2i64.pow(16);
    });

    b.iter(|| {
        fwht(&mut majority_counter.clone(), BITS);
    });
}

#[bench]
fn parfwht_speed(b: &mut Bencher) {
    let mut majority_counter = vec![0; 2usize.pow(BITS)];
    let rng = &mut rand::thread_rng();
    majority_counter.iter_mut().for_each(|el| {
        *el = (rng).gen::<i64>() % 2i64.pow(16);
    });

    b.iter(|| parfwht(&mut majority_counter.clone(), BITS));
}
