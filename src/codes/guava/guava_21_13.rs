use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[21, 13]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode21_13;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1122305 ],
                &[ 1187842 ],
                &[ 1318916 ],
                &[ 1581064 ],
                &[ 1753104 ],
                &[ 1884192 ],
                &[ 1097792 ],
                &[ 1130624 ],
                &[ 1196288 ],
                &[ 1327616 ],
                &[ 1590272 ],
                &[ 690176 ],
                &[ 823296 ],
                
            ], 21));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1984257 ],
                &[ 168290 ],
                &[ 266788 ],
                &[ 558152 ],
                &[ 2031728 ],
                &[ 2049920 ],
                &[ 2070528 ],
                &[ 1073152 ],
                
            ], 21));
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
            map.insert(22, &[32]);     // 22 => [32]
            map.insert(26, &[64]);     // 26 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(41, &[1024]);     // 41 => [1024]
            map.insert(64, &[2048]);     // 64 => [2048]
            map.insert(70, &[4096]);     // 70 => [4096]
            map.insert(128, &[8192]);     // 128 => [8192]
            map.insert(161, &[16384]);     // 161 => [16384]
            map.insert(74, &[32768]);     // 74 => [32768]
            map.insert(112, &[65536]);     // 112 => [65536]
            map.insert(115, &[131072]);     // 115 => [131072]
            map.insert(117, &[262144]);     // 117 => [262144]
            map.insert(121, &[524288]);     // 121 => [524288]
            map.insert(241, &[1048576]);     // 241 => [1048576]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(23, &[33]);     // 23 => [33]
            map.insert(27, &[65]);     // 27 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(40, &[1025]);     // 40 => [1025]
            map.insert(65, &[2049]);     // 65 => [2049]
            map.insert(71, &[4097]);     // 71 => [4097]
            map.insert(129, &[8193]);     // 129 => [8193]
            map.insert(160, &[16385]);     // 160 => [16385]
            map.insert(75, &[32769]);     // 75 => [32769]
            map.insert(113, &[65537]);     // 113 => [65537]
            map.insert(114, &[131073]);     // 114 => [131073]
            map.insert(116, &[262145]);     // 116 => [262145]
            map.insert(120, &[524289]);     // 120 => [524289]
            map.insert(240, &[1048577]);     // 240 => [1048577]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(20, &[34]);     // 20 => [34]
            map.insert(24, &[66]);     // 24 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(43, &[1026]);     // 43 => [1026]
            map.insert(66, &[2050]);     // 66 => [2050]
            map.insert(68, &[4098]);     // 68 => [4098]
            map.insert(130, &[8194]);     // 130 => [8194]
            map.insert(163, &[16386]);     // 163 => [16386]
            map.insert(72, &[32770]);     // 72 => [32770]
            map.insert(119, &[262146]);     // 119 => [262146]
            map.insert(123, &[524290]);     // 123 => [524290]
            map.insert(243, &[1048578]);     // 243 => [1048578]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(30, &[68]);     // 30 => [68]
            map.insert(45, &[1028]);     // 45 => [1028]
            map.insert(132, &[8196]);     // 132 => [8196]
            map.insert(165, &[16388]);     // 165 => [16388]
            map.insert(78, &[32772]);     // 78 => [32772]
            map.insert(125, &[524292]);     // 125 => [524292]
            map.insert(245, &[1048580]);     // 245 => [1048580]
            map.insert(136, &[8200]);     // 136 => [8200]
            map.insert(169, &[16392]);     // 169 => [16392]
            map.insert(249, &[1048584]);     // 249 => [1048584]
            map.insert(48, &[144]);     // 48 => [144]
            map.insert(51, &[272]);     // 51 => [272]
            map.insert(53, &[528]);     // 53 => [528]
            map.insert(57, &[1040]);     // 57 => [1040]
            map.insert(80, &[2064]);     // 80 => [2064]
            map.insert(86, &[4112]);     // 86 => [4112]
            map.insert(144, &[8208]);     // 144 => [8208]
            map.insert(177, &[16400]);     // 177 => [16400]
            map.insert(90, &[32784]);     // 90 => [32784]
            map.insert(96, &[65552]);     // 96 => [65552]
            map.insert(99, &[131088]);     // 99 => [131088]
            map.insert(101, &[262160]);     // 101 => [262160]
            map.insert(105, &[524304]);     // 105 => [524304]
            map.insert(225, &[1048592]);     // 225 => [1048592]
            map.insert(54, &[160]);     // 54 => [160]
            map.insert(63, &[1056]);     // 63 => [1056]
            map.insert(150, &[8224]);     // 150 => [8224]
            map.insert(183, &[16416]);     // 183 => [16416]
            map.insert(92, &[32800]);     // 92 => [32800]
            map.insert(102, &[65568]);     // 102 => [65568]
            map.insert(111, &[524320]);     // 111 => [524320]
            map.insert(231, &[1048608]);     // 231 => [1048608]
            map.insert(58, &[192]);     // 58 => [192]
            map.insert(154, &[8256]);     // 154 => [8256]
            map.insert(187, &[16448]);     // 187 => [16448]
            map.insert(106, &[65600]);     // 106 => [65600]
            map.insert(235, &[1048640]);     // 235 => [1048640]
            map.insert(83, &[131200]);     // 83 => [131200]
            map.insert(85, &[262272]);     // 85 => [262272]
            map.insert(89, &[524416]);     // 89 => [524416]
            map.insert(209, &[1048704]);     // 209 => [1048704]
            map.insert(210, &[1048832]);     // 210 => [1048832]
            map.insert(212, &[1049088]);     // 212 => [1049088]
            map.insert(216, &[1049600]);     // 216 => [1049600]
            map.insert(192, &[10240]);     // 192 => [10240]
            map.insert(198, &[12288]);     // 198 => [12288]
            map.insert(202, &[40960]);     // 202 => [40960]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(21, &[35]);     // 21 => [35]
            map.insert(25, &[67]);     // 25 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(42, &[1027]);     // 42 => [1027]
            map.insert(67, &[2051]);     // 67 => [2051]
            map.insert(69, &[4099]);     // 69 => [4099]
            map.insert(131, &[8195]);     // 131 => [8195]
            map.insert(162, &[16387]);     // 162 => [16387]
            map.insert(73, &[32771]);     // 73 => [32771]
            map.insert(118, &[262147]);     // 118 => [262147]
            map.insert(122, &[524291]);     // 122 => [524291]
            map.insert(242, &[1048579]);     // 242 => [1048579]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(31, &[69]);     // 31 => [69]
            map.insert(44, &[1029]);     // 44 => [1029]
            map.insert(133, &[8197]);     // 133 => [8197]
            map.insert(164, &[16389]);     // 164 => [16389]
            map.insert(79, &[32773]);     // 79 => [32773]
            map.insert(124, &[524293]);     // 124 => [524293]
            map.insert(244, &[1048581]);     // 244 => [1048581]
            map.insert(137, &[8201]);     // 137 => [8201]
            map.insert(168, &[16393]);     // 168 => [16393]
            map.insert(248, &[1048585]);     // 248 => [1048585]
            map.insert(49, &[145]);     // 49 => [145]
            map.insert(50, &[273]);     // 50 => [273]
            map.insert(52, &[529]);     // 52 => [529]
            map.insert(56, &[1041]);     // 56 => [1041]
            map.insert(81, &[2065]);     // 81 => [2065]
            map.insert(87, &[4113]);     // 87 => [4113]
            map.insert(145, &[8209]);     // 145 => [8209]
            map.insert(176, &[16401]);     // 176 => [16401]
            map.insert(91, &[32785]);     // 91 => [32785]
            map.insert(97, &[65553]);     // 97 => [65553]
            map.insert(98, &[131089]);     // 98 => [131089]
            map.insert(100, &[262161]);     // 100 => [262161]
            map.insert(104, &[524305]);     // 104 => [524305]
            map.insert(224, &[1048593]);     // 224 => [1048593]
            map.insert(55, &[161]);     // 55 => [161]
            map.insert(62, &[1057]);     // 62 => [1057]
            map.insert(151, &[8225]);     // 151 => [8225]
            map.insert(182, &[16417]);     // 182 => [16417]
            map.insert(93, &[32801]);     // 93 => [32801]
            map.insert(103, &[65569]);     // 103 => [65569]
            map.insert(110, &[524321]);     // 110 => [524321]
            map.insert(230, &[1048609]);     // 230 => [1048609]
            map.insert(59, &[193]);     // 59 => [193]
            map.insert(155, &[8257]);     // 155 => [8257]
            map.insert(186, &[16449]);     // 186 => [16449]
            map.insert(107, &[65601]);     // 107 => [65601]
            map.insert(234, &[1048641]);     // 234 => [1048641]
            map.insert(82, &[131201]);     // 82 => [131201]
            map.insert(84, &[262273]);     // 84 => [262273]
            map.insert(88, &[524417]);     // 88 => [524417]
            map.insert(208, &[1048705]);     // 208 => [1048705]
            map.insert(211, &[1048833]);     // 211 => [1048833]
            map.insert(213, &[1049089]);     // 213 => [1049089]
            map.insert(217, &[1049601]);     // 217 => [1049601]
            map.insert(193, &[10241]);     // 193 => [10241]
            map.insert(199, &[12289]);     // 199 => [12289]
            map.insert(203, &[40961]);     // 203 => [40961]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(28, &[70]);     // 28 => [70]
            map.insert(47, &[1030]);     // 47 => [1030]
            map.insert(134, &[8198]);     // 134 => [8198]
            map.insert(167, &[16390]);     // 167 => [16390]
            map.insert(76, &[32774]);     // 76 => [32774]
            map.insert(127, &[524294]);     // 127 => [524294]
            map.insert(247, &[1048582]);     // 247 => [1048582]
            map.insert(138, &[8202]);     // 138 => [8202]
            map.insert(171, &[16394]);     // 171 => [16394]
            map.insert(251, &[1048586]);     // 251 => [1048586]
            map.insert(146, &[8210]);     // 146 => [8210]
            map.insert(179, &[16402]);     // 179 => [16402]
            map.insert(227, &[1048594]);     // 227 => [1048594]
            map.insert(61, &[1058]);     // 61 => [1058]
            map.insert(148, &[8226]);     // 148 => [8226]
            map.insert(181, &[16418]);     // 181 => [16418]
            map.insert(94, &[32802]);     // 94 => [32802]
            map.insert(109, &[524322]);     // 109 => [524322]
            map.insert(229, &[1048610]);     // 229 => [1048610]
            map.insert(152, &[8258]);     // 152 => [8258]
            map.insert(185, &[16450]);     // 185 => [16450]
            map.insert(233, &[1048642]);     // 233 => [1048642]
            map.insert(214, &[1049090]);     // 214 => [1049090]
            map.insert(218, &[1049602]);     // 218 => [1049602]
            map.insert(194, &[10242]);     // 194 => [10242]
            map.insert(196, &[12290]);     // 196 => [12290]
            map.insert(200, &[40962]);     // 200 => [40962]
            map.insert(140, &[8204]);     // 140 => [8204]
            map.insert(173, &[16396]);     // 173 => [16396]
            map.insert(253, &[1048588]);     // 253 => [1048588]
            map.insert(158, &[8260]);     // 158 => [8260]
            map.insert(191, &[16452]);     // 191 => [16452]
            map.insert(239, &[1048644]);     // 239 => [1048644]
            map.insert(220, &[1049604]);     // 220 => [1049604]
            map.insert(206, &[40964]);     // 206 => [40964]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(29, &[71]);     // 29 => [71]
            map.insert(46, &[1031]);     // 46 => [1031]
            map.insert(135, &[8199]);     // 135 => [8199]
            map.insert(166, &[16391]);     // 166 => [16391]
            map.insert(77, &[32775]);     // 77 => [32775]
            map.insert(126, &[524295]);     // 126 => [524295]
            map.insert(246, &[1048583]);     // 246 => [1048583]
            map.insert(139, &[8203]);     // 139 => [8203]
            map.insert(170, &[16395]);     // 170 => [16395]
            map.insert(250, &[1048587]);     // 250 => [1048587]
            map.insert(147, &[8211]);     // 147 => [8211]
            map.insert(178, &[16403]);     // 178 => [16403]
            map.insert(226, &[1048595]);     // 226 => [1048595]
            map.insert(60, &[1059]);     // 60 => [1059]
            map.insert(149, &[8227]);     // 149 => [8227]
            map.insert(180, &[16419]);     // 180 => [16419]
            map.insert(95, &[32803]);     // 95 => [32803]
            map.insert(108, &[524323]);     // 108 => [524323]
            map.insert(228, &[1048611]);     // 228 => [1048611]
            map.insert(153, &[8259]);     // 153 => [8259]
            map.insert(184, &[16451]);     // 184 => [16451]
            map.insert(232, &[1048643]);     // 232 => [1048643]
            map.insert(215, &[1049091]);     // 215 => [1049091]
            map.insert(219, &[1049603]);     // 219 => [1049603]
            map.insert(195, &[10243]);     // 195 => [10243]
            map.insert(197, &[12291]);     // 197 => [12291]
            map.insert(201, &[40963]);     // 201 => [40963]
            map.insert(141, &[8205]);     // 141 => [8205]
            map.insert(172, &[16397]);     // 172 => [16397]
            map.insert(252, &[1048589]);     // 252 => [1048589]
            map.insert(159, &[8261]);     // 159 => [8261]
            map.insert(190, &[16453]);     // 190 => [16453]
            map.insert(238, &[1048645]);     // 238 => [1048645]
            map.insert(221, &[1049605]);     // 221 => [1049605]
            map.insert(207, &[40965]);     // 207 => [40965]
            map.insert(142, &[8206]);     // 142 => [8206]
            map.insert(175, &[16398]);     // 175 => [16398]
            map.insert(255, &[1048590]);     // 255 => [1048590]
            map.insert(156, &[8262]);     // 156 => [8262]
            map.insert(189, &[16454]);     // 189 => [16454]
            map.insert(237, &[1048646]);     // 237 => [1048646]
            map.insert(222, &[1049606]);     // 222 => [1049606]
            map.insert(204, &[40966]);     // 204 => [40966]
            map.insert(143, &[8207]);     // 143 => [8207]
            map.insert(174, &[16399]);     // 174 => [16399]
            map.insert(254, &[1048591]);     // 254 => [1048591]
            map.insert(157, &[8263]);     // 157 => [8263]
            map.insert(188, &[16455]);     // 188 => [16455]
            map.insert(236, &[1048647]);     // 236 => [1048647]
            map.insert(223, &[1049607]);     // 223 => [1049607]
            map.insert(205, &[40967]);     // 205 => [40967]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode21_13 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode21_13 {
    fn name(&self) -> String {
        "[21, 13] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        21
    }

    fn dimension(&self) -> usize {
        13
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
        let mut error = BinVector::with_capacity(21);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 21 / 64 + if 21 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(21) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(13);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[21 / 64] & !((1 << 21) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode21_13.generator_matrix();
        assert_eq!(code.ncols(), 21);
        assert_eq!(code.nrows(), 13);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode21_13;
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
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, false, false, false, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, false, false, false, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true, false, false, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, true, true, false, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, true, true, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, true, false, false, false, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, true, false, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, true, false, false, true, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, true, true, false, true, true, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, true, true, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, true, true, true, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, true, true, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, false, true, true, true, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, false, true, true, true, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, false, false, true, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, false, false, true, false, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, false, true, true, true, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, true, false, false, true, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, false, false, false, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, true, true, false, false, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, false, true, true, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, false, true, false, true, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, false, true, true, true, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, true, false, true, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, true, false, true, false, false, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, false, true, true, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, false, true, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, false, false, false, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, false, false, true, true, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, false, true, false, false, false, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, true, false, false, false, true, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, true, true, true, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, false, true, true, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
