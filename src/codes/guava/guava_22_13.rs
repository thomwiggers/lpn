use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[22, 13]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode22_13;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 966657 ],
                &[ 2408450 ],
                &[ 3907588 ],
                &[ 3719176 ],
                &[ 1220624 ],
                &[ 1769504 ],
                &[ 3104832 ],
                &[ 368768 ],
                &[ 3662080 ],
                &[ 2744832 ],
                &[ 2892800 ],
                &[ 3115008 ],
                &[ 3477504 ],
                
            ], 22));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 893953 ],
                &[ 1225730 ],
                &[ 3012612 ],
                &[ 3947016 ],
                &[ 3592720 ],
                &[ 3665440 ],
                &[ 2328128 ],
                &[ 3695744 ],
                &[ 61184 ],
                
            ], 22));
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
            map.insert(32, &[32]);     // 32 => [32]
            map.insert(64, &[64]);     // 64 => [64]
            map.insert(128, &[128]);     // 128 => [128]
            map.insert(256, &[256]);     // 256 => [256]
            map.insert(376, &[512]);     // 376 => [512]
            map.insert(483, &[1024]);     // 483 => [1024]
            map.insert(300, &[2048]);     // 300 => [2048]
            map.insert(30, &[4096]);     // 30 => [4096]
            map.insert(431, &[8192]);     // 431 => [8192]
            map.insert(436, &[16384]);     // 436 => [16384]
            map.insert(375, &[32768]);     // 375 => [32768]
            map.insert(101, &[65536]);     // 101 => [65536]
            map.insert(114, &[131072]);     // 114 => [131072]
            map.insert(61, &[262144]);     // 61 => [262144]
            map.insert(141, &[524288]);     // 141 => [524288]
            map.insert(186, &[1048576]);     // 186 => [1048576]
            map.insert(252, &[2097152]);     // 252 => [2097152]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(257, &[257]);     // 257 => [257]
            map.insert(377, &[513]);     // 377 => [513]
            map.insert(482, &[1025]);     // 482 => [1025]
            map.insert(301, &[2049]);     // 301 => [2049]
            map.insert(31, &[4097]);     // 31 => [4097]
            map.insert(430, &[8193]);     // 430 => [8193]
            map.insert(437, &[16385]);     // 437 => [16385]
            map.insert(374, &[32769]);     // 374 => [32769]
            map.insert(100, &[65537]);     // 100 => [65537]
            map.insert(115, &[131073]);     // 115 => [131073]
            map.insert(60, &[262145]);     // 60 => [262145]
            map.insert(140, &[524289]);     // 140 => [524289]
            map.insert(187, &[1048577]);     // 187 => [1048577]
            map.insert(253, &[2097153]);     // 253 => [2097153]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(258, &[258]);     // 258 => [258]
            map.insert(378, &[514]);     // 378 => [514]
            map.insert(481, &[1026]);     // 481 => [1026]
            map.insert(302, &[2050]);     // 302 => [2050]
            map.insert(28, &[4098]);     // 28 => [4098]
            map.insert(429, &[8194]);     // 429 => [8194]
            map.insert(438, &[16386]);     // 438 => [16386]
            map.insert(373, &[32770]);     // 373 => [32770]
            map.insert(103, &[65538]);     // 103 => [65538]
            map.insert(112, &[131074]);     // 112 => [131074]
            map.insert(63, &[262146]);     // 63 => [262146]
            map.insert(143, &[524290]);     // 143 => [524290]
            map.insert(184, &[1048578]);     // 184 => [1048578]
            map.insert(254, &[2097154]);     // 254 => [2097154]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(260, &[260]);     // 260 => [260]
            map.insert(380, &[516]);     // 380 => [516]
            map.insert(487, &[1028]);     // 487 => [1028]
            map.insert(296, &[2052]);     // 296 => [2052]
            map.insert(26, &[4100]);     // 26 => [4100]
            map.insert(427, &[8196]);     // 427 => [8196]
            map.insert(432, &[16388]);     // 432 => [16388]
            map.insert(371, &[32772]);     // 371 => [32772]
            map.insert(97, &[65540]);     // 97 => [65540]
            map.insert(118, &[131076]);     // 118 => [131076]
            map.insert(57, &[262148]);     // 57 => [262148]
            map.insert(137, &[524292]);     // 137 => [524292]
            map.insert(190, &[1048580]);     // 190 => [1048580]
            map.insert(248, &[2097156]);     // 248 => [2097156]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(264, &[264]);     // 264 => [264]
            map.insert(368, &[520]);     // 368 => [520]
            map.insert(491, &[1032]);     // 491 => [1032]
            map.insert(292, &[2056]);     // 292 => [2056]
            map.insert(22, &[4104]);     // 22 => [4104]
            map.insert(423, &[8200]);     // 423 => [8200]
            map.insert(444, &[16392]);     // 444 => [16392]
            map.insert(383, &[32776]);     // 383 => [32776]
            map.insert(109, &[65544]);     // 109 => [65544]
            map.insert(122, &[131080]);     // 122 => [131080]
            map.insert(53, &[262152]);     // 53 => [262152]
            map.insert(133, &[524296]);     // 133 => [524296]
            map.insert(178, &[1048584]);     // 178 => [1048584]
            map.insert(244, &[2097160]);     // 244 => [2097160]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(272, &[272]);     // 272 => [272]
            map.insert(360, &[528]);     // 360 => [528]
            map.insert(499, &[1040]);     // 499 => [1040]
            map.insert(316, &[2064]);     // 316 => [2064]
            map.insert(14, &[4112]);     // 14 => [4112]
            map.insert(447, &[8208]);     // 447 => [8208]
            map.insert(420, &[16400]);     // 420 => [16400]
            map.insert(359, &[32784]);     // 359 => [32784]
            map.insert(117, &[65552]);     // 117 => [65552]
            map.insert(98, &[131088]);     // 98 => [131088]
            map.insert(45, &[262160]);     // 45 => [262160]
            map.insert(157, &[524304]);     // 157 => [524304]
            map.insert(170, &[1048592]);     // 170 => [1048592]
            map.insert(236, &[2097168]);     // 236 => [2097168]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(288, &[288]);     // 288 => [288]
            map.insert(344, &[544]);     // 344 => [544]
            map.insert(451, &[1056]);     // 451 => [1056]
            map.insert(268, &[2080]);     // 268 => [2080]
            map.insert(62, &[4128]);     // 62 => [4128]
            map.insert(399, &[8224]);     // 399 => [8224]
            map.insert(404, &[16416]);     // 404 => [16416]
            map.insert(343, &[32800]);     // 343 => [32800]
            map.insert(69, &[65568]);     // 69 => [65568]
            map.insert(82, &[131104]);     // 82 => [131104]
            map.insert(29, &[262176]);     // 29 => [262176]
            map.insert(173, &[524320]);     // 173 => [524320]
            map.insert(154, &[1048608]);     // 154 => [1048608]
            map.insert(220, &[2097184]);     // 220 => [2097184]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(320, &[320]);     // 320 => [320]
            map.insert(312, &[576]);     // 312 => [576]
            map.insert(419, &[1088]);     // 419 => [1088]
            map.insert(364, &[2112]);     // 364 => [2112]
            map.insert(94, &[4160]);     // 94 => [4160]
            map.insert(495, &[8256]);     // 495 => [8256]
            map.insert(500, &[16448]);     // 500 => [16448]
            map.insert(311, &[32832]);     // 311 => [32832]
            map.insert(37, &[65600]);     // 37 => [65600]
            map.insert(50, &[131136]);     // 50 => [131136]
            map.insert(125, &[262208]);     // 125 => [262208]
            map.insert(205, &[524352]);     // 205 => [524352]
            map.insert(250, &[1048640]);     // 250 => [1048640]
            map.insert(188, &[2097216]);     // 188 => [2097216]
            map.insert(384, &[384]);     // 384 => [384]
            map.insert(504, &[640]);     // 504 => [640]
            map.insert(355, &[1152]);     // 355 => [1152]
            map.insert(428, &[2176]);     // 428 => [2176]
            map.insert(158, &[4224]);     // 158 => [4224]
            map.insert(303, &[8320]);     // 303 => [8320]
            map.insert(308, &[16512]);     // 308 => [16512]
            map.insert(503, &[32896]);     // 503 => [32896]
            map.insert(229, &[65664]);     // 229 => [65664]
            map.insert(242, &[131200]);     // 242 => [131200]
            map.insert(189, &[262272]);     // 189 => [262272]
            map.insert(13, &[524416]);     // 13 => [524416]
            map.insert(58, &[1048704]);     // 58 => [1048704]
            map.insert(124, &[2097280]);     // 124 => [2097280]
            map.insert(120, &[768]);     // 120 => [768]
            map.insert(227, &[1280]);     // 227 => [1280]
            map.insert(44, &[2304]);     // 44 => [2304]
            map.insert(286, &[4352]);     // 286 => [4352]
            map.insert(175, &[8448]);     // 175 => [8448]
            map.insert(180, &[16640]);     // 180 => [16640]
            map.insert(119, &[33024]);     // 119 => [33024]
            map.insert(357, &[65792]);     // 357 => [65792]
            map.insert(370, &[131328]);     // 370 => [131328]
            map.insert(317, &[262400]);     // 317 => [262400]
            map.insert(397, &[524544]);     // 397 => [524544]
            map.insert(442, &[1048832]);     // 442 => [1048832]
            map.insert(508, &[2097408]);     // 508 => [2097408]
            map.insert(155, &[1536]);     // 155 => [1536]
            map.insert(84, &[2560]);     // 84 => [2560]
            map.insert(358, &[4608]);     // 358 => [4608]
            map.insert(215, &[8704]);     // 215 => [8704]
            map.insert(204, &[16896]);     // 204 => [16896]
            map.insert(15, &[33280]);     // 15 => [33280]
            map.insert(285, &[66048]);     // 285 => [66048]
            map.insert(266, &[131584]);     // 266 => [131584]
            map.insert(325, &[262656]);     // 325 => [262656]
            map.insert(501, &[524800]);     // 501 => [524800]
            map.insert(450, &[1049088]);     // 450 => [1049088]
            map.insert(388, &[2097664]);     // 388 => [2097664]
            map.insert(207, &[3072]);     // 207 => [3072]
            map.insert(509, &[5120]);     // 509 => [5120]
            map.insert(76, &[9216]);     // 76 => [9216]
            map.insert(87, &[17408]);     // 87 => [17408]
            map.insert(148, &[33792]);     // 148 => [33792]
            map.insert(390, &[66560]);     // 390 => [66560]
            map.insert(401, &[132096]);     // 401 => [132096]
            map.insert(478, &[263168]);     // 478 => [263168]
            map.insert(366, &[525312]);     // 366 => [525312]
            map.insert(345, &[1049600]);     // 345 => [1049600]
            map.insert(287, &[2098176]);     // 287 => [2098176]
            map.insert(306, &[6144]);     // 306 => [6144]
            map.insert(131, &[10240]);     // 131 => [10240]
            map.insert(152, &[18432]);     // 152 => [18432]
            map.insert(91, &[34816]);     // 91 => [34816]
            map.insert(329, &[67584]);     // 329 => [67584]
            map.insert(350, &[133120]);     // 350 => [133120]
            map.insert(273, &[264192]);     // 273 => [264192]
            map.insert(417, &[526336]);     // 417 => [526336]
            map.insert(406, &[1050624]);     // 406 => [1050624]
            map.insert(464, &[2099200]);     // 464 => [2099200]
            map.insert(433, &[12288]);     // 433 => [12288]
            map.insert(426, &[20480]);     // 426 => [20480]
            map.insert(361, &[36864]);     // 361 => [36864]
            map.insert(123, &[69632]);     // 123 => [69632]
            map.insert(108, &[135168]);     // 108 => [135168]
            map.insert(35, &[266240]);     // 35 => [266240]
            map.insert(147, &[528384]);     // 147 => [528384]
            map.insert(164, &[1052672]);     // 164 => [1052672]
            map.insert(226, &[2101248]);     // 226 => [2101248]
            map.insert(27, &[24576]);     // 27 => [24576]
            map.insert(216, &[40960]);     // 216 => [40960]
            map.insert(458, &[73728]);     // 458 => [73728]
            map.insert(477, &[139264]);     // 477 => [139264]
            map.insert(402, &[270336]);     // 402 => [270336]
            map.insert(290, &[532480]);     // 290 => [532480]
            map.insert(277, &[1056768]);     // 277 => [1056768]
            map.insert(339, &[2105344]);     // 339 => [2105344]
            map.insert(195, &[49152]);     // 195 => [49152]
            map.insert(465, &[81920]);     // 465 => [81920]
            map.insert(454, &[147456]);     // 454 => [147456]
            map.insert(393, &[278528]);     // 393 => [278528]
            map.insert(313, &[540672]);     // 313 => [540672]
            map.insert(270, &[1064960]);     // 270 => [1064960]
            map.insert(328, &[2113536]);     // 328 => [2113536]
            map.insert(274, &[98304]);     // 274 => [98304]
            map.insert(261, &[163840]);     // 261 => [163840]
            map.insert(330, &[294912]);     // 330 => [294912]
            map.insert(506, &[557056]);     // 506 => [557056]
            map.insert(461, &[1081344]);     // 461 => [1081344]
            map.insert(395, &[2129920]);     // 395 => [2129920]
            map.insert(23, &[196608]);     // 23 => [196608]
            map.insert(88, &[327680]);     // 88 => [327680]
            map.insert(232, &[589824]);     // 232 => [589824]
            map.insert(223, &[1114112]);     // 223 => [1114112]
            map.insert(153, &[2162688]);     // 153 => [2162688]
            map.insert(79, &[393216]);     // 79 => [393216]
            map.insert(255, &[655360]);     // 255 => [655360]
            map.insert(200, &[1179648]);     // 200 => [1179648]
            map.insert(142, &[2228224]);     // 142 => [2228224]
            map.insert(176, &[786432]);     // 176 => [786432]
            map.insert(135, &[1310720]);     // 135 => [1310720]
            map.insert(193, &[2359296]);     // 193 => [2359296]
            map.insert(55, &[1572864]);     // 55 => [1572864]
            map.insert(113, &[2621440]);     // 113 => [2621440]
            map.insert(70, &[3145728]);     // 70 => [3145728]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(259, &[259]);     // 259 => [259]
            map.insert(379, &[515]);     // 379 => [515]
            map.insert(480, &[1027]);     // 480 => [1027]
            map.insert(439, &[16387]);     // 439 => [16387]
            map.insert(372, &[32771]);     // 372 => [32771]
            map.insert(102, &[65539]);     // 102 => [65539]
            map.insert(185, &[1048579]);     // 185 => [1048579]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(381, &[517]);     // 381 => [517]
            map.insert(486, &[1029]);     // 486 => [1029]
            map.insert(297, &[2053]);     // 297 => [2053]
            map.insert(56, &[262149]);     // 56 => [262149]
            map.insert(191, &[1048581]);     // 191 => [1048581]
            map.insert(249, &[2097157]);     // 249 => [2097157]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(265, &[265]);     // 265 => [265]
            map.insert(369, &[521]);     // 369 => [521]
            map.insert(490, &[1033]);     // 490 => [1033]
            map.insert(293, &[2057]);     // 293 => [2057]
            map.insert(422, &[8201]);     // 422 => [8201]
            map.insert(445, &[16393]);     // 445 => [16393]
            map.insert(382, &[32777]);     // 382 => [32777]
            map.insert(52, &[262153]);     // 52 => [262153]
            map.insert(179, &[1048585]);     // 179 => [1048585]
            map.insert(245, &[2097161]);     // 245 => [2097161]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(498, &[1041]);     // 498 => [1041]
            map.insert(446, &[8209]);     // 446 => [8209]
            map.insert(421, &[16401]);     // 421 => [16401]
            map.insert(116, &[65553]);     // 116 => [65553]
            map.insert(99, &[131089]);     // 99 => [131089]
            map.insert(156, &[524305]);     // 156 => [524305]
            map.insert(171, &[1048593]);     // 171 => [1048593]
            map.insert(237, &[2097169]);     // 237 => [2097169]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(289, &[289]);     // 289 => [289]
            map.insert(269, &[2081]);     // 269 => [2081]
            map.insert(398, &[8225]);     // 398 => [8225]
            map.insert(405, &[16417]);     // 405 => [16417]
            map.insert(342, &[32801]);     // 342 => [32801]
            map.insert(83, &[131105]);     // 83 => [131105]
            map.insert(172, &[524321]);     // 172 => [524321]
            map.insert(221, &[2097185]);     // 221 => [2097185]
            map.insert(321, &[321]);     // 321 => [321]
            map.insert(418, &[1089]);     // 418 => [1089]
            map.insert(365, &[2113]);     // 365 => [2113]
            map.insert(95, &[4161]);     // 95 => [4161]
            map.insert(494, &[8257]);     // 494 => [8257]
            map.insert(310, &[32833]);     // 310 => [32833]
            map.insert(51, &[131137]);     // 51 => [131137]
            map.insert(251, &[1048641]);     // 251 => [1048641]
            map.insert(385, &[385]);     // 385 => [385]
            map.insert(505, &[641]);     // 505 => [641]
            map.insert(354, &[1153]);     // 354 => [1153]
            map.insert(159, &[4225]);     // 159 => [4225]
            map.insert(309, &[16513]);     // 309 => [16513]
            map.insert(502, &[32897]);     // 502 => [32897]
            map.insert(228, &[65665]);     // 228 => [65665]
            map.insert(243, &[131201]);     // 243 => [131201]
            map.insert(59, &[1048705]);     // 59 => [1048705]
            map.insert(121, &[769]);     // 121 => [769]
            map.insert(174, &[8449]);     // 174 => [8449]
            map.insert(181, &[16641]);     // 181 => [16641]
            map.insert(356, &[65793]);     // 356 => [65793]
            map.insert(396, &[524545]);     // 396 => [524545]
            map.insert(443, &[1048833]);     // 443 => [1048833]
            map.insert(85, &[2561]);     // 85 => [2561]
            map.insert(214, &[8705]);     // 214 => [8705]
            map.insert(284, &[66049]);     // 284 => [66049]
            map.insert(267, &[131585]);     // 267 => [131585]
            map.insert(324, &[262657]);     // 324 => [262657]
            map.insert(389, &[2097665]);     // 389 => [2097665]
            map.insert(206, &[3073]);     // 206 => [3073]
            map.insert(77, &[9217]);     // 77 => [9217]
            map.insert(86, &[17409]);     // 86 => [17409]
            map.insert(149, &[33793]);     // 149 => [33793]
            map.insert(391, &[66561]);     // 391 => [66561]
            map.insert(400, &[132097]);     // 400 => [132097]
            map.insert(479, &[263169]);     // 479 => [263169]
            map.insert(367, &[525313]);     // 367 => [525313]
            map.insert(307, &[6145]);     // 307 => [6145]
            map.insert(90, &[34817]);     // 90 => [34817]
            map.insert(351, &[133121]);     // 351 => [133121]
            map.insert(416, &[526337]);     // 416 => [526337]
            map.insert(407, &[1050625]);     // 407 => [1050625]
            map.insert(146, &[528385]);     // 146 => [528385]
            map.insert(165, &[1052673]);     // 165 => [1052673]
            map.insert(217, &[40961]);     // 217 => [40961]
            map.insert(459, &[73729]);     // 459 => [73729]
            map.insert(476, &[139265]);     // 476 => [139265]
            map.insert(403, &[270337]);     // 403 => [270337]
            map.insert(291, &[532481]);     // 291 => [532481]
            map.insert(276, &[1056769]);     // 276 => [1056769]
            map.insert(338, &[2105345]);     // 338 => [2105345]
            map.insert(194, &[49153]);     // 194 => [49153]
            map.insert(455, &[147457]);     // 455 => [147457]
            map.insert(392, &[278529]);     // 392 => [278529]
            map.insert(271, &[1064961]);     // 271 => [1064961]
            map.insert(275, &[98305]);     // 275 => [98305]
            map.insert(331, &[294913]);     // 331 => [294913]
            map.insert(507, &[557057]);     // 507 => [557057]
            map.insert(460, &[1081345]);     // 460 => [1081345]
            map.insert(394, &[2129921]);     // 394 => [2129921]
            map.insert(89, &[327681]);     // 89 => [327681]
            map.insert(233, &[589825]);     // 233 => [589825]
            map.insert(222, &[1114113]);     // 222 => [1114113]
            map.insert(78, &[393217]);     // 78 => [393217]
            map.insert(201, &[1179649]);     // 201 => [1179649]
            map.insert(177, &[786433]);     // 177 => [786433]
            map.insert(134, &[1310721]);     // 134 => [1310721]
            map.insert(54, &[1572865]);     // 54 => [1572865]
            map.insert(71, &[3145729]);     // 71 => [3145729]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(262, &[262]);     // 262 => [262]
            map.insert(485, &[1030]);     // 485 => [1030]
            map.insert(298, &[2054]);     // 298 => [2054]
            map.insert(425, &[8198]);     // 425 => [8198]
            map.insert(434, &[16390]);     // 434 => [16390]
            map.insert(139, &[524294]);     // 139 => [524294]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(138, &[138]);     // 138 => [138]
            map.insert(489, &[1034]);     // 489 => [1034]
            map.insert(294, &[2058]);     // 294 => [2058]
            map.insert(111, &[65546]);     // 111 => [65546]
            map.insert(246, &[2097162]);     // 246 => [2097162]
            map.insert(362, &[530]);     // 362 => [530]
            map.insert(497, &[1042]);     // 497 => [1042]
            map.insert(318, &[2066]);     // 318 => [2066]
            map.insert(47, &[262162]);     // 47 => [262162]
            map.insert(168, &[1048594]);     // 168 => [1048594]
            map.insert(238, &[2097170]);     // 238 => [2097170]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(346, &[546]);     // 346 => [546]
            map.insert(449, &[1058]);     // 449 => [1058]
            map.insert(341, &[32802]);     // 341 => [32802]
            map.insert(322, &[322]);     // 322 => [322]
            map.insert(314, &[578]);     // 314 => [578]
            map.insert(92, &[4162]);     // 92 => [4162]
            map.insert(493, &[8258]);     // 493 => [8258]
            map.insert(39, &[65602]);     // 39 => [65602]
            map.insert(127, &[262210]);     // 127 => [262210]
            map.insert(386, &[386]);     // 386 => [386]
            map.insert(353, &[1154]);     // 353 => [1154]
            map.insert(231, &[65666]);     // 231 => [65666]
            map.insert(240, &[131202]);     // 240 => [131202]
            map.insert(126, &[2097282]);     // 126 => [2097282]
            map.insert(225, &[1282]);     // 225 => [1282]
            map.insert(46, &[2306]);     // 46 => [2306]
            map.insert(182, &[16642]);     // 182 => [16642]
            map.insert(319, &[262402]);     // 319 => [262402]
            map.insert(440, &[1048834]);     // 440 => [1048834]
            map.insert(510, &[2097410]);     // 510 => [2097410]
            map.insert(213, &[8706]);     // 213 => [8706]
            map.insert(327, &[262658]);     // 327 => [262658]
            map.insert(448, &[1049090]);     // 448 => [1049090]
            map.insert(511, &[5122]);     // 511 => [5122]
            map.insert(150, &[33794]);     // 150 => [33794]
            map.insert(347, &[1049602]);     // 347 => [1049602]
            map.insert(304, &[6146]);     // 304 => [6146]
            map.insert(348, &[133122]);     // 348 => [133122]
            map.insert(466, &[2099202]);     // 466 => [2099202]
            map.insert(435, &[12290]);     // 435 => [12290]
            map.insert(424, &[20482]);     // 424 => [20482]
            map.insert(363, &[36866]);     // 363 => [36866]
            map.insert(110, &[135170]);     // 110 => [135170]
            map.insert(166, &[1052674]);     // 166 => [1052674]
            map.insert(224, &[2101250]);     // 224 => [2101250]
            map.insert(218, &[40962]);     // 218 => [40962]
            map.insert(456, &[73730]);     // 456 => [73730]
            map.insert(279, &[1056770]);     // 279 => [1056770]
            map.insert(337, &[2105346]);     // 337 => [2105346]
            map.insert(467, &[81922]);     // 467 => [81922]
            map.insert(452, &[147458]);     // 452 => [147458]
            map.insert(315, &[540674]);     // 315 => [540674]
            map.insert(263, &[163842]);     // 263 => [163842]
            map.insert(463, &[1081346]);     // 463 => [1081346]
            map.insert(234, &[589826]);     // 234 => [589826]
            map.insert(202, &[1179650]);     // 202 => [1179650]
            map.insert(105, &[65548]);     // 105 => [65548]
            map.insert(169, &[524324]);     // 169 => [524324]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(496, &[16452]);     // 496 => [16452]
            map.insert(299, &[8324]);     // 299 => [8324]
            map.insert(282, &[4356]);     // 282 => [4356]
            map.insert(211, &[8708]);     // 211 => [8708]
            map.insert(281, &[66052]);     // 281 => [66052]
            map.insert(203, &[3076]);     // 203 => [3076]
            map.insert(474, &[263172]);     // 474 => [263172]
            map.insert(349, &[1049604]);     // 349 => [1049604]
            map.insert(283, &[2098180]);     // 283 => [2098180]
            map.insert(333, &[67588]);     // 333 => [67588]
            map.insert(468, &[2099204]);     // 468 => [2099204]
            map.insert(104, &[135172]);     // 104 => [135172]
            map.insert(151, &[528388]);     // 151 => [528388]
            map.insert(230, &[2101252]);     // 230 => [2101252]
            map.insert(462, &[73732]);     // 462 => [73732]
            map.insert(473, &[139268]);     // 473 => [139268]
            map.insert(199, &[49156]);     // 199 => [49156]
            map.insert(469, &[81924]);     // 469 => [81924]
            map.insert(332, &[2113540]);     // 332 => [2113540]
            map.insert(278, &[98308]);     // 278 => [98308]
            map.insert(334, &[294916]);     // 334 => [294916]
            map.insert(457, &[1081348]);     // 457 => [1081348]
            map.insert(219, &[1114116]);     // 219 => [1114116]
            map.insert(75, &[393220]);     // 75 => [393220]
            map.insert(197, &[2359300]);     // 197 => [2359300]
            map.insert(280, &[280]);     // 280 => [280]
            map.insert(352, &[536]);     // 352 => [536]
            map.insert(106, &[131096]);     // 106 => [131096]
            map.insert(336, &[552]);     // 336 => [552]
            map.insert(412, &[16424]);     // 412 => [16424]
            map.insert(212, &[2097192]);     // 212 => [2097192]
            map.insert(295, &[8328]);     // 295 => [8328]
            map.insert(235, &[1288]);     // 235 => [1288]
            map.insert(167, &[8456]);     // 167 => [8456]
            map.insert(409, &[132104]);     // 409 => [132104]
            map.insert(470, &[263176]);     // 470 => [263176]
            map.insert(414, &[1050632]);     // 414 => [1050632]
            map.insert(472, &[2099208]);     // 472 => [2099208]
            map.insert(441, &[12296]);     // 441 => [12296]
            map.insert(43, &[266248]);     // 43 => [266248]
            map.insert(208, &[40968]);     // 208 => [40968]
            map.insert(410, &[270344]);     // 410 => [270344]
            map.insert(305, &[540680]);     // 305 => [540680]
            map.insert(453, &[1081352]);     // 453 => [1081352]
            map.insert(387, &[2129928]);     // 387 => [2129928]
            map.insert(247, &[655368]);     // 247 => [655368]
            map.insert(415, &[8240]);     // 415 => [8240]
            map.insert(484, &[16464]);     // 484 => [16464]
            map.insert(488, &[656]);     // 488 => [656]
            map.insert(413, &[524560]);     // 413 => [524560]
            map.insert(492, &[2097424]);     // 492 => [2097424]
            map.insert(107, &[69648]);     // 107 => [69648]
            map.insert(323, &[2105360]);     // 323 => [2105360]
            map.insert(411, &[2129936]);     // 411 => [2129936]
            map.insert(239, &[655376]);     // 239 => [655376]
            map.insert(209, &[2359312]);     // 209 => [2359312]
            map.insert(93, &[262240]);     // 93 => [262240]
            map.insert(471, &[32928]);     // 471 => [32928]
            map.insert(210, &[131232]);     // 210 => [131232]
            map.insert(326, &[4640]);     // 326 => [4640]
            map.insert(163, &[10272]);     // 163 => [10272]
            map.insert(183, &[1572992]);     // 183 => [1572992]
            map.insert(241, &[2621568]);     // 241 => [2621568]
            map.insert(198, &[3145856]);     // 198 => [3145856]
            map.insert(340, &[2816]);     // 340 => [2816]
            map.insert(408, &[18688]);     // 408 => [18688]
            map.insert(335, &[393472]);     // 335 => [393472]
            map.insert(475, &[263173]);     // 475 => [263173]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode22_13 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode22_13 {
    fn name(&self) -> String {
        "[22, 13] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        22
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
        codeword.truncate(13);
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
        let code = GuavaCode22_13.generator_matrix();
        assert_eq!(code.ncols(), 22);
        assert_eq!(code.nrows(), 13);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode22_13;
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
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, false, false, true, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, false, false, true, true, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, false, false, true, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, false, false, true, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, false, false, false, false, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, false, false, true, true, true, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, false, false, true, true, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false, false, false, true, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, true, true, false, false, true, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, false, false, false, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, true, false, false, false, false, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, false, false, true, false, false, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, false, true, true, true, false, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, true, false, true, false, true, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, false, false, true, true, false, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, false, true, false, false, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, false, false, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, false, false, false, true, true, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, true, false, true, true, false, false, true, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
