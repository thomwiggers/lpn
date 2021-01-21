use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[20, 11]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode20_11;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 976897 ],
                &[ 929794 ],
                &[ 305156 ],
                &[ 442376 ],
                &[ 776208 ],
                &[ 92192 ],
                &[ 915520 ],
                &[ 686208 ],
                &[ 723200 ],
                &[ 778752 ],
                &[ 869376 ],
                
            ], 20));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 834049 ],
                &[ 576130 ],
                &[ 684164 ],
                &[ 955016 ],
                &[ 755856 ],
                &[ 880672 ],
                &[ 217792 ],
                &[ 223488 ],
                &[ 508928 ],
                
            ], 20));
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
            map.insert(94, &[128]);     // 94 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(75, &[512]);     // 75 => [512]
            map.insert(256, &[1024]);     // 256 => [1024]
            map.insert(147, &[2048]);     // 147 => [2048]
            map.insert(109, &[4096]);     // 109 => [4096]
            map.insert(165, &[8192]);     // 165 => [8192]
            map.insert(486, &[16384]);     // 486 => [16384]
            map.insert(283, &[32768]);     // 283 => [32768]
            map.insert(496, &[65536]);     // 496 => [65536]
            map.insert(476, &[131072]);     // 476 => [131072]
            map.insert(297, &[262144]);     // 297 => [262144]
            map.insert(63, &[524288]);     // 63 => [524288]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(95, &[129]);     // 95 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(74, &[513]);     // 74 => [513]
            map.insert(257, &[1025]);     // 257 => [1025]
            map.insert(146, &[2049]);     // 146 => [2049]
            map.insert(108, &[4097]);     // 108 => [4097]
            map.insert(164, &[8193]);     // 164 => [8193]
            map.insert(487, &[16385]);     // 487 => [16385]
            map.insert(282, &[32769]);     // 282 => [32769]
            map.insert(497, &[65537]);     // 497 => [65537]
            map.insert(477, &[131073]);     // 477 => [131073]
            map.insert(296, &[262145]);     // 296 => [262145]
            map.insert(62, &[524289]);     // 62 => [524289]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(92, &[130]);     // 92 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(73, &[514]);     // 73 => [514]
            map.insert(258, &[1026]);     // 258 => [1026]
            map.insert(145, &[2050]);     // 145 => [2050]
            map.insert(111, &[4098]);     // 111 => [4098]
            map.insert(167, &[8194]);     // 167 => [8194]
            map.insert(484, &[16386]);     // 484 => [16386]
            map.insert(281, &[32770]);     // 281 => [32770]
            map.insert(498, &[65538]);     // 498 => [65538]
            map.insert(478, &[131074]);     // 478 => [131074]
            map.insert(299, &[262146]);     // 299 => [262146]
            map.insert(61, &[524290]);     // 61 => [524290]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(90, &[132]);     // 90 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(79, &[516]);     // 79 => [516]
            map.insert(260, &[1028]);     // 260 => [1028]
            map.insert(151, &[2052]);     // 151 => [2052]
            map.insert(105, &[4100]);     // 105 => [4100]
            map.insert(161, &[8196]);     // 161 => [8196]
            map.insert(482, &[16388]);     // 482 => [16388]
            map.insert(287, &[32772]);     // 287 => [32772]
            map.insert(500, &[65540]);     // 500 => [65540]
            map.insert(472, &[131076]);     // 472 => [131076]
            map.insert(301, &[262148]);     // 301 => [262148]
            map.insert(59, &[524292]);     // 59 => [524292]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(86, &[136]);     // 86 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(67, &[520]);     // 67 => [520]
            map.insert(264, &[1032]);     // 264 => [1032]
            map.insert(155, &[2056]);     // 155 => [2056]
            map.insert(101, &[4104]);     // 101 => [4104]
            map.insert(173, &[8200]);     // 173 => [8200]
            map.insert(494, &[16392]);     // 494 => [16392]
            map.insert(275, &[32776]);     // 275 => [32776]
            map.insert(504, &[65544]);     // 504 => [65544]
            map.insert(468, &[131080]);     // 468 => [131080]
            map.insert(289, &[262152]);     // 289 => [262152]
            map.insert(55, &[524296]);     // 55 => [524296]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(78, &[144]);     // 78 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(91, &[528]);     // 91 => [528]
            map.insert(272, &[1040]);     // 272 => [1040]
            map.insert(131, &[2064]);     // 131 => [2064]
            map.insert(125, &[4112]);     // 125 => [4112]
            map.insert(181, &[8208]);     // 181 => [8208]
            map.insert(502, &[16400]);     // 502 => [16400]
            map.insert(267, &[32784]);     // 267 => [32784]
            map.insert(480, &[65552]);     // 480 => [65552]
            map.insert(460, &[131088]);     // 460 => [131088]
            map.insert(313, &[262160]);     // 313 => [262160]
            map.insert(47, &[524304]);     // 47 => [524304]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(126, &[160]);     // 126 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(107, &[544]);     // 107 => [544]
            map.insert(288, &[1056]);     // 288 => [1056]
            map.insert(179, &[2080]);     // 179 => [2080]
            map.insert(77, &[4128]);     // 77 => [4128]
            map.insert(133, &[8224]);     // 133 => [8224]
            map.insert(454, &[16416]);     // 454 => [16416]
            map.insert(315, &[32800]);     // 315 => [32800]
            map.insert(464, &[65568]);     // 464 => [65568]
            map.insert(508, &[131104]);     // 508 => [131104]
            map.insert(265, &[262176]);     // 265 => [262176]
            map.insert(31, &[524320]);     // 31 => [524320]
            map.insert(30, &[192]);     // 30 => [192]
            map.insert(192, &[320]);     // 192 => [320]
            map.insert(11, &[576]);     // 11 => [576]
            map.insert(320, &[1088]);     // 320 => [1088]
            map.insert(211, &[2112]);     // 211 => [2112]
            map.insert(45, &[4160]);     // 45 => [4160]
            map.insert(229, &[8256]);     // 229 => [8256]
            map.insert(422, &[16448]);     // 422 => [16448]
            map.insert(347, &[32832]);     // 347 => [32832]
            map.insert(432, &[65600]);     // 432 => [65600]
            map.insert(412, &[131136]);     // 412 => [131136]
            map.insert(361, &[262208]);     // 361 => [262208]
            map.insert(127, &[524352]);     // 127 => [524352]
            map.insert(222, &[384]);     // 222 => [384]
            map.insert(21, &[640]);     // 21 => [640]
            map.insert(350, &[1152]);     // 350 => [1152]
            map.insert(205, &[2176]);     // 205 => [2176]
            map.insert(51, &[4224]);     // 51 => [4224]
            map.insert(251, &[8320]);     // 251 => [8320]
            map.insert(440, &[16512]);     // 440 => [16512]
            map.insert(325, &[32896]);     // 325 => [32896]
            map.insert(430, &[65664]);     // 430 => [65664]
            map.insert(386, &[131200]);     // 386 => [131200]
            map.insert(375, &[262272]);     // 375 => [262272]
            map.insert(97, &[524416]);     // 97 => [524416]
            map.insert(203, &[768]);     // 203 => [768]
            map.insert(384, &[1280]);     // 384 => [1280]
            map.insert(19, &[2304]);     // 19 => [2304]
            map.insert(237, &[4352]);     // 237 => [4352]
            map.insert(37, &[8448]);     // 37 => [8448]
            map.insert(358, &[16640]);     // 358 => [16640]
            map.insert(411, &[33024]);     // 411 => [33024]
            map.insert(368, &[65792]);     // 368 => [65792]
            map.insert(348, &[131328]);     // 348 => [131328]
            map.insert(425, &[262400]);     // 425 => [262400]
            map.insert(191, &[524544]);     // 191 => [524544]
            map.insert(331, &[1536]);     // 331 => [1536]
            map.insert(216, &[2560]);     // 216 => [2560]
            map.insert(38, &[4608]);     // 38 => [4608]
            map.insert(238, &[8704]);     // 238 => [8704]
            map.insert(429, &[16896]);     // 429 => [16896]
            map.insert(336, &[33280]);     // 336 => [33280]
            map.insert(443, &[66048]);     // 443 => [66048]
            map.insert(407, &[131584]);     // 407 => [131584]
            map.insert(354, &[262656]);     // 354 => [262656]
            map.insert(116, &[524800]);     // 116 => [524800]
            map.insert(403, &[3072]);     // 403 => [3072]
            map.insert(365, &[5120]);     // 365 => [5120]
            map.insert(421, &[9216]);     // 421 => [9216]
            map.insert(230, &[17408]);     // 230 => [17408]
            map.insert(27, &[33792]);     // 27 => [33792]
            map.insert(240, &[66560]);     // 240 => [66560]
            map.insert(220, &[132096]);     // 220 => [132096]
            map.insert(41, &[263168]);     // 41 => [263168]
            map.insert(319, &[525312]);     // 319 => [525312]
            map.insert(254, &[6144]);     // 254 => [6144]
            map.insert(54, &[10240]);     // 54 => [10240]
            map.insert(373, &[18432]);     // 373 => [18432]
            map.insert(392, &[34816]);     // 392 => [34816]
            map.insert(355, &[67584]);     // 355 => [67584]
            map.insert(335, &[133120]);     // 335 => [133120]
            map.insert(442, &[264192]);     // 442 => [264192]
            map.insert(172, &[526336]);     // 172 => [526336]
            map.insert(200, &[12288]);     // 200 => [12288]
            map.insert(395, &[20480]);     // 395 => [20480]
            map.insert(374, &[36864]);     // 374 => [36864]
            map.insert(413, &[69632]);     // 413 => [69632]
            map.insert(433, &[135168]);     // 433 => [135168]
            map.insert(324, &[266240]);     // 324 => [266240]
            map.insert(82, &[528384]);     // 82 => [528384]
            map.insert(323, &[24576]);     // 323 => [24576]
            map.insert(446, &[40960]);     // 446 => [40960]
            map.insert(341, &[73728]);     // 341 => [73728]
            map.insert(377, &[139264]);     // 377 => [139264]
            map.insert(396, &[270336]);     // 396 => [270336]
            map.insert(154, &[532480]);     // 154 => [532480]
            map.insert(253, &[49152]);     // 253 => [49152]
            map.insert(22, &[81920]);     // 22 => [81920]
            map.insert(58, &[147456]);     // 58 => [147456]
            map.insert(207, &[278528]);     // 207 => [278528]
            map.insert(473, &[540672]);     // 473 => [540672]
            map.insert(235, &[98304]);     // 235 => [98304]
            map.insert(199, &[163840]);     // 199 => [163840]
            map.insert(50, &[294912]);     // 50 => [294912]
            map.insert(292, &[557056]);     // 292 => [557056]
            map.insert(44, &[196608]);     // 44 => [196608]
            map.insert(217, &[327680]);     // 217 => [327680]
            map.insert(463, &[589824]);     // 463 => [589824]
            map.insert(245, &[393216]);     // 245 => [393216]
            map.insert(483, &[655360]);     // 483 => [655360]
            map.insert(278, &[786432]);     // 278 => [786432]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(93, &[131]);     // 93 => [131]
            map.insert(259, &[1027]);     // 259 => [1027]
            map.insert(110, &[4099]);     // 110 => [4099]
            map.insert(166, &[8195]);     // 166 => [8195]
            map.insert(485, &[16387]);     // 485 => [16387]
            map.insert(280, &[32771]);     // 280 => [32771]
            map.insert(499, &[65539]);     // 499 => [65539]
            map.insert(479, &[131075]);     // 479 => [131075]
            map.insert(298, &[262147]);     // 298 => [262147]
            map.insert(60, &[524291]);     // 60 => [524291]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(261, &[1029]);     // 261 => [1029]
            map.insert(150, &[2053]);     // 150 => [2053]
            map.insert(104, &[4101]);     // 104 => [4101]
            map.insert(286, &[32773]);     // 286 => [32773]
            map.insert(501, &[65541]);     // 501 => [65541]
            map.insert(300, &[262149]);     // 300 => [262149]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(87, &[137]);     // 87 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(100, &[4105]);     // 100 => [4105]
            map.insert(495, &[16393]);     // 495 => [16393]
            map.insert(274, &[32777]);     // 274 => [32777]
            map.insert(505, &[65545]);     // 505 => [65545]
            map.insert(469, &[131081]);     // 469 => [131081]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(273, &[1041]);     // 273 => [1041]
            map.insert(124, &[4113]);     // 124 => [4113]
            map.insert(180, &[8209]);     // 180 => [8209]
            map.insert(503, &[16401]);     // 503 => [16401]
            map.insert(266, &[32785]);     // 266 => [32785]
            map.insert(481, &[65553]);     // 481 => [65553]
            map.insert(461, &[131089]);     // 461 => [131089]
            map.insert(312, &[262161]);     // 312 => [262161]
            map.insert(46, &[524305]);     // 46 => [524305]
            map.insert(106, &[545]);     // 106 => [545]
            map.insert(178, &[2081]);     // 178 => [2081]
            map.insert(76, &[4129]);     // 76 => [4129]
            map.insert(455, &[16417]);     // 455 => [16417]
            map.insert(314, &[32801]);     // 314 => [32801]
            map.insert(465, &[65569]);     // 465 => [65569]
            map.insert(509, &[131105]);     // 509 => [131105]
            map.insert(193, &[321]);     // 193 => [321]
            map.insert(321, &[1089]);     // 321 => [1089]
            map.insert(210, &[2113]);     // 210 => [2113]
            map.insert(228, &[8257]);     // 228 => [8257]
            map.insert(423, &[16449]);     // 423 => [16449]
            map.insert(346, &[32833]);     // 346 => [32833]
            map.insert(360, &[262209]);     // 360 => [262209]
            map.insert(223, &[385]);     // 223 => [385]
            map.insert(351, &[1153]);     // 351 => [1153]
            map.insert(204, &[2177]);     // 204 => [2177]
            map.insert(250, &[8321]);     // 250 => [8321]
            map.insert(441, &[16513]);     // 441 => [16513]
            map.insert(431, &[65665]);     // 431 => [65665]
            map.insert(387, &[131201]);     // 387 => [131201]
            map.insert(202, &[769]);     // 202 => [769]
            map.insert(385, &[1281]);     // 385 => [1281]
            map.insert(236, &[4353]);     // 236 => [4353]
            map.insert(359, &[16641]);     // 359 => [16641]
            map.insert(410, &[33025]);     // 410 => [33025]
            map.insert(369, &[65793]);     // 369 => [65793]
            map.insert(349, &[131329]);     // 349 => [131329]
            map.insert(424, &[262401]);     // 424 => [262401]
            map.insert(190, &[524545]);     // 190 => [524545]
            map.insert(330, &[1537]);     // 330 => [1537]
            map.insert(39, &[4609]);     // 39 => [4609]
            map.insert(239, &[8705]);     // 239 => [8705]
            map.insert(428, &[16897]);     // 428 => [16897]
            map.insert(337, &[33281]);     // 337 => [33281]
            map.insert(406, &[131585]);     // 406 => [131585]
            map.insert(117, &[524801]);     // 117 => [524801]
            map.insert(402, &[3073]);     // 402 => [3073]
            map.insert(364, &[5121]);     // 364 => [5121]
            map.insert(420, &[9217]);     // 420 => [9217]
            map.insert(231, &[17409]);     // 231 => [17409]
            map.insert(26, &[33793]);     // 26 => [33793]
            map.insert(241, &[66561]);     // 241 => [66561]
            map.insert(221, &[132097]);     // 221 => [132097]
            map.insert(318, &[525313]);     // 318 => [525313]
            map.insert(255, &[6145]);     // 255 => [6145]
            map.insert(372, &[18433]);     // 372 => [18433]
            map.insert(393, &[34817]);     // 393 => [34817]
            map.insert(334, &[133121]);     // 334 => [133121]
            map.insert(201, &[12289]);     // 201 => [12289]
            map.insert(394, &[20481]);     // 394 => [20481]
            map.insert(83, &[528385]);     // 83 => [528385]
            map.insert(322, &[24577]);     // 322 => [24577]
            map.insert(447, &[40961]);     // 447 => [40961]
            map.insert(340, &[73729]);     // 340 => [73729]
            map.insert(376, &[139265]);     // 376 => [139265]
            map.insert(397, &[270337]);     // 397 => [270337]
            map.insert(252, &[49153]);     // 252 => [49153]
            map.insert(23, &[81921]);     // 23 => [81921]
            map.insert(206, &[278529]);     // 206 => [278529]
            map.insert(234, &[98305]);     // 234 => [98305]
            map.insert(198, &[163841]);     // 198 => [163841]
            map.insert(293, &[557057]);     // 293 => [557057]
            map.insert(462, &[589825]);     // 462 => [589825]
            map.insert(244, &[393217]);     // 244 => [393217]
            map.insert(279, &[786433]);     // 279 => [786433]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(88, &[134]);     // 88 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(262, &[1030]);     // 262 => [1030]
            map.insert(149, &[2054]);     // 149 => [2054]
            map.insert(163, &[8198]);     // 163 => [8198]
            map.insert(285, &[32774]);     // 285 => [32774]
            map.insert(474, &[131078]);     // 474 => [131078]
            map.insert(303, &[262150]);     // 303 => [262150]
            map.insert(57, &[524294]);     // 57 => [524294]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(84, &[138]);     // 84 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(153, &[2058]);     // 153 => [2058]
            map.insert(103, &[4106]);     // 103 => [4106]
            map.insert(175, &[8202]);     // 175 => [8202]
            map.insert(492, &[16394]);     // 492 => [16394]
            map.insert(506, &[65546]);     // 506 => [65546]
            map.insert(470, &[131082]);     // 470 => [131082]
            map.insert(291, &[262154]);     // 291 => [262154]
            map.insert(53, &[524298]);     // 53 => [524298]
            map.insert(89, &[530]);     // 89 => [530]
            map.insert(183, &[8210]);     // 183 => [8210]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(290, &[1058]);     // 290 => [1058]
            map.insert(177, &[2082]);     // 177 => [2082]
            map.insert(135, &[8226]);     // 135 => [8226]
            map.insert(452, &[16418]);     // 452 => [16418]
            map.insert(466, &[65570]);     // 466 => [65570]
            map.insert(510, &[131106]);     // 510 => [131106]
            map.insert(29, &[524322]);     // 29 => [524322]
            map.insert(28, &[194]);     // 28 => [194]
            map.insert(194, &[322]);     // 194 => [322]
            map.insert(209, &[2114]);     // 209 => [2114]
            map.insert(345, &[32834]);     // 345 => [32834]
            map.insert(434, &[65602]);     // 434 => [65602]
            map.insert(414, &[131138]);     // 414 => [131138]
            map.insert(363, &[262210]);     // 363 => [262210]
            map.insert(249, &[8322]);     // 249 => [8322]
            map.insert(327, &[32898]);     // 327 => [32898]
            map.insert(99, &[524418]);     // 99 => [524418]
            map.insert(356, &[16642]);     // 356 => [16642]
            map.insert(409, &[33026]);     // 409 => [33026]
            map.insert(370, &[65794]);     // 370 => [65794]
            map.insert(427, &[262402]);     // 427 => [262402]
            map.insert(189, &[524546]);     // 189 => [524546]
            map.insert(329, &[1538]);     // 329 => [1538]
            map.insert(218, &[2562]);     // 218 => [2562]
            map.insert(338, &[33282]);     // 338 => [33282]
            map.insert(405, &[131586]);     // 405 => [131586]
            map.insert(352, &[262658]);     // 352 => [262658]
            map.insert(118, &[524802]);     // 118 => [524802]
            map.insert(401, &[3074]);     // 401 => [3074]
            map.insert(367, &[5122]);     // 367 => [5122]
            map.insert(242, &[66562]);     // 242 => [66562]
            map.insert(43, &[263170]);     // 43 => [263170]
            map.insert(317, &[525314]);     // 317 => [525314]
            map.insert(52, &[10242]);     // 52 => [10242]
            map.insert(353, &[67586]);     // 353 => [67586]
            map.insert(333, &[133122]);     // 333 => [133122]
            map.insert(174, &[526338]);     // 174 => [526338]
            map.insert(415, &[69634]);     // 415 => [69634]
            map.insert(435, &[135170]);     // 435 => [135170]
            map.insert(326, &[266242]);     // 326 => [266242]
            map.insert(444, &[40962]);     // 444 => [40962]
            map.insert(343, &[73730]);     // 343 => [73730]
            map.insert(379, &[139266]);     // 379 => [139266]
            map.insert(398, &[270338]);     // 398 => [270338]
            map.insert(152, &[532482]);     // 152 => [532482]
            map.insert(56, &[147458]);     // 56 => [147458]
            map.insert(475, &[540674]);     // 475 => [540674]
            map.insert(233, &[98306]);     // 233 => [98306]
            map.insert(197, &[163842]);     // 197 => [163842]
            map.insert(294, &[557058]);     // 294 => [557058]
            map.insert(219, &[327682]);     // 219 => [327682]
            map.insert(247, &[393218]);     // 247 => [393218]
            map.insert(276, &[786434]);     // 276 => [786434]
            map.insert(140, &[268]);     // 140 => [268]
            map.insert(71, &[524]);     // 71 => [524]
            map.insert(268, &[1036]);     // 268 => [1036]
            map.insert(159, &[2060]);     // 159 => [2060]
            map.insert(169, &[8204]);     // 169 => [8204]
            map.insert(490, &[16396]);     // 490 => [16396]
            map.insert(148, &[276]);     // 148 => [276]
            map.insert(121, &[4116]);     // 121 => [4116]
            map.insert(271, &[32788]);     // 271 => [32788]
            map.insert(456, &[131092]);     // 456 => [131092]
            map.insert(122, &[164]);     // 122 => [164]
            map.insert(450, &[16420]);     // 450 => [16420]
            map.insert(269, &[262180]);     // 269 => [262180]
            map.insert(196, &[324]);     // 196 => [324]
            map.insert(15, &[580]);     // 15 => [580]
            map.insert(215, &[2116]);     // 215 => [2116]
            map.insert(225, &[8260]);     // 225 => [8260]
            map.insert(418, &[16452]);     // 418 => [16452]
            map.insert(436, &[65604]);     // 436 => [65604]
            map.insert(408, &[131140]);     // 408 => [131140]
            map.insert(123, &[524356]);     // 123 => [524356]
            map.insert(426, &[65668]);     // 426 => [65668]
            map.insert(390, &[131204]);     // 390 => [131204]
            map.insert(371, &[262276]);     // 371 => [262276]
            map.insert(388, &[1284]);     // 388 => [1284]
            map.insert(344, &[131332]);     // 344 => [131332]
            map.insert(187, &[524548]);     // 187 => [524548]
            map.insert(112, &[524804]);     // 112 => [524804]
            map.insert(417, &[9220]);     // 417 => [9220]
            map.insert(226, &[17412]);     // 226 => [17412]
            map.insert(168, &[526340]);     // 168 => [526340]
            map.insert(399, &[20484]);     // 399 => [20484]
            map.insert(437, &[135172]);     // 437 => [135172]
            map.insert(381, &[139268]);     // 381 => [139268]
            map.insert(158, &[532484]);     // 158 => [532484]
            map.insert(195, &[163844]);     // 195 => [163844]
            map.insert(459, &[589828]);     // 459 => [589828]
            map.insert(139, &[2072]);     // 139 => [2072]
            map.insert(488, &[65560]);     // 488 => [65560]
            map.insert(305, &[262168]);     // 305 => [262168]
            map.insert(141, &[8232]);     // 141 => [8232]
            map.insert(307, &[32808]);     // 307 => [32808]
            map.insert(328, &[1096]);     // 328 => [1096]
            map.insert(339, &[32840]);     // 339 => [32840]
            map.insert(404, &[131144]);     // 404 => [131144]
            map.insert(119, &[524360]);     // 119 => [524360]
            map.insert(214, &[392]);     // 214 => [392]
            map.insert(342, &[1160]);     // 342 => [1160]
            map.insert(243, &[8328]);     // 243 => [8328]
            map.insert(383, &[262280]);     // 383 => [262280]
            map.insert(366, &[16648]);     // 366 => [16648]
            map.insert(208, &[2568]);     // 208 => [2568]
            map.insert(362, &[262664]);     // 362 => [262664]
            map.insert(357, &[5128]);     // 357 => [5128]
            map.insert(248, &[66568]);     // 248 => [66568]
            map.insert(212, &[132104]);     // 212 => [132104]
            map.insert(311, &[525320]);     // 311 => [525320]
            map.insert(246, &[6152]);     // 246 => [6152]
            map.insert(382, &[36872]);     // 382 => [36872]
            map.insert(332, &[266248]);     // 332 => [266248]
            map.insert(438, &[40968]);     // 438 => [40968]
            map.insert(227, &[98312]);     // 227 => [98312]
            map.insert(491, &[655368]);     // 491 => [655368]
            map.insert(176, &[304]);     // 176 => [304]
            map.insert(304, &[1072]);     // 304 => [1072]
            map.insert(448, &[65584]);     // 448 => [65584]
            map.insert(416, &[65616]);     // 416 => [65616]
            map.insert(113, &[524432]);     // 113 => [524432]
            map.insert(400, &[1296]);     // 400 => [1296]
            map.insert(445, &[16912]);     // 445 => [16912]
            map.insert(391, &[131600]);     // 391 => [131600]
            map.insert(224, &[66576]);     // 224 => [66576]
            map.insert(188, &[526352]);     // 188 => [526352]
            map.insert(457, &[540688]);     // 457 => [540688]
            map.insert(308, &[557072]);     // 308 => [557072]
            map.insert(380, &[131360]);     // 380 => [131360]
            map.insert(439, &[131616]);     // 439 => [131616]
            map.insert(389, &[9248]);     // 389 => [9248]
            map.insert(232, &[12320]);     // 232 => [12320]
            map.insert(114, &[528416]);     // 114 => [528416]
            map.insert(186, &[532512]);     // 186 => [532512]
            map.insert(213, &[393248]);     // 213 => [393248]
            map.insert(451, &[655392]);     // 451 => [655392]
            map.insert(310, &[786464]);     // 310 => [786464]
            map.insert(85, &[704]);     // 85 => [704]
            map.insert(115, &[4288]);     // 115 => [4288]
            map.insert(284, &[131392]);     // 284 => [131392]
            map.insert(489, &[262464]);     // 489 => [262464]
            map.insert(102, &[4672]);     // 102 => [4672]
            map.insert(493, &[16960]);     // 493 => [16960]
            map.insert(507, &[66112]);     // 507 => [66112]
            map.insert(471, &[131648]);     // 471 => [131648]
            map.insert(467, &[3136]);     // 467 => [3136]
            map.insert(156, &[132160]);     // 156 => [132160]
            map.insert(309, &[18496]);     // 309 => [18496]
            map.insert(277, &[73792]);     // 277 => [73792]
            map.insert(143, &[278592]);     // 143 => [278592]
            map.insert(171, &[98368]);     // 171 => [98368]
            map.insert(419, &[655424]);     // 419 => [655424]
            map.insert(453, &[33152]);     // 453 => [33152]
            map.insert(302, &[65920]);     // 302 => [65920]
            map.insert(120, &[4736]);     // 120 => [4736]
            map.insert(270, &[33408]);     // 270 => [33408]
            map.insert(316, &[262784]);     // 316 => [262784]
            map.insert(184, &[17536]);     // 184 => [17536]
            map.insert(295, &[139392]);     // 295 => [139392]
            map.insert(378, &[557184]);     // 378 => [557184]
            map.insert(182, &[10496]);     // 182 => [10496]
            map.insert(306, &[139776]);     // 306 => [139776]
            map.insert(157, &[70656]);     // 157 => [70656]
            map.insert(170, &[167936]);     // 170 => [167936]
            map.insert(458, &[212992]);     // 458 => [212992]
            map.insert(263, &[1031]);     // 263 => [1031]
            map.insert(511, &[131107]);     // 511 => [131107]
            map.insert(449, &[65585]);     // 449 => [65585]
            map.insert(142, &[278593]);     // 142 => [278593]
            map.insert(185, &[17537]);     // 185 => [17537]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode20_11 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode20_11 {
    fn name(&self) -> String {
        "[20, 11] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        20
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
        codeword.truncate(11);
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
        let code = GuavaCode20_11.generator_matrix();
        assert_eq!(code.ncols(), 20);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode20_11;
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
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, true, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, false, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, false, false, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, false, false, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, true, true, false, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, true, true, false, true, false, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, true, false, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, true, false, false, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, false, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, false, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, true, true, false, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, true, true, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, true, true, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, true, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, true, true, false, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true, false, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, false, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, false, true, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, false, false, true, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, true, true, false, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, true, false, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, false, false, true, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, false, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, true, false, false, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, true, false, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
