use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[19, 11]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode19_11;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 329729 ],
                &[ 395266 ],
                &[ 438276 ],
                &[ 471048 ],
                &[ 274448 ],
                &[ 282656 ],
                &[ 299072 ],
                &[ 331904 ],
                &[ 397568 ],
                &[ 172544 ],
                &[ 205824 ],
                
            ], 19));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 405777 ],
                &[ 139538 ],
                &[ 62852 ],
                &[ 472472 ],
                &[ 16416 ],
                &[ 496064 ],
                &[ 517632 ],
                &[ 268288 ],
                
            ], 19));
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
            map.insert(16, &[32]);     // 16 => [32]
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(44, &[128]);     // 44 => [128]
            map.insert(47, &[256]);     // 47 => [256]
            map.insert(64, &[512]);     // 64 => [512]
            map.insert(76, &[1024]);     // 76 => [1024]
            map.insert(128, &[2048]);     // 128 => [2048]
            map.insert(173, &[4096]);     // 173 => [4096]
            map.insert(79, &[8192]);     // 79 => [8192]
            map.insert(84, &[16384]);     // 84 => [16384]
            map.insert(100, &[32768]);     // 100 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(107, &[131072]);     // 107 => [131072]
            map.insert(233, &[262144]);     // 233 => [262144]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(17, &[33]);     // 17 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(45, &[129]);     // 45 => [129]
            map.insert(46, &[257]);     // 46 => [257]
            map.insert(65, &[513]);     // 65 => [513]
            map.insert(77, &[1025]);     // 77 => [1025]
            map.insert(129, &[2049]);     // 129 => [2049]
            map.insert(172, &[4097]);     // 172 => [4097]
            map.insert(78, &[8193]);     // 78 => [8193]
            map.insert(85, &[16385]);     // 85 => [16385]
            map.insert(101, &[32769]);     // 101 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(106, &[131073]);     // 106 => [131073]
            map.insert(232, &[262145]);     // 232 => [262145]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(18, &[34]);     // 18 => [34]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(66, &[514]);     // 66 => [514]
            map.insert(130, &[2050]);     // 130 => [2050]
            map.insert(175, &[4098]);     // 175 => [4098]
            map.insert(86, &[16386]);     // 86 => [16386]
            map.insert(102, &[32770]);     // 102 => [32770]
            map.insert(235, &[262146]);     // 235 => [262146]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(15, &[20]);     // 15 => [20]
            map.insert(20, &[36]);     // 20 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(40, &[132]);     // 40 => [132]
            map.insert(43, &[260]);     // 43 => [260]
            map.insert(68, &[516]);     // 68 => [516]
            map.insert(72, &[1028]);     // 72 => [1028]
            map.insert(132, &[2052]);     // 132 => [2052]
            map.insert(169, &[4100]);     // 169 => [4100]
            map.insert(75, &[8196]);     // 75 => [8196]
            map.insert(80, &[16388]);     // 80 => [16388]
            map.insert(96, &[32772]);     // 96 => [32772]
            map.insert(108, &[65540]);     // 108 => [65540]
            map.insert(111, &[131076]);     // 111 => [131076]
            map.insert(237, &[262148]);     // 237 => [262148]
            map.insert(24, &[40]);     // 24 => [40]
            map.insert(39, &[264]);     // 39 => [264]
            map.insert(136, &[2056]);     // 136 => [2056]
            map.insert(165, &[4104]);     // 165 => [4104]
            map.insert(71, &[8200]);     // 71 => [8200]
            map.insert(92, &[16392]);     // 92 => [16392]
            map.insert(99, &[131080]);     // 99 => [131080]
            map.insert(225, &[262152]);     // 225 => [262152]
            map.insert(27, &[48]);     // 27 => [48]
            map.insert(139, &[2064]);     // 139 => [2064]
            map.insert(166, &[4112]);     // 166 => [4112]
            map.insert(95, &[16400]);     // 95 => [16400]
            map.insert(226, &[262160]);     // 226 => [262160]
            map.insert(48, &[96]);     // 48 => [96]
            map.insert(60, &[160]);     // 60 => [160]
            map.insert(63, &[288]);     // 63 => [288]
            map.insert(144, &[2080]);     // 144 => [2080]
            map.insert(189, &[4128]);     // 189 => [4128]
            map.insert(116, &[32800]);     // 116 => [32800]
            map.insert(120, &[65568]);     // 120 => [65568]
            map.insert(123, &[131104]);     // 123 => [131104]
            map.insert(249, &[262176]);     // 249 => [262176]
            map.insert(160, &[2112]);     // 160 => [2112]
            map.insert(141, &[4160]);     // 141 => [4160]
            map.insert(201, &[262208]);     // 201 => [262208]
            map.insert(197, &[262272]);     // 197 => [262272]
            map.insert(198, &[262400]);     // 198 => [262400]
            map.insert(192, &[2560]);     // 192 => [2560]
            map.insert(204, &[3072]);     // 204 => [3072]
            map.insert(207, &[10240]);     // 207 => [10240]
            map.insert(212, &[18432]);     // 212 => [18432]
            map.insert(228, &[34816]);     // 228 => [34816]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(19, &[35]);     // 19 => [35]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(67, &[515]);     // 67 => [515]
            map.insert(131, &[2051]);     // 131 => [2051]
            map.insert(174, &[4099]);     // 174 => [4099]
            map.insert(87, &[16387]);     // 87 => [16387]
            map.insert(103, &[32771]);     // 103 => [32771]
            map.insert(234, &[262147]);     // 234 => [262147]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(14, &[21]);     // 14 => [21]
            map.insert(21, &[37]);     // 21 => [37]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(41, &[133]);     // 41 => [133]
            map.insert(42, &[261]);     // 42 => [261]
            map.insert(69, &[517]);     // 69 => [517]
            map.insert(73, &[1029]);     // 73 => [1029]
            map.insert(133, &[2053]);     // 133 => [2053]
            map.insert(168, &[4101]);     // 168 => [4101]
            map.insert(74, &[8197]);     // 74 => [8197]
            map.insert(81, &[16389]);     // 81 => [16389]
            map.insert(97, &[32773]);     // 97 => [32773]
            map.insert(109, &[65541]);     // 109 => [65541]
            map.insert(110, &[131077]);     // 110 => [131077]
            map.insert(236, &[262149]);     // 236 => [262149]
            map.insert(25, &[41]);     // 25 => [41]
            map.insert(38, &[265]);     // 38 => [265]
            map.insert(137, &[2057]);     // 137 => [2057]
            map.insert(164, &[4105]);     // 164 => [4105]
            map.insert(70, &[8201]);     // 70 => [8201]
            map.insert(93, &[16393]);     // 93 => [16393]
            map.insert(98, &[131081]);     // 98 => [131081]
            map.insert(224, &[262153]);     // 224 => [262153]
            map.insert(26, &[49]);     // 26 => [49]
            map.insert(138, &[2065]);     // 138 => [2065]
            map.insert(167, &[4113]);     // 167 => [4113]
            map.insert(94, &[16401]);     // 94 => [16401]
            map.insert(227, &[262161]);     // 227 => [262161]
            map.insert(49, &[97]);     // 49 => [97]
            map.insert(61, &[161]);     // 61 => [161]
            map.insert(62, &[289]);     // 62 => [289]
            map.insert(145, &[2081]);     // 145 => [2081]
            map.insert(188, &[4129]);     // 188 => [4129]
            map.insert(117, &[32801]);     // 117 => [32801]
            map.insert(121, &[65569]);     // 121 => [65569]
            map.insert(122, &[131105]);     // 122 => [131105]
            map.insert(248, &[262177]);     // 248 => [262177]
            map.insert(161, &[2113]);     // 161 => [2113]
            map.insert(140, &[4161]);     // 140 => [4161]
            map.insert(200, &[262209]);     // 200 => [262209]
            map.insert(196, &[262273]);     // 196 => [262273]
            map.insert(199, &[262401]);     // 199 => [262401]
            map.insert(193, &[2561]);     // 193 => [2561]
            map.insert(205, &[3073]);     // 205 => [3073]
            map.insert(206, &[10241]);     // 206 => [10241]
            map.insert(213, &[18433]);     // 213 => [18433]
            map.insert(229, &[34817]);     // 229 => [34817]
            map.insert(22, &[38]);     // 22 => [38]
            map.insert(134, &[2054]);     // 134 => [2054]
            map.insert(171, &[4102]);     // 171 => [4102]
            map.insert(82, &[16390]);     // 82 => [16390]
            map.insert(239, &[262150]);     // 239 => [262150]
            map.insert(50, &[98]);     // 50 => [98]
            map.insert(146, &[2082]);     // 146 => [2082]
            map.insert(191, &[4130]);     // 191 => [4130]
            map.insert(118, &[32802]);     // 118 => [32802]
            map.insert(251, &[262178]);     // 251 => [262178]
            map.insert(162, &[2114]);     // 162 => [2114]
            map.insert(143, &[4162]);     // 143 => [4162]
            map.insert(203, &[262210]);     // 203 => [262210]
            map.insert(194, &[2562]);     // 194 => [2562]
            map.insert(214, &[18434]);     // 214 => [18434]
            map.insert(230, &[34818]);     // 230 => [34818]
            map.insert(28, &[44]);     // 28 => [44]
            map.insert(88, &[16396]);     // 88 => [16396]
            map.insert(31, &[52]);     // 31 => [52]
            map.insert(91, &[16404]);     // 91 => [16404]
            map.insert(52, &[100]);     // 52 => [100]
            map.insert(56, &[164]);     // 56 => [164]
            map.insert(59, &[292]);     // 59 => [292]
            map.insert(148, &[2084]);     // 148 => [2084]
            map.insert(185, &[4132]);     // 185 => [4132]
            map.insert(112, &[32804]);     // 112 => [32804]
            map.insert(124, &[65572]);     // 124 => [65572]
            map.insert(127, &[131108]);     // 127 => [131108]
            map.insert(253, &[262180]);     // 253 => [262180]
            map.insert(208, &[18436]);     // 208 => [18436]
            map.insert(55, &[296]);     // 55 => [296]
            map.insert(152, &[2088]);     // 152 => [2088]
            map.insert(181, &[4136]);     // 181 => [4136]
            map.insert(115, &[131112]);     // 115 => [131112]
            map.insert(241, &[262184]);     // 241 => [262184]
            map.insert(220, &[18440]);     // 220 => [18440]
            map.insert(155, &[2096]);     // 155 => [2096]
            map.insert(182, &[4144]);     // 182 => [4144]
            map.insert(242, &[262192]);     // 242 => [262192]
            map.insert(223, &[18448]);     // 223 => [18448]
            map.insert(176, &[2144]);     // 176 => [2144]
            map.insert(157, &[4192]);     // 157 => [4192]
            map.insert(217, &[262240]);     // 217 => [262240]
            map.insert(244, &[34848]);     // 244 => [34848]
            map.insert(23, &[39]);     // 23 => [39]
            map.insert(135, &[2055]);     // 135 => [2055]
            map.insert(170, &[4103]);     // 170 => [4103]
            map.insert(83, &[16391]);     // 83 => [16391]
            map.insert(238, &[262151]);     // 238 => [262151]
            map.insert(51, &[99]);     // 51 => [99]
            map.insert(147, &[2083]);     // 147 => [2083]
            map.insert(190, &[4131]);     // 190 => [4131]
            map.insert(119, &[32803]);     // 119 => [32803]
            map.insert(250, &[262179]);     // 250 => [262179]
            map.insert(163, &[2115]);     // 163 => [2115]
            map.insert(142, &[4163]);     // 142 => [4163]
            map.insert(202, &[262211]);     // 202 => [262211]
            map.insert(195, &[2563]);     // 195 => [2563]
            map.insert(215, &[18435]);     // 215 => [18435]
            map.insert(231, &[34819]);     // 231 => [34819]
            map.insert(29, &[45]);     // 29 => [45]
            map.insert(89, &[16397]);     // 89 => [16397]
            map.insert(30, &[53]);     // 30 => [53]
            map.insert(90, &[16405]);     // 90 => [16405]
            map.insert(53, &[101]);     // 53 => [101]
            map.insert(57, &[165]);     // 57 => [165]
            map.insert(58, &[293]);     // 58 => [293]
            map.insert(149, &[2085]);     // 149 => [2085]
            map.insert(184, &[4133]);     // 184 => [4133]
            map.insert(113, &[32805]);     // 113 => [32805]
            map.insert(125, &[65573]);     // 125 => [65573]
            map.insert(126, &[131109]);     // 126 => [131109]
            map.insert(252, &[262181]);     // 252 => [262181]
            map.insert(209, &[18437]);     // 209 => [18437]
            map.insert(54, &[297]);     // 54 => [297]
            map.insert(153, &[2089]);     // 153 => [2089]
            map.insert(180, &[4137]);     // 180 => [4137]
            map.insert(114, &[131113]);     // 114 => [131113]
            map.insert(240, &[262185]);     // 240 => [262185]
            map.insert(221, &[18441]);     // 221 => [18441]
            map.insert(154, &[2097]);     // 154 => [2097]
            map.insert(183, &[4145]);     // 183 => [4145]
            map.insert(243, &[262193]);     // 243 => [262193]
            map.insert(222, &[18449]);     // 222 => [18449]
            map.insert(177, &[2145]);     // 177 => [2145]
            map.insert(156, &[4193]);     // 156 => [4193]
            map.insert(216, &[262241]);     // 216 => [262241]
            map.insert(245, &[34849]);     // 245 => [34849]
            map.insert(150, &[2086]);     // 150 => [2086]
            map.insert(187, &[4134]);     // 187 => [4134]
            map.insert(255, &[262182]);     // 255 => [262182]
            map.insert(210, &[18438]);     // 210 => [18438]
            map.insert(178, &[2146]);     // 178 => [2146]
            map.insert(159, &[4194]);     // 159 => [4194]
            map.insert(219, &[262242]);     // 219 => [262242]
            map.insert(246, &[34850]);     // 246 => [34850]
            map.insert(151, &[2087]);     // 151 => [2087]
            map.insert(186, &[4135]);     // 186 => [4135]
            map.insert(254, &[262183]);     // 254 => [262183]
            map.insert(211, &[18439]);     // 211 => [18439]
            map.insert(179, &[2147]);     // 179 => [2147]
            map.insert(158, &[4195]);     // 158 => [4195]
            map.insert(218, &[262243]);     // 218 => [262243]
            map.insert(247, &[34851]);     // 247 => [34851]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode19_11 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode19_11 {
    fn name(&self) -> String {
        "[19, 11] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        19
    }

    fn dimension(&self) -> usize {
        11
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
        let mut error = BinVector::with_capacity(19);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 19 / 64 + if 19 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(19) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(11);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[19 / 64] & !((1 << 19) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode19_11.generator_matrix();
        assert_eq!(code.ncols(), 19);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode19_11;
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
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, true, true, true, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, true, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, false, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, true, true, true, false, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, false, false, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, true, true, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, false, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, true, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, true, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, false, false, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, false, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, false, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, true, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, false, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, true, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, false, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, false, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, true, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, false, true, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, true, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, true, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, true, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, false, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, false, false, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, true, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false, true, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
