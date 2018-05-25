#![feature(test)]
extern crate lpn;
extern crate m4ri_rust;
extern crate test;

use lpn::codes::*;
use m4ri_rust::friendly::*;
use test::Bencher;

fn get_code() -> StGenCode<'static, 'static> {
    let codes: Vec<&BinaryCode<'static>> = vec![
        &HammingCode7_4,
        &HammingCode3_1,
        &HammingCode15_11,
        &HammingCode7_4,
    ];
    StGenCode::new(codes, 5, 100, 3)
}

#[bench]
fn stgen_encode(b: &mut Bencher) {
    let code = get_code();

    let i = BinVector::random(code.dimension());

    b.iter(|| code.encode(&i));
}
