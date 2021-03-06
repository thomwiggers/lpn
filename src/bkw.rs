//! Defines the algorithms from the classic Blum, Kalai and Wasserman paper
use crate::oracle::*;
use fnv::FnvHashMap;
use m4ri_rust::friendly::BinVector;
use std::{default::Default, num::NonZeroUsize, ops};

use rayon::iter::{Chain, FilterMap, Once, RepeatN, Zip};
use rayon::prelude::*;
use unchecked_unwrap::UncheckedUnwrap;

/// The full BKW solving algorithm.
///
/// Does `a-1` applications of [`partition-reduce`] with `$b$` and solves via majority.
///
/// $k' = k - (a-1) * b$
/// $n' = n - (a-1)*2^b
/// $d' = delta^{2*(a-1)}$
pub fn bkw(mut oracle: LpnOracle, a: u32, b: u32) -> BinVector {
    bkw_reduce(&mut oracle, a, b);
    majority(oracle)
}

pub(crate) fn create_pivots(
    oracle_samples: &mut [Sample],
    bitrange: &std::ops::Range<usize>,
) -> Vec<usize> {
    let maxj = 2usize.pow(bitrange.len() as u32);

    oracle_samples.sort_unstable_by_key(|s| query_bits_range(s, bitrange.clone()));

    // create iterator to zip
    // preallocate because collect() will try to reserve()
    // this leads to power-of-two allocs.
    let mut result = Vec::with_capacity(maxj);
    (0..maxj)
        .into_par_iter()
        .map(|item| {
            oracle_samples.partition_point(|q| query_bits_range(q, bitrange.clone()) <= item as u64)
        })
        .collect_into_vec(&mut result);
    result.shrink_to_fit();
    result
}

/// XXX get rid of this allow?
type Slicer<'data> = fn(((usize, usize), &'data [Sample])) -> Option<&'data mut [Sample]>;
type PartitionIterator<'data, 'pivots> = FilterMap<
    Zip<
        Zip<
            Chain<Once<usize>, rayon::iter::Copied<rayon::slice::Iter<'pivots, usize>>>,
            Chain<rayon::iter::Copied<rayon::slice::Iter<'pivots, usize>>, Once<usize>>,
        >,
        RepeatN<&'data [Sample]>,
    >,
    Slicer<'data>,
>;
#[allow(mutable_transmutes)]
pub(crate) fn create_partitions<'data, 'pivots>(
    oracle_samples: &'data mut [Sample],
    pivots: &'pivots Vec<usize>,
) -> PartitionIterator<'data, 'pivots> {
    let num_samples = oracle_samples.len();
    let num_pivots = pivots.len();
    let lefts = rayon::iter::once(0).chain(pivots.par_iter().copied());
    let rights = pivots
        .into_par_iter()
        .copied()
        .chain(rayon::iter::once(num_samples));
    let oracle_samples = &*oracle_samples;
    let refs = rayon::iter::repeatn(oracle_samples, num_pivots + 1);
    let func: Slicer = |((l, r), oracle_samples)| unsafe {
        if l == r {
            None
        } else {
            debug_assert!(l < r, "{} >= {}", l, r);
            let slice =
                std::mem::transmute::<&'data [Sample], &'data mut [Sample]>(&oracle_samples[l..r]);
            Some(slice)
        }
    };
    let partitions = lefts.zip(rights).zip(refs).filter_map(func);

    debug_assert_eq!(
        num_samples,
        partitions.clone().map(|s| s.len()).sum::<usize>()
    );

    partitions
}

/// Reduces the LPN problem size using the reduction from Blum, Kalai and Wasserman.
pub fn partition_reduce(oracle: &mut LpnOracle, b: u32) {
    bkw_reduce(oracle, 2, b);
}

fn bkw_reduce_inplace(oracle: &mut LpnOracle, i: usize, b: usize) {
    let num_samples = oracle.samples.len();
    let k = oracle.get_k() as usize;

    let maxj = 2usize.pow(b as u32);
    // max j:
    println!(
        "BKW iteration, {} samples left, expecting to remove {} through indexing method",
        num_samples, maxj
    );

    let mut firsts_idxs: Vec<Option<NonZeroUsize>> = vec![None; maxj];

    let bitrange: ops::Range<usize> = (k - (b * i))..(k - (b * (i - 1)));
    // first collect "firsts" so we can do the later part in parallel
    for (j, q) in oracle.samples.iter_mut().enumerate().skip(1) {
        let idx = query_bits_range(&q, bitrange.clone()) as usize;
        if firsts_idxs[idx].is_some() {
            if firsts_idxs.iter().all(|item| item.is_some()) {
                break;
            }
        } else {
            // this can never be zero.
            firsts_idxs[idx] = Some(unsafe { NonZeroUsize::new_unchecked(j) });
        }
    }
    let mut firsts = vec![None; maxj];
    firsts_idxs.sort_unstable();
    firsts_idxs
        .into_iter()
        .rev()
        .filter(|x| x.is_some())
        .for_each(|idx| {
            // safe as we've excluded the None values
            let idx = unsafe { idx.unchecked_unwrap() }.get();
            let item = oracle.samples.swap_remove(idx);
            let idx = query_bits_range(&item, bitrange.clone()) as usize;
            firsts[idx] = Some(item);
        });
    // not consuming the iterator to do as much as possible in-place.
    oracle.samples.par_iter_mut().for_each(|q| {
        let idx = query_bits_range(&q, bitrange.clone()) as usize;
        if let Some(item) = &firsts[idx] {
            q.xor_into(item);
        }
    });
}

