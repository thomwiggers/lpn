use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[11, 3]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode11_3;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 857 ],
                &[ 1978 ],
                &[ 1988 ],
                
            ], 11));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1537 ],
                &[ 578 ],
                &[ 1604 ],
                &[ 1096 ],
                &[ 1104 ],
                &[ 608 ],
                &[ 1152 ],
                &[ 768 ],
                
            ], 11));
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
            map.insert(62, &[64]);     // 62 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(167, &[512]);     // 167 => [512]
            map.insert(93, &[1024]);     // 93 => [1024]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(63, &[65]);     // 63 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(166, &[513]);     // 166 => [513]
            map.insert(92, &[1025]);     // 92 => [1025]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(60, &[66]);     // 60 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(165, &[514]);     // 165 => [514]
            map.insert(95, &[1026]);     // 95 => [1026]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(58, &[68]);     // 58 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(163, &[516]);     // 163 => [516]
            map.insert(89, &[1028]);     // 89 => [1028]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(54, &[72]);     // 54 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(175, &[520]);     // 175 => [520]
            map.insert(85, &[1032]);     // 85 => [1032]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(46, &[80]);     // 46 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(183, &[528]);     // 183 => [528]
            map.insert(77, &[1040]);     // 77 => [1040]
            map.insert(30, &[96]);     // 30 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(135, &[544]);     // 135 => [544]
            map.insert(125, &[1056]);     // 125 => [1056]
            map.insert(126, &[192]);     // 126 => [192]
            map.insert(190, &[320]);     // 190 => [320]
            map.insert(153, &[576]);     // 153 => [576]
            map.insert(99, &[1088]);     // 99 => [1088]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(231, &[640]);     // 231 => [640]
            map.insert(29, &[1152]);     // 29 => [1152]
            map.insert(39, &[768]);     // 39 => [768]
            map.insert(221, &[1280]);     // 221 => [1280]
            map.insert(250, &[1536]);     // 250 => [1536]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(61, &[67]);     // 61 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(164, &[515]);     // 164 => [515]
            map.insert(94, &[1027]);     // 94 => [1027]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(59, &[69]);     // 59 => [69]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(162, &[517]);     // 162 => [517]
            map.insert(88, &[1029]);     // 88 => [1029]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(55, &[73]);     // 55 => [73]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(174, &[521]);     // 174 => [521]
            map.insert(84, &[1033]);     // 84 => [1033]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(47, &[81]);     // 47 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(182, &[529]);     // 182 => [529]
            map.insert(76, &[1041]);     // 76 => [1041]
            map.insert(31, &[97]);     // 31 => [97]
            map.insert(97, &[161]);     // 97 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(134, &[545]);     // 134 => [545]
            map.insert(124, &[1057]);     // 124 => [1057]
            map.insert(127, &[193]);     // 127 => [193]
            map.insert(191, &[321]);     // 191 => [321]
            map.insert(152, &[577]);     // 152 => [577]
            map.insert(98, &[1089]);     // 98 => [1089]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(230, &[641]);     // 230 => [641]
            map.insert(28, &[1153]);     // 28 => [1153]
            map.insert(38, &[769]);     // 38 => [769]
            map.insert(220, &[1281]);     // 220 => [1281]
            map.insert(251, &[1537]);     // 251 => [1537]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(56, &[70]);     // 56 => [70]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(91, &[1030]);     // 91 => [1030]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(52, &[74]);     // 52 => [74]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(173, &[522]);     // 173 => [522]
            map.insert(87, &[1034]);     // 87 => [1034]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(44, &[82]);     // 44 => [82]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(146, &[274]);     // 146 => [274]
            map.insert(181, &[530]);     // 181 => [530]
            map.insert(79, &[1042]);     // 79 => [1042]
            map.insert(188, &[322]);     // 188 => [322]
            map.insert(155, &[578]);     // 155 => [578]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(229, &[642]);     // 229 => [642]
            map.insert(223, &[1282]);     // 223 => [1282]
            map.insert(248, &[1538]);     // 248 => [1538]
            map.insert(140, &[268]);     // 140 => [268]
            map.insert(171, &[524]);     // 171 => [524]
            map.insert(148, &[276]);     // 148 => [276]
            map.insert(179, &[532]);     // 179 => [532]
            map.insert(100, &[164]);     // 100 => [164]
            map.insert(121, &[1060]);     // 121 => [1060]
            map.insert(122, &[196]);     // 122 => [196]
            map.insert(186, &[324]);     // 186 => [324]
            map.insert(157, &[580]);     // 157 => [580]
            map.insert(103, &[1092]);     // 103 => [1092]
            map.insert(196, &[388]);     // 196 => [388]
            map.insert(227, &[644]);     // 227 => [644]
            map.insert(217, &[1284]);     // 217 => [1284]
            map.insert(254, &[1540]);     // 254 => [1540]
            map.insert(104, &[168]);     // 104 => [168]
            map.insert(168, &[296]);     // 168 => [296]
            map.insert(143, &[552]);     // 143 => [552]
            map.insert(117, &[1064]);     // 117 => [1064]
            map.insert(118, &[200]);     // 118 => [200]
            map.insert(107, &[1096]);     // 107 => [1096]
            map.insert(200, &[392]);     // 200 => [392]
            map.insert(239, &[648]);     // 239 => [648]
            map.insert(213, &[1288]);     // 213 => [1288]
            map.insert(242, &[1544]);     // 242 => [1544]
            map.insert(112, &[176]);     // 112 => [176]
            map.insert(176, &[304]);     // 176 => [304]
            map.insert(151, &[560]);     // 151 => [560]
            map.insert(109, &[1072]);     // 109 => [1072]
            map.insert(110, &[208]);     // 110 => [208]
            map.insert(115, &[1104]);     // 115 => [1104]
            map.insert(208, &[400]);     // 208 => [400]
            map.insert(247, &[656]);     // 247 => [656]
            map.insert(205, &[1296]);     // 205 => [1296]
            map.insert(234, &[1552]);     // 234 => [1552]
            map.insert(158, &[352]);     // 158 => [352]
            map.insert(185, &[608]);     // 185 => [608]
            map.insert(224, &[416]);     // 224 => [416]
            map.insert(199, &[672]);     // 199 => [672]
            map.insert(253, &[1312]);     // 253 => [1312]
            map.insert(218, &[1568]);     // 218 => [1568]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(57, &[71]);     // 57 => [71]
            map.insert(71, &[135]);     // 71 => [135]
            map.insert(90, &[1031]);     // 90 => [1031]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(53, &[75]);     // 53 => [75]
            map.insert(75, &[139]);     // 75 => [139]
            map.insert(139, &[267]);     // 139 => [267]
            map.insert(172, &[523]);     // 172 => [523]
            map.insert(86, &[1035]);     // 86 => [1035]
            map.insert(51, &[51]);     // 51 => [51]
            map.insert(45, &[83]);     // 45 => [83]
            map.insert(83, &[147]);     // 83 => [147]
            map.insert(147, &[275]);     // 147 => [275]
            map.insert(180, &[531]);     // 180 => [531]
            map.insert(78, &[1043]);     // 78 => [1043]
            map.insert(189, &[323]);     // 189 => [323]
            map.insert(154, &[579]);     // 154 => [579]
            map.insert(195, &[387]);     // 195 => [387]
            map.insert(228, &[643]);     // 228 => [643]
            map.insert(222, &[1283]);     // 222 => [1283]
            map.insert(249, &[1539]);     // 249 => [1539]
            map.insert(141, &[269]);     // 141 => [269]
            map.insert(170, &[525]);     // 170 => [525]
            map.insert(149, &[277]);     // 149 => [277]
            map.insert(178, &[533]);     // 178 => [533]
            map.insert(101, &[165]);     // 101 => [165]
            map.insert(120, &[1061]);     // 120 => [1061]
            map.insert(123, &[197]);     // 123 => [197]
            map.insert(187, &[325]);     // 187 => [325]
            map.insert(156, &[581]);     // 156 => [581]
            map.insert(102, &[1093]);     // 102 => [1093]
            map.insert(197, &[389]);     // 197 => [389]
            map.insert(226, &[645]);     // 226 => [645]
            map.insert(216, &[1285]);     // 216 => [1285]
            map.insert(255, &[1541]);     // 255 => [1541]
            map.insert(105, &[169]);     // 105 => [169]
            map.insert(169, &[297]);     // 169 => [297]
            map.insert(142, &[553]);     // 142 => [553]
            map.insert(116, &[1065]);     // 116 => [1065]
            map.insert(119, &[201]);     // 119 => [201]
            map.insert(106, &[1097]);     // 106 => [1097]
            map.insert(201, &[393]);     // 201 => [393]
            map.insert(238, &[649]);     // 238 => [649]
            map.insert(212, &[1289]);     // 212 => [1289]
            map.insert(243, &[1545]);     // 243 => [1545]
            map.insert(113, &[177]);     // 113 => [177]
            map.insert(177, &[305]);     // 177 => [305]
            map.insert(150, &[561]);     // 150 => [561]
            map.insert(108, &[1073]);     // 108 => [1073]
            map.insert(111, &[209]);     // 111 => [209]
            map.insert(114, &[1105]);     // 114 => [1105]
            map.insert(209, &[401]);     // 209 => [401]
            map.insert(246, &[657]);     // 246 => [657]
            map.insert(204, &[1297]);     // 204 => [1297]
            map.insert(235, &[1553]);     // 235 => [1553]
            map.insert(159, &[353]);     // 159 => [353]
            map.insert(184, &[609]);     // 184 => [609]
            map.insert(225, &[417]);     // 225 => [417]
            map.insert(198, &[673]);     // 198 => [673]
            map.insert(252, &[1313]);     // 252 => [1313]
            map.insert(219, &[1569]);     // 219 => [1569]
            map.insert(202, &[394]);     // 202 => [394]
            map.insert(237, &[650]);     // 237 => [650]
            map.insert(215, &[1290]);     // 215 => [1290]
            map.insert(240, &[1546]);     // 240 => [1546]
            map.insert(210, &[402]);     // 210 => [402]
            map.insert(245, &[658]);     // 245 => [658]
            map.insert(207, &[1298]);     // 207 => [1298]
            map.insert(232, &[1554]);     // 232 => [1554]
            map.insert(203, &[395]);     // 203 => [395]
            map.insert(236, &[651]);     // 236 => [651]
            map.insert(214, &[1291]);     // 214 => [1291]
            map.insert(241, &[1547]);     // 241 => [1547]
            map.insert(211, &[403]);     // 211 => [403]
            map.insert(244, &[659]);     // 244 => [659]
            map.insert(206, &[1299]);     // 206 => [1299]
            map.insert(233, &[1555]);     // 233 => [1555]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode11_3 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode11_3 {
    fn name(&self) -> String {
        "[11, 3] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        11
    }

    fn dimension(&self) -> usize {
        3
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
        let mut error = BinVector::with_capacity(11);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 11 / 64 + if 11 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(11) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(3);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[11 / 64] & !((1 << 11) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode11_3.generator_matrix();
        assert_eq!(code.ncols(), 11);
        assert_eq!(code.nrows(), 3);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode11_3;
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
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, true, true, false, true, false, true, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
