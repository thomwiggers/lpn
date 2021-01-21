use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[20, 12]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode20_12;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 593921 ],
                &[ 659458 ],
                &[ 790532 ],
                &[ 876552 ],
                &[ 942096 ],
                &[ 548896 ],
                &[ 565312 ],
                &[ 598144 ],
                &[ 663808 ],
                &[ 795136 ],
                &[ 345088 ],
                &[ 411648 ],
                
            ], 20));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 944945 ],
                &[ 133394 ],
                &[ 279076 ],
                &[ 1015864 ],
                &[ 32832 ],
                &[ 992128 ],
                &[ 1035264 ],
                &[ 536576 ],
                
            ], 20));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(256, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(11, &[16]);     // 11 => [16]
            map.insert(13, &[32]);     // 13 => [32]
            map.insert(16, &[64]);     // 16 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(64, &[1024]);     // 64 => [1024]
            map.insert(67, &[2048]);     // 67 => [2048]
            map.insert(128, &[4096]);     // 128 => [4096]
            map.insert(161, &[8192]);     // 161 => [8192]
            map.insert(69, &[16384]);     // 69 => [16384]
            map.insert(88, &[32768]);     // 88 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(107, &[131072]);     // 107 => [131072]
            map.insert(109, &[262144]);     // 109 => [262144]
            map.insert(233, &[524288]);     // 233 => [524288]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(12, &[33]);     // 12 => [33]
            map.insert(17, &[65]);     // 17 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(65, &[1025]);     // 65 => [1025]
            map.insert(66, &[2049]);     // 66 => [2049]
            map.insert(129, &[4097]);     // 129 => [4097]
            map.insert(160, &[8193]);     // 160 => [8193]
            map.insert(68, &[16385]);     // 68 => [16385]
            map.insert(89, &[32769]);     // 89 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(106, &[131073]);     // 106 => [131073]
            map.insert(108, &[262145]);     // 108 => [262145]
            map.insert(232, &[524289]);     // 232 => [524289]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(15, &[34]);     // 15 => [34]
            map.insert(18, &[66]);     // 18 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(130, &[4098]);     // 130 => [4098]
            map.insert(163, &[8194]);     // 163 => [8194]
            map.insert(71, &[16386]);     // 71 => [16386]
            map.insert(90, &[32770]);     // 90 => [32770]
            map.insert(111, &[262146]);     // 111 => [262146]
            map.insert(235, &[524290]);     // 235 => [524290]
            map.insert(20, &[68]);     // 20 => [68]
            map.insert(132, &[4100]);     // 132 => [4100]
            map.insert(165, &[8196]);     // 165 => [8196]
            map.insert(92, &[32772]);     // 92 => [32772]
            map.insert(237, &[524292]);     // 237 => [524292]
            map.insert(24, &[72]);     // 24 => [72]
            map.insert(40, &[136]);     // 40 => [136]
            map.insert(43, &[264]);     // 43 => [264]
            map.insert(45, &[520]);     // 45 => [520]
            map.insert(72, &[1032]);     // 72 => [1032]
            map.insert(75, &[2056]);     // 75 => [2056]
            map.insert(136, &[4104]);     // 136 => [4104]
            map.insert(169, &[8200]);     // 169 => [8200]
            map.insert(77, &[16392]);     // 77 => [16392]
            map.insert(80, &[32776]);     // 80 => [32776]
            map.insert(96, &[65544]);     // 96 => [65544]
            map.insert(99, &[131080]);     // 99 => [131080]
            map.insert(101, &[262152]);     // 101 => [262152]
            map.insert(225, &[524296]);     // 225 => [524296]
            map.insert(27, &[80]);     // 27 => [80]
            map.insert(46, &[528]);     // 46 => [528]
            map.insert(139, &[4112]);     // 139 => [4112]
            map.insert(170, &[8208]);     // 170 => [8208]
            map.insert(78, &[16400]);     // 78 => [16400]
            map.insert(83, &[32784]);     // 83 => [32784]
            map.insert(102, &[262160]);     // 102 => [262160]
            map.insert(226, &[524304]);     // 226 => [524304]
            map.insert(29, &[96]);     // 29 => [96]
            map.insert(141, &[4128]);     // 141 => [4128]
            map.insert(172, &[8224]);     // 172 => [8224]
            map.insert(85, &[32800]);     // 85 => [32800]
            map.insert(228, &[524320]);     // 228 => [524320]
            map.insert(48, &[192]);     // 48 => [192]
            map.insert(51, &[320]);     // 51 => [320]
            map.insert(53, &[576]);     // 53 => [576]
            map.insert(144, &[4160]);     // 144 => [4160]
            map.insert(177, &[8256]);     // 177 => [8256]
            map.insert(120, &[65600]);     // 120 => [65600]
            map.insert(123, &[131136]);     // 123 => [131136]
            map.insert(125, &[262208]);     // 125 => [262208]
            map.insert(249, &[524352]);     // 249 => [524352]
            map.insert(201, &[524416]);     // 201 => [524416]
            map.insert(202, &[524544]);     // 202 => [524544]
            map.insert(204, &[524800]);     // 204 => [524800]
            map.insert(192, &[5120]);     // 192 => [5120]
            map.insert(195, &[6144]);     // 195 => [6144]
            map.insert(197, &[20480]);     // 197 => [20480]
            map.insert(216, &[36864]);     // 216 => [36864]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(14, &[35]);     // 14 => [35]
            map.insert(19, &[67]);     // 19 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(131, &[4099]);     // 131 => [4099]
            map.insert(162, &[8195]);     // 162 => [8195]
            map.insert(70, &[16387]);     // 70 => [16387]
            map.insert(91, &[32771]);     // 91 => [32771]
            map.insert(110, &[262147]);     // 110 => [262147]
            map.insert(234, &[524291]);     // 234 => [524291]
            map.insert(21, &[69]);     // 21 => [69]
            map.insert(133, &[4101]);     // 133 => [4101]
            map.insert(164, &[8197]);     // 164 => [8197]
            map.insert(93, &[32773]);     // 93 => [32773]
            map.insert(236, &[524293]);     // 236 => [524293]
            map.insert(25, &[73]);     // 25 => [73]
            map.insert(41, &[137]);     // 41 => [137]
            map.insert(42, &[265]);     // 42 => [265]
            map.insert(44, &[521]);     // 44 => [521]
            map.insert(73, &[1033]);     // 73 => [1033]
            map.insert(74, &[2057]);     // 74 => [2057]
            map.insert(137, &[4105]);     // 137 => [4105]
            map.insert(168, &[8201]);     // 168 => [8201]
            map.insert(76, &[16393]);     // 76 => [16393]
            map.insert(81, &[32777]);     // 81 => [32777]
            map.insert(97, &[65545]);     // 97 => [65545]
            map.insert(98, &[131081]);     // 98 => [131081]
            map.insert(100, &[262153]);     // 100 => [262153]
            map.insert(224, &[524297]);     // 224 => [524297]
            map.insert(26, &[81]);     // 26 => [81]
            map.insert(47, &[529]);     // 47 => [529]
            map.insert(138, &[4113]);     // 138 => [4113]
            map.insert(171, &[8209]);     // 171 => [8209]
            map.insert(79, &[16401]);     // 79 => [16401]
            map.insert(82, &[32785]);     // 82 => [32785]
            map.insert(103, &[262161]);     // 103 => [262161]
            map.insert(227, &[524305]);     // 227 => [524305]
            map.insert(28, &[97]);     // 28 => [97]
            map.insert(140, &[4129]);     // 140 => [4129]
            map.insert(173, &[8225]);     // 173 => [8225]
            map.insert(84, &[32801]);     // 84 => [32801]
            map.insert(229, &[524321]);     // 229 => [524321]
            map.insert(49, &[193]);     // 49 => [193]
            map.insert(50, &[321]);     // 50 => [321]
            map.insert(52, &[577]);     // 52 => [577]
            map.insert(145, &[4161]);     // 145 => [4161]
            map.insert(176, &[8257]);     // 176 => [8257]
            map.insert(121, &[65601]);     // 121 => [65601]
            map.insert(122, &[131137]);     // 122 => [131137]
            map.insert(124, &[262209]);     // 124 => [262209]
            map.insert(248, &[524353]);     // 248 => [524353]
            map.insert(200, &[524417]);     // 200 => [524417]
            map.insert(203, &[524545]);     // 203 => [524545]
            map.insert(205, &[524801]);     // 205 => [524801]
            map.insert(193, &[5121]);     // 193 => [5121]
            map.insert(194, &[6145]);     // 194 => [6145]
            map.insert(196, &[20481]);     // 196 => [20481]
            map.insert(217, &[36865]);     // 217 => [36865]
            map.insert(22, &[70]);     // 22 => [70]
            map.insert(134, &[4102]);     // 134 => [4102]
            map.insert(167, &[8198]);     // 167 => [8198]
            map.insert(94, &[32774]);     // 94 => [32774]
            map.insert(239, &[524294]);     // 239 => [524294]
            map.insert(31, &[98]);     // 31 => [98]
            map.insert(143, &[4130]);     // 143 => [4130]
            map.insert(174, &[8226]);     // 174 => [8226]
            map.insert(87, &[32802]);     // 87 => [32802]
            map.insert(230, &[524322]);     // 230 => [524322]
            map.insert(55, &[578]);     // 55 => [578]
            map.insert(146, &[4162]);     // 146 => [4162]
            map.insert(179, &[8258]);     // 179 => [8258]
            map.insert(127, &[262210]);     // 127 => [262210]
            map.insert(251, &[524354]);     // 251 => [524354]
            map.insert(206, &[524802]);     // 206 => [524802]
            map.insert(199, &[20482]);     // 199 => [20482]
            map.insert(218, &[36866]);     // 218 => [36866]
            map.insert(148, &[4164]);     // 148 => [4164]
            map.insert(181, &[8260]);     // 181 => [8260]
            map.insert(253, &[524356]);     // 253 => [524356]
            map.insert(220, &[36868]);     // 220 => [36868]
            map.insert(56, &[200]);     // 56 => [200]
            map.insert(59, &[328]);     // 59 => [328]
            map.insert(61, &[584]);     // 61 => [584]
            map.insert(152, &[4168]);     // 152 => [4168]
            map.insert(185, &[8264]);     // 185 => [8264]
            map.insert(112, &[65608]);     // 112 => [65608]
            map.insert(115, &[131144]);     // 115 => [131144]
            map.insert(117, &[262216]);     // 117 => [262216]
            map.insert(241, &[524360]);     // 241 => [524360]
            map.insert(208, &[36872]);     // 208 => [36872]
            map.insert(62, &[592]);     // 62 => [592]
            map.insert(155, &[4176]);     // 155 => [4176]
            map.insert(186, &[8272]);     // 186 => [8272]
            map.insert(118, &[262224]);     // 118 => [262224]
            map.insert(242, &[524368]);     // 242 => [524368]
            map.insert(211, &[36880]);     // 211 => [36880]
            map.insert(157, &[4192]);     // 157 => [4192]
            map.insert(188, &[8288]);     // 188 => [8288]
            map.insert(244, &[524384]);     // 244 => [524384]
            map.insert(213, &[36896]);     // 213 => [36896]
            map.insert(23, &[71]);     // 23 => [71]
            map.insert(135, &[4103]);     // 135 => [4103]
            map.insert(166, &[8199]);     // 166 => [8199]
            map.insert(95, &[32775]);     // 95 => [32775]
            map.insert(238, &[524295]);     // 238 => [524295]
            map.insert(30, &[99]);     // 30 => [99]
            map.insert(142, &[4131]);     // 142 => [4131]
            map.insert(175, &[8227]);     // 175 => [8227]
            map.insert(86, &[32803]);     // 86 => [32803]
            map.insert(231, &[524323]);     // 231 => [524323]
            map.insert(54, &[579]);     // 54 => [579]
            map.insert(147, &[4163]);     // 147 => [4163]
            map.insert(178, &[8259]);     // 178 => [8259]
            map.insert(126, &[262211]);     // 126 => [262211]
            map.insert(250, &[524355]);     // 250 => [524355]
            map.insert(207, &[524803]);     // 207 => [524803]
            map.insert(198, &[20483]);     // 198 => [20483]
            map.insert(219, &[36867]);     // 219 => [36867]
            map.insert(149, &[4165]);     // 149 => [4165]
            map.insert(180, &[8261]);     // 180 => [8261]
            map.insert(252, &[524357]);     // 252 => [524357]
            map.insert(221, &[36869]);     // 221 => [36869]
            map.insert(57, &[201]);     // 57 => [201]
            map.insert(58, &[329]);     // 58 => [329]
            map.insert(60, &[585]);     // 60 => [585]
            map.insert(153, &[4169]);     // 153 => [4169]
            map.insert(184, &[8265]);     // 184 => [8265]
            map.insert(113, &[65609]);     // 113 => [65609]
            map.insert(114, &[131145]);     // 114 => [131145]
            map.insert(116, &[262217]);     // 116 => [262217]
            map.insert(240, &[524361]);     // 240 => [524361]
            map.insert(209, &[36873]);     // 209 => [36873]
            map.insert(63, &[593]);     // 63 => [593]
            map.insert(154, &[4177]);     // 154 => [4177]
            map.insert(187, &[8273]);     // 187 => [8273]
            map.insert(119, &[262225]);     // 119 => [262225]
            map.insert(243, &[524369]);     // 243 => [524369]
            map.insert(210, &[36881]);     // 210 => [36881]
            map.insert(156, &[4193]);     // 156 => [4193]
            map.insert(189, &[8289]);     // 189 => [8289]
            map.insert(245, &[524385]);     // 245 => [524385]
            map.insert(212, &[36897]);     // 212 => [36897]
            map.insert(150, &[4166]);     // 150 => [4166]
            map.insert(183, &[8262]);     // 183 => [8262]
            map.insert(255, &[524358]);     // 255 => [524358]
            map.insert(222, &[36870]);     // 222 => [36870]
            map.insert(159, &[4194]);     // 159 => [4194]
            map.insert(190, &[8290]);     // 190 => [8290]
            map.insert(246, &[524386]);     // 246 => [524386]
            map.insert(215, &[36898]);     // 215 => [36898]
            map.insert(151, &[4167]);     // 151 => [4167]
            map.insert(182, &[8263]);     // 182 => [8263]
            map.insert(254, &[524359]);     // 254 => [524359]
            map.insert(223, &[36871]);     // 223 => [36871]
            map.insert(158, &[4195]);     // 158 => [4195]
            map.insert(191, &[8291]);     // 191 => [8291]
            map.insert(247, &[524387]);     // 247 => [524387]
            map.insert(214, &[36899]);     // 214 => [36899]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode20_12 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode20_12 {
    fn name(&self) -> String {
        "[20, 12] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        20
    }

    fn dimension(&self) -> usize {
        12
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
        let mut error = BinVector::with_capacity(20);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 20 / 64 + if 20 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(20) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(12);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[20 / 64] & !((1 << 20) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode20_12.generator_matrix();
        assert_eq!(code.ncols(), 20);
        assert_eq!(code.nrows(), 12);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode20_12;
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
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, true, false, true, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, false, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, false, true, false, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, false, false, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, false, false, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, true, false, false, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, true, false, false, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, true, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, false, false, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, false, true, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, true, false, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, true, false, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, false, true, true, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, true, true, true, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, true, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, true, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, true, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, true, true, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, false, false, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, false, true, false, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, true, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, true, true, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, true, true, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, true, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, true, false, true, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, true, false, true, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
