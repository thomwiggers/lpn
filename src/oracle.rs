//! Describes the LPN problem oracle on which we apply reductions and solving algorithms
//!
//! This project currently makes strong assumptions that u64 == usize
use indicatif::ProgressBar;
use m4ri_rust::friendly::*;
use rand::distributions::{Bernoulli, Distribution};
use std::{
    cmp, fmt,
    mem::{self, MaybeUninit},
    ops::Range,
};

use rand::prelude::*;
use rand_hc::Hc128Rng;
use rayon::prelude::*;

use crate::util::log_2;

pub(crate) type StorageBlock = u64;
pub(crate) const ONE: StorageBlock = 1;

/// How many bits are stored in each underlying storage block?
const fn bits_per_block() -> usize {
    bytes_per_block() * 8
}

/// How many bytes are stored in each underlying storage block?
const fn bytes_per_block() -> usize {
    std::mem::size_of::<StorageBlock>()
}

/// Return the offset in the vector of the storage block storing the bit `off`.
const fn block_offset(off: usize) -> usize {
    off / bits_per_block()
}

/// Takes as input a number of bits requiring storage; returns an aligned number of blocks needed
/// to store those bits.
const fn blocks_required(num_bits: usize) -> usize {
    num_bits / bits_per_block()
        + if num_bits % bits_per_block() == 0 {
            0
        } else {
            1
        }
}

/// change me according to k
pub const MAX_K: usize = (4 * bits_per_block()) - 1;
/// length of a sample in bytes
pub(crate) const SAMPLE_LEN: usize = blocks_required(MAX_K + 1);
/// Block in which noise bit is stored (the K'th bit)
pub(crate) const NOISE_BIT_BLOCK: usize = block_offset(MAX_K);
/// Index of the noise bit
pub(crate) const NOISE_BIT_IDX: usize = bits_per_block() - 1;
/// Mask to & with to extract just the noise bit
pub(crate) const NOISE_BIT_MASK: StorageBlock = (1 as StorageBlock) << NOISE_BIT_IDX;

pub(crate) type SampleStorage = [StorageBlock; SAMPLE_LEN];

/// Represents a sample in the oracle
///
/// `<a, s> + e = c`
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Sample {
    sample: [StorageBlock; SAMPLE_LEN],
}

impl fmt::Debug for Sample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sample = self
            .sample
            .iter()
            .copied()
            .map(|b| format!("{:064b}", b))
            .collect::<Vec<String>>();
        f.debug_tuple("Sample").field(&sample).finish()
    }
}

impl Sample {
    const fn new() -> Sample {
        Sample {
            sample: [0; SAMPLE_LEN],
        }
    }

    pub fn vector_product(&self, other: &Sample, len: usize) -> bool {
        self.sample
            .iter()
            .zip(other.sample[0..=block_offset(len)].iter())
            .fold(0, |acc, (v1, v2)| (v1 & v2).count_ones() + acc)
            % 2
            == 1
    }

    /// Get the Hamming weight of the sample
    pub fn count_ones(&self) -> u32 {
        let mut acc = 0;
        for block in &self.sample[..SAMPLE_LEN - 1] {
            acc += block.count_ones();
        }
        let last_block = self.get_block(NOISE_BIT_BLOCK);
        acc += last_block.count_ones();
        acc
    }

    /// get the noisy inner product
    pub fn get_product(&self) -> bool {
        (self.sample[NOISE_BIT_BLOCK] >> NOISE_BIT_IDX) == 1
    }

    /// absorb another sample
    pub fn xor_into(&mut self, other: &Sample) {
        let before_a = self.get_product();
        let before_b = other.get_product();
        self.sample
            .iter_mut()
            .zip(other.sample.iter())
            .for_each(|(v1, v2)| *v1 ^= v2);
        debug_assert_eq!(self.get_product(), before_a ^ before_b);
    }

    /// set noise bit
    pub fn set_product(&mut self, new_product: bool) {
        self.sample[NOISE_BIT_BLOCK] &= !NOISE_BIT_MASK; // get without noise bit
        self.sample[NOISE_BIT_BLOCK] |= if new_product { ONE << NOISE_BIT_IDX } else { 0 };
    }

