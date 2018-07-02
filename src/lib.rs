//#![feature(nll)]

extern crate fnv;
extern crate itertools;
extern crate m4ri_rust;
extern crate rand;
extern crate rayon;
#[cfg(test)]
#[macro_use] extern crate lazy_static;
extern crate binomial_iter;

pub mod bkw;
pub mod covering_codes;
pub mod gauss;
pub mod lf1;
pub mod oracle;

pub mod codes;
