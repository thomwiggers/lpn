use oracle::LpnOracle;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::mem;

pub fn lf1_solve(oracle: LpnOracle) -> BinVector
{
    // get the (a, c) samples as matrix A and vector c
    let mut c = BinVector::new();
    let mut sample_matrix = Vec::new();
    let n_prime = oracle.queries.len();
    assert!(n_prime > 0, "What, no queries?");
    sample_matrix.reserve_exact(oracle.queries.len());
    c.reserve(oracle.queries.len());
    let b = oracle.queries[0].a.len();
    assert!(b < 20, "Don't use too large b!");
    assert!(b > 0, "Wtf, b = 0?");

    for query in oracle.queries {
        sample_matrix.push(query.a);
        c.push(query.s);
    }

    let a_matrix = BinMatrix::new(sample_matrix);

    let computation = |candidate: usize| {
        // A u32 is 4 u8s.
        let candidate_bytes = unsafe {
            mem::transmute::<usize,[u8;mem::size_of::<usize>()]>(candidate.to_le())
        };
        let mut candidate_vector = BinVector::from_bytes(&candidate_bytes);
        candidate_vector.truncate(b as usize);

        let mut matrix_vector_product: BinVector = &a_matrix * &candidate_vector;
        matrix_vector_product += &c;
        let hw = matrix_vector_product.count_ones();
        n_prime as i32 - 2 * (hw as i32)
    };

    // first result for v = 0
    let mut best_candidate = 0;
    let mut best_vec_weight = computation(0);
    println!("Doing LF1 naively");
    for candidate in 1..2usize.pow(b as u32) {
        let candidate_weight = computation(candidate);
        if candidate_weight > best_vec_weight {
            best_vec_weight = candidate_weight;
            best_candidate = candidate;
        }
    }

    let candidate_bytes;
    unsafe {
        candidate_bytes = mem::transmute::<usize,[u8;mem::size_of::<usize>()]>(best_candidate);
    }
    let mut candidate_vector = BinVector::from_bytes(&candidate_bytes);
    candidate_vector.truncate(b as usize);
    candidate_vector
}
