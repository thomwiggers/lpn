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

unsafe impl<'a, 'b> Sync for StGenCode<'a, 'b> {}

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
        let mut k_sum = codes[0].dimension();
        for code in codes.iter().skip(1) {
            let ni = code.length() - code.dimension();
            let noise = BinMatrix::random(k_sum, ni);
            debug_assert_ne!(k_sum, 0);
            debug_assert_eq!(noise.nrows(), k_sum);
            noises.push(noise);
            k_sum += code.dimension();
        }
        debug_assert_eq!(noises.len(), codes.len() - 1);
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
            let get_code_bits = |code: &BinaryCode| {
                (code.generator_matrix().get_window(
                    0,
                    code.dimension(),
                    code.dimension(),
                    code.length(),
                ))
            };
            let initialized: bool = *self.init.lock().unwrap();
            if !initialized {
                let b0 = get_code_bits(self.codes[0]);
                debug_assert_eq!(b0.nrows(), self.codes[0].dimension());
                let mut gen = b0.clone();
                let mut ki = b0.nrows();
                for (i, code) in self.codes.iter().skip(1).enumerate() {
                    debug_assert_eq!(gen.nrows(), ki);
                    let bi = get_code_bits(code.clone());
                    let ni = bi.ncols();
                    debug_assert_eq!(bi.nrows(), code.dimension());
                    let corner = (gen.nrows(), gen.ncols());
                    let noise_block = &self.noises[i];
                    debug_assert_eq!(noise_block.ncols(), ni);
                    debug_assert_eq!(
                        noise_block.nrows(),
                        gen.nrows(),
                        "INT: noise block {} isn't right",
                        i
                    );
                    gen = gen.augmented(noise_block);
                    gen = gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols()));
                    ki += bi.nrows();
                    gen.set_window(corner.0, corner.1, &bi);
                }
                debug_assert_eq!(
                    gen.nrows(),
                    self.dimension(),
                    "INT: The right part should have the size of the dimension"
                );
                let gen_box = Box::new(BinMatrix::identity(self.dimension()).augmented(&gen));
                debug_assert_eq!(gen_box.nrows(), self.dimension());
                debug_assert_eq!(gen_box.ncols(), self.length());

                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen_box);
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
        let orig_c = c;
        let mut c = c.clone();
        // the position of the end of the identity part of the code, not the actual dimension
        let k = self.dimension();
        let mut k_sum = 0;
        let mut n_sum = 0;
        let mut l_previous = Vec::with_capacity(2usize.pow(self.wb as u32));
        // pre-seed L[0]
        l_previous.push((BinVector::new(), BinVector::new()));
        // next round and final result
        let mut l_next = Vec::with_capacity(2usize.pow(self.wb as u32));
        let mut wi = min(self.w0, self.codes[0].length() as u32);
        for i in 0..self.codes.len() {
            // set helpful vars
            let small_code = self.codes[i];
            let ki = small_code.dimension();
            let ni = small_code.length() - ki;
            n_sum += ni;
            k_sum += ki;
            let mut b = BinVector::with_capacity(ki + ni);
            let (lower, c_) = split_binvec(c, ki);
            c = c_;
            debug_assert_eq!(
                lower.len(),
                ki,
                "internal: the segment of C we took is not of size ki={}",
                ki
            );
            b.extend_from_binvec(&lower);
            let mut c_upper = BinVector::with_capacity(ni);
            for i in 0..ni {
                c_upper.push(orig_c[k + (n_sum - ni) + i]);
            }
            debug_assert_eq!(c_upper.len(), ni);
            for (xp, ep) in l_previous.drain(..).into_iter() {
                let mut b = b.clone();
                if i > 0 {
                    let block_noise = &self.noises[i - 1];
                    debug_assert_eq!(
                        block_noise.nrows(),
                        xp.len(),
                        "Length of vector or noise matrix wrong"
                    );
                    let product = &xp * block_noise;
                    debug_assert_eq!(product.len(), ni, "internal: product length wrong");
                    b.extend_from_binvec(&(&product + &c_upper));
                } else {
                    b.extend_from_binvec(&c_upper);
                }

                // allow this many possible errors
                let max_weight = if i > 0 {
                    min((ni + ki) as u32, min(wi - ep.count_ones(), self.wb))
                } else {
                    wi
                };
                // foreach possible error...
                let (ep_lo, ep_hi) = split_binvec(ep, k_sum - ki);
                for e_prime in vectors_up_to(max_weight as usize, ni + ki) {
                    // find x'G st xG + b == e'
                    let x_prime = small_code.decode_to_message(&(&b + &e_prime));
                    // XXX should this be the case?
                    //if &small_code.encode(&x_prime) + &e_prime != b {
                    //    continue;
                    //}
                    let (e_prime_lo, e_prime_hi) = split_binvec(e_prime, ki);
                    let mut e_new = BinVector::with_capacity(k_sum + n_sum);
                    e_new.extend_from_binvec(&ep_lo);
                    e_new.extend_from_binvec(&e_prime_lo);
                    e_new.extend_from_binvec(&ep_hi);
                    e_new.extend_from_binvec(&e_prime_hi);

                    let mut x_new = xp.clone();
                    x_new.extend_from_binvec(&x_prime);

                    l_next.push((x_new, e_new));
                }
            }
            let tmp = l_previous;
            l_previous = l_next;
            l_next = tmp;
            if l_previous.len() < self.l_max {
                wi += 1;
            }
        }

        if let Some((x, _e)) = l_previous.into_iter().min_by_key(|(_x, e)| e.count_ones()) {
            //debug_assert_eq!(self.encode(&x), &e + orig_c, "This isn't a valid solution?!");
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
        let codes: Vec<&BinaryCode<'static>> = vec![
            &HammingCode7_4,
            &HammingCode3_1,
            &HammingCode15_11,
            &HammingCode7_4,
        ];
        StGenCode::new(codes, 5, 100, 3)
    }

    #[test]
    fn test_generator_matrix() {
        let code = get_code();

        let gen = code.generator_matrix();
        let mut col = code.dimension();
        let mut row = 0;
        for subcode in code.codes {
            let ki = subcode.dimension();
            let ni = subcode.length() - ki;
            let window = gen.get_window(row, col, row + ki, col + ni);
            let other_window = subcode.generator_matrix().get_window(0, ki, ki, ni + ki);
            assert_eq!(window.nrows(), other_window.nrows());
            assert_eq!(window.ncols(), other_window.ncols());
            let mut result = true;
            for i in 0..window.nrows() {
                for j in 0..window.ncols() {
                    if window.bit(i, j) != other_window.bit(i, j) {
                        println!("bit {},{} unequal", i, j);
                        result = false;
                    } else {
                        println!("bit {},{} equal!", i, j);
                    }
                }
            }
            assert!(result);
            assert_eq!(window, other_window);
            col += ni;
            row += ki;
        }
    }

    #[test]
    fn test_concatenated_code() {
        let code = get_code();

        let input = BinVector::random(code.dimension());

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
