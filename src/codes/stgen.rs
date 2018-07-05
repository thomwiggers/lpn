use codes::BinaryCode;
use itertools::{Combinations, Itertools};
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::cell::UnsafeCell;
use std::cmp::min;
use std::mem;
use std::ptr;
use std::sync::Mutex;

/// 'Concatenated' Linear Codes with extra noise
///
/// This struct allows to construct a Linear code from the direct sum
/// of smaller codes.
pub struct StGenCode<'codes> {
    codes: Vec<&'codes dyn BinaryCode>,
    noises: Vec<Option<BinMatrix>>,
    init: Mutex<bool>,
    generator: UnsafeCell<*mut BinMatrix>,
    w0: u32,
    l_max: usize,
    wb: u32,
}

impl<'codes> Clone for StGenCode<'codes> {
    fn clone(&self) -> Self {
        StGenCode {
            codes: self.codes.clone(),
            noises: self.noises.clone(),
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
            w0: self.w0,
            l_max: self.l_max,
            wb: self.wb,
        }
    }
}

unsafe impl<'a> Sync for StGenCode<'a> {}

impl<'codes> StGenCode<'codes> {
    /// Construct a new stgencode
    pub fn new(
        codes: Vec<&'codes dyn BinaryCode>,
        w0: u32,
        l_max: usize,
        wb: u32,
    ) -> StGenCode<'codes> {
        debug_assert_ne!(codes.len(), 0, "need at least 1 code");
        let mut noises = Vec::with_capacity(codes.len() - 1);
        noises.push(None); // first block always none
        let mut k_sum = codes[0].dimension();
        for code in codes.iter().skip(1) {
            let ni = code.length() - code.dimension();
            let noise = if ni != 0 {
                Some(BinMatrix::random(k_sum, ni))
            } else {
                None
            };
            debug_assert_ne!(k_sum, 0);
            noises.push(noise);
            k_sum += code.dimension();
        }
        debug_assert_eq!(noises.len(), codes.len());
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

impl<'codes> BinaryCode for StGenCode<'codes> {
    fn name(&self) -> String {
        let names = self.codes.iter().fold(
            String::with_capacity(self.codes.iter().fold(0, |acc, c| acc + 2 + c.name().len())),
            |mut s, code| {
                s.push_str(&code.name());
                s.push_str(", ");
                s
            },
        );
        format!(
            "[{}, {}] StGen code with w0={}, wb={}, L={}, codes=[{}]",
            self.length(),
            self.dimension(),
            self.w0,
            self.wb,
            self.l_max,
            names,
        )
    }

    fn length(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.length())
    }

