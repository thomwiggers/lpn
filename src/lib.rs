#![feature(partition_point)]
#![feature(maybe_uninit_slice)]
//! This library provides everything you need to program attacks on LPN
//! as if you were writing them on paper.
#[cfg(feature = "jemallocator")]
extern crate jemallocator;

#[cfg_attr(feature = "jemallocator", global_allocator)]
#[cfg(feature = "jemallocator")]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

extern crate binomial_iter;
extern crate fnv;
extern crate itertools;
#[cfg(test)]
extern crate lazy_static;
extern crate m4ri_rust;
extern crate rand;
extern crate rayon;

#[cfg_attr(feature = "codes", macro_use)]
extern crate serde;

pub mod bkw;
#[cfg(feature = "codes")]
pub mod covering_codes;
pub mod gauss;
pub mod lf1;
pub mod oracle;

#[cfg(feature = "codes")]
pub mod codes;

mod util;