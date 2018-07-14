use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;


#[derive(Clone)]
pub struct HammingCode63_57;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 63]> = 0 as *const FnvHashMap<usize, [bool; 63]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]),
                BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]),
                BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, true]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, false, true, false, true, false, false, true, false, false, true, true, true, true, false, true]),
                BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, false, false, true, true, false, false, true, false, false, true, false, true, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, true, true, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 0 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 1 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 2 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(3, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 3 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(4, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 4 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(5, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 5 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(6, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 6 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(7, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 7 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(8, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 8 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(9, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 9 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(10, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 10 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(11, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 11 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(12, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 12 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(13, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 13 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(14, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 14 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(15, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 15 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(16, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 16 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(17, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 17 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(18, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 18 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(19, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 19 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(20, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 20 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(21, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 21 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(22, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 22 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(23, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 23 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(24, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 24 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(25, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 25 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(26, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 26 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(27, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 27 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(28, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 28 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(29, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 29 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(30, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 30 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(31, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 31 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(32, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 32 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(33, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 33 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(34, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 34 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(35, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 35 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(36, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 36 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(37, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 37 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(38, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 38 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(39, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 39 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(40, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 40 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(41, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 41 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(42, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 42 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(43, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 43 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(44, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 44 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(45, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 45 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(46, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 46 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(47, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 47 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(48, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 48 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(49, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 49 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(50, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 50 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(51, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 51 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(52, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 52 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(53, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 53 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(54, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 54 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(55, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 55 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(56, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 56 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(57, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 57 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(58, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 58 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(59, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 59 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(60, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 60 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(61, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 61 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(62, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 62 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(63, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 63 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode for HammingCode63_57 {
    fn name(&self) -> String {
        "[63, 57] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        63
    }

    fn dimension(&self) -> usize {
        57
    }

    fn generator_matrix(&self) -> &BinMatrix {
        init();
        unsafe {
            GENERATOR_MATRIX.as_ref().unwrap()
        }
    }

    fn parity_check_matrix(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX.as_ref().unwrap()
        }
    }

    fn decode_to_code(&self, c: &BinVector) -> Result<BinVector, &str> {
        init();
        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        debug_assert_eq!(c.len(), self.length(), "the length doesn't match the expected length (length of the code)");
        let he = self.parity_check_matrix() * c;
        let error = BinVector::from_bools(&map[&(he.as_u64() as usize)]);
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((self.parity_check_matrix() * &result).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(57);
        Ok(codeword)
        
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1f64 + (63 as f64) * delta) / ((63 + 1) as f64)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode63_57.generator_matrix();
        assert_eq!(code.ncols(), 63);
        assert_eq!(code.nrows(), 57);
    }

    #[test]
    fn random_decode_tests() {

        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, false, false, false, false, true, true, false, true, true, false, false, true, false, false, false, false, false, false, true, true, true, true, true, false, true, false, true, true, false, true, true, false, true, true, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, false, false, false, false, true, true, false, true, true, false, false, true, false, false, false, false, false, false, true, true, true, true, true, false, true, false, true, true, false, true, true, false, true, true, true, false, false, false, false, true, false, true, true, false, false, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, false, true, true, false, true, true, false, false, true, false, true, true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, false, true, true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, false, true, true, false, false, true, false, false, true, true, true, false, false, false, false, false, true, false, true, false, true, false, true, true, false, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, false, true, true, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, true, true, true, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, true, false, true, false, true, false, false, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, true, true, true, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, true, false, true, false, true, false, false, true, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, false, true, false, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, true, true, false, false, false, false, true, true, true, true, false, true, true, false, true, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, false, false, false, false, true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, true, true, false, false, false, false, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, false, false, false, false, true, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, true, false, false, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, true, true, true, false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, true, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true, false, false, false, true, true, true, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true, true, true, false, true, true, false, false, true, true, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true, true, true, false, true, true, false, false, true, true, false, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, true, false, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, false, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true, false, true, true, true, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true, false, true, true, true, false, true, true, true, true, true, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true, true, false, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true, true, true, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, true, true, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true, true, true, false, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, true, true, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true, true, true, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, true, false, false, false, true, false, false, true, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, true, false, false, false, true, false, false, true, true, true, true, false, true, false, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, true, false, false, true, true, false, false, true, false, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, true, true, false, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, true, true, false, true, true, true, false, true, true, false, false, true, false, true, false, true, false, true, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, true, false, true, false, true, false, true, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true, true, false, false, true, false, false, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, false, true, false, true, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true, true, false, false, true, false, false, false, true, false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, false, true, false, true, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false, true, true, false, false, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false, true, true, false, false, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false, true, false, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, true, true, false, true, true, false, true, true, false, false, true, true, true, true, false, false, false, true, true, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, true, true, false, true, true, false, true, true, false, false, true, true, true, true, false, false, false, true, true, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, true, true, false, false, false, true, false, true, true, false, true, true, true, false, false, false, false, true, false, false, true, true, true, false, true, true, false, true, true, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, true, true, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, true, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, true, true, false, true, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, true, true, false, true, true, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, false, false, true, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, false, false, true, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, false, true, false, true, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, true, true, false, false, true, false, false, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, false, true, false, true, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, true, true, true, false, false, true, false, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, true, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, false, true, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, true, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, true, false, false, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, true, false, false, true, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false, true, false, true, true, true, true, true, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false, true, false, true, true, true, true, true, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, true, true, false, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, true, true, false, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false, true, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, false, true, true, false, true, false, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, true, false, false, false, true, true, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, true, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, true, true, true, false, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, true, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, true, false, true, false, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, true, false, true, false, true, true, true, true, true, true, false, false, false, true, true, false, false, true, false, true, true, false, true, true, true, false, true, false, true, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, true, false, true, false, true, true, true, true, true, true, false, false, false, true, true, false, false, true, false, true, true, false, true, true, true, false, true, true, true, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, false, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, true, true, true, false, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, false, false, false, false, true, true, false, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, true, true, true, false, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, false, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, false, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, false, true, true, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true, true, true, true, true, false, false, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, false, true, true, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true, true, true, true, true, false, false, false, true, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, true, true, true, true, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, false, true, true, true, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, true, false, true, true, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, false, true, true, true, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, true, false, false, false, true, false, true, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, true, false, false, false, true, false, true, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, false, false, false, true, true, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, false, false, false, true, true, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, true, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, true, true, true, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, true, true, true, false, true, false, true, true, true, false, false, true, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, false, false, false, true, true, false, true, true, true, true, false, false, false, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, true, true, true, false, true, false, true, true, true, false, false, true, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, true, false, false, true, true, false, true, true, true, true, false, false, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true, false, false, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, true, false, true, false, true, false, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, true, false, true, false, true, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, true, false, false, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false, false, true, false, false, true, false, true, false, false, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, true, false, false, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, false, true, false, false, true, true, true, true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, false, false, false, false, true, true, true, true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, true, true, true, true, false, true, true, true, false, true, false, true, false, true, true, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, false, true, true, false, false, false, false, false, false, true, true, true, true, false, true, true, true, false, true, false, true, false, true, true, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, true, false, true, true, true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, true, true, false, true, false, false, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, true, false, true, true, true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, true, false, true, false, false, true, false, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, true, false, false, true, true, true, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, true, false, false, true, true, true, true, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, true, false, true, false, true, false, true, true, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, true, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, true, true, false, false, true, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, true, true, true, true, false, true, true, true, false, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, true, true, false, false, true, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true, false, false, true, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, true, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, true, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, true, false, true, false, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, true, false, true, false, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, false, true, true, false, true, true, false, false, false, false, false, false, true, false, true, false, true, true, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, false, true, true, false, true, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, true, false, false, false, false, true, false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, false, true, false, true, true, false, false, true, false, false, true, true, false, false, false, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true, false, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, false, true, false, true, true, false, false, true, false, false, true, true, false, false, false, false, false, false, true, false, true, false, true, true, false, false, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false, true, true, false, true, true, true, false, true, true, false, false, false, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, false, false, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false, false, false, false, true, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, false, false, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false, true, false, false, true, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, true, true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, false, false, true, false, false, false, false, true, true, false, false, true, true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, true, true, true, true, false, false, true, true, false, false, false, true, false, false, false, false, true, false, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, true, true, false, true, false, false, true, false, false, true, false, true, true, false, true, false, true, false, false, false, true, true, true, false, true, false, true, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, true, true, false, true, false, false, true, false, false, true, false, true, true, false, true, false, true, false, true, false, true, true, true, false, true, false, true, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, false, true, false, true, false, false, false, false, true, true, true, true, false, true, false, false, true, false, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, false, true, false, true, false, false, false, false, true, true, true, true, false, true, false, false, true, false, true, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true, false, false, true, true, false, false, false, true, true, false, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true, false, false, true, true, false, false, false, true, true, false, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, false, true, false, true, true, false, false, false, true, false, false, true, true, true, false, false, true, false, true, true, true, false, false, false, true, true, false, true, false, false, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, false, true, false, true, true, false, false, false, true, false, false, true, true, true, false, false, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, true, true, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, true, false, false, true, true, false, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, true, true, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false, false, false, true, true, false, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, false, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, false, true, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, true, true, true, true, true, false, true, true, false, false, true, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, true, false, true, true, false, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, true, false, false, false, false, false, true, true, true, true, true, false, true, true, false, false, true, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, true, false, true, true, false, true, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, false, false, true, true, false, true, true, false, false, true, true, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, false, false, true, true, false, true, true, false, false, true, true, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, true, false, true, true, true, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true, true, false, false, true, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, true, false, true, true, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true, true, false, false, true, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, false, true, true, true, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, true, true, false, false, true, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, true, true, false, false, true, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, false, false, false, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, false, false, false, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, true, false, true, false, true, true, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, true, false, false, true, false, false, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, true, false, true, false, true, true, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, true, false, false, true, false, true, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, true, true, true, false, true, true, true, false, false, true, false, true, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, true, true, true, false, true, true, true, false, false, true, false, true, false, false, false, true, false, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, false, false, true, false, false, false, true, false, true, true, true, false, true, true, true, true, true, true, false, true, false, false, false, true, true, true, true, true, true, false, false, true, true, true, false, false, true, true, false, true, false, false, false, true, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, false, false, true, false, false, false, true, false, true, true, true, false, true, true, true, true, true, true, false, true, false, false, false, true, true, true, true, true, true, false, false, true, true, true, false, false, true, true, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, false, true, true, true, false, true, false, false, true, false, true, true, true, true, true, false, false, true, false, true, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, false, true, true, true, false, true, false, false, true, false, true, true, true, true, true, false, false, true, false, true, true, true, false, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, true, false, false, false, false, false, false, true, true, false, true, true, false, false, true, false, true, true, true, true, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, true, true, false, false, false, false, false, true, true, false, true, true, false, false, true, false, true, true, true, true, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, false, false, true, true, false, false, false, false, true, true, false, true, true, false, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, false, true, true, false, true, false, false, true, true, true, false, true, false, true, true, true, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, false, false, true, true, false, false, false, false, true, true, false, true, true, false, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, false, true, true, false, true, false, false, true, true, true, false, true, false, true, true, true, false, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, false, true, true, false, false, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, false, true, true, true, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, false, false, false, true, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, true, true, true, true, true, true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, false, false, false, true, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, false, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, true, false, true, true, false, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, false, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, false, false, true, true, false, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true, false, true, false, false, true, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, true, true, false, false, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true, false, true, false, false, true, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, true, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, true, false, true, false, true, false, false, false, false, true, true, false, true, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, true, false, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, true, true, true, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, true, false, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, false, true, true, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, true, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, true, false, true, false, false, false, false, false, false, false, false, true, true, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, true, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, false, false, false, false, true, false, true, true, false, false, false, true, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, true, false, true, true, true, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, false, false, false, false, true, false, true, true, false, false, false, true, false, true, true, true, true, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, true, false, true, true, true, true, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, true, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, true, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, false, false, true, false, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, false, false, true, true, false, false, false, true, true, true, true, true, false, false, false, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true, true, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, true, true, true, false, false, false, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true, true, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, true, true, false, true, true, false, true, true, false, false, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, true, false, false, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, false, true, false, true, false, false, true, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, true, true, false, true, false, false, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, false, false, false, true, false, false, true, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, true, true, false, true, false, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, false, true, false, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, true, false, true, true, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, false, true, false, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, true, false, true, true, false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, true, true, true, true, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, false, false, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, false, false, true, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, true, false, true, false, true, false, false, true, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, true, false, true, false, true, false, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, true, true, false, false, true, true, false, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, true, false, false, true, false, true, true, false, true, false, true, false, true, true, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, true, true, true, false, true, true, false, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, true, false, false, true, false, true, true, false, true, false, true, false, true, true, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, false, false, false, false, true, true, true, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, false, false, true, true, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, false, false, true, true, false, true, false, false, false, false, false, true, false, true, false, true, true, false, false, false, false, true, false, true, false, false, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, false, true, false, true, true, true, true, true, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, false, true, true, true, false, true, true, true, false, true, false, true, true, false, true, false, false, false, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, true, true, true, false, true, true, true, false, true, false, true, true, true, false, true, true, true, false, true, true, true, false, true, false, true, true, false, true, false, false, false, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, true, false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, true, false, false, true, true, false, false, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, true, false, false, false, true, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, true, false, false, true, true, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, false, true, false, true, false, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false, false, true, false, true, true, true, true, true, false, true, true, true, true, false, true, false, false, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, false, true, false, true, false, true, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false, false, true, false, true, true, true, true, true, false, true, true, true, true, false, true, false, false, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, false, false, true, false, false, true, true, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, false, true, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, false, false, true, false, false, true, true, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, false, true, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, true, true, true, true, false, false, true, false, true, true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, true, true, false, true, false, true, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, true, true, false, true, false, true, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, false, true, true, true, true, false, true, true, true, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, false, true, true, true, false, false, false, true, false, false, false, false, true, true, false, false, false, true, false, false, false, true, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, false, true, true, true, true, false, true, true, true, true, false, false, false, false, false, true, true, true, false, true, false, true, false, true, false, true, true, true, false, false, false, true, false, false, false, false, true, true, false, false, false, true, false, false, false, true, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, false, false, false, true, true, true, true, true, false, true, true, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, false, false, true, true, true, true, true, true, false, true, true, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, false, false, false, true, false, false, true, false, true, true, false, false, true, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, true, false, true, false, true, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, false, false, false, true, false, false, true, false, true, true, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, true, false, true, false, true, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, false, true, false, false, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false, false, true, true, false, true, false, true, true, true, false, false, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false, false, true, true, false, true, false, true, true, true, false, false, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, true, true, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, true, false, false, true, false, false, true, false, false, false, true, false, true, true, false, true, true, true, false, false, false, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, true, false, false, true, false, false, true, false, false, false, true, false, true, true, false, true, true, true, false, false, false, true, true, false, false, false, true, false, false, true, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, false, false, false, true, false, true, true, true, true, false, false, false, true, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, false, false, false, true, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, true, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, true, true, false, true, true, true, false, false, false, true, false, true, true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, false, false, false, true, false, true, false, false, true, true, true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, true, true, true, false, false, false, true, false, true, false, false, true, true, true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, true, false, true, true, true, true, false, false, false, true, false, true, false, false, false, false, true, true, false, true, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, false, false, true, false, true, true, true, true, true, true, false, false, false, true, false, true, false, false, false, false, true, true, false, true, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, true, true, false, true, true, false, false, false, true, false, true, true, false, true, false, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, true, true, false, true, true, true, false, false, true, false, true, true, false, true, false, true, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, false, false, true, true, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, false, false, true, true, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, false, true, true, false, false, false, false, false, true, true, false, true, true, false, false, false, false, true, false, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, false, true, true, false, false, false, false, false, false, true, false, true, true, false, false, false, false, true, false, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, false, false, true, true, false, false, true, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, true, true, false, true, true, true, false, true, true, false, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, false, true, true, false, false, true, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, true, true, false, true, true, true, false, true, true, false, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, true, false, true, true, false, false, true, true, true, false, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, false, false, false, false, false, false, false, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, true, true, false, false, true, true, true, false, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false, false, true, true, false, false, false, false, false, false, false, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, true, true, true, true, true, true, false, false, true, false, false, false, false, false, false, false, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true, true, false, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, false, false, true, false, false, false, false, false, false, false, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, true, false, true, true, true, true, true, false, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true, true, false, true, true, true, true, true, false, true, true, true, true, true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, true, false, true, false, true, true, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, true, false, true, false, true, true, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, false, true, true, true, false, false, true, false, true, true, false, false, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, false, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true, false, false, true, true, true, true, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, false, false, true, true, true, false, true, false, false, false, true, true, false, true, false, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, true, true, true, true, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, true, true, true, true, false, false, false, true, true, true, true, false, true, false, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, true, false, true, false, true, false, true, false, false, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, true, false, false, true, true, false, false, false, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, false, false, true, true, false, false, true, false, true, false, true, false, true, false, false, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, true, false, false, true, true, false, false, false, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, true, true, false, false, false, false, false, true, false, true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, true, true, false, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, false, true, false, true, false, true, false, false, true, false, false, true, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, true, false, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, true, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, true, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, false, true, false, false, true, true, false, false, false, false, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, true, true, false, false, false, false, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true, false, false, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true, false, false, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, false, true, true, false, false, false, false, false, false, true, true, false, false, true, true, false, false, false, false, false, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, false, true, true, false, false, true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, false, true, false, false, true, false, false, false, true, true, true, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, false, true, false, false, true, false, false, false, true, false, true, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, true, true, false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, false, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, true, false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, false, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, false, false, false, true, false, true, false, true, false, false, false, true, false, false, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, false, false, false, true, false, true, false, true, false, false, false, true, false, false, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, false, false, false, true, false, true, true, false, true, true, false, true, true, false, false, false, true, true, false, false, true, false, false, true, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, false, false, false, true, false, true, true, false, true, true, false, true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, true, true, false, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, true, false, true, false, false, true, true, true, false, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, false, false, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, false, false, true, true, true, true, false, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, false, true, true, false, true, false, true, true, false, false, true, false, true, false, false, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, false, false, false, false, true, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, false, false, false, false, true, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, true, false, false, true, false, false, true, true, false, false, false, true, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, true, false, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, true, false, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, true, false, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, true, false, false, true, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, true, false, false, true, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true, false, false, true, true, true, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, true, false, false, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, false, true, false, true, false, false, true, false, true, true, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, false, true, false, true, false, false, true, false, true, true, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, false, true, false, false, false, false, true, false, false, false, true, true, false, false, false, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, true, false, true, true, false, false, false, true, false, false, false, true, true, false, false, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, false, true, true, false, true, true, false, true, true, false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, true, true, true, true, true, false, false, true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, false, true, true, true, true, false, false, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, true, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, false, true, true, true, true, false, false, true, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, false, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, false, true, false, false, true, true, true, false, true, true, false, true, true, true, false, true, true, true, false, true, false, false, false, true, true, false, true, false, true, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, false, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, false, true, false, false, true, true, true, false, true, true, false, true, true, true, false, true, true, true, false, true, false, false, false, true, true, false, true, false, true, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, false, true, false, false, false, true, true, true, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, true, false, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, false, true, false, false, false, true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, true, false, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, true, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, true, false, true, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, false, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, false, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true, false, false, false, false, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true, false, false, false, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, true, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, true, false, false, false, false, true, false, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, false, false, false, true, false, true, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, false, false, false, true, false, true, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, false, true, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, true, true, false, true, false, true, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, true, true, false, true, false, true, true, true, false, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, false, true, true, true, true, false, true, true, false, false, true, false, false, false, true, false, false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, false, true, true, true, true, false, true, true, false, false, true, false, false, false, true, false, false, true, false, false, false, false, true, true, false, true, true, false, false, true, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, false, false, false, true, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, false, false, true, true, false, true, true, false, true, false, false, true, false, true, false, true, true, false, true, true, false, false, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, false, false, true, true, false, true, true, false, true, false, false, true, false, true, false, false, true, false, true, true, false, false, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, false, false, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, false, false, false, true, false, true, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, true, false, true, false, false, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, false, true, true, false, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, false, true, true, true, false, false, true, true, false, false, true, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, false, false, true, true, false, false, true, true, false, false, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, false, true, false, true, true, true, false, true, true, true, false, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, false, true, false, true, true, false, false, true, false, true, false, false, true, false, false, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, false, true, false, true, true, false, false, true, false, true, false, false, true, false, false, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, true, false, false, true, false, true, false, true, true, false, false, true, false, true, true, false, false, false, true, false, true, true, false, true, false, false, false, true, true, false, true, true, true, false, true, true, false, true, false, false, true, false, false, true, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, true, false, false, true, false, true, false, true, true, false, false, true, false, true, true, false, false, false, true, false, true, true, false, true, false, false, false, true, true, false, true, true, true, false, true, true, false, true, false, false, true, false, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, false, false, false, false, false, true, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, false, false, false, false, false, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, true, true, false, true, true, false, true, true, true, false, false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, true, false, false, false, false, false, true, false, true, false, false, true, true, false, false, true, false, false, true, false, true, true, true, false, false, true, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false, false, false, true, true, false, false, true, false, false, true, false, true, true, true, false, false, true, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, true, false, true, false, false, true, false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, false, true, false, true, true, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, true, true, false, false, true, false, false, true, true, false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, false, false, true, true, false, false, false, true, true, false, true, true, false, false, true, true, false, true, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, true, true, false, false, true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, true, true, true, false, true, true, false, true, false, false, false, true, true, false, false, false, true, true, false, true, true, false, false, true, true, false, true, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, true, false, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, true, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, true, true, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, true, false, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, true, true, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, true, true, true, true, false, false, false, false, true, true, true, false, true, true, false, true, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, true, true, true, true, false, false, false, false, true, true, true, false, true, true, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, true, false, true, false, true, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, true, false, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, true, true, false, true, false, true, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, true, false, true, true, false, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, false, false, true, false, true, true, false, false, true, false, false, true, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, true, false, true, true, false, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

}
