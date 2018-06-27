extern crate itertools;
extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

use lpn::codes::*;

static K: usize = 128;

fn main() {
    let mut codes: Vec<(String, &dyn BinaryCode)> = vec![
        ("Hamming [3, 1]".to_owned(), &HammingCode3_1),
        ("Hamming [7, 4]".to_owned(), &HammingCode7_4),
        ("Hamming [15, 11]".to_owned(), &HammingCode15_11),
        ("Hamming [31, 26]".to_owned(), &HammingCode31_26),
    ];
    codes.reserve(K);
    for k in 1..K {
        let code = IdentityCode::new(k);
        let tuple: (String, &dyn BinaryCode) = (
            format!("Identity [{}, {}]", k, k),
            &&code
        );
        codes.push(tuple);
    }


}
