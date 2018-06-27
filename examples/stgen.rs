extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

use lpn::codes::*;
use m4ri_rust::friendly::*;

fn get_code() -> StGenCode<'static, 'static> {
    let codes: Vec<&BinaryCode<'static>> = vec![
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
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
    ];
    StGenCode::new(codes, 3, 100, 3)
}

fn main() {
    let code = get_code();
    println!("Code: [{}, {}]", code.length(), code.dimension());

    //(0..100).into_par_iter().for_each(|_| {
    for _ in 0..10 {
        let i = BinVector::random(code.length());
        code.decode_to_message(&i);
    }
    //});
}