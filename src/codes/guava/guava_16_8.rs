use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[16, 8]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode16_8;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 30977 ],
                &[ 29186 ],
                &[ 45828 ],
                &[ 58888 ],
                &[ 52240 ],
                &[ 20256 ],
                &[ 7744 ],
                &[ 60288 ],
                
            ], 16));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 61953 ],
                &[ 30850 ],
                &[ 21636 ],
                &[ 65160 ],
                &[ 10896 ],
                &[ 7712 ],
                &[ 52800 ],
                &[ 47360 ],
                
            ], 16));
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
            map.insert(30, &[128]);     // 30 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(121, &[512]);     // 121 => [512]
            map.insert(108, &[1024]);     // 108 => [1024]
            map.insert(250, &[2048]);     // 250 => [2048]
            map.insert(175, &[4096]);     // 175 => [4096]
            map.insert(155, &[8192]);     // 155 => [8192]
            map.insert(79, &[16384]);     // 79 => [16384]
            map.insert(201, &[32768]);     // 201 => [32768]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(31, &[129]);     // 31 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(120, &[513]);     // 120 => [513]
            map.insert(109, &[1025]);     // 109 => [1025]
            map.insert(251, &[2049]);     // 251 => [2049]
            map.insert(174, &[4097]);     // 174 => [4097]
            map.insert(154, &[8193]);     // 154 => [8193]
            map.insert(78, &[16385]);     // 78 => [16385]
            map.insert(200, &[32769]);     // 200 => [32769]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(28, &[130]);     // 28 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(123, &[514]);     // 123 => [514]
            map.insert(110, &[1026]);     // 110 => [1026]
            map.insert(248, &[2050]);     // 248 => [2050]
            map.insert(173, &[4098]);     // 173 => [4098]
            map.insert(153, &[8194]);     // 153 => [8194]
            map.insert(77, &[16386]);     // 77 => [16386]
            map.insert(203, &[32770]);     // 203 => [32770]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(26, &[132]);     // 26 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(104, &[1028]);     // 104 => [1028]
            map.insert(254, &[2052]);     // 254 => [2052]
            map.insert(171, &[4100]);     // 171 => [4100]
            map.insert(159, &[8196]);     // 159 => [8196]
            map.insert(75, &[16388]);     // 75 => [16388]
            map.insert(205, &[32772]);     // 205 => [32772]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(22, &[136]);     // 22 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(100, &[1032]);     // 100 => [1032]
            map.insert(242, &[2056]);     // 242 => [2056]
            map.insert(167, &[4104]);     // 167 => [4104]
            map.insert(147, &[8200]);     // 147 => [8200]
            map.insert(71, &[16392]);     // 71 => [16392]
            map.insert(193, &[32776]);     // 193 => [32776]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(14, &[144]);     // 14 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(124, &[1040]);     // 124 => [1040]
            map.insert(234, &[2064]);     // 234 => [2064]
            map.insert(191, &[4112]);     // 191 => [4112]
            map.insert(139, &[8208]);     // 139 => [8208]
            map.insert(95, &[16400]);     // 95 => [16400]
            map.insert(217, &[32784]);     // 217 => [32784]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(62, &[160]);     // 62 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(76, &[1056]);     // 76 => [1056]
            map.insert(218, &[2080]);     // 218 => [2080]
            map.insert(143, &[4128]);     // 143 => [4128]
            map.insert(187, &[8224]);     // 187 => [8224]
            map.insert(111, &[16416]);     // 111 => [16416]
            map.insert(233, &[32800]);     // 233 => [32800]
            map.insert(94, &[192]);     // 94 => [192]
            map.insert(192, &[320]);     // 192 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(44, &[1088]);     // 44 => [1088]
            map.insert(186, &[2112]);     // 186 => [2112]
            map.insert(239, &[4160]);     // 239 => [4160]
            map.insert(219, &[8256]);     // 219 => [8256]
            map.insert(15, &[16448]);     // 15 => [16448]
            map.insert(137, &[32832]);     // 137 => [32832]
            map.insert(158, &[384]);     // 158 => [384]
            map.insert(103, &[640]);     // 103 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(228, &[2176]);     // 228 => [2176]
            map.insert(177, &[4224]);     // 177 => [4224]
            map.insert(133, &[8320]);     // 133 => [8320]
            map.insert(81, &[16512]);     // 81 => [16512]
            map.insert(215, &[32896]);     // 215 => [32896]
            map.insert(249, &[768]);     // 249 => [768]
            map.insert(236, &[1280]);     // 236 => [1280]
            map.insert(122, &[2304]);     // 122 => [2304]
            map.insert(47, &[4352]);     // 47 => [4352]
            map.insert(27, &[8448]);     // 27 => [8448]
            map.insert(207, &[16640]);     // 207 => [16640]
            map.insert(73, &[33024]);     // 73 => [33024]
            map.insert(21, &[1536]);     // 21 => [1536]
            map.insert(131, &[2560]);     // 131 => [2560]
            map.insert(214, &[4608]);     // 214 => [4608]
            map.insert(226, &[8704]);     // 226 => [8704]
            map.insert(54, &[16896]);     // 54 => [16896]
            map.insert(176, &[33280]);     // 176 => [33280]
            map.insert(150, &[3072]);     // 150 => [3072]
            map.insert(195, &[5120]);     // 195 => [5120]
            map.insert(247, &[9216]);     // 247 => [9216]
            map.insert(35, &[17408]);     // 35 => [17408]
            map.insert(165, &[33792]);     // 165 => [33792]
            map.insert(85, &[6144]);     // 85 => [6144]
            map.insert(97, &[10240]);     // 97 => [10240]
            map.insert(181, &[18432]);     // 181 => [18432]
            map.insert(51, &[34816]);     // 51 => [34816]
            map.insert(52, &[12288]);     // 52 => [12288]
            map.insert(224, &[20480]);     // 224 => [20480]
            map.insert(102, &[36864]);     // 102 => [36864]
            map.insert(212, &[24576]);     // 212 => [24576]
            map.insert(82, &[40960]);     // 82 => [40960]
            map.insert(134, &[49152]);     // 134 => [49152]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(29, &[131]);     // 29 => [131]
            map.insert(172, &[4099]);     // 172 => [4099]
            map.insert(152, &[8195]);     // 152 => [8195]
            map.insert(202, &[32771]);     // 202 => [32771]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(255, &[2053]);     // 255 => [2053]
            map.insert(170, &[4101]);     // 170 => [4101]
            map.insert(74, &[16389]);     // 74 => [16389]
            map.insert(204, &[32773]);     // 204 => [32773]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(23, &[137]);     // 23 => [137]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(101, &[1033]);     // 101 => [1033]
            map.insert(243, &[2057]);     // 243 => [2057]
            map.insert(166, &[4105]);     // 166 => [4105]
            map.insert(146, &[8201]);     // 146 => [8201]
            map.insert(70, &[16393]);     // 70 => [16393]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(235, &[2065]);     // 235 => [2065]
            map.insert(190, &[4113]);     // 190 => [4113]
            map.insert(138, &[8209]);     // 138 => [8209]
            map.insert(216, &[32785]);     // 216 => [32785]
            map.insert(63, &[161]);     // 63 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(88, &[545]);     // 88 => [545]
            map.insert(142, &[4129]);     // 142 => [4129]
            map.insert(232, &[32801]);     // 232 => [32801]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(45, &[1089]);     // 45 => [1089]
            map.insert(238, &[4161]);     // 238 => [4161]
            map.insert(115, &[1153]);     // 115 => [1153]
            map.insert(229, &[2177]);     // 229 => [2177]
            map.insert(237, &[1281]);     // 237 => [1281]
            map.insert(46, &[4353]);     // 46 => [4353]
            map.insert(206, &[16641]);     // 206 => [16641]
            map.insert(227, &[8705]);     // 227 => [8705]
            map.insert(55, &[16897]);     // 55 => [16897]
            map.insert(151, &[3073]);     // 151 => [3073]
            map.insert(194, &[5121]);     // 194 => [5121]
            map.insert(246, &[9217]);     // 246 => [9217]
            map.insert(164, &[33793]);     // 164 => [33793]
            map.insert(84, &[6145]);     // 84 => [6145]
            map.insert(180, &[18433]);     // 180 => [18433]
            map.insert(50, &[34817]);     // 50 => [34817]
            map.insert(53, &[12289]);     // 53 => [12289]
            map.insert(225, &[20481]);     // 225 => [20481]
            map.insert(213, &[24577]);     // 213 => [24577]
            map.insert(83, &[40961]);     // 83 => [40961]
            map.insert(135, &[49153]);     // 135 => [49153]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(127, &[518]);     // 127 => [518]
            map.insert(106, &[1030]);     // 106 => [1030]
            map.insert(252, &[2054]);     // 252 => [2054]
            map.insert(169, &[4102]);     // 169 => [4102]
            map.insert(157, &[8198]);     // 157 => [8198]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(240, &[2058]);     // 240 => [2058]
            map.insert(107, &[530]);     // 107 => [530]
            map.insert(126, &[1042]);     // 126 => [1042]
            map.insert(189, &[4114]);     // 189 => [4114]
            map.insert(93, &[16402]);     // 93 => [16402]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(60, &[162]);     // 60 => [162]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(91, &[546]);     // 91 => [546]
            map.insert(141, &[4130]);     // 141 => [4130]
            map.insert(185, &[8226]);     // 185 => [8226]
            map.insert(92, &[194]);     // 92 => [194]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(184, &[2114]);     // 184 => [2114]
            map.insert(156, &[386]);     // 156 => [386]
            map.insert(230, &[2178]);     // 230 => [2178]
            map.insert(179, &[4226]);     // 179 => [4226]
            map.insert(178, &[33282]);     // 178 => [33282]
            map.insert(148, &[3074]);     // 148 => [3074]
            map.insert(245, &[9218]);     // 245 => [9218]
            map.insert(87, &[6146]);     // 87 => [6146]
            map.insert(99, &[10242]);     // 99 => [10242]
            map.insert(183, &[18434]);     // 183 => [18434]
            map.insert(140, &[268]);     // 140 => [268]
            map.insert(117, &[524]);     // 117 => [524]
            map.insert(163, &[4108]);     // 163 => [4108]
            map.insert(197, &[32780]);     // 197 => [32780]
            map.insert(221, &[32788]);     // 221 => [32788]
            map.insert(58, &[164]);     // 58 => [164]
            map.insert(222, &[2084]);     // 222 => [2084]
            map.insert(90, &[196]);     // 90 => [196]
            map.insert(196, &[324]);     // 196 => [324]
            map.insert(61, &[580]);     // 61 => [580]
            map.insert(223, &[8260]);     // 223 => [8260]
            map.insert(118, &[1156]);     // 118 => [1156]
            map.insert(211, &[32900]);     // 211 => [32900]
            map.insert(253, &[772]);     // 253 => [772]
            map.insert(43, &[4356]);     // 43 => [4356]
            map.insert(210, &[4612]);     // 210 => [4612]
            map.insert(199, &[5124]);     // 199 => [5124]
            map.insert(39, &[17412]);     // 39 => [17412]
            map.insert(208, &[24580]);     // 208 => [24580]
            map.insert(86, &[40964]);     // 86 => [40964]
            map.insert(116, &[1048]);     // 116 => [1048]
            map.insert(209, &[32792]);     // 209 => [32792]
            map.insert(168, &[296]);     // 168 => [296]
            map.insert(231, &[4168]);     // 231 => [4168]
            map.insert(241, &[776]);     // 241 => [776]
            map.insert(220, &[24584]);     // 220 => [24584]
            map.insert(119, &[656]);     // 119 => [656]
            map.insert(244, &[2192]);     // 244 => [2192]
            map.insert(149, &[8336]);     // 149 => [8336]
            map.insert(198, &[4624]);     // 198 => [4624]
            map.insert(182, &[3104]);     // 182 => [3104]
            map.insert(188, &[4115]);     // 188 => [4115]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode16_8 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode16_8 {
    fn name(&self) -> String {
        "[16, 8] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        16
    }

    fn dimension(&self) -> usize {
        8
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
        let mut error = BinVector::with_capacity(16);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 16 / 64 + if 16 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(16) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(8);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[16 / 64] & !((1 << 16) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode16_8.generator_matrix();
        assert_eq!(code.ncols(), 16);
        assert_eq!(code.nrows(), 8);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode16_8;
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
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_8;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
