//! Defines the Pooled Gauss solving algorithms by Esser, Kübler and May
use crate::oracle::LpnOracle;
use m4ri_rust::friendly::solve_left;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use rand::prelude::*;

/// Solves an LPN problem using Pooled Gauss
#[allow(clippy::many_single_char_names, clippy::needless_pass_by_value)]
pub fn pooled_gauss_solve(oracle: LpnOracle) -> BinVector {
    let k = oracle.k;
    let alpha = 0.5f64.powi(k as i32);
    let tau = (1.0 - oracle.delta) / 2.0;
    let beta = ((1f64 - tau) / 2f64).powi(k as i32);
    let m: f64 = (((1.5 * (1.0 / alpha).ln()).sqrt() + (1.0 / beta).ln().sqrt()) / (0.5 - tau))
        .powi(2)
        .floor();
    let c = (tau * m + (3.0 * (0.5 - tau) * (1.0 / alpha).ln() * m).sqrt().floor()) as u32;
    let m = m as usize;
    let mut rng = thread_rng();

    println!(
        "Attempting Pooled Gauss solving method, k={}, tau={}",
        k, tau
    );
    println!("Target secret weight <= {}", c);
    println!("Building (Am, b) with length {}", m);
    let (am, bm) = sample_matrix(m, &oracle, &mut rng);
    debug_assert_eq!(am.ncols(), k as usize);
    debug_assert_eq!(am.nrows(), m);
    debug_assert_eq!(bm.nrows(), m);
    debug_assert_eq!(bm.ncols(), 1);

    let test = |s_prime: &BinMatrix, tries: &mut usize| {
        debug_assert_eq!(s_prime.nrows(), oracle.k as usize);
        debug_assert_eq!(s_prime.ncols(), 1);
        *tries += 1;
        if *tries % 1000 == 0 {
            println!("Attempt {}...", tries);
        }
        let mut testproduct = &am * s_prime;
        testproduct += &bm;
        let result = testproduct.as_vector().count_ones() <= c;
        debug_assert_eq!(
            result,
            s_prime.as_vector() == oracle.secret,
            "Test will reject or accept an (in)correct secret with weight {} <= {}",
            testproduct.as_vector().count_ones(),
            c
        );
        result
    };

    println!("Starting random sampling of invertible (A, b)");

    let mut tries = 0;
    let s_prime = loop {
        // find k-rank matrix
        let (a, mut b) = loop {
            let (a_try, b_try) = sample_matrix(k as usize, &oracle, &mut rng);
            if a_try.clone().echelonize() == k as usize {
                break (a_try, b_try);
            }
        };
        // A*s = b
        if !solve_left(a, &mut b) {
            println!("Somehow, solving failed....");
            continue;
        }
        let result = { test(&b, &mut tries) };
        if result {
            println!("Found after {} tries", tries);
            break b;
        }
    };

    s_prime.as_vector()
}

/// Randomly sample ``k`` queries from the oracle as a ``(A, s)``.
fn sample_matrix(k: usize, oracle: &LpnOracle, rng: &mut ThreadRng) -> (BinMatrix, BinMatrix) {
    let samples = oracle.samples.choose_multiple(rng, k);
    // replace by matrix directly?
    let mut b_bits = BinVector::with_capacity(k as usize);
    (
        BinMatrix::new(
            samples
                .into_iter()
                .map(|q| {
                    b_bits.push(q.c);
                    q.a.clone()
                })
                .collect(),
        ),
        b_bits.as_column_matrix(),
    )
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_gauss() {
        let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
        oracle.get_samples(40555);
        let secret = oracle.secret.clone();
        let solution = pooled_gauss_solve(oracle);
        assert_eq!(solution, secret);
    }
}
