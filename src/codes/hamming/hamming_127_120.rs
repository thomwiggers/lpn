use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[127, 120]`` Hamming code
///
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct HammingCode127_120;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 2]> = 0 as *const FnvHashMap<u64, &'static [usize; 2]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1, 6917529027641081856 ],
                &[ 2, 5764607523034234880 ],
                &[ 4, 3458764513820540928 ],
                &[ 8, 5188146770730811392 ],
                &[ 16, 2882303761517117440 ],
                &[ 32, 1729382256910270464 ],
                &[ 64, 8646911284551352320 ],
                &[ 128, 4899916394579099648 ],
                &[ 256, 2594073385365405696 ],
                &[ 512, 1441151880758558720 ],
                &[ 1024, 8358680908399640576 ],
                &[ 2048, 864691128455135232 ],
                &[ 4096, 7782220156096217088 ],
                &[ 8192, 6629298651489370112 ],
                &[ 16384, 4323455642275676160 ],
                &[ 32768, 4755801206503243776 ],
                &[ 65536, 2449958197289549824 ],
                &[ 131072, 1297036692682702848 ],
                &[ 262144, 8214565720323784704 ],
                &[ 524288, 720575940379279360 ],
                &[ 1048576, 7638104968020361216 ],
                &[ 2097152, 6485183463413514240 ],
                &[ 4194304, 4179340454199820288 ],
                &[ 8388608, 432345564227567616 ],
                &[ 16777216, 7349874591868649472 ],
                &[ 33554432, 6196953087261802496 ],
                &[ 67108864, 3891110078048108544 ],
                &[ 134217728, 5620492334958379008 ],
                &[ 268435456, 3314649325744685056 ],
                &[ 536870912, 2161727821137838080 ],
                &[ 1073741824, 9079256848778919936 ],
                &[ 2147483648, 4683743612465315840 ],
                &[ 4294967296, 2377900603251621888 ],
                &[ 8589934592, 1224979098644774912 ],
                &[ 17179869184, 8142508126285856768 ],
                &[ 34359738368, 648518346341351424 ],
                &[ 68719476736, 7566047373982433280 ],
                &[ 137438953472, 6413125869375586304 ],
                &[ 274877906944, 4107282860161892352 ],
                &[ 549755813888, 360287970189639680 ],
                &[ 1099511627776, 7277816997830721536 ],
                &[ 2199023255552, 6124895493223874560 ],
                &[ 4398046511104, 3819052484010180608 ],
                &[ 8796093022208, 5548434740920451072 ],
                &[ 17592186044416, 3242591731706757120 ],
                &[ 35184372088832, 2089670227099910144 ],
                &[ 70368744177664, 9007199254740992000 ],
                &[ 140737488355328, 216172782113783808 ],
                &[ 281474976710656, 7133701809754865664 ],
                &[ 562949953421312, 5980780305148018688 ],
                &[ 1125899906842624, 3674937295934324736 ],
                &[ 2251799813685248, 5404319552844595200 ],
                &[ 4503599627370496, 3098476543630901248 ],
                &[ 9007199254740992, 1945555039024054272 ],
                &[ 18014398509481984, 8863084066665136128 ],
                &[ 36028797018963968, 5116089176692883456 ],
                &[ 72057594037927936, 2810246167479189504 ],
                &[ 144115188075855872, 1657324662872342528 ],
                &[ 288230376151711744, 8574853690513424384 ],
                &[ 576460752303423488, 1080863910568919040 ],
                &[ 1152921504606846976, 7998392938210000896 ],
                &[ 2305843009213693952, 6845471433603153920 ],
                &[ 4611686018427387904, 4539628424389459968 ],
                &[ 9223372036854775808, 9151314442816847872 ],
                &[ 0, 2233785415175766017 ],
                &[ 0, 3386706919782612994 ],
                &[ 0, 5692549928996306948 ],
                &[ 0, 3963167672086036488 ],
                &[ 0, 6269010681299730448 ],
                &[ 0, 7421932185906577440 ],
                &[ 0, 504403158265495616 ],
                &[ 0, 4251398048237748352 ],
                &[ 0, 6557241057451442432 ],
                &[ 0, 7710162562058289664 ],
                &[ 0, 792633534417208320 ],
                &[ 0, 8286623314361714688 ],
                &[ 0, 1369094286720634880 ],
                &[ 0, 2522015791327485952 ],
                &[ 0, 4827858800541188096 ],
                &[ 0, 4395513236313636864 ],
                &[ 0, 6701356245527363584 ],
                &[ 0, 7854277750134276096 ],
                &[ 0, 936748722493325312 ],
                &[ 0, 8430738502438092800 ],
                &[ 0, 1513209474797535232 ],
                &[ 0, 2666130979405430784 ],
                &[ 0, 4971973988621221888 ],
                &[ 0, 8718968878597668864 ],
                &[ 0, 1801439850964975616 ],
                &[ 0, 2954361355588599808 ],
                &[ 0, 5260204364835848192 ],
                &[ 0, 3530822107992686592 ],
                &[ 0, 5836665117340598272 ],
                &[ 0, 6989586622215880704 ],
                &[ 0, 4467570831425273856 ],
                &[ 0, 6773413841712709632 ],
                &[ 0, 7926335348467040256 ],
                &[ 0, 1008806325120925696 ],
                &[ 0, 8502796113655365632 ],
                &[ 0, 1585267103194152960 ],
                &[ 0, 2738188642160738304 ],
                &[ 0, 5044031720093908992 ],
                &[ 0, 8791026747505115136 ],
                &[ 0, 1873497994741940224 ],
                &[ 0, 3026420049104601088 ],
                &[ 0, 5332264157829922816 ],
                &[ 0, 3602884099942907904 ],
                &[ 0, 5908731507203112960 ],
                &[ 0, 7061661807902982144 ],
                &[ 0, 8935176845075152896 ],
                &[ 0, 2017683001806159872 ],
                &[ 0, 3170674875157184512 ],
                &[ 0, 5476658621859233792 ],
                &[ 0, 3747557839925673984 ],
                &[ 0, 6053963799092789248 ],
                &[ 0, 7208011203606478848 ],
                &[ 0, 4039728865751334912 ],
                &[ 0, 6350075474592399360 ],
                &[ 0, 7512004178453987328 ],
                &[ 0, 8106479329266892800 ],
                
            ], 127));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 6148914691236517205, 6855968104251741525 ],
                &[ 7378697629483820646, 8019102459406739046 ],
                &[ 8680820740569200760, 8614854182177372280 ],
                &[ 9187483429707480960, 8930672971635261312 ],
                &[ 9223231301513871360, 9079221665480540160 ],
                &[ 9223372034707292160, 9151314441743106048 ],
                &[ 9223372036854775808, 9223372036854775807 ],
                
            ], 127));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(128, Default::default()));
            map.insert(0, &[0, 0]);     // 0 => [0, 0]
            map.insert(1, &[1, 0]);     // 1 => [1, 0]
            map.insert(2, &[2, 0]);     // 2 => [2, 0]
            map.insert(3, &[4, 0]);     // 3 => [4, 0]
            map.insert(4, &[8, 0]);     // 4 => [8, 0]
            map.insert(5, &[16, 0]);     // 5 => [16, 0]
            map.insert(6, &[32, 0]);     // 6 => [32, 0]
            map.insert(7, &[64, 0]);     // 7 => [64, 0]
            map.insert(8, &[128, 0]);     // 8 => [128, 0]
            map.insert(9, &[256, 0]);     // 9 => [256, 0]
            map.insert(10, &[512, 0]);     // 10 => [512, 0]
            map.insert(11, &[1024, 0]);     // 11 => [1024, 0]
            map.insert(12, &[2048, 0]);     // 12 => [2048, 0]
            map.insert(13, &[4096, 0]);     // 13 => [4096, 0]
            map.insert(14, &[8192, 0]);     // 14 => [8192, 0]
            map.insert(15, &[16384, 0]);     // 15 => [16384, 0]
            map.insert(16, &[32768, 0]);     // 16 => [32768, 0]
            map.insert(17, &[65536, 0]);     // 17 => [65536, 0]
            map.insert(18, &[131072, 0]);     // 18 => [131072, 0]
            map.insert(19, &[262144, 0]);     // 19 => [262144, 0]
            map.insert(20, &[524288, 0]);     // 20 => [524288, 0]
            map.insert(21, &[1048576, 0]);     // 21 => [1048576, 0]
            map.insert(22, &[2097152, 0]);     // 22 => [2097152, 0]
            map.insert(23, &[4194304, 0]);     // 23 => [4194304, 0]
            map.insert(24, &[8388608, 0]);     // 24 => [8388608, 0]
            map.insert(25, &[16777216, 0]);     // 25 => [16777216, 0]
            map.insert(26, &[33554432, 0]);     // 26 => [33554432, 0]
            map.insert(27, &[67108864, 0]);     // 27 => [67108864, 0]
            map.insert(28, &[134217728, 0]);     // 28 => [134217728, 0]
            map.insert(29, &[268435456, 0]);     // 29 => [268435456, 0]
            map.insert(30, &[536870912, 0]);     // 30 => [536870912, 0]
            map.insert(31, &[1073741824, 0]);     // 31 => [1073741824, 0]
            map.insert(32, &[2147483648, 0]);     // 32 => [2147483648, 0]
            map.insert(33, &[4294967296, 0]);     // 33 => [4294967296, 0]
            map.insert(34, &[8589934592, 0]);     // 34 => [8589934592, 0]
            map.insert(35, &[17179869184, 0]);     // 35 => [17179869184, 0]
            map.insert(36, &[34359738368, 0]);     // 36 => [34359738368, 0]
            map.insert(37, &[68719476736, 0]);     // 37 => [68719476736, 0]
            map.insert(38, &[137438953472, 0]);     // 38 => [137438953472, 0]
            map.insert(39, &[274877906944, 0]);     // 39 => [274877906944, 0]
            map.insert(40, &[549755813888, 0]);     // 40 => [549755813888, 0]
            map.insert(41, &[1099511627776, 0]);     // 41 => [1099511627776, 0]
            map.insert(42, &[2199023255552, 0]);     // 42 => [2199023255552, 0]
            map.insert(43, &[4398046511104, 0]);     // 43 => [4398046511104, 0]
            map.insert(44, &[8796093022208, 0]);     // 44 => [8796093022208, 0]
            map.insert(45, &[17592186044416, 0]);     // 45 => [17592186044416, 0]
            map.insert(46, &[35184372088832, 0]);     // 46 => [35184372088832, 0]
            map.insert(47, &[70368744177664, 0]);     // 47 => [70368744177664, 0]
            map.insert(48, &[140737488355328, 0]);     // 48 => [140737488355328, 0]
            map.insert(49, &[281474976710656, 0]);     // 49 => [281474976710656, 0]
            map.insert(50, &[562949953421312, 0]);     // 50 => [562949953421312, 0]
            map.insert(51, &[1125899906842624, 0]);     // 51 => [1125899906842624, 0]
            map.insert(52, &[2251799813685248, 0]);     // 52 => [2251799813685248, 0]
            map.insert(53, &[4503599627370496, 0]);     // 53 => [4503599627370496, 0]
            map.insert(54, &[9007199254740992, 0]);     // 54 => [9007199254740992, 0]
            map.insert(55, &[18014398509481984, 0]);     // 55 => [18014398509481984, 0]
            map.insert(56, &[36028797018963968, 0]);     // 56 => [36028797018963968, 0]
            map.insert(57, &[72057594037927936, 0]);     // 57 => [72057594037927936, 0]
            map.insert(58, &[144115188075855872, 0]);     // 58 => [144115188075855872, 0]
            map.insert(59, &[288230376151711744, 0]);     // 59 => [288230376151711744, 0]
            map.insert(60, &[576460752303423488, 0]);     // 60 => [576460752303423488, 0]
            map.insert(61, &[1152921504606846976, 0]);     // 61 => [1152921504606846976, 0]
            map.insert(62, &[2305843009213693952, 0]);     // 62 => [2305843009213693952, 0]
            map.insert(63, &[4611686018427387904, 0]);     // 63 => [4611686018427387904, 0]
            map.insert(64, &[9223372036854775808, 0]);     // 64 => [9223372036854775808, 0]
            map.insert(65, &[0, 1]);     // 65 => [0, 1]
            map.insert(66, &[0, 2]);     // 66 => [0, 2]
            map.insert(67, &[0, 4]);     // 67 => [0, 4]
            map.insert(68, &[0, 8]);     // 68 => [0, 8]
            map.insert(69, &[0, 16]);     // 69 => [0, 16]
            map.insert(70, &[0, 32]);     // 70 => [0, 32]
            map.insert(71, &[0, 64]);     // 71 => [0, 64]
            map.insert(72, &[0, 128]);     // 72 => [0, 128]
            map.insert(73, &[0, 256]);     // 73 => [0, 256]
            map.insert(74, &[0, 512]);     // 74 => [0, 512]
            map.insert(75, &[0, 1024]);     // 75 => [0, 1024]
            map.insert(76, &[0, 2048]);     // 76 => [0, 2048]
            map.insert(77, &[0, 4096]);     // 77 => [0, 4096]
            map.insert(78, &[0, 8192]);     // 78 => [0, 8192]
            map.insert(79, &[0, 16384]);     // 79 => [0, 16384]
            map.insert(80, &[0, 32768]);     // 80 => [0, 32768]
            map.insert(81, &[0, 65536]);     // 81 => [0, 65536]
            map.insert(82, &[0, 131072]);     // 82 => [0, 131072]
            map.insert(83, &[0, 262144]);     // 83 => [0, 262144]
            map.insert(84, &[0, 524288]);     // 84 => [0, 524288]
            map.insert(85, &[0, 1048576]);     // 85 => [0, 1048576]
            map.insert(86, &[0, 2097152]);     // 86 => [0, 2097152]
            map.insert(87, &[0, 4194304]);     // 87 => [0, 4194304]
            map.insert(88, &[0, 8388608]);     // 88 => [0, 8388608]
            map.insert(89, &[0, 16777216]);     // 89 => [0, 16777216]
            map.insert(90, &[0, 33554432]);     // 90 => [0, 33554432]
            map.insert(91, &[0, 67108864]);     // 91 => [0, 67108864]
            map.insert(92, &[0, 134217728]);     // 92 => [0, 134217728]
            map.insert(93, &[0, 268435456]);     // 93 => [0, 268435456]
            map.insert(94, &[0, 536870912]);     // 94 => [0, 536870912]
            map.insert(96, &[0, 1073741824]);     // 96 => [0, 1073741824]
            map.insert(97, &[0, 2147483648]);     // 97 => [0, 2147483648]
            map.insert(98, &[0, 4294967296]);     // 98 => [0, 4294967296]
            map.insert(99, &[0, 8589934592]);     // 99 => [0, 8589934592]
            map.insert(100, &[0, 17179869184]);     // 100 => [0, 17179869184]
            map.insert(101, &[0, 34359738368]);     // 101 => [0, 34359738368]
            map.insert(102, &[0, 68719476736]);     // 102 => [0, 68719476736]
            map.insert(103, &[0, 137438953472]);     // 103 => [0, 137438953472]
            map.insert(104, &[0, 274877906944]);     // 104 => [0, 274877906944]
            map.insert(105, &[0, 549755813888]);     // 105 => [0, 549755813888]
            map.insert(106, &[0, 1099511627776]);     // 106 => [0, 1099511627776]
            map.insert(107, &[0, 2199023255552]);     // 107 => [0, 2199023255552]
            map.insert(108, &[0, 4398046511104]);     // 108 => [0, 4398046511104]
            map.insert(109, &[0, 8796093022208]);     // 109 => [0, 8796093022208]
            map.insert(110, &[0, 17592186044416]);     // 110 => [0, 17592186044416]
            map.insert(112, &[0, 35184372088832]);     // 112 => [0, 35184372088832]
            map.insert(113, &[0, 70368744177664]);     // 113 => [0, 70368744177664]
            map.insert(114, &[0, 140737488355328]);     // 114 => [0, 140737488355328]
            map.insert(115, &[0, 281474976710656]);     // 115 => [0, 281474976710656]
            map.insert(116, &[0, 562949953421312]);     // 116 => [0, 562949953421312]
            map.insert(117, &[0, 1125899906842624]);     // 117 => [0, 1125899906842624]
            map.insert(118, &[0, 2251799813685248]);     // 118 => [0, 2251799813685248]
            map.insert(120, &[0, 4503599627370496]);     // 120 => [0, 4503599627370496]
            map.insert(121, &[0, 9007199254740992]);     // 121 => [0, 9007199254740992]
            map.insert(122, &[0, 18014398509481984]);     // 122 => [0, 18014398509481984]
            map.insert(124, &[0, 36028797018963968]);     // 124 => [0, 36028797018963968]
            map.insert(95, &[0, 72057594037927936]);     // 95 => [0, 72057594037927936]
            map.insert(111, &[0, 144115188075855872]);     // 111 => [0, 144115188075855872]
            map.insert(119, &[0, 288230376151711744]);     // 119 => [0, 288230376151711744]
            map.insert(123, &[0, 576460752303423488]);     // 123 => [0, 576460752303423488]
            map.insert(125, &[0, 1152921504606846976]);     // 125 => [0, 1152921504606846976]
            map.insert(126, &[0, 2305843009213693952]);     // 126 => [0, 2305843009213693952]
            map.insert(127, &[0, 4611686018427387904]);     // 127 => [0, 4611686018427387904]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl HammingCode127_120 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for HammingCode127_120 {
    fn name(&self) -> String {
        "[127, 120] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        127
    }

    fn dimension(&self) -> usize {
        120
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
        let he = c * self.parity_check_matrix_transposed();
        let mut error = BinVector::with_capacity(127);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 127 / 64 + if 127 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(127) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(120);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[127 / 64] & !((1 << 63) - 1), 0, "this message has excess bits");

        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        let he = &BinMatrix::from_slices(&[&c[..]], self.length()) * self.parity_check_matrix_transposed();
        let error = map[unsafe { &he.get_word_unchecked(0, 0) }];
        c.iter_mut().zip(error.iter().copied()).for_each(|(sample, error)| *sample ^= error as u64);
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1f64 + f64::from(127) * delta) / f64::from(127 + 1)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;
    use crate::oracle::Sample;

    #[test]
    fn size() {
        let code = HammingCode127_120.generator_matrix();
        assert_eq!(code.ncols(), 127);
        assert_eq!(code.nrows(), 120);
    }

    #[test]
    fn test_decode_sample() {
        let code = HammingCode127_120;
        for _ in 0..1000 {
            // setup
            let vec = BinVector::random(code.length());
            let mut sample_a = Sample::from_binvector(&vec, false);
            let mut sample_b = Sample::from_binvector(&vec, true);
            
            let decoded_vec = code.decode_to_message(&vec).unwrap();
            println!("decoded_vec: {:?}", decoded_vec);

            // test vectors
            let decoded_vec_sample_a = Sample::from_binvector(&decoded_vec, false);
            let decoded_vec_sample_b = Sample::from_binvector(&decoded_vec, true);

            code.decode_sample(&mut sample_a);
            code.decode_sample(&mut sample_b);
            assert_eq!(sample_a.get_product(), false);
            assert_eq!(sample_b.get_product(), true);
            assert_eq!(sample_a, decoded_vec_sample_a);
            assert_eq!(sample_b, decoded_vec_sample_b);
        }
    }

    #[test]
    fn random_decode_tests() {

        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, false, true, false, true, true, false, true, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, true, false, false, true, true, false, false, true, true, false, true, false, false, true, false, true, true, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, true, false, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, false, true, false, true, true, false, true, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, true, false, false, true, true, false, false, true, true, false, true, false, false, true, false, true, true, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, true, true, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, false, true, true, false, true, false, true, false, false, true, false, true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, true, true, true, true, true, false, true, true, false, false, true, true, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, true, false, true, false, false, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, false, true, true, true, true, true, false, false, false, true, false, false, true, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, true, true, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, true, false, true, false, false, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, false, true, true, true, true, true, false, false, false, true, false, false, true, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, true, true, false, false, false, false, false, true, false, true, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, true, false, true, false, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, false, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, true, false, false, true, true, true, false, true, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, true, true, true, true, false, true, true, false, true, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, false, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, true, false, false, true, true, true, false, true, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, false, false, true, true, true, false, true, false, true, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, true, true, true, true, false, true, true, false, true, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, false, false, true, false, false, true, true, true, true, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true, true, true, false, true, true, false, true, true, true, true, true, false, true, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true, true, true, false, true, true, false, true, true, true, true, true, false, true, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, false, true, true, true, false, true, true, false, true, true, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, false, true, false, true, false, false, false, false, true, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, true, true, true, false, true, true, false, false, true, false, true, true, false, true, true, true, false, true, false, true, true, true, true, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, false, true, true, true, false, true, true, false, true, true, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, true, true, true, false, true, true, false, false, true, false, true, true, false, true, true, true, false, true, false, true, true, true, true, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, false, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, false, true, false, false, true, true, false, true, false, true, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, false, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, false, true, true, false, false, true, false, true, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, false, true, true, true, true, false, false, true, false, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, false, false, true, true, false, false, true, false, true, false, false, true, true, true, true, true, true, false, false, true, true, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, false, true, true, true, true, false, false, true, false, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, true, false, true, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, false, false, false, false, false, true, true, false, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, true, false, true, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, true, true, false, true, true, true, false, true, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, false, false, false, false, false, true, true, false, false, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, true, true, false, false, true, false, false, true, false, true, false, false, false, true, true, false, false, true, true, true, true, false, false, true, true, true, false, true, true, false, false, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, true, true, false, false, true, false, false, true, false, true, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, true, false, false, false, true, true, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, false, false, false, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, false, true, false, false, true, false, false, true, true, true, false, false, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, false, false, false, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, false, true, false, false, true, false, false, true, true, true, true, false, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, true, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, true, true, false, false, true, false, false, false, false, false, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false, true, true, false, false, true, true, false, false, false, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, false, true, false, true, false, true, true, false, true, false, false, true, true, true, true, true, true, false, true, false, true, false, true, true, true, true, false, true, true, false, false, false, true, true, false, true, true, false, false, true, false, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, true, false, true, true, true, false, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, false, true, false, true, false, true, true, false, true, false, false, true, true, true, true, true, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, false, true, true, false, true, true, false, false, true, false, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, true, false, true, true, true, false, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, false, false, false, false, true, true, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, false, false, false, false, false, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, false, false, false, false, false, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, true, true, true, false, false, true, true, false, true, true, true, false, true, true, true, true, false, false, false, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, false, false, false, false, false, false, true, false, true, true, false, true, true, false, false, true, false, true, false, true, false, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, true, true, true, false, false, true, true, false, true, true, true, false, true, true, true, true, false, false, false, true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, false, false, false, false, false, false, true, false, true, true, false, true, true, false, false, true, false, true, false, true, false, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, true, true, false, true, false, false, true, false, false, false, true, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, false, true, false, true, false, false, false, true, true, true, true, false, true, true, false, true, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, true, true, false, true, false, false, true, false, false, false, true, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, true, true, false, false, false, false, false, true, false, true, false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, true, true, true, false, false, true, false, false, true, true, false, true, true, false, false, true, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, true, false, false, false, false, true, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, true, true, true, false, false, true, false, false, true, true, false, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, true, false, false, false, false, true, true, true, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, true, false, false, false, false, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, false, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, false, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, true, false, true, false, true, true, false, false, true, false, false, true, true, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, true, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, false, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, true, false, true, false, true, true, false, false, true, false, false, true, true, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true, true, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, true, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, false, false, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode127_120;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, false, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, true, true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, false, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, true, true, true, false, false, true, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, true, true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
