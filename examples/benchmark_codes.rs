#![feature(box_syntax)]
extern crate itertools;
extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

use lpn::codes::*;
use rayon::prelude::*;
use itertools::Itertools;

use std::sync::*;

const K: usize = 512;
const DELTA: f64 = 1.0 / 8.0;

fn main() {
    let identities: Vec<IdentityCode> = (0..(K + 1)).into_iter().map(IdentityCode::new).collect();
    let repetitions: Vec<RepetitionCode> =
        (0..(K + 1)).into_iter().map(RepetitionCode::new).collect();
    let mut codes: Vec<&dyn BinaryCode> = vec![
        &HammingCode3_1,
        &HammingCode7_4,
        &HammingCode15_11,
        &HammingCode31_26,
        &GolayCode23_12,
        &GolayCode23_12,
    ];
    codes.reserve(K);
    for k in 1..(K + 1) {
        codes.push(&identities[k]);
        codes.push(&repetitions[k]);
    }

    let mut bias = [[0.0; K + 1]; K + 1];
    let stgen_bias = Arc::new(RwLock::new([[0.0; K + 1]; K + 1]));
    let mut params: Vec<Vec<Vec<&dyn BinaryCode>>> = vec![vec![Vec::new(); K + 1]; K + 1];

    bias[1][1] = 0.0;
    params[1][1].push(&identities[1]);

    for code in &codes {
        let n = code.length();
        let m = code.dimension();
        bias[n][m] = code.bias(DELTA);
        params[n][m] = vec![*code];
    }
    let mut stgens: Vec<Vec<Option<StGenCode>>> = vec![vec![None; K + 1]; K + 1];

    for j in 1..(K + 1) {
        for i in (j + 1)..(K + 1) {
            for code in &codes {
                let n = code.length();
                let m = code.dimension();
                let (idx_x, idx_y) = ((i as isize) - (n as isize), (j as isize) - (m as isize));
                if idx_x < 1 || idx_y < 1 {
                    continue;
                }
                let b_prior = bias[idx_x as usize][idx_y as usize];
                if (b_prior * bias[n][m]).abs() > bias[i][j].abs() {
                    bias[i][j] = b_prior * bias[n][m];
                    let mut new_params = params[idx_x as usize][idx_y as usize].clone();
                    new_params.push(*code);
                    debug_assert_eq!(new_params.iter().fold(0, |acc, c| acc + c.length()), i);
                    params[i][j] = new_params;
                }
            }
        }
    }

    for j in 1..(K + 1) {
        for i in 1..(K + 1) {
            let codes = params[i][j].clone();
            if codes.len() > 1 {
                stgens[i][j] = Some(StGenCode::new(codes, 300, 200, 4));
            }
        }
    }

    // zip (i, j)
    ((1..(K + 1)).into_iter().cartesian_product((1..(K + 1)).into_iter()).collect::<Vec<_>>())
        .into_par_iter()
        .for_each(|(i, j)| {
            if bias[i][j] != 0.0 {
                if let Some(stgen) = stgens[i][j].as_ref() {
                    println!("Computing bias for {}", stgen.name());
                    stgen_bias.write().unwrap()[i][j] = stgen.bias(DELTA);
                }
            }
        });

    for i in 1..(K + 1) {
        for j in 1..(K + 1) {
            if bias[i][j] == 0.0 {
                continue;
            }
            let codes = &params[i][j];
            println!(
                "[{}, {}] concatenated code with bc={:1.5e}: {:?}",
                i,
                j,
                bias[i][j],
                codes.iter().map(|c| c.name()).collect::<Vec<_>>()
            );
            if stgen_bias.read().unwrap()[i][j] > 0.0 {
                println!(
                    "[{}, {}] stgen code has bc={:1.5e}",
                    i,
                    j,
                    stgen_bias.read().unwrap()[i][j]
                );
            }
        }
    }
}
