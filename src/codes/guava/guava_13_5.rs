use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[13, 5]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode13_5;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7361 ],
                &[ 6530 ],
                &[ 2532 ],
                &[ 968 ],
                &[ 7536 ],
                
            ], 13));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2817 ],
                &[ 5122 ],
                &[ 6148 ],
                &[ 520 ],
                &[ 3856 ],
                &[ 5920 ],
                &[ 7744 ],
                &[ 1408 ],
                
            ], 13));
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
            map.insert(177, &[256]);     // 177 => [256]
            map.insert(121, &[512]);     // 121 => [512]
            map.insert(242, &[1024]);     // 242 => [1024]
            map.insert(85, &[2048]);     // 85 => [2048]
            map.insert(102, &[4096]);     // 102 => [4096]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(176, &[257]);     // 176 => [257]
            map.insert(120, &[513]);     // 120 => [513]
            map.insert(243, &[1025]);     // 243 => [1025]
            map.insert(84, &[2049]);     // 84 => [2049]
            map.insert(103, &[4097]);     // 103 => [4097]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(179, &[258]);     // 179 => [258]
            map.insert(123, &[514]);     // 123 => [514]
            map.insert(240, &[1026]);     // 240 => [1026]
            map.insert(87, &[2050]);     // 87 => [2050]
            map.insert(100, &[4098]);     // 100 => [4098]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(181, &[260]);     // 181 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(246, &[1028]);     // 246 => [1028]
            map.insert(81, &[2052]);     // 81 => [2052]
            map.insert(98, &[4100]);     // 98 => [4100]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(185, &[264]);     // 185 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(250, &[1032]);     // 250 => [1032]
            map.insert(93, &[2056]);     // 93 => [2056]
            map.insert(110, &[4104]);     // 110 => [4104]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(161, &[272]);     // 161 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(226, &[1040]);     // 226 => [1040]
            map.insert(69, &[2064]);     // 69 => [2064]
            map.insert(118, &[4112]);     // 118 => [4112]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(145, &[288]);     // 145 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(210, &[1056]);     // 210 => [1056]
            map.insert(117, &[2080]);     // 117 => [2080]
            map.insert(70, &[4128]);     // 70 => [4128]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(241, &[320]);     // 241 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(178, &[1088]);     // 178 => [1088]
            map.insert(21, &[2112]);     // 21 => [2112]
            map.insert(38, &[4160]);     // 38 => [4160]
            map.insert(49, &[384]);     // 49 => [384]
            map.insert(249, &[640]);     // 249 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(213, &[2176]);     // 213 => [2176]
            map.insert(230, &[4224]);     // 230 => [4224]
            map.insert(200, &[768]);     // 200 => [768]
            map.insert(67, &[1280]);     // 67 => [1280]
            map.insert(228, &[2304]);     // 228 => [2304]
            map.insert(215, &[4352]);     // 215 => [4352]
            map.insert(139, &[1536]);     // 139 => [1536]
            map.insert(44, &[2560]);     // 44 => [2560]
            map.insert(31, &[4608]);     // 31 => [4608]
            map.insert(167, &[3072]);     // 167 => [3072]
            map.insert(148, &[5120]);     // 148 => [5120]
            map.insert(51, &[6144]);     // 51 => [6144]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(122, &[515]);     // 122 => [515]
            map.insert(86, &[2051]);     // 86 => [2051]
            map.insert(101, &[4099]);     // 101 => [4099]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(180, &[261]);     // 180 => [261]
            map.insert(124, &[517]);     // 124 => [517]
            map.insert(247, &[1029]);     // 247 => [1029]
            map.insert(99, &[4101]);     // 99 => [4101]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(184, &[265]);     // 184 => [265]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(251, &[1033]);     // 251 => [1033]
            map.insert(92, &[2057]);     // 92 => [2057]
            map.insert(111, &[4105]);     // 111 => [4105]
            map.insert(104, &[529]);     // 104 => [529]
            map.insert(227, &[1041]);     // 227 => [1041]
            map.insert(119, &[4113]);     // 119 => [4113]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(88, &[545]);     // 88 => [545]
            map.insert(211, &[1057]);     // 211 => [1057]
            map.insert(116, &[2081]);     // 116 => [2081]
            map.insert(71, &[4129]);     // 71 => [4129]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(39, &[4161]);     // 39 => [4161]
            map.insert(248, &[641]);     // 248 => [641]
            map.insert(115, &[1153]);     // 115 => [1153]
            map.insert(212, &[2177]);     // 212 => [2177]
            map.insert(231, &[4225]);     // 231 => [4225]
            map.insert(201, &[769]);     // 201 => [769]
            map.insert(229, &[2305]);     // 229 => [2305]
            map.insert(214, &[4353]);     // 214 => [4353]
            map.insert(138, &[1537]);     // 138 => [1537]
            map.insert(45, &[2561]);     // 45 => [2561]
            map.insert(30, &[4609]);     // 30 => [4609]
            map.insert(166, &[3073]);     // 166 => [3073]
            map.insert(149, &[5121]);     // 149 => [5121]
            map.insert(50, &[6145]);     // 50 => [6145]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(183, &[262]);     // 183 => [262]
            map.insert(127, &[518]);     // 127 => [518]
            map.insert(244, &[1030]);     // 244 => [1030]
            map.insert(83, &[2054]);     // 83 => [2054]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(187, &[266]);     // 187 => [266]
            map.insert(95, &[2058]);     // 95 => [2058]
            map.insert(108, &[4106]);     // 108 => [4106]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(146, &[146]);     // 146 => [146]
            map.insert(163, &[274]);     // 163 => [274]
            map.insert(107, &[530]);     // 107 => [530]
            map.insert(224, &[1042]);     // 224 => [1042]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(147, &[290]);     // 147 => [290]
            map.insert(91, &[546]);     // 91 => [546]
            map.insert(208, &[1058]);     // 208 => [1058]
            map.insert(194, &[194]);     // 194 => [194]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(23, &[2114]);     // 23 => [2114]
            map.insert(202, &[770]);     // 202 => [770]
            map.insert(46, &[2562]);     // 46 => [2562]
            map.insert(29, &[4610]);     // 29 => [4610]
            map.insert(165, &[3074]);     // 165 => [3074]
            map.insert(150, &[5122]);     // 150 => [5122]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(76, &[76]);     // 76 => [76]
            map.insert(140, &[140]);     // 140 => [140]
            map.insert(189, &[268]);     // 189 => [268]
            map.insert(254, &[1036]);     // 254 => [1036]
            map.insert(106, &[4108]);     // 106 => [4108]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(109, &[532]);     // 109 => [532]
            map.insert(164, &[164]);     // 164 => [164]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(245, &[324]);     // 245 => [324]
            map.insert(61, &[580]);     // 61 => [580]
            map.insert(182, &[1092]);     // 182 => [1092]
            map.insert(53, &[388]);     // 53 => [388]
            map.insert(253, &[644]);     // 253 => [644]
            map.insert(209, &[2180]);     // 209 => [2180]
            map.insert(204, &[772]);     // 204 => [772]
            map.insert(143, &[1540]);     // 143 => [1540]
            map.insert(27, &[4612]);     // 27 => [4612]
            map.insert(55, &[6148]);     // 55 => [6148]
            map.insert(152, &[152]);     // 152 => [152]
            map.insert(169, &[280]);     // 169 => [280]
            map.insert(234, &[1048]);     // 234 => [1048]
            map.insert(77, &[2072]);     // 77 => [2072]
            map.insert(126, &[4120]);     // 126 => [4120]
            map.insert(168, &[168]);     // 168 => [168]
            map.insert(153, &[296]);     // 153 => [296]
            map.insert(218, &[1064]);     // 218 => [1064]
            map.insert(78, &[4136]);     // 78 => [4136]
            map.insert(186, &[1096]);     // 186 => [1096]
            map.insert(221, &[2184]);     // 221 => [2184]
            map.insert(238, &[4232]);     // 238 => [4232]
            map.insert(75, &[1288]);     // 75 => [1288]
            map.insert(236, &[2312]);     // 236 => [2312]
            map.insert(223, &[4360]);     // 223 => [4360]
            map.insert(175, &[3080]);     // 175 => [3080]
            map.insert(156, &[5128]);     // 156 => [5128]
            map.insert(225, &[336]);     // 225 => [336]
            map.insert(54, &[4176]);     // 54 => [4176]
            map.insert(233, &[656]);     // 233 => [656]
            map.insert(197, &[2192]);     // 197 => [2192]
            map.insert(216, &[784]);     // 216 => [784]
            map.insert(199, &[4368]);     // 199 => [4368]
            map.insert(155, &[1552]);     // 155 => [1552]
            map.insert(60, &[2576]);     // 60 => [2576]
            map.insert(15, &[4624]);     // 15 => [4624]
            map.insert(217, &[672]);     // 217 => [672]
            map.insert(198, &[4256]);     // 198 => [4256]
            map.insert(232, &[800]);     // 232 => [800]
            map.insert(171, &[1568]);     // 171 => [1568]
            map.insert(63, &[4640]);     // 63 => [4640]
            map.insert(135, &[3104]);     // 135 => [3104]
            map.insert(151, &[4416]);     // 151 => [4416]
            map.insert(203, &[1600]);     // 203 => [1600]
            map.insert(195, &[1408]);     // 195 => [1408]
            map.insert(172, &[2688]);     // 172 => [2688]
            map.insert(159, &[4736]);     // 159 => [4736]
            map.insert(58, &[1792]);     // 58 => [1792]
            map.insert(157, &[2816]);     // 157 => [2816]
            map.insert(174, &[4864]);     // 174 => [4864]
            map.insert(222, &[3584]);     // 222 => [3584]
            map.insert(237, &[5632]);     // 237 => [5632]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(94, &[2059]);     // 94 => [2059]
            map.insert(90, &[547]);     // 90 => [547]
            map.insert(47, &[2563]);     // 47 => [2563]
            map.insert(141, &[141]);     // 141 => [141]
            map.insert(188, &[269]);     // 188 => [269]
            map.insert(255, &[1037]);     // 255 => [1037]
            map.insert(252, &[645]);     // 252 => [645]
            map.insert(205, &[773]);     // 205 => [773]
            map.insert(142, &[1541]);     // 142 => [1541]
            map.insert(235, &[1049]);     // 235 => [1049]
            map.insert(219, &[1065]);     // 219 => [1065]
            map.insert(79, &[4137]);     // 79 => [4137]
            map.insert(220, &[2185]);     // 220 => [2185]
            map.insert(239, &[4233]);     // 239 => [4233]
            map.insert(154, &[1553]);     // 154 => [1553]
            map.insert(170, &[1569]);     // 170 => [1569]
            map.insert(62, &[4641]);     // 62 => [4641]
            map.insert(173, &[2689]);     // 173 => [2689]
            map.insert(158, &[4737]);     // 158 => [4737]
            map.insert(191, &[270]);     // 191 => [270]
            map.insert(206, &[774]);     // 206 => [774]
            map.insert(190, &[1100]);     // 190 => [1100]
            map.insert(207, &[1604]);     // 207 => [1604]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode13_5 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode13_5 {
    fn name(&self) -> String {
        "[13, 5] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        13
    }

    fn dimension(&self) -> usize {
        5
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
        let mut error = BinVector::with_capacity(13);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 13 / 64 + if 13 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(13) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(5);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[13 / 64] & !((1 << 13) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode13_5.generator_matrix();
        assert_eq!(code.ncols(), 13);
        assert_eq!(code.nrows(), 5);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode13_5;
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
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_5;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, true, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
