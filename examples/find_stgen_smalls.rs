#![feature(box_syntax)]
extern crate lpn;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use std::env;

use lpn::codes::*;
use rand::prelude::*;

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

lazy_static! {
    static ref IDENTITIES: Vec<IdentityCode> = (1..=4).map(IdentityCode::new).collect();
    static ref REPETITIONS: Vec<RepetitionCode> = (2..=4).map(RepetitionCode::new).collect();
}

fn generate_codes(n_min: usize, n_max: usize) {
    let mut codes: Vec<&dyn BinaryCode> =
        vec![&MdsCode3_2, &MdsCode4_3, &MdsCode5_4, &CustomCode5_3];
    codes.reserve(IDENTITIES.len() + REPETITIONS.len());
    for rep in REPETITIONS.iter() {
        codes.push(rep);
    }

    eprintln!("Benchmaring codes that are all the same");
    generate_codes_all_same(&codes, n_min, n_max);
    eprintln!("Benchmaring codes that start with a certain prefix");
    generate_codes_id_prefix(&codes, n_min, n_max);

    for id in IDENTITIES.iter() {
        codes.push(id);
    }
    eprintln!("Benchmaring randomly selected codes");
    for n in n_min..=n_max {
        for blocks in generate_random_codes(NUM_RANDOMS, &codes, n) {
            print_codes(&measure_codes(&blocks));
        }
    }
}

/// This generates codes that are all the same
/// complexity is O(|codes|*n_max*L_max/L_step*w0_max)
///
/// n: number of codes
fn generate_codes_all_same<'c>(codes: &[&'c dyn BinaryCode], n_min: usize, n_max: usize) {
    for code in codes {
        for n in n_min..=n_max {
            let blocks = std::iter::repeat(*code).take(n).collect::<Vec<_>>();
            print_codes(&measure_codes(&blocks));
        }
    }
}

/// Generate codes prefixed by IdentityCode
fn generate_codes_id_prefix<'c>(codes: &[&'c dyn BinaryCode], n_min: usize, n_max: usize) {
    for n in (n_min + 1)..=n_max {
        for id in IDENTITIES.iter() {
            for code in codes {
                let mut blocks: Vec<&dyn BinaryCode> = vec![id];
                blocks.extend(std::iter::repeat(*code).take(n).collect::<Vec<_>>());
                print_codes(&measure_codes(&blocks));
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
        .map(|_| {
            (0..n)
                .map(|_| {
                    *codes.choose(&mut rng).unwrap()
                })
                .collect()
        })
        .collect()
}

/// Print the code measurements
fn print_codes<'c>(results: &[(StGenCode<'c>, f64)]) {
    for (code, bias) in results {
        let json = serde_json::to_string(&code).unwrap();
        println!(
            "StGen [{},{}]\t{}\t{}\t{}\t{}\t{}\t{}",
            code.length(),
            code.dimension(),
            code.l_max(),
            code.wb(),
            code.w0(),
            bias,
            code.decoding_complexity(),
            json
        );
    }
}

fn measure_codes<'c>(blocks: &[&'c dyn BinaryCode]) -> Vec<(StGenCode<'c>, f64)> {
    let mut bests = Vec::with_capacity(
        (L_MAX - L_MIN) / L_STEP * ((W0_MAX - W0_MIN) as usize) * ((WB_MAX - WB_MIN) as usize),
    );
    for l in (L_MIN..=L_MAX).step_by(L_STEP) {
        for wb in WB_MIN..=WB_MAX {
            for w0 in W0_MIN..=W0_MAX {
                let best_stgen = (0..STGENS)
                    .map(|_| {
                        let code = StGenCode::new(blocks.to_vec(), w0, l, wb, WINC);
                        let bias = code.bias(0.75);
                        (code, bias)
                    })
                    .max_by(|t1, t2| {
                        if (t1.1).lt(&t2.1) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    })
                    .unwrap();
                bests.push(best_stgen);
            }
        }
    }
    bests
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Got args: {:?}", args);
        panic!("Insufficient args. Expect ./{} n_min n_max", args[0]);
    }
    let n_min = args[1].parse::<usize>().expect("Needs to be int");
    let n_max = args[2].parse::<usize>().expect("Needs to be int");

    generate_codes(n_min, n_max);
}