    /// Obtain the sample
    pub fn get_sample(&self) -> &[StorageBlock] {
        &self.sample
    }

    /// Obtain the sample mutably
    pub fn get_sample_mut(&mut self) -> &mut [StorageBlock] {
        &mut self.sample
    }

    /// Obtain
    pub fn into_inner(self) -> SampleStorage {
        self.sample
    }

    /// Truncate
    pub fn truncate(&mut self, len: usize, truncating_secret: bool) {
        let used_bits = len % bits_per_block();
        // If there are no unused bits, there's no need to perform masking.
        if used_bits > 0 {
            let off = block_offset(len);
            let msk = ((1 << used_bits) - 1)
                | if off == NOISE_BIT_BLOCK && !truncating_secret {
                    1 << NOISE_BIT_IDX
                } else {
                    0
                };
            let old_v = self.sample[off];
            let new_v = old_v & msk;
            if new_v != old_v {
                self.sample[off] = new_v;
            }
        }
        // zero out any other blocks
        ((block_offset(len) + 1)..SAMPLE_LEN).for_each(|idx| {
            if idx == NOISE_BIT_BLOCK && !truncating_secret {
                self.sample[NOISE_BIT_BLOCK] &= NOISE_BIT_MASK;
            } else {
                self.sample[idx] = 0
            }
        });
    }

    pub fn as_binvector(&self, len: usize) -> BinVector {
        let mut vec = BinVector::from_elem(len, false);
        let vecstorage = unsafe { vec.get_storage_mut() };
        vecstorage[..blocks_required(len)]
            .copy_from_slice(unsafe { std::mem::transmute(&self.sample[..blocks_required(len)]) });

        unsafe {
            vec.set_len(len);
            vec.mask_last_block();
        }
        vec
    }

    pub fn from_binvector(vec: &BinVector, product: bool) -> Sample {
        debug_assert!(vec.len() < MAX_K);
        let mut sample = Self::new();
        sample.sample[..blocks_required(vec.len())].copy_from_slice(unsafe {
            std::mem::transmute(&vec.get_storage()[..blocks_required(vec.len())])
        });
        if product {
            sample.set_product(product);
        }
        sample
    }

    pub fn set_from_binvec(&mut self, vec: &BinVector) {
        let product = self.get_product();
        let last_block = blocks_required(vec.len());
        self.sample[..last_block]
            .copy_from_slice(unsafe { std::mem::transmute(&vec.get_storage()[..last_block]) });
        self.truncate(vec.len(), true);
        self.set_product(product);
    }

    pub fn get_block(&self, index: usize) -> StorageBlock {
        if index == NOISE_BIT_BLOCK {
            self.sample[index] & !NOISE_BIT_MASK
        } else {
            self.sample[index]
        }
    }
}

/// This struct represents the oracle of the LPN problem.
///
/// We need to obtain the queries needed before applying reductions or transformations.
#[derive(Clone)]
pub struct LpnOracle {
    /// The samples held by this oracle.
    ///
    /// Can be obtained via `get_samples`
    pub samples: Vec<Sample>,
    /// The secret of this problem
    pub secret: Sample,
    /// The size of this problem
    k: usize,
    /// The bias of this problem
    pub delta: f64,
    /// The bias of the secret
    pub delta_s: f64,
    /// The transformation matrix used by the sparse secret reduction
    pub(crate) sparse_transform_matrix: Option<BinMatrix>,
    /// The vector used by the sparse secret reduction
    pub(crate) sparse_transform_vector: Option<BinVector>,
}

impl LpnOracle {
    /// Create a new LPN problem with a random secret
    pub fn new(k: u32, tau: f64) -> LpnOracle {
        let k = k as usize;
        assert!(
            MAX_K > k as usize,
            "We require K limit to be hardcoded, sorry. Max K for this build: {}",
            MAX_K
        );
        if blocks_required(k + 1) < blocks_required(MAX_K + 1) {
            log::warn!("WARNING: you could reduce MAX_K, be faster and use less memory!");
        }
        debug_assert!((0.0..1.0).contains(&tau), "0 <= tau < 1");
        debug_assert!(k > 0, "should have k > 0");
        let mut secret = Sample {
            sample: rand::random(),
        };
        secret.truncate(k, true);
        log::info!("Constructed Oracle with k={}, τ={:0.5}", k, tau);

        LpnOracle {
            samples: vec![],
            secret,
            k,
            delta: 1f64 - 2f64 * tau,
            delta_s: 0f64, // uniformly random
            sparse_transform_matrix: None,
            sparse_transform_vector: None,
        }
    }

