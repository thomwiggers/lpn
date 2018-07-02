#![feature(test)]
extern crate lpn;
extern crate m4ri_rust;
extern crate test;

use lpn::codes::*;
use m4ri_rust::friendly::*;
use test::Bencher;

fn get_code() -> ConcatenatedCode<'static> {
    let codes: Vec<&BinaryCode> = vec![
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode3_1,
        &HammingCode15_11,
        &HammingCode7_4,
    ];
    ConcatenatedCode::new(codes)
}

#[bench]
fn encode(b: &mut Bencher) {
    let code = get_code();

    let i = BinVector::random(code.dimension());

    b.iter(|| code.encode(&i));
}

#[bench]
fn decode(b: &mut Bencher) {
    let code = get_code();

    b.iter(|| {
        let i = BinVector::random(code.length());
        code.decode_to_message(&i).unwrap()
    });
}
