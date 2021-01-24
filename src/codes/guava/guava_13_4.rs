use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[13, 4]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode13_4;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7361 ],
                &[ 1266 ],
                &[ 2532 ],
                &[ 7864 ],
                
            ], 13));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4609 ],
                &[ 5122 ],
                &[ 6148 ],
                &[ 520 ],
                &[ 5648 ],
                &[ 3616 ],
                &[ 7744 ],
                &[ 7296 ],
                &[ 6400 ],
                
            ], 13));
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
            map.insert(121, &[512]);     // 121 => [512]
            map.insert(242, &[1024]);     // 242 => [1024]
            map.insert(484, &[2048]);     // 484 => [2048]
            map.insert(471, &[4096]);     // 471 => [4096]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(257, &[257]);     // 257 => [257]
            map.insert(120, &[513]);     // 120 => [513]
            map.insert(243, &[1025]);     // 243 => [1025]
            map.insert(485, &[2049]);     // 485 => [2049]
            map.insert(470, &[4097]);     // 470 => [4097]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(258, &[258]);     // 258 => [258]
            map.insert(123, &[514]);     // 123 => [514]
            map.insert(240, &[1026]);     // 240 => [1026]
            map.insert(486, &[2050]);     // 486 => [2050]
            map.insert(469, &[4098]);     // 469 => [4098]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(260, &[260]);     // 260 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(246, &[1028]);     // 246 => [1028]
            map.insert(480, &[2052]);     // 480 => [2052]
            map.insert(467, &[4100]);     // 467 => [4100]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(264, &[264]);     // 264 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(250, &[1032]);     // 250 => [1032]
            map.insert(492, &[2056]);     // 492 => [2056]
            map.insert(479, &[4104]);     // 479 => [4104]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(272, &[272]);     // 272 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(226, &[1040]);     // 226 => [1040]
            map.insert(500, &[2064]);     // 500 => [2064]
            map.insert(455, &[4112]);     // 455 => [4112]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(288, &[288]);     // 288 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(210, &[1056]);     // 210 => [1056]
            map.insert(452, &[2080]);     // 452 => [2080]
            map.insert(503, &[4128]);     // 503 => [4128]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(320, &[320]);     // 320 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(178, &[1088]);     // 178 => [1088]
            map.insert(420, &[2112]);     // 420 => [2112]
            map.insert(407, &[4160]);     // 407 => [4160]
            map.insert(384, &[384]);     // 384 => [384]
            map.insert(249, &[640]);     // 249 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(356, &[2176]);     // 356 => [2176]
            map.insert(343, &[4224]);     // 343 => [4224]
            map.insert(377, &[768]);     // 377 => [768]
            map.insert(498, &[1280]);     // 498 => [1280]
            map.insert(228, &[2304]);     // 228 => [2304]
            map.insert(215, &[4352]);     // 215 => [4352]
            map.insert(139, &[1536]);     // 139 => [1536]
            map.insert(413, &[2560]);     // 413 => [2560]
            map.insert(430, &[4608]);     // 430 => [4608]
            map.insert(278, &[3072]);     // 278 => [3072]
            map.insert(293, &[5120]);     // 293 => [5120]
            map.insert(51, &[6144]);     // 51 => [6144]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(259, &[259]);     // 259 => [259]
            map.insert(122, &[515]);     // 122 => [515]
            map.insert(241, &[1027]);     // 241 => [1027]
            map.insert(487, &[2051]);     // 487 => [2051]
            map.insert(468, &[4099]);     // 468 => [4099]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(261, &[261]);     // 261 => [261]
            map.insert(124, &[517]);     // 124 => [517]
            map.insert(247, &[1029]);     // 247 => [1029]
            map.insert(481, &[2053]);     // 481 => [2053]
            map.insert(466, &[4101]);     // 466 => [4101]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(265, &[265]);     // 265 => [265]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(251, &[1033]);     // 251 => [1033]
            map.insert(493, &[2057]);     // 493 => [2057]
            map.insert(478, &[4105]);     // 478 => [4105]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(273, &[273]);     // 273 => [273]
            map.insert(104, &[529]);     // 104 => [529]
            map.insert(227, &[1041]);     // 227 => [1041]
            map.insert(501, &[2065]);     // 501 => [2065]
            map.insert(454, &[4113]);     // 454 => [4113]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(289, &[289]);     // 289 => [289]
            map.insert(88, &[545]);     // 88 => [545]
            map.insert(211, &[1057]);     // 211 => [1057]
            map.insert(453, &[2081]);     // 453 => [2081]
            map.insert(502, &[4129]);     // 502 => [4129]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(321, &[321]);     // 321 => [321]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(179, &[1089]);     // 179 => [1089]
            map.insert(421, &[2113]);     // 421 => [2113]
            map.insert(406, &[4161]);     // 406 => [4161]
            map.insert(385, &[385]);     // 385 => [385]
            map.insert(248, &[641]);     // 248 => [641]
            map.insert(115, &[1153]);     // 115 => [1153]
            map.insert(357, &[2177]);     // 357 => [2177]
            map.insert(342, &[4225]);     // 342 => [4225]
            map.insert(376, &[769]);     // 376 => [769]
            map.insert(499, &[1281]);     // 499 => [1281]
            map.insert(229, &[2305]);     // 229 => [2305]
            map.insert(214, &[4353]);     // 214 => [4353]
            map.insert(138, &[1537]);     // 138 => [1537]
            map.insert(412, &[2561]);     // 412 => [2561]
            map.insert(431, &[4609]);     // 431 => [4609]
            map.insert(279, &[3073]);     // 279 => [3073]
            map.insert(292, &[5121]);     // 292 => [5121]
            map.insert(50, &[6145]);     // 50 => [6145]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(262, &[262]);     // 262 => [262]
            map.insert(127, &[518]);     // 127 => [518]
            map.insert(244, &[1030]);     // 244 => [1030]
            map.insert(482, &[2054]);     // 482 => [2054]
            map.insert(465, &[4102]);     // 465 => [4102]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(266, &[266]);     // 266 => [266]
            map.insert(494, &[2058]);     // 494 => [2058]
            map.insert(477, &[4106]);     // 477 => [4106]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(146, &[146]);     // 146 => [146]
            map.insert(274, &[274]);     // 274 => [274]
            map.insert(107, &[530]);     // 107 => [530]
            map.insert(224, &[1042]);     // 224 => [1042]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(290, &[290]);     // 290 => [290]
            map.insert(91, &[546]);     // 91 => [546]
            map.insert(208, &[1058]);     // 208 => [1058]
            map.insert(194, &[194]);     // 194 => [194]
            map.insert(322, &[322]);     // 322 => [322]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(176, &[1090]);     // 176 => [1090]
            map.insert(422, &[2114]);     // 422 => [2114]
            map.insert(405, &[4162]);     // 405 => [4162]
            map.insert(386, &[386]);     // 386 => [386]
            map.insert(358, &[2178]);     // 358 => [2178]
            map.insert(341, &[4226]);     // 341 => [4226]
            map.insert(379, &[770]);     // 379 => [770]
            map.insert(496, &[1282]);     // 496 => [1282]
            map.insert(230, &[2306]);     // 230 => [2306]
            map.insert(213, &[4354]);     // 213 => [4354]
            map.insert(415, &[2562]);     // 415 => [2562]
            map.insert(428, &[4610]);     // 428 => [4610]
            map.insert(276, &[3074]);     // 276 => [3074]
            map.insert(295, &[5122]);     // 295 => [5122]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(76, &[76]);     // 76 => [76]
            map.insert(140, &[140]);     // 140 => [140]
            map.insert(268, &[268]);     // 268 => [268]
            map.insert(117, &[524]);     // 117 => [524]
            map.insert(254, &[1036]);     // 254 => [1036]
            map.insert(488, &[2060]);     // 488 => [2060]
            map.insert(475, &[4108]);     // 475 => [4108]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(84, &[84]);     // 84 => [84]
            map.insert(148, &[148]);     // 148 => [148]
            map.insert(109, &[532]);     // 109 => [532]
            map.insert(451, &[4116]);     // 451 => [4116]
            map.insert(100, &[100]);     // 100 => [100]
            map.insert(164, &[164]);     // 164 => [164]
            map.insert(93, &[548]);     // 93 => [548]
            map.insert(448, &[2084]);     // 448 => [2084]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(324, &[324]);     // 324 => [324]
            map.insert(61, &[580]);     // 61 => [580]
            map.insert(182, &[1092]);     // 182 => [1092]
            map.insert(416, &[2116]);     // 416 => [2116]
            map.insert(403, &[4164]);     // 403 => [4164]
            map.insert(388, &[388]);     // 388 => [388]
            map.insert(253, &[644]);     // 253 => [644]
            map.insert(118, &[1156]);     // 118 => [1156]
            map.insert(352, &[2180]);     // 352 => [2180]
            map.insert(339, &[4228]);     // 339 => [4228]
            map.insert(381, &[772]);     // 381 => [772]
            map.insert(143, &[1540]);     // 143 => [1540]
            map.insert(409, &[2564]);     // 409 => [2564]
            map.insert(426, &[4612]);     // 426 => [4612]
            map.insert(55, &[6148]);     // 55 => [6148]
            map.insert(152, &[152]);     // 152 => [152]
            map.insert(280, &[280]);     // 280 => [280]
            map.insert(234, &[1048]);     // 234 => [1048]
            map.insert(508, &[2072]);     // 508 => [2072]
            map.insert(463, &[4120]);     // 463 => [4120]
            map.insert(168, &[168]);     // 168 => [168]
            map.insert(296, &[296]);     // 296 => [296]
            map.insert(218, &[1064]);     // 218 => [1064]
            map.insert(460, &[2088]);     // 460 => [2088]
            map.insert(511, &[4136]);     // 511 => [4136]
            map.insert(200, &[200]);     // 200 => [200]
            map.insert(328, &[328]);     // 328 => [328]
            map.insert(186, &[1096]);     // 186 => [1096]
            map.insert(392, &[392]);     // 392 => [392]
            map.insert(364, &[2184]);     // 364 => [2184]
            map.insert(351, &[4232]);     // 351 => [4232]
            map.insert(369, &[776]);     // 369 => [776]
            map.insert(506, &[1288]);     // 506 => [1288]
            map.insert(236, &[2312]);     // 236 => [2312]
            map.insert(223, &[4360]);     // 223 => [4360]
            map.insert(286, &[3080]);     // 286 => [3080]
            map.insert(301, &[5128]);     // 301 => [5128]
            map.insert(304, &[304]);     // 304 => [304]
            map.insert(336, &[336]);     // 336 => [336]
            map.insert(436, &[2128]);     // 436 => [2128]
            map.insert(391, &[4176]);     // 391 => [4176]
            map.insert(400, &[400]);     // 400 => [400]
            map.insert(233, &[656]);     // 233 => [656]
            map.insert(372, &[2192]);     // 372 => [2192]
            map.insert(327, &[4240]);     // 327 => [4240]
            map.insert(361, &[784]);     // 361 => [784]
            map.insert(199, &[4368]);     // 199 => [4368]
            map.insert(155, &[1552]);     // 155 => [1552]
            map.insert(397, &[2576]);     // 397 => [2576]
            map.insert(446, &[4624]);     // 446 => [4624]
            map.insert(309, &[5136]);     // 309 => [5136]
            map.insert(439, &[4192]);     // 439 => [4192]
            map.insert(217, &[672]);     // 217 => [672]
            map.insert(375, &[4256]);     // 375 => [4256]
            map.insert(345, &[800]);     // 345 => [800]
            map.insert(171, &[1568]);     // 171 => [1568]
            map.insert(445, &[2592]);     // 445 => [2592]
            map.insert(398, &[4640]);     // 398 => [4640]
            map.insert(310, &[3104]);     // 310 => [3104]
            map.insert(185, &[704]);     // 185 => [704]
            map.insert(313, &[832]);     // 313 => [832]
            map.insert(434, &[1344]);     // 434 => [1344]
            map.insert(151, &[4416]);     // 151 => [4416]
            map.insert(203, &[1600]);     // 203 => [1600]
            map.insert(505, &[896]);     // 505 => [896]
            map.insert(370, &[1408]);     // 370 => [1408]
            map.insert(87, &[4480]);     // 87 => [4480]
            map.insert(285, &[2688]);     // 285 => [2688]
            map.insert(302, &[4736]);     // 302 => [4736]
            map.insert(395, &[1792]);     // 395 => [1792]
            map.insert(157, &[2816]);     // 157 => [2816]
            map.insert(174, &[4864]);     // 174 => [4864]
            map.insert(307, &[6400]);     // 307 => [6400]
            map.insert(367, &[3584]);     // 367 => [3584]
            map.insert(348, &[5632]);     // 348 => [5632]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(39, &[39]);     // 39 => [39]
            map.insert(71, &[71]);     // 71 => [71]
            map.insert(135, &[135]);     // 135 => [135]
            map.insert(263, &[263]);     // 263 => [263]
            map.insert(126, &[519]);     // 126 => [519]
            map.insert(245, &[1031]);     // 245 => [1031]
            map.insert(483, &[2055]);     // 483 => [2055]
            map.insert(464, &[4103]);     // 464 => [4103]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(75, &[75]);     // 75 => [75]
            map.insert(267, &[267]);     // 267 => [267]
            map.insert(495, &[2059]);     // 495 => [2059]
            map.insert(476, &[4107]);     // 476 => [4107]
            map.insert(83, &[83]);     // 83 => [83]
            map.insert(147, &[147]);     // 147 => [147]
            map.insert(275, &[275]);     // 275 => [275]
            map.insert(106, &[531]);     // 106 => [531]
            map.insert(225, &[1043]);     // 225 => [1043]
            map.insert(99, &[99]);     // 99 => [99]
            map.insert(163, &[163]);     // 163 => [163]
            map.insert(291, &[291]);     // 291 => [291]
            map.insert(90, &[547]);     // 90 => [547]
            map.insert(209, &[1059]);     // 209 => [1059]
            map.insert(195, &[195]);     // 195 => [195]
            map.insert(323, &[323]);     // 323 => [323]
            map.insert(58, &[579]);     // 58 => [579]
            map.insert(177, &[1091]);     // 177 => [1091]
            map.insert(423, &[2115]);     // 423 => [2115]
            map.insert(404, &[4163]);     // 404 => [4163]
            map.insert(387, &[387]);     // 387 => [387]
            map.insert(359, &[2179]);     // 359 => [2179]
            map.insert(340, &[4227]);     // 340 => [4227]
            map.insert(378, &[771]);     // 378 => [771]
            map.insert(497, &[1283]);     // 497 => [1283]
            map.insert(231, &[2307]);     // 231 => [2307]
            map.insert(212, &[4355]);     // 212 => [4355]
            map.insert(414, &[2563]);     // 414 => [2563]
            map.insert(429, &[4611]);     // 429 => [4611]
            map.insert(277, &[3075]);     // 277 => [3075]
            map.insert(294, &[5123]);     // 294 => [5123]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(45, &[45]);     // 45 => [45]
            map.insert(77, &[77]);     // 77 => [77]
            map.insert(141, &[141]);     // 141 => [141]
            map.insert(269, &[269]);     // 269 => [269]
            map.insert(116, &[525]);     // 116 => [525]
            map.insert(255, &[1037]);     // 255 => [1037]
            map.insert(489, &[2061]);     // 489 => [2061]
            map.insert(474, &[4109]);     // 474 => [4109]
            map.insert(53, &[53]);     // 53 => [53]
            map.insert(85, &[85]);     // 85 => [85]
            map.insert(149, &[149]);     // 149 => [149]
            map.insert(108, &[533]);     // 108 => [533]
            map.insert(450, &[4117]);     // 450 => [4117]
            map.insert(101, &[101]);     // 101 => [101]
            map.insert(165, &[165]);     // 165 => [165]
            map.insert(92, &[549]);     // 92 => [549]
            map.insert(449, &[2085]);     // 449 => [2085]
            map.insert(197, &[197]);     // 197 => [197]
            map.insert(325, &[325]);     // 325 => [325]
            map.insert(60, &[581]);     // 60 => [581]
            map.insert(183, &[1093]);     // 183 => [1093]
            map.insert(417, &[2117]);     // 417 => [2117]
            map.insert(402, &[4165]);     // 402 => [4165]
            map.insert(389, &[389]);     // 389 => [389]
            map.insert(252, &[645]);     // 252 => [645]
            map.insert(119, &[1157]);     // 119 => [1157]
            map.insert(353, &[2181]);     // 353 => [2181]
            map.insert(338, &[4229]);     // 338 => [4229]
            map.insert(380, &[773]);     // 380 => [773]
            map.insert(142, &[1541]);     // 142 => [1541]
            map.insert(408, &[2565]);     // 408 => [2565]
            map.insert(427, &[4613]);     // 427 => [4613]
            map.insert(54, &[6149]);     // 54 => [6149]
            map.insert(153, &[153]);     // 153 => [153]
            map.insert(281, &[281]);     // 281 => [281]
            map.insert(235, &[1049]);     // 235 => [1049]
            map.insert(509, &[2073]);     // 509 => [2073]
            map.insert(462, &[4121]);     // 462 => [4121]
            map.insert(169, &[169]);     // 169 => [169]
            map.insert(297, &[297]);     // 297 => [297]
            map.insert(219, &[1065]);     // 219 => [1065]
            map.insert(461, &[2089]);     // 461 => [2089]
            map.insert(510, &[4137]);     // 510 => [4137]
            map.insert(201, &[201]);     // 201 => [201]
            map.insert(329, &[329]);     // 329 => [329]
            map.insert(187, &[1097]);     // 187 => [1097]
            map.insert(393, &[393]);     // 393 => [393]
            map.insert(365, &[2185]);     // 365 => [2185]
            map.insert(350, &[4233]);     // 350 => [4233]
            map.insert(368, &[777]);     // 368 => [777]
            map.insert(507, &[1289]);     // 507 => [1289]
            map.insert(237, &[2313]);     // 237 => [2313]
            map.insert(222, &[4361]);     // 222 => [4361]
            map.insert(287, &[3081]);     // 287 => [3081]
            map.insert(300, &[5129]);     // 300 => [5129]
            map.insert(305, &[305]);     // 305 => [305]
            map.insert(337, &[337]);     // 337 => [337]
            map.insert(437, &[2129]);     // 437 => [2129]
            map.insert(390, &[4177]);     // 390 => [4177]
            map.insert(401, &[401]);     // 401 => [401]
            map.insert(232, &[657]);     // 232 => [657]
            map.insert(373, &[2193]);     // 373 => [2193]
            map.insert(326, &[4241]);     // 326 => [4241]
            map.insert(360, &[785]);     // 360 => [785]
            map.insert(198, &[4369]);     // 198 => [4369]
            map.insert(154, &[1553]);     // 154 => [1553]
            map.insert(396, &[2577]);     // 396 => [2577]
            map.insert(447, &[4625]);     // 447 => [4625]
            map.insert(308, &[5137]);     // 308 => [5137]
            map.insert(438, &[4193]);     // 438 => [4193]
            map.insert(216, &[673]);     // 216 => [673]
            map.insert(374, &[4257]);     // 374 => [4257]
            map.insert(344, &[801]);     // 344 => [801]
            map.insert(170, &[1569]);     // 170 => [1569]
            map.insert(444, &[2593]);     // 444 => [2593]
            map.insert(399, &[4641]);     // 399 => [4641]
            map.insert(311, &[3105]);     // 311 => [3105]
            map.insert(184, &[705]);     // 184 => [705]
            map.insert(312, &[833]);     // 312 => [833]
            map.insert(435, &[1345]);     // 435 => [1345]
            map.insert(150, &[4417]);     // 150 => [4417]
            map.insert(202, &[1601]);     // 202 => [1601]
            map.insert(504, &[897]);     // 504 => [897]
            map.insert(371, &[1409]);     // 371 => [1409]
            map.insert(86, &[4481]);     // 86 => [4481]
            map.insert(284, &[2689]);     // 284 => [2689]
            map.insert(303, &[4737]);     // 303 => [4737]
            map.insert(394, &[1793]);     // 394 => [1793]
            map.insert(156, &[2817]);     // 156 => [2817]
            map.insert(175, &[4865]);     // 175 => [4865]
            map.insert(306, &[6401]);     // 306 => [6401]
            map.insert(366, &[3585]);     // 366 => [3585]
            map.insert(349, &[5633]);     // 349 => [5633]
            map.insert(30, &[30]);     // 30 => [30]
            map.insert(46, &[46]);     // 46 => [46]
            map.insert(78, &[78]);     // 78 => [78]
            map.insert(270, &[270]);     // 270 => [270]
            map.insert(490, &[2062]);     // 490 => [2062]
            map.insert(473, &[4110]);     // 473 => [4110]
            map.insert(111, &[534]);     // 111 => [534]
            map.insert(102, &[102]);     // 102 => [102]
            map.insert(166, &[166]);     // 166 => [166]
            map.insert(95, &[550]);     // 95 => [550]
            map.insert(63, &[582]);     // 63 => [582]
            map.insert(180, &[1094]);     // 180 => [1094]
            map.insert(418, &[2118]);     // 418 => [2118]
            map.insert(354, &[2182]);     // 354 => [2182]
            map.insert(383, &[774]);     // 383 => [774]
            map.insert(411, &[2566]);     // 411 => [2566]
            map.insert(424, &[4614]);     // 424 => [4614]
            map.insert(282, &[282]);     // 282 => [282]
            map.insert(298, &[298]);     // 298 => [298]
            map.insert(330, &[330]);     // 330 => [330]
            map.insert(238, &[2314]);     // 238 => [2314]
            map.insert(221, &[4362]);     // 221 => [4362]
            map.insert(363, &[786]);     // 363 => [786]
            map.insert(347, &[802]);     // 347 => [802]
            map.insert(315, &[834]);     // 315 => [834]
            map.insert(432, &[1346]);     // 432 => [1346]
            map.insert(159, &[2818]);     // 159 => [2818]
            map.insert(172, &[4866]);     // 172 => [4866]
            map.insert(459, &[4124]);     // 459 => [4124]
            map.insert(456, &[2092]);     // 456 => [2092]
            map.insert(204, &[204]);     // 204 => [204]
            map.insert(332, &[332]);     // 332 => [332]
            map.insert(190, &[1100]);     // 190 => [1100]
            map.insert(442, &[4628]);     // 442 => [4628]
            map.insert(441, &[2596]);     // 441 => [2596]
            map.insert(189, &[708]);     // 189 => [708]
            map.insert(317, &[836]);     // 317 => [836]
            map.insert(207, &[1604]);     // 207 => [1604]
            map.insert(335, &[4248]);     // 335 => [4248]
            map.insert(318, &[3112]);     // 318 => [3112]
            map.insert(31, &[31]);     // 31 => [31]
            map.insert(47, &[47]);     // 47 => [47]
            map.insert(79, &[79]);     // 79 => [79]
            map.insert(271, &[271]);     // 271 => [271]
            map.insert(491, &[2063]);     // 491 => [2063]
            map.insert(472, &[4111]);     // 472 => [4111]
            map.insert(110, &[535]);     // 110 => [535]
            map.insert(103, &[103]);     // 103 => [103]
            map.insert(167, &[167]);     // 167 => [167]
            map.insert(94, &[551]);     // 94 => [551]
            map.insert(62, &[583]);     // 62 => [583]
            map.insert(181, &[1095]);     // 181 => [1095]
            map.insert(419, &[2119]);     // 419 => [2119]
            map.insert(355, &[2183]);     // 355 => [2183]
            map.insert(382, &[775]);     // 382 => [775]
            map.insert(410, &[2567]);     // 410 => [2567]
            map.insert(425, &[4615]);     // 425 => [4615]
            map.insert(283, &[283]);     // 283 => [283]
            map.insert(299, &[299]);     // 299 => [299]
            map.insert(331, &[331]);     // 331 => [331]
            map.insert(239, &[2315]);     // 239 => [2315]
            map.insert(220, &[4363]);     // 220 => [4363]
            map.insert(362, &[787]);     // 362 => [787]
            map.insert(346, &[803]);     // 346 => [803]
            map.insert(314, &[835]);     // 314 => [835]
            map.insert(433, &[1347]);     // 433 => [1347]
            map.insert(158, &[2819]);     // 158 => [2819]
            map.insert(173, &[4867]);     // 173 => [4867]
            map.insert(458, &[4125]);     // 458 => [4125]
            map.insert(457, &[2093]);     // 457 => [2093]
            map.insert(205, &[205]);     // 205 => [205]
            map.insert(333, &[333]);     // 333 => [333]
            map.insert(191, &[1101]);     // 191 => [1101]
            map.insert(443, &[4629]);     // 443 => [4629]
            map.insert(440, &[2597]);     // 440 => [2597]
            map.insert(188, &[709]);     // 188 => [709]
            map.insert(316, &[837]);     // 316 => [837]
            map.insert(206, &[1605]);     // 206 => [1605]
            map.insert(334, &[4249]);     // 334 => [4249]
            map.insert(319, &[3113]);     // 319 => [3113]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode13_4 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode13_4 {
    fn name(&self) -> String {
        "[13, 4] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        13
    }

    fn dimension(&self) -> usize {
        4
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
        let mut error = BinVector::with_capacity(13);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 13 / 64 + if 13 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(13) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(4);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[13 / 64] & !((1 << 13) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode13_4.generator_matrix();
        assert_eq!(code.ncols(), 13);
        assert_eq!(code.nrows(), 4);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode13_4;
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
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_4;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, true, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
