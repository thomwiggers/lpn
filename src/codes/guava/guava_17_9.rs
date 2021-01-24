use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[17, 9]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode17_9;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 110081 ],
                &[ 61954 ],
                &[ 58372 ],
                &[ 91656 ],
                &[ 117776 ],
                &[ 104480 ],
                &[ 40512 ],
                &[ 15488 ],
                &[ 120576 ],
                
            ], 17));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 94721 ],
                &[ 123906 ],
                &[ 61700 ],
                &[ 121608 ],
                &[ 36624 ],
                &[ 75552 ],
                &[ 85568 ],
                &[ 105600 ],
                
            ], 17));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(256, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(16, &[16]);     // 16 => [16]
            map.insert(32, &[32]);     // 32 => [32]
            map.insert(64, &[64]);     // 64 => [64]
            map.insert(128, &[128]);     // 128 => [128]
            map.insert(60, &[256]);     // 60 => [256]
            map.insert(121, &[512]);     // 121 => [512]
            map.insert(242, &[1024]);     // 242 => [1024]
            map.insert(216, &[2048]);     // 216 => [2048]
            map.insert(141, &[4096]);     // 141 => [4096]
            map.insert(39, &[8192]);     // 39 => [8192]
            map.insert(79, &[16384]);     // 79 => [16384]
            map.insert(158, &[32768]);     // 158 => [32768]
            map.insert(235, &[65536]);     // 235 => [65536]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(61, &[257]);     // 61 => [257]
            map.insert(120, &[513]);     // 120 => [513]
            map.insert(243, &[1025]);     // 243 => [1025]
            map.insert(217, &[2049]);     // 217 => [2049]
            map.insert(140, &[4097]);     // 140 => [4097]
            map.insert(38, &[8193]);     // 38 => [8193]
            map.insert(78, &[16385]);     // 78 => [16385]
            map.insert(159, &[32769]);     // 159 => [32769]
            map.insert(234, &[65537]);     // 234 => [65537]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(62, &[258]);     // 62 => [258]
            map.insert(123, &[514]);     // 123 => [514]
            map.insert(240, &[1026]);     // 240 => [1026]
            map.insert(218, &[2050]);     // 218 => [2050]
            map.insert(143, &[4098]);     // 143 => [4098]
            map.insert(37, &[8194]);     // 37 => [8194]
            map.insert(77, &[16386]);     // 77 => [16386]
            map.insert(156, &[32770]);     // 156 => [32770]
            map.insert(233, &[65538]);     // 233 => [65538]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(56, &[260]);     // 56 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(246, &[1028]);     // 246 => [1028]
            map.insert(220, &[2052]);     // 220 => [2052]
            map.insert(137, &[4100]);     // 137 => [4100]
            map.insert(35, &[8196]);     // 35 => [8196]
            map.insert(75, &[16388]);     // 75 => [16388]
            map.insert(154, &[32772]);     // 154 => [32772]
            map.insert(239, &[65540]);     // 239 => [65540]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(52, &[264]);     // 52 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(250, &[1032]);     // 250 => [1032]
            map.insert(208, &[2056]);     // 208 => [2056]
            map.insert(133, &[4104]);     // 133 => [4104]
            map.insert(47, &[8200]);     // 47 => [8200]
            map.insert(71, &[16392]);     // 71 => [16392]
            map.insert(150, &[32776]);     // 150 => [32776]
            map.insert(227, &[65544]);     // 227 => [65544]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(44, &[272]);     // 44 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(226, &[1040]);     // 226 => [1040]
            map.insert(200, &[2064]);     // 200 => [2064]
            map.insert(157, &[4112]);     // 157 => [4112]
            map.insert(55, &[8208]);     // 55 => [8208]
            map.insert(95, &[16400]);     // 95 => [16400]
            map.insert(142, &[32784]);     // 142 => [32784]
            map.insert(251, &[65552]);     // 251 => [65552]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(28, &[288]);     // 28 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(210, &[1056]);     // 210 => [1056]
            map.insert(248, &[2080]);     // 248 => [2080]
            map.insert(173, &[4128]);     // 173 => [4128]
            map.insert(7, &[8224]);     // 7 => [8224]
            map.insert(111, &[16416]);     // 111 => [16416]
            map.insert(190, &[32800]);     // 190 => [32800]
            map.insert(203, &[65568]);     // 203 => [65568]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(124, &[320]);     // 124 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(178, &[1088]);     // 178 => [1088]
            map.insert(152, &[2112]);     // 152 => [2112]
            map.insert(205, &[4160]);     // 205 => [4160]
            map.insert(103, &[8256]);     // 103 => [8256]
            map.insert(15, &[16448]);     // 15 => [16448]
            map.insert(222, &[32832]);     // 222 => [32832]
            map.insert(171, &[65600]);     // 171 => [65600]
            map.insert(188, &[384]);     // 188 => [384]
            map.insert(249, &[640]);     // 249 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(88, &[2176]);     // 88 => [2176]
            map.insert(13, &[4224]);     // 13 => [4224]
            map.insert(167, &[8320]);     // 167 => [8320]
            map.insert(207, &[16512]);     // 207 => [16512]
            map.insert(30, &[32896]);     // 30 => [32896]
            map.insert(107, &[65664]);     // 107 => [65664]
            map.insert(69, &[768]);     // 69 => [768]
            map.insert(206, &[1280]);     // 206 => [1280]
            map.insert(228, &[2304]);     // 228 => [2304]
            map.insert(177, &[4352]);     // 177 => [4352]
            map.insert(27, &[8448]);     // 27 => [8448]
            map.insert(115, &[16640]);     // 115 => [16640]
            map.insert(162, &[33024]);     // 162 => [33024]
            map.insert(215, &[65792]);     // 215 => [65792]
            map.insert(139, &[1536]);     // 139 => [1536]
            map.insert(161, &[2560]);     // 161 => [2560]
            map.insert(244, &[4608]);     // 244 => [4608]
            map.insert(94, &[8704]);     // 94 => [8704]
            map.insert(54, &[16896]);     // 54 => [16896]
            map.insert(231, &[33280]);     // 231 => [33280]
            map.insert(146, &[66048]);     // 146 => [66048]
            map.insert(42, &[3072]);     // 42 => [3072]
            map.insert(127, &[5120]);     // 127 => [5120]
            map.insert(213, &[9216]);     // 213 => [9216]
            map.insert(189, &[17408]);     // 189 => [17408]
            map.insert(108, &[33792]);     // 108 => [33792]
            map.insert(25, &[66560]);     // 25 => [66560]
            map.insert(85, &[6144]);     // 85 => [6144]
            map.insert(255, &[10240]);     // 255 => [10240]
            map.insert(151, &[18432]);     // 151 => [18432]
            map.insert(70, &[34816]);     // 70 => [34816]
            map.insert(51, &[67584]);     // 51 => [67584]
            map.insert(170, &[12288]);     // 170 => [12288]
            map.insert(194, &[20480]);     // 194 => [20480]
            map.insert(19, &[36864]);     // 19 => [36864]
            map.insert(102, &[69632]);     // 102 => [69632]
            map.insert(104, &[24576]);     // 104 => [24576]
            map.insert(185, &[40960]);     // 185 => [40960]
            map.insert(204, &[73728]);     // 204 => [73728]
            map.insert(209, &[49152]);     // 209 => [49152]
            map.insert(164, &[81920]);     // 164 => [81920]
            map.insert(117, &[98304]);     // 117 => [98304]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(63, &[259]);     // 63 => [259]
            map.insert(122, &[515]);     // 122 => [515]
            map.insert(241, &[1027]);     // 241 => [1027]
            map.insert(219, &[2051]);     // 219 => [2051]
            map.insert(76, &[16387]);     // 76 => [16387]
            map.insert(232, &[65539]);     // 232 => [65539]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(247, &[1029]);     // 247 => [1029]
            map.insert(221, &[2053]);     // 221 => [2053]
            map.insert(74, &[16389]);     // 74 => [16389]
            map.insert(155, &[32773]);     // 155 => [32773]
            map.insert(238, &[65541]);     // 238 => [65541]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(53, &[265]);     // 53 => [265]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(46, &[8201]);     // 46 => [8201]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(45, &[273]);     // 45 => [273]
            map.insert(201, &[2065]);     // 201 => [2065]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(29, &[289]);     // 29 => [289]
            map.insert(211, &[1057]);     // 211 => [1057]
            map.insert(172, &[4129]);     // 172 => [4129]
            map.insert(110, &[16417]);     // 110 => [16417]
            map.insert(191, &[32801]);     // 191 => [32801]
            map.insert(202, &[65569]);     // 202 => [65569]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(179, &[1089]);     // 179 => [1089]
            map.insert(153, &[2113]);     // 153 => [2113]
            map.insert(14, &[16449]);     // 14 => [16449]
            map.insert(223, &[32833]);     // 223 => [32833]
            map.insert(166, &[8321]);     // 166 => [8321]
            map.insert(31, &[32897]);     // 31 => [32897]
            map.insert(106, &[65665]);     // 106 => [65665]
            map.insert(229, &[2305]);     // 229 => [2305]
            map.insert(176, &[4353]);     // 176 => [4353]
            map.insert(26, &[8449]);     // 26 => [8449]
            map.insert(163, &[33025]);     // 163 => [33025]
            map.insert(214, &[65793]);     // 214 => [65793]
            map.insert(138, &[1537]);     // 138 => [1537]
            map.insert(245, &[4609]);     // 245 => [4609]
            map.insert(230, &[33281]);     // 230 => [33281]
            map.insert(147, &[66049]);     // 147 => [66049]
            map.insert(43, &[3073]);     // 43 => [3073]
            map.insert(126, &[5121]);     // 126 => [5121]
            map.insert(212, &[9217]);     // 212 => [9217]
            map.insert(109, &[33793]);     // 109 => [33793]
            map.insert(84, &[6145]);     // 84 => [6145]
            map.insert(254, &[10241]);     // 254 => [10241]
            map.insert(50, &[67585]);     // 50 => [67585]
            map.insert(195, &[20481]);     // 195 => [20481]
            map.insert(184, &[40961]);     // 184 => [40961]
            map.insert(165, &[81921]);     // 165 => [81921]
            map.insert(116, &[98305]);     // 116 => [98305]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(58, &[262]);     // 58 => [262]
            map.insert(237, &[65542]);     // 237 => [65542]
            map.insert(135, &[4106]);     // 135 => [4106]
            map.insert(148, &[32778]);     // 148 => [32778]
            map.insert(225, &[65546]);     // 225 => [65546]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(224, &[1042]);     // 224 => [1042]
            map.insert(93, &[16402]);     // 93 => [16402]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(91, &[546]);     // 91 => [546]
            map.insert(175, &[4130]);     // 175 => [4130]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(101, &[8258]);     // 101 => [8258]
            map.insert(169, &[65602]);     // 169 => [65602]
            map.insert(90, &[2178]);     // 90 => [2178]
            map.insert(92, &[8706]);     // 92 => [8706]
            map.insert(87, &[6146]);     // 87 => [6146]
            map.insert(253, &[10242]);     // 253 => [10242]
            map.insert(149, &[18434]);     // 149 => [18434]
            map.insert(168, &[12290]);     // 168 => [12290]
            map.insert(100, &[69634]);     // 100 => [69634]
            map.insert(187, &[40962]);     // 187 => [40962]
            map.insert(119, &[98306]);     // 119 => [98306]
            map.insert(252, &[2084]);     // 252 => [2084]
            map.insert(186, &[32804]);     // 186 => [32804]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(182, &[1092]);     // 182 => [1092]
            map.insert(99, &[8260]);     // 99 => [8260]
            map.insert(118, &[1156]);     // 118 => [1156]
            map.insert(181, &[4356]);     // 181 => [4356]
            map.insert(174, &[12292]);     // 174 => [12292]
            map.insert(198, &[20484]);     // 198 => [20484]
            map.insert(23, &[36868]);     // 23 => [36868]
            map.insert(197, &[4168]);     // 197 => [4168]
            map.insert(180, &[392]);     // 180 => [392]
            map.insert(199, &[16520]);     // 199 => [16520]
            map.insert(236, &[2312]);     // 236 => [2312]
            map.insert(86, &[8712]);     // 86 => [8712]
            map.insert(183, &[8336]);     // 183 => [8336]
            map.insert(83, &[16672]);     // 83 => [16672]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode17_9 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode17_9 {
    fn name(&self) -> String {
        "[17, 9] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        17
    }

    fn dimension(&self) -> usize {
        9
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
        let mut error = BinVector::with_capacity(17);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 17 / 64 + if 17 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(17) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(9);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[17 / 64] & !((1 << 17) - 1), 0, "this message has excess bits");

        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        let he = &BinMatrix::from_slices(&[&c[..]], self.length()) * self.parity_check_matrix_transposed();
        let error = map[unsafe { &he.get_word_unchecked(0, 0) }];
        c.iter_mut().zip(error.iter().copied()).for_each(|(sample, error)| *sample ^= error as u64);
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;
    use crate::oracle::Sample;

    #[test]
    fn size() {
        let code = GuavaCode17_9.generator_matrix();
        assert_eq!(code.ncols(), 17);
        assert_eq!(code.nrows(), 9);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode17_9;
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
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, false, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, false, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_9;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, true, true, true, false, true, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
