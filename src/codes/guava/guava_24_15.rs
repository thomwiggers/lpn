use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[24, 15]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode24_15;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 11698177 ],
                &[ 8552450 ],
                &[ 8683524 ],
                &[ 8945672 ],
                &[ 9469968 ],
                &[ 10518560 ],
                &[ 12615744 ],
                &[ 11731072 ],
                &[ 8585472 ],
                &[ 8716800 ],
                &[ 8979456 ],
                &[ 9504768 ],
                &[ 10555392 ],
                &[ 12656640 ],
                &[ 3293184 ],
                
            ], 24));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 14626593 ],
                &[ 2232610 ],
                &[ 262660 ],
                &[ 525320 ],
                &[ 3151920 ],
                &[ 4202560 ],
                &[ 81792 ],
                &[ 16662528 ],
                &[ 8486912 ],
                
            ], 24));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(512, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(16, &[16]);     // 16 => [16]
            map.insert(19, &[32]);     // 19 => [32]
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(67, &[256]);     // 67 => [256]
            map.insert(69, &[512]);     // 69 => [512]
            map.insert(73, &[1024]);     // 73 => [1024]
            map.insert(81, &[2048]);     // 81 => [2048]
            map.insert(82, &[4096]);     // 82 => [4096]
            map.insert(97, &[8192]);     // 97 => [8192]
            map.insert(128, &[16384]);     // 128 => [16384]
            map.insert(256, &[32768]);     // 256 => [32768]
            map.insert(321, &[65536]);     // 321 => [65536]
            map.insert(131, &[131072]);     // 131 => [131072]
            map.insert(133, &[262144]);     // 133 => [262144]
            map.insert(137, &[524288]);     // 137 => [524288]
            map.insert(145, &[1048576]);     // 145 => [1048576]
            map.insert(146, &[2097152]);     // 146 => [2097152]
            map.insert(161, &[4194304]);     // 161 => [4194304]
            map.insert(385, &[8388608]);     // 385 => [8388608]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(66, &[257]);     // 66 => [257]
            map.insert(68, &[513]);     // 68 => [513]
            map.insert(72, &[1025]);     // 72 => [1025]
            map.insert(80, &[2049]);     // 80 => [2049]
            map.insert(83, &[4097]);     // 83 => [4097]
            map.insert(96, &[8193]);     // 96 => [8193]
            map.insert(129, &[16385]);     // 129 => [16385]
            map.insert(257, &[32769]);     // 257 => [32769]
            map.insert(320, &[65537]);     // 320 => [65537]
            map.insert(130, &[131073]);     // 130 => [131073]
            map.insert(132, &[262145]);     // 132 => [262145]
            map.insert(136, &[524289]);     // 136 => [524289]
            map.insert(144, &[1048577]);     // 144 => [1048577]
            map.insert(147, &[2097153]);     // 147 => [2097153]
            map.insert(160, &[4194305]);     // 160 => [4194305]
            map.insert(384, &[8388609]);     // 384 => [8388609]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(71, &[514]);     // 71 => [514]
            map.insert(75, &[1026]);     // 75 => [1026]
            map.insert(99, &[8194]);     // 99 => [8194]
            map.insert(258, &[32770]);     // 258 => [32770]
            map.insert(323, &[65538]);     // 323 => [65538]
            map.insert(135, &[262146]);     // 135 => [262146]
            map.insert(139, &[524290]);     // 139 => [524290]
            map.insert(163, &[4194306]);     // 163 => [4194306]
            map.insert(387, &[8388610]);     // 387 => [8388610]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(23, &[36]);     // 23 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(77, &[1028]);     // 77 => [1028]
            map.insert(85, &[2052]);     // 85 => [2052]
            map.insert(86, &[4100]);     // 86 => [4100]
            map.insert(101, &[8196]);     // 101 => [8196]
            map.insert(260, &[32772]);     // 260 => [32772]
            map.insert(325, &[65540]);     // 325 => [65540]
            map.insert(141, &[524292]);     // 141 => [524292]
            map.insert(149, &[1048580]);     // 149 => [1048580]
            map.insert(150, &[2097156]);     // 150 => [2097156]
            map.insert(165, &[4194308]);     // 165 => [4194308]
            map.insert(389, &[8388612]);     // 389 => [8388612]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(40, &[72]);     // 40 => [72]
            map.insert(89, &[2056]);     // 89 => [2056]
            map.insert(90, &[4104]);     // 90 => [4104]
            map.insert(105, &[8200]);     // 105 => [8200]
            map.insert(264, &[32776]);     // 264 => [32776]
            map.insert(329, &[65544]);     // 329 => [65544]
            map.insert(153, &[1048584]);     // 153 => [1048584]
            map.insert(154, &[2097160]);     // 154 => [2097160]
            map.insert(169, &[4194312]);     // 169 => [4194312]
            map.insert(393, &[8388616]);     // 393 => [8388616]
            map.insert(48, &[80]);     // 48 => [80]
            map.insert(113, &[8208]);     // 113 => [8208]
            map.insert(272, &[32784]);     // 272 => [32784]
            map.insert(337, &[65552]);     // 337 => [65552]
            map.insert(177, &[4194320]);     // 177 => [4194320]
            map.insert(401, &[8388624]);     // 401 => [8388624]
            map.insert(51, &[96]);     // 51 => [96]
            map.insert(114, &[8224]);     // 114 => [8224]
            map.insert(275, &[32800]);     // 275 => [32800]
            map.insert(338, &[65568]);     // 338 => [65568]
            map.insert(178, &[4194336]);     // 178 => [4194336]
            map.insert(402, &[8388640]);     // 402 => [8388640]
            map.insert(288, &[32832]);     // 288 => [32832]
            map.insert(353, &[65600]);     // 353 => [65600]
            map.insert(417, &[8388672]);     // 417 => [8388672]
            map.insert(192, &[16512]);     // 192 => [16512]
            map.insert(195, &[131200]);     // 195 => [131200]
            map.insert(197, &[262272]);     // 197 => [262272]
            map.insert(201, &[524416]);     // 201 => [524416]
            map.insert(209, &[1048704]);     // 209 => [1048704]
            map.insert(210, &[2097280]);     // 210 => [2097280]
            map.insert(225, &[4194432]);     // 225 => [4194432]
            map.insert(449, &[8388736]);     // 449 => [8388736]
            map.insert(198, &[262400]);     // 198 => [262400]
            map.insert(202, &[524544]);     // 202 => [524544]
            map.insert(226, &[4194560]);     // 226 => [4194560]
            map.insert(450, &[8388864]);     // 450 => [8388864]
            map.insert(204, &[524800]);     // 204 => [524800]
            map.insert(212, &[1049088]);     // 212 => [1049088]
            map.insert(215, &[2097664]);     // 215 => [2097664]
            map.insert(228, &[4194816]);     // 228 => [4194816]
            map.insert(452, &[8389120]);     // 452 => [8389120]
            map.insert(216, &[1049600]);     // 216 => [1049600]
            map.insert(219, &[2098176]);     // 219 => [2098176]
            map.insert(232, &[4195328]);     // 232 => [4195328]
            map.insert(456, &[8389632]);     // 456 => [8389632]
            map.insert(240, &[4196352]);     // 240 => [4196352]
            map.insert(464, &[8390656]);     // 464 => [8390656]
            map.insert(243, &[4198400]);     // 243 => [4198400]
            map.insert(467, &[8392704]);     // 467 => [8392704]
            map.insert(480, &[8396800]);     // 480 => [8396800]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(70, &[515]);     // 70 => [515]
            map.insert(74, &[1027]);     // 74 => [1027]
            map.insert(98, &[8195]);     // 98 => [8195]
            map.insert(259, &[32771]);     // 259 => [32771]
            map.insert(322, &[65539]);     // 322 => [65539]
            map.insert(134, &[262147]);     // 134 => [262147]
            map.insert(138, &[524291]);     // 138 => [524291]
            map.insert(162, &[4194307]);     // 162 => [4194307]
            map.insert(386, &[8388611]);     // 386 => [8388611]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(22, &[37]);     // 22 => [37]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(76, &[1029]);     // 76 => [1029]
            map.insert(84, &[2053]);     // 84 => [2053]
            map.insert(87, &[4101]);     // 87 => [4101]
            map.insert(100, &[8197]);     // 100 => [8197]
            map.insert(261, &[32773]);     // 261 => [32773]
            map.insert(324, &[65541]);     // 324 => [65541]
            map.insert(140, &[524293]);     // 140 => [524293]
            map.insert(148, &[1048581]);     // 148 => [1048581]
            map.insert(151, &[2097157]);     // 151 => [2097157]
            map.insert(164, &[4194309]);     // 164 => [4194309]
            map.insert(388, &[8388613]);     // 388 => [8388613]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(41, &[73]);     // 41 => [73]
            map.insert(88, &[2057]);     // 88 => [2057]
            map.insert(91, &[4105]);     // 91 => [4105]
            map.insert(104, &[8201]);     // 104 => [8201]
            map.insert(265, &[32777]);     // 265 => [32777]
            map.insert(328, &[65545]);     // 328 => [65545]
            map.insert(152, &[1048585]);     // 152 => [1048585]
            map.insert(155, &[2097161]);     // 155 => [2097161]
            map.insert(168, &[4194313]);     // 168 => [4194313]
            map.insert(392, &[8388617]);     // 392 => [8388617]
            map.insert(49, &[81]);     // 49 => [81]
            map.insert(112, &[8209]);     // 112 => [8209]
            map.insert(273, &[32785]);     // 273 => [32785]
            map.insert(336, &[65553]);     // 336 => [65553]
            map.insert(176, &[4194321]);     // 176 => [4194321]
            map.insert(400, &[8388625]);     // 400 => [8388625]
            map.insert(50, &[97]);     // 50 => [97]
            map.insert(115, &[8225]);     // 115 => [8225]
            map.insert(274, &[32801]);     // 274 => [32801]
            map.insert(339, &[65569]);     // 339 => [65569]
            map.insert(179, &[4194337]);     // 179 => [4194337]
            map.insert(403, &[8388641]);     // 403 => [8388641]
            map.insert(289, &[32833]);     // 289 => [32833]
            map.insert(352, &[65601]);     // 352 => [65601]
            map.insert(416, &[8388673]);     // 416 => [8388673]
            map.insert(193, &[16513]);     // 193 => [16513]
            map.insert(194, &[131201]);     // 194 => [131201]
            map.insert(196, &[262273]);     // 196 => [262273]
            map.insert(200, &[524417]);     // 200 => [524417]
            map.insert(208, &[1048705]);     // 208 => [1048705]
            map.insert(211, &[2097281]);     // 211 => [2097281]
            map.insert(224, &[4194433]);     // 224 => [4194433]
            map.insert(448, &[8388737]);     // 448 => [8388737]
            map.insert(199, &[262401]);     // 199 => [262401]
            map.insert(203, &[524545]);     // 203 => [524545]
            map.insert(227, &[4194561]);     // 227 => [4194561]
            map.insert(451, &[8388865]);     // 451 => [8388865]
            map.insert(205, &[524801]);     // 205 => [524801]
            map.insert(213, &[1049089]);     // 213 => [1049089]
            map.insert(214, &[2097665]);     // 214 => [2097665]
            map.insert(229, &[4194817]);     // 229 => [4194817]
            map.insert(453, &[8389121]);     // 453 => [8389121]
            map.insert(217, &[1049601]);     // 217 => [1049601]
            map.insert(218, &[2098177]);     // 218 => [2098177]
            map.insert(233, &[4195329]);     // 233 => [4195329]
            map.insert(457, &[8389633]);     // 457 => [8389633]
            map.insert(241, &[4196353]);     // 241 => [4196353]
            map.insert(465, &[8390657]);     // 465 => [8390657]
            map.insert(242, &[4198401]);     // 242 => [4198401]
            map.insert(466, &[8392705]);     // 466 => [8392705]
            map.insert(481, &[8396801]);     // 481 => [8396801]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(79, &[1030]);     // 79 => [1030]
            map.insert(103, &[8198]);     // 103 => [8198]
            map.insert(262, &[32774]);     // 262 => [32774]
            map.insert(327, &[65542]);     // 327 => [65542]
            map.insert(143, &[524294]);     // 143 => [524294]
            map.insert(167, &[4194310]);     // 167 => [4194310]
            map.insert(391, &[8388614]);     // 391 => [8388614]
            map.insert(42, &[74]);     // 42 => [74]
            map.insert(107, &[8202]);     // 107 => [8202]
            map.insert(266, &[32778]);     // 266 => [32778]
            map.insert(331, &[65546]);     // 331 => [65546]
            map.insert(171, &[4194314]);     // 171 => [4194314]
            map.insert(395, &[8388618]);     // 395 => [8388618]
            map.insert(290, &[32834]);     // 290 => [32834]
            map.insert(355, &[65602]);     // 355 => [65602]
            map.insert(419, &[8388674]);     // 419 => [8388674]
            map.insert(206, &[524802]);     // 206 => [524802]
            map.insert(230, &[4194818]);     // 230 => [4194818]
            map.insert(454, &[8389122]);     // 454 => [8389122]
            map.insert(234, &[4195330]);     // 234 => [4195330]
            map.insert(458, &[8389634]);     // 458 => [8389634]
            map.insert(482, &[8396802]);     // 482 => [8396802]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(31, &[44]);     // 31 => [44]
            map.insert(44, &[76]);     // 44 => [76]
            map.insert(93, &[2060]);     // 93 => [2060]
            map.insert(94, &[4108]);     // 94 => [4108]
            map.insert(109, &[8204]);     // 109 => [8204]
            map.insert(268, &[32780]);     // 268 => [32780]
            map.insert(333, &[65548]);     // 333 => [65548]
            map.insert(157, &[1048588]);     // 157 => [1048588]
            map.insert(158, &[2097164]);     // 158 => [2097164]
            map.insert(173, &[4194316]);     // 173 => [4194316]
            map.insert(397, &[8388620]);     // 397 => [8388620]
            map.insert(52, &[84]);     // 52 => [84]
            map.insert(117, &[8212]);     // 117 => [8212]
            map.insert(276, &[32788]);     // 276 => [32788]
            map.insert(341, &[65556]);     // 341 => [65556]
            map.insert(181, &[4194324]);     // 181 => [4194324]
            map.insert(405, &[8388628]);     // 405 => [8388628]
            map.insert(55, &[100]);     // 55 => [100]
            map.insert(118, &[8228]);     // 118 => [8228]
            map.insert(279, &[32804]);     // 279 => [32804]
            map.insert(342, &[65572]);     // 342 => [65572]
            map.insert(182, &[4194340]);     // 182 => [4194340]
            map.insert(406, &[8388644]);     // 406 => [8388644]
            map.insert(292, &[32836]);     // 292 => [32836]
            map.insert(357, &[65604]);     // 357 => [65604]
            map.insert(421, &[8388676]);     // 421 => [8388676]
            map.insert(220, &[1049604]);     // 220 => [1049604]
            map.insert(223, &[2098180]);     // 223 => [2098180]
            map.insert(236, &[4195332]);     // 236 => [4195332]
            map.insert(460, &[8389636]);     // 460 => [8389636]
            map.insert(244, &[4196356]);     // 244 => [4196356]
            map.insert(468, &[8390660]);     // 468 => [8390660]
            map.insert(247, &[4198404]);     // 247 => [4198404]
            map.insert(471, &[8392708]);     // 471 => [8392708]
            map.insert(484, &[8396804]);     // 484 => [8396804]
            map.insert(56, &[88]);     // 56 => [88]
            map.insert(121, &[8216]);     // 121 => [8216]
            map.insert(280, &[32792]);     // 280 => [32792]
            map.insert(345, &[65560]);     // 345 => [65560]
            map.insert(185, &[4194328]);     // 185 => [4194328]
            map.insert(409, &[8388632]);     // 409 => [8388632]
            map.insert(59, &[104]);     // 59 => [104]
            map.insert(122, &[8232]);     // 122 => [8232]
            map.insert(283, &[32808]);     // 283 => [32808]
            map.insert(346, &[65576]);     // 346 => [65576]
            map.insert(186, &[4194344]);     // 186 => [4194344]
            map.insert(410, &[8388648]);     // 410 => [8388648]
            map.insert(296, &[32840]);     // 296 => [32840]
            map.insert(361, &[65608]);     // 361 => [65608]
            map.insert(425, &[8388680]);     // 425 => [8388680]
            map.insert(248, &[4196360]);     // 248 => [4196360]
            map.insert(472, &[8390664]);     // 472 => [8390664]
            map.insert(251, &[4198408]);     // 251 => [4198408]
            map.insert(475, &[8392712]);     // 475 => [8392712]
            map.insert(488, &[8396808]);     // 488 => [8396808]
            map.insert(304, &[32848]);     // 304 => [32848]
            map.insert(369, &[65616]);     // 369 => [65616]
            map.insert(433, &[8388688]);     // 433 => [8388688]
            map.insert(496, &[8396816]);     // 496 => [8396816]
            map.insert(307, &[32864]);     // 307 => [32864]
            map.insert(370, &[65632]);     // 370 => [65632]
            map.insert(434, &[8388704]);     // 434 => [8388704]
            map.insert(499, &[8396832]);     // 499 => [8396832]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[71]);     // 39 => [71]
            map.insert(78, &[1031]);     // 78 => [1031]
            map.insert(102, &[8199]);     // 102 => [8199]
            map.insert(263, &[32775]);     // 263 => [32775]
            map.insert(326, &[65543]);     // 326 => [65543]
            map.insert(142, &[524295]);     // 142 => [524295]
            map.insert(166, &[4194311]);     // 166 => [4194311]
            map.insert(390, &[8388615]);     // 390 => [8388615]
            map.insert(43, &[75]);     // 43 => [75]
            map.insert(106, &[8203]);     // 106 => [8203]
            map.insert(267, &[32779]);     // 267 => [32779]
            map.insert(330, &[65547]);     // 330 => [65547]
            map.insert(170, &[4194315]);     // 170 => [4194315]
            map.insert(394, &[8388619]);     // 394 => [8388619]
            map.insert(291, &[32835]);     // 291 => [32835]
            map.insert(354, &[65603]);     // 354 => [65603]
            map.insert(418, &[8388675]);     // 418 => [8388675]
            map.insert(207, &[524803]);     // 207 => [524803]
            map.insert(231, &[4194819]);     // 231 => [4194819]
            map.insert(455, &[8389123]);     // 455 => [8389123]
            map.insert(235, &[4195331]);     // 235 => [4195331]
            map.insert(459, &[8389635]);     // 459 => [8389635]
            map.insert(483, &[8396803]);     // 483 => [8396803]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(30, &[45]);     // 30 => [45]
            map.insert(45, &[77]);     // 45 => [77]
            map.insert(92, &[2061]);     // 92 => [2061]
            map.insert(95, &[4109]);     // 95 => [4109]
            map.insert(108, &[8205]);     // 108 => [8205]
            map.insert(269, &[32781]);     // 269 => [32781]
            map.insert(332, &[65549]);     // 332 => [65549]
            map.insert(156, &[1048589]);     // 156 => [1048589]
            map.insert(159, &[2097165]);     // 159 => [2097165]
            map.insert(172, &[4194317]);     // 172 => [4194317]
            map.insert(396, &[8388621]);     // 396 => [8388621]
            map.insert(53, &[85]);     // 53 => [85]
            map.insert(116, &[8213]);     // 116 => [8213]
            map.insert(277, &[32789]);     // 277 => [32789]
            map.insert(340, &[65557]);     // 340 => [65557]
            map.insert(180, &[4194325]);     // 180 => [4194325]
            map.insert(404, &[8388629]);     // 404 => [8388629]
            map.insert(54, &[101]);     // 54 => [101]
            map.insert(119, &[8229]);     // 119 => [8229]
            map.insert(278, &[32805]);     // 278 => [32805]
            map.insert(343, &[65573]);     // 343 => [65573]
            map.insert(183, &[4194341]);     // 183 => [4194341]
            map.insert(407, &[8388645]);     // 407 => [8388645]
            map.insert(293, &[32837]);     // 293 => [32837]
            map.insert(356, &[65605]);     // 356 => [65605]
            map.insert(420, &[8388677]);     // 420 => [8388677]
            map.insert(221, &[1049605]);     // 221 => [1049605]
            map.insert(222, &[2098181]);     // 222 => [2098181]
            map.insert(237, &[4195333]);     // 237 => [4195333]
            map.insert(461, &[8389637]);     // 461 => [8389637]
            map.insert(245, &[4196357]);     // 245 => [4196357]
            map.insert(469, &[8390661]);     // 469 => [8390661]
            map.insert(246, &[4198405]);     // 246 => [4198405]
            map.insert(470, &[8392709]);     // 470 => [8392709]
            map.insert(485, &[8396805]);     // 485 => [8396805]
            map.insert(57, &[89]);     // 57 => [89]
            map.insert(120, &[8217]);     // 120 => [8217]
            map.insert(281, &[32793]);     // 281 => [32793]
            map.insert(344, &[65561]);     // 344 => [65561]
            map.insert(184, &[4194329]);     // 184 => [4194329]
            map.insert(408, &[8388633]);     // 408 => [8388633]
            map.insert(58, &[105]);     // 58 => [105]
            map.insert(123, &[8233]);     // 123 => [8233]
            map.insert(282, &[32809]);     // 282 => [32809]
            map.insert(347, &[65577]);     // 347 => [65577]
            map.insert(187, &[4194345]);     // 187 => [4194345]
            map.insert(411, &[8388649]);     // 411 => [8388649]
            map.insert(297, &[32841]);     // 297 => [32841]
            map.insert(360, &[65609]);     // 360 => [65609]
            map.insert(424, &[8388681]);     // 424 => [8388681]
            map.insert(249, &[4196361]);     // 249 => [4196361]
            map.insert(473, &[8390665]);     // 473 => [8390665]
            map.insert(250, &[4198409]);     // 250 => [4198409]
            map.insert(474, &[8392713]);     // 474 => [8392713]
            map.insert(489, &[8396809]);     // 489 => [8396809]
            map.insert(305, &[32849]);     // 305 => [32849]
            map.insert(368, &[65617]);     // 368 => [65617]
            map.insert(432, &[8388689]);     // 432 => [8388689]
            map.insert(497, &[8396817]);     // 497 => [8396817]
            map.insert(306, &[32865]);     // 306 => [32865]
            map.insert(371, &[65633]);     // 371 => [65633]
            map.insert(435, &[8388705]);     // 435 => [8388705]
            map.insert(498, &[8396833]);     // 498 => [8396833]
            map.insert(46, &[78]);     // 46 => [78]
            map.insert(111, &[8206]);     // 111 => [8206]
            map.insert(270, &[32782]);     // 270 => [32782]
            map.insert(335, &[65550]);     // 335 => [65550]
            map.insert(175, &[4194318]);     // 175 => [4194318]
            map.insert(399, &[8388622]);     // 399 => [8388622]
            map.insert(294, &[32838]);     // 294 => [32838]
            map.insert(359, &[65606]);     // 359 => [65606]
            map.insert(423, &[8388678]);     // 423 => [8388678]
            map.insert(238, &[4195334]);     // 238 => [4195334]
            map.insert(462, &[8389638]);     // 462 => [8389638]
            map.insert(486, &[8396806]);     // 486 => [8396806]
            map.insert(298, &[32842]);     // 298 => [32842]
            map.insert(363, &[65610]);     // 363 => [65610]
            map.insert(427, &[8388682]);     // 427 => [8388682]
            map.insert(490, &[8396810]);     // 490 => [8396810]
            map.insert(60, &[92]);     // 60 => [92]
            map.insert(125, &[8220]);     // 125 => [8220]
            map.insert(284, &[32796]);     // 284 => [32796]
            map.insert(349, &[65564]);     // 349 => [65564]
            map.insert(189, &[4194332]);     // 189 => [4194332]
            map.insert(413, &[8388636]);     // 413 => [8388636]
            map.insert(63, &[108]);     // 63 => [108]
            map.insert(126, &[8236]);     // 126 => [8236]
            map.insert(287, &[32812]);     // 287 => [32812]
            map.insert(350, &[65580]);     // 350 => [65580]
            map.insert(190, &[4194348]);     // 190 => [4194348]
            map.insert(414, &[8388652]);     // 414 => [8388652]
            map.insert(300, &[32844]);     // 300 => [32844]
            map.insert(365, &[65612]);     // 365 => [65612]
            map.insert(429, &[8388684]);     // 429 => [8388684]
            map.insert(252, &[4196364]);     // 252 => [4196364]
            map.insert(476, &[8390668]);     // 476 => [8390668]
            map.insert(255, &[4198412]);     // 255 => [4198412]
            map.insert(479, &[8392716]);     // 479 => [8392716]
            map.insert(492, &[8396812]);     // 492 => [8396812]
            map.insert(308, &[32852]);     // 308 => [32852]
            map.insert(373, &[65620]);     // 373 => [65620]
            map.insert(437, &[8388692]);     // 437 => [8388692]
            map.insert(500, &[8396820]);     // 500 => [8396820]
            map.insert(311, &[32868]);     // 311 => [32868]
            map.insert(374, &[65636]);     // 374 => [65636]
            map.insert(438, &[8388708]);     // 438 => [8388708]
            map.insert(503, &[8396836]);     // 503 => [8396836]
            map.insert(312, &[32856]);     // 312 => [32856]
            map.insert(377, &[65624]);     // 377 => [65624]
            map.insert(441, &[8388696]);     // 441 => [8388696]
            map.insert(504, &[8396824]);     // 504 => [8396824]
            map.insert(315, &[32872]);     // 315 => [32872]
            map.insert(378, &[65640]);     // 378 => [65640]
            map.insert(442, &[8388712]);     // 442 => [8388712]
            map.insert(507, &[8396840]);     // 507 => [8396840]
            map.insert(47, &[79]);     // 47 => [79]
            map.insert(110, &[8207]);     // 110 => [8207]
            map.insert(271, &[32783]);     // 271 => [32783]
            map.insert(334, &[65551]);     // 334 => [65551]
            map.insert(174, &[4194319]);     // 174 => [4194319]
            map.insert(398, &[8388623]);     // 398 => [8388623]
            map.insert(295, &[32839]);     // 295 => [32839]
            map.insert(358, &[65607]);     // 358 => [65607]
            map.insert(422, &[8388679]);     // 422 => [8388679]
            map.insert(239, &[4195335]);     // 239 => [4195335]
            map.insert(463, &[8389639]);     // 463 => [8389639]
            map.insert(487, &[8396807]);     // 487 => [8396807]
            map.insert(299, &[32843]);     // 299 => [32843]
            map.insert(362, &[65611]);     // 362 => [65611]
            map.insert(426, &[8388683]);     // 426 => [8388683]
            map.insert(491, &[8396811]);     // 491 => [8396811]
            map.insert(61, &[93]);     // 61 => [93]
            map.insert(124, &[8221]);     // 124 => [8221]
            map.insert(285, &[32797]);     // 285 => [32797]
            map.insert(348, &[65565]);     // 348 => [65565]
            map.insert(188, &[4194333]);     // 188 => [4194333]
            map.insert(412, &[8388637]);     // 412 => [8388637]
            map.insert(62, &[109]);     // 62 => [109]
            map.insert(127, &[8237]);     // 127 => [8237]
            map.insert(286, &[32813]);     // 286 => [32813]
            map.insert(351, &[65581]);     // 351 => [65581]
            map.insert(191, &[4194349]);     // 191 => [4194349]
            map.insert(415, &[8388653]);     // 415 => [8388653]
            map.insert(301, &[32845]);     // 301 => [32845]
            map.insert(364, &[65613]);     // 364 => [65613]
            map.insert(428, &[8388685]);     // 428 => [8388685]
            map.insert(253, &[4196365]);     // 253 => [4196365]
            map.insert(477, &[8390669]);     // 477 => [8390669]
            map.insert(254, &[4198413]);     // 254 => [4198413]
            map.insert(478, &[8392717]);     // 478 => [8392717]
            map.insert(493, &[8396813]);     // 493 => [8396813]
            map.insert(309, &[32853]);     // 309 => [32853]
            map.insert(372, &[65621]);     // 372 => [65621]
            map.insert(436, &[8388693]);     // 436 => [8388693]
            map.insert(501, &[8396821]);     // 501 => [8396821]
            map.insert(310, &[32869]);     // 310 => [32869]
            map.insert(375, &[65637]);     // 375 => [65637]
            map.insert(439, &[8388709]);     // 439 => [8388709]
            map.insert(502, &[8396837]);     // 502 => [8396837]
            map.insert(313, &[32857]);     // 313 => [32857]
            map.insert(376, &[65625]);     // 376 => [65625]
            map.insert(440, &[8388697]);     // 440 => [8388697]
            map.insert(505, &[8396825]);     // 505 => [8396825]
            map.insert(314, &[32873]);     // 314 => [32873]
            map.insert(379, &[65641]);     // 379 => [65641]
            map.insert(443, &[8388713]);     // 443 => [8388713]
            map.insert(506, &[8396841]);     // 506 => [8396841]
            map.insert(302, &[32846]);     // 302 => [32846]
            map.insert(367, &[65614]);     // 367 => [65614]
            map.insert(431, &[8388686]);     // 431 => [8388686]
            map.insert(494, &[8396814]);     // 494 => [8396814]
            map.insert(316, &[32860]);     // 316 => [32860]
            map.insert(381, &[65628]);     // 381 => [65628]
            map.insert(445, &[8388700]);     // 445 => [8388700]
            map.insert(508, &[8396828]);     // 508 => [8396828]
            map.insert(319, &[32876]);     // 319 => [32876]
            map.insert(382, &[65644]);     // 382 => [65644]
            map.insert(446, &[8388716]);     // 446 => [8388716]
            map.insert(511, &[8396844]);     // 511 => [8396844]
            map.insert(303, &[32847]);     // 303 => [32847]
            map.insert(366, &[65615]);     // 366 => [65615]
            map.insert(430, &[8388687]);     // 430 => [8388687]
            map.insert(495, &[8396815]);     // 495 => [8396815]
            map.insert(317, &[32861]);     // 317 => [32861]
            map.insert(380, &[65629]);     // 380 => [65629]
            map.insert(444, &[8388701]);     // 444 => [8388701]
            map.insert(509, &[8396829]);     // 509 => [8396829]
            map.insert(318, &[32877]);     // 318 => [32877]
            map.insert(383, &[65645]);     // 383 => [65645]
            map.insert(447, &[8388717]);     // 447 => [8388717]
            map.insert(510, &[8396845]);     // 510 => [8396845]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode24_15 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode24_15 {
    fn name(&self) -> String {
        "[24, 15] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        24
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
        codeword.truncate(15);
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
        let code = GuavaCode24_15.generator_matrix();
        assert_eq!(code.ncols(), 24);
        assert_eq!(code.nrows(), 15);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode24_15;
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
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, false, true, false, true, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, true, true, true, true, false, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, false, true, false, false, true, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, true, false, false, true, true, false, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, false, true, true, true, true, true, true, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, false, true, true, true, true, true, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, false, false, false, false, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, false, true, true, true, false, false, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, true, true, true, false, false, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, false, true, false, false, true, true, false, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, true, false, false, false, true, false, false, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, true, true, false, true, false, false, true, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, true, true, false, true, true, false, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, false, false, true, false, false, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, false, true, false, false, true, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, true, true, false, false, true, false, true, false, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, true, false, false, true, true, true, false, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, true, true, true, false, true, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
