///! Try to find codes that work, using the
///! assumptions we know from the Griesmer bounds

extern crate factorial;
extern crate num_traits;
extern crate num_rational;
extern crate num_bigint;

use num_bigint::{BigInt, ToBigInt, ToBigUint};
use num_rational::{BigRational, Ratio};
use num_traits::{ToPrimitive, Pow, Zero};
use factorial::Factorial;

const K: usize = 512;

macro_rules! one {
    () => { Ratio::from_integer(1u32.to_bigint().unwrap()) }
}

macro_rules! two {
    () => { Ratio::from_integer(2u32.to_bigint().unwrap()) }
}

macro_rules! half {
    () => { Ratio::new(1u32.to_bigint().unwrap(), 2u32.to_bigint().unwrap()) }
}
macro_rules! delta {
    () => { Ratio::new(1u32.to_bigint().unwrap(), 8u32.to_bigint().unwrap()) }
}

fn fact(n: usize) -> BigInt {
    n.to_biguint().unwrap().factorial().to_bigint().unwrap()
}

fn choose(n: usize, k: usize) -> BigInt {
    debug_assert!(n > k);
    let num = fact(n);
    let denom = fact(k) * fact(n - k);
    num / denom
}

fn to_float(r: BigRational) -> f64 {
    let (num, denom) = r.into();
    num.to_f64().unwrap() / denom.to_f64().unwrap()
}

/// We possibly need something more perecise.
fn powf(r: BigRational, exp: f64) -> BigRational {
    let float = to_float(r);
    debug_assert_ne!(float, 0f64);
    BigRational::from_float(float.powf(exp)).unwrap()
}

fn bc(k_prime: usize) -> BigRational {
    let delta_part = powf(half!() + half!()*delta!(), (K as f64)/(k_prime as f64));
    (two!() * delta_part - one!()) / delta!()
}

fn quasi_perfect_code_bias(k_prime: usize, d: usize) -> BigRational {
    let r = (d - 1) / 2;
    two!().pow((k_prime as i32) - (K as i32))
        * (0..=r).into_iter().fold(BigRational::zero(), |acc, w| {
            acc + BigRational::from_integer(choose(K, w))
                * (delta!().pow(w) - delta!().pow(r + 1))
                * delta!().pow(r + 1)
    })
}

fn perfect_code_bias(k_prime: usize, d: usize) -> BigRational {
    let r = (d - 1) / 2;
    two!().pow((k_prime as i32) - (K as i32))
        * (0..=r).into_iter().fold(BigRational::zero(), |acc, w| {
            acc + BigRational::from_integer(choose(K, w)) * delta!().pow(w)
        })
}

#[allow(unused)]
fn minimal_d_perfect(k_prime: usize) -> usize {
    let bc_to_beat = bc(k_prime);
    for i in 1..K {
        if bc_to_beat <= perfect_code_bias(k_prime, i) {
            return i;
        }
    }
    panic!("Haven't found bias");
}

#[allow(unused)]
fn minimal_d_quasi_perfect(k_prime: usize) -> usize {
    let bc_to_beat = bc(k_prime);
    for i in 1..K {
        if bc_to_beat <= quasi_perfect_code_bias(k_prime, i) {
            return i;
        }
    }
    panic!("Haven't found bias");
}

#[allow(unused)]
fn griesmer_bound(k_prime: usize) -> usize {
    for d in (1..K).rev() {
        if K >= (0..(k_prime-1)).into_iter().fold(0, |acc, i| {
            BigRational::new(d.into(), 2u32.to_bigint().unwrap().pow(i)).ceil().to_integer().to_usize().unwrap()
        }) {
            return d;
        }
    }
    panic!("Haven't found Griesmer bound!");
}

fn main() {
    println!("hi");
}

mod test {
    use super::*;

    #[test]
    fn test_bc() {
        for i in 0..K {
            assert!(bc(i) != BigRational::from_integer(0u32.into()), "bc == 0 for i={}", i);
        }
        assert_eq!(bc(512), BigRational::from_integer(1u32.into()));
    }

    #[test]
    fn test_bounds_work() {
        for i in 1..K {
            griesmer_bound(i);
            minimal_d_quasi_perfect(i);
            minimal_d_perfect(i);
        }
    }
}
