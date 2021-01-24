use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[12, 4]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode12_4;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1489 ],
                &[ 1714 ],
                &[ 3956 ],
                &[ 3976 ],
                
            ], 12));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1537 ],
                &[ 2562 ],
                &[ 1156 ],
                &[ 3208 ],
                &[ 2192 ],
                &[ 3744 ],
                &[ 704 ],
                &[ 3840 ],
                
            ], 12));
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
            map.insert(124, &[128]);     // 124 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(227, &[512]);     // 227 => [512]
            map.insert(173, &[1024]);     // 173 => [1024]
            map.insert(186, &[2048]);     // 186 => [2048]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(125, &[129]);     // 125 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(226, &[513]);     // 226 => [513]
            map.insert(172, &[1025]);     // 172 => [1025]
            map.insert(187, &[2049]);     // 187 => [2049]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(126, &[130]);     // 126 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(225, &[514]);     // 225 => [514]
            map.insert(175, &[1026]);     // 175 => [1026]
            map.insert(184, &[2050]);     // 184 => [2050]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(120, &[132]);     // 120 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(231, &[516]);     // 231 => [516]
            map.insert(169, &[1028]);     // 169 => [1028]
            map.insert(190, &[2052]);     // 190 => [2052]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(116, &[136]);     // 116 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(235, &[520]);     // 235 => [520]
            map.insert(165, &[1032]);     // 165 => [1032]
            map.insert(178, &[2056]);     // 178 => [2056]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(108, &[144]);     // 108 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(243, &[528]);     // 243 => [528]
            map.insert(189, &[1040]);     // 189 => [1040]
            map.insert(170, &[2064]);     // 170 => [2064]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(92, &[160]);     // 92 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(195, &[544]);     // 195 => [544]
            map.insert(141, &[1056]);     // 141 => [1056]
            map.insert(154, &[2080]);     // 154 => [2080]
            map.insert(60, &[192]);     // 60 => [192]
            map.insert(192, &[320]);     // 192 => [320]
            map.insert(163, &[576]);     // 163 => [576]
            map.insert(237, &[1088]);     // 237 => [1088]
            map.insert(250, &[2112]);     // 250 => [2112]
            map.insert(252, &[384]);     // 252 => [384]
            map.insert(159, &[640]);     // 159 => [640]
            map.insert(209, &[1152]);     // 209 => [1152]
            map.insert(198, &[2176]);     // 198 => [2176]
            map.insert(99, &[768]);     // 99 => [768]
            map.insert(45, &[1280]);     // 45 => [1280]
            map.insert(58, &[2304]);     // 58 => [2304]
            map.insert(78, &[1536]);     // 78 => [1536]
            map.insert(89, &[2560]);     // 89 => [2560]
            map.insert(23, &[3072]);     // 23 => [3072]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(127, &[131]);     // 127 => [131]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(224, &[515]);     // 224 => [515]
            map.insert(174, &[1027]);     // 174 => [1027]
            map.insert(185, &[2051]);     // 185 => [2051]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(121, &[133]);     // 121 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(230, &[517]);     // 230 => [517]
            map.insert(168, &[1029]);     // 168 => [1029]
            map.insert(191, &[2053]);     // 191 => [2053]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(117, &[137]);     // 117 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(234, &[521]);     // 234 => [521]
            map.insert(164, &[1033]);     // 164 => [1033]
            map.insert(179, &[2057]);     // 179 => [2057]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(109, &[145]);     // 109 => [145]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(242, &[529]);     // 242 => [529]
            map.insert(188, &[1041]);     // 188 => [1041]
            map.insert(171, &[2065]);     // 171 => [2065]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(93, &[161]);     // 93 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(194, &[545]);     // 194 => [545]
            map.insert(140, &[1057]);     // 140 => [1057]
            map.insert(155, &[2081]);     // 155 => [2081]
            map.insert(61, &[193]);     // 61 => [193]
            map.insert(193, &[321]);     // 193 => [321]
            map.insert(162, &[577]);     // 162 => [577]
            map.insert(236, &[1089]);     // 236 => [1089]
            map.insert(251, &[2113]);     // 251 => [2113]
            map.insert(253, &[385]);     // 253 => [385]
            map.insert(158, &[641]);     // 158 => [641]
            map.insert(208, &[1153]);     // 208 => [1153]
            map.insert(199, &[2177]);     // 199 => [2177]
            map.insert(98, &[769]);     // 98 => [769]
            map.insert(44, &[1281]);     // 44 => [1281]
            map.insert(59, &[2305]);     // 59 => [2305]
            map.insert(79, &[1537]);     // 79 => [1537]
            map.insert(88, &[2561]);     // 88 => [2561]
            map.insert(22, &[3073]);     // 22 => [3073]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(122, &[134]);     // 122 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(229, &[518]);     // 229 => [518]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(118, &[138]);     // 118 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(233, &[522]);     // 233 => [522]
            map.insert(167, &[1034]);     // 167 => [1034]
            map.insert(176, &[2058]);     // 176 => [2058]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(110, &[146]);     // 110 => [146]
            map.insert(146, &[274]);     // 146 => [274]
            map.insert(241, &[530]);     // 241 => [530]
            map.insert(94, &[162]);     // 94 => [162]
            map.insert(143, &[1058]);     // 143 => [1058]
            map.insert(152, &[2082]);     // 152 => [2082]
            map.insert(62, &[194]);     // 62 => [194]
            map.insert(239, &[1090]);     // 239 => [1090]
            map.insert(248, &[2114]);     // 248 => [2114]
            map.insert(254, &[386]);     // 254 => [386]
            map.insert(157, &[642]);     // 157 => [642]
            map.insert(211, &[1154]);     // 211 => [1154]
            map.insert(196, &[2178]);     // 196 => [2178]
            map.insert(47, &[1282]);     // 47 => [1282]
            map.insert(56, &[2306]);     // 56 => [2306]
            map.insert(76, &[1538]);     // 76 => [1538]
            map.insert(91, &[2562]);     // 91 => [2562]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(112, &[140]);     // 112 => [140]
            map.insert(182, &[2060]);     // 182 => [2060]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(84, &[84]);     // 84 => [84]
            map.insert(104, &[148]);     // 104 => [148]
            map.insert(148, &[276]);     // 148 => [276]
            map.insert(247, &[532]);     // 247 => [532]
            map.insert(100, &[100]);     // 100 => [100]
            map.insert(213, &[1156]);     // 213 => [1156]
            map.insert(103, &[772]);     // 103 => [772]
            map.insert(181, &[1048]);     // 181 => [1048]
            map.insert(203, &[552]);     // 203 => [552]
            map.insert(200, &[328]);     // 200 => [328]
            map.insert(244, &[392]);     // 244 => [392]
            map.insert(151, &[648]);     // 151 => [648]
            map.insert(217, &[1160]);     // 217 => [1160]
            map.insert(206, &[2184]);     // 206 => [2184]
            map.insert(107, &[776]);     // 107 => [776]
            map.insert(31, &[3080]);     // 31 => [3080]
            map.insert(214, &[2192]);     // 214 => [2192]
            map.insert(115, &[784]);     // 115 => [784]
            map.insert(205, &[1120]);     // 205 => [1120]
            map.insert(218, &[2144]);     // 218 => [2144]
            map.insert(220, &[416]);     // 220 => [416]
            map.insert(55, &[3104]);     // 55 => [3104]
            map.insert(223, &[704]);     // 223 => [704]
            map.insert(87, &[3136]);     // 87 => [3136]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[39]);     // 39 => [39]
            map.insert(71, &[71]);     // 71 => [71]
            map.insert(123, &[135]);     // 123 => [135]
            map.insert(135, &[263]);     // 135 => [263]
            map.insert(228, &[519]);     // 228 => [519]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(75, &[75]);     // 75 => [75]
            map.insert(119, &[139]);     // 119 => [139]
            map.insert(139, &[267]);     // 139 => [267]
            map.insert(232, &[523]);     // 232 => [523]
            map.insert(166, &[1035]);     // 166 => [1035]
            map.insert(177, &[2059]);     // 177 => [2059]
            map.insert(51, &[51]);     // 51 => [51]
            map.insert(83, &[83]);     // 83 => [83]
            map.insert(111, &[147]);     // 111 => [147]
            map.insert(147, &[275]);     // 147 => [275]
            map.insert(240, &[531]);     // 240 => [531]
            map.insert(95, &[163]);     // 95 => [163]
            map.insert(142, &[1059]);     // 142 => [1059]
            map.insert(153, &[2083]);     // 153 => [2083]
            map.insert(63, &[195]);     // 63 => [195]
            map.insert(238, &[1091]);     // 238 => [1091]
            map.insert(249, &[2115]);     // 249 => [2115]
            map.insert(255, &[387]);     // 255 => [387]
            map.insert(156, &[643]);     // 156 => [643]
            map.insert(210, &[1155]);     // 210 => [1155]
            map.insert(197, &[2179]);     // 197 => [2179]
            map.insert(46, &[1283]);     // 46 => [1283]
            map.insert(57, &[2307]);     // 57 => [2307]
            map.insert(77, &[1539]);     // 77 => [1539]
            map.insert(90, &[2563]);     // 90 => [2563]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(113, &[141]);     // 113 => [141]
            map.insert(183, &[2061]);     // 183 => [2061]
            map.insert(53, &[53]);     // 53 => [53]
            map.insert(85, &[85]);     // 85 => [85]
            map.insert(105, &[149]);     // 105 => [149]
            map.insert(149, &[277]);     // 149 => [277]
            map.insert(246, &[533]);     // 246 => [533]
            map.insert(101, &[101]);     // 101 => [101]
            map.insert(212, &[1157]);     // 212 => [1157]
            map.insert(102, &[773]);     // 102 => [773]
            map.insert(180, &[1049]);     // 180 => [1049]
            map.insert(202, &[553]);     // 202 => [553]
            map.insert(201, &[329]);     // 201 => [329]
            map.insert(245, &[393]);     // 245 => [393]
            map.insert(150, &[649]);     // 150 => [649]
            map.insert(216, &[1161]);     // 216 => [1161]
            map.insert(207, &[2185]);     // 207 => [2185]
            map.insert(106, &[777]);     // 106 => [777]
            map.insert(30, &[3081]);     // 30 => [3081]
            map.insert(215, &[2193]);     // 215 => [2193]
            map.insert(114, &[785]);     // 114 => [785]
            map.insert(204, &[1121]);     // 204 => [1121]
            map.insert(219, &[2145]);     // 219 => [2145]
            map.insert(221, &[417]);     // 221 => [417]
            map.insert(54, &[3105]);     // 54 => [3105]
            map.insert(222, &[705]);     // 222 => [705]
            map.insert(86, &[3137]);     // 86 => [3137]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode12_4 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode12_4 {
    fn name(&self) -> String {
        "[12, 4] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        12
    }

    fn dimension(&self) -> usize {
        4
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
        let mut error = BinVector::with_capacity(12);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 12 / 64 + if 12 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(12) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(4);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[12 / 64] & !((1 << 12) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode12_4.generator_matrix();
        assert_eq!(code.ncols(), 12);
        assert_eq!(code.nrows(), 4);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode12_4;
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
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, true, false, true, true, true, false, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
