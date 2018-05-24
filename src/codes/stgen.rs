use codes::BinaryCode;
use itertools::{Combinations, Itertools};
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::cell::UnsafeCell;
use std::cmp::min;
use std::ptr;
use std::sync::Mutex;

/// 'Concatenated' Linear Codes
///
/// This struct allows to construct a Linear code from the direct sum
/// of smaller codes.
///
/// It will construct the generator matrix lazily and use the encode and
/// decode mechanism of the 'child' codes.
pub struct StGenCode<'a, 'c: 'a> {
    codes: Vec<&'a BinaryCode<'c>>,
    noises: Vec<BinMatrix>,
    init: Mutex<bool>,
    generator: UnsafeCell<*mut BinMatrix>,
    w0: u32,
    l_max: usize,
    wb: u32,
}

impl<'codes, 'code> StGenCode<'codes, 'code> {
    /// Construct a new stgencode
    pub fn new(
        codes: Vec<&'codes BinaryCode<'code>>,
        w0: u32,
        l_max: usize,
        wb: u32,
    ) -> StGenCode<'codes, 'code> {
        debug_assert_ne!(codes.len(), 0, "need at least 1 code");
        let mut noises = Vec::with_capacity(codes.len() - 1);
        let mut Ki = codes[0].dimension();
        for code in codes.iter().skip(1) {
            let noise = BinMatrix::random(Ki, code.length());
            noises.push(noise);
            Ki += code.dimension();
        }
        StGenCode {
            codes,
            noises,
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
            w0,
            l_max,
            wb,
        }
    }

    /// Get the max list size
    pub fn l_max(&self) -> usize {
        self.l_max
    }

    /// Get the starting weight limit
    pub fn w0(&self) -> u32 {
        self.w0
    }

    /// Get the round weight limit
    pub fn wb(&self) -> u32 {
        self.wb
    }
}

impl<'codes, 'code> BinaryCode<'codes> for StGenCode<'codes, 'code> {
    fn length(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.length())
    }

    fn dimension(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.dimension())
    }

    fn generator_matrix(&self) -> &'codes BinMatrix {
        debug_assert_ne!(
            self.codes.len(),
            0,
            "We need at least one code for this to work"
        );
        // check if we've initialized the generator
        {
            let initialized: bool = *self.init.lock().unwrap();
            if !initialized {
                let mut gen = Box::new(self.codes[0].generator_matrix().clone());
                for code in self.codes.iter().skip(1) {
                    let corner = (gen.nrows(), gen.ncols());
                    let noise = BinMatrix::random(gen.nrows(), code.length());
                    gen = Box::new(gen.augmented(&noise));
                    gen = Box::new(gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols())));
                    gen.set_window(corner.0, corner.1, code.generator_matrix());
                }
                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen);
                }
            };
        }
        unsafe { (*(self.generator.get())).as_ref().unwrap() }
    }

    fn parity_check_matrix(&self) -> &'static BinMatrix {
        panic!("Not yet implemented");
    }

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        // track helpful variables
        let mut c = c.clone();
        let k = self.dimension();
        let mut Ki = 0;
        let mut Ni = 0;
        let mut L_previous = Vec::with_capacity(2usize.pow(self.wb as u32));
        // pre-seed L[0]
        L_previous.push((BinVector::new(), BinVector::new()));
        // next round and final result
        let mut L_next = Vec::with_capacity(2usize.pow(self.wb as u32));
        let mut wi = self.w0;
        for i in 0..self.codes.len() {
            // set helpful vars
            let small_code = self.codes[i];
            let ki = small_code.dimension();
            let ni = small_code.length();
            Ni += ni;
            Ki += ki;
            let mut b = BinVector::with_capacity(ki + ni);
            let (lower, c_) = split_binvec(c, ki);
            c = c_;
            b.extend(lower.iter());
            let mut c_upper = BinVector::with_capacity(ni);
            for i in 0..ni {
                c_upper.push(c[(k - Ki) + (Ni - ni) + i]);
            }
            for (xp, ep) in L_previous.drain(..).into_iter() {
                let mut b = b.clone();
                if i > 0 {
                    let block_noise = &self.noises[i];
                    b.extend((&(&xp * block_noise) + &c_upper).iter());
                } else {
                    b.extend(c_upper.iter());
                }

                // allow this many possible errors
                let max_weight = min(wi - ep.count_ones(), self.wb);
                // foreach possible error...
                for e_prime in vectors_up_to(max_weight as usize, ni + ki) {
                    // find x'G st xG + b == e'
                    let x_prime = small_code.decode_to_message(&(&b + &e_prime));
                    if &small_code.encode(&x_prime) + &b != e_prime {
                        println!("I still don't see why this is possible");
                        continue;
                    }
                    let (ep_lo, ep_hi) = split_binvec(ep.clone(), Ki - ki);
                    let (e_prime_lo, e_prime_hi) = split_binvec(e_prime, ki);
                    let mut e_new = BinVector::with_capacity(Ki + Ni);
                    e_new.extend(ep_lo.iter());
                    e_new.extend(e_prime_lo.iter());
                    e_new.extend(ep_hi.iter());
                    e_new.extend(e_prime_hi.iter());

                    let mut x_new = BinVector::with_capacity(Ki);
                    x_new.extend(xp.iter());
                    x_new.extend(x_prime.iter());

                    L_next.push((x_new, e_new));
                }
            }
            let tmp = L_previous;
            L_previous = L_next;
            L_next = tmp;
            if L_previous.len() < self.l_max {
                wi += 1;
            }
        }

        if let Some((x, _e)) = L_previous.into_iter().min_by_key(|(x, _e)| x.count_ones()) {
            x
        } else {
            panic!("No result found!");
        }
    }
}

