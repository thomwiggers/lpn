//! Defines the algorithms from the Levieil and Fouque paper (LF1, LF2)
use crate::{
    bkw::{create_partitions, create_pivots},
    oracle::{are_last_bits_zero, query_bits_range, LpnOracle, Sample, SampleStorage},
    util::log_2,
};
use itertools::Itertools;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use packed_simd_2::i64x4;
use rayon::prelude::*;
use std::{
    ops,
    sync::atomic::{AtomicI64, Ordering},
};
use unchecked_unwrap::UncheckedUnwrap;

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

    log::trace!("Doing LF1 naively");
    let max = 2usize.pow(b as u32);
    // find the candidate with the best weight
    let best_candidate = (1..max)
        .into_par_iter()
        .max_by_key(|candidate| computation(*candidate))
        .expect("Can't work on an empty list");

    log::info!("Best candidate weight: {}", best_candidate.count_ones());
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
    xor_drop_reduce(oracle, b, 0)
}

fn fill_delete_ranges(deletes: &mut Vec<&mut [Sample]>, extras: &mut Vec<Sample>) {
    while deletes.len() > 0 && extras.len() > 0 {
        let fillable = unsafe { deletes.pop().unchecked_unwrap() };
        let fillable_len = fillable.len();
        if fillable_len <= extras.len() {
            let offset = extras.len() - fillable_len;
            debug_assert_eq!(extras[offset..].len(), fillable_len);
            let src = &extras[offset..];
            for (dst, src) in fillable.iter_mut().zip_eq(src) {
                *dst = src.clone();
            }
            extras.truncate(offset);
        } else {
            let (fillable, remainder) = fillable.split_at_mut(extras.len());
            debug_assert_eq!(fillable.len(), extras.len());
            for (dst, src) in fillable.iter_mut().zip_eq(extras.iter()) {
                *dst = src.clone();
            }
            if deletes.capacity() < 10 {
                deletes.reserve_exact(1000);
            }
            extras.truncate(extras.len() - fillable.len());
            deletes.push(remainder);
        }
    }
}