    /// Create a new LPN problem with a set secret
    pub fn new_with_secret(secret: Sample, k: u32, tau: f64) -> LpnOracle {
        let mut lpn = Self::new(k, tau);
        lpn.secret = secret;
        lpn
    }

    /// Get new samples from the oracle
    ///
    /// These samples are stored in ``oracle.samples``
    ///
    /// Uses parallelism
    pub fn get_samples(&mut self, n: usize) {
        let mut input_samples = Vec::with_capacity(n);
        self.get_some_samples(&mut input_samples, n);
        self.samples.reserve_exact(input_samples.len());
        self.samples.extend(input_samples);
    }

    fn get_some_samples(&self, result: &mut Vec<Sample>, n: usize) {
        let k = self.k as usize;

        let tau = (1.0 - self.delta) / 2.0;
        let dist = Bernoulli::new(tau).unwrap();
        let secret = &self.secret;

        // allocate the space.
        result.reserve_exact(n);
        // obtain the vector itself as maybeuninit
        let samples = unsafe {
            std::mem::transmute::<&mut Vec<Sample>, &mut Vec<MaybeUninit<Sample>>>(result)
        };
        // we cheat the size by using set_len
        unsafe { samples.set_len(n) };
        result.shrink_to_fit();
        // bitbang some contents into you, multithreaded of course
        let chunk_size: usize = std::cmp::max(n / rayon::current_num_threads(), 10_000);
        samples.par_chunks_mut(chunk_size).for_each_init(
            || Hc128Rng::from_entropy(),
            |rng, samples| {
                let new_samples = MaybeUninit::slice_as_mut_ptr(samples) as *mut u8;
                let size = std::mem::size_of::<[StorageBlock; SAMPLE_LEN]>();
                let new_samples =
                    unsafe { std::slice::from_raw_parts_mut(new_samples, size * samples.len()) };
                rng.fill_bytes(new_samples);
            },
        );

        // these have now been initialized
        let samples = unsafe {
            std::mem::transmute::<&mut Vec<MaybeUninit<Sample>>, &mut Vec<Sample>>(samples)
        };

        if block_offset(k) < NOISE_BIT_BLOCK {
            samples.par_iter_mut().for_each_init(
                || Hc128Rng::from_entropy(),
                |rng, sample| {
                    let noise_bit = dist.sample(rng);
                    sample.sample[(block_offset(k) + 1)..SAMPLE_LEN]
                        .iter_mut()
                        .for_each(|block| *block = 0);
                    sample.sample[block_offset(k)] &= (ONE << (k % bits_per_block())) - 1;
                    let product = sample.vector_product(&secret, k) ^ noise_bit;
                    if product {
                        sample.sample[NOISE_BIT_BLOCK] |= NOISE_BIT_MASK;
                    }
                },
            );
        } else {
            samples.par_iter_mut().for_each_init(
                || Hc128Rng::from_entropy(),
                |rng, sample| {
                    sample.sample[NOISE_BIT_BLOCK] &= (ONE << (k % bits_per_block())) - 1;
                    let noise_bit = dist.sample(rng);
                    let product = sample.vector_product(&secret, k) ^ noise_bit;
                    if product {
                        sample.sample[NOISE_BIT_BLOCK] |= ONE << NOISE_BIT_IDX;
                    }
                },
            );
        }
        if cfg!(debug_assertions) {
            let max_k = cmp::min(k + 10, MAX_K);
            for sample in samples {
                debug_assert_eq!(query_bits_range(sample, k..max_k), 0);
            }
        }
    }

