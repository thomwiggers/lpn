use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[12, 3]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode12_3;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2665 ],
                &[ 1690 ],
                &[ 3900 ],
                
            ], 12));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1537 ],
                &[ 2562 ],
                &[ 3588 ],
                &[ 520 ],
                &[ 1040 ],
                &[ 2080 ],
                &[ 1600 ],
                &[ 2688 ],
                &[ 3840 ],
                
            ], 12));
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
            map.insert(463, &[512]);     // 463 => [512]
            map.insert(341, &[1024]);     // 341 => [1024]
            map.insert(422, &[2048]);     // 422 => [2048]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(257, &[257]);     // 257 => [257]
            map.insert(462, &[513]);     // 462 => [513]
            map.insert(340, &[1025]);     // 340 => [1025]
            map.insert(423, &[2049]);     // 423 => [2049]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(258, &[258]);     // 258 => [258]
            map.insert(461, &[514]);     // 461 => [514]
            map.insert(343, &[1026]);     // 343 => [1026]
            map.insert(420, &[2050]);     // 420 => [2050]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(260, &[260]);     // 260 => [260]
            map.insert(459, &[516]);     // 459 => [516]
            map.insert(337, &[1028]);     // 337 => [1028]
            map.insert(418, &[2052]);     // 418 => [2052]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(264, &[264]);     // 264 => [264]
            map.insert(455, &[520]);     // 455 => [520]
            map.insert(349, &[1032]);     // 349 => [1032]
            map.insert(430, &[2056]);     // 430 => [2056]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(272, &[272]);     // 272 => [272]
            map.insert(479, &[528]);     // 479 => [528]
            map.insert(325, &[1040]);     // 325 => [1040]
            map.insert(438, &[2064]);     // 438 => [2064]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(288, &[288]);     // 288 => [288]
            map.insert(495, &[544]);     // 495 => [544]
            map.insert(373, &[1056]);     // 373 => [1056]
            map.insert(390, &[2080]);     // 390 => [2080]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(320, &[320]);     // 320 => [320]
            map.insert(399, &[576]);     // 399 => [576]
            map.insert(277, &[1088]);     // 277 => [1088]
            map.insert(486, &[2112]);     // 486 => [2112]
            map.insert(384, &[384]);     // 384 => [384]
            map.insert(335, &[640]);     // 335 => [640]
            map.insert(469, &[1152]);     // 469 => [1152]
            map.insert(294, &[2176]);     // 294 => [2176]
            map.insert(207, &[768]);     // 207 => [768]
            map.insert(85, &[1280]);     // 85 => [1280]
            map.insert(166, &[2304]);     // 166 => [2304]
            map.insert(154, &[1536]);     // 154 => [1536]
            map.insert(105, &[2560]);     // 105 => [2560]
            map.insert(243, &[3072]);     // 243 => [3072]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(67, &[67]);     // 67 => [67]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(259, &[259]);     // 259 => [259]
            map.insert(460, &[515]);     // 460 => [515]
            map.insert(342, &[1027]);     // 342 => [1027]
            map.insert(421, &[2051]);     // 421 => [2051]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(261, &[261]);     // 261 => [261]
            map.insert(458, &[517]);     // 458 => [517]
            map.insert(336, &[1029]);     // 336 => [1029]
            map.insert(419, &[2053]);     // 419 => [2053]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(265, &[265]);     // 265 => [265]
            map.insert(454, &[521]);     // 454 => [521]
            map.insert(348, &[1033]);     // 348 => [1033]
            map.insert(431, &[2057]);     // 431 => [2057]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(273, &[273]);     // 273 => [273]
            map.insert(478, &[529]);     // 478 => [529]
            map.insert(324, &[1041]);     // 324 => [1041]
            map.insert(439, &[2065]);     // 439 => [2065]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(289, &[289]);     // 289 => [289]
            map.insert(494, &[545]);     // 494 => [545]
            map.insert(372, &[1057]);     // 372 => [1057]
            map.insert(391, &[2081]);     // 391 => [2081]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(321, &[321]);     // 321 => [321]
            map.insert(398, &[577]);     // 398 => [577]
            map.insert(276, &[1089]);     // 276 => [1089]
            map.insert(487, &[2113]);     // 487 => [2113]
            map.insert(385, &[385]);     // 385 => [385]
            map.insert(334, &[641]);     // 334 => [641]
            map.insert(468, &[1153]);     // 468 => [1153]
            map.insert(295, &[2177]);     // 295 => [2177]
            map.insert(206, &[769]);     // 206 => [769]
            map.insert(84, &[1281]);     // 84 => [1281]
            map.insert(167, &[2305]);     // 167 => [2305]
            map.insert(155, &[1537]);     // 155 => [1537]
            map.insert(104, &[2561]);     // 104 => [2561]
            map.insert(242, &[3073]);     // 242 => [3073]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(262, &[262]);     // 262 => [262]
            map.insert(457, &[518]);     // 457 => [518]
            map.insert(339, &[1030]);     // 339 => [1030]
            map.insert(416, &[2054]);     // 416 => [2054]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(138, &[138]);     // 138 => [138]
            map.insert(266, &[266]);     // 266 => [266]
            map.insert(453, &[522]);     // 453 => [522]
            map.insert(351, &[1034]);     // 351 => [1034]
            map.insert(428, &[2058]);     // 428 => [2058]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(146, &[146]);     // 146 => [146]
            map.insert(274, &[274]);     // 274 => [274]
            map.insert(477, &[530]);     // 477 => [530]
            map.insert(327, &[1042]);     // 327 => [1042]
            map.insert(436, &[2066]);     // 436 => [2066]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(290, &[290]);     // 290 => [290]
            map.insert(493, &[546]);     // 493 => [546]
            map.insert(375, &[1058]);     // 375 => [1058]
            map.insert(388, &[2082]);     // 388 => [2082]
            map.insert(194, &[194]);     // 194 => [194]
            map.insert(322, &[322]);     // 322 => [322]
            map.insert(397, &[578]);     // 397 => [578]
            map.insert(279, &[1090]);     // 279 => [1090]
            map.insert(484, &[2114]);     // 484 => [2114]
            map.insert(386, &[386]);     // 386 => [386]
            map.insert(333, &[642]);     // 333 => [642]
            map.insert(471, &[1154]);     // 471 => [1154]
            map.insert(292, &[2178]);     // 292 => [2178]
            map.insert(205, &[770]);     // 205 => [770]
            map.insert(87, &[1282]);     // 87 => [1282]
            map.insert(164, &[2306]);     // 164 => [2306]
            map.insert(152, &[1538]);     // 152 => [1538]
            map.insert(107, &[2562]);     // 107 => [2562]
            map.insert(241, &[3074]);     // 241 => [3074]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(76, &[76]);     // 76 => [76]
            map.insert(140, &[140]);     // 140 => [140]
            map.insert(268, &[268]);     // 268 => [268]
            map.insert(451, &[524]);     // 451 => [524]
            map.insert(345, &[1036]);     // 345 => [1036]
            map.insert(426, &[2060]);     // 426 => [2060]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(148, &[148]);     // 148 => [148]
            map.insert(475, &[532]);     // 475 => [532]
            map.insert(434, &[2068]);     // 434 => [2068]
            map.insert(100, &[100]);     // 100 => [100]
            map.insert(491, &[548]);     // 491 => [548]
            map.insert(369, &[1060]);     // 369 => [1060]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(395, &[580]);     // 395 => [580]
            map.insert(482, &[2116]);     // 482 => [2116]
            map.insert(331, &[644]);     // 331 => [644]
            map.insert(465, &[1156]);     // 465 => [1156]
            map.insert(203, &[772]);     // 203 => [772]
            map.insert(158, &[1540]);     // 158 => [1540]
            map.insert(109, &[2564]);     // 109 => [2564]
            map.insert(247, &[3076]);     // 247 => [3076]
            map.insert(56, &[56]);     // 56 => [56]
            map.insert(88, &[88]);     // 88 => [88]
            map.insert(280, &[280]);     // 280 => [280]
            map.insert(446, &[2072]);     // 446 => [2072]
            map.insert(168, &[168]);     // 168 => [168]
            map.insert(296, &[296]);     // 296 => [296]
            map.insert(381, &[1064]);     // 381 => [1064]
            map.insert(200, &[200]);     // 200 => [200]
            map.insert(328, &[328]);     // 328 => [328]
            map.insert(285, &[1096]);     // 285 => [1096]
            map.insert(392, &[392]);     // 392 => [392]
            map.insert(302, &[2184]);     // 302 => [2184]
            map.insert(199, &[776]);     // 199 => [776]
            map.insert(93, &[1288]);     // 93 => [1288]
            map.insert(174, &[2312]);     // 174 => [2312]
            map.insert(251, &[3080]);     // 251 => [3080]
            map.insert(112, &[112]);     // 112 => [112]
            map.insert(176, &[176]);     // 176 => [176]
            map.insert(304, &[304]);     // 304 => [304]
            map.insert(511, &[560]);     // 511 => [560]
            map.insert(357, &[1072]);     // 357 => [1072]
            map.insert(406, &[2096]);     // 406 => [2096]
            map.insert(208, &[208]);     // 208 => [208]
            map.insert(415, &[592]);     // 415 => [592]
            map.insert(502, &[2128]);     // 502 => [2128]
            map.insert(400, &[400]);     // 400 => [400]
            map.insert(310, &[2192]);     // 310 => [2192]
            map.insert(223, &[784]);     // 223 => [784]
            map.insert(182, &[2320]);     // 182 => [2320]
            map.insert(121, &[2576]);     // 121 => [2576]
            map.insert(227, &[3088]);     // 227 => [3088]
            map.insert(224, &[224]);     // 224 => [224]
            map.insert(352, &[352]);     // 352 => [352]
            map.insert(309, &[1120]);     // 309 => [1120]
            map.insert(367, &[672]);     // 367 => [672]
            map.insert(501, &[1184]);     // 501 => [1184]
            map.insert(239, &[800]);     // 239 => [800]
            map.insert(117, &[1312]);     // 117 => [1312]
            map.insert(186, &[1568]);     // 186 => [1568]
            map.insert(211, &[3104]);     // 211 => [3104]
            map.insert(448, &[448]);     // 448 => [448]
            map.insert(271, &[704]);     // 271 => [704]
            map.insert(405, &[1216]);     // 405 => [1216]
            map.insert(358, &[2240]);     // 358 => [2240]
            map.insert(143, &[832]);     // 143 => [832]
            map.insert(230, &[2368]);     // 230 => [2368]
            map.insert(218, &[1600]);     // 218 => [1600]
            map.insert(179, &[3136]);     // 179 => [3136]
            map.insert(79, &[896]);     // 79 => [896]
            map.insert(213, &[1408]);     // 213 => [1408]
            map.insert(233, &[2688]);     // 233 => [2688]
            map.insert(115, &[3200]);     // 115 => [3200]
            map.insert(410, &[1792]);     // 410 => [1792]
            map.insert(361, &[2816]);     // 361 => [2816]
            map.insert(499, &[3328]);     // 499 => [3328]
            map.insert(316, &[3584]);     // 316 => [3584]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(39, &[39]);     // 39 => [39]
            map.insert(71, &[71]);     // 71 => [71]
            map.insert(135, &[135]);     // 135 => [135]
            map.insert(263, &[263]);     // 263 => [263]
            map.insert(456, &[519]);     // 456 => [519]
            map.insert(338, &[1031]);     // 338 => [1031]
            map.insert(417, &[2055]);     // 417 => [2055]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(75, &[75]);     // 75 => [75]
            map.insert(139, &[139]);     // 139 => [139]
            map.insert(267, &[267]);     // 267 => [267]
            map.insert(452, &[523]);     // 452 => [523]
            map.insert(350, &[1035]);     // 350 => [1035]
            map.insert(429, &[2059]);     // 429 => [2059]
            map.insert(51, &[51]);     // 51 => [51]
            map.insert(83, &[83]);     // 83 => [83]
            map.insert(147, &[147]);     // 147 => [147]
            map.insert(275, &[275]);     // 275 => [275]
            map.insert(476, &[531]);     // 476 => [531]
            map.insert(326, &[1043]);     // 326 => [1043]
            map.insert(437, &[2067]);     // 437 => [2067]
            map.insert(99, &[99]);     // 99 => [99]
            map.insert(163, &[163]);     // 163 => [163]
            map.insert(291, &[291]);     // 291 => [291]
            map.insert(492, &[547]);     // 492 => [547]
            map.insert(374, &[1059]);     // 374 => [1059]
            map.insert(389, &[2083]);     // 389 => [2083]
            map.insert(195, &[195]);     // 195 => [195]
            map.insert(323, &[323]);     // 323 => [323]
            map.insert(396, &[579]);     // 396 => [579]
            map.insert(278, &[1091]);     // 278 => [1091]
            map.insert(485, &[2115]);     // 485 => [2115]
            map.insert(387, &[387]);     // 387 => [387]
            map.insert(332, &[643]);     // 332 => [643]
            map.insert(470, &[1155]);     // 470 => [1155]
            map.insert(293, &[2179]);     // 293 => [2179]
            map.insert(204, &[771]);     // 204 => [771]
            map.insert(86, &[1283]);     // 86 => [1283]
            map.insert(165, &[2307]);     // 165 => [2307]
            map.insert(153, &[1539]);     // 153 => [1539]
            map.insert(106, &[2563]);     // 106 => [2563]
            map.insert(240, &[3075]);     // 240 => [3075]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(45, &[45]);     // 45 => [45]
            map.insert(77, &[77]);     // 77 => [77]
            map.insert(141, &[141]);     // 141 => [141]
            map.insert(269, &[269]);     // 269 => [269]
            map.insert(450, &[525]);     // 450 => [525]
            map.insert(344, &[1037]);     // 344 => [1037]
            map.insert(427, &[2061]);     // 427 => [2061]
            map.insert(53, &[53]);     // 53 => [53]
            map.insert(149, &[149]);     // 149 => [149]
            map.insert(474, &[533]);     // 474 => [533]
            map.insert(435, &[2069]);     // 435 => [2069]
            map.insert(101, &[101]);     // 101 => [101]
            map.insert(490, &[549]);     // 490 => [549]
            map.insert(368, &[1061]);     // 368 => [1061]
            map.insert(197, &[197]);     // 197 => [197]
            map.insert(394, &[581]);     // 394 => [581]
            map.insert(483, &[2117]);     // 483 => [2117]
            map.insert(330, &[645]);     // 330 => [645]
            map.insert(464, &[1157]);     // 464 => [1157]
            map.insert(202, &[773]);     // 202 => [773]
            map.insert(159, &[1541]);     // 159 => [1541]
            map.insert(108, &[2565]);     // 108 => [2565]
            map.insert(246, &[3077]);     // 246 => [3077]
            map.insert(57, &[57]);     // 57 => [57]
            map.insert(89, &[89]);     // 89 => [89]
            map.insert(281, &[281]);     // 281 => [281]
            map.insert(447, &[2073]);     // 447 => [2073]
            map.insert(169, &[169]);     // 169 => [169]
            map.insert(297, &[297]);     // 297 => [297]
            map.insert(380, &[1065]);     // 380 => [1065]
            map.insert(201, &[201]);     // 201 => [201]
            map.insert(329, &[329]);     // 329 => [329]
            map.insert(284, &[1097]);     // 284 => [1097]
            map.insert(393, &[393]);     // 393 => [393]
            map.insert(303, &[2185]);     // 303 => [2185]
            map.insert(198, &[777]);     // 198 => [777]
            map.insert(92, &[1289]);     // 92 => [1289]
            map.insert(175, &[2313]);     // 175 => [2313]
            map.insert(250, &[3081]);     // 250 => [3081]
            map.insert(113, &[113]);     // 113 => [113]
            map.insert(177, &[177]);     // 177 => [177]
            map.insert(305, &[305]);     // 305 => [305]
            map.insert(510, &[561]);     // 510 => [561]
            map.insert(356, &[1073]);     // 356 => [1073]
            map.insert(407, &[2097]);     // 407 => [2097]
            map.insert(209, &[209]);     // 209 => [209]
            map.insert(414, &[593]);     // 414 => [593]
            map.insert(503, &[2129]);     // 503 => [2129]
            map.insert(401, &[401]);     // 401 => [401]
            map.insert(311, &[2193]);     // 311 => [2193]
            map.insert(222, &[785]);     // 222 => [785]
            map.insert(183, &[2321]);     // 183 => [2321]
            map.insert(120, &[2577]);     // 120 => [2577]
            map.insert(226, &[3089]);     // 226 => [3089]
            map.insert(225, &[225]);     // 225 => [225]
            map.insert(353, &[353]);     // 353 => [353]
            map.insert(308, &[1121]);     // 308 => [1121]
            map.insert(366, &[673]);     // 366 => [673]
            map.insert(500, &[1185]);     // 500 => [1185]
            map.insert(238, &[801]);     // 238 => [801]
            map.insert(116, &[1313]);     // 116 => [1313]
            map.insert(187, &[1569]);     // 187 => [1569]
            map.insert(210, &[3105]);     // 210 => [3105]
            map.insert(449, &[449]);     // 449 => [449]
            map.insert(270, &[705]);     // 270 => [705]
            map.insert(404, &[1217]);     // 404 => [1217]
            map.insert(359, &[2241]);     // 359 => [2241]
            map.insert(142, &[833]);     // 142 => [833]
            map.insert(231, &[2369]);     // 231 => [2369]
            map.insert(219, &[1601]);     // 219 => [1601]
            map.insert(178, &[3137]);     // 178 => [3137]
            map.insert(78, &[897]);     // 78 => [897]
            map.insert(212, &[1409]);     // 212 => [1409]
            map.insert(232, &[2689]);     // 232 => [2689]
            map.insert(114, &[3201]);     // 114 => [3201]
            map.insert(411, &[1793]);     // 411 => [1793]
            map.insert(360, &[2817]);     // 360 => [2817]
            map.insert(498, &[3329]);     // 498 => [3329]
            map.insert(317, &[3585]);     // 317 => [3585]
            map.insert(30, &[30]);     // 30 => [30]
            map.insert(46, &[46]);     // 46 => [46]
            map.insert(347, &[1038]);     // 347 => [1038]
            map.insert(424, &[2062]);     // 424 => [2062]
            map.insert(54, &[54]);     // 54 => [54]
            map.insert(150, &[150]);     // 150 => [150]
            map.insert(473, &[534]);     // 473 => [534]
            map.insert(432, &[2070]);     // 432 => [2070]
            map.insert(102, &[102]);     // 102 => [102]
            map.insert(489, &[550]);     // 489 => [550]
            map.insert(371, &[1062]);     // 371 => [1062]
            map.insert(480, &[2118]);     // 480 => [2118]
            map.insert(467, &[1158]);     // 467 => [1158]
            map.insert(156, &[1542]);     // 156 => [1542]
            map.insert(111, &[2566]);     // 111 => [2566]
            map.insert(245, &[3078]);     // 245 => [3078]
            map.insert(58, &[58]);     // 58 => [58]
            map.insert(90, &[90]);     // 90 => [90]
            map.insert(282, &[282]);     // 282 => [282]
            map.insert(444, &[2074]);     // 444 => [2074]
            map.insert(170, &[170]);     // 170 => [170]
            map.insert(298, &[298]);     // 298 => [298]
            map.insert(383, &[1066]);     // 383 => [1066]
            map.insert(287, &[1098]);     // 287 => [1098]
            map.insert(300, &[2186]);     // 300 => [2186]
            map.insert(95, &[1290]);     // 95 => [1290]
            map.insert(172, &[2314]);     // 172 => [2314]
            map.insert(249, &[3082]);     // 249 => [3082]
            map.insert(306, &[306]);     // 306 => [306]
            map.insert(509, &[562]);     // 509 => [562]
            map.insert(413, &[594]);     // 413 => [594]
            map.insert(402, &[402]);     // 402 => [402]
            map.insert(221, &[786]);     // 221 => [786]
            map.insert(180, &[2322]);     // 180 => [2322]
            map.insert(123, &[2578]);     // 123 => [2578]
            map.insert(354, &[354]);     // 354 => [354]
            map.insert(365, &[674]);     // 365 => [674]
            map.insert(237, &[802]);     // 237 => [802]
            map.insert(119, &[1314]);     // 119 => [1314]
            map.insert(184, &[1570]);     // 184 => [1570]
            map.insert(228, &[2370]);     // 228 => [2370]
            map.insert(216, &[1602]);     // 216 => [1602]
            map.insert(215, &[1410]);     // 215 => [1410]
            map.insert(235, &[2690]);     // 235 => [2690]
            map.insert(408, &[1794]);     // 408 => [1794]
            map.insert(363, &[2818]);     // 363 => [2818]
            map.insert(497, &[3330]);     // 497 => [3330]
            map.insert(318, &[3586]);     // 318 => [3586]
            map.insert(60, &[60]);     // 60 => [60]
            map.insert(442, &[2076]);     // 442 => [2076]
            map.insert(377, &[1068]);     // 377 => [1068]
            map.insert(255, &[3084]);     // 255 => [3084]
            map.insert(507, &[564]);     // 507 => [564]
            map.insert(125, &[2580]);     // 125 => [2580]
            map.insert(190, &[1572]);     // 190 => [1572]
            map.insert(312, &[3588]);     // 312 => [3588]
            map.insert(31, &[31]);     // 31 => [31]
            map.insert(47, &[47]);     // 47 => [47]
            map.insert(346, &[1039]);     // 346 => [1039]
            map.insert(425, &[2063]);     // 425 => [2063]
            map.insert(55, &[55]);     // 55 => [55]
            map.insert(151, &[151]);     // 151 => [151]
            map.insert(472, &[535]);     // 472 => [535]
            map.insert(433, &[2071]);     // 433 => [2071]
            map.insert(103, &[103]);     // 103 => [103]
            map.insert(488, &[551]);     // 488 => [551]
            map.insert(370, &[1063]);     // 370 => [1063]
            map.insert(481, &[2119]);     // 481 => [2119]
            map.insert(466, &[1159]);     // 466 => [1159]
            map.insert(157, &[1543]);     // 157 => [1543]
            map.insert(110, &[2567]);     // 110 => [2567]
            map.insert(244, &[3079]);     // 244 => [3079]
            map.insert(59, &[59]);     // 59 => [59]
            map.insert(91, &[91]);     // 91 => [91]
            map.insert(283, &[283]);     // 283 => [283]
            map.insert(445, &[2075]);     // 445 => [2075]
            map.insert(171, &[171]);     // 171 => [171]
            map.insert(299, &[299]);     // 299 => [299]
            map.insert(382, &[1067]);     // 382 => [1067]
            map.insert(286, &[1099]);     // 286 => [1099]
            map.insert(301, &[2187]);     // 301 => [2187]
            map.insert(94, &[1291]);     // 94 => [1291]
            map.insert(173, &[2315]);     // 173 => [2315]
            map.insert(248, &[3083]);     // 248 => [3083]
            map.insert(307, &[307]);     // 307 => [307]
            map.insert(508, &[563]);     // 508 => [563]
            map.insert(412, &[595]);     // 412 => [595]
            map.insert(403, &[403]);     // 403 => [403]
            map.insert(220, &[787]);     // 220 => [787]
            map.insert(181, &[2323]);     // 181 => [2323]
            map.insert(122, &[2579]);     // 122 => [2579]
            map.insert(355, &[355]);     // 355 => [355]
            map.insert(364, &[675]);     // 364 => [675]
            map.insert(236, &[803]);     // 236 => [803]
            map.insert(118, &[1315]);     // 118 => [1315]
            map.insert(185, &[1571]);     // 185 => [1571]
            map.insert(229, &[2371]);     // 229 => [2371]
            map.insert(217, &[1603]);     // 217 => [1603]
            map.insert(214, &[1411]);     // 214 => [1411]
            map.insert(234, &[2691]);     // 234 => [2691]
            map.insert(409, &[1795]);     // 409 => [1795]
            map.insert(362, &[2819]);     // 362 => [2819]
            map.insert(496, &[3331]);     // 496 => [3331]
            map.insert(319, &[3587]);     // 319 => [3587]
            map.insert(61, &[61]);     // 61 => [61]
            map.insert(443, &[2077]);     // 443 => [2077]
            map.insert(376, &[1069]);     // 376 => [1069]
            map.insert(254, &[3085]);     // 254 => [3085]
            map.insert(506, &[565]);     // 506 => [565]
            map.insert(124, &[2581]);     // 124 => [2581]
            map.insert(191, &[1573]);     // 191 => [1573]
            map.insert(313, &[3589]);     // 313 => [3589]
            map.insert(62, &[62]);     // 62 => [62]
            map.insert(440, &[2078]);     // 440 => [2078]
            map.insert(379, &[1070]);     // 379 => [1070]
            map.insert(253, &[3086]);     // 253 => [3086]
            map.insert(505, &[566]);     // 505 => [566]
            map.insert(127, &[2582]);     // 127 => [2582]
            map.insert(188, &[1574]);     // 188 => [1574]
            map.insert(314, &[3590]);     // 314 => [3590]
            map.insert(63, &[63]);     // 63 => [63]
            map.insert(441, &[2079]);     // 441 => [2079]
            map.insert(378, &[1071]);     // 378 => [1071]
            map.insert(252, &[3087]);     // 252 => [3087]
            map.insert(504, &[567]);     // 504 => [567]
            map.insert(126, &[2583]);     // 126 => [2583]
            map.insert(189, &[1575]);     // 189 => [1575]
            map.insert(315, &[3591]);     // 315 => [3591]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode12_3 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode12_3 {
    fn name(&self) -> String {
        "[12, 3] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        12
    }

    fn dimension(&self) -> usize {
        3
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
        let mut error = BinVector::with_capacity(12);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 12 / 64 + if 12 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(12) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(3);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[12 / 64] & !((1 << 12) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode12_3.generator_matrix();
        assert_eq!(code.ncols(), 12);
        assert_eq!(code.nrows(), 3);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode12_3;
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
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, true, false, true, true, false, false, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
