#![feature(box_syntax)]
extern crate itertools;
extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

use lpn::codes::*;
use rayon::prelude::*;

use std::sync::*;

const K: usize = 32;
const DELTA: f64 = 1.0 / 8.0;

fn main() {
    let identities: Vec<IdentityCode> = (0..(K + 1))
        .into_iter()
        .map(IdentityCode::new)
        .collect();
    let repetitions: Vec<RepetitionCode> = (0..(K + 1))
        .into_iter()
        .map(RepetitionCode::new)
        .collect();
    let mut codes: Vec<(String, &dyn BinaryCode)> = vec![
        ("Hamming [3, 1]".to_owned(), &HammingCode3_1),
        ("Hamming [7, 4]".to_owned(), &HammingCode7_4),
        ("Hamming [15, 11]".to_owned(), &HammingCode15_11),
        ("Hamming [31, 26]".to_owned(), &HammingCode31_26),
        ("Golay [23, 12]".to_owned(), &GolayCode23_12),
        ("Golay [24, 12]".to_owned(), &GolayCode23_12),
    ];
    codes.reserve(K);
    for k in 1..(K + 1) {
        let tuple: (_, &dyn BinaryCode) = (format!("Identity [{}, {}]", k, k), &identities[k]);
        codes.push(tuple);
        let tuple: (_, &dyn BinaryCode) = (format!("Repetition [{}, 1]", k), &repetitions[k]);
        codes.push(tuple);
    }

    let mut bias = [[0.0; K + 1]; K + 1];
    let stgen_bias = Arc::new(RwLock::new([[0.0; K + 1]; K + 1]));
    let mut params: Vec<Vec<Vec<(_, &(dyn BinaryCode))>>> = vec![vec![Vec::new(); K + 1]; K + 1];

    bias[1][1] = 0.0;
    params[1][1].push(("Identity [1, 1]".to_owned(), &identities[1]));

    for (name, code) in &codes {
        let n = code.length();
        let m = code.dimension();
        bias[n][m] = code.bias(DELTA);
        params[n][m] = vec![(name.to_string(), *code)];
    }
    let mut stgens: Vec<Vec<Option<StGenCode>>> = vec![vec![None; K + 1]; K + 1];
    let mut names: Vec<Vec<Vec<_>>> = vec![vec![Vec::new(); K + 1]; K + 1];

    for j in 1..(K + 1) {
        for i in (j + 1)..(K + 1) {
            for (name, code) in &codes {
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
                    new_params.push((name.to_owned(), *code));
                    debug_assert_eq!(new_params.iter().fold(0, |acc, c| acc + c.1.length()), i);
                    params[i][j] = new_params;
                }
            }
        }
    }

    for j in 1..(K + 1) {
        for i in 1..(K + 1) {
            let (the_names, codes): (Vec<_>, Vec<_>) = params[i][j].clone().into_iter().unzip();
            if codes.len() > 1 {
                names[i][j] = the_names;
                stgens[i][j] = Some(StGenCode::new(codes, 300, 200, 4));
            }
        }
    }

    for i in 1..(K + 1) {
        (1..(K + 1)).into_par_iter().for_each(|j| {
            if bias[i][j] != 0.0 {
                if names[i][j].len() > 1 {
                    println!(
                        "Computing bias for [{}, {}] code with {:?}",
                        i, j, names[i][j]
                    );
                    let stgen = stgens[i][j].as_ref().expect("What, no munny?");
                    stgen_bias.write().unwrap()[i][j] = stgen.bias(DELTA);
                }
            }
        });
    }

    for i in 1..(K + 1) {
        for j in 1..(K + 1) {
            if bias[i][j] == 0.0 {
                continue;
            }
            let (names, _): (Vec<_>, Vec<_>) = params[i][j].clone().into_iter().unzip();
            println!(
                "[{}, {}] concatenated code with bc={:1.5e}: {:?}",
                i, j, bias[i][j], names
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
