use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[22, 14]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode22_14;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2179073 ],
                &[ 2244610 ],
                &[ 2375684 ],
                &[ 2637832 ],
                &[ 3162128 ],
                &[ 3506208 ],
                &[ 3768384 ],
                &[ 2195584 ],
                &[ 2261248 ],
                &[ 2392576 ],
                &[ 2655232 ],
                &[ 3180544 ],
                &[ 1380352 ],
                &[ 1646592 ],
                
            ], 22));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4099841 ],
                &[ 131330 ],
                &[ 3911108 ],
                &[ 533576 ],
                &[ 3114896 ],
                &[ 36832 ],
                &[ 4141056 ],
                &[ 2146304 ],
                
            ], 22));
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
            map.insert(44, &[64]);     // 44 => [64]
            map.insert(52, &[128]);     // 52 => [128]
            map.insert(55, &[256]);     // 55 => [256]
            map.insert(49, &[512]);     // 49 => [512]
            map.insert(61, &[1024]);     // 61 => [1024]
            map.insert(37, &[2048]);     // 37 => [2048]
            map.insert(64, &[4096]);     // 64 => [4096]
            map.insert(76, &[8192]);     // 76 => [8192]
            map.insert(128, &[16384]);     // 128 => [16384]
            map.insert(181, &[32768]);     // 181 => [32768]
            map.insert(84, &[65536]);     // 84 => [65536]
            map.insert(87, &[131072]);     // 87 => [131072]
            map.insert(81, &[262144]);     // 81 => [262144]
            map.insert(93, &[524288]);     // 93 => [524288]
            map.insert(69, &[1048576]);     // 69 => [1048576]
            map.insert(213, &[2097152]);     // 213 => [2097152]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(45, &[65]);     // 45 => [65]
            map.insert(53, &[129]);     // 53 => [129]
            map.insert(54, &[257]);     // 54 => [257]
            map.insert(48, &[513]);     // 48 => [513]
            map.insert(60, &[1025]);     // 60 => [1025]
            map.insert(36, &[2049]);     // 36 => [2049]
            map.insert(65, &[4097]);     // 65 => [4097]
            map.insert(77, &[8193]);     // 77 => [8193]
            map.insert(129, &[16385]);     // 129 => [16385]
            map.insert(180, &[32769]);     // 180 => [32769]
            map.insert(85, &[65537]);     // 85 => [65537]
            map.insert(86, &[131073]);     // 86 => [131073]
            map.insert(80, &[262145]);     // 80 => [262145]
            map.insert(92, &[524289]);     // 92 => [524289]
            map.insert(68, &[1048577]);     // 68 => [1048577]
            map.insert(212, &[2097153]);     // 212 => [2097153]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(46, &[66]);     // 46 => [66]
            map.insert(51, &[514]);     // 51 => [514]
            map.insert(63, &[1026]);     // 63 => [1026]
            map.insert(39, &[2050]);     // 39 => [2050]
            map.insert(66, &[4098]);     // 66 => [4098]
            map.insert(78, &[8194]);     // 78 => [8194]
            map.insert(130, &[16386]);     // 130 => [16386]
            map.insert(183, &[32770]);     // 183 => [32770]
            map.insert(83, &[262146]);     // 83 => [262146]
            map.insert(95, &[524290]);     // 95 => [524290]
            map.insert(71, &[1048578]);     // 71 => [1048578]
            map.insert(215, &[2097154]);     // 215 => [2097154]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(40, &[68]);     // 40 => [68]
            map.insert(57, &[1028]);     // 57 => [1028]
            map.insert(72, &[8196]);     // 72 => [8196]
            map.insert(132, &[16388]);     // 132 => [16388]
            map.insert(177, &[32772]);     // 177 => [32772]
            map.insert(89, &[524292]);     // 89 => [524292]
            map.insert(209, &[2097156]);     // 209 => [2097156]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(136, &[16392]);     // 136 => [16392]
            map.insert(189, &[32776]);     // 189 => [32776]
            map.insert(221, &[2097160]);     // 221 => [2097160]
            map.insert(144, &[16400]);     // 144 => [16400]
            map.insert(165, &[32784]);     // 165 => [32784]
            map.insert(197, &[2097168]);     // 197 => [2097168]
            map.insert(23, &[288]);     // 23 => [288]
            map.insert(29, &[1056]);     // 29 => [1056]
            map.insert(96, &[4128]);     // 96 => [4128]
            map.insert(108, &[8224]);     // 108 => [8224]
            map.insert(160, &[16416]);     // 160 => [16416]
            map.insert(149, &[32800]);     // 149 => [32800]
            map.insert(116, &[65568]);     // 116 => [65568]
            map.insert(119, &[131104]);     // 119 => [131104]
            map.insert(113, &[262176]);     // 113 => [262176]
            map.insert(125, &[524320]);     // 125 => [524320]
            map.insert(101, &[1048608]);     // 101 => [1048608]
            map.insert(245, &[2097184]);     // 245 => [2097184]
            map.insert(27, &[320]);     // 27 => [320]
            map.insert(172, &[16448]);     // 172 => [16448]
            map.insert(153, &[32832]);     // 153 => [32832]
            map.insert(120, &[65600]);     // 120 => [65600]
            map.insert(123, &[131136]);     // 123 => [131136]
            map.insert(105, &[1048640]);     // 105 => [1048640]
            map.insert(249, &[2097216]);     // 249 => [2097216]
            map.insert(99, &[131200]);     // 99 => [131200]
            map.insert(225, &[2097280]);     // 225 => [2097280]
            map.insert(102, &[262400]);     // 102 => [262400]
            map.insert(106, &[524544]);     // 106 => [524544]
            map.insert(114, &[1048832]);     // 114 => [1048832]
            map.insert(226, &[2097408]);     // 226 => [2097408]
            map.insert(228, &[2097664]);     // 228 => [2097664]
            map.insert(232, &[2098176]);     // 232 => [2098176]
            map.insert(240, &[2099200]);     // 240 => [2099200]
            map.insert(192, &[20480]);     // 192 => [20480]
            map.insert(204, &[24576]);     // 204 => [24576]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(47, &[67]);     // 47 => [67]
            map.insert(50, &[515]);     // 50 => [515]
            map.insert(62, &[1027]);     // 62 => [1027]
            map.insert(38, &[2051]);     // 38 => [2051]
            map.insert(67, &[4099]);     // 67 => [4099]
            map.insert(79, &[8195]);     // 79 => [8195]
            map.insert(131, &[16387]);     // 131 => [16387]
            map.insert(182, &[32771]);     // 182 => [32771]
            map.insert(82, &[262147]);     // 82 => [262147]
            map.insert(94, &[524291]);     // 94 => [524291]
            map.insert(70, &[1048579]);     // 70 => [1048579]
            map.insert(214, &[2097155]);     // 214 => [2097155]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(41, &[69]);     // 41 => [69]
            map.insert(56, &[1029]);     // 56 => [1029]
            map.insert(73, &[8197]);     // 73 => [8197]
            map.insert(133, &[16389]);     // 133 => [16389]
            map.insert(176, &[32773]);     // 176 => [32773]
            map.insert(88, &[524293]);     // 88 => [524293]
            map.insert(208, &[2097157]);     // 208 => [2097157]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(137, &[16393]);     // 137 => [16393]
            map.insert(188, &[32777]);     // 188 => [32777]
            map.insert(220, &[2097161]);     // 220 => [2097161]
            map.insert(145, &[16401]);     // 145 => [16401]
            map.insert(164, &[32785]);     // 164 => [32785]
            map.insert(196, &[2097169]);     // 196 => [2097169]
            map.insert(22, &[289]);     // 22 => [289]
            map.insert(28, &[1057]);     // 28 => [1057]
            map.insert(97, &[4129]);     // 97 => [4129]
            map.insert(109, &[8225]);     // 109 => [8225]
            map.insert(161, &[16417]);     // 161 => [16417]
            map.insert(148, &[32801]);     // 148 => [32801]
            map.insert(117, &[65569]);     // 117 => [65569]
            map.insert(118, &[131105]);     // 118 => [131105]
            map.insert(112, &[262177]);     // 112 => [262177]
            map.insert(124, &[524321]);     // 124 => [524321]
            map.insert(100, &[1048609]);     // 100 => [1048609]
            map.insert(244, &[2097185]);     // 244 => [2097185]
            map.insert(26, &[321]);     // 26 => [321]
            map.insert(173, &[16449]);     // 173 => [16449]
            map.insert(152, &[32833]);     // 152 => [32833]
            map.insert(121, &[65601]);     // 121 => [65601]
            map.insert(122, &[131137]);     // 122 => [131137]
            map.insert(104, &[1048641]);     // 104 => [1048641]
            map.insert(248, &[2097217]);     // 248 => [2097217]
            map.insert(98, &[131201]);     // 98 => [131201]
            map.insert(224, &[2097281]);     // 224 => [2097281]
            map.insert(103, &[262401]);     // 103 => [262401]
            map.insert(107, &[524545]);     // 107 => [524545]
            map.insert(115, &[1048833]);     // 115 => [1048833]
            map.insert(227, &[2097409]);     // 227 => [2097409]
            map.insert(229, &[2097665]);     // 229 => [2097665]
            map.insert(233, &[2098177]);     // 233 => [2098177]
            map.insert(241, &[2099201]);     // 241 => [2099201]
            map.insert(193, &[20481]);     // 193 => [20481]
            map.insert(205, &[24577]);     // 205 => [24577]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(42, &[70]);     // 42 => [70]
            map.insert(59, &[1030]);     // 59 => [1030]
            map.insert(74, &[8198]);     // 74 => [8198]
            map.insert(134, &[16390]);     // 134 => [16390]
            map.insert(179, &[32774]);     // 179 => [32774]
            map.insert(91, &[524294]);     // 91 => [524294]
            map.insert(211, &[2097158]);     // 211 => [2097158]
            map.insert(138, &[16394]);     // 138 => [16394]
            map.insert(191, &[32778]);     // 191 => [32778]
            map.insert(223, &[2097162]);     // 223 => [2097162]
            map.insert(146, &[16402]);     // 146 => [16402]
            map.insert(167, &[32786]);     // 167 => [32786]
            map.insert(199, &[2097170]);     // 199 => [2097170]
            map.insert(31, &[1058]);     // 31 => [1058]
            map.insert(110, &[8226]);     // 110 => [8226]
            map.insert(162, &[16418]);     // 162 => [16418]
            map.insert(151, &[32802]);     // 151 => [32802]
            map.insert(127, &[524322]);     // 127 => [524322]
            map.insert(247, &[2097186]);     // 247 => [2097186]
            map.insert(174, &[16450]);     // 174 => [16450]
            map.insert(155, &[32834]);     // 155 => [32834]
            map.insert(251, &[2097218]);     // 251 => [2097218]
            map.insert(230, &[2097666]);     // 230 => [2097666]
            map.insert(234, &[2098178]);     // 234 => [2098178]
            map.insert(242, &[2099202]);     // 242 => [2099202]
            map.insert(194, &[20482]);     // 194 => [20482]
            map.insert(206, &[24578]);     // 206 => [24578]
            map.insert(140, &[16396]);     // 140 => [16396]
            map.insert(185, &[32780]);     // 185 => [32780]
            map.insert(217, &[2097164]);     // 217 => [2097164]
            map.insert(168, &[16452]);     // 168 => [16452]
            map.insert(157, &[32836]);     // 157 => [32836]
            map.insert(253, &[2097220]);     // 253 => [2097220]
            map.insert(236, &[2098180]);     // 236 => [2098180]
            map.insert(200, &[24580]);     // 200 => [24580]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(43, &[71]);     // 43 => [71]
            map.insert(58, &[1031]);     // 58 => [1031]
            map.insert(75, &[8199]);     // 75 => [8199]
            map.insert(135, &[16391]);     // 135 => [16391]
            map.insert(178, &[32775]);     // 178 => [32775]
            map.insert(90, &[524295]);     // 90 => [524295]
            map.insert(210, &[2097159]);     // 210 => [2097159]
            map.insert(139, &[16395]);     // 139 => [16395]
            map.insert(190, &[32779]);     // 190 => [32779]
            map.insert(222, &[2097163]);     // 222 => [2097163]
            map.insert(147, &[16403]);     // 147 => [16403]
            map.insert(166, &[32787]);     // 166 => [32787]
            map.insert(198, &[2097171]);     // 198 => [2097171]
            map.insert(30, &[1059]);     // 30 => [1059]
            map.insert(111, &[8227]);     // 111 => [8227]
            map.insert(163, &[16419]);     // 163 => [16419]
            map.insert(150, &[32803]);     // 150 => [32803]
            map.insert(126, &[524323]);     // 126 => [524323]
            map.insert(246, &[2097187]);     // 246 => [2097187]
            map.insert(175, &[16451]);     // 175 => [16451]
            map.insert(154, &[32835]);     // 154 => [32835]
            map.insert(250, &[2097219]);     // 250 => [2097219]
            map.insert(231, &[2097667]);     // 231 => [2097667]
            map.insert(235, &[2098179]);     // 235 => [2098179]
            map.insert(243, &[2099203]);     // 243 => [2099203]
            map.insert(195, &[20483]);     // 195 => [20483]
            map.insert(207, &[24579]);     // 207 => [24579]
            map.insert(141, &[16397]);     // 141 => [16397]
            map.insert(184, &[32781]);     // 184 => [32781]
            map.insert(216, &[2097165]);     // 216 => [2097165]
            map.insert(169, &[16453]);     // 169 => [16453]
            map.insert(156, &[32837]);     // 156 => [32837]
            map.insert(252, &[2097221]);     // 252 => [2097221]
            map.insert(237, &[2098181]);     // 237 => [2098181]
            map.insert(201, &[24581]);     // 201 => [24581]
            map.insert(142, &[16398]);     // 142 => [16398]
            map.insert(187, &[32782]);     // 187 => [32782]
            map.insert(219, &[2097166]);     // 219 => [2097166]
            map.insert(170, &[16454]);     // 170 => [16454]
            map.insert(159, &[32838]);     // 159 => [32838]
            map.insert(255, &[2097222]);     // 255 => [2097222]
            map.insert(238, &[2098182]);     // 238 => [2098182]
            map.insert(202, &[24582]);     // 202 => [24582]
            map.insert(143, &[16399]);     // 143 => [16399]
            map.insert(186, &[32783]);     // 186 => [32783]
            map.insert(218, &[2097167]);     // 218 => [2097167]
            map.insert(171, &[16455]);     // 171 => [16455]
            map.insert(158, &[32839]);     // 158 => [32839]
            map.insert(254, &[2097223]);     // 254 => [2097223]
            map.insert(239, &[2098183]);     // 239 => [2098183]
            map.insert(203, &[24583]);     // 203 => [24583]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode22_14 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode22_14 {
    fn name(&self) -> String {
        "[22, 14] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        22
    }

    fn dimension(&self) -> usize {
        14
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
        let mut error = BinVector::with_capacity(22);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 22 / 64 + if 22 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(22) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(14);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[22 / 64] & !((1 << 22) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode22_14.generator_matrix();
        assert_eq!(code.ncols(), 22);
        assert_eq!(code.nrows(), 14);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode22_14;
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
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, true, false, true, true, false, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, true, false, true, true, false, true, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, true, false, true, true, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, false, false, false, true, true, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, true, false, true, true, false, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, false, false, false, true, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, false, false, true, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, true, true, false, false, false, false, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, false, false, false, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, true, false, true, false, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, true, false, true, false, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, false, true, false, false, false, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, true, true, false, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, true, false, false, true, false, true, false, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, true, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, true, true, true, true, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, true, false, false, false, true, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, true, true, false, true, false, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, false, true, true, true, true, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, false, false, true, true, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, false, true, true, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
