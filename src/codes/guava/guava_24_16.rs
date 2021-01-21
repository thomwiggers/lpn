use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[24, 16]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode24_16;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 13959169 ],
                &[ 15007746 ],
                &[ 8716292 ],
                &[ 8978440 ],
                &[ 9502736 ],
                &[ 10551328 ],
                &[ 12648512 ],
                &[ 14024832 ],
                &[ 15073536 ],
                &[ 8782336 ],
                &[ 9044992 ],
                &[ 9570304 ],
                &[ 10620928 ],
                &[ 12722176 ],
                &[ 5521408 ],
                &[ 6586368 ],
                
            ], 24));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 10391393 ],
                &[ 2134306 ],
                &[ 4465220 ],
                &[ 525320 ],
                &[ 7354480 ],
                &[ 147328 ],
                &[ 16564224 ],
                &[ 8585216 ],
                
            ], 24));
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
            map.insert(19, &[32]);     // 19 => [32]
            map.insert(21, &[64]);     // 21 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(41, &[1024]);     // 41 => [1024]
            map.insert(49, &[2048]);     // 49 => [2048]
            map.insert(50, &[4096]);     // 50 => [4096]
            map.insert(52, &[8192]);     // 52 => [8192]
            map.insert(64, &[16384]);     // 64 => [16384]
            map.insert(67, &[32768]);     // 67 => [32768]
            map.insert(128, &[65536]);     // 128 => [65536]
            map.insert(161, &[131072]);     // 161 => [131072]
            map.insert(69, &[262144]);     // 69 => [262144]
            map.insert(73, &[524288]);     // 73 => [524288]
            map.insert(81, &[1048576]);     // 81 => [1048576]
            map.insert(82, &[2097152]);     // 82 => [2097152]
            map.insert(84, &[4194304]);     // 84 => [4194304]
            map.insert(193, &[8388608]);     // 193 => [8388608]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(20, &[65]);     // 20 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(40, &[1025]);     // 40 => [1025]
            map.insert(48, &[2049]);     // 48 => [2049]
            map.insert(51, &[4097]);     // 51 => [4097]
            map.insert(53, &[8193]);     // 53 => [8193]
            map.insert(65, &[16385]);     // 65 => [16385]
            map.insert(66, &[32769]);     // 66 => [32769]
            map.insert(129, &[65537]);     // 129 => [65537]
            map.insert(160, &[131073]);     // 160 => [131073]
            map.insert(68, &[262145]);     // 68 => [262145]
            map.insert(72, &[524289]);     // 72 => [524289]
            map.insert(80, &[1048577]);     // 80 => [1048577]
            map.insert(83, &[2097153]);     // 83 => [2097153]
            map.insert(85, &[4194305]);     // 85 => [4194305]
            map.insert(192, &[8388609]);     // 192 => [8388609]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(23, &[66]);     // 23 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(43, &[1026]);     // 43 => [1026]
            map.insert(54, &[8194]);     // 54 => [8194]
            map.insert(130, &[65538]);     // 130 => [65538]
            map.insert(163, &[131074]);     // 163 => [131074]
            map.insert(71, &[262146]);     // 71 => [262146]
            map.insert(75, &[524290]);     // 75 => [524290]
            map.insert(86, &[4194306]);     // 86 => [4194306]
            map.insert(195, &[8388610]);     // 195 => [8388610]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(45, &[1028]);     // 45 => [1028]
            map.insert(132, &[65540]);     // 132 => [65540]
            map.insert(165, &[131076]);     // 165 => [131076]
            map.insert(77, &[524292]);     // 77 => [524292]
            map.insert(197, &[8388612]);     // 197 => [8388612]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(29, &[72]);     // 29 => [72]
            map.insert(57, &[2056]);     // 57 => [2056]
            map.insert(58, &[4104]);     // 58 => [4104]
            map.insert(60, &[8200]);     // 60 => [8200]
            map.insert(136, &[65544]);     // 136 => [65544]
            map.insert(169, &[131080]);     // 169 => [131080]
            map.insert(89, &[1048584]);     // 89 => [1048584]
            map.insert(90, &[2097160]);     // 90 => [2097160]
            map.insert(92, &[4194312]);     // 92 => [4194312]
            map.insert(201, &[8388616]);     // 201 => [8388616]
            map.insert(144, &[65552]);     // 144 => [65552]
            map.insert(177, &[131088]);     // 177 => [131088]
            map.insert(209, &[8388624]);     // 209 => [8388624]
            map.insert(147, &[65568]);     // 147 => [65568]
            map.insert(178, &[131104]);     // 178 => [131104]
            map.insert(210, &[8388640]);     // 210 => [8388640]
            map.insert(149, &[65600]);     // 149 => [65600]
            map.insert(180, &[131136]);     // 180 => [131136]
            map.insert(212, &[8388672]);     // 212 => [8388672]
            map.insert(96, &[16512]);     // 96 => [16512]
            map.insert(99, &[32896]);     // 99 => [32896]
            map.insert(101, &[262272]);     // 101 => [262272]
            map.insert(105, &[524416]);     // 105 => [524416]
            map.insert(113, &[1048704]);     // 113 => [1048704]
            map.insert(114, &[2097280]);     // 114 => [2097280]
            map.insert(116, &[4194432]);     // 116 => [4194432]
            map.insert(225, &[8388736]);     // 225 => [8388736]
            map.insert(102, &[262400]);     // 102 => [262400]
            map.insert(106, &[524544]);     // 106 => [524544]
            map.insert(119, &[4194560]);     // 119 => [4194560]
            map.insert(226, &[8388864]);     // 226 => [8388864]
            map.insert(108, &[524800]);     // 108 => [524800]
            map.insert(228, &[8389120]);     // 228 => [8389120]
            map.insert(120, &[1049600]);     // 120 => [1049600]
            map.insert(123, &[2098176]);     // 123 => [2098176]
            map.insert(125, &[4195328]);     // 125 => [4195328]
            map.insert(232, &[8389632]);     // 232 => [8389632]
            map.insert(240, &[8390656]);     // 240 => [8390656]
            map.insert(243, &[8392704]);     // 243 => [8392704]
            map.insert(245, &[8396800]);     // 245 => [8396800]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(22, &[67]);     // 22 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(42, &[1027]);     // 42 => [1027]
            map.insert(55, &[8195]);     // 55 => [8195]
            map.insert(131, &[65539]);     // 131 => [65539]
            map.insert(162, &[131075]);     // 162 => [131075]
            map.insert(70, &[262147]);     // 70 => [262147]
            map.insert(74, &[524291]);     // 74 => [524291]
            map.insert(87, &[4194307]);     // 87 => [4194307]
            map.insert(194, &[8388611]);     // 194 => [8388611]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(44, &[1029]);     // 44 => [1029]
            map.insert(133, &[65541]);     // 133 => [65541]
            map.insert(164, &[131077]);     // 164 => [131077]
            map.insert(76, &[524293]);     // 76 => [524293]
            map.insert(196, &[8388613]);     // 196 => [8388613]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(28, &[73]);     // 28 => [73]
            map.insert(56, &[2057]);     // 56 => [2057]
            map.insert(59, &[4105]);     // 59 => [4105]
            map.insert(61, &[8201]);     // 61 => [8201]
            map.insert(137, &[65545]);     // 137 => [65545]
            map.insert(168, &[131081]);     // 168 => [131081]
            map.insert(88, &[1048585]);     // 88 => [1048585]
            map.insert(91, &[2097161]);     // 91 => [2097161]
            map.insert(93, &[4194313]);     // 93 => [4194313]
            map.insert(200, &[8388617]);     // 200 => [8388617]
            map.insert(145, &[65553]);     // 145 => [65553]
            map.insert(176, &[131089]);     // 176 => [131089]
            map.insert(208, &[8388625]);     // 208 => [8388625]
            map.insert(146, &[65569]);     // 146 => [65569]
            map.insert(179, &[131105]);     // 179 => [131105]
            map.insert(211, &[8388641]);     // 211 => [8388641]
            map.insert(148, &[65601]);     // 148 => [65601]
            map.insert(181, &[131137]);     // 181 => [131137]
            map.insert(213, &[8388673]);     // 213 => [8388673]
            map.insert(97, &[16513]);     // 97 => [16513]
            map.insert(98, &[32897]);     // 98 => [32897]
            map.insert(100, &[262273]);     // 100 => [262273]
            map.insert(104, &[524417]);     // 104 => [524417]
            map.insert(112, &[1048705]);     // 112 => [1048705]
            map.insert(115, &[2097281]);     // 115 => [2097281]
            map.insert(117, &[4194433]);     // 117 => [4194433]
            map.insert(224, &[8388737]);     // 224 => [8388737]
            map.insert(103, &[262401]);     // 103 => [262401]
            map.insert(107, &[524545]);     // 107 => [524545]
            map.insert(118, &[4194561]);     // 118 => [4194561]
            map.insert(227, &[8388865]);     // 227 => [8388865]
            map.insert(109, &[524801]);     // 109 => [524801]
            map.insert(229, &[8389121]);     // 229 => [8389121]
            map.insert(121, &[1049601]);     // 121 => [1049601]
            map.insert(122, &[2098177]);     // 122 => [2098177]
            map.insert(124, &[4195329]);     // 124 => [4195329]
            map.insert(233, &[8389633]);     // 233 => [8389633]
            map.insert(241, &[8390657]);     // 241 => [8390657]
            map.insert(242, &[8392705]);     // 242 => [8392705]
            map.insert(244, &[8396801]);     // 244 => [8396801]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(47, &[1030]);     // 47 => [1030]
            map.insert(134, &[65542]);     // 134 => [65542]
            map.insert(167, &[131078]);     // 167 => [131078]
            map.insert(79, &[524294]);     // 79 => [524294]
            map.insert(199, &[8388614]);     // 199 => [8388614]
            map.insert(31, &[74]);     // 31 => [74]
            map.insert(62, &[8202]);     // 62 => [8202]
            map.insert(138, &[65546]);     // 138 => [65546]
            map.insert(171, &[131082]);     // 171 => [131082]
            map.insert(94, &[4194314]);     // 94 => [4194314]
            map.insert(203, &[8388618]);     // 203 => [8388618]
            map.insert(151, &[65602]);     // 151 => [65602]
            map.insert(182, &[131138]);     // 182 => [131138]
            map.insert(214, &[8388674]);     // 214 => [8388674]
            map.insert(110, &[524802]);     // 110 => [524802]
            map.insert(230, &[8389122]);     // 230 => [8389122]
            map.insert(127, &[4195330]);     // 127 => [4195330]
            map.insert(234, &[8389634]);     // 234 => [8389634]
            map.insert(247, &[8396802]);     // 247 => [8396802]
            map.insert(140, &[65548]);     // 140 => [65548]
            map.insert(173, &[131084]);     // 173 => [131084]
            map.insert(205, &[8388620]);     // 205 => [8388620]
            map.insert(236, &[8389636]);     // 236 => [8389636]
            map.insert(152, &[65560]);     // 152 => [65560]
            map.insert(185, &[131096]);     // 185 => [131096]
            map.insert(217, &[8388632]);     // 217 => [8388632]
            map.insert(155, &[65576]);     // 155 => [65576]
            map.insert(186, &[131112]);     // 186 => [131112]
            map.insert(218, &[8388648]);     // 218 => [8388648]
            map.insert(157, &[65608]);     // 157 => [65608]
            map.insert(188, &[131144]);     // 188 => [131144]
            map.insert(220, &[8388680]);     // 220 => [8388680]
            map.insert(248, &[8390664]);     // 248 => [8390664]
            map.insert(251, &[8392712]);     // 251 => [8392712]
            map.insert(253, &[8396808]);     // 253 => [8396808]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(46, &[1031]);     // 46 => [1031]
            map.insert(135, &[65543]);     // 135 => [65543]
            map.insert(166, &[131079]);     // 166 => [131079]
            map.insert(78, &[524295]);     // 78 => [524295]
            map.insert(198, &[8388615]);     // 198 => [8388615]
            map.insert(30, &[75]);     // 30 => [75]
            map.insert(63, &[8203]);     // 63 => [8203]
            map.insert(139, &[65547]);     // 139 => [65547]
            map.insert(170, &[131083]);     // 170 => [131083]
            map.insert(95, &[4194315]);     // 95 => [4194315]
            map.insert(202, &[8388619]);     // 202 => [8388619]
            map.insert(150, &[65603]);     // 150 => [65603]
            map.insert(183, &[131139]);     // 183 => [131139]
            map.insert(215, &[8388675]);     // 215 => [8388675]
            map.insert(111, &[524803]);     // 111 => [524803]
            map.insert(231, &[8389123]);     // 231 => [8389123]
            map.insert(126, &[4195331]);     // 126 => [4195331]
            map.insert(235, &[8389635]);     // 235 => [8389635]
            map.insert(246, &[8396803]);     // 246 => [8396803]
            map.insert(141, &[65549]);     // 141 => [65549]
            map.insert(172, &[131085]);     // 172 => [131085]
            map.insert(204, &[8388621]);     // 204 => [8388621]
            map.insert(237, &[8389637]);     // 237 => [8389637]
            map.insert(153, &[65561]);     // 153 => [65561]
            map.insert(184, &[131097]);     // 184 => [131097]
            map.insert(216, &[8388633]);     // 216 => [8388633]
            map.insert(154, &[65577]);     // 154 => [65577]
            map.insert(187, &[131113]);     // 187 => [131113]
            map.insert(219, &[8388649]);     // 219 => [8388649]
            map.insert(156, &[65609]);     // 156 => [65609]
            map.insert(189, &[131145]);     // 189 => [131145]
            map.insert(221, &[8388681]);     // 221 => [8388681]
            map.insert(249, &[8390665]);     // 249 => [8390665]
            map.insert(250, &[8392713]);     // 250 => [8392713]
            map.insert(252, &[8396809]);     // 252 => [8396809]
            map.insert(142, &[65550]);     // 142 => [65550]
            map.insert(175, &[131086]);     // 175 => [131086]
            map.insert(207, &[8388622]);     // 207 => [8388622]
            map.insert(238, &[8389638]);     // 238 => [8389638]
            map.insert(159, &[65610]);     // 159 => [65610]
            map.insert(190, &[131146]);     // 190 => [131146]
            map.insert(222, &[8388682]);     // 222 => [8388682]
            map.insert(255, &[8396810]);     // 255 => [8396810]
            map.insert(143, &[65551]);     // 143 => [65551]
            map.insert(174, &[131087]);     // 174 => [131087]
            map.insert(206, &[8388623]);     // 206 => [8388623]
            map.insert(239, &[8389639]);     // 239 => [8389639]
            map.insert(158, &[65611]);     // 158 => [65611]
            map.insert(191, &[131147]);     // 191 => [131147]
            map.insert(223, &[8388683]);     // 223 => [8388683]
            map.insert(254, &[8396811]);     // 254 => [8396811]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode24_16 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode24_16 {
    fn name(&self) -> String {
        "[24, 16] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        24
    }

    fn dimension(&self) -> usize {
        16
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
        let mut error = BinVector::with_capacity(24);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 24 / 64 + if 24 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(24) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(16);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[24 / 64] & !((1 << 24) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode24_16.generator_matrix();
        assert_eq!(code.ncols(), 24);
        assert_eq!(code.nrows(), 16);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode24_16;
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
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, false, true, true, false, false, true, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false, true, true, false, false, true, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, true, false, true, true, false, true, true, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, true, true, false, true, true, false, true, true, true, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false, true, false, true, true, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false, true, false, true, true, false, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, true, false, true, false, true, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, false, true, true, false, true, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, false, false, true, true, true, false, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, true, false, false, true, true, true, false, false, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, true, false, false, false, true, false, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, true, false, false, false, true, false, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, false, false, true, false, false, false, false, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, false, true, true, true, true, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, false, true, false, false, true, true, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, false, true, false, false, true, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, true, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, false, false, false, true, false, true, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, false, true, true, false, true, true, true, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, true, true, false, true, true, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, false, true, false, true, false, false, false, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, false, false, true, false, false, false, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, false, false, false, true, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_16;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, false, false, true, false, true, true, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, false, false, true, false, true, true, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
