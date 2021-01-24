use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[14, 6]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode14_6;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 11457 ],
                &[ 14722 ],
                &[ 13060 ],
                &[ 5064 ],
                &[ 1936 ],
                &[ 15072 ],
                
            ], 14));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2817 ],
                &[ 7426 ],
                &[ 10244 ],
                &[ 15112 ],
                &[ 3856 ],
                &[ 7712 ],
                &[ 11840 ],
                &[ 15488 ],
                
            ], 14));
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
            map.insert(27, &[256]);     // 27 => [256]
            map.insert(121, &[512]);     // 121 => [512]
            map.insert(242, &[1024]);     // 242 => [1024]
            map.insert(255, &[2048]);     // 255 => [2048]
            map.insert(170, &[4096]);     // 170 => [4096]
            map.insert(204, &[8192]);     // 204 => [8192]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(26, &[257]);     // 26 => [257]
            map.insert(120, &[513]);     // 120 => [513]
            map.insert(243, &[1025]);     // 243 => [1025]
            map.insert(254, &[2049]);     // 254 => [2049]
            map.insert(171, &[4097]);     // 171 => [4097]
            map.insert(205, &[8193]);     // 205 => [8193]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(25, &[258]);     // 25 => [258]
            map.insert(123, &[514]);     // 123 => [514]
            map.insert(240, &[1026]);     // 240 => [1026]
            map.insert(253, &[2050]);     // 253 => [2050]
            map.insert(168, &[4098]);     // 168 => [4098]
            map.insert(206, &[8194]);     // 206 => [8194]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(31, &[260]);     // 31 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(246, &[1028]);     // 246 => [1028]
            map.insert(251, &[2052]);     // 251 => [2052]
            map.insert(174, &[4100]);     // 174 => [4100]
            map.insert(200, &[8196]);     // 200 => [8196]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(19, &[264]);     // 19 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(250, &[1032]);     // 250 => [1032]
            map.insert(247, &[2056]);     // 247 => [2056]
            map.insert(162, &[4104]);     // 162 => [4104]
            map.insert(196, &[8200]);     // 196 => [8200]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(11, &[272]);     // 11 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(226, &[1040]);     // 226 => [1040]
            map.insert(239, &[2064]);     // 239 => [2064]
            map.insert(186, &[4112]);     // 186 => [4112]
            map.insert(220, &[8208]);     // 220 => [8208]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(59, &[288]);     // 59 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(210, &[1056]);     // 210 => [1056]
            map.insert(223, &[2080]);     // 223 => [2080]
            map.insert(138, &[4128]);     // 138 => [4128]
            map.insert(236, &[8224]);     // 236 => [8224]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(91, &[320]);     // 91 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(178, &[1088]);     // 178 => [1088]
            map.insert(191, &[2112]);     // 191 => [2112]
            map.insert(234, &[4160]);     // 234 => [4160]
            map.insert(140, &[8256]);     // 140 => [8256]
            map.insert(155, &[384]);     // 155 => [384]
            map.insert(249, &[640]);     // 249 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(127, &[2176]);     // 127 => [2176]
            map.insert(42, &[4224]);     // 42 => [4224]
            map.insert(76, &[8320]);     // 76 => [8320]
            map.insert(98, &[768]);     // 98 => [768]
            map.insert(233, &[1280]);     // 233 => [1280]
            map.insert(228, &[2304]);     // 228 => [2304]
            map.insert(177, &[4352]);     // 177 => [4352]
            map.insert(215, &[8448]);     // 215 => [8448]
            map.insert(139, &[1536]);     // 139 => [1536]
            map.insert(134, &[2560]);     // 134 => [2560]
            map.insert(211, &[4608]);     // 211 => [4608]
            map.insert(181, &[8704]);     // 181 => [8704]
            map.insert(13, &[3072]);     // 13 => [3072]
            map.insert(88, &[5120]);     // 88 => [5120]
            map.insert(62, &[9216]);     // 62 => [9216]
            map.insert(85, &[6144]);     // 85 => [6144]
            map.insert(51, &[10240]);     // 51 => [10240]
            map.insert(102, &[12288]);     // 102 => [12288]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(122, &[515]);     // 122 => [515]
            map.insert(241, &[1027]);     // 241 => [1027]
            map.insert(252, &[2051]);     // 252 => [2051]
            map.insert(169, &[4099]);     // 169 => [4099]
            map.insert(207, &[8195]);     // 207 => [8195]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(30, &[261]);     // 30 => [261]
            map.insert(124, &[517]);     // 124 => [517]
            map.insert(175, &[4101]);     // 175 => [4101]
            map.insert(201, &[8197]);     // 201 => [8197]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(163, &[4105]);     // 163 => [4105]
            map.insert(197, &[8201]);     // 197 => [8201]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(104, &[529]);     // 104 => [529]
            map.insert(227, &[1041]);     // 227 => [1041]
            map.insert(238, &[2065]);     // 238 => [2065]
            map.insert(187, &[4113]);     // 187 => [4113]
            map.insert(221, &[8209]);     // 221 => [8209]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(58, &[289]);     // 58 => [289]
            map.insert(222, &[2081]);     // 222 => [2081]
            map.insert(237, &[8225]);     // 237 => [8225]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(90, &[321]);     // 90 => [321]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(179, &[1089]);     // 179 => [1089]
            map.insert(190, &[2113]);     // 190 => [2113]
            map.insert(235, &[4161]);     // 235 => [4161]
            map.insert(141, &[8257]);     // 141 => [8257]
            map.insert(154, &[385]);     // 154 => [385]
            map.insert(248, &[641]);     // 248 => [641]
            map.insert(115, &[1153]);     // 115 => [1153]
            map.insert(126, &[2177]);     // 126 => [2177]
            map.insert(43, &[4225]);     // 43 => [4225]
            map.insert(77, &[8321]);     // 77 => [8321]
            map.insert(99, &[769]);     // 99 => [769]
            map.insert(232, &[1281]);     // 232 => [1281]
            map.insert(229, &[2305]);     // 229 => [2305]
            map.insert(176, &[4353]);     // 176 => [4353]
            map.insert(214, &[8449]);     // 214 => [8449]
            map.insert(135, &[2561]);     // 135 => [2561]
            map.insert(180, &[8705]);     // 180 => [8705]
            map.insert(63, &[9217]);     // 63 => [9217]
            map.insert(84, &[6145]);     // 84 => [6145]
            map.insert(50, &[10241]);     // 50 => [10241]
            map.insert(103, &[12289]);     // 103 => [12289]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(29, &[262]);     // 29 => [262]
            map.insert(244, &[1030]);     // 244 => [1030]
            map.insert(172, &[4102]);     // 172 => [4102]
            map.insert(202, &[8198]);     // 202 => [8198]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(245, &[2058]);     // 245 => [2058]
            map.insert(198, &[8202]);     // 198 => [8202]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(146, &[146]);     // 146 => [146]
            map.insert(107, &[530]);     // 107 => [530]
            map.insert(224, &[1042]);     // 224 => [1042]
            map.insert(184, &[4114]);     // 184 => [4114]
            map.insert(208, &[1058]);     // 208 => [1058]
            map.insert(194, &[194]);     // 194 => [194]
            map.insert(189, &[2114]);     // 189 => [2114]
            map.insert(142, &[8258]);     // 142 => [8258]
            map.insert(153, &[386]);     // 153 => [386]
            map.insert(78, &[8322]);     // 78 => [8322]
            map.insert(230, &[2306]);     // 230 => [2306]
            map.insert(213, &[8450]);     // 213 => [8450]
            map.insert(209, &[4610]);     // 209 => [4610]
            map.insert(183, &[8706]);     // 183 => [8706]
            map.insert(15, &[3074]);     // 15 => [3074]
            map.insert(60, &[9218]);     // 60 => [9218]
            map.insert(87, &[6146]);     // 87 => [6146]
            map.insert(100, &[12290]);     // 100 => [12290]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(23, &[268]);     // 23 => [268]
            map.insert(117, &[524]);     // 117 => [524]
            map.insert(166, &[4108]);     // 166 => [4108]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(148, &[148]);     // 148 => [148]
            map.insert(109, &[532]);     // 109 => [532]
            map.insert(216, &[8212]);     // 216 => [8212]
            map.insert(164, &[164]);     // 164 => [164]
            map.insert(93, &[548]);     // 93 => [548]
            map.insert(219, &[2084]);     // 219 => [2084]
            map.insert(95, &[324]);     // 95 => [324]
            map.insert(61, &[580]);     // 61 => [580]
            map.insert(182, &[1092]);     // 182 => [1092]
            map.insert(159, &[388]);     // 159 => [388]
            map.insert(118, &[1156]);     // 118 => [1156]
            map.insert(46, &[4228]);     // 46 => [4228]
            map.insert(143, &[1540]);     // 143 => [1540]
            map.insert(92, &[5124]);     // 92 => [5124]
            map.insert(55, &[10244]);     // 55 => [10244]
            map.insert(152, &[152]);     // 152 => [152]
            map.insert(231, &[2072]);     // 231 => [2072]
            map.insert(212, &[8216]);     // 212 => [8216]
            map.insert(218, &[1064]);     // 218 => [1064]
            map.insert(83, &[328]);     // 83 => [328]
            map.insert(147, &[392]);     // 147 => [392]
            map.insert(119, &[2184]);     // 119 => [2184]
            map.insert(106, &[776]);     // 106 => [776]
            map.insert(225, &[1288]);     // 225 => [1288]
            map.insert(185, &[4360]);     // 185 => [4360]
            map.insert(54, &[9224]);     // 54 => [9224]
            map.insert(110, &[12296]);     // 110 => [12296]
            map.insert(75, &[336]);     // 75 => [336]
            map.insert(156, &[8272]);     // 156 => [8272]
            map.insert(111, &[2192]);     // 111 => [2192]
            map.insert(199, &[8464]);     // 199 => [8464]
            map.insert(150, &[2576]);     // 150 => [2576]
            map.insert(195, &[4624]);     // 195 => [4624]
            map.insert(165, &[8720]);     // 165 => [8720]
            map.insert(217, &[672]);     // 217 => [672]
            map.insert(108, &[8352]);     // 108 => [8352]
            map.insert(149, &[8736]);     // 149 => [8736]
            map.insert(45, &[3104]);     // 45 => [3104]
            map.insert(151, &[8512]);     // 151 => [8512]
            map.insert(203, &[1600]);     // 203 => [1600]
            map.insert(53, &[8832]);     // 53 => [8832]
            map.insert(157, &[2816]);     // 157 => [2816]
            map.insert(116, &[3584]);     // 116 => [3584]
            map.insert(71, &[9728]);     // 71 => [9728]
            map.insert(167, &[7168]);     // 167 => [7168]
            map.insert(39, &[39]);     // 39 => [39]
            map.insert(173, &[4103]);     // 173 => [4103]
            map.insert(188, &[2115]);     // 188 => [2115]
            map.insert(79, &[8323]);     // 79 => [8323]
            map.insert(86, &[6147]);     // 86 => [6147]
            map.insert(101, &[12291]);     // 101 => [12291]
            map.insert(94, &[325]);     // 94 => [325]
            map.insert(158, &[389]);     // 158 => [389]
            map.insert(47, &[4229]);     // 47 => [4229]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode14_6 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode14_6 {
    fn name(&self) -> String {
        "[14, 6] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        14
    }

    fn dimension(&self) -> usize {
        6
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
        let mut error = BinVector::with_capacity(14);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 14 / 64 + if 14 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(14) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(6);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[14 / 64] & !((1 << 14) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode14_6.generator_matrix();
        assert_eq!(code.ncols(), 14);
        assert_eq!(code.nrows(), 6);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode14_6;
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
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_6;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, true, false, false, true, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
