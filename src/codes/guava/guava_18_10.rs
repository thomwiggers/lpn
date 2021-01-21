use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[18, 10]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode18_10;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 197633 ],
                &[ 219138 ],
                &[ 235524 ],
                &[ 137224 ],
                &[ 141328 ],
                &[ 149536 ],
                &[ 165952 ],
                &[ 198784 ],
                &[ 86272 ],
                &[ 102912 ],
                
            ], 18));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 133121 ],
                &[ 31426 ],
                &[ 33348 ],
                &[ 202888 ],
                &[ 8208 ],
                &[ 248032 ],
                &[ 258816 ],
                &[ 134144 ],
                
            ], 18));
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
            map.insert(38, &[64]);     // 38 => [64]
            map.insert(42, &[128]);     // 42 => [128]
            map.insert(64, &[256]);     // 64 => [256]
            map.insert(70, &[512]);     // 70 => [512]
            map.insert(128, &[1024]);     // 128 => [1024]
            map.insert(171, &[2048]);     // 171 => [2048]
            map.insert(74, &[4096]);     // 74 => [4096]
            map.insert(82, &[8192]);     // 82 => [8192]
            map.insert(98, &[16384]);     // 98 => [16384]
            map.insert(100, &[32768]);     // 100 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(233, &[131072]);     // 233 => [131072]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(39, &[65]);     // 39 => [65]
            map.insert(43, &[129]);     // 43 => [129]
            map.insert(65, &[257]);     // 65 => [257]
            map.insert(71, &[513]);     // 71 => [513]
            map.insert(129, &[1025]);     // 129 => [1025]
            map.insert(170, &[2049]);     // 170 => [2049]
            map.insert(75, &[4097]);     // 75 => [4097]
            map.insert(83, &[8193]);     // 83 => [8193]
            map.insert(99, &[16385]);     // 99 => [16385]
            map.insert(101, &[32769]);     // 101 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(232, &[131073]);     // 232 => [131073]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(36, &[66]);     // 36 => [66]
            map.insert(40, &[130]);     // 40 => [130]
            map.insert(66, &[258]);     // 66 => [258]
            map.insert(68, &[514]);     // 68 => [514]
            map.insert(130, &[1026]);     // 130 => [1026]
            map.insert(169, &[2050]);     // 169 => [2050]
            map.insert(72, &[4098]);     // 72 => [4098]
            map.insert(80, &[8194]);     // 80 => [8194]
            map.insert(96, &[16386]);     // 96 => [16386]
            map.insert(102, &[32770]);     // 102 => [32770]
            map.insert(106, &[65538]);     // 106 => [65538]
            map.insert(235, &[131074]);     // 235 => [131074]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(46, &[132]);     // 46 => [132]
            map.insert(132, &[1028]);     // 132 => [1028]
            map.insert(175, &[2052]);     // 175 => [2052]
            map.insert(78, &[4100]);     // 78 => [4100]
            map.insert(86, &[8196]);     // 86 => [8196]
            map.insert(108, &[65540]);     // 108 => [65540]
            map.insert(237, &[131076]);     // 237 => [131076]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(136, &[1032]);     // 136 => [1032]
            map.insert(163, &[2056]);     // 163 => [2056]
            map.insert(90, &[8200]);     // 90 => [8200]
            map.insert(225, &[131080]);     // 225 => [131080]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(54, &[80]);     // 54 => [80]
            map.insert(58, &[144]);     // 58 => [144]
            map.insert(144, &[1040]);     // 144 => [1040]
            map.insert(187, &[2064]);     // 187 => [2064]
            map.insert(114, &[16400]);     // 114 => [16400]
            map.insert(116, &[32784]);     // 116 => [32784]
            map.insert(120, &[65552]);     // 120 => [65552]
            map.insert(249, &[131088]);     // 249 => [131088]
            map.insert(160, &[1056]);     // 160 => [1056]
            map.insert(139, &[2080]);     // 139 => [2080]
            map.insert(201, &[131104]);     // 201 => [131104]
            map.insert(166, &[1088]);     // 166 => [1088]
            map.insert(141, &[2112]);     // 141 => [2112]
            map.insert(207, &[131136]);     // 207 => [131136]
            map.insert(195, &[131200]);     // 195 => [131200]
            map.insert(192, &[1280]);     // 192 => [1280]
            map.insert(198, &[1536]);     // 198 => [1536]
            map.insert(202, &[5120]);     // 202 => [5120]
            map.insert(210, &[9216]);     // 210 => [9216]
            map.insert(226, &[17408]);     // 226 => [17408]
            map.insert(228, &[33792]);     // 228 => [33792]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(37, &[67]);     // 37 => [67]
            map.insert(41, &[131]);     // 41 => [131]
            map.insert(67, &[259]);     // 67 => [259]
            map.insert(69, &[515]);     // 69 => [515]
            map.insert(131, &[1027]);     // 131 => [1027]
            map.insert(168, &[2051]);     // 168 => [2051]
            map.insert(73, &[4099]);     // 73 => [4099]
            map.insert(81, &[8195]);     // 81 => [8195]
            map.insert(97, &[16387]);     // 97 => [16387]
            map.insert(103, &[32771]);     // 103 => [32771]
            map.insert(107, &[65539]);     // 107 => [65539]
            map.insert(234, &[131075]);     // 234 => [131075]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(47, &[133]);     // 47 => [133]
            map.insert(133, &[1029]);     // 133 => [1029]
            map.insert(174, &[2053]);     // 174 => [2053]
            map.insert(79, &[4101]);     // 79 => [4101]
            map.insert(87, &[8197]);     // 87 => [8197]
            map.insert(109, &[65541]);     // 109 => [65541]
            map.insert(236, &[131077]);     // 236 => [131077]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(137, &[1033]);     // 137 => [1033]
            map.insert(162, &[2057]);     // 162 => [2057]
            map.insert(91, &[8201]);     // 91 => [8201]
            map.insert(224, &[131081]);     // 224 => [131081]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(55, &[81]);     // 55 => [81]
            map.insert(59, &[145]);     // 59 => [145]
            map.insert(145, &[1041]);     // 145 => [1041]
            map.insert(186, &[2065]);     // 186 => [2065]
            map.insert(115, &[16401]);     // 115 => [16401]
            map.insert(117, &[32785]);     // 117 => [32785]
            map.insert(121, &[65553]);     // 121 => [65553]
            map.insert(248, &[131089]);     // 248 => [131089]
            map.insert(161, &[1057]);     // 161 => [1057]
            map.insert(138, &[2081]);     // 138 => [2081]
            map.insert(200, &[131105]);     // 200 => [131105]
            map.insert(167, &[1089]);     // 167 => [1089]
            map.insert(140, &[2113]);     // 140 => [2113]
            map.insert(206, &[131137]);     // 206 => [131137]
            map.insert(194, &[131201]);     // 194 => [131201]
            map.insert(193, &[1281]);     // 193 => [1281]
            map.insert(199, &[1537]);     // 199 => [1537]
            map.insert(203, &[5121]);     // 203 => [5121]
            map.insert(211, &[9217]);     // 211 => [9217]
            map.insert(227, &[17409]);     // 227 => [17409]
            map.insert(229, &[33793]);     // 229 => [33793]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(44, &[134]);     // 44 => [134]
            map.insert(134, &[1030]);     // 134 => [1030]
            map.insert(173, &[2054]);     // 173 => [2054]
            map.insert(76, &[4102]);     // 76 => [4102]
            map.insert(84, &[8198]);     // 84 => [8198]
            map.insert(110, &[65542]);     // 110 => [65542]
            map.insert(239, &[131078]);     // 239 => [131078]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(88, &[8202]);     // 88 => [8202]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(52, &[82]);     // 52 => [82]
            map.insert(56, &[146]);     // 56 => [146]
            map.insert(146, &[1042]);     // 146 => [1042]
            map.insert(185, &[2066]);     // 185 => [2066]
            map.insert(112, &[16402]);     // 112 => [16402]
            map.insert(118, &[32786]);     // 118 => [32786]
            map.insert(122, &[65554]);     // 122 => [65554]
            map.insert(251, &[131090]);     // 251 => [131090]
            map.insert(164, &[1090]);     // 164 => [1090]
            map.insert(143, &[2114]);     // 143 => [2114]
            map.insert(205, &[131138]);     // 205 => [131138]
            map.insert(196, &[1538]);     // 196 => [1538]
            map.insert(208, &[9218]);     // 208 => [9218]
            map.insert(230, &[33794]);     // 230 => [33794]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(94, &[8204]);     // 94 => [8204]
            map.insert(62, &[148]);     // 62 => [148]
            map.insert(148, &[1044]);     // 148 => [1044]
            map.insert(191, &[2068]);     // 191 => [2068]
            map.insert(124, &[65556]);     // 124 => [65556]
            map.insert(253, &[131092]);     // 253 => [131092]
            map.insert(214, &[9220]);     // 214 => [9220]
            map.insert(152, &[1048]);     // 152 => [1048]
            map.insert(179, &[2072]);     // 179 => [2072]
            map.insert(241, &[131096]);     // 241 => [131096]
            map.insert(218, &[9224]);     // 218 => [9224]
            map.insert(176, &[1072]);     // 176 => [1072]
            map.insert(155, &[2096]);     // 155 => [2096]
            map.insert(217, &[131120]);     // 217 => [131120]
            map.insert(182, &[1104]);     // 182 => [1104]
            map.insert(157, &[2128]);     // 157 => [2128]
            map.insert(223, &[131152]);     // 223 => [131152]
            map.insert(242, &[17424]);     // 242 => [17424]
            map.insert(244, &[33808]);     // 244 => [33808]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(45, &[135]);     // 45 => [135]
            map.insert(135, &[1031]);     // 135 => [1031]
            map.insert(172, &[2055]);     // 172 => [2055]
            map.insert(77, &[4103]);     // 77 => [4103]
            map.insert(85, &[8199]);     // 85 => [8199]
            map.insert(111, &[65543]);     // 111 => [65543]
            map.insert(238, &[131079]);     // 238 => [131079]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(89, &[8203]);     // 89 => [8203]
            map.insert(51, &[51]);     // 51 => [51]
            map.insert(53, &[83]);     // 53 => [83]
            map.insert(57, &[147]);     // 57 => [147]
            map.insert(147, &[1043]);     // 147 => [1043]
            map.insert(184, &[2067]);     // 184 => [2067]
            map.insert(113, &[16403]);     // 113 => [16403]
            map.insert(119, &[32787]);     // 119 => [32787]
            map.insert(123, &[65555]);     // 123 => [65555]
            map.insert(250, &[131091]);     // 250 => [131091]
            map.insert(165, &[1091]);     // 165 => [1091]
            map.insert(142, &[2115]);     // 142 => [2115]
            map.insert(204, &[131139]);     // 204 => [131139]
            map.insert(197, &[1539]);     // 197 => [1539]
            map.insert(209, &[9219]);     // 209 => [9219]
            map.insert(231, &[33795]);     // 231 => [33795]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(95, &[8205]);     // 95 => [8205]
            map.insert(63, &[149]);     // 63 => [149]
            map.insert(149, &[1045]);     // 149 => [1045]
            map.insert(190, &[2069]);     // 190 => [2069]
            map.insert(125, &[65557]);     // 125 => [65557]
            map.insert(252, &[131093]);     // 252 => [131093]
            map.insert(215, &[9221]);     // 215 => [9221]
            map.insert(153, &[1049]);     // 153 => [1049]
            map.insert(178, &[2073]);     // 178 => [2073]
            map.insert(240, &[131097]);     // 240 => [131097]
            map.insert(219, &[9225]);     // 219 => [9225]
            map.insert(177, &[1073]);     // 177 => [1073]
            map.insert(154, &[2097]);     // 154 => [2097]
            map.insert(216, &[131121]);     // 216 => [131121]
            map.insert(183, &[1105]);     // 183 => [1105]
            map.insert(156, &[2129]);     // 156 => [2129]
            map.insert(222, &[131153]);     // 222 => [131153]
            map.insert(243, &[17425]);     // 243 => [17425]
            map.insert(245, &[33809]);     // 245 => [33809]
            map.insert(30, &[30]);     // 30 => [30]
            map.insert(92, &[8206]);     // 92 => [8206]
            map.insert(60, &[150]);     // 60 => [150]
            map.insert(150, &[1046]);     // 150 => [1046]
            map.insert(189, &[2070]);     // 189 => [2070]
            map.insert(126, &[65558]);     // 126 => [65558]
            map.insert(255, &[131094]);     // 255 => [131094]
            map.insert(212, &[9222]);     // 212 => [9222]
            map.insert(180, &[1106]);     // 180 => [1106]
            map.insert(159, &[2130]);     // 159 => [2130]
            map.insert(221, &[131154]);     // 221 => [131154]
            map.insert(246, &[33810]);     // 246 => [33810]
            map.insert(31, &[31]);     // 31 => [31]
            map.insert(93, &[8207]);     // 93 => [8207]
            map.insert(61, &[151]);     // 61 => [151]
            map.insert(151, &[1047]);     // 151 => [1047]
            map.insert(188, &[2071]);     // 188 => [2071]
            map.insert(127, &[65559]);     // 127 => [65559]
            map.insert(254, &[131095]);     // 254 => [131095]
            map.insert(213, &[9223]);     // 213 => [9223]
            map.insert(181, &[1107]);     // 181 => [1107]
            map.insert(158, &[2131]);     // 158 => [2131]
            map.insert(220, &[131155]);     // 220 => [131155]
            map.insert(247, &[33811]);     // 247 => [33811]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode18_10 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode18_10 {
    fn name(&self) -> String {
        "[18, 10] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        18
    }

    fn dimension(&self) -> usize {
        10
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
        let mut error = BinVector::with_capacity(18);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 18 / 64 + if 18 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(18) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(10);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[18 / 64] & !((1 << 18) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode18_10.generator_matrix();
        assert_eq!(code.ncols(), 18);
        assert_eq!(code.nrows(), 10);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode18_10;
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
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, false, true, true, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, false, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, false, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, true, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, true, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, true, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
