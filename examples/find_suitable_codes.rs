//! Tries to find codes with desirable properties
//!
//! Not directly related to solving LPN

extern crate lpn;

#[allow(unused)]
use std::env;

#[cfg(all(feature = "hamming", feature="golay"))]
mod program {
    use lazy_static::lazy_static;
    use lpn::codes::*;

    const K: usize = 512;

    lazy_static! {
        static ref IDENTITIES: Vec<IdentityCode> = (0..=K).map(IdentityCode::new).collect();
        static ref REPETITIONS: Vec<RepetitionCode> = (0..=K).map(RepetitionCode::new).collect();
    }

    fn generate_codes(k: usize, k_min: usize, k_max: usize) {
        let mut codes: Vec<&dyn BinaryCode> = vec![
            &HammingCode3_1,
            &HammingCode7_4,
            &HammingCode15_11,
            &HammingCode31_26,
            &HammingCode63_57,
            &HammingCode127_120,
            &GolayCode23_12,
            &GolayCode23_12,
        ];
        codes.reserve(2 * k_max);
        for k in 1..=k_max {
            codes.push(&IDENTITIES[k]);
            codes.push(&REPETITIONS[k]);
        }

        let candidates = generate_code_recurse(k, 0, 0, &codes, Vec::new(), k_min, k_max);
        println!("Found {} candidates", candidates.len());
    }

    /// This uses exhaustive search, complexity is O(|codes|!)
    fn generate_code_recurse<'c>(
        target_len: usize,
        current_len: usize,
        current_dim: usize,
        codes: &[&'c dyn BinaryCode],
        current_combination: Vec<&'c dyn BinaryCode>,
        k_min: usize,
        k_max: usize,
    ) -> Vec<Vec<&'c dyn BinaryCode>> {
        if current_len > target_len || current_dim > k_max {
            return Vec::new();
        }

        if current_len == target_len && current_dim >= k_min && current_dim <= k_max {
            return vec![current_combination];
        }

        codes
            .iter()
            .flat_map(|code| {
                if current_combination.is_empty() {
                    println!("Working on starts with {}", code.name());
                }
                let new_len = code.length() + current_len;
                let new_dim = code.dimension() + current_dim;
                let mut new_combination: Vec<&dyn BinaryCode> = current_combination.clone();
                new_combination.push(*code);

                generate_code_recurse(
                    target_len,
                    new_len,
                    new_dim,
                    codes,
                    new_combination,
                    k_min,
                    k_max,
                )
            })
            .collect()
    }

    pub fn run(k: usize, k_min: usize, k_max: usize) {
        generate_codes(k, k_min, k_max);
    }
}

#[cfg(all(feature = "hamming", feature="golay"))]
fn main() {
    println!("This function has O(k!) runtime. Are you sure?");
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        println!("Got args: {:?}", args);
        panic!("Insufficient args. Expect k k'_min k'_max");
    }

    let k = args[1].parse::<usize>().expect("Needs to be int");
    let k_min = args[2].parse::<usize>().expect("Needs to be int");
    let k_max = args[3].parse::<usize>().expect("Needs to be int");

    program::run(k, k_min, k_max);
}

#[cfg(not(all(feature = "hamming", feature="golay")))]
fn main() {
    println!("Disabled necessary feature, example won't work");
}