/// Split a binvector in two
///
/// # Example
/// ```no_compile
/// let a = BinVector::from_bools(&[true, false, false]);
/// let (a, b) = split_binvec(a, 1);
/// assert_eq!(a, BinVector::from_bools(&[true]));
/// assert_eq!(b, BinVector::from_bools(&[false, false]));
/// ```
fn split_binvec(mut vec: BinVector, count: usize) -> (BinVector, BinVector) {
    let a = BinVector::from(vec.split_off(count));
    (vec, a)
}

struct WeightIterator {
    weight: usize,
    length: usize,
    current_weight: usize,
    combinations_iterator: Combinations<::std::ops::Range<usize>>,
}

impl Iterator for WeightIterator {
    type Item = BinVector;

    fn next(&mut self) -> Option<Self::Item> {
        let bits = match self.combinations_iterator.next() {
            Some(combination) => Some(combination),
            None => {
                if self.current_weight == self.weight {
                    None
                } else {
                    self.current_weight += 1;
                    self.combinations_iterator = (0..self.length).combinations(self.current_weight);
                    self.combinations_iterator.next()
                }
            }
        };

        match bits {
            Some(combination) => {
                let mut v = BinVector::from_elem(self.length, false);
                for bit in combination {
                    v.set(bit, true);
                }
                Some(v)
            }
            None => None,
        }
    }
}

fn vectors_up_to(weight: usize, length: usize) -> WeightIterator {
    debug_assert!(
        weight <= length,
        "Weight should be less than n ({} <= {})",
        weight,
        length
    );
    let iter = (0..length).combinations(weight);
    WeightIterator {
        weight,
        length,
        current_weight: 0,
        combinations_iterator: iter,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use codes::hamming::*;
    use m4ri_rust::friendly::BinVector;

    fn get_code() -> StGenCode<'static, 'static> {
        let codes: Vec<&BinaryCode<'static>> = vec![&HammingCode7_4, &HammingCode3_1];
        StGenCode::new(codes, 4, 100, 3)
    }

    #[test]
    fn test_concatenated_code() {
        let code = get_code();

        assert_eq!(code.length(), 7 + 3, "Length wrong");
        assert_eq!(code.dimension(), 4 + 1, "Dimension wrong");

        let mut input = BinVector::from_bytes(&[0b10101000]);
        input.truncate(5);

        // idempotent
        assert_eq!(
            input,
            code.decode_to_message(&code.encode(&input)),
            "not idempotent"
        );
    }

    #[test]
    fn test_split_binvec() {
        let a = BinVector::from_bools(&[true, false, false]);
        let (a, b) = split_binvec(a, 1);
        assert_eq!(a, BinVector::from_bools(&[true]));
        assert_eq!(b, BinVector::from_bools(&[false, false]));
    }
}
