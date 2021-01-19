//! Tries to find StGen codes with desirable properties
//!
//! Not directly related to LPN solving

extern crate lazy_static;
extern crate lpn;
extern crate rand;

#[cfg(feature = "codes")]
mod program {

    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::{Result, Write};

    use lpn::codes::*;
    use rand::prelude::*;
    use sha2::{Digest, Sha256};

    const STGENS: usize = 2;
    const W0_MIN: u32 = 2;
    const W0_MAX: u32 = 4;
    const WB_MIN: u32 = 1;
    const WB_MAX: u32 = 4;
    const WINC: u32 = 1;
    const L_MIN: usize = 200;
    const L_MAX: usize = 1200;
    const L_STEP: usize = 200;
    const NUM_RANDOMS: usize = 20;
    const NUM_TAUS: usize = 8;
    //                      1/512       1/256       1/128      1/64      1/32     1/16     1/8    1/4
    const TAUS: [f64; NUM_TAUS] = [
        0.001953125,
        0.00390625,
        0.0078125,
        0.015625,
        0.03125,
        0.0625,
        0.125,
        0.25,
    ];

    lazy_static! {
        static ref IDENTITIES: Vec<IdentityCode> = (1..=4).map(IdentityCode::new).collect();
        static ref REPETITIONS: Vec<RepetitionCode> = (2..=4).map(RepetitionCode::new).collect();
        static ref DELTAS: Vec<f64> = TAUS.iter().map(|t| 1.0 - 2.0 * t).collect();
    }

    fn generate_codes(mut result_file: File, n_min: usize, n_max: usize) {
        let mut codes: Vec<&dyn BinaryCode> =
            vec![&MdsCode3_2, &MdsCode4_3, &MdsCode5_4, &CustomCode5_3];
        codes.reserve(IDENTITIES.len() + REPETITIONS.len());
        for rep in REPETITIONS.iter() {
            codes.push(rep);
        }

        eprintln!("Benchmarking codes that are all the same");
        generate_codes_all_same(&mut result_file, &codes, n_min, n_max);
        eprintln!("Benchmarking codes that start with a certain prefix");
        generate_codes_id_prefix(&mut result_file, &codes, n_min, n_max);

        for id in IDENTITIES.iter() {
            codes.push(id);
        }
        eprintln!("Benchmarking randomly selected codes");
        for n in n_min..=n_max {
            for blocks in generate_random_codes(NUM_RANDOMS, &codes, n) {
                print_codes(&mut result_file, &measure_codes(&blocks));
            }
        }
    }

    /// This generates codes that are all the same
    /// complexity is O(|codes|*n_max*L_max/L_step*w0_max)
    ///
    /// n: number of codes
    fn generate_codes_all_same<'f, 'c>(
        result_file: &'f mut File,
        codes: &[&'c dyn BinaryCode],
        n_min: usize,
        n_max: usize,
    ) {
        for code in codes {
            for n in n_min..=n_max {
                let blocks = std::iter::repeat(*code).take(n).collect::<Vec<_>>();
                print_codes(result_file, &measure_codes(&blocks));
            }
        }
    }

    /// Generate codes prefixed by IdentityCode
    fn generate_codes_id_prefix<'f, 'c>(
        result_file: &'f mut File,
        codes: &[&'c dyn BinaryCode],
        n_min: usize,
        n_max: usize,
    ) {
        for n in (n_min + 1)..=n_max {
            for id in IDENTITIES.iter() {
                for code in codes {
                    let mut blocks: Vec<&dyn BinaryCode> = vec![id];
                    blocks.extend(std::iter::repeat(*code).take(n).collect::<Vec<_>>());
                    print_codes(result_file, &measure_codes(&blocks));
                }
            }
        }
    }

    /// Generate `amount` codes built from `n` blocks by randomly selecting blocks
    fn generate_random_codes<'c>(
        amount: usize,
        codes: &[&'c dyn BinaryCode],
        n: usize,
    ) -> Vec<Vec<&'c dyn BinaryCode>> {
        let mut rng = rand::thread_rng();
        (0..amount)
            .map(|_| (0..n).map(|_| *codes.choose(&mut rng).unwrap()).collect())
            .collect()
    }

    /// Print the code measurements
    fn print_codes<'f, 'c>(result_file: &mut File, results: &[(StGenCode<'c>, Vec<f64>)]) {
        for (code, biases) in results {
            print!(
                "StGen [{},{}]\t{}\t{}\t{}\t",
                code.length(),
                code.dimension(),
                code.l_max(),
                code.wb(),
                code.w0(),
            );

            for (bias, delta) in biases.iter().zip(DELTAS.iter()) {
                print!("{}:{}\t", delta, bias);
            }
            let hash = append_code_to_file(result_file, &code).unwrap();
            println!("{}\t{}", code.decoding_complexity(), hash);
        }
    }

    fn measure_codes<'c>(blocks: &[&'c dyn BinaryCode]) -> Vec<(StGenCode<'c>, Vec<f64>)> {
        let mut bests = Vec::with_capacity(
            (L_MAX - L_MIN) / L_STEP * ((W0_MAX - W0_MIN) as usize) * ((WB_MAX - WB_MIN) as usize),
        );
        for l in (L_MIN..=L_MAX).step_by(L_STEP) {
            for wb in WB_MIN..=WB_MAX {
                for w0 in W0_MIN..=W0_MAX {
                    let best_stgen = (0..STGENS)
                        .map(|_| {
                            let code = StGenCode::new(blocks.to_vec(), w0, l, wb, WINC);
                            if let Some(biases) = code.biases(&DELTAS) {
                                (code, *biases.last().unwrap(), biases)
                            } else {
                                (code, 0f64, Vec::new())
                            }
                        })
                        .max_by(|t1, t2| {
                            if (t1.1).lt(&t2.1) {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        })
                        .unwrap();
                    bests.push((best_stgen.0, best_stgen.2));
                }
            }
        }
        bests
    }

    fn append_code_to_file<'f, 'c>(file: &'f mut File, code: &StGenCode<'c>) -> Result<String> {
        let json = serde_json::to_string(&code).unwrap();
        let hash = Sha256::digest(json.as_bytes());
        file.write_fmt(format_args!("{:x}\t{}\n", hash, json))?;
        return Ok(format!("{:x}", hash));
    }
}

#[cfg(feature = "codes")]
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Got args: {:?}", args);
        panic!(
            "Insufficient args. Expect ./{} results.txt n_min n_max",
            args[0]
        );
    }
    let filename = &args[1];
    let n_min = args[2].parse::<usize>().expect("Needs to be int");
    let n_max = args[3].parse::<usize>().expect("Needs to be int");

    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    program::generate_codes(file, n_min, n_max);
}

#[cfg(not(feature = "codes"))]
fn main() {
    println!("necessary feature disabled");
}
