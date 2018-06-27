extern crate itertools;
extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

use itertools::Itertools;
use lpn::codes::*;
use m4ri_rust::friendly::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::mem;

static K: usize = 128;

fn main() {
    let mut codes: Vec<(String, &dyn BinaryCode)> = vec![
        ("Hamming [3, 1]".to_owned(), &HammingCode3_1),
        ("Hamming [7, 4]".to_owned(), &HammingCode7_4),
        ("Hamming [15, 11]".to_owned(), &HammingCode15_11),
        ("Hamming [31, 26]".to_owned(), &HammingCode31_26),
    ];
    codes.reserve(K);
    let owned_codes = Vec::with_capacity(K);
    for k in 0..K {
        let new_code = IdentityCode::new(k);
        codes.push((format!("Identity [{},{}]", k, k), &&new_code));
        owned_codes.push(new_code);
    }


}
