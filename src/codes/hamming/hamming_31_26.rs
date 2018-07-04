use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;


pub struct HammingCode31_26;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 31]> = 0 as *const FnvHashMap<usize, [bool; 31]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]),
                BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]),
                BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, true]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, true, true, false, true]),
                BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, false, false, true, false, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, false, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(32, Default::default()));
            map.insert(0, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 0 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 1 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 2 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(3, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 3 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(4, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 4 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(5, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 5 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(6, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 6 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(7, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 7 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(8, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 8 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(9, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 9 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(10, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 10 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(11, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 11 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(12, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 12 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(13, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 13 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(14, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 14 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(15, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 15 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(16, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 16 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(17, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 17 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(18, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 18 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(19, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 19 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(20, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 20 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(21, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 21 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(22, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 22 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(23, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 23 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(24, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 24 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(25, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 25 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(26, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 26 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(27, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 27 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(28, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 28 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(29, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 29 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(30, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 30 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(31, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 31 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode for HammingCode31_26 {
    fn name(&self) -> String {
        "[31, 26] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        31
    }

    fn dimension(&self) -> usize {
        26
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
        codeword.truncate(26);
        Ok(codeword)
        
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1f64 + (31 as f64) * delta) / ((31 + 1) as f64)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode31_26.generator_matrix();
        assert_eq!(code.ncols(), 31);
        assert_eq!(code.nrows(), 26);
    }

    #[test]
    fn decode() {
        let code = HammingCode31_26;

        let codeword = code.decode_to_message(&BinVector::from_elem(31, true)).unwrap();
        assert_eq!(codeword, BinVector::from_elem(26, true));

        let mut vec = BinVector::from_elem(31, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(&vec).unwrap();
        assert_eq!(codeword, BinVector::from_elem(26, true));

        let vec = code.decode_to_code(&BinVector::from_elem(31, false)).unwrap();
        assert_eq!(vec, BinVector::from_elem(31, false));
    }

    #[test]
    fn encode_decode_tests() {

        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, true, true, false, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, true, true, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, true, true, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, false, false, true, false, false, true, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, false, false, true, false, false, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, false, true, true, false, true, true, true, false, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, false, true, true, false, true, true, true, false, false, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, true, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, true, false, false, true, false, true, true, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, false, true, true, true, false, false, false, true, false, true, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, false, true, true, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true, true, false, true, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, true, false, true, false, false, true, false, false, true, false, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, false, true, true, false, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true, false, true, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true, false, true, true, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, false, true, true, true, true, false, true, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, true, false, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, true, true, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true, false, true, false, false, true, true, false, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
        {
            let code = HammingCode31_26;
            let m = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
        }
        
    }

}
