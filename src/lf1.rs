use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use oracle::LpnOracle;
use rayon::prelude::*;
use std::mem;

#[inline]
fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

pub fn lf1_solve(oracle: LpnOracle) -> BinVector {
    // get the (a, c) samples as matrix A and vector c
    let n_prime = oracle.queries.len();
    assert!(n_prime > 0, "What, no queries?");

    let b = oracle.queries[0].a.len();
    assert!(b < 21, "Don't use too large b! b = {}", b);
    assert!(b > 0, "Wtf, b = 0?");

    // split the query set into a matrix and a vector
    let (a_matrix, c) = {
        let mut c = BinVector::with_capacity(n_prime);
        (
            BinMatrix::new(
                oracle
                    .queries
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
