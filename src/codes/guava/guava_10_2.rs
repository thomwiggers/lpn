use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[10, 2]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode10_2;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 909 ],
                &[ 1010 ],
                
            ], 10));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 577 ],
                &[ 66 ],
                &[ 580 ],
                &[ 584 ],
                &[ 80 ],
                &[ 96 ],
                &[ 640 ],
                &[ 768 ],
                
            ], 10));
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
            map.insert(63, &[64]);     // 63 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(205, &[512]);     // 205 => [512]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(62, &[65]);     // 62 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(204, &[513]);     // 204 => [513]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(61, &[66]);     // 61 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(207, &[514]);     // 207 => [514]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(59, &[68]);     // 59 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(201, &[516]);     // 201 => [516]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(55, &[72]);     // 55 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(197, &[520]);     // 197 => [520]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(47, &[80]);     // 47 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(221, &[528]);     // 221 => [528]
            map.insert(31, &[96]);     // 31 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(237, &[544]);     // 237 => [544]
            map.insert(127, &[192]);     // 127 => [192]
            map.insert(191, &[320]);     // 191 => [320]
            map.insert(242, &[576]);     // 242 => [576]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(141, &[640]);     // 141 => [640]
            map.insert(77, &[768]);     // 77 => [768]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(60, &[67]);     // 60 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(206, &[515]);     // 206 => [515]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(58, &[69]);     // 58 => [69]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(200, &[517]);     // 200 => [517]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(54, &[73]);     // 54 => [73]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(196, &[521]);     // 196 => [521]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(46, &[81]);     // 46 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(220, &[529]);     // 220 => [529]
            map.insert(30, &[97]);     // 30 => [97]
            map.insert(97, &[161]);     // 97 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(236, &[545]);     // 236 => [545]
            map.insert(126, &[193]);     // 126 => [193]
            map.insert(190, &[321]);     // 190 => [321]
            map.insert(243, &[577]);     // 243 => [577]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(140, &[641]);     // 140 => [641]
            map.insert(76, &[769]);     // 76 => [769]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(57, &[70]);     // 57 => [70]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(203, &[518]);     // 203 => [518]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(53, &[74]);     // 53 => [74]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(199, &[522]);     // 199 => [522]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(45, &[82]);     // 45 => [82]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(146, &[274]);     // 146 => [274]
            map.insert(223, &[530]);     // 223 => [530]
            map.insert(29, &[98]);     // 29 => [98]
            map.insert(98, &[162]);     // 98 => [162]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(239, &[546]);     // 239 => [546]
            map.insert(125, &[194]);     // 125 => [194]
            map.insert(189, &[322]);     // 189 => [322]
            map.insert(240, &[578]);     // 240 => [578]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(143, &[642]);     // 143 => [642]
            map.insert(79, &[770]);     // 79 => [770]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(51, &[76]);     // 51 => [76]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(43, &[84]);     // 43 => [84]
            map.insert(84, &[148]);     // 84 => [148]
            map.insert(148, &[276]);     // 148 => [276]
            map.insert(217, &[532]);     // 217 => [532]
            map.insert(27, &[100]);     // 27 => [100]
            map.insert(100, &[164]);     // 100 => [164]
            map.insert(164, &[292]);     // 164 => [292]
            map.insert(233, &[548]);     // 233 => [548]
            map.insert(123, &[196]);     // 123 => [196]
            map.insert(187, &[324]);     // 187 => [324]
            map.insert(246, &[580]);     // 246 => [580]
            map.insert(56, &[56]);     // 56 => [56]
            map.insert(39, &[88]);     // 39 => [88]
            map.insert(88, &[152]);     // 88 => [152]
            map.insert(152, &[280]);     // 152 => [280]
            map.insert(213, &[536]);     // 213 => [536]
            map.insert(23, &[104]);     // 23 => [104]
            map.insert(104, &[168]);     // 104 => [168]
            map.insert(168, &[296]);     // 168 => [296]
            map.insert(229, &[552]);     // 229 => [552]
            map.insert(119, &[200]);     // 119 => [200]
            map.insert(183, &[328]);     // 183 => [328]
            map.insert(250, &[584]);     // 250 => [584]
            map.insert(15, &[112]);     // 15 => [112]
            map.insert(112, &[176]);     // 112 => [176]
            map.insert(176, &[304]);     // 176 => [304]
            map.insert(253, &[560]);     // 253 => [560]
            map.insert(111, &[208]);     // 111 => [208]
            map.insert(175, &[336]);     // 175 => [336]
            map.insert(226, &[592]);     // 226 => [592]
            map.insert(208, &[400]);     // 208 => [400]
            map.insert(157, &[656]);     // 157 => [656]
            map.insert(93, &[784]);     // 93 => [784]
            map.insert(95, &[224]);     // 95 => [224]
            map.insert(159, &[352]);     // 159 => [352]
            map.insert(210, &[608]);     // 210 => [608]
            map.insert(224, &[416]);     // 224 => [416]
            map.insert(173, &[672]);     // 173 => [672]
            map.insert(109, &[800]);     // 109 => [800]
            map.insert(255, &[448]);     // 255 => [448]
            map.insert(178, &[704]);     // 178 => [704]
            map.insert(114, &[832]);     // 114 => [832]
            map.insert(71, &[135]);     // 71 => [135]
            map.insert(135, &[263]);     // 135 => [263]
            map.insert(202, &[519]);     // 202 => [519]
            map.insert(75, &[139]);     // 75 => [139]
            map.insert(139, &[267]);     // 139 => [267]
            map.insert(198, &[523]);     // 198 => [523]
            map.insert(83, &[147]);     // 83 => [147]
            map.insert(147, &[275]);     // 147 => [275]
            map.insert(222, &[531]);     // 222 => [531]
            map.insert(99, &[163]);     // 99 => [163]
            map.insert(163, &[291]);     // 163 => [291]
            map.insert(238, &[547]);     // 238 => [547]
            map.insert(124, &[195]);     // 124 => [195]
            map.insert(188, &[323]);     // 188 => [323]
            map.insert(241, &[579]);     // 241 => [579]
            map.insert(195, &[387]);     // 195 => [387]
            map.insert(142, &[643]);     // 142 => [643]
            map.insert(78, &[771]);     // 78 => [771]
            map.insert(85, &[149]);     // 85 => [149]
            map.insert(149, &[277]);     // 149 => [277]
            map.insert(216, &[533]);     // 216 => [533]
            map.insert(101, &[165]);     // 101 => [165]
            map.insert(165, &[293]);     // 165 => [293]
            map.insert(232, &[549]);     // 232 => [549]
            map.insert(122, &[197]);     // 122 => [197]
            map.insert(186, &[325]);     // 186 => [325]
            map.insert(247, &[581]);     // 247 => [581]
            map.insert(89, &[153]);     // 89 => [153]
            map.insert(153, &[281]);     // 153 => [281]
            map.insert(212, &[537]);     // 212 => [537]
            map.insert(105, &[169]);     // 105 => [169]
            map.insert(169, &[297]);     // 169 => [297]
            map.insert(228, &[553]);     // 228 => [553]
            map.insert(118, &[201]);     // 118 => [201]
            map.insert(182, &[329]);     // 182 => [329]
            map.insert(251, &[585]);     // 251 => [585]
            map.insert(113, &[177]);     // 113 => [177]
            map.insert(177, &[305]);     // 177 => [305]
            map.insert(252, &[561]);     // 252 => [561]
            map.insert(110, &[209]);     // 110 => [209]
            map.insert(174, &[337]);     // 174 => [337]
            map.insert(227, &[593]);     // 227 => [593]
            map.insert(209, &[401]);     // 209 => [401]
            map.insert(156, &[657]);     // 156 => [657]
            map.insert(92, &[785]);     // 92 => [785]
            map.insert(94, &[225]);     // 94 => [225]
            map.insert(158, &[353]);     // 158 => [353]
            map.insert(211, &[609]);     // 211 => [609]
            map.insert(225, &[417]);     // 225 => [417]
            map.insert(172, &[673]);     // 172 => [673]
            map.insert(108, &[801]);     // 108 => [801]
            map.insert(254, &[449]);     // 254 => [449]
            map.insert(179, &[705]);     // 179 => [705]
            map.insert(115, &[833]);     // 115 => [833]
            map.insert(86, &[150]);     // 86 => [150]
            map.insert(150, &[278]);     // 150 => [278]
            map.insert(219, &[534]);     // 219 => [534]
            map.insert(102, &[166]);     // 102 => [166]
            map.insert(166, &[294]);     // 166 => [294]
            map.insert(235, &[550]);     // 235 => [550]
            map.insert(121, &[198]);     // 121 => [198]
            map.insert(185, &[326]);     // 185 => [326]
            map.insert(244, &[582]);     // 244 => [582]
            map.insert(90, &[154]);     // 90 => [154]
            map.insert(154, &[282]);     // 154 => [282]
            map.insert(215, &[538]);     // 215 => [538]
            map.insert(106, &[170]);     // 106 => [170]
            map.insert(170, &[298]);     // 170 => [298]
            map.insert(231, &[554]);     // 231 => [554]
            map.insert(117, &[202]);     // 117 => [202]
            map.insert(181, &[330]);     // 181 => [330]
            map.insert(248, &[586]);     // 248 => [586]
            map.insert(116, &[180]);     // 116 => [180]
            map.insert(180, &[308]);     // 180 => [308]
            map.insert(249, &[564]);     // 249 => [564]
            map.insert(107, &[212]);     // 107 => [212]
            map.insert(171, &[340]);     // 171 => [340]
            map.insert(230, &[596]);     // 230 => [596]
            map.insert(91, &[228]);     // 91 => [228]
            map.insert(155, &[356]);     // 155 => [356]
            map.insert(214, &[612]);     // 214 => [612]
            map.insert(120, &[184]);     // 120 => [184]
            map.insert(184, &[312]);     // 184 => [312]
            map.insert(245, &[568]);     // 245 => [568]
            map.insert(103, &[216]);     // 103 => [216]
            map.insert(167, &[344]);     // 167 => [344]
            map.insert(234, &[600]);     // 234 => [600]
            map.insert(87, &[232]);     // 87 => [232]
            map.insert(151, &[360]);     // 151 => [360]
            map.insert(218, &[616]);     // 218 => [616]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode10_2 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode10_2 {
    fn name(&self) -> String {
        "[10, 2] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        10
    }

    fn dimension(&self) -> usize {
        2
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
        let mut error = BinVector::with_capacity(10);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 10 / 64 + if 10 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(10) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(2);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[10 / 64] & !((1 << 10) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode10_2.generator_matrix();
        assert_eq!(code.ncols(), 10);
        assert_eq!(code.nrows(), 2);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode10_2;
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
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_2;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, true, true, false, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
