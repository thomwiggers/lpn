use itertools::Itertools;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use oracle::{query_bits_range, LpnOracle, Query};
use rayon::prelude::*;
use std::mem;
use std::ops;

#[inline]
fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

/// Use the vector-matrix product version of the fwht
pub fn lf1_solve(oracle: LpnOracle) -> BinVector {
    // get the (a, c) samples as matrix A and vector c
    let n_prime = oracle.samples.len();
    assert!(n_prime > 0, "What, no samples?");

    let b = oracle.samples[0].a.len();
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
                        c.push(q.s);
                        q.a
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
    let candidate_vector = usize_to_binvec(best_candidate, b);
    candidate_vector
}

/// This is the LF2 reduction. This reduction grows the number of samples.
///
/// Applies a single round reduction
///
/// $k' = k - (a-1)*b$
/// $n' = n(n-1) / 2^{b+1}$  (for a = 1)
/// $\delta' = \delta^2$
pub fn xor_reduction(oracle: &mut LpnOracle, b: u32) {
    let k = oracle.k;
    assert!(b <= k);

    let num_samples = oracle.samples.len();
    println!("Xor-reduce iteration, {} samples", num_samples);
    // Partition into V_j
    // max j:
    let maxj = 2usize.pow(b);

    // using [v; n] clones and won't set capacity correctly.
    let mut vector_partitions = Vec::with_capacity(maxj);
    // num / 2^b choose 2
    let query_capacity = ((num_samples / maxj) * (num_samples / maxj - 1)) / 2;
    for _ in 0..maxj {
        vector_partitions.push(Vec::with_capacity(query_capacity));
    }

    let bitrange: ops::Range<usize> = ((k - b) as usize)..(k as usize);
    for mut q in oracle.samples.drain(..) {
        let idx = query_bits_range(&(q.a), &bitrange) as usize;
        if vector_partitions[idx].capacity() == 0 {
            println!(
                "Vector {} is full, will need to resize from {}",
                idx,
                vector_partitions[idx].len()
            );
        }
        q.a.truncate((k - b) as usize);
        vector_partitions[idx].push(q);
    }

    vector_partitions.par_iter_mut().for_each(|partition| {
        *partition = partition
            .iter()
            .tuple_combinations()
            .map(|(v1, v2)| Query {
                a: &v1.a + &v2.a,
                s: v1.s ^ v2.s,
                e: v1.e ^ v2.e,
            })
            .collect();
    });

    oracle.samples.reserve(vector_partitions.iter().fold(0, |acc, x| acc + x.len()));
    for partition in vector_partitions {
        oracle.samples.extend(partition.into_iter());
    }

    // Set the new k
    oracle.k = k - b;
    oracle.secret.truncate(oracle.k as usize);
    oracle.delta = oracle.delta.powi(2);

    println!(
        "Xor-reduce iterations done, {} samples now, k' = {}",
        oracle.samples.len(),
        oracle.k
    );
}

/// Solving using the Fast Walsh-Hamadard Transform
/// This section of code is based on the implementation of
/// LPN by Tramer (Bogos Tramer Vaudenay 2015)
pub fn fwht_solve(oracle: LpnOracle) -> BinVector {
    println!("FWHT solving...");
    debug_assert_eq!(oracle.samples[0].a.len() as u32, oracle.k);

    let mut majority_counter = vec![0i64; 2usize.pow(oracle.k)];
    oracle.samples.into_iter().for_each(|q| {
        majority_counter[q.a.as_u64() as usize] += if q.s { -1 } else { 1 };
    });

    fwht(majority_counter.as_mut_slice(), oracle.k);

    let guess = (0..2usize.pow(oracle.k as u32))
        .into_iter()
        .max_by_key(|x| majority_counter[*x])
        .unwrap();

    let mut result = BinVector::with_capacity(oracle.k as usize);
    for i in 0..oracle.k {
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
fn fwht(data: &mut [i64], bits: u32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmute_usize_to_u8s() {
        assert_eq!(
            usize_to_binvec(2, 4),
            BinVector::from_bools(&[false, false, true, false])
        );

        let a = 0x00_00_00_00__00_00_00_01usize;
        let binvec = usize_to_binvec(a, 50);
        for i in 0..49 {
            assert_eq!(binvec.get(i), Some(false), "bit {} isn't 0", i);
        }
        assert_eq!(binvec.get(49), Some(true));
    }

}