pub fn xor_drop_reduce(oracle: &mut LpnOracle, b: u32, zero_bits: usize) {
    let k = oracle.get_k();
    let b = b as usize;
    assert!(b < k);

    let num_samples = oracle.samples.len();

    let expected_samples =
        (num_samples * (num_samples - 1)) / 2usize.pow((b + 1 + zero_bits) as u32);

    log::info!(
        "xor-reduce iteration, b={}, {} samples (log2: {}), expect to obtain {} (log2: {})",
        b,
        num_samples,
        log_2(num_samples),
        expected_samples,
        log_2(expected_samples)
    );
    // Partition into V_j
    let bitrange: ops::Range<usize> = (k - b)..k;
    oracle
        .samples
        .par_sort_unstable_by_key(|q| query_bits_range(q, bitrange.clone()));

    let dup_count = (&oracle.samples[1..])
        .iter()
        .fold((&oracle.samples[0], 0usize), |(prev, count), s| {
            if prev == s {
                (s, count + 1)
            } else {
                (s, count)
            }
        })
        .1;
    if dup_count > 0 {
        log::warn!("There are {} duplicate samples", dup_count);
    }

    // split into partitions
    log::debug!("Creating partitions");

    let pivots = create_pivots(&mut oracle.samples, &bitrange);
    let partitions = create_partitions(&mut oracle.samples, &pivots);

    log::debug!("xor-reducing");
    let (mut delete_ranges, mut extra_stuff): (Vec<_>, Vec<Sample>) = partitions
        .into_par_iter()
        .fold(
            || {
                (
                    Vec::<&mut [Sample]>::new(), // Ranges that should be filled or emptied out
                    Vec::<Sample>::new(),        // Additional samples
                )
            },
            |(mut deletes, mut extras), partition: &mut [Sample]| {
                let new_samples = partition
                    .iter()
                    .tuple_combinations()
                    .map(|(v1, v2)| {
                        debug_assert_eq!(
                            query_bits_range(v1, bitrange.clone()),
                            query_bits_range(v2, bitrange.clone())
                        );
                        let mut vnew = v1.clone();
                        debug_assert_eq!(&vnew, v1);
                        vnew.xor_into(v2);
                        debug_assert_ne!(&vnew, v1);
                        debug_assert_ne!(&vnew, v2);
                        debug_assert_eq!(
                            v1.as_binvector(k) + v2.as_binvector(k),
                            vnew.as_binvector(k)
                        );
                        vnew
                    })
                    .collect::<Vec<_>>();
                extras.reserve_exact(new_samples.len());
                if zero_bits != 0 {
                    extras.extend(
                        new_samples
                            .into_iter()
                            .filter(|x| are_last_bits_zero(x, k, zero_bits)),
                    );
                } else {
                    extras.extend(new_samples);
                };

                if deletes.capacity() < 10 {
                    deletes.reserve_exact(1000);
                }
                deletes.push(partition);

                fill_delete_ranges(&mut deletes, &mut extras);

                (deletes, extras)
            },
        )
        .reduce(
            || (Vec::new(), Vec::new()),
            |(mut a_delete, mut a_extra): (Vec<&mut [Sample]>, Vec<Sample>),
             (mut b_delete, mut b_extra): (Vec<_>, Vec<Sample>)| {
                fill_delete_ranges(&mut a_delete, &mut b_extra);
                fill_delete_ranges(&mut b_delete, &mut a_extra);

                let mut a_delete = if a_delete.len() > b_delete.len() {
                    a_delete.reserve_exact(b_delete.len() + 100);
                    a_delete.extend(b_delete);
                    a_delete
                } else {
                    b_delete.reserve_exact(a_delete.len() + 100);
                    b_delete.extend(a_delete);
                    b_delete
                };
                let mut a_extra = if a_extra.len() > b_extra.len() {
                    a_extra.reserve_exact(b_extra.len() + 100);
                    a_extra.extend(b_extra);
                    a_extra
                } else {
                    b_extra.reserve_exact(a_extra.len() + 100);
                    b_extra.extend(a_extra);
                    b_extra
                };

                fill_delete_ranges(&mut a_delete, &mut a_extra);

                (a_delete, a_extra)
            },
        );
    log::trace!("Done with xor-reducing, moving into cleanup");

    fill_delete_ranges(&mut delete_ranges, &mut extra_stuff);

    let num_extra_samples = extra_stuff.len();
    let delete_count = delete_ranges.iter().map(|v| v.len()).sum::<usize>();
    log::debug!(
        "deleting {} samples before extending with {} extra samples",
        delete_count,
        num_extra_samples
    );

    // sort samples to clear out remainder
    if delete_count > 0 {
        delete_ranges.into_par_iter().for_each(|deletable_samples| {
            for sample in deletable_samples {
                for block in sample.get_sample_mut() {
                    *block = !0;
                }
            }
        });

        oracle.samples.par_sort_unstable();
        debug_assert_eq!(oracle.samples.last().unwrap().get_sample()[0], !0);

        oracle.samples.truncate(oracle.samples.len() - delete_count);
    }

    log::trace!(
        "extending with {} newly generated samples",
        num_extra_samples
    );
    oracle.samples.reserve_exact(num_extra_samples);
    oracle.samples.extend(extra_stuff);
    oracle.samples.shrink_to_fit();

    debug_assert!(!oracle
        .samples
        .iter()
        .any(|q| query_bits_range(q, bitrange.clone()) != 0));

    // Set the new k
    log::trace!("truncating oracle");
    unsafe { oracle.set_k(k - b) };
    oracle.secret.truncate(k - b, true);
    oracle.delta = oracle.delta.powi(2);
    log::debug!(
        "xor-reduce iteration done, {} samples (2^{}) now, k' = {}",
        oracle.samples.len(),
        log_2(oracle.samples.len()),
        oracle.get_k()
    );
}

/// Solving using the Fast Walsh-Hamadard Transform
///
/// This section of code is based on the implementation of
/// LPN by Tramer (Bogos, Tramer, Vaudenay 2015)
pub fn fwht_solve(oracle: LpnOracle) -> BinVector {
    log::info!("FWHT solving for k' = {}", oracle.get_k());
    assert!(oracle.get_k() < crate::util::num_bits::<usize>());

    let k = oracle.get_k() as u32;
    let mut majority_counter = count_samples(oracle);

    log::debug!("FWHT");
    parfwht(&mut majority_counter[..], k);

    let guess = (0..2usize.pow(k))
        .max_by_key(|x| majority_counter[*x])
        .unwrap();

    let mut result = BinVector::with_capacity(k as usize);
    for i in 0..k {
        result.push(guess >> i & 1 == 1);
    }
    result
}

#[cfg(target_arch = "x86_64")]
fn count_samples(oracle: LpnOracle) -> Vec<i64> {
    let k = oracle.get_k() as u32;

    let mut sum_vector = Vec::new();
    sum_vector.resize_with(2usize.pow(k), || AtomicI64::new(0));

    oracle
        .samples
        .into_par_iter()
        .for_each_with(&sum_vector[..], |counters, sample| {
            let idx = sample.get_block(0) as usize;
            counters[idx].fetch_add(if sample.get_product() { -1 } else { 1 }, Ordering::Relaxed);
        });
    sum_vector
        .into_iter()
        .map(|i| i.into_inner())
        .collect::<Vec<_>>()
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
