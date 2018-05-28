use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use oracle::LpnOracle;
use std::mem;

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
    let mut c = BinVector::new();
    let mut sample_matrix = Vec::new();
    let n_prime = oracle.queries.len();
    assert!(n_prime > 0, "What, no queries?");
    sample_matrix.reserve_exact(oracle.queries.len());
    c.reserve(oracle.queries.len());
    let b = oracle.queries[0].a.len();
    assert!(b < 21, "Don't use too large b! b = {}", b);
    assert!(b > 0, "Wtf, b = 0?");

    for query in oracle.queries {
        sample_matrix.push(query.a);
        c.push(query.s);
    }

    let a_matrix = BinMatrix::new(sample_matrix);

    let computation = |candidate: usize| {
        // A u32 is 4 u8s.
        let candidate_vector = usize_to_binvec(candidate, b);

        let mut matrix_vector_product: BinVector = &a_matrix * &candidate_vector;
        matrix_vector_product += &c;
        let hw = matrix_vector_product.count_ones();
        n_prime as i32 - 2 * (hw as i32)
    };

    // first result for v = 0
    let mut best_candidates: Vec<usize> = vec![0];
    let mut best_vec_weight = computation(0);
    println!("Doing LF1 naively");
    let max = 2usize.pow(b as u32);
    for candidate in 1..max {
        if candidate % 1000 == 0 {
            println!("Processed {}/{} candidates", candidate, max);
        }
        let candidate_weight = computation(candidate);
        if candidate_weight > best_vec_weight {
            best_vec_weight = candidate_weight;
            best_candidates.truncate(0);
            best_candidates.push(candidate);
        } else if candidate_weight == best_vec_weight {
            best_candidates.push(candidate);
        }
    }

    if best_candidates.len() > 1 {
        println!("* Debug: we also had these candidates: ");
        for candidate in &best_candidates {
            println!("\t{:?}", candidate);
        }
    }

    println!("Best candidate weight: {}", best_vec_weight);
    let best_candidate = best_candidates.pop().unwrap();
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
