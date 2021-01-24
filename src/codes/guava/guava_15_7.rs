use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[15, 7]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode15_7;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 14593 ],
                &[ 22914 ],
                &[ 29444 ],
                &[ 26120 ],
                &[ 10128 ],
                &[ 3872 ],
                &[ 30144 ],
                
            ], 15));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 15425 ],
                &[ 10818 ],
                &[ 1604 ],
                &[ 27720 ],
                &[ 30224 ],
                &[ 7712 ],
                &[ 23680 ],
                &[ 30976 ],
                
            ], 15));
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
            map.insert(15, &[64]);     // 15 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(54, &[512]);     // 54 => [512]
            map.insert(125, &[1024]);     // 125 => [1024]
            map.insert(235, &[2048]);     // 235 => [2048]
            map.insert(241, &[4096]);     // 241 => [4096]
            map.insert(155, &[8192]);     // 155 => [8192]
            map.insert(216, &[16384]);     // 216 => [16384]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(14, &[65]);     // 14 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(55, &[513]);     // 55 => [513]
            map.insert(124, &[1025]);     // 124 => [1025]
            map.insert(234, &[2049]);     // 234 => [2049]
            map.insert(240, &[4097]);     // 240 => [4097]
            map.insert(154, &[8193]);     // 154 => [8193]
            map.insert(217, &[16385]);     // 217 => [16385]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(13, &[66]);     // 13 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(52, &[514]);     // 52 => [514]
            map.insert(127, &[1026]);     // 127 => [1026]
            map.insert(233, &[2050]);     // 233 => [2050]
            map.insert(243, &[4098]);     // 243 => [4098]
            map.insert(153, &[8194]);     // 153 => [8194]
            map.insert(218, &[16386]);     // 218 => [16386]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(11, &[68]);     // 11 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(50, &[516]);     // 50 => [516]
            map.insert(121, &[1028]);     // 121 => [1028]
            map.insert(239, &[2052]);     // 239 => [2052]
            map.insert(245, &[4100]);     // 245 => [4100]
            map.insert(159, &[8196]);     // 159 => [8196]
            map.insert(220, &[16388]);     // 220 => [16388]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(7, &[72]);     // 7 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(62, &[520]);     // 62 => [520]
            map.insert(117, &[1032]);     // 117 => [1032]
            map.insert(227, &[2056]);     // 227 => [2056]
            map.insert(249, &[4104]);     // 249 => [4104]
            map.insert(147, &[8200]);     // 147 => [8200]
            map.insert(208, &[16392]);     // 208 => [16392]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(31, &[80]);     // 31 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(38, &[528]);     // 38 => [528]
            map.insert(109, &[1040]);     // 109 => [1040]
            map.insert(251, &[2064]);     // 251 => [2064]
            map.insert(225, &[4112]);     // 225 => [4112]
            map.insert(139, &[8208]);     // 139 => [8208]
            map.insert(200, &[16400]);     // 200 => [16400]
            map.insert(47, &[96]);     // 47 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(22, &[544]);     // 22 => [544]
            map.insert(93, &[1056]);     // 93 => [1056]
            map.insert(203, &[2080]);     // 203 => [2080]
            map.insert(209, &[4128]);     // 209 => [4128]
            map.insert(187, &[8224]);     // 187 => [8224]
            map.insert(248, &[16416]);     // 248 => [16416]
            map.insert(79, &[192]);     // 79 => [192]
            map.insert(143, &[320]);     // 143 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(114, &[1088]);     // 114 => [1088]
            map.insert(228, &[2112]);     // 228 => [2112]
            map.insert(254, &[4160]);     // 254 => [4160]
            map.insert(148, &[8256]);     // 148 => [8256]
            map.insert(215, &[16448]);     // 215 => [16448]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(118, &[640]);     // 118 => [640]
            map.insert(61, &[1152]);     // 61 => [1152]
            map.insert(171, &[2176]);     // 171 => [2176]
            map.insert(177, &[4224]);     // 177 => [4224]
            map.insert(219, &[8320]);     // 219 => [8320]
            map.insert(152, &[16512]);     // 152 => [16512]
            map.insert(182, &[768]);     // 182 => [768]
            map.insert(253, &[1280]);     // 253 => [1280]
            map.insert(107, &[2304]);     // 107 => [2304]
            map.insert(113, &[4352]);     // 113 => [4352]
            map.insert(27, &[8448]);     // 27 => [8448]
            map.insert(88, &[16640]);     // 88 => [16640]
            map.insert(75, &[1536]);     // 75 => [1536]
            map.insert(221, &[2560]);     // 221 => [2560]
            map.insert(199, &[4608]);     // 199 => [4608]
            map.insert(173, &[8704]);     // 173 => [8704]
            map.insert(238, &[16896]);     // 238 => [16896]
            map.insert(150, &[3072]);     // 150 => [3072]
            map.insert(140, &[5120]);     // 140 => [5120]
            map.insert(230, &[9216]);     // 230 => [9216]
            map.insert(165, &[17408]);     // 165 => [17408]
            map.insert(26, &[6144]);     // 26 => [6144]
            map.insert(112, &[10240]);     // 112 => [10240]
            map.insert(51, &[18432]);     // 51 => [18432]
            map.insert(106, &[12288]);     // 106 => [12288]
            map.insert(41, &[20480]);     // 41 => [20480]
            map.insert(67, &[24576]);     // 67 => [24576]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(53, &[515]);     // 53 => [515]
            map.insert(126, &[1027]);     // 126 => [1027]
            map.insert(232, &[2051]);     // 232 => [2051]
            map.insert(242, &[4099]);     // 242 => [4099]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(120, &[1029]);     // 120 => [1029]
            map.insert(244, &[4101]);     // 244 => [4101]
            map.insert(158, &[8197]);     // 158 => [8197]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(63, &[521]);     // 63 => [521]
            map.insert(116, &[1033]);     // 116 => [1033]
            map.insert(226, &[2057]);     // 226 => [2057]
            map.insert(146, &[8201]);     // 146 => [8201]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(30, &[81]);     // 30 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(39, &[529]);     // 39 => [529]
            map.insert(108, &[1041]);     // 108 => [1041]
            map.insert(250, &[2065]);     // 250 => [2065]
            map.insert(224, &[4113]);     // 224 => [4113]
            map.insert(138, &[8209]);     // 138 => [8209]
            map.insert(201, &[16401]);     // 201 => [16401]
            map.insert(46, &[97]);     // 46 => [97]
            map.insert(97, &[161]);     // 97 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(23, &[545]);     // 23 => [545]
            map.insert(92, &[1057]);     // 92 => [1057]
            map.insert(202, &[2081]);     // 202 => [2081]
            map.insert(186, &[8225]);     // 186 => [8225]
            map.insert(78, &[193]);     // 78 => [193]
            map.insert(142, &[321]);     // 142 => [321]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(115, &[1089]);     // 115 => [1089]
            map.insert(229, &[2113]);     // 229 => [2113]
            map.insert(255, &[4161]);     // 255 => [4161]
            map.insert(149, &[8257]);     // 149 => [8257]
            map.insert(214, &[16449]);     // 214 => [16449]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(119, &[641]);     // 119 => [641]
            map.insert(60, &[1153]);     // 60 => [1153]
            map.insert(170, &[2177]);     // 170 => [2177]
            map.insert(176, &[4225]);     // 176 => [4225]
            map.insert(183, &[769]);     // 183 => [769]
            map.insert(252, &[1281]);     // 252 => [1281]
            map.insert(89, &[16641]);     // 89 => [16641]
            map.insert(74, &[1537]);     // 74 => [1537]
            map.insert(198, &[4609]);     // 198 => [4609]
            map.insert(172, &[8705]);     // 172 => [8705]
            map.insert(151, &[3073]);     // 151 => [3073]
            map.insert(141, &[5121]);     // 141 => [5121]
            map.insert(231, &[9217]);     // 231 => [9217]
            map.insert(164, &[17409]);     // 164 => [17409]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(123, &[1030]);     // 123 => [1030]
            map.insert(237, &[2054]);     // 237 => [2054]
            map.insert(247, &[4102]);     // 247 => [4102]
            map.insert(157, &[8198]);     // 157 => [8198]
            map.insert(222, &[16390]);     // 222 => [16390]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(210, &[16394]);     // 210 => [16394]
            map.insert(29, &[82]);     // 29 => [82]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(111, &[1042]);     // 111 => [1042]
            map.insert(45, &[98]);     // 45 => [98]
            map.insert(98, &[162]);     // 98 => [162]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(95, &[1058]);     // 95 => [1058]
            map.insert(211, &[4130]);     // 211 => [4130]
            map.insert(185, &[8226]);     // 185 => [8226]
            map.insert(77, &[194]);     // 77 => [194]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(213, &[16450]);     // 213 => [16450]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(169, &[2178]);     // 169 => [2178]
            map.insert(179, &[4226]);     // 179 => [4226]
            map.insert(180, &[770]);     // 180 => [770]
            map.insert(105, &[2306]);     // 105 => [2306]
            map.insert(90, &[16642]);     // 90 => [16642]
            map.insert(223, &[2562]);     // 223 => [2562]
            map.insert(197, &[4610]);     // 197 => [4610]
            map.insert(175, &[8706]);     // 175 => [8706]
            map.insert(236, &[16898]);     // 236 => [16898]
            map.insert(167, &[17410]);     // 167 => [17410]
            map.insert(104, &[12290]);     // 104 => [12290]
            map.insert(43, &[20482]);     // 43 => [20482]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(76, &[140]);     // 76 => [140]
            map.insert(58, &[524]);     // 58 => [524]
            map.insert(212, &[16396]);     // 212 => [16396]
            map.insert(84, &[148]);     // 84 => [148]
            map.insert(204, &[16404]);     // 204 => [16404]
            map.insert(100, &[164]);     // 100 => [164]
            map.insert(207, &[2084]);     // 207 => [2084]
            map.insert(191, &[8228]);     // 191 => [8228]
            map.insert(196, &[388]);     // 196 => [388]
            map.insert(181, &[4228]);     // 181 => [4228]
            map.insert(156, &[16516]);     // 156 => [16516]
            map.insert(178, &[772]);     // 178 => [772]
            map.insert(195, &[4612]);     // 195 => [4612]
            map.insert(110, &[12292]);     // 110 => [12292]
            map.insert(71, &[24580]);     // 71 => [24580]
            map.insert(101, &[1048]);     // 101 => [1048]
            map.insert(168, &[296]);     // 168 => [296]
            map.insert(85, &[1064]);     // 85 => [1064]
            map.insert(135, &[328]);     // 135 => [328]
            map.insert(122, &[1096]);     // 122 => [1096]
            map.insert(246, &[4168]);     // 246 => [4168]
            map.insert(163, &[2184]);     // 163 => [2184]
            map.insert(190, &[776]);     // 190 => [776]
            map.insert(99, &[2312]);     // 99 => [2312]
            map.insert(102, &[656]);     // 102 => [656]
            map.insert(166, &[784]);     // 166 => [784]
            map.insert(91, &[1552]);     // 91 => [1552]
            map.insert(205, &[2576]);     // 205 => [2576]
            map.insert(189, &[8720]);     // 189 => [8720]
            map.insert(83, &[24592]);     // 83 => [24592]
            map.insert(86, &[672]);     // 86 => [672]
            map.insert(184, &[16544]);     // 184 => [16544]
            map.insert(206, &[16928]);     // 206 => [16928]
            map.insert(87, &[16704]);     // 87 => [16704]
            map.insert(174, &[17024]);     // 174 => [17024]
            map.insert(103, &[7168]);     // 103 => [7168]
            map.insert(94, &[1059]);     // 94 => [1059]
            map.insert(188, &[8721]);     // 188 => [8721]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode15_7 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode15_7 {
    fn name(&self) -> String {
        "[15, 7] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        15
    }

    fn dimension(&self) -> usize {
        7
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
        let mut error = BinVector::with_capacity(15);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 15 / 64 + if 15 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(15) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(7);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[15 / 64] & !((1 << 15) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode15_7.generator_matrix();
        assert_eq!(code.ncols(), 15);
        assert_eq!(code.nrows(), 7);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode15_7;
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
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, true, false, false, true, true, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
