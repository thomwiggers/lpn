use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[21, 12]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode21_12;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1204225 ],
                &[ 1953794 ],
                &[ 1859588 ],
                &[ 610312 ],
                &[ 884752 ],
                &[ 1552416 ],
                &[ 184384 ],
                &[ 1831040 ],
                &[ 1372416 ],
                &[ 1446400 ],
                &[ 1557504 ],
                &[ 1738752 ],
                
            ], 21));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1017857 ],
                &[ 1506306 ],
                &[ 1973508 ],
                &[ 1796360 ],
                &[ 1910032 ],
                &[ 1511712 ],
                &[ 1761344 ],
                &[ 435584 ],
                &[ 446976 ],
                
            ], 21));
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
            map.insert(188, &[256]);     // 188 => [256]
            map.insert(256, &[512]);     // 256 => [512]
            map.insert(150, &[1024]);     // 150 => [1024]
            map.insert(15, &[2048]);     // 15 => [2048]
            map.insert(294, &[4096]);     // 294 => [4096]
            map.insert(218, &[8192]);     // 218 => [8192]
            map.insert(330, &[16384]);     // 330 => [16384]
            map.insert(451, &[32768]);     // 451 => [32768]
            map.insert(57, &[65536]);     // 57 => [65536]
            map.insert(495, &[131072]);     // 495 => [131072]
            map.insert(439, &[262144]);     // 439 => [262144]
            map.insert(93, &[524288]);     // 93 => [524288]
            map.insert(126, &[1048576]);     // 126 => [1048576]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(189, &[257]);     // 189 => [257]
            map.insert(257, &[513]);     // 257 => [513]
            map.insert(151, &[1025]);     // 151 => [1025]
            map.insert(14, &[2049]);     // 14 => [2049]
            map.insert(295, &[4097]);     // 295 => [4097]
            map.insert(219, &[8193]);     // 219 => [8193]
            map.insert(331, &[16385]);     // 331 => [16385]
            map.insert(450, &[32769]);     // 450 => [32769]
            map.insert(56, &[65537]);     // 56 => [65537]
            map.insert(494, &[131073]);     // 494 => [131073]
            map.insert(438, &[262145]);     // 438 => [262145]
            map.insert(92, &[524289]);     // 92 => [524289]
            map.insert(127, &[1048577]);     // 127 => [1048577]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(190, &[258]);     // 190 => [258]
            map.insert(258, &[514]);     // 258 => [514]
            map.insert(148, &[1026]);     // 148 => [1026]
            map.insert(13, &[2050]);     // 13 => [2050]
            map.insert(292, &[4098]);     // 292 => [4098]
            map.insert(216, &[8194]);     // 216 => [8194]
            map.insert(328, &[16386]);     // 328 => [16386]
            map.insert(449, &[32770]);     // 449 => [32770]
            map.insert(59, &[65538]);     // 59 => [65538]
            map.insert(493, &[131074]);     // 493 => [131074]
            map.insert(437, &[262146]);     // 437 => [262146]
            map.insert(95, &[524290]);     // 95 => [524290]
            map.insert(124, &[1048578]);     // 124 => [1048578]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(184, &[260]);     // 184 => [260]
            map.insert(260, &[516]);     // 260 => [516]
            map.insert(146, &[1028]);     // 146 => [1028]
            map.insert(11, &[2052]);     // 11 => [2052]
            map.insert(290, &[4100]);     // 290 => [4100]
            map.insert(222, &[8196]);     // 222 => [8196]
            map.insert(334, &[16388]);     // 334 => [16388]
            map.insert(455, &[32772]);     // 455 => [32772]
            map.insert(61, &[65540]);     // 61 => [65540]
            map.insert(491, &[131076]);     // 491 => [131076]
            map.insert(435, &[262148]);     // 435 => [262148]
            map.insert(89, &[524292]);     // 89 => [524292]
            map.insert(122, &[1048580]);     // 122 => [1048580]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(180, &[264]);     // 180 => [264]
            map.insert(264, &[520]);     // 264 => [520]
            map.insert(158, &[1032]);     // 158 => [1032]
            map.insert(7, &[2056]);     // 7 => [2056]
            map.insert(302, &[4104]);     // 302 => [4104]
            map.insert(210, &[8200]);     // 210 => [8200]
            map.insert(322, &[16392]);     // 322 => [16392]
            map.insert(459, &[32776]);     // 459 => [32776]
            map.insert(49, &[65544]);     // 49 => [65544]
            map.insert(487, &[131080]);     // 487 => [131080]
            map.insert(447, &[262152]);     // 447 => [262152]
            map.insert(85, &[524296]);     // 85 => [524296]
            map.insert(118, &[1048584]);     // 118 => [1048584]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(172, &[272]);     // 172 => [272]
            map.insert(272, &[528]);     // 272 => [528]
            map.insert(134, &[1040]);     // 134 => [1040]
            map.insert(31, &[2064]);     // 31 => [2064]
            map.insert(310, &[4112]);     // 310 => [4112]
            map.insert(202, &[8208]);     // 202 => [8208]
            map.insert(346, &[16400]);     // 346 => [16400]
            map.insert(467, &[32784]);     // 467 => [32784]
            map.insert(41, &[65552]);     // 41 => [65552]
            map.insert(511, &[131088]);     // 511 => [131088]
            map.insert(423, &[262160]);     // 423 => [262160]
            map.insert(77, &[524304]);     // 77 => [524304]
            map.insert(110, &[1048592]);     // 110 => [1048592]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(156, &[288]);     // 156 => [288]
            map.insert(288, &[544]);     // 288 => [544]
            map.insert(182, &[1056]);     // 182 => [1056]
            map.insert(47, &[2080]);     // 47 => [2080]
            map.insert(262, &[4128]);     // 262 => [4128]
            map.insert(250, &[8224]);     // 250 => [8224]
            map.insert(362, &[16416]);     // 362 => [16416]
            map.insert(483, &[32800]);     // 483 => [32800]
            map.insert(25, &[65568]);     // 25 => [65568]
            map.insert(463, &[131104]);     // 463 => [131104]
            map.insert(407, &[262176]);     // 407 => [262176]
            map.insert(125, &[524320]);     // 125 => [524320]
            map.insert(94, &[1048608]);     // 94 => [1048608]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(252, &[320]);     // 252 => [320]
            map.insert(320, &[576]);     // 320 => [576]
            map.insert(214, &[1088]);     // 214 => [1088]
            map.insert(79, &[2112]);     // 79 => [2112]
            map.insert(358, &[4160]);     // 358 => [4160]
            map.insert(154, &[8256]);     // 154 => [8256]
            map.insert(266, &[16448]);     // 266 => [16448]
            map.insert(387, &[32832]);     // 387 => [32832]
            map.insert(121, &[65600]);     // 121 => [65600]
            map.insert(431, &[131136]);     // 431 => [131136]
            map.insert(503, &[262208]);     // 503 => [262208]
            map.insert(29, &[524352]);     // 29 => [524352]
            map.insert(62, &[1048640]);     // 62 => [1048640]
            map.insert(60, &[384]);     // 60 => [384]
            map.insert(384, &[640]);     // 384 => [640]
            map.insert(22, &[1152]);     // 22 => [1152]
            map.insert(143, &[2176]);     // 143 => [2176]
            map.insert(422, &[4224]);     // 422 => [4224]
            map.insert(90, &[8320]);     // 90 => [8320]
            map.insert(458, &[16512]);     // 458 => [16512]
            map.insert(323, &[32896]);     // 323 => [32896]
            map.insert(185, &[65664]);     // 185 => [65664]
            map.insert(367, &[131200]);     // 367 => [131200]
            map.insert(311, &[262272]);     // 311 => [262272]
            map.insert(221, &[524416]);     // 221 => [524416]
            map.insert(254, &[1048704]);     // 254 => [1048704]
            map.insert(444, &[768]);     // 444 => [768]
            map.insert(42, &[1280]);     // 42 => [1280]
            map.insert(179, &[2304]);     // 179 => [2304]
            map.insert(410, &[4352]);     // 410 => [4352]
            map.insert(102, &[8448]);     // 102 => [8448]
            map.insert(502, &[16640]);     // 502 => [16640]
            map.insert(383, &[33024]);     // 383 => [33024]
            map.insert(133, &[65792]);     // 133 => [65792]
            map.insert(339, &[131328]);     // 339 => [131328]
            map.insert(267, &[262400]);     // 267 => [262400]
            map.insert(225, &[524544]);     // 225 => [524544]
            map.insert(194, &[1048832]);     // 194 => [1048832]
            map.insert(406, &[1536]);     // 406 => [1536]
            map.insert(271, &[2560]);     // 271 => [2560]
            map.insert(38, &[4608]);     // 38 => [4608]
            map.insert(474, &[8704]);     // 474 => [8704]
            map.insert(74, &[16896]);     // 74 => [16896]
            map.insert(195, &[33280]);     // 195 => [33280]
            map.insert(313, &[66048]);     // 313 => [66048]
            map.insert(239, &[131584]);     // 239 => [131584]
            map.insert(183, &[262656]);     // 183 => [262656]
            map.insert(349, &[524800]);     // 349 => [524800]
            map.insert(382, &[1049088]);     // 382 => [1049088]
            map.insert(153, &[3072]);     // 153 => [3072]
            map.insert(432, &[5120]);     // 432 => [5120]
            map.insert(76, &[9216]);     // 76 => [9216]
            map.insert(476, &[17408]);     // 476 => [17408]
            map.insert(341, &[33792]);     // 341 => [33792]
            map.insert(175, &[66560]);     // 175 => [66560]
            map.insert(377, &[132096]);     // 377 => [132096]
            map.insert(289, &[263168]);     // 289 => [263168]
            map.insert(203, &[525312]);     // 203 => [525312]
            map.insert(232, &[1049600]);     // 232 => [1049600]
            map.insert(297, &[6144]);     // 297 => [6144]
            map.insert(213, &[10240]);     // 213 => [10240]
            map.insert(325, &[18432]);     // 325 => [18432]
            map.insert(460, &[34816]);     // 460 => [34816]
            map.insert(54, &[67584]);     // 54 => [67584]
            map.insert(480, &[133120]);     // 480 => [133120]
            map.insert(440, &[264192]);     // 440 => [264192]
            map.insert(82, &[526336]);     // 82 => [526336]
            map.insert(113, &[1050624]);     // 113 => [1050624]
            map.insert(508, &[12288]);     // 508 => [12288]
            map.insert(108, &[20480]);     // 108 => [20480]
            map.insert(229, &[36864]);     // 229 => [36864]
            map.insert(287, &[69632]);     // 287 => [69632]
            map.insert(201, &[135168]);     // 201 => [135168]
            map.insert(145, &[266240]);     // 145 => [266240]
            map.insert(379, &[528384]);     // 379 => [528384]
            map.insert(344, &[1052672]);     // 344 => [1052672]
            map.insert(400, &[24576]);     // 400 => [24576]
            map.insert(281, &[40960]);     // 281 => [40960]
            map.insert(227, &[73728]);     // 227 => [73728]
            map.insert(309, &[139264]);     // 309 => [139264]
            map.insert(365, &[270336]);     // 365 => [270336]
            map.insert(135, &[532480]);     // 135 => [532480]
            map.insert(164, &[1056768]);     // 164 => [1056768]
            map.insert(137, &[49152]);     // 137 => [49152]
            map.insert(371, &[81920]);     // 371 => [81920]
            map.insert(165, &[147456]);     // 165 => [147456]
            map.insert(253, &[278528]);     // 253 => [278528]
            map.insert(279, &[540672]);     // 279 => [540672]
            map.insert(308, &[1064960]);     // 308 => [1064960]
            map.insert(506, &[98304]);     // 506 => [98304]
            map.insert(44, &[163840]);     // 44 => [163840]
            map.insert(116, &[294912]);     // 116 => [294912]
            map.insert(414, &[557056]);     // 414 => [557056]
            map.insert(445, &[1081344]);     // 445 => [1081344]
            map.insert(470, &[196608]);     // 470 => [196608]
            map.insert(398, &[327680]);     // 398 => [327680]
            map.insert(100, &[589824]);     // 100 => [589824]
            map.insert(71, &[1114112]);     // 71 => [1114112]
            map.insert(88, &[393216]);     // 88 => [393216]
            map.insert(434, &[655360]);     // 434 => [655360]
            map.insert(401, &[1179648]);     // 401 => [1179648]
            map.insert(490, &[786432]);     // 490 => [786432]
            map.insert(457, &[1310720]);     // 457 => [1310720]
            map.insert(35, &[1572864]);     // 35 => [1572864]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(191, &[259]);     // 191 => [259]
            map.insert(259, &[515]);     // 259 => [515]
            map.insert(149, &[1027]);     // 149 => [1027]
            map.insert(293, &[4099]);     // 293 => [4099]
            map.insert(217, &[8195]);     // 217 => [8195]
            map.insert(329, &[16387]);     // 329 => [16387]
            map.insert(448, &[32771]);     // 448 => [32771]
            map.insert(58, &[65539]);     // 58 => [65539]
            map.insert(492, &[131075]);     // 492 => [131075]
            map.insert(436, &[262147]);     // 436 => [262147]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(261, &[517]);     // 261 => [517]
            map.insert(147, &[1029]);     // 147 => [1029]
            map.insert(291, &[4101]);     // 291 => [4101]
            map.insert(223, &[8197]);     // 223 => [8197]
            map.insert(335, &[16389]);     // 335 => [16389]
            map.insert(454, &[32773]);     // 454 => [32773]
            map.insert(123, &[1048581]);     // 123 => [1048581]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(181, &[265]);     // 181 => [265]
            map.insert(265, &[521]);     // 265 => [521]
            map.insert(159, &[1033]);     // 159 => [1033]
            map.insert(303, &[4105]);     // 303 => [4105]
            map.insert(211, &[8201]);     // 211 => [8201]
            map.insert(486, &[131081]);     // 486 => [131081]
            map.insert(446, &[262153]);     // 446 => [262153]
            map.insert(84, &[524297]);     // 84 => [524297]
            map.insert(119, &[1048585]);     // 119 => [1048585]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(173, &[273]);     // 173 => [273]
            map.insert(273, &[529]);     // 273 => [529]
            map.insert(30, &[2065]);     // 30 => [2065]
            map.insert(347, &[16401]);     // 347 => [16401]
            map.insert(466, &[32785]);     // 466 => [32785]
            map.insert(510, &[131089]);     // 510 => [131089]
            map.insert(111, &[1048593]);     // 111 => [1048593]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(157, &[289]);     // 157 => [289]
            map.insert(46, &[2081]);     // 46 => [2081]
            map.insert(263, &[4129]);     // 263 => [4129]
            map.insert(251, &[8225]);     // 251 => [8225]
            map.insert(363, &[16417]);     // 363 => [16417]
            map.insert(482, &[32801]);     // 482 => [32801]
            map.insert(462, &[131105]);     // 462 => [131105]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(321, &[577]);     // 321 => [577]
            map.insert(215, &[1089]);     // 215 => [1089]
            map.insert(78, &[2113]);     // 78 => [2113]
            map.insert(359, &[4161]);     // 359 => [4161]
            map.insert(155, &[8257]);     // 155 => [8257]
            map.insert(386, &[32833]);     // 386 => [32833]
            map.insert(120, &[65601]);     // 120 => [65601]
            map.insert(430, &[131137]);     // 430 => [131137]
            map.insert(28, &[524353]);     // 28 => [524353]
            map.insert(63, &[1048641]);     // 63 => [1048641]
            map.insert(385, &[641]);     // 385 => [641]
            map.insert(23, &[1153]);     // 23 => [1153]
            map.insert(142, &[2177]);     // 142 => [2177]
            map.insert(91, &[8321]);     // 91 => [8321]
            map.insert(366, &[131201]);     // 366 => [131201]
            map.insert(220, &[524417]);     // 220 => [524417]
            map.insert(255, &[1048705]);     // 255 => [1048705]
            map.insert(43, &[1281]);     // 43 => [1281]
            map.insert(178, &[2305]);     // 178 => [2305]
            map.insert(411, &[4353]);     // 411 => [4353]
            map.insert(103, &[8449]);     // 103 => [8449]
            map.insert(338, &[131329]);     // 338 => [131329]
            map.insert(224, &[524545]);     // 224 => [524545]
            map.insert(270, &[2561]);     // 270 => [2561]
            map.insert(39, &[4609]);     // 39 => [4609]
            map.insert(475, &[8705]);     // 475 => [8705]
            map.insert(75, &[16897]);     // 75 => [16897]
            map.insert(312, &[66049]);     // 312 => [66049]
            map.insert(238, &[131585]);     // 238 => [131585]
            map.insert(348, &[524801]);     // 348 => [524801]
            map.insert(152, &[3073]);     // 152 => [3073]
            map.insert(433, &[5121]);     // 433 => [5121]
            map.insert(477, &[17409]);     // 477 => [17409]
            map.insert(340, &[33793]);     // 340 => [33793]
            map.insert(174, &[66561]);     // 174 => [66561]
            map.insert(376, &[132097]);     // 376 => [132097]
            map.insert(233, &[1049601]);     // 233 => [1049601]
            map.insert(296, &[6145]);     // 296 => [6145]
            map.insert(212, &[10241]);     // 212 => [10241]
            map.insert(324, &[18433]);     // 324 => [18433]
            map.insert(461, &[34817]);     // 461 => [34817]
            map.insert(55, &[67585]);     // 55 => [67585]
            map.insert(481, &[133121]);     // 481 => [133121]
            map.insert(441, &[264193]);     // 441 => [264193]
            map.insert(83, &[526337]);     // 83 => [526337]
            map.insert(112, &[1050625]);     // 112 => [1050625]
            map.insert(509, &[12289]);     // 509 => [12289]
            map.insert(109, &[20481]);     // 109 => [20481]
            map.insert(228, &[36865]);     // 228 => [36865]
            map.insert(286, &[69633]);     // 286 => [69633]
            map.insert(200, &[135169]);     // 200 => [135169]
            map.insert(378, &[528385]);     // 378 => [528385]
            map.insert(345, &[1052673]);     // 345 => [1052673]
            map.insert(280, &[40961]);     // 280 => [40961]
            map.insert(226, &[73729]);     // 226 => [73729]
            map.insert(364, &[270337]);     // 364 => [270337]
            map.insert(370, &[81921]);     // 370 => [81921]
            map.insert(278, &[540673]);     // 278 => [540673]
            map.insert(507, &[98305]);     // 507 => [98305]
            map.insert(45, &[163841]);     // 45 => [163841]
            map.insert(117, &[294913]);     // 117 => [294913]
            map.insert(415, &[557057]);     // 415 => [557057]
            map.insert(471, &[196609]);     // 471 => [196609]
            map.insert(399, &[327681]);     // 399 => [327681]
            map.insert(101, &[589825]);     // 101 => [589825]
            map.insert(70, &[1114113]);     // 70 => [1114113]
            map.insert(456, &[1310721]);     // 456 => [1310721]
            map.insert(186, &[262]);     // 186 => [262]
            map.insert(332, &[16390]);     // 332 => [16390]
            map.insert(453, &[32774]);     // 453 => [32774]
            map.insert(489, &[131078]);     // 489 => [131078]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(138, &[138]);     // 138 => [138]
            map.insert(300, &[4106]);     // 300 => [4106]
            map.insert(208, &[8202]);     // 208 => [8202]
            map.insert(51, &[65546]);     // 51 => [65546]
            map.insert(485, &[131082]);     // 485 => [131082]
            map.insert(87, &[524298]);     // 87 => [524298]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(274, &[530]);     // 274 => [530]
            map.insert(465, &[32786]);     // 465 => [32786]
            map.insert(421, &[262162]);     // 421 => [262162]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(248, &[8226]);     // 248 => [8226]
            map.insert(360, &[16418]);     // 360 => [16418]
            map.insert(27, &[65570]);     // 27 => [65570]
            map.insert(405, &[262178]);     // 405 => [262178]
            map.insert(356, &[4162]);     // 356 => [4162]
            map.insert(429, &[131138]);     // 429 => [131138]
            map.insert(501, &[262210]);     // 501 => [262210]
            map.insert(141, &[2178]);     // 141 => [2178]
            map.insert(420, &[4226]);     // 420 => [4226]
            map.insert(187, &[65666]);     // 187 => [65666]
            map.insert(177, &[2306]);     // 177 => [2306]
            map.insert(408, &[4354]);     // 408 => [4354]
            map.insert(500, &[16642]);     // 500 => [16642]
            map.insert(381, &[33026]);     // 381 => [33026]
            map.insert(337, &[131330]);     // 337 => [131330]
            map.insert(404, &[1538]);     // 404 => [1538]
            map.insert(269, &[2562]);     // 269 => [2562]
            map.insert(472, &[8706]);     // 472 => [8706]
            map.insert(315, &[66050]);     // 315 => [66050]
            map.insert(237, &[131586]);     // 237 => [131586]
            map.insert(351, &[524802]);     // 351 => [524802]
            map.insert(380, &[1049090]);     // 380 => [1049090]
            map.insert(478, &[17410]);     // 478 => [17410]
            map.insert(343, &[33794]);     // 343 => [33794]
            map.insert(234, &[1049602]);     // 234 => [1049602]
            map.insert(299, &[6146]);     // 299 => [6146]
            map.insert(327, &[18434]);     // 327 => [18434]
            map.insert(52, &[67586]);     // 52 => [67586]
            map.insert(442, &[264194]);     // 442 => [264194]
            map.insert(115, &[1050626]);     // 115 => [1050626]
            map.insert(231, &[36866]);     // 231 => [36866]
            map.insert(285, &[69634]);     // 285 => [69634]
            map.insert(402, &[24578]);     // 402 => [24578]
            map.insert(283, &[40962]);     // 283 => [40962]
            map.insert(166, &[1056770]);     // 166 => [1056770]
            map.insert(139, &[49154]);     // 139 => [49154]
            map.insert(369, &[81922]);     // 369 => [81922]
            map.insert(167, &[147458]);     // 167 => [147458]
            map.insert(277, &[540674]);     // 277 => [540674]
            map.insert(504, &[98306]);     // 504 => [98306]
            map.insert(412, &[557058]);     // 412 => [557058]
            map.insert(468, &[196610]);     // 468 => [196610]
            map.insert(396, &[327682]);     // 396 => [327682]
            map.insert(403, &[1179650]);     // 403 => [1179650]
            map.insert(488, &[786434]);     // 488 => [786434]
            map.insert(140, &[140]);     // 140 => [140]
            map.insert(176, &[268]);     // 176 => [268]
            map.insert(268, &[524]);     // 268 => [524]
            map.insert(298, &[4108]);     // 298 => [4108]
            map.insert(326, &[16396]);     // 326 => [16396]
            map.insert(53, &[65548]);     // 53 => [65548]
            map.insert(443, &[262156]);     // 443 => [262156]
            map.insert(114, &[1048588]);     // 114 => [1048588]
            map.insert(168, &[276]);     // 168 => [276]
            map.insert(276, &[532]);     // 276 => [532]
            map.insert(306, &[4116]);     // 306 => [4116]
            map.insert(206, &[8212]);     // 206 => [8212]
            map.insert(350, &[16404]);     // 350 => [16404]
            map.insert(419, &[262164]);     // 419 => [262164]
            map.insert(106, &[1048596]);     // 106 => [1048596]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(354, &[4164]);     // 354 => [4164]
            map.insert(391, &[32836]);     // 391 => [32836]
            map.insert(427, &[131140]);     // 427 => [131140]
            map.insert(499, &[262212]);     // 499 => [262212]
            map.insert(388, &[644]);     // 388 => [644]
            map.insert(418, &[4228]);     // 418 => [4228]
            map.insert(307, &[262276]);     // 307 => [262276]
            map.insert(498, &[16644]);     // 498 => [16644]
            map.insert(198, &[1048836]);     // 198 => [1048836]
            map.insert(199, &[33284]);     // 199 => [33284]
            map.insert(317, &[66052]);     // 317 => [66052]
            map.insert(235, &[131588]);     // 235 => [131588]
            map.insert(171, &[66564]);     // 171 => [66564]
            map.insert(207, &[525316]);     // 207 => [525316]
            map.insert(236, &[1049604]);     // 236 => [1049604]
            map.insert(301, &[6148]);     // 301 => [6148]
            map.insert(209, &[10244]);     // 209 => [10244]
            map.insert(484, &[133124]);     // 484 => [133124]
            map.insert(86, &[526340]);     // 86 => [526340]
            map.insert(104, &[20484]);     // 104 => [20484]
            map.insert(205, &[135172]);     // 205 => [135172]
            map.insert(305, &[139268]);     // 305 => [139268]
            map.insert(361, &[270340]);     // 361 => [270340]
            map.insert(375, &[81924]);     // 375 => [81924]
            map.insert(249, &[278532]);     // 249 => [278532]
            map.insert(275, &[540676]);     // 275 => [540676]
            map.insert(304, &[1064964]);     // 304 => [1064964]
            map.insert(394, &[327684]);     // 394 => [327684]
            map.insert(318, &[4120]);     // 318 => [4120]
            map.insert(242, &[8232]);     // 242 => [8232]
            map.insert(244, &[328]);     // 244 => [328]
            map.insert(395, &[32840]);     // 395 => [32840]
            map.insert(392, &[648]);     // 392 => [648]
            map.insert(319, &[262280]);     // 319 => [262280]
            map.insert(246, &[1048712]);     // 246 => [1048712]
            map.insert(374, &[1049096]);     // 374 => [1049096]
            map.insert(333, &[18440]);     // 333 => [18440]
            map.insert(452, &[34824]);     // 452 => [34824]
            map.insert(336, &[1052680]);     // 336 => [1052680]
            map.insert(357, &[270344]);     // 357 => [270344]
            map.insert(245, &[278536]);     // 245 => [278536]
            map.insert(316, &[1064968]);     // 316 => [1064968]
            map.insert(390, &[327688]);     // 390 => [327688]
            map.insert(409, &[1179656]);     // 409 => [1179656]
            map.insert(479, &[131120]);     // 479 => [131120]
            map.insert(282, &[16464]);     // 282 => [16464]
            map.insert(105, &[65616]);     // 105 => [65616]
            map.insert(169, &[65680]);     // 169 => [65680]
            map.insert(428, &[784]);     // 428 => [784]
            map.insert(163, &[2320]);     // 163 => [2320]
            map.insert(241, &[524560]);     // 241 => [524560]
            map.insert(416, &[5136]);     // 416 => [5136]
            map.insert(197, &[10256]);     // 197 => [10256]
            map.insert(496, &[133136]);     // 496 => [133136]
            map.insert(424, &[264208]);     // 424 => [264208]
            map.insert(243, &[73744]);     // 243 => [73744]
            map.insert(355, &[81936]);     // 355 => [81936]
            map.insert(473, &[1310736]);     // 473 => [1310736]
            map.insert(352, &[608]);     // 352 => [608]
            map.insert(373, &[33824]);     // 373 => [33824]
            map.insert(413, &[1081376]);     // 413 => [1081376]
            map.insert(247, &[262720]);     // 247 => [262720]
            map.insert(353, &[263232]);     // 353 => [263232]
            map.insert(464, &[24640]);     // 464 => [24640]
            map.insert(372, &[1065024]);     // 372 => [1065024]
            map.insert(426, &[786496]);     // 426 => [786496]
            map.insert(393, &[1310784]);     // 393 => [1310784]
            map.insert(99, &[1572928]);     // 99 => [1572928]
            map.insert(170, &[1408]);     // 170 => [1408]
            map.insert(230, &[8576]);     // 230 => [8576]
            map.insert(204, &[9344]);     // 204 => [9344]
            map.insert(469, &[33920]);     // 469 => [33920]
            map.insert(505, &[132224]);     // 505 => [132224]
            map.insert(417, &[263296]);     // 417 => [263296]
            map.insert(425, &[6272]);     // 425 => [6272]
            map.insert(342, &[196736]);     // 342 => [196736]
            map.insert(389, &[66304]);     // 389 => [66304]
            map.insert(240, &[9472]);     // 240 => [9472]
            map.insert(368, &[35072]);     // 368 => [35072]
            map.insert(107, &[279552]);     // 107 => [279552]
            map.insert(314, &[141312]);     // 314 => [141312]
            map.insert(284, &[69635]);     // 284 => [69635]
            map.insert(397, &[327683]);     // 397 => [327683]
            map.insert(497, &[133137]);     // 497 => [133137]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode21_12 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode21_12 {
    fn name(&self) -> String {
        "[21, 12] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        21
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
        codeword.truncate(12);
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
        let code = GuavaCode21_12.generator_matrix();
        assert_eq!(code.ncols(), 21);
        assert_eq!(code.nrows(), 12);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode21_12;
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
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, false, false, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, false, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, false, true, true, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, false, false, false, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, false, false, false, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, false, true, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, true, true, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, false, true, false, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, true, true, true, true, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, true, false, true, false, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, true, false, true, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, true, true, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, true, true, true, false, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, false, true, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false, false, false, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, false, false, true, false, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, true, true, false, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, false, true, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, false, true, false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
