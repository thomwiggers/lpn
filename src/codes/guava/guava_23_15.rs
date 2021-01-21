use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 15]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode23_15;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7503873 ],
                &[ 4358146 ],
                &[ 4489220 ],
                &[ 4751368 ],
                &[ 5275664 ],
                &[ 6324256 ],
                &[ 7012416 ],
                &[ 7536768 ],
                &[ 4391168 ],
                &[ 4522496 ],
                &[ 4785152 ],
                &[ 5310464 ],
                &[ 6361088 ],
                &[ 2760704 ],
                &[ 3293184 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 6229793 ],
                &[ 2232610 ],
                &[ 262660 ],
                &[ 7822216 ],
                &[ 5195696 ],
                &[ 73664 ],
                &[ 8282112 ],
                &[ 4292608 ],
                
            ], 23));
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
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(56, &[128]);     // 56 => [128]
            map.insert(59, &[256]);     // 59 => [256]
            map.insert(61, &[512]);     // 61 => [512]
            map.insert(49, &[1024]);     // 49 => [1024]
            map.insert(41, &[2048]);     // 41 => [2048]
            map.insert(42, &[4096]);     // 42 => [4096]
            map.insert(64, &[8192]);     // 64 => [8192]
            map.insert(88, &[16384]);     // 88 => [16384]
            map.insert(128, &[32768]);     // 128 => [32768]
            map.insert(185, &[65536]);     // 185 => [65536]
            map.insert(91, &[131072]);     // 91 => [131072]
            map.insert(93, &[262144]);     // 93 => [262144]
            map.insert(81, &[524288]);     // 81 => [524288]
            map.insert(73, &[1048576]);     // 73 => [1048576]
            map.insert(74, &[2097152]);     // 74 => [2097152]
            map.insert(217, &[4194304]);     // 217 => [4194304]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(57, &[129]);     // 57 => [129]
            map.insert(58, &[257]);     // 58 => [257]
            map.insert(60, &[513]);     // 60 => [513]
            map.insert(48, &[1025]);     // 48 => [1025]
            map.insert(40, &[2049]);     // 40 => [2049]
            map.insert(43, &[4097]);     // 43 => [4097]
            map.insert(65, &[8193]);     // 65 => [8193]
            map.insert(89, &[16385]);     // 89 => [16385]
            map.insert(129, &[32769]);     // 129 => [32769]
            map.insert(184, &[65537]);     // 184 => [65537]
            map.insert(90, &[131073]);     // 90 => [131073]
            map.insert(92, &[262145]);     // 92 => [262145]
            map.insert(80, &[524289]);     // 80 => [524289]
            map.insert(72, &[1048577]);     // 72 => [1048577]
            map.insert(75, &[2097153]);     // 75 => [2097153]
            map.insert(216, &[4194305]);     // 216 => [4194305]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(63, &[514]);     // 63 => [514]
            map.insert(51, &[1026]);     // 51 => [1026]
            map.insert(66, &[8194]);     // 66 => [8194]
            map.insert(130, &[32770]);     // 130 => [32770]
            map.insert(187, &[65538]);     // 187 => [65538]
            map.insert(95, &[262146]);     // 95 => [262146]
            map.insert(83, &[524290]);     // 83 => [524290]
            map.insert(219, &[4194306]);     // 219 => [4194306]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(23, &[36]);     // 23 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(53, &[1028]);     // 53 => [1028]
            map.insert(45, &[2052]);     // 45 => [2052]
            map.insert(46, &[4100]);     // 46 => [4100]
            map.insert(68, &[8196]);     // 68 => [8196]
            map.insert(132, &[32772]);     // 132 => [32772]
            map.insert(189, &[65540]);     // 189 => [65540]
            map.insert(85, &[524292]);     // 85 => [524292]
            map.insert(77, &[1048580]);     // 77 => [1048580]
            map.insert(78, &[2097156]);     // 78 => [2097156]
            map.insert(221, &[4194308]);     // 221 => [4194308]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(136, &[32776]);     // 136 => [32776]
            map.insert(177, &[65544]);     // 177 => [65544]
            map.insert(209, &[4194312]);     // 209 => [4194312]
            map.insert(144, &[32784]);     // 144 => [32784]
            map.insert(169, &[65552]);     // 169 => [65552]
            map.insert(201, &[4194320]);     // 201 => [4194320]
            map.insert(147, &[32800]);     // 147 => [32800]
            map.insert(170, &[65568]);     // 170 => [65568]
            map.insert(202, &[4194336]);     // 202 => [4194336]
            map.insert(29, &[576]);     // 29 => [576]
            map.insert(96, &[8256]);     // 96 => [8256]
            map.insert(120, &[16448]);     // 120 => [16448]
            map.insert(160, &[32832]);     // 160 => [32832]
            map.insert(153, &[65600]);     // 153 => [65600]
            map.insert(123, &[131136]);     // 123 => [131136]
            map.insert(125, &[262208]);     // 125 => [262208]
            map.insert(113, &[524352]);     // 113 => [524352]
            map.insert(105, &[1048640]);     // 105 => [1048640]
            map.insert(106, &[2097216]);     // 106 => [2097216]
            map.insert(249, &[4194368]);     // 249 => [4194368]
            map.insert(99, &[131200]);     // 99 => [131200]
            map.insert(101, &[262272]);     // 101 => [262272]
            map.insert(114, &[2097280]);     // 114 => [2097280]
            map.insert(225, &[4194432]);     // 225 => [4194432]
            map.insert(102, &[262400]);     // 102 => [262400]
            map.insert(226, &[4194560]);     // 226 => [4194560]
            map.insert(108, &[524800]);     // 108 => [524800]
            map.insert(116, &[1049088]);     // 116 => [1049088]
            map.insert(119, &[2097664]);     // 119 => [2097664]
            map.insert(228, &[4194816]);     // 228 => [4194816]
            map.insert(232, &[4195328]);     // 232 => [4195328]
            map.insert(240, &[4196352]);     // 240 => [4196352]
            map.insert(243, &[4198400]);     // 243 => [4198400]
            map.insert(192, &[40960]);     // 192 => [40960]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(62, &[515]);     // 62 => [515]
            map.insert(50, &[1027]);     // 50 => [1027]
            map.insert(67, &[8195]);     // 67 => [8195]
            map.insert(131, &[32771]);     // 131 => [32771]
            map.insert(186, &[65539]);     // 186 => [65539]
            map.insert(94, &[262147]);     // 94 => [262147]
            map.insert(82, &[524291]);     // 82 => [524291]
            map.insert(218, &[4194307]);     // 218 => [4194307]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(22, &[37]);     // 22 => [37]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(52, &[1029]);     // 52 => [1029]
            map.insert(44, &[2053]);     // 44 => [2053]
            map.insert(47, &[4101]);     // 47 => [4101]
            map.insert(69, &[8197]);     // 69 => [8197]
            map.insert(133, &[32773]);     // 133 => [32773]
            map.insert(188, &[65541]);     // 188 => [65541]
            map.insert(84, &[524293]);     // 84 => [524293]
            map.insert(76, &[1048581]);     // 76 => [1048581]
            map.insert(79, &[2097157]);     // 79 => [2097157]
            map.insert(220, &[4194309]);     // 220 => [4194309]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(137, &[32777]);     // 137 => [32777]
            map.insert(176, &[65545]);     // 176 => [65545]
            map.insert(208, &[4194313]);     // 208 => [4194313]
            map.insert(145, &[32785]);     // 145 => [32785]
            map.insert(168, &[65553]);     // 168 => [65553]
            map.insert(200, &[4194321]);     // 200 => [4194321]
            map.insert(146, &[32801]);     // 146 => [32801]
            map.insert(171, &[65569]);     // 171 => [65569]
            map.insert(203, &[4194337]);     // 203 => [4194337]
            map.insert(28, &[577]);     // 28 => [577]
            map.insert(97, &[8257]);     // 97 => [8257]
            map.insert(121, &[16449]);     // 121 => [16449]
            map.insert(161, &[32833]);     // 161 => [32833]
            map.insert(152, &[65601]);     // 152 => [65601]
            map.insert(122, &[131137]);     // 122 => [131137]
            map.insert(124, &[262209]);     // 124 => [262209]
            map.insert(112, &[524353]);     // 112 => [524353]
            map.insert(104, &[1048641]);     // 104 => [1048641]
            map.insert(107, &[2097217]);     // 107 => [2097217]
            map.insert(248, &[4194369]);     // 248 => [4194369]
            map.insert(98, &[131201]);     // 98 => [131201]
            map.insert(100, &[262273]);     // 100 => [262273]
            map.insert(115, &[2097281]);     // 115 => [2097281]
            map.insert(224, &[4194433]);     // 224 => [4194433]
            map.insert(103, &[262401]);     // 103 => [262401]
            map.insert(227, &[4194561]);     // 227 => [4194561]
            map.insert(109, &[524801]);     // 109 => [524801]
            map.insert(117, &[1049089]);     // 117 => [1049089]
            map.insert(118, &[2097665]);     // 118 => [2097665]
            map.insert(229, &[4194817]);     // 229 => [4194817]
            map.insert(233, &[4195329]);     // 233 => [4195329]
            map.insert(241, &[4196353]);     // 241 => [4196353]
            map.insert(242, &[4198401]);     // 242 => [4198401]
            map.insert(193, &[40961]);     // 193 => [40961]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(55, &[1030]);     // 55 => [1030]
            map.insert(70, &[8198]);     // 70 => [8198]
            map.insert(134, &[32774]);     // 134 => [32774]
            map.insert(191, &[65542]);     // 191 => [65542]
            map.insert(87, &[524294]);     // 87 => [524294]
            map.insert(223, &[4194310]);     // 223 => [4194310]
            map.insert(138, &[32778]);     // 138 => [32778]
            map.insert(179, &[65546]);     // 179 => [65546]
            map.insert(211, &[4194314]);     // 211 => [4194314]
            map.insert(31, &[578]);     // 31 => [578]
            map.insert(162, &[32834]);     // 162 => [32834]
            map.insert(155, &[65602]);     // 155 => [65602]
            map.insert(127, &[262210]);     // 127 => [262210]
            map.insert(251, &[4194370]);     // 251 => [4194370]
            map.insert(110, &[524802]);     // 110 => [524802]
            map.insert(230, &[4194818]);     // 230 => [4194818]
            map.insert(234, &[4195330]);     // 234 => [4195330]
            map.insert(194, &[40962]);     // 194 => [40962]
            map.insert(140, &[32780]);     // 140 => [32780]
            map.insert(181, &[65548]);     // 181 => [65548]
            map.insert(213, &[4194316]);     // 213 => [4194316]
            map.insert(148, &[32788]);     // 148 => [32788]
            map.insert(173, &[65556]);     // 173 => [65556]
            map.insert(205, &[4194324]);     // 205 => [4194324]
            map.insert(151, &[32804]);     // 151 => [32804]
            map.insert(174, &[65572]);     // 174 => [65572]
            map.insert(206, &[4194340]);     // 206 => [4194340]
            map.insert(164, &[32836]);     // 164 => [32836]
            map.insert(157, &[65604]);     // 157 => [65604]
            map.insert(253, &[4194372]);     // 253 => [4194372]
            map.insert(236, &[4195332]);     // 236 => [4195332]
            map.insert(244, &[4196356]);     // 244 => [4196356]
            map.insert(247, &[4198404]);     // 247 => [4198404]
            map.insert(196, &[40964]);     // 196 => [40964]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[71]);     // 39 => [71]
            map.insert(54, &[1031]);     // 54 => [1031]
            map.insert(71, &[8199]);     // 71 => [8199]
            map.insert(135, &[32775]);     // 135 => [32775]
            map.insert(190, &[65543]);     // 190 => [65543]
            map.insert(86, &[524295]);     // 86 => [524295]
            map.insert(222, &[4194311]);     // 222 => [4194311]
            map.insert(139, &[32779]);     // 139 => [32779]
            map.insert(178, &[65547]);     // 178 => [65547]
            map.insert(210, &[4194315]);     // 210 => [4194315]
            map.insert(30, &[579]);     // 30 => [579]
            map.insert(163, &[32835]);     // 163 => [32835]
            map.insert(154, &[65603]);     // 154 => [65603]
            map.insert(126, &[262211]);     // 126 => [262211]
            map.insert(250, &[4194371]);     // 250 => [4194371]
            map.insert(111, &[524803]);     // 111 => [524803]
            map.insert(231, &[4194819]);     // 231 => [4194819]
            map.insert(235, &[4195331]);     // 235 => [4195331]
            map.insert(195, &[40963]);     // 195 => [40963]
            map.insert(141, &[32781]);     // 141 => [32781]
            map.insert(180, &[65549]);     // 180 => [65549]
            map.insert(212, &[4194317]);     // 212 => [4194317]
            map.insert(149, &[32789]);     // 149 => [32789]
            map.insert(172, &[65557]);     // 172 => [65557]
            map.insert(204, &[4194325]);     // 204 => [4194325]
            map.insert(150, &[32805]);     // 150 => [32805]
            map.insert(175, &[65573]);     // 175 => [65573]
            map.insert(207, &[4194341]);     // 207 => [4194341]
            map.insert(165, &[32837]);     // 165 => [32837]
            map.insert(156, &[65605]);     // 156 => [65605]
            map.insert(252, &[4194373]);     // 252 => [4194373]
            map.insert(237, &[4195333]);     // 237 => [4195333]
            map.insert(245, &[4196357]);     // 245 => [4196357]
            map.insert(246, &[4198405]);     // 246 => [4198405]
            map.insert(197, &[40965]);     // 197 => [40965]
            map.insert(142, &[32782]);     // 142 => [32782]
            map.insert(183, &[65550]);     // 183 => [65550]
            map.insert(215, &[4194318]);     // 215 => [4194318]
            map.insert(166, &[32838]);     // 166 => [32838]
            map.insert(159, &[65606]);     // 159 => [65606]
            map.insert(255, &[4194374]);     // 255 => [4194374]
            map.insert(238, &[4195334]);     // 238 => [4195334]
            map.insert(198, &[40966]);     // 198 => [40966]
            map.insert(143, &[32783]);     // 143 => [32783]
            map.insert(182, &[65551]);     // 182 => [65551]
            map.insert(214, &[4194319]);     // 214 => [4194319]
            map.insert(167, &[32839]);     // 167 => [32839]
            map.insert(158, &[65607]);     // 158 => [65607]
            map.insert(254, &[4194375]);     // 254 => [4194375]
            map.insert(239, &[4195335]);     // 239 => [4195335]
            map.insert(199, &[40967]);     // 199 => [40967]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode23_15 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode23_15 {
    fn name(&self) -> String {
        "[23, 15] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        15
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
        let mut error = BinVector::with_capacity(23);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 23 / 64 + if 23 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(23) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(15);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[23 / 64] & !((1 << 23) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode23_15.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 15);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode23_15;
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
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, true, true, true, true, true, true, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, true, true, true, true, true, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, false, true, true, false, false, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, true, true, true, true, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, false, false, false, false, false, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, false, false, false, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, false, false, false, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, true, true, false, false, true, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, true, false, false, true, false, false, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, false, false, true, false, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, false, false, false, true, true, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, false, false, false, true, true, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, false, true, true, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, true, false, true, true, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, false, false, false, false, true, true, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, false, false, true, false, true, true, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, false, true, true, true, false, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, true, false, true, true, false, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, false, true, true, false, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, false, true, false, true, false, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, false, true, false, true, false, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, true, true, true, false, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, true, true, false, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_15;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, true, true, true, false, true, false, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
