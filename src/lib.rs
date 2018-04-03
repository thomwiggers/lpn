#![cfg_attr(all(i128, feature="unstable"), feature(i128))]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate num_traits;
extern crate bit;


pub mod bitvector;