fn bkw_reduce_sorted(oracle: &mut LpnOracle, i: usize, b: usize) {
    let k = oracle.get_k();
    let bitrange: ops::Range<usize> = (k - (b * i))..(k - (b * (i - 1)));

    let maxj = 2usize.pow(b as u32);
    // max j:
    println!(
        "BKW iteration, {} samples left, expecting to remove {} through sorting method",
        oracle.samples.len(),
        maxj
    );

    oracle.samples.par_sort_unstable_by_key(|q| {
        let key = query_bits_range(q, bitrange.clone());
        key
    });

    // split into partitions
    let oracle_start = oracle.samples.as_ptr() as usize;
    log::debug!("Creating pivots");
    let pivots = create_pivots(&mut oracle.samples, &bitrange);
    let partitions: PartitionIterator = create_partitions(&mut oracle.samples, &pivots);

    // process produced slices
    let partitions = partitions
        .map(|partition: &mut [Sample]| {
            let (partition, remainder) = partition.split_at_mut(1);
            let first = &partition[0];
            let len = remainder.len();
            remainder.iter_mut().enumerate().for_each(|(idx, q)| {
                let l = query_bits_range(&first, bitrange.clone());
                let r = query_bits_range(&q, bitrange.clone());
                debug_assert_eq!(l, r, "{:b} != {:b} (idx: {}/{})", l, r, idx, len);
                q.xor_into(first);
                debug_assert_eq!(0, query_bits_range(&q, bitrange.clone()));
            });
            partition.as_ptr() as usize
        })
        .collect::<Vec<_>>();

    // compute indexes of firsts
    let firsts = partitions
        .into_iter()
        .map(|partition| (partition - oracle_start) / std::mem::size_of::<Sample>());

    // this is descending because par_iter_map preserves order.
    for index in firsts {
        oracle.samples.swap_remove(index);
    }
}

/// Performs the BKW reduction algorithm, see [`partition_reduce`] for public usage
fn bkw_reduce(oracle: &mut LpnOracle, a: u32, b: u32) {
    let k = oracle.get_k();
    let a = a as usize;
    let b = b as usize;
    assert!(a * b <= k, "a*b <= k");

    for i in 1..a {
        // somewhat empirically decided through benchmark
        // probably related to size of LUT fitting in cache
        if b < 10 {
            bkw_reduce_inplace(oracle, i, b);
        } else {
            bkw_reduce_sorted(oracle, i, b)
        }
    }

    // Set the new k
    oracle.truncate(k - (a - 1) * b);
    println!(
        "BKW iterations done, {} samples left, k' = {}",
        oracle.samples.len(),
        oracle.get_k()
    );
}

/// Recover the secret using the majority strategy from BKW
pub fn majority(oracle: LpnOracle) -> BinVector {
    println!("BKW Solver: majority");
    let b = oracle.get_k();
    debug_assert!(b <= 20, "Don't run BKW on too-large b!");
    println!(
        "Selecting all samples with hw=1 from {} samples",
        oracle.samples.len()
    );
    let samples = oracle
        .samples
        .into_iter()
        .filter_map(|q| if q.count_ones() == 1 { Some(q) } else { None })
        .collect::<Vec<Sample>>();

    // allocate smaller vec
    let mut count_sum: FnvHashMap<StorageBlock, (u64, u64)> =
        FnvHashMap::with_capacity_and_hasher(b, Default::default());

    println!(
        "Sorting out and counting {} samples for majority selection",
        samples.len()
    );
    for query in samples.into_iter() {
        debug_assert_eq!(query.count_ones(), 1);
        let count_sum = count_sum.entry(query.get_block(0)).or_insert((0, 0));
        count_sum.0 += 1;
        if query.get_product() {
            count_sum.1 += 1;
        }
    }

    let mut result = BinVector::with_capacity(b as usize);
    let mut i = 1;
    while i < 1 << b {
        let (count, sum) = count_sum.get(&i).expect("this bucket can't be empty!");
        result.push(*count < 2 * sum);
        i <<= 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bkw() {
        let a = 4;
        let b = 8;

        let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
        oracle.get_samples(400_000);

        // get secret for checking
        let secret = &oracle.secret;
        println!("{:x?}", secret);
        let mut secret = secret.as_binvector(oracle.get_k());

        // run bkw
        let solution = bkw(oracle, a, b);
        secret.truncate(solution.len());
        assert_eq!(solution, secret);
    }

    #[test]
    fn test_partition() {
        let k = MAX_K - 10;
        let mut oracle = LpnOracle::new(k as u32, 1.0 / 8.0);
        oracle.get_samples(1000);
        let bitrange = k - 10..k;
        let pivots = create_pivots(&mut oracle.samples, &bitrange);
        let parts: PartitionIterator = create_partitions(&mut oracle.samples, &pivots);
        let mut failed = false;
        for part in parts.collect::<Vec<_>>() {
            let first_range = query_bits_range(&part[0], bitrange.clone());
            for (idx, sample) in part[1..].into_iter().enumerate() {
                let bits = query_bits_range(sample, bitrange.clone());
                if bits != first_range {
                    println!("failed for idx {} ({:b})", idx + 1, bits);
                    failed = true;
                }
            }
            if !failed {
                println!("still okay");
            }
        }
        assert!(!failed);
    }
}