    /// Get samples from the oracle with a trailing number of zero bits
    ///
    /// Uses parallelism
    pub fn get_samples_drop(&mut self, n: usize, trailing_zeros: usize) {
        let original_len = self.samples.len();
        log::trace!(
            "Getting additional samples until we have {} (2^{}) that have {} trailing zeros",
            n,
            log_2(n),
            trailing_zeros
        );
        let k = self.k;

        let progress = ProgressBar::new(n as u64);
        progress.println("Getting samples");
        progress.reset();

        if sys_info::mem_info().is_err() {
            log::warn!("meminfo failed, only fetching max 2^28 samples");
        }

        let mut input_vec = Vec::new();
        const SAMPLE_SIZE: usize = mem::size_of::<Sample>();
        while (self.samples.len() - original_len) < n {
            // do some minimal amount of samples to reduce short iterations
            let samples_to_get = n - (self.samples.len() - original_len);

            let samples_to_get = if let Ok(meminfo) = sys_info::mem_info() {
                std::cmp::min(
                    samples_to_get << trailing_zeros,
                    // include the current capacity, otherwise we only use a third or so of RAM
                    ((input_vec.capacity() * SAMPLE_SIZE) + ((meminfo.free * 1000) as usize))
                        / (2 * SAMPLE_SIZE),
                )
            } else {
                std::cmp::min(samples_to_get << trailing_zeros, 2usize.pow(28))
            };
            // get_some_samples manages the size of input_vec.
            self.get_some_samples(&mut input_vec, samples_to_get);
            let before_extend = self.samples.len();
            self.samples.par_extend(
                input_vec
                    .par_drain(..)
                    .filter(|sample| are_last_bits_zero(sample, k, trailing_zeros)),
            );
            progress.inc((self.samples.len() - before_extend) as u64);
        }
        self.samples.truncate(n);
        self.samples.shrink_to_fit();
        self.k -= trailing_zeros;
    }

    pub fn get_k(&self) -> usize {
        self.k
    }

    /// Override what the value of k is, without running truncate.
    pub unsafe fn set_k(&mut self, new_k: usize) {
        self.k = new_k
    }

    /// Updates the problem to have fewer bits
    pub fn truncate(&mut self, new_k: usize) {
        // update k
        let traverses_blocks = block_offset(self.k) > block_offset(new_k);
        self.k = new_k;

        let used_bits = new_k % bits_per_block();
        // If there are no unused bits, there's no need to perform masking.
        if used_bits > 0 {
            let off = block_offset(new_k);
            let msk = ((1 << used_bits) - 1)
                | if off == NOISE_BIT_BLOCK {
                    1 << NOISE_BIT_IDX
                } else {
                    0
                };

            self.samples.par_iter_mut().for_each(|sample| {
                let old_v = sample.sample[off];
                let new_v = old_v & msk;
                if new_v != old_v {
                    sample.sample[off] = new_v;
                }
            });
        }

        if traverses_blocks {
            // zero out any other blocks
            let start = block_offset(new_k) + 1;
            self.samples.par_iter_mut().for_each(|sample| {
                (start..SAMPLE_LEN).for_each(|idx| {
                    if idx == NOISE_BIT_BLOCK {
                        // only preserves the noise bit
                        sample.sample[NOISE_BIT_BLOCK] &= NOISE_BIT_MASK;
                    } else {
                        sample.sample[idx] = 0
                    }
                })
            });
        }

        self.secret.truncate(new_k, true);
    }
}

#[inline]
pub fn are_last_bits_zero(b: &Sample, k: usize, n_bits: usize) -> bool {
    n_bits == 0 || query_bits_range(b, k - n_bits..k) == 0
}

#[inline]
pub(crate) fn query_bits_range(b: &Sample, range: Range<usize>) -> u64 {
    debug_assert!(range.len() <= 64);

    let b1 = b.get_block(block_offset(range.start));
    let bits_prefixing_b1 = range.start % bits_per_block();
    let mut b1 = b1 >> bits_prefixing_b1;
    let bits_in_b1 = std::cmp::min(64 - bits_prefixing_b1, range.len());
    // mask b1
    if bits_in_b1 != 64 {
        let mask = (ONE << bits_in_b1) - 1;
        b1 = b1 & mask;
    }
    let remaining_bits = range.len() - bits_in_b1;
    if remaining_bits > 0 {
        let mask = (ONE << remaining_bits) - 1;
        let b2 = b.get_block(block_offset(range.end - 1)) & mask;
        b1 |= b2 << bits_in_b1;
    }

    b1 as u64
}

