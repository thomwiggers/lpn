extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;
extern crate itertools;

use std::mem;
use std::collections::HashSet;
use lpn::codes::*;
use m4ri_rust::friendly::*;
use itertools::Itertools;
use rayon::prelude::*;

static N: usize = 1000;

fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

fn get_distances<'a>(code: &'a BinaryCode<'a>) -> Vec<i32> {
    let mut distances = Vec::with_capacity(N);
    if 2f64.powi(code.length() as i32) > 1.5 * N as f64 {
        let mut seen = HashSet::new();     
        while seen.len() < N {
            let v = BinVector::random(code.length());
            if seen.contains(&v) {
                continue;
            }
            distances.push((&v + &code.decode_to_code(&v)).count_ones() as i32);
            seen.insert(v);
        }
    } else {
        for i in 0..2usize.pow(code.length() as u32) {
            let v = usize_to_binvec(i, code.length());
            distances.push((&v + &code.decode_to_code(&v)).count_ones() as i32);
        }
    }
    distances
}

fn main() {
    let codes: Vec<(String, &BinaryCode)> = vec![
        ("Hamming [3, 1]".to_owned(), &HammingCode3_1),
        ("Hamming [7, 4]".to_owned(), &HammingCode7_4),
        ("Hamming [15, 11]".to_owned(), &HammingCode15_11),
        ("Hamming [31, 26]".to_owned(), &HammingCode31_26),
    ];

    let candidate_codes: Vec<(&str, &BinaryCode)> = vec![
        ("Hamming [3, 1]", &HammingCode3_1),
        ("Hamming [3, 1]", &HammingCode3_1),
        ("Hamming [3, 1]", &HammingCode3_1),
        ("Hamming [7, 4]", &HammingCode7_4),
        ("Hamming [7, 4]", &HammingCode7_4),
        ("Hamming [7, 4]", &HammingCode7_4),
        ("Hamming [7, 4]", &HammingCode7_4),
        ("Hamming [15, 11]", &HammingCode15_11),
        ("Hamming [15, 11]", &HammingCode15_11),
        ("Hamming [31, 26]", &HammingCode31_26),
    ];

    let mut combined_codes = Vec::new();
    let mut combined_names = Vec::new();
    let mut testset: Vec<&BinaryCode> = vec![
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
    ];
    let stgen_code = StGenCode::new(testset.clone(), 3, 500, 3);
    let concat_code = ConcatenatedCode::new(testset);
    combined_names.push((999, stgen_code.length(), stgen_code.dimension(), vec![String::from("manual")]));
    combined_codes.push((String::from("[999]"), concat_code, stgen_code));
    
    /*
    for (i, combination) in candidate_codes.iter().combinations(5).enumerate() {
        let mut subcodes = Vec::with_capacity(combination.len());
        let mut names = Vec::with_capacity(combination.len());
        for (name, candidate) in &combination {
            names.push(name);
            subcodes.push(*candidate);
        }

        let stgen_code = StGenCode::new(subcodes.clone(), 3, 500, 3);
        let concat_code = ConcatenatedCode::new(subcodes);
        combined_names.push((i, stgen_code.length(), stgen_code.dimension(), names));
        combined_codes.push((format!("[{}]", i), concat_code, stgen_code));
    }
    */

    println!("");
    for (i, len, dim, codes) in combined_names.iter() {
        print!("{} [{}, {}]: ", i, len, dim);
        for code in codes {
            print!("{}, ", code);
        }
        println!("");
    }

    println!("");
    println!("Measuring biases\n");
    let biases: [f64; 4] = [0.03125, 0.1, 0.125, 0.25];
    print!("{:<20}", "name");
    biases.iter().for_each(|b| print!("{:>16.5}", b));
    print!("\n------------------------------------------");
    println!("------------------------------------------");
    for (name, code) in codes.into_iter() {
        print!("{:<20}", name);
        let distances = get_distances(code);
        for bias in &biases {
            let sum: f64 = distances.iter().map(|dist| bias.powi(*dist)).sum();    
            print!("{:16.5e}", sum/(distances.len() as f64));
        }
        println!("");
    }

    //let result_strings = Vec::with_capacity(stgen_codes.len());
    combined_codes.into_par_iter().for_each(|(name, concat_code, stgen_code)| {
        let mut line = format!("{:<20}", format!("Concat {}", name));
        let distances = get_distances(concat_code);

        for bias in &biases {
            let sum: f64 = distances.iter().map(|dist| bias.powi(*dist)).sum();    
            line.push_str(&format!("{:16.5e}", sum/(distances.len() as f64)));
        }

        line.push_str(&format!("\n{:<20}", format!("StGen {}", name)));
        let distances = get_distances(stgen_code);

        for bias in &biases {
            let sum: f64 = distances.iter().map(|dist| bias.powi(*dist)).sum();    
            line.push_str(&format!("{:16.5e}", sum/(distances.len() as f64)));
        }
        println!("{}", line);
        //result_strings.push(line);
    });

}
