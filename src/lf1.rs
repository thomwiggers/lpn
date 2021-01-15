//! Defines the algorithms from the Levieil and Fouque paper (LF1, LF2)
use crate::oracle::{query_bits_range, LpnOracle};
use itertools::Itertools;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use rayon::prelude::*;
use std::ops;

use std::mem::{transmute, size_of};

use packed_simd_2::i64x4;

#[inline]
fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { transmute::<usize, [u8; size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

/// Recover the secret through the FWHT
///
/// Use the vector-matrix product version of the fwht
pub fn lf1_solve(oracle: LpnOracle) -> BinVector {
    // get the (a, c) samples as matrix A and vector c
    let n_prime = oracle.samples.len();
    assert!(n_prime > 0, "What, no samples?");

    let b = oracle.get_k();
    assert!(b < 21, "Don't use too large b! b = {}", b);
    assert!(b > 0, "Wtf, b = 0?");

    // split the query set into a matrix and a vector
    let (a_matrix, c) = {
        let mut c = BinVector::with_capacity(n_prime);
        (
            BinMatrix::new(
                oracle
                    .samples
                    .into_iter()
                    .map(|q| {
                        c.push(q.get_product());
                        q.as_binvector(b)
                    })
                    .collect(),
            ),
            c,
        )
    };

    // LF1 query weight computation
    let computation = |candidate: usize| {
        // A u32 is 4 u8s.
        let candidate_vector = usize_to_binvec(candidate, b);

        let mut matrix_vector_product: BinVector = &a_matrix * &candidate_vector;
        matrix_vector_product += &c;
        let hw = matrix_vector_product.count_ones();
        n_prime as i32 - 2 * (hw as i32)
    };

    println!("Doing LF1 naively");
    let max = 2usize.pow(b as u32);
    // find the candidate with the best weight
    let best_candidate = (1..max)
        .into_par_iter()
        .max_by_key(|candidate| computation(*candidate))
        .expect("Can't work on an empty list");

    println!("Best candidate weight: {}", best_candidate.count_ones());
    usize_to_binvec(best_candidate, b)
}

/// This is the LF2 reduction. This reduction grows the number of samples.
///
/// Applies a single round reduction
///
/// $k' = k - (a-1)*b$
/// $n' = n(n-1) / 2^{b+1}$  (for a = 1)
/// $\delta' = \delta^2$
pub fn xor_reduce(oracle: &mut LpnOracle, b: u32) {
    let k = oracle.get_k();
    let b = b as usize;
    assert!(b <= k);

    let num_samples = oracle.samples.len();
    println!("xor-reduce iteration, {} samples", num_samples);
    // Partition into V_j
    // max j:
    let maxj = 2usize.pow(b as u32);

    // using [v; n] clones and won't set capacity correctly.
    let mut vector_partitions = Vec::with_capacity(maxj);
    // num / 2^b choose 2
    let query_capacity = ((num_samples / maxj) * (num_samples / maxj - 1)) / 2;
    for _ in 0..maxj {
        vector_partitions.push(Vec::with_capacity(query_capacity));
    }

    let bitrange: ops::Range<usize> = (k - b)..k;
    for q in oracle.samples.drain(..) {
        let idx = query_bits_range(&q, bitrange.clone()) as usize;
        if vector_partitions[idx].capacity() == 0 {
            println!(
                "Vector {} is full, will need to resize from {}",
                idx,
                vector_partitions[idx].len()
            );
        }
        vector_partitions[idx].push(q);
    }

    vector_partitions.par_iter_mut().for_each(|partition| {
        *partition = partition
            .iter()
            .tuple_combinations()
            .map(|(v1, v2)| { 
                let mut vnew = v1.clone();
                vnew.xor_into(v2);
                vnew
            })
            .collect();
    });

    oracle
        .samples
        .reserve(vector_partitions.iter().fold(0, |acc, x| acc + x.len()));
    for partition in vector_partitions {
        oracle.samples.extend(partition.into_iter());
    }

    // Set the new k
    oracle.truncate(k - b);
    oracle.delta = oracle.delta.powi(2);

    println!(
        "Xor-reduce iteration done, {} samples now, k' = {}",
        oracle.samples.len(),
        oracle.get_k()
    );
}

/// Solving using the Fast Walsh-Hamadard Transform
///
/// This section of code is based on the implementation of
/// LPN by Tramer (Bogos, Tramer, Vaudenay 2015)
pub fn fwht_solve(oracle: LpnOracle) -> BinVector {
    println!("FWHT solving...");
    let k = oracle.get_k() as u32;

    let mut majority_counter = vec![0; 2usize.pow(k)];
    oracle.samples.into_iter().for_each(|q| {
        majority_counter[q.get_block(0) as usize] += if q.get_product() { -1 } else { 1 };
    });

    println!("FWHT");
    parfwht(majority_counter.as_mut_slice(), k);

    let guess = (0..2usize.pow(k))
        .max_by_key(|x| majority_counter[*x])
        .unwrap();

    let mut result = BinVector::with_capacity(k as usize);
    for i in 0..k {
        result.push(guess >> i & 1 == 1);
    }
    result
}

/// Fast Walsh Hamadard Transform
///
/// Adapted from Bogos, Tramer, Vaudenay,
/// who used http://www.musicdsp.org/showone.php?id=18
///
/// Data: data to transform the transform over (whoooah)
/// bits: log2(size), the length of the data
#[inline]
#[allow(clippy::many_single_char_names)]
pub fn fwht(data: &mut [i64], bits: u32) {
    let n = bits;
    for i in 0..n {
        let mut j = 0;
        while j < (1 << n) {
            let mut k = 0;
            while k < (1 << i) {
                let a = j + k;
                let b = j + k + (1 << i);

                let tmp = data[a];
                data[a] += data[b];
                data[b] = tmp - data[b];

                k += 1;
            }
            j += 1 << (i + 1);
        }
    }
}

#[inline]
pub fn parfwht(data: &mut [i64], bits: u32) {
    let n = 1 << bits;
    assert!(data.len() == n);

    let mut stride = n / 2;
    // cycle through stages with different butterfly strides
    while stride >= 1 {
        // cycle through subvectors for (2 * stride) elements
        if stride >= 4 {
            data.chunks_mut(2*stride)
                .for_each(|data| {
                let (left, right) = data.split_at_mut(stride);
                (0..stride).step_by(4).into_iter().for_each(|j| {
                    unsafe {
                        let l = i64x4::from_slice_unaligned_unchecked(&left[j..j+4]);
                        let r = i64x4::from_slice_unaligned_unchecked(&right[j..j+4]);
                        (l+r).write_to_slice_unaligned_unchecked(&mut left[j..j+4]);
                        (l-r).write_to_slice_unaligned_unchecked(&mut right[j..j+4]);
                    }
                });
            });
        } else {
            data.chunks_mut(2*stride)
                .for_each(|data| {
                let (left, right) = data.split_at_mut(stride);
                (0..stride).into_iter().for_each(|j| {
                    let l = left[j];
                    let r = right[j];
                    left[j] = l + r;
                    right[j] = l - r;
                });
            });
        }
        stride >>= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn transmute_usize_to_u8s() {
        assert_eq!(
            usize_to_binvec(2, 4),
            BinVector::from_bools(&[false, false, true, false])
        );

        let a = 0x0000_0000_0000_0001usize;
        let binvec = usize_to_binvec(a, 50);
        for i in 0..49 {
            assert_eq!(binvec.get(i), Some(false), "bit {} isn't 0", i);
        }
        assert_eq!(binvec.get(49), Some(true));
    }

    #[test]
    fn test_fwht() {
        let bits = 16;
        let mut majority_counter = vec![0; 2usize.pow(bits)];
        let rng = &mut rand::thread_rng();
        majority_counter.iter_mut().for_each(|el| {
            *el = (rng).gen::<i64>() % 2i64.pow(16);
        });

        let mut majority_1 = majority_counter.clone();
        let mut majority_2 = majority_counter;
        fwht(&mut majority_1, bits);
        parfwht(&mut majority_2, bits);

        assert_eq!(majority_1, majority_2, "Should be the same");
    }
}
