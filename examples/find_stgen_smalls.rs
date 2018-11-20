#![feature(box_syntax)]
extern crate lpn;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use std::env;

use lpn::codes::*;

const STGENS: usize = 2;
const W0_MIN: u32 = 2;
const W0_MAX: u32 = 4;
const WB_MIN: u32 = 1;
const WB_MAX: u32 = 4;
const WINC: u32 = 1;
const L_MIN: usize = 200;
const L_MAX: usize = 1200;
const L_STEP: usize = 200;

lazy_static! {
    static ref IDENTITIES: Vec<IdentityCode> = (1..=4).map(IdentityCode::new).collect();
    static ref REPETITIONS: Vec<RepetitionCode> = (2..=4).map(RepetitionCode::new).collect();
}

fn generate_codes(n_max: usize) {
    let mut codes: Vec<&dyn BinaryCode> =
        vec![&MdsCode3_2, &MdsCode4_3, &MdsCode5_4, &CustomCode5_3];
    codes.reserve(IDENTITIES.len() + REPETITIONS.len());
    for rep in REPETITIONS.iter() {
        codes.push(rep);
    }

    generate_codes_all_same(&codes, n_max);

    for id in IDENTITIES.iter() {
        codes.push(id);
    }
}

/// This uses exhaustive search, complexity is O(|codes|!)
///
/// n: number of codes
fn generate_codes_all_same<'c>(codes: &[&'c dyn BinaryCode], max_n: usize) {
    let mut bests = Vec::with_capacity(codes.len() * max_n);
    for code in codes.iter() {
        for l in (L_MIN..=L_MAX).step_by(L_STEP) {
            for wb in WB_MIN..=WB_MAX {
                for w0 in W0_MIN..=W0_MAX {
                    for n in 1..=max_n {
                        let blocks = std::iter::repeat(*code).take(n).collect::<Vec<_>>();
                        let best_stgen = (0..STGENS)
                            .map(|_| {
                                let code = StGenCode::new(blocks.clone(), w0, l, wb, WINC);
                                println!("Measuring {}", code.name());
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
        }
    }
    for (code, bias) in bests.into_iter() {
        let json = serde_json::to_string(&code).unwrap();
        println!(
            "{}\t{}\t{}\t{}",
            code.name(),
            bias,
            code.decoding_complexity(),
            json
        );
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Got args: {:?}", args);
        panic!("Insufficient args. Expect ./{} n_max", args[0]);
    }

    let n_max = args[1].parse::<usize>().expect("Needs to be int");

    generate_codes(n_max);
}
