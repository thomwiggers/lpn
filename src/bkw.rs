//! Defines the algorithms from the classic Blum, Kalai and Wasserman paper
use fnv::FnvHashMap;
use m4ri_rust::friendly::BinVector;
use oracle::query_bits_range;
use oracle::*;
use std::default::Default;
use std::ops;

/// The full BKW solving algorithm.
///
/// Does `a-1` applications of `partition-reduce(8)` and solves via majority.
///
/// $k' = k - (a-1) * b$
/// $n' = n - (a-1)*2^b
/// $d' = delta^{2*(a-1)}$
pub fn bkw(mut oracle: LpnOracle, a: u32, b: u32) -> BinVector {
    bkw_reduce(&mut oracle, a, b);
    majority(oracle)
}

/// Reduces the LPN problem size using the reduction from Blum, Kalai and Wasserman.
pub fn partition_reduce(oracle: &mut LpnOracle, b: u32) {
    bkw_reduce(oracle, 2, b);
}

fn bkw_reduce(oracle: &mut LpnOracle, a: u32, b: u32) {
    let k = oracle.k;
    assert!(a * b <= k, "a*b <= k");

    for i in 1..a {
        let num_samples = oracle.samples.len();
        println!("BKW iteration {}, {} samples left", i, num_samples);
        // Partition into V_j
        // max j:
        let maxj = 2usize.pow(b);
        let query_capacity = num_samples / maxj;

        let mut vector_partitions: Vec<Vec<&Query>> = Vec::with_capacity(maxj);
        // using [v; n] clones and won't set capacity correctly.
        for _ in 0..maxj {
            vector_partitions.push(Vec::with_capacity(query_capacity));
        }
        let mut firsts: Vec<Option<&Query>> = vec![None; maxj];

        let bitrange: ops::Range<usize> = ((k - (b * i)) as usize)..((k - (b * (i - 1))) as usize);
        let mut indexes = Vec::with_capacity(maxj);
        for (j, q) in oracle.samples.iter_mut().enumerate() {
            let idx = query_bits_range(&(q.a), &bitrange) as usize;
            q.a.truncate((k - (b * i)) as usize);
            if let Some(first) = firsts[idx] {
                q.a += &first.a;
                q.c ^= first.c;
            } else {
                firsts[idx] = Some(q);
                indexes.push(j);
            }
        }
        indexes.into_iter().for_each(|idx| {
            oracle.samples.swap_remove(idx);
        });
    }

    // Set the new k
    oracle.k = k - (a - 1) * b;
    oracle.secret.truncate(oracle.k as usize);
    println!(
        "BKW iterations done, {} samples left, k' = {}",
        oracle.samples.len(),
        oracle.k
    );
}

/// Recover the secret using the majority strategy from BKW
pub fn majority(oracle: LpnOracle) -> BinVector {
    println!("BKW Solver: majority");
    let b = oracle.samples[0].a.len();
    debug_assert!(b <= 20, "Don't run BKW on too large b!");
    println!(
        "Selecting all samples with hw=1 from {} samples",
        oracle.samples.len()
    );
    let range = 0..(b);
    let samples = oracle
        .samples
        .into_iter()
        .filter(|q| q.count_ones() == 1)
        .map(|q| (query_bits_range(&q.a, &range), q.c))
        .collect::<Vec<(u64, bool)>>();

    // allocate smaller vec
    let mut counts: FnvHashMap<u64, u64> =
        FnvHashMap::with_capacity_and_hasher(b, Default::default());
    let mut sums: FnvHashMap<u64, u64> =
        FnvHashMap::with_capacity_and_hasher(b, Default::default());

    println!(
        "Sorting out and counting {} samples for majority selection",
        samples.len()
    );
    for query in samples.into_iter() {
        debug_assert_eq!(query.0.count_ones(), 1);
        let count = counts.entry(query.0).or_insert(0);
        *count += 1;
        if query.1 {
            let sum = sums.entry(query.0).or_insert(0);
            *sum += 1;
        }
    }

    let mut result = BinVector::with_capacity(b as usize);
    let mut i = 1 << (b - 1);
    while i > 0 {
        let count = counts.get(&i).expect("this bucket can't be empty!");
        if let Some(sum) = sums.get(&i) {
            result.push(*count < 2 * sum);
        } else {
            result.push(false);
        }
        i >>= 1;
    }
    result
}
