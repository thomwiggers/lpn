//! Defines the algorithms from the Levieil and Fouque paper (LF1, LF2)
use crate::oracle::{query_bits_range, LpnOracle, Sample, SampleStorage};
use itertools::Itertools;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use packed_simd_2::i64x4;
use rayon::prelude::*;
use std::ops;

use std::mem::size_of;

#[inline]
fn usize_to_binmatrix(c: u64, size: usize) -> BinMatrix {
    const BLOCKSIZE: usize = size_of::<u64>() * 8;
    let c = c.wrapping_shl((BLOCKSIZE - size) as u32).reverse_bits();
    BinMatrix::from_slices(&[&[c]], size)
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
            BinMatrix::from_slices(
                &oracle
                    .samples
                    .into_iter()
                    .map(|q| {
                        c.push(q.get_product());
                        q.into_inner()
                    })
                    .collect::<Vec<SampleStorage>>(),
                b,
            ),
            c,
        )
    };

    // Prepare for computation
    let transposed_a = a_matrix.transposed();
    let c = c.as_matrix();
    drop(a_matrix);

    // LF1 query weight computation
    let computation = |candidate: usize| {
        // A u32 is 4 u8s.
        let candidate_vector = usize_to_binmatrix(candidate as u64, b);

        let mut matrix_vector_product: BinMatrix = &candidate_vector * &transposed_a;
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
    usize_to_binmatrix(best_candidate as u64, b).as_vector()
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
    let bitrange: ops::Range<usize> = (k - b)..k;
    println!("Sorting");
    oracle
        .samples
        .par_sort_unstable_by_key(|q| query_bits_range(q, bitrange.clone()));

    // split into partitions
    println!("Finding partitioning points");
    let oracle_start = oracle.samples.as_ptr();
    let maxj = 2u64.pow(b as u32);
    let pivots = (0..maxj).into_par_iter().map(|item| {
        oracle.samples.partition_point(|q| item <= query_bits_range(q, bitrange.clone()))
    }).collect::<Vec<_>>();
 
    println!("creating slices");
    let mut samples = &mut oracle.samples[..];
    let mut collected = 0;
    let mut partitions = Vec::with_capacity(pivots.len());
    for pivot in pivots {
        let (left, right) = samples.split_at_mut(pivot - collected);
        samples = right;
        collected += pivot - collected;
        partitions.push(left)
    }
        
    println!("xor-reducing");
    let (delete_ranges, extra_stuff): (Vec<_>, Vec<Vec<Sample>>) = partitions
        .par_iter_mut()
        .fold(
            || (Vec::<&mut [Sample]>::new(), Vec::<Vec<Sample>>::new()),
            |(mut deletes, mut extra), partition| {
                let mut result = partition
                    .iter()
                    .tuple_combinations()
                    .map(|(v1, v2)| {
                        let mut vnew = v1.clone();
                        vnew.xor_into(v2);
                        vnew
                    })
                    .collect::<Vec<_>>(); // going to be hard to avoid this allocate / collect.
                if result.len() > partition.len() {
                    partition.copy_from_slice(&result[result.len() - partition.len()..]);
                    let mut taken = partition.len();
                    // plug any holes left by previous iterations
                    while taken < result.len() {
                        if let Some(fillable) = deletes.pop() {
                            let copy_to = result.len() - taken;
                            if fillable.len() > result.len() - taken {
                                let copy_from = copy_to - fillable.len();
                                let (fillable_here, fillable_later) =
                                    fillable.split_at_mut(result.len());
                                fillable_here.copy_from_slice(&result[copy_from..copy_to]);
                                deletes.push(fillable_later);
                                break;
                            } else {
                                let copy_from = copy_to - fillable.len();
                                fillable.copy_from_slice(&result[copy_from..copy_to]);
                                taken += fillable.len();
                            }
                        } else {
                            result.truncate(taken);
                            result.shrink_to_fit();
                            extra.push(result);
                            break;
                        }
                    }
                } else if result.len() < partition.len() {
                    partition[..result.len()].copy_from_slice(&result[..]);
                    let (_, deleteme) = partition.split_at_mut(result.len());
                    deletes.push(deleteme);
                }
                (deletes, extra)
            },
        )
        .reduce(
            || (Vec::new(), Vec::new()),
            |mut a: (Vec<&mut [Sample]>, Vec<Vec<Sample>>), b: (Vec<_>, Vec<Vec<Sample>>)| {
                (a.0).extend(b.0);
                (a.1).extend(b.1);
                a
            },
        );

    let delete_ranges = delete_ranges
        .into_iter()
        .rev()
        .map(|old_partition| {
            let old_partition = old_partition.as_ptr_range();
            let startpos = (old_partition.start as *const _ as usize - oracle_start as usize)
                / std::mem::size_of::<Sample>();
            let endpos = (old_partition.end as *const _ as usize - oracle_start as usize)
                / std::mem::size_of::<Sample>();
            startpos..endpos
        })
        .collect::<Vec<_>>();
    delete_ranges
        .into_iter()
        .for_each(|range| oracle.samples.drain(range).for_each(drop));
    let num_extra_samples = extra_stuff.iter().fold(0, |acc, v| acc + v.len());
    oracle.samples.reserve_exact(num_extra_samples);
    oracle.samples.extend(extra_stuff.into_iter().flatten());

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

    let mut majority_counter = oracle
        .samples
        .into_par_iter()
        .fold(
            || vec![0; 2usize.pow(k)],
            |mut counters, sample| {
                let idx = sample.get_block(0) as usize;
                counters[idx] += if sample.get_product() { -1 } else { 1 };
                counters
            },
        )
        .reduce(
            || vec![0; 2usize.pow(k)],
            |mut a, b| {
                a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
                a
            },
        );

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
            data.par_chunks_mut(2 * stride).for_each(|data| {
                let (left, right) = data.split_at_mut(stride);
                (0..stride).step_by(4).into_iter().for_each(|j| unsafe {
                    let l = i64x4::from_slice_unaligned_unchecked(&left[j..j + 4]);
                    let r = i64x4::from_slice_unaligned_unchecked(&right[j..j + 4]);
                    (l + r).write_to_slice_unaligned_unchecked(&mut left[j..j + 4]);
                    (l - r).write_to_slice_unaligned_unchecked(&mut right[j..j + 4]);
                });
            });
        } else {
            data.par_chunks_mut(2 * stride).for_each(|data| {
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

    fn usize_to_binvec(c: u64, size: usize) -> BinVector {
        let bytes = c.to_be_bytes();
        let skip = (64 - size) / 8;
        let mut binvec = BinVector::from_bytes(&bytes[skip..]);
        let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
        debug_assert_eq!(result.len(), size);
        result
    }

    #[test]
    fn usize_to_binmatrix_correct() {
        for _ in 0..1000 {
            let mut c: u64 = rand::random();
            let len = rand::thread_rng().gen_range(1..=64);
            if len < 64 {
                c = c & ((1 << len) - 1);
            }
            assert_eq!(
                usize_to_binvec(c, len),
                usize_to_binmatrix(c, len).as_vector(),
                "c = {:b}",
                c
            );
            assert_eq!(
                usize_to_binvec(c, len).as_matrix(),
                usize_to_binmatrix(c, len)
            );
        }
    }

    #[test]
    fn transmute_usize_to_u8s() {
        assert_eq!(
            usize_to_binvec(2, 4),
            BinVector::from_bools(&[false, false, true, false])
        );

        let a = 0x0000_0000_0000_0001u64;
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
