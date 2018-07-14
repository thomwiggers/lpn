use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;


#[derive(Clone)]
pub struct GolayCode23_12;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 23]> = 0 as *const FnvHashMap<usize, [bool; 23]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, true, true, false, false, false, true]),
                BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, true]),
                BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, true, true, false, false, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, true, true, false, false, false, true, true]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, true, false]),
                BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, true]),
                BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, true]),
                BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, true, false, false, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, true, true]),
                BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, true, false, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(2048, Default::default()));
            map.insert(0, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 0 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 1 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 2 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(3, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 3 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(4, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 4 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(5, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 5 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(6, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 6 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(7, [true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 7 => (1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(8, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 8 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(9, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 9 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(10, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 10 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(11, [true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 11 => (1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(12, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 12 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(13, [true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 13 => (1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(14, [false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 14 => (0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(15, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 15 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(16, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 16 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(17, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 17 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(18, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 18 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(19, [true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 19 => (1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(20, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 20 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(21, [true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 21 => (1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(22, [false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 22 => (0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(23, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true]); // 23 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(24, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 24 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(25, [true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 25 => (1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(26, [false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 26 => (0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(27, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false]); // 27 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(28, [false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 28 => (0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(29, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 29 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(30, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 30 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(31, [false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 31 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(32, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 32 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(33, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 33 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(34, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 34 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(35, [true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 35 => (1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(36, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 36 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(37, [true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 37 => (1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(38, [false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 38 => (0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(39, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 39 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(40, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 40 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(41, [true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 41 => (1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(42, [false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 42 => (0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(43, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 43 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(44, [false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 44 => (0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(45, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 45 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(46, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 46 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(47, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 47 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(48, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 48 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(49, [true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 49 => (1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(50, [false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 50 => (0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(51, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 51 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(52, [false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 52 => (0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(53, [false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 53 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(54, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false]); // 54 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(55, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 55 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(56, [false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 56 => (0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(57, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false]); // 57 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(58, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 58 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(59, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 59 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(60, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 60 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(61, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true]); // 61 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1)
            map.insert(62, [false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 62 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(63, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 63 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(64, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 64 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(65, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 65 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(66, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 66 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(67, [true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 67 => (1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(68, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 68 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(69, [true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 69 => (1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(70, [false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 70 => (0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(71, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 71 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(72, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 72 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(73, [true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 73 => (1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(74, [false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 74 => (0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(75, [false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 75 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(76, [false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 76 => (0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(77, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false]); // 77 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(78, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 78 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(79, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 79 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(80, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 80 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(81, [true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 81 => (1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(82, [false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 82 => (0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(83, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 83 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(84, [false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 84 => (0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(85, [false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 85 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(86, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 86 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(87, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 87 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(88, [false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 88 => (0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(89, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 89 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(90, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 90 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(91, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 91 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(92, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 92 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(93, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false]); // 93 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(94, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 94 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(95, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 95 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(96, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 96 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(97, [true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 97 => (1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(98, [false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 98 => (0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(99, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true]); // 99 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1)
            map.insert(100, [false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 100 => (0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(101, [false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 101 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(102, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 102 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(103, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false]); // 103 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(104, [false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 104 => (0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(105, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 105 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(106, [false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 106 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(107, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 107 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(108, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true]); // 108 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(109, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 109 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(110, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false]); // 110 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(111, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 111 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(112, [false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 112 => (0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(113, [false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 113 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(114, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false]); // 114 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(115, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 115 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(116, [true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 116 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(117, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 117 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(118, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 118 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(119, [false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 119 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(120, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 120 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(121, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 121 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(122, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 122 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(123, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 123 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(124, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 124 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(125, [false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 125 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(126, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 126 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(127, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 127 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(128, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 128 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(129, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 129 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(130, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 130 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(131, [true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 131 => (1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(132, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 132 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(133, [true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 133 => (1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(134, [false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 134 => (0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(135, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 135 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(136, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 136 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(137, [true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 137 => (1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(138, [false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 138 => (0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(139, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 139 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(140, [false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 140 => (0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(141, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false]); // 141 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(142, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 142 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(143, [false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 143 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(144, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 144 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(145, [true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 145 => (1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(146, [false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 146 => (0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(147, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 147 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(148, [false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 148 => (0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(149, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false]); // 149 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0)
            map.insert(150, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 150 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(151, [false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 151 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(152, [false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 152 => (0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(153, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 153 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(154, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false]); // 154 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(155, [false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 155 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(156, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 156 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(157, [false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 157 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(158, [true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 158 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(159, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 159 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(160, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 160 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(161, [true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 161 => (1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(162, [false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 162 => (0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(163, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 163 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(164, [false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 164 => (0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(165, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 165 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(166, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false]); // 166 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(167, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false]); // 167 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(168, [false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 168 => (0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(169, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false]); // 169 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(170, [false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 170 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(171, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true]); // 171 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(172, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 172 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(173, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 173 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(174, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 174 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(175, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 175 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(176, [false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 176 => (0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(177, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 177 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(178, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 178 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(179, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 179 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(180, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 180 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(181, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false]); // 181 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(182, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 182 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(183, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 183 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(184, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 184 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(185, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 185 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(186, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false]); // 186 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(187, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 187 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(188, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 188 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(189, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 189 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(190, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 190 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(191, [false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 191 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(192, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 192 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(193, [true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 193 => (1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(194, [false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 194 => (0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(195, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 195 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(196, [false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 196 => (0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(197, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 197 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(198, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 198 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(199, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 199 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(200, [false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 200 => (0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(201, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 201 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(202, [false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 202 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(203, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false]); // 203 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0)
            map.insert(204, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 204 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(205, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 205 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(206, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false]); // 206 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(207, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 207 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(208, [false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 208 => (0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(209, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 209 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(210, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false]); // 210 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(211, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false]); // 211 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(212, [false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 212 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(213, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 213 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(214, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 214 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(215, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 215 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(216, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 216 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(217, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 217 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(218, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 218 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(219, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 219 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(220, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false]); // 220 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(221, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 221 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(222, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 222 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(223, [false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 223 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(224, [false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 224 => (0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(225, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false]); // 225 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(226, [false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 226 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(227, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 227 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(228, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false]); // 228 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(229, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 229 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(230, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 230 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(231, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 231 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(232, [false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 232 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(233, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 233 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(234, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 234 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(235, [true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 235 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(236, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 236 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(237, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 237 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(238, [false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 238 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(239, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 239 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(240, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 240 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(241, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false]); // 241 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(242, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 242 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(243, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 243 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(244, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 244 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(245, [false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 245 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(246, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 246 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(247, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 247 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(248, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false]); // 248 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(249, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 249 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(250, [false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 250 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(251, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 251 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(252, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 252 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(253, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 253 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(254, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 254 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(255, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 255 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(256, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 256 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(257, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 257 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(258, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 258 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(259, [true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 259 => (1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(260, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 260 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(261, [true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 261 => (1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(262, [false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 262 => (0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(263, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false]); // 263 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(264, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 264 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(265, [true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 265 => (1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(266, [false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 266 => (0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(267, [false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 267 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(268, [false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 268 => (0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(269, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 269 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(270, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 270 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(271, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false]); // 271 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0)
            map.insert(272, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 272 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(273, [true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 273 => (1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(274, [false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 274 => (0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(275, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 275 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(276, [false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 276 => (0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(277, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 277 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(278, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false]); // 278 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(279, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 279 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(280, [false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 280 => (0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(281, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 281 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(282, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false]); // 282 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(283, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true]); // 283 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1)
            map.insert(284, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 284 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(285, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 285 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(286, [false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 286 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(287, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 287 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(288, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 288 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(289, [true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 289 => (1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(290, [false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 290 => (0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(291, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 291 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(292, [false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 292 => (0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(293, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false]); // 293 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(294, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 294 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(295, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true]); // 295 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(296, [false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 296 => (0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(297, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true]); // 297 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(298, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false]); // 298 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0)
            map.insert(299, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 299 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(300, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false]); // 300 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(301, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 301 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(302, [false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 302 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(303, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 303 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(304, [false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 304 => (0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(305, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 305 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(306, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 306 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(307, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 307 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(308, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true]); // 308 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(309, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false]); // 309 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(310, [false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 310 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(311, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 311 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(312, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 312 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(313, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false]); // 313 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(314, [false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 314 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(315, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 315 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(316, [false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 316 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(317, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 317 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(318, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 318 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(319, [true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 319 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(320, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 320 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(321, [true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 321 => (1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(322, [false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 322 => (0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(323, [false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 323 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(324, [false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 324 => (0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(325, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true]); // 325 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1)
            map.insert(326, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 326 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(327, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 327 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(328, [false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 328 => (0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(329, [false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 329 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(330, [true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 330 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(331, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 331 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(332, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false]); // 332 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(333, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 333 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(334, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false]); // 334 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(335, [false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 335 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(336, [false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 336 => (0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(337, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false]); // 337 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0)
            map.insert(338, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true]); // 338 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(339, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 339 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(340, [false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 340 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(341, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 341 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(342, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 342 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(343, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 343 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(344, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 344 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(345, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false]); // 345 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(346, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 346 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(347, [false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 347 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(348, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true]); // 348 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(349, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 349 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(350, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 350 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(351, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 351 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(352, [false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 352 => (0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(353, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false]); // 353 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(354, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 354 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(355, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false]); // 355 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(356, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 356 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(357, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 357 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(358, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 358 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(359, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 359 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(360, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 360 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(361, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 361 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(362, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true]); // 362 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(363, [false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 363 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(364, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 364 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(365, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 365 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(366, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 366 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(367, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 367 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(368, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 368 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(369, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 369 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(370, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false]); // 370 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(371, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 371 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(372, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false]); // 372 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0)
            map.insert(373, [false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 373 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(374, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 374 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(375, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 375 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(376, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 376 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(377, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 377 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(378, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 378 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(379, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 379 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(380, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 380 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(381, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 381 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(382, [false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 382 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(383, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true]); // 383 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(384, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 384 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(385, [true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 385 => (1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(386, [false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 386 => (0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(387, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 387 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(388, [false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 388 => (0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(389, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false]); // 389 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(390, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 390 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(391, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 391 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(392, [false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 392 => (0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(393, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false]); // 393 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(394, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 394 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(395, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false]); // 395 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(396, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 396 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(397, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 397 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(398, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 398 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(399, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 399 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(400, [false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 400 => (0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(401, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 401 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(402, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 402 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(403, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 403 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(404, [false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 404 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(405, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true]); // 405 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(406, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false]); // 406 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0)
            map.insert(407, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 407 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(408, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false]); // 408 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(409, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false]); // 409 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(410, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 410 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(411, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 411 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(412, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false]); // 412 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(413, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 413 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(414, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 414 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(415, [false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 415 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(416, [false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 416 => (0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(417, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 417 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(418, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 418 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(419, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 419 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(420, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false]); // 420 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(421, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 421 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(422, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false]); // 422 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(423, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 423 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(424, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 424 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(425, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 425 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(426, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false]); // 426 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(427, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 427 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(428, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 428 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(429, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 429 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(430, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 430 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(431, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 431 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(432, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 432 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(433, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 433 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(434, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 434 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(435, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 435 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(436, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 436 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(437, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 437 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(438, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 438 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(439, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 439 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(440, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true]); // 440 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(441, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 441 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(442, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 442 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(443, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 443 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(444, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 444 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(445, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 445 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(446, [false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 446 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(447, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 447 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(448, [false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 448 => (0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(449, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 449 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(450, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false]); // 450 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(451, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true]); // 451 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(452, [false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 452 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(453, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 453 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(454, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false]); // 454 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(455, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 455 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(456, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false]); // 456 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0)
            map.insert(457, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 457 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(458, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 458 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(459, [false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 459 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(460, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 460 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(461, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 461 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(462, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 462 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(463, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 463 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(464, [false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 464 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(465, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false]); // 465 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(466, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 466 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(467, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 467 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(468, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 468 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(469, [true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 469 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(470, [false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 470 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(471, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 471 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(472, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 472 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(473, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 473 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(474, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 474 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(475, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 475 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(476, [false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 476 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(477, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 477 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(478, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 478 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(479, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false]); // 479 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(480, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true]); // 480 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(481, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 481 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(482, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false]); // 482 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(483, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 483 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(484, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 484 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(485, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 485 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(486, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 486 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(487, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 487 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(488, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 488 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(489, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 489 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(490, [false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 490 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(491, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 491 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(492, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 492 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(493, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 493 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(494, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 494 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(495, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 495 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(496, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false]); // 496 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(497, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 497 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(498, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 498 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(499, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 499 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(500, [false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 500 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(501, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 501 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(502, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 502 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(503, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 503 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(504, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 504 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(505, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 505 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(506, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 506 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(507, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 507 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(508, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 508 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(509, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 509 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(510, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false]); // 510 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(511, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 511 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(512, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 512 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(513, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 513 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(514, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 514 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(515, [true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 515 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(516, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 516 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(517, [true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 517 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(518, [false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 518 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(519, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false]); // 519 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(520, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 520 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(521, [true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 521 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(522, [false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 522 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(523, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 523 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(524, [false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 524 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(525, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 525 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(526, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false]); // 526 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(527, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 527 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(528, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 528 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(529, [true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 529 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(530, [false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 530 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(531, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 531 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(532, [false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 532 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(533, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 533 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(534, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 534 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(535, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 535 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(536, [false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 536 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(537, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 537 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(538, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 538 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(539, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false]); // 539 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(540, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 540 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(541, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 541 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(542, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false]); // 542 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0)
            map.insert(543, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 543 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(544, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 544 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(545, [true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 545 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(546, [false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 546 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(547, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false]); // 547 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(548, [false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 548 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(549, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 549 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(550, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 550 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(551, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 551 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(552, [false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 552 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(553, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 553 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(554, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 554 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(555, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 555 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(556, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false]); // 556 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(557, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false]); // 557 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(558, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 558 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(559, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false]); // 559 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(560, [false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 560 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(561, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false]); // 561 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(562, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 562 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(563, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 563 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(564, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false]); // 564 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(565, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 565 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(566, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 566 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(567, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 567 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(568, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 568 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(569, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 569 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(570, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 570 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(571, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 571 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(572, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 572 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(573, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 573 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(574, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 574 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(575, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 575 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(576, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 576 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(577, [true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 577 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(578, [false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 578 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(579, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 579 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(580, [false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 580 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(581, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false]); // 581 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(582, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 582 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(583, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 583 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(584, [false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 584 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(585, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 585 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(586, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false]); // 586 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(587, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 587 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(588, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 588 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(589, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 589 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(590, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 590 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(591, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 591 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(592, [false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 592 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(593, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 593 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(594, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 594 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(595, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 595 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(596, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true]); // 596 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1)
            map.insert(597, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 597 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(598, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false]); // 598 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(599, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 599 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(600, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false]); // 600 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(601, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false]); // 601 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(602, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 602 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(603, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 603 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(604, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 604 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(605, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 605 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(606, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false]); // 606 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(607, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 607 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(608, [false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 608 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(609, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 609 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(610, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 610 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(611, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 611 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(612, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 612 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(613, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false]); // 613 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0)
            map.insert(614, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 614 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(615, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 615 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(616, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 616 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(617, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 617 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(618, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false]); // 618 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(619, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 619 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(620, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 620 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(621, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 621 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(622, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 622 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(623, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 623 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(624, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 624 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(625, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 625 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(626, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false]); // 626 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(627, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 627 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(628, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 628 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(629, [false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false]); // 629 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(630, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 630 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(631, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 631 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(632, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 632 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(633, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 633 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(634, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 634 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(635, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false]); // 635 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(636, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 636 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(637, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 637 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(638, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 638 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(639, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 639 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(640, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 640 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(641, [true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 641 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(642, [false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 642 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(643, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 643 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(644, [false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 644 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(645, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 645 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(646, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 646 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(647, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 647 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(648, [false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 648 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(649, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 649 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(650, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 650 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(651, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 651 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(652, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 652 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(653, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false]); // 653 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(654, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 654 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(655, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 655 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(656, [false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 656 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(657, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false]); // 657 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(658, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 658 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(659, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true]); // 659 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1)
            map.insert(660, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 660 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(661, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 661 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(662, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 662 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(663, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 663 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(664, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false]); // 664 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(665, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 665 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(666, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 666 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(667, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 667 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(668, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true]); // 668 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(669, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 669 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(670, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 670 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(671, [false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false]); // 671 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(672, [false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 672 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(673, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 673 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(674, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false]); // 674 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0)
            map.insert(675, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 675 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(676, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 676 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(677, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 677 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(678, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 678 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(679, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 679 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(680, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 680 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(681, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 681 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(682, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 682 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(683, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 683 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(684, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 684 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(685, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 685 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(686, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 686 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(687, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 687 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(688, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 688 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(689, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 689 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(690, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false]); // 690 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(691, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 691 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(692, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 692 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(693, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 693 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(694, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 694 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(695, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 695 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(696, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 696 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(697, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 697 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(698, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 698 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(699, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 699 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(700, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false]); // 700 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(701, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 701 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(702, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 702 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(703, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 703 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(704, [false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 704 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(705, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 705 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(706, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true]); // 706 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(707, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 707 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(708, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 708 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(709, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 709 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(710, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false]); // 710 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(711, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false]); // 711 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(712, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 712 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(713, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false]); // 713 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(714, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 714 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(715, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 715 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(716, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 716 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(717, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 717 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(718, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 718 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(719, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 719 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(720, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 720 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(721, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 721 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(722, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 722 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(723, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 723 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(724, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 724 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(725, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 725 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(726, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 726 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(727, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 727 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(728, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 728 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(729, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true]); // 729 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(730, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 730 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(731, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 731 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(732, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 732 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(733, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 733 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(734, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 734 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(735, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 735 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(736, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 736 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(737, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 737 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(738, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 738 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(739, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 739 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(740, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false]); // 740 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(741, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 741 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(742, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 742 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(743, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 743 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(744, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true]); // 744 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1)
            map.insert(745, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 745 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(746, [false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false]); // 746 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(747, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 747 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(748, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 748 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(749, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 749 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(750, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 750 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(751, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 751 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(752, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 752 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(753, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 753 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(754, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 754 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(755, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 755 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(756, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 756 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(757, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 757 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(758, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true]); // 758 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(759, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 759 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(760, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 760 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(761, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 761 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(762, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 762 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(763, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 763 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(764, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 764 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(765, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 765 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(766, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 766 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(767, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 767 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(768, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 768 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(769, [true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 769 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(770, [false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 770 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(771, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true]); // 771 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(772, [false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 772 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(773, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 773 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(774, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 774 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(775, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 775 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(776, [false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 776 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(777, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 777 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(778, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false]); // 778 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(779, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 779 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(780, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 780 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(781, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 781 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(782, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false]); // 782 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(783, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 783 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(784, [false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 784 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(785, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 785 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(786, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false]); // 786 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(787, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false]); // 787 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(788, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 788 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(789, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 789 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(790, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true]); // 790 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(791, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 791 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(792, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 792 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(793, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 793 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(794, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 794 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(795, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 795 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(796, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 796 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(797, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 797 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(798, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 798 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(799, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 799 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(800, [false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 800 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(801, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false]); // 801 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(802, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 802 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(803, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 803 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(804, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 804 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(805, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false]); // 805 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(806, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 806 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(807, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 807 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(808, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 808 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(809, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 809 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(810, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 810 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(811, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 811 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(812, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true]); // 812 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1)
            map.insert(813, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 813 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(814, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 814 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(815, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 815 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(816, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false]); // 816 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(817, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 817 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(818, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false]); // 818 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(819, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 819 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(820, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 820 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(821, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 821 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(822, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 822 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(823, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false]); // 823 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(824, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false]); // 824 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0)
            map.insert(825, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 825 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(826, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true]); // 826 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(827, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 827 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(828, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 828 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(829, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 829 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(830, [false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false]); // 830 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(831, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 831 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(832, [false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 832 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(833, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 833 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(834, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 834 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(835, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false]); // 835 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0)
            map.insert(836, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 836 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(837, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false]); // 837 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(838, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 838 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(839, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 839 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(840, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true]); // 840 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(841, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false]); // 841 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(842, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 842 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(843, [false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false]); // 843 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(844, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false]); // 844 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(845, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 845 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(846, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 846 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(847, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true]); // 847 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(848, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false]); // 848 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(849, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 849 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(850, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 850 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(851, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false]); // 851 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(852, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false]); // 852 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(853, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 853 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(854, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 854 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(855, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false]); // 855 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(856, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 856 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(857, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 857 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(858, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 858 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(859, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 859 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(860, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 860 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(861, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 861 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(862, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 862 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(863, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 863 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(864, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 864 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(865, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 865 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(866, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 866 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(867, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 867 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(868, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 868 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(869, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 869 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(870, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 870 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(871, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 871 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(872, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false]); // 872 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(873, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false]); // 873 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(874, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 874 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(875, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 875 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(876, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 876 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(877, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 877 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(878, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 878 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(879, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 879 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(880, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 880 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(881, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 881 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(882, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 882 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(883, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 883 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(884, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 884 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(885, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 885 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(886, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 886 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(887, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 887 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(888, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false]); // 888 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(889, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 889 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(890, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 890 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(891, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false]); // 891 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(892, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false]); // 892 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(893, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 893 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(894, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 894 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(895, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false]); // 895 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(896, [false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 896 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(897, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 897 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(898, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false]); // 898 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(899, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 899 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(900, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false]); // 900 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0)
            map.insert(901, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 901 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(902, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 902 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(903, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 903 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(904, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 904 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(905, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true]); // 905 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(906, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 906 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(907, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false]); // 907 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(908, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false]); // 908 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(909, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 909 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(910, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true]); // 910 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(911, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 911 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(912, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true]); // 912 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1)
            map.insert(913, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false]); // 913 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(914, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 914 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(915, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 915 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(916, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 916 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(917, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 917 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(918, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false]); // 918 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(919, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 919 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(920, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 920 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(921, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 921 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(922, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 922 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(923, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 923 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(924, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 924 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(925, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 925 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(926, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 926 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(927, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 927 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(928, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 928 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(929, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false]); // 929 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0)
            map.insert(930, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true]); // 930 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(931, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 931 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(932, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 932 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(933, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true]); // 933 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(934, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 934 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(935, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 935 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(936, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 936 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(937, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 937 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(938, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 938 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(939, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false]); // 939 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(940, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 940 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(941, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 941 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(942, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false]); // 942 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(943, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false]); // 943 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(944, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 944 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(945, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 945 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(946, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 946 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(947, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 947 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(948, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 948 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(949, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 949 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(950, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 950 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(951, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 951 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(952, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 952 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(953, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false]); // 953 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(954, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 954 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(955, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 955 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(956, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 956 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(957, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 957 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(958, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false]); // 958 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(959, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true]); // 959 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(960, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 960 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(961, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 961 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(962, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 962 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(963, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 963 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(964, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true]); // 964 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(965, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 965 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(966, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 966 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(967, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false]); // 967 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(968, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 968 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(969, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 969 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(970, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 970 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(971, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 971 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(972, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false]); // 972 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(973, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 973 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(974, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 974 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(975, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false]); // 975 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(976, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 976 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(977, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 977 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(978, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 978 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(979, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 979 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(980, [false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false]); // 980 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(981, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false]); // 981 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(982, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 982 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(983, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true]); // 983 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(984, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 984 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(985, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 985 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(986, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 986 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(987, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 987 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(988, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 988 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(989, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 989 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(990, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 990 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(991, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 991 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(992, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false]); // 992 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(993, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false]); // 993 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(994, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 994 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(995, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false]); // 995 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(996, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 996 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(997, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 997 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(998, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 998 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(999, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 999 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1000, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false]); // 1000 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1001, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 1001 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1002, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1002 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1003, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true]); // 1003 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1)
            map.insert(1004, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 1004 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1005, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 1005 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1006, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 1006 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1007, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 1007 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1008, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false]); // 1008 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1009, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]); // 1009 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1010, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1010 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1011, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 1011 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1012, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false]); // 1012 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1013, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false]); // 1013 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1014, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 1014 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1015, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false]); // 1015 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1016, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1016 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1017, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false]); // 1017 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1018, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1018 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1019, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1019 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1020, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true]); // 1020 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1021, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 1021 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1022, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 1022 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1023, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false]); // 1023 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1024, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1024 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1025, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1025 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1026, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1026 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1027, [true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1027 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1028, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1028 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1029, [true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1029 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1030, [false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1030 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1031, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1031 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1032, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1032 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1033, [true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1033 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1034, [false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1034 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1035, [false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1035 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1036, [false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1036 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1037, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true]); // 1037 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1)
            map.insert(1038, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false]); // 1038 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(1039, [false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1039 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1040, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1040 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1041, [true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1041 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1042, [false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1042 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1043, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false]); // 1043 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1044, [false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1044 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1045, [false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1045 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1046, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1046 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1047, [false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1047 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1048, [false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1048 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1049, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false]); // 1049 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1050, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1050 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1051, [false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1051 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1052, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false]); // 1052 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1053, [false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1053 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1054, [true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1054 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1055, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1055 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1056, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1056 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1057, [true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1057 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1058, [false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1058 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1059, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false]); // 1059 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(1060, [false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1060 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1061, [false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1061 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1062, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1062 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1063, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1063 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1064, [false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1064 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1065, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1065 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1066, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1066 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1067, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false]); // 1067 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1068, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1068 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1069, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1069 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1070, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1070 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1071, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false]); // 1071 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1072, [false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1072 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1073, [false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1073 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1074, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1074 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1075, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true, false, false]); // 1075 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(1076, [true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1076 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1077, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1077 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1078, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false]); // 1078 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1079, [false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1079 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1080, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1080 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1081, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1081 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1082, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1082 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1083, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1083 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1084, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false]); // 1084 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0)
            map.insert(1085, [false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1085 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1086, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1086 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1087, [false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1087 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1088, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1088 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1089, [true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1089 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1090, [false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1090 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1091, [false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1091 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1092, [false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1092 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1093, [false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1093 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1094, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true]); // 1094 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1095, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false]); // 1095 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0)
            map.insert(1096, [false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1096 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1097, [false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1097 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1098, [true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1098 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1099, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1099 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1100, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1100 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1101, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false]); // 1101 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1102, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1102 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1103, [false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1103 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1104, [false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1104 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1105, [false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1105 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1106, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1106 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1107, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true]); // 1107 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1108, [true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1108 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1109, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1109 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1110, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1110 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1111, [false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1111 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1112, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false]); // 1112 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(1113, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1113 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1114, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false]); // 1114 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1115, [false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1115 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1116, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1116 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1117, [false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1117 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1118, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false]); // 1118 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1119, [false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1119 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1120, [false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1120 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1121, [false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1121 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1122, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false]); // 1122 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1123, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1123 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1124, [true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1124 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1125, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1125 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1126, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1126 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1127, [false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1127 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1128, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false]); // 1128 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(1129, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true]); // 1129 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(1130, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1130 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1131, [false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1131 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1132, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1132 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1133, [false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1133 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1134, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1134 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1135, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1135 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1136, [true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1136 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1137, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1137 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1138, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1138 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1139, [false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1139 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1140, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1140 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1141, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1141 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1142, [true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1142 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1143, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1143 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1144, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1144 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1145, [false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1145 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1146, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1146 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1147, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false]); // 1147 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(1148, [true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1148 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1149, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1149 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1150, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1150 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1151, [false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1151 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1152, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1152 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1153, [true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1153 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1154, [false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1154 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1155, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true]); // 1155 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(1156, [false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1156 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1157, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1157 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1158, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1158 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1159, [false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1159 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1160, [false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1160 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1161, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1161 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1162, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false]); // 1162 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1163, [false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1163 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1164, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1164 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1165, [false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1165 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1166, [true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1166 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1167, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1167 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1168, [false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1168 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1169, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1169 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1170, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1170 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1171, [false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1171 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1172, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true]); // 1172 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1173, [false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1173 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1174, [true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1174 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1175, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1175 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1176, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1176 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1177, [false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1177 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1178, [true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1178 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1179, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1179 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1180, [true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1180 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1181, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1181 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1182, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1182 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1183, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1183 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1184, [false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1184 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1185, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1185 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1186, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1186 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1187, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1187 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1188, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1188 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1189, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false]); // 1189 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1190, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1190 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1191, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1191 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1192, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1192 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1193, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1193 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1194, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1194 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1195, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1195 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1196, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true]); // 1196 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1197, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1197 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1198, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1198 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1199, [false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1199 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1200, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false]); // 1200 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1201, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true]); // 1201 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1)
            map.insert(1202, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false]); // 1202 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(1203, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1203 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1204, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1204 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1205, [false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1205 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1206, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1206 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1207, [false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1207 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1208, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1208 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1209, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1209 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1210, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1210 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1211, [false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1211 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1212, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false]); // 1212 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(1213, [false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1213 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1214, [true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1214 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1215, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1215 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1216, [false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1216 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1217, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false]); // 1217 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1218, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1218 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1219, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1219 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1220, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1220 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1221, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true]); // 1221 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(1222, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1222 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1223, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false]); // 1223 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(1224, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1224 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1225, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false]); // 1225 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(1226, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true]); // 1226 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1)
            map.insert(1227, [false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1227 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1228, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1228 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1229, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1229 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1230, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1230 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1231, [false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1231 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1232, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1232 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1233, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1233 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1234, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1234 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1235, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1235 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1236, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false]); // 1236 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(1237, [false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1237 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1238, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1238 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1239, [false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1239 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1240, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1240 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1241, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true]); // 1241 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1242, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1242 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1243, [false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1243 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1244, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1244 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1245, [false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1245 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1246, [true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1246 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1247, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1247 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1248, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1248 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1249, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1249 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1250, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1250 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1251, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1251 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1252, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false]); // 1252 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(1253, [false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1253 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1254, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1254 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1255, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1255 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1256, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1256 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1257, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1257 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1258, [false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false]); // 1258 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1259, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1259 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1260, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1260 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1261, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1261 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1262, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1262 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1263, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true]); // 1263 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1264, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1264 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1265, [false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1265 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1266, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1266 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1267, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1267 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1268, [true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1268 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1269, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1269 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1270, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true]); // 1270 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(1271, [false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1271 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1272, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1272 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1273, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1273 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1274, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1274 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1275, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false]); // 1275 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0)
            map.insert(1276, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1276 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1277, [false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1277 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1278, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1278 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1279, [false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1279 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1280, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1280 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1281, [true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1281 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1282, [false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1282 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1283, [false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1283 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1284, [false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1284 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1285, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1285 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1286, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1286 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1287, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1287 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1288, [false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1288 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1289, [false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1289 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1290, [true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1290 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1291, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1291 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1292, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1292 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1293, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false]); // 1293 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(1294, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1294 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1295, [false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1295 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1296, [false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1296 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1297, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true]); // 1297 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(1298, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1298 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1299, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1299 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1300, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1300 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1301, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false]); // 1301 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1302, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1302 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1303, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1303 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1304, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1304 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1305, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1305 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1306, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false]); // 1306 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1307, [false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1307 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1308, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1308 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1309, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1309 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1310, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1310 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1311, [false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1311 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1312, [false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1312 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1313, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1313 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1314, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false]); // 1314 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1315, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1315 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1316, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1316 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1317, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1317 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1318, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1318 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1319, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1319 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1320, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1320 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1321, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1321 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1322, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1322 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1323, [false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1323 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1324, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1324 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1325, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1325 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1326, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1326 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1327, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1327 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1328, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false]); // 1328 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(1329, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1329 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1330, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1330 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1331, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1331 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1332, [false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1332 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1333, [false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1333 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1334, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1334 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1335, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1335 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1336, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1336 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1337, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1337 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1338, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1338 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1339, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1339 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1340, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1340 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1341, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1341 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1342, [false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false]); // 1342 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1343, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false]); // 1343 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0)
            map.insert(1344, [false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1344 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1345, [false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1345 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1346, [true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1346 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1347, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1347 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1348, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false]); // 1348 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0)
            map.insert(1349, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1349 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1350, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1350 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1351, [false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1351 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1352, [true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1352 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1353, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1353 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1354, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1354 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1355, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1355 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1356, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1356 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1357, [false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1357 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1358, [true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1358 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1359, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1359 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1360, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1360 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1361, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1361 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1362, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1362 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1363, [false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1363 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1364, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1364 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1365, [false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1365 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1366, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1366 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1367, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false]); // 1367 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1368, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1368 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1369, [false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1369 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1370, [true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1370 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1371, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1371 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1372, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1372 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1373, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1373 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1374, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1374 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1375, [false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1375 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1376, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1376 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1377, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false]); // 1377 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1378, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1378 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1379, [false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1379 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1380, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false]); // 1380 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1381, [false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1381 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1382, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1382 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1383, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1383 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1384, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1384 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1385, [false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1385 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1386, [true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1386 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1387, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1387 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1388, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1388 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1389, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1389 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1390, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1390 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1391, [false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1391 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1392, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1392 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1393, [false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1393 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1394, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1394 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1395, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1395 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1396, [true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1396 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1397, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1397 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1398, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1398 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1399, [false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1399 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1400, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false]); // 1400 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(1401, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1401 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1402, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1402 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1403, [false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1403 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1404, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1404 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1405, [false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1405 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1406, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1406 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1407, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1407 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1408, [false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1408 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1409, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1409 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1410, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1410 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1411, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false]); // 1411 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0)
            map.insert(1412, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1412 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1413, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1413 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1414, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1414 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1415, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1415 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1416, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1416 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1417, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1417 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1418, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1418 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1419, [false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1419 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1420, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false]); // 1420 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(1421, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1421 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1422, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true]); // 1422 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(1423, [false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1423 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1424, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1424 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1425, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1425 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1426, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true]); // 1426 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1427, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1427 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1428, [false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1428 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1429, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1429 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1430, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1430 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1431, [false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1431 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1432, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1432 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1433, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1433 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1434, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1434 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1435, [false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1435 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1436, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1436 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1437, [false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1437 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1438, [true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1438 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1439, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1439 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1440, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1440 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1441, [false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false]); // 1441 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1442, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1442 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1443, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1443 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1444, [false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1444 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1445, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1445 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1446, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1446 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1447, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1447 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1448, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1448 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1449, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1449 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1450, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1450 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1451, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1451 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1452, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1452 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1453, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1453 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1454, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1454 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1455, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1455 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1456, [false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1456 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1457, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1457 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1458, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1458 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1459, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1459 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1460, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1460 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1461, [true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1461 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1462, [false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1462 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1463, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1463 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1464, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1464 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1465, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1465 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1466, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1466 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1467, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1467 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1468, [false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1468 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1469, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true]); // 1469 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(1470, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1470 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1471, [false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1471 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1472, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1472 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1473, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1473 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1474, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1474 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1475, [false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1475 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1476, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1476 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1477, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1477 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1478, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1478 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1479, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1479 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1480, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false]); // 1480 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1481, [false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1481 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1482, [true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1482 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1483, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1483 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1484, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1484 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1485, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1485 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1486, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1486 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1487, [false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1487 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1488, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1488 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1489, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1489 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1490, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1490 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1491, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1491 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1492, [false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false]); // 1492 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1493, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1493 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1494, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1494 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1495, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 1495 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1496, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1496 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1497, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1497 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1498, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 1498 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1499, [false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1499 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1500, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1500 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1501, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false]); // 1501 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1502, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1502 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1503, [false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1503 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1504, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1504 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1505, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1505 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1506, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1506 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1507, [false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1507 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1508, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1508 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1509, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1509 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1510, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1510 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1511, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false]); // 1511 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(1512, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1512 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1513, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1513 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1514, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1514 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1515, [false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1515 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1516, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1516 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1517, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1517 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1518, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1518 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1519, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1519 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1520, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 1520 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(1521, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 1521 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(1522, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1522 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1523, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1523 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1524, [false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1524 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1525, [false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1525 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1526, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1526 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1527, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false]); // 1527 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(1528, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1528 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1529, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 1529 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1530, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1530 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1531, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true]); // 1531 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(1532, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1532 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1533, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 1533 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1534, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1534 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1535, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 1535 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1536, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1536 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1537, [true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1537 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1538, [false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1538 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1539, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1539 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1540, [false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1540 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1541, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1541 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1542, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1542 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1543, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1543 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1544, [false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1544 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1545, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false]); // 1545 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1546, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1546 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1547, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false]); // 1547 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0)
            map.insert(1548, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1548 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1549, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1549 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1550, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1550 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1551, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1551 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1552, [false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1552 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1553, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1553 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1554, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1554 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1555, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1555 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1556, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false]); // 1556 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(1557, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true]); // 1557 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1)
            map.insert(1558, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1558 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1559, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1559 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1560, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1560 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1561, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1561 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1562, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1562 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1563, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1563 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1564, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false]); // 1564 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1565, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1565 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1566, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1566 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1567, [false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1567 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1568, [false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1568 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1569, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1569 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1570, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1570 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1571, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1571 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1572, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false]); // 1572 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0)
            map.insert(1573, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1573 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1574, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false]); // 1574 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1575, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1575 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1576, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1576 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1577, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1577 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1578, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1578 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1579, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1579 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1580, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1580 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1581, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1581 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1582, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1582 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1583, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1583 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1584, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1584 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1585, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false]); // 1585 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1586, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1586 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1587, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1587 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1588, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1588 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1589, [false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1589 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1590, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1590 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1591, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false]); // 1591 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(1592, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1592 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1593, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1593 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1594, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1594 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1595, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1595 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1596, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1596 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1597, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1597 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1598, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1598 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1599, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1599 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1600, [false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1600 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1601, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true]); // 1601 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1)
            map.insert(1602, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false]); // 1602 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(1603, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1603 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1604, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1604 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1605, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1605 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1606, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1606 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1607, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1607 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1608, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1608 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1609, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1609 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1610, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false]); // 1610 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1611, [false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1611 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1612, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1612 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1613, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1613 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1614, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1614 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1615, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true]); // 1615 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1616, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1616 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1617, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1617 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1618, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1618 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1619, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, false]); // 1619 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1620, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1620 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1621, [false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1621 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1622, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1622 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1623, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1623 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1624, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1624 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1625, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1625 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1626, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1626 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1627, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1627 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1628, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1628 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1629, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1629 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1630, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1630 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1631, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, false, false]); // 1631 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0)
            map.insert(1632, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false]); // 1632 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1633, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1633 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1634, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1634 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1635, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1635 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1636, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true]); // 1636 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1637, [false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1637 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1638, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1638 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1639, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1639 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1640, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1640 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1641, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false]); // 1641 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1642, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1642 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1643, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1643 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1644, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1644 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1645, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1645 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1646, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false]); // 1646 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0)
            map.insert(1647, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false]); // 1647 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1648, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false]); // 1648 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0)
            map.insert(1649, [false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1649 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1650, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1650 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1651, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1651 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1652, [true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1652 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1653, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1653 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1654, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1654 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1655, [false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1655 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1656, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1656 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1657, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1657 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1658, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1658 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1659, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1659 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1660, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false]); // 1660 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1661, [false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1661 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1662, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1662 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1663, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1663 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1664, [false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1664 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1665, [false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1665 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1666, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1666 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1667, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1667 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1668, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1668 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1669, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false]); // 1669 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1670, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true]); // 1670 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1)
            map.insert(1671, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1671 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1672, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1672 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1673, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1673 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1674, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false]); // 1674 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1675, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, false]); // 1675 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(1676, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1676 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1677, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1677 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1678, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1678 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1679, [false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1679 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1680, [true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1680 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1681, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1681 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1682, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false]); // 1682 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1683, [false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1683 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1684, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1684 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1685, [false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1685 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1686, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false]); // 1686 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1687, [false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1687 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1688, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false]); // 1688 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0)
            map.insert(1689, [false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1689 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1690, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1690 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1691, [false, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1691 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1692, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1692 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1693, [false, true, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1693 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1694, [true, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1694 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1695, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1695 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1696, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false]); // 1696 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1697, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1697 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1698, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1698 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1699, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1699 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1700, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1700 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1701, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, true]); // 1701 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1702, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false]); // 1702 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1703, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1703 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1704, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false]); // 1704 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(1705, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false]); // 1705 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0)
            map.insert(1706, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1706 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1707, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1707 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1708, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1708 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1709, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1709 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1710, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false]); // 1710 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1711, [false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1711 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1712, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1712 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1713, [false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1713 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1714, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1714 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1715, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1715 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1716, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1716 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1717, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1717 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1718, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1718 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1719, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false]); // 1719 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0)
            map.insert(1720, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1720 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1721, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false]); // 1721 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1722, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1722 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1723, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1723 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1724, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1724 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1725, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false]); // 1725 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1726, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1726 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1727, [false, false, false, false, false, true, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1727 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1728, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1728 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1729, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1729 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1730, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1730 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1731, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1731 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1732, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1732 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1733, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1733 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1734, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1734 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1735, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1735 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1736, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1736 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1737, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1737 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1738, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1738 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1739, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1739 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1740, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1740 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1741, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1741 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1742, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1742 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1743, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1743 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1744, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, true]); // 1744 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1745, [false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1745 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1746, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false]); // 1746 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0)
            map.insert(1747, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1747 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1748, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1748 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1749, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false]); // 1749 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1750, [false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1750 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1751, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 1751 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1752, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1752 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1753, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1753 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1754, [false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 1754 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1755, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false]); // 1755 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1756, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1756 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1757, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1757 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1758, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1758 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1759, [false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1759 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1760, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1760 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1761, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1761 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1762, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1762 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1763, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1763 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1764, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1764 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1765, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1765 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1766, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1766 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1767, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1767 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1768, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1768 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1769, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1769 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1770, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1770 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1771, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1771 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1772, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1772 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1773, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1773 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1774, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1774 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1775, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1775 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1776, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false]); // 1776 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1777, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1777 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1778, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1778 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1779, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1779 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1780, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1780 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1781, [false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1781 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1782, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false]); // 1782 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1783, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1783 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1784, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false, false, false, false]); // 1784 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1785, [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 1785 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1786, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1786 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1787, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1787 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1788, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1788 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1789, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true]); // 1789 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1)
            map.insert(1790, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false]); // 1790 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(1791, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false]); // 1791 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1792, [false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 1792 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1793, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1793 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1794, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1794 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1795, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1795 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1796, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true]); // 1796 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1797, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1797 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1798, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1798 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1799, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false]); // 1799 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1800, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false]); // 1800 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0)
            map.insert(1801, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1801 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1802, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1802 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1803, [false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1803 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1804, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1804 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1805, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1805 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1806, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1806 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1807, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1807 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1808, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1808 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1809, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1809 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1810, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1810 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1811, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1811 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1812, [false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1812 => (0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1813, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1813 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1814, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false]); // 1814 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0)
            map.insert(1815, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1815 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1816, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false]); // 1816 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1817, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1817 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1818, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1818 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1819, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1819 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1820, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1820 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1821, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1821 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1822, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1822 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1823, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1823 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1824, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1824 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1825, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1825 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1826, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false]); // 1826 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1827, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1827 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1828, [false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1828 => (0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1829, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1829 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1830, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1830 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1831, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true]); // 1831 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1)
            map.insert(1832, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1832 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1833, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1833 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1834, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1834 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1835, [false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false]); // 1835 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1836, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false]); // 1836 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1837, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false]); // 1837 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1838, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1838 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1839, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1839 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1840, [false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1840 => (0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1841, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1841 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1842, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1842 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1843, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1843 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1844, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1844 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1845, [true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1845 => (1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1846, [false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1846 => (0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1847, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false]); // 1847 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1848, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1848 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1849, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false]); // 1849 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1850, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1850 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1851, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1851 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1852, [false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1852 => (0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1853, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1853 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1854, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1854 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1855, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1855 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1856, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1856 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1857, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1857 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1858, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true]); // 1858 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1)
            map.insert(1859, [false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1859 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1860, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1860 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1861, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1861 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1862, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1862 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1863, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1863 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1864, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1864 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1865, [false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1865 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1866, [true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1866 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1867, [false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1867 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1868, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1868 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1869, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1869 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1870, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1870 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1871, [false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1871 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1872, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1872 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1873, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1873 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1874, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1874 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1875, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1875 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1876, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1876 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1877, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1877 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1878, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false]); // 1878 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1879, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 1879 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1880, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1880 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1881, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false]); // 1881 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0)
            map.insert(1882, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 1882 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1883, [false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1883 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1884, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true]); // 1884 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1)
            map.insert(1885, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1885 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1886, [false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false]); // 1886 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1887, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1887 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1888, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1888 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1889, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1889 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1890, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1890 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1891, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1891 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1892, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1892 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1893, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1893 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1894, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1894 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1895, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1895 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1896, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1896 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1897, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1897 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1898, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1898 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1899, [false, false, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1899 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1900, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1900 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1901, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true]); // 1901 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1)
            map.insert(1902, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1902 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1903, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1903 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1904, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 1904 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1905, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true]); // 1905 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1906, [false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false]); // 1906 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1907, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false]); // 1907 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0)
            map.insert(1908, [false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1908 => (0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1909, [false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 1909 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1910, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 1910 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1911, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1911 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1912, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 1912 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1913, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 1913 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1914, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1914 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1915, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1915 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1916, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false, false]); // 1916 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0)
            map.insert(1917, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1917 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1918, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1918 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1919, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 1919 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1920, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1920 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1921, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1921 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1922, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1922 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1923, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1923 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1924, [false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1924 => (0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1925, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false]); // 1925 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1926, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1926 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1927, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false]); // 1927 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1928, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1928 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1929, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1929 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1930, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1930 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1931, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1931 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1932, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1932 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1933, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1933 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1934, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false]); // 1934 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1935, [false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1935 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1936, [false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1936 => (0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1937, [false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false]); // 1937 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1938, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1938 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1939, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false]); // 1939 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1940, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1940 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1941, [true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1941 => (1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1942, [false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1942 => (0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1943, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 1943 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1944, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false]); // 1944 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(1945, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1945 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1946, [false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 1946 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1947, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false]); // 1947 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0)
            map.insert(1948, [false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1948 => (0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1949, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, true, false]); // 1949 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1950, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false]); // 1950 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1951, [false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 1951 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1952, [false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1952 => (0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1953, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false]); // 1953 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0)
            map.insert(1954, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1954 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1955, [false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1955 => (0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1956, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1956 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1957, [true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1957 => (1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1958, [false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1958 => (0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1959, [false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1959 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1960, [false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false]); // 1960 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1961, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1961 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1962, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false]); // 1962 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0)
            map.insert(1963, [false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1963 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1964, [false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1964 => (0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1965, [false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1965 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1966, [true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1966 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1967, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1967 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1968, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1968 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1969, [true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1969 => (1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1970, [false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1970 => (0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1971, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, false, false]); // 1971 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1972, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1972 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1973, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1973 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1974, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1974 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1975, [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1975 => (1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1976, [false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1976 => (0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1977, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 1977 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(1978, [false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 1978 => (0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1979, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, false]); // 1979 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0)
            map.insert(1980, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1980 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1981, [true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1981 => (1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1982, [false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 1982 => (0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1983, [false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 1983 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1984, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false]); // 1984 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0)
            map.insert(1985, [false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false]); // 1985 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(1986, [false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false]); // 1986 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(1987, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 1987 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1988, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1988 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1989, [false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false]); // 1989 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1990, [false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false]); // 1990 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1991, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 1991 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(1992, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1992 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1993, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]); // 1993 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1)
            map.insert(1994, [false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 1994 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(1995, [false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 1995 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(1996, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1996 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1997, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1997 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1998, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 1998 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(1999, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false]); // 1999 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0)
            map.insert(2000, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]); // 2000 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2001, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false]); // 2001 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0)
            map.insert(2002, [false, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2002 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2003, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2003 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2004, [false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2004 => (0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2005, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2005 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2006, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2006 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2007, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2007 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2008, [false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2008 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2009, [false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2009 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2010, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2010 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2011, [true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2011 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2012, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 2012 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(2013, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false, false, false]); // 2013 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2014, [false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2014 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2015, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2015 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2016, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, true]); // 2016 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1)
            map.insert(2017, [false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2017 => (0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2018, [true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2018 => (1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2019, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2019 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2020, [false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2020 => (0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2021, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, true, false]); // 2021 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0)
            map.insert(2022, [false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false]); // 2022 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2023, [false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2023 => (0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2024, [false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, false]); // 2024 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(2025, [false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2025 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2026, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false]); // 2026 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0)
            map.insert(2027, [false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2027 => (0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2028, [false, false, false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 2028 => (0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(2029, [false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false]); // 2029 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
            map.insert(2030, [false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true]); // 2030 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1)
            map.insert(2031, [false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false]); // 2031 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2032, [false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2032 => (0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2033, [false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2033 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2034, [false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, false, false, false]); // 2034 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0)
            map.insert(2035, [false, false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 2035 => (0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
            map.insert(2036, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2036 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2037, [true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2037 => (1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2038, [false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2038 => (0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2039, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true]); // 2039 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1)
            map.insert(2040, [true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2040 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2041, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2041 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2042, [false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, false, false, false]); // 2042 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
            map.insert(2043, [false, true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2043 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2044, [false, false, false, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 2044 => (0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
            map.insert(2045, [false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false]); // 2045 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
            map.insert(2046, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false]); // 2046 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0)
            map.insert(2047, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false]); // 2047 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode for GolayCode23_12 {
    fn name(&self) -> String {
        "[23, 12] Golay code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        12
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
        codeword.truncate(12);
        Ok(codeword)
        
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1.0 + 23.0 * delta + 253.0 * delta.powi(2) + 1771.0 * delta.powi(3))/2f64.powi(11)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = GolayCode23_12.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 12);
    }

    #[test]
    fn random_decode_tests() {

        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, false, false, false, true, true, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, true, false, false, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, false, false, true, true, false, true, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, false, true, true, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, false, false, false, false, true, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, false, true, true, false, true, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, false, true, true, false, true, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, false, true, false, true, true, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, false, true, true, true, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, false, false, true, false, false, false, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, false, true, false, true, true, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, false, true, true, false, true, false, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, false, false, false, true, false, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, true, false, false, false, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, true, false, false, false, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, true, false, false, true, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, false, true, false, false, false, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, true, false, false, false, true, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, false, false, false, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, true, true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, true, false, false, false, false, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, false, false, true, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, false, true, false, false, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, true, false, false, true, false, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, false, true, true, false, true, true, true, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, false, true, true, true, false, true, true, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, false, true, false, true, false, false, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, false, false, true, false, true, true, false, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, false, true, false, true, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, true, false, true, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, false, false, true, true, true, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, true, false, true, false, false, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, true, true, false, false, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, true, true, true, true, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, false, false, false, false, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, false, true, false, false, true, true, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true, true, false, true, true, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, true, false, true, true, true, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, false, true, true, true, false, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, true, true, false, false, true, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, true, true, false, false, true, true, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, true, true, false, false, true, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, true, true, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, false, false, true, true, false, false, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, true, true, false, true, true, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, true, false, true, true, false, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, true, false, false, true, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, true, true, false, false, false, false, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, true, true, false, true, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, false, true, true, false, true, true, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, true, false, true, false, true, true, false, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, false, false, false, false, true, false, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true, false, true, true, true, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, true, true, false, true, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, false, false, true, true, false, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, true, true, false, true, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, false, true, true, false, false, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, false, true, false, true, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false, false, true, false, true, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, false, true, true, true, false, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, true, true, true, false, false, false, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, false, true, true, true, true, false, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, false, false, false, true, false, true, false, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, false, false, true, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, true, true, false, true, false, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, true, true, false, true, false, false, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, true, true, false, false, false, false, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, true, false, false, true, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, true, true, true, true, false, true, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, true, true, false, false, true, true, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, false, false, true, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, true, true, true, false, true, true, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, true, true, true, false, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false, true, false, true, false, false, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, false, false, false, false, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, false, false, true, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, true, false, false, true, false, false, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, true, true, true, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, true, false, true, true, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, true, false, false, false, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, false, true, true, false, false, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, true, true, false, true, true, false, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, true, true, false, false, true, false, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, true, true, true, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, false, true, true, true, true, true, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, false, true, true, true, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, false, true, false, true, true, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, false, true, false, true, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, false, true, false, true, false, false, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false, false, true, false, false, false, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, false, false, false, false, false, false, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, true, false, true, false, true, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, false, true, false, true, true, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, true, true, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, true, true, true, true, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, true, true, false, true, false, true, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, true, true, false, false, true, true, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, false, true, true, false, false, true, false, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false, false, true, false, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, false, true, false, false, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false, false, false, true, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, true, false, false, false, false, true, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, true, false, true, false, false, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, false, true, false, true, false, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, true, false, true, true, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, false, true, false, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, false, true, false, true, false, true, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, false, true, true, false, false, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, true, true, false, false, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, false, false, true, true, true, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, true, false, false, true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, false, true, false, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, false, false, true, false, false, false, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, true, true, true, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, true, true, true, true, false, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, false, true, false, true, false, false, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, false, true, true, true, true, true, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, true, true, true, true, true, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, true, false, false, true, true, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, false, true, false, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, false, true, true, true, true, false, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, true, true, false, false, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, true, false, true, true, false, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, false, false, true, true, false, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, false, false, true, true, true, false, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, false, true, true, true, false, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, true, true, false, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, true, true, true, false, true, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, true, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true, false, true, true, false, true, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true, false, true, true, true, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, true, true, true, true, false, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, false, true, false, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, false, true, true, false, true, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, false, false, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, true, true, false, false, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, true, false, false, false, false, true, false, true, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, true, false, true, false, true, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, true, true, false, true, false, true, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, true, true, false, false, false, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, false, true, false, true, false, true, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, false, true, true, false, false, false, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, false, false, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false, false, false, true, true, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, true, false, true, true, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, false, false, true, false, true, false, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, false, false, true, true, false, false, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, true, true, true, true, false, false, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, true, true, false, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, true, true, true, true, false, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, false, false, true, true, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, true, false, false, false, true, false, true, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, true, false, true, false, true, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, true, true, false, true, false, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, true, true, true, false, true, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, true, false, true, false, true, false, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, true, true, false, true, true, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true, true, true, false, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true, true, false, false, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, false, true, true, true, true, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, true, true, true, true, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, true, true, true, true, true, true, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, false, true, true, true, false, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, false, true, true, true, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true, true, true, true, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, true, true, true, true, false, true, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, false, true, false, false, true, true, false, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, false, true, true, false, false, false, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, false, false, false, true, false, false, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, true, true, false, false, true, false, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true, false, true, true, false, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, false, true, false, true, true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, false, true, true, false, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, true, false, false, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, true, true, false, false, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, true, false, true, true, true, false, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, false, true, false, false, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false, false, false, true, true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false, false, true, true, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true, true, true, true, true, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, false, true, true, false, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, false, true, true, false, false, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, false, true, true, false, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, true, true, true, true, true, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, true, true, true, true, true, false, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, true, false, false, true, true, true, true, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, true, false, false, true, false, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, false, false, true, false, true, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, true, false, false, true, true, true, false, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }

        {
            let code = GolayCode23_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, false, true, true, false, false, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

}
