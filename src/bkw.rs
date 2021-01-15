//! Defines the algorithms from the classic Blum, Kalai and Wasserman paper
use crate::oracle::query_bits_range;
use crate::oracle::*;
use fnv::FnvHashMap;
use m4ri_rust::friendly::BinVector;
use std::default::Default;
use std::ops;

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

/// Reduces the LPN problem size using the reduction from Blum, Kalai and Wasserman.
pub fn partition_reduce(oracle: &mut LpnOracle, b: u32) {
    bkw_reduce(oracle, 2, b);
}

/// Performs the BKW reduction algorithm, see [`partition_reduce`] for public usage
fn bkw_reduce(oracle: &mut LpnOracle, a: u32, b: u32) {
    let k = oracle.get_k();
    let a = a as usize;
    let b = b as usize;
    assert!(a * b <= k, "a*b <= k");

    for i in 1..a {
        let num_samples = oracle.samples.len();
        // max j:
        let maxj = 2usize.pow(b as u32);
        println!("BKW iteration {}, {} samples left, expecting to remove {}", i, num_samples, maxj);
        

        let mut firsts: Vec<Option<&Sample>> = vec![None; maxj];

        let bitrange: ops::Range<usize> = ((k - (b * i)) as usize)..((k - (b * (i - 1))) as usize);
        let mut indexes = Vec::with_capacity(maxj);
        // not consuming the iterator to do as much as possible in-place.
        for (j, q) in oracle.samples.iter_mut().enumerate() {
            let idx = query_bits_range(&q, bitrange.clone()) as usize;
            if let Some(first) = firsts[idx] {
                q.xor_into(first)
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
    oracle.truncate( k - (a - 1) * b);
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
    debug_assert!(b <= 20, "Don't run BKW on too large b!");
    println!(
        "Selecting all samples with hw=1 from {} samples",
        oracle.samples.len()
    );
    assert!(oracle.samples.iter().find(|s| !s.get_product()).is_some(), "before select");
    let samples = oracle
        .samples
        .into_iter()
        .filter_map(|q| if q.count_ones() == 1 {
            Some(q)
        } else {
            None
        })
        .collect::<Vec<Sample>>();
    assert!(samples.iter().find(|s| !s.get_product()).is_some(), "after");


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
        oracle.get_samples(200_000);

        // get secret for checking
        let secret = &oracle.secret;
        println!("{:x?}", secret);
        let mut secret = secret.as_binvector(oracle.get_k());

        // run bkw
        let solution = bkw(oracle, a, b);
        secret.truncate(solution.len());
        assert_eq!(solution, secret);
    }
}