    fn dimension(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.dimension())
    }

    fn generator_matrix(&self) -> &BinMatrix {
        debug_assert_ne!(
            self.codes.len(),
            0,
            "We need at least one code for this to work"
        );
        // check if we've initialized the generator
        {
            let get_code_bits = |code: &BinaryCode| {
                debug_assert_ne!(code.dimension(), code.length(), "Would construct 0 matrix");
                code.generator_matrix().get_window(
                    0,
                    code.dimension(),
                    code.dimension(),
                    code.length(),
                )
            };
            let mut initialized = self.init.lock().unwrap();
            if !*initialized {
                let mut ki = 0;
                let mut start = 0;
                let mut gen = loop {
                    let code = self.codes[start];
                    ki += code.dimension();
                    start += 1;
                    if code.dimension() != code.length() {
                        let b0 = get_code_bits(code);
                        debug_assert_eq!(b0.nrows(), code.dimension());
                        break if b0.nrows() != ki {
                            // we skipped at least one block
                            let noise_blk = self.noises[start - 1].as_ref().unwrap(); // this must exist
                            debug_assert_eq!(noise_blk.nrows(), ki - b0.nrows());
                            debug_assert_eq!(noise_blk.ncols(), b0.ncols());
                            let origin = noise_blk.stacked(&b0);
                            origin
                        } else {
                            b0.clone()
                        };
                    }
                };
                for (i, code) in self.codes.iter().skip(1).enumerate().skip(start - 1) {
                    debug_assert_eq!(gen.nrows(), ki);
                    let ni = code.length() - code.dimension();
                    if ni == 0 {
                        ki += code.length();
                        // add something to the bottom
                        gen = gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols()));
                        continue;
                    }
                    let bi = get_code_bits(code.clone());
                    debug_assert_eq!(bi.nrows(), code.dimension());
                    debug_assert_eq!(bi.ncols(), ni);
                    let corner = (gen.nrows(), gen.ncols());
                    let noise_block = self.noises[i + 1].as_ref().unwrap();
                    debug_assert_eq!(noise_block.ncols(), ni);
                    debug_assert_eq!(noise_block.nrows(), ki);
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
                    "INT: The right part should have $dimension rows"
                );
                let gen_box = Box::new(BinMatrix::identity(self.dimension()).augmented(&gen));
                debug_assert_eq!(gen_box.nrows(), self.dimension(), "INT: rows incorrect");
                debug_assert_eq!(gen_box.ncols(), self.length(), "INT: cols incorrect");

                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen_box);
                }
                *initialized = true;
            };
        }
        unsafe { (*(self.generator.get())).as_ref().unwrap() }
    }

    fn parity_check_matrix(&self) -> &'static BinMatrix {
        panic!("Not yet implemented");
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
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

        let mut b = BinVector::with_capacity(20);
        let mut b_tmp = BinVector::with_capacity(20);
        for i in 0..self.codes.len() {
            // set helpful vars
            let small_code = self.codes[i];
            let ki = small_code.dimension();
            let ni = small_code.length() - ki;
            n_sum += ni;
            k_sum += ki;
            let (lower, c_) = split_binvec(c, ki);
            c = c_;
            debug_assert_eq!(
                lower.len(),
                ki,
                "internal: the segment of C we took is not of size ki",
            );

            // set relevant part of B
            b.clear();
            b.extend_from_binvec(&lower);

            let mut c_upper = BinVector::with_capacity(ni);
            for i in 0..ni {
                c_upper.push(orig_c[k + (n_sum - ni) + i]);
            }
            debug_assert_eq!(c_upper.len(), ni);
            for (xp, ep) in l_previous.drain(..).into_iter() {
                b_tmp.clear();
                b_tmp.extend_from_binvec(&b);
                debug_assert!(
                    b_tmp.capacity() < 100000,
                    "capacity is {}",
                    b_tmp.capacity()
                );
                if let Some(block_noise) = self.noises[i].as_ref() {
                    debug_assert_eq!(
                        block_noise.nrows(),
                        xp.len(),
                        "Length of vector or noise matrix wrong"
                    );
                    let product = &xp * block_noise;
                    debug_assert_eq!(product.len(), ni, "internal: product length wrong");
                    b_tmp.extend_from_binvec(&(&product + &c_upper));
                } else {
                    b_tmp.extend_from_binvec(&c_upper);
                }

                debug_assert_eq!(b_tmp.len(), ki + ni);

                // allow this many possible errors
                let max_weight = if i > 0 {
                    min((ni + ki) as u32, min(wi - ep.count_ones(), self.wb))
                } else {
                    min(wi, self.wb)
                };
                debug_assert!(max_weight <= self.wb);

                // foreach possible error...
                let (ep_lo, ep_hi) = split_binvec(ep, k_sum - ki);
                for e_prime in vectors_up_to(max_weight as usize, ni + ki) {
                    // find x'G st xG + b == e'
                    let mut x_code = small_code.decode_to_code(&(&b_tmp + &e_prime))?;
                    if &x_code + &e_prime != b_tmp {
                        continue;
                    }
                    // rename to be more accurate
                    x_code.truncate(ki);
                    let x_prime = x_code;
                    let (e_prime_lo, e_prime_hi) = split_binvec(e_prime, ki);
                    let mut e_new = BinVector::with_capacity(ni + ki);
                    e_new.extend_from_binvec(&ep_lo);
                    e_new.extend_from_binvec(&e_prime_lo);
                    e_new.extend_from_binvec(&ep_hi);
                    e_new.extend_from_binvec(&e_prime_hi);

                    let mut x_new = xp.clone();
                    x_new.extend_from_binvec(&x_prime);

                    l_next.push((x_new, e_new));
                }
            }
            // swap
            l_next = mem::replace(&mut l_previous, l_next);

            if l_previous.len() < self.l_max {
                wi += 1;
            } else {
                l_previous
                    .sort_unstable_by(|(_, e1), (_, e2)| e1.count_ones().cmp(&e2.count_ones()));
                l_previous.truncate(self.l_max);
            }
        }

        if let Some((x, e)) = l_previous.into_iter().min_by_key(|(_x, e)| e.count_ones()) {
            debug_assert_eq!(
                &(&self.encode(&x) + &e),
                orig_c,
                "This isn't a valid solution?! {:?} G + {:?} != {:?}",
                &x,
                &e,
                orig_c,
            );
            Ok(x)
        } else {
            Err("No result found")
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
    let iter = (0..length).combinations(0);
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
    use codes::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn test_vectors_up_to() {
        let mut generator = vectors_up_to(3, 3);
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[false, false, false]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[true, false, false]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[false, true, false]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[false, false, true]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[true, true, false]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[true, false, true]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[false, true, true]))
        );
        assert_eq!(
            generator.next(),
            Some(BinVector::from_bools(&[true, true, true]))
        );
        assert_eq!(generator.next(), None);
    }

    lazy_static! {
        static ref IDCODE: IdentityCode = IdentityCode::new(3);
    }
    fn get_code() -> StGenCode<'static> {
        let codes: Vec<&dyn BinaryCode> = vec![
            &*IDCODE,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &*IDCODE,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
        ];
        StGenCode::new(codes, 3, 100, 3)
    }

    #[test]
    fn test_generator_matrix() {
        let code = get_code();

        let gen = code.generator_matrix();
        let mut col = code.dimension();
        let mut row = 0;
        for (i, subcode) in code.codes.iter().enumerate() {
            let ki = subcode.dimension();
            let ni = subcode.length() - ki;
            if ni == 0 {
                // skip this one
                row += ki;
                continue;
            }
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
                    }
                }
            }
            assert!(result);
            assert_eq!(window, other_window);

            // check noise blocks
            if i > 0 && ni != 0 {
                let noise = code.noises[i].as_ref().unwrap();
                let window = gen.get_window(0, col, row, col + ni);
                assert_eq!(window.nrows(), noise.nrows());
                assert_eq!(window.ncols(), noise.ncols());
                let mut result = true;
                for i in 0..window.nrows() {
                    for j in 0..window.ncols() {
                        if window.bit(i, j) != noise.bit(i, j) {
                            result = false;
                        }
                    }
                }
                assert!(result);
                assert_eq!(window, *noise);
            }
            col += ni;
            row += ki;
        }
    }

    #[test]
    fn test_decode() {
        let code = get_code();

        let input = BinVector::random(code.dimension());

        // idempotent
        assert_eq!(
            input,
            code.decode_to_message(&code.encode(&input)).unwrap(),
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