#[cfg(test)]
mod test {
    use rand::prelude::*;

    use super::*;
    pub(crate) fn query_bits_range_ref(v: &Sample, range: Range<usize>) -> u64 {
        debug_assert!(range.len() <= 64, "Needs to fit in u64");
        let len = range.len();
        let mut result = 0u64;
        for (i, ri) in range.enumerate() {
            let inner_offset = ri % bits_per_block();
            result |= (((v.sample[block_offset(ri)] >> inner_offset) as u64) & 1) << i;
        }
        debug_assert_eq!(
            result.wrapping_shr(len as u32),
            if len == 64 { result } else { 0 }
        );
        result
    }

    #[test]
    fn bitrange_reference() {
        let v = Sample {
            sample: [0b1000_1001; SAMPLE_LEN],
        };
        assert_eq!(query_bits_range_ref(&v, 0..64), 0b1000_1001);
        assert_eq!(query_bits_range_ref(&v, 0..3), 0b0000_0001);
        assert_eq!(query_bits_range_ref(&v, 2..4), 0b0000_0010);
        assert_eq!(query_bits_range_ref(&v, 3..4), 0b0000_0001);
        assert_eq!(query_bits_range_ref(&v, 3..6), 0b0000_0001);
        assert_eq!(query_bits_range_ref(&v, 3..8), 0b0001_0001);
        assert_eq!(query_bits_range_ref(&v, 64..67), 0b0000_0001);
        assert_eq!(query_bits_range_ref(&v, 63..71), 0b0001_0010);
    }

    #[test]
    fn bitrange() {
        let v = Sample {
            sample: [0b1000_1001; SAMPLE_LEN],
        };
        assert_eq!(query_bits_range(&v, 0..64), 0b1000_1001);
        assert_eq!(query_bits_range(&v, 0..3), 0b0000_0001);
        assert_eq!(query_bits_range(&v, 2..4), 0b0000_00010);
        assert_eq!(query_bits_range(&v, 3..4), 0b0000_0001);
        assert_eq!(query_bits_range(&v, 3..6), 0b0000_0001);
        assert_eq!(query_bits_range(&v, 3..8), 0b0001_0001);
        assert_eq!(query_bits_range(&v, 64..67), 0b0000_0001);
        assert_eq!(query_bits_range(&v, 63..71), 0b0001_0010);
    }

    #[test]
    fn bitrange_generated() {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let vec: [StorageBlock; SAMPLE_LEN] = rng.gen();
            let start = rng.gen_range(0..(vec.len() * bits_per_block() - 1));
            let end = rng.gen_range(
                (start + 1)..std::cmp::min(start + bits_per_block(), vec.len() * bits_per_block()),
            );
            let range = start..end;
            let sample = Sample { sample: vec };
            assert_eq!(
                query_bits_range(&sample, range.clone()),
                query_bits_range_ref(&sample, range.clone()),
                "failed for range: {:?}\non sample {:?}",
                range,
                sample.sample
            );
        }
    }

    #[test]
    fn sample_from_binvec_and_back() {
        let rng = &mut rand::thread_rng();
        for _ in 0..10000 {
            let k = rng.gen_range(0..(MAX_K - 10));
            let vec = BinVector::random(k);
            let mut sample = Sample::new();
            sample.set_from_binvec(&vec);
            assert_eq!(vec, sample.as_binvector(k));
            assert_eq!(vec, Sample::from_binvector(&vec, false).as_binvector(k));
            assert_eq!(vec, Sample::from_binvector(&vec, true).as_binvector(k));
        }
    }

    #[test]
    fn test_from_binvec() {
        let binvec = BinVector::from_bytes(&[0b001000]);
        let sample = Sample::from_binvector(&binvec, false);
        assert_eq!(sample.get_block(0) as usize, binvec.get_storage()[0]);
    }
}
