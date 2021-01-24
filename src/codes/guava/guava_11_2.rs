use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[11, 2]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode11_2;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1949 ],
                &[ 2018 ],
                
            ], 11));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1089 ],
                &[ 66 ],
                &[ 1092 ],
                &[ 1096 ],
                &[ 1104 ],
                &[ 96 ],
                &[ 1152 ],
                &[ 1280 ],
                &[ 1536 ],
                
            ], 11));
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
            map.insert(63, &[64]);     // 63 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(256, &[512]);     // 256 => [512]
            map.insert(477, &[1024]);     // 477 => [1024]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(62, &[65]);     // 62 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(257, &[513]);     // 257 => [513]
            map.insert(476, &[1025]);     // 476 => [1025]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(61, &[66]);     // 61 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(258, &[514]);     // 258 => [514]
            map.insert(479, &[1026]);     // 479 => [1026]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(59, &[68]);     // 59 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(260, &[516]);     // 260 => [516]
            map.insert(473, &[1028]);     // 473 => [1028]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(55, &[72]);     // 55 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(264, &[520]);     // 264 => [520]
            map.insert(469, &[1032]);     // 469 => [1032]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(47, &[80]);     // 47 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(272, &[528]);     // 272 => [528]
            map.insert(461, &[1040]);     // 461 => [1040]
            map.insert(31, &[96]);     // 31 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(288, &[544]);     // 288 => [544]
            map.insert(509, &[1056]);     // 509 => [1056]
            map.insert(127, &[192]);     // 127 => [192]
            map.insert(191, &[320]);     // 191 => [320]
            map.insert(319, &[576]);     // 319 => [576]
            map.insert(482, &[1088]);     // 482 => [1088]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(320, &[640]);     // 320 => [640]
            map.insert(413, &[1152]);     // 413 => [1152]
            map.insert(384, &[768]);     // 384 => [768]
            map.insert(349, &[1280]);     // 349 => [1280]
            map.insert(221, &[1536]);     // 221 => [1536]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(60, &[67]);     // 60 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(259, &[515]);     // 259 => [515]
            map.insert(478, &[1027]);     // 478 => [1027]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(58, &[69]);     // 58 => [69]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(261, &[517]);     // 261 => [517]
            map.insert(472, &[1029]);     // 472 => [1029]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(54, &[73]);     // 54 => [73]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(265, &[521]);     // 265 => [521]
            map.insert(468, &[1033]);     // 468 => [1033]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(46, &[81]);     // 46 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(145, &[273]);     // 145 => [273]
            map.insert(273, &[529]);     // 273 => [529]
            map.insert(460, &[1041]);     // 460 => [1041]
            map.insert(30, &[97]);     // 30 => [97]
            map.insert(97, &[161]);     // 97 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(289, &[545]);     // 289 => [545]
            map.insert(508, &[1057]);     // 508 => [1057]
            map.insert(126, &[193]);     // 126 => [193]
            map.insert(190, &[321]);     // 190 => [321]
            map.insert(318, &[577]);     // 318 => [577]
            map.insert(483, &[1089]);     // 483 => [1089]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(321, &[641]);     // 321 => [641]
            map.insert(412, &[1153]);     // 412 => [1153]
            map.insert(385, &[769]);     // 385 => [769]
            map.insert(348, &[1281]);     // 348 => [1281]
            map.insert(220, &[1537]);     // 220 => [1537]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(57, &[70]);     // 57 => [70]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(262, &[518]);     // 262 => [518]
            map.insert(475, &[1030]);     // 475 => [1030]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(53, &[74]);     // 53 => [74]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(266, &[522]);     // 266 => [522]
            map.insert(471, &[1034]);     // 471 => [1034]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(45, &[82]);     // 45 => [82]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(146, &[274]);     // 146 => [274]
            map.insert(274, &[530]);     // 274 => [530]
            map.insert(463, &[1042]);     // 463 => [1042]
            map.insert(29, &[98]);     // 29 => [98]
            map.insert(98, &[162]);     // 98 => [162]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(290, &[546]);     // 290 => [546]
            map.insert(511, &[1058]);     // 511 => [1058]
            map.insert(125, &[194]);     // 125 => [194]
            map.insert(189, &[322]);     // 189 => [322]
            map.insert(317, &[578]);     // 317 => [578]
            map.insert(480, &[1090]);     // 480 => [1090]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(322, &[642]);     // 322 => [642]
            map.insert(415, &[1154]);     // 415 => [1154]
            map.insert(386, &[770]);     // 386 => [770]
            map.insert(351, &[1282]);     // 351 => [1282]
            map.insert(223, &[1538]);     // 223 => [1538]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(51, &[76]);     // 51 => [76]
            map.insert(76, &[140]);     // 76 => [140]
            map.insert(140, &[268]);     // 140 => [268]
            map.insert(268, &[524]);     // 268 => [524]
            map.insert(465, &[1036]);     // 465 => [1036]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(43, &[84]);     // 43 => [84]
            map.insert(84, &[148]);     // 84 => [148]
            map.insert(148, &[276]);     // 148 => [276]
            map.insert(276, &[532]);     // 276 => [532]
            map.insert(457, &[1044]);     // 457 => [1044]
            map.insert(27, &[100]);     // 27 => [100]
            map.insert(100, &[164]);     // 100 => [164]
            map.insert(164, &[292]);     // 164 => [292]
            map.insert(292, &[548]);     // 292 => [548]
            map.insert(505, &[1060]);     // 505 => [1060]
            map.insert(123, &[196]);     // 123 => [196]
            map.insert(187, &[324]);     // 187 => [324]
            map.insert(315, &[580]);     // 315 => [580]
            map.insert(486, &[1092]);     // 486 => [1092]
            map.insert(196, &[388]);     // 196 => [388]
            map.insert(324, &[644]);     // 324 => [644]
            map.insert(409, &[1156]);     // 409 => [1156]
            map.insert(388, &[772]);     // 388 => [772]
            map.insert(345, &[1284]);     // 345 => [1284]
            map.insert(217, &[1540]);     // 217 => [1540]
            map.insert(56, &[56]);     // 56 => [56]
            map.insert(39, &[88]);     // 39 => [88]
            map.insert(88, &[152]);     // 88 => [152]
            map.insert(152, &[280]);     // 152 => [280]
            map.insert(280, &[536]);     // 280 => [536]
            map.insert(453, &[1048]);     // 453 => [1048]
            map.insert(23, &[104]);     // 23 => [104]
            map.insert(104, &[168]);     // 104 => [168]
            map.insert(168, &[296]);     // 168 => [296]
            map.insert(296, &[552]);     // 296 => [552]
            map.insert(501, &[1064]);     // 501 => [1064]
            map.insert(119, &[200]);     // 119 => [200]
            map.insert(183, &[328]);     // 183 => [328]
            map.insert(311, &[584]);     // 311 => [584]
            map.insert(490, &[1096]);     // 490 => [1096]
            map.insert(200, &[392]);     // 200 => [392]
            map.insert(328, &[648]);     // 328 => [648]
            map.insert(405, &[1160]);     // 405 => [1160]
            map.insert(392, &[776]);     // 392 => [776]
            map.insert(341, &[1288]);     // 341 => [1288]
            map.insert(213, &[1544]);     // 213 => [1544]
            map.insert(15, &[112]);     // 15 => [112]
            map.insert(112, &[176]);     // 112 => [176]
            map.insert(176, &[304]);     // 176 => [304]
            map.insert(304, &[560]);     // 304 => [560]
            map.insert(493, &[1072]);     // 493 => [1072]
            map.insert(111, &[208]);     // 111 => [208]
            map.insert(175, &[336]);     // 175 => [336]
            map.insert(303, &[592]);     // 303 => [592]
            map.insert(498, &[1104]);     // 498 => [1104]
            map.insert(208, &[400]);     // 208 => [400]
            map.insert(336, &[656]);     // 336 => [656]
            map.insert(397, &[1168]);     // 397 => [1168]
            map.insert(400, &[784]);     // 400 => [784]
            map.insert(333, &[1296]);     // 333 => [1296]
            map.insert(205, &[1552]);     // 205 => [1552]
            map.insert(95, &[224]);     // 95 => [224]
            map.insert(159, &[352]);     // 159 => [352]
            map.insert(287, &[608]);     // 287 => [608]
            map.insert(450, &[1120]);     // 450 => [1120]
            map.insert(224, &[416]);     // 224 => [416]
            map.insert(352, &[672]);     // 352 => [672]
            map.insert(445, &[1184]);     // 445 => [1184]
            map.insert(416, &[800]);     // 416 => [800]
            map.insert(381, &[1312]);     // 381 => [1312]
            map.insert(253, &[1568]);     // 253 => [1568]
            map.insert(255, &[448]);     // 255 => [448]
            map.insert(383, &[704]);     // 383 => [704]
            map.insert(418, &[1216]);     // 418 => [1216]
            map.insert(447, &[832]);     // 447 => [832]
            map.insert(354, &[1344]);     // 354 => [1344]
            map.insert(226, &[1600]);     // 226 => [1600]
            map.insert(448, &[896]);     // 448 => [896]
            map.insert(285, &[1408]);     // 285 => [1408]
            map.insert(157, &[1664]);     // 157 => [1664]
            map.insert(93, &[1792]);     // 93 => [1792]
            map.insert(71, &[135]);     // 71 => [135]
            map.insert(135, &[263]);     // 135 => [263]
            map.insert(263, &[519]);     // 263 => [519]
            map.insert(474, &[1031]);     // 474 => [1031]
            map.insert(75, &[139]);     // 75 => [139]
            map.insert(139, &[267]);     // 139 => [267]
            map.insert(267, &[523]);     // 267 => [523]
            map.insert(470, &[1035]);     // 470 => [1035]
            map.insert(83, &[147]);     // 83 => [147]
            map.insert(147, &[275]);     // 147 => [275]
            map.insert(275, &[531]);     // 275 => [531]
            map.insert(462, &[1043]);     // 462 => [1043]
            map.insert(99, &[163]);     // 99 => [163]
            map.insert(163, &[291]);     // 163 => [291]
            map.insert(291, &[547]);     // 291 => [547]
            map.insert(510, &[1059]);     // 510 => [1059]
            map.insert(124, &[195]);     // 124 => [195]
            map.insert(188, &[323]);     // 188 => [323]
            map.insert(316, &[579]);     // 316 => [579]
            map.insert(481, &[1091]);     // 481 => [1091]
            map.insert(195, &[387]);     // 195 => [387]
            map.insert(323, &[643]);     // 323 => [643]
            map.insert(414, &[1155]);     // 414 => [1155]
            map.insert(387, &[771]);     // 387 => [771]
            map.insert(350, &[1283]);     // 350 => [1283]
            map.insert(222, &[1539]);     // 222 => [1539]
            map.insert(77, &[141]);     // 77 => [141]
            map.insert(141, &[269]);     // 141 => [269]
            map.insert(269, &[525]);     // 269 => [525]
            map.insert(464, &[1037]);     // 464 => [1037]
            map.insert(85, &[149]);     // 85 => [149]
            map.insert(149, &[277]);     // 149 => [277]
            map.insert(277, &[533]);     // 277 => [533]
            map.insert(456, &[1045]);     // 456 => [1045]
            map.insert(101, &[165]);     // 101 => [165]
            map.insert(165, &[293]);     // 165 => [293]
            map.insert(293, &[549]);     // 293 => [549]
            map.insert(504, &[1061]);     // 504 => [1061]
            map.insert(122, &[197]);     // 122 => [197]
            map.insert(186, &[325]);     // 186 => [325]
            map.insert(314, &[581]);     // 314 => [581]
            map.insert(487, &[1093]);     // 487 => [1093]
            map.insert(197, &[389]);     // 197 => [389]
            map.insert(325, &[645]);     // 325 => [645]
            map.insert(408, &[1157]);     // 408 => [1157]
            map.insert(389, &[773]);     // 389 => [773]
            map.insert(344, &[1285]);     // 344 => [1285]
            map.insert(216, &[1541]);     // 216 => [1541]
            map.insert(89, &[153]);     // 89 => [153]
            map.insert(153, &[281]);     // 153 => [281]
            map.insert(281, &[537]);     // 281 => [537]
            map.insert(452, &[1049]);     // 452 => [1049]
            map.insert(105, &[169]);     // 105 => [169]
            map.insert(169, &[297]);     // 169 => [297]
            map.insert(297, &[553]);     // 297 => [553]
            map.insert(500, &[1065]);     // 500 => [1065]
            map.insert(118, &[201]);     // 118 => [201]
            map.insert(182, &[329]);     // 182 => [329]
            map.insert(310, &[585]);     // 310 => [585]
            map.insert(491, &[1097]);     // 491 => [1097]
            map.insert(201, &[393]);     // 201 => [393]
            map.insert(329, &[649]);     // 329 => [649]
            map.insert(404, &[1161]);     // 404 => [1161]
            map.insert(393, &[777]);     // 393 => [777]
            map.insert(340, &[1289]);     // 340 => [1289]
            map.insert(212, &[1545]);     // 212 => [1545]
            map.insert(113, &[177]);     // 113 => [177]
            map.insert(177, &[305]);     // 177 => [305]
            map.insert(305, &[561]);     // 305 => [561]
            map.insert(492, &[1073]);     // 492 => [1073]
            map.insert(110, &[209]);     // 110 => [209]
            map.insert(174, &[337]);     // 174 => [337]
            map.insert(302, &[593]);     // 302 => [593]
            map.insert(499, &[1105]);     // 499 => [1105]
            map.insert(209, &[401]);     // 209 => [401]
            map.insert(337, &[657]);     // 337 => [657]
            map.insert(396, &[1169]);     // 396 => [1169]
            map.insert(401, &[785]);     // 401 => [785]
            map.insert(332, &[1297]);     // 332 => [1297]
            map.insert(204, &[1553]);     // 204 => [1553]
            map.insert(94, &[225]);     // 94 => [225]
            map.insert(158, &[353]);     // 158 => [353]
            map.insert(286, &[609]);     // 286 => [609]
            map.insert(451, &[1121]);     // 451 => [1121]
            map.insert(225, &[417]);     // 225 => [417]
            map.insert(353, &[673]);     // 353 => [673]
            map.insert(444, &[1185]);     // 444 => [1185]
            map.insert(417, &[801]);     // 417 => [801]
            map.insert(380, &[1313]);     // 380 => [1313]
            map.insert(252, &[1569]);     // 252 => [1569]
            map.insert(254, &[449]);     // 254 => [449]
            map.insert(382, &[705]);     // 382 => [705]
            map.insert(419, &[1217]);     // 419 => [1217]
            map.insert(446, &[833]);     // 446 => [833]
            map.insert(355, &[1345]);     // 355 => [1345]
            map.insert(227, &[1601]);     // 227 => [1601]
            map.insert(449, &[897]);     // 449 => [897]
            map.insert(284, &[1409]);     // 284 => [1409]
            map.insert(156, &[1665]);     // 156 => [1665]
            map.insert(92, &[1793]);     // 92 => [1793]
            map.insert(78, &[142]);     // 78 => [142]
            map.insert(142, &[270]);     // 142 => [270]
            map.insert(270, &[526]);     // 270 => [526]
            map.insert(467, &[1038]);     // 467 => [1038]
            map.insert(86, &[150]);     // 86 => [150]
            map.insert(150, &[278]);     // 150 => [278]
            map.insert(278, &[534]);     // 278 => [534]
            map.insert(459, &[1046]);     // 459 => [1046]
            map.insert(102, &[166]);     // 102 => [166]
            map.insert(166, &[294]);     // 166 => [294]
            map.insert(294, &[550]);     // 294 => [550]
            map.insert(507, &[1062]);     // 507 => [1062]
            map.insert(121, &[198]);     // 121 => [198]
            map.insert(185, &[326]);     // 185 => [326]
            map.insert(313, &[582]);     // 313 => [582]
            map.insert(484, &[1094]);     // 484 => [1094]
            map.insert(198, &[390]);     // 198 => [390]
            map.insert(326, &[646]);     // 326 => [646]
            map.insert(411, &[1158]);     // 411 => [1158]
            map.insert(390, &[774]);     // 390 => [774]
            map.insert(347, &[1286]);     // 347 => [1286]
            map.insert(219, &[1542]);     // 219 => [1542]
            map.insert(90, &[154]);     // 90 => [154]
            map.insert(154, &[282]);     // 154 => [282]
            map.insert(282, &[538]);     // 282 => [538]
            map.insert(455, &[1050]);     // 455 => [1050]
            map.insert(106, &[170]);     // 106 => [170]
            map.insert(170, &[298]);     // 170 => [298]
            map.insert(298, &[554]);     // 298 => [554]
            map.insert(503, &[1066]);     // 503 => [1066]
            map.insert(117, &[202]);     // 117 => [202]
            map.insert(181, &[330]);     // 181 => [330]
            map.insert(309, &[586]);     // 309 => [586]
            map.insert(488, &[1098]);     // 488 => [1098]
            map.insert(202, &[394]);     // 202 => [394]
            map.insert(330, &[650]);     // 330 => [650]
            map.insert(407, &[1162]);     // 407 => [1162]
            map.insert(394, &[778]);     // 394 => [778]
            map.insert(343, &[1290]);     // 343 => [1290]
            map.insert(215, &[1546]);     // 215 => [1546]
            map.insert(114, &[178]);     // 114 => [178]
            map.insert(178, &[306]);     // 178 => [306]
            map.insert(306, &[562]);     // 306 => [562]
            map.insert(495, &[1074]);     // 495 => [1074]
            map.insert(109, &[210]);     // 109 => [210]
            map.insert(173, &[338]);     // 173 => [338]
            map.insert(301, &[594]);     // 301 => [594]
            map.insert(496, &[1106]);     // 496 => [1106]
            map.insert(210, &[402]);     // 210 => [402]
            map.insert(338, &[658]);     // 338 => [658]
            map.insert(399, &[1170]);     // 399 => [1170]
            map.insert(402, &[786]);     // 402 => [786]
            map.insert(335, &[1298]);     // 335 => [1298]
            map.insert(207, &[1554]);     // 207 => [1554]
            map.insert(108, &[172]);     // 108 => [172]
            map.insert(172, &[300]);     // 172 => [300]
            map.insert(300, &[556]);     // 300 => [556]
            map.insert(497, &[1068]);     // 497 => [1068]
            map.insert(115, &[204]);     // 115 => [204]
            map.insert(179, &[332]);     // 179 => [332]
            map.insert(307, &[588]);     // 307 => [588]
            map.insert(494, &[1100]);     // 494 => [1100]
            map.insert(116, &[180]);     // 116 => [180]
            map.insert(180, &[308]);     // 180 => [308]
            map.insert(308, &[564]);     // 308 => [564]
            map.insert(489, &[1076]);     // 489 => [1076]
            map.insert(107, &[212]);     // 107 => [212]
            map.insert(171, &[340]);     // 171 => [340]
            map.insert(299, &[596]);     // 299 => [596]
            map.insert(502, &[1108]);     // 502 => [1108]
            map.insert(91, &[228]);     // 91 => [228]
            map.insert(155, &[356]);     // 155 => [356]
            map.insert(283, &[612]);     // 283 => [612]
            map.insert(454, &[1124]);     // 454 => [1124]
            map.insert(228, &[420]);     // 228 => [420]
            map.insert(356, &[676]);     // 356 => [676]
            map.insert(441, &[1188]);     // 441 => [1188]
            map.insert(420, &[804]);     // 420 => [804]
            map.insert(377, &[1316]);     // 377 => [1316]
            map.insert(249, &[1572]);     // 249 => [1572]
            map.insert(251, &[452]);     // 251 => [452]
            map.insert(379, &[708]);     // 379 => [708]
            map.insert(422, &[1220]);     // 422 => [1220]
            map.insert(443, &[836]);     // 443 => [836]
            map.insert(358, &[1348]);     // 358 => [1348]
            map.insert(230, &[1604]);     // 230 => [1604]
            map.insert(120, &[184]);     // 120 => [184]
            map.insert(184, &[312]);     // 184 => [312]
            map.insert(312, &[568]);     // 312 => [568]
            map.insert(485, &[1080]);     // 485 => [1080]
            map.insert(103, &[216]);     // 103 => [216]
            map.insert(167, &[344]);     // 167 => [344]
            map.insert(295, &[600]);     // 295 => [600]
            map.insert(506, &[1112]);     // 506 => [1112]
            map.insert(87, &[232]);     // 87 => [232]
            map.insert(151, &[360]);     // 151 => [360]
            map.insert(279, &[616]);     // 279 => [616]
            map.insert(458, &[1128]);     // 458 => [1128]
            map.insert(232, &[424]);     // 232 => [424]
            map.insert(360, &[680]);     // 360 => [680]
            map.insert(437, &[1192]);     // 437 => [1192]
            map.insert(424, &[808]);     // 424 => [808]
            map.insert(373, &[1320]);     // 373 => [1320]
            map.insert(245, &[1576]);     // 245 => [1576]
            map.insert(247, &[456]);     // 247 => [456]
            map.insert(375, &[712]);     // 375 => [712]
            map.insert(426, &[1224]);     // 426 => [1224]
            map.insert(439, &[840]);     // 439 => [840]
            map.insert(362, &[1352]);     // 362 => [1352]
            map.insert(234, &[1608]);     // 234 => [1608]
            map.insert(79, &[240]);     // 79 => [240]
            map.insert(143, &[368]);     // 143 => [368]
            map.insert(271, &[624]);     // 271 => [624]
            map.insert(466, &[1136]);     // 466 => [1136]
            map.insert(240, &[432]);     // 240 => [432]
            map.insert(368, &[688]);     // 368 => [688]
            map.insert(429, &[1200]);     // 429 => [1200]
            map.insert(432, &[816]);     // 432 => [816]
            map.insert(365, &[1328]);     // 365 => [1328]
            map.insert(237, &[1584]);     // 237 => [1584]
            map.insert(239, &[464]);     // 239 => [464]
            map.insert(367, &[720]);     // 367 => [720]
            map.insert(434, &[1232]);     // 434 => [1232]
            map.insert(431, &[848]);     // 431 => [848]
            map.insert(370, &[1360]);     // 370 => [1360]
            map.insert(242, &[1616]);     // 242 => [1616]
            map.insert(199, &[391]);     // 199 => [391]
            map.insert(327, &[647]);     // 327 => [647]
            map.insert(410, &[1159]);     // 410 => [1159]
            map.insert(391, &[775]);     // 391 => [775]
            map.insert(346, &[1287]);     // 346 => [1287]
            map.insert(218, &[1543]);     // 218 => [1543]
            map.insert(203, &[395]);     // 203 => [395]
            map.insert(331, &[651]);     // 331 => [651]
            map.insert(406, &[1163]);     // 406 => [1163]
            map.insert(395, &[779]);     // 395 => [779]
            map.insert(342, &[1291]);     // 342 => [1291]
            map.insert(214, &[1547]);     // 214 => [1547]
            map.insert(211, &[403]);     // 211 => [403]
            map.insert(339, &[659]);     // 339 => [659]
            map.insert(398, &[1171]);     // 398 => [1171]
            map.insert(403, &[787]);     // 403 => [787]
            map.insert(334, &[1299]);     // 334 => [1299]
            map.insert(206, &[1555]);     // 206 => [1555]
            map.insert(229, &[421]);     // 229 => [421]
            map.insert(357, &[677]);     // 357 => [677]
            map.insert(440, &[1189]);     // 440 => [1189]
            map.insert(421, &[805]);     // 421 => [805]
            map.insert(376, &[1317]);     // 376 => [1317]
            map.insert(248, &[1573]);     // 248 => [1573]
            map.insert(250, &[453]);     // 250 => [453]
            map.insert(378, &[709]);     // 378 => [709]
            map.insert(423, &[1221]);     // 423 => [1221]
            map.insert(442, &[837]);     // 442 => [837]
            map.insert(359, &[1349]);     // 359 => [1349]
            map.insert(231, &[1605]);     // 231 => [1605]
            map.insert(233, &[425]);     // 233 => [425]
            map.insert(361, &[681]);     // 361 => [681]
            map.insert(436, &[1193]);     // 436 => [1193]
            map.insert(425, &[809]);     // 425 => [809]
            map.insert(372, &[1321]);     // 372 => [1321]
            map.insert(244, &[1577]);     // 244 => [1577]
            map.insert(246, &[457]);     // 246 => [457]
            map.insert(374, &[713]);     // 374 => [713]
            map.insert(427, &[1225]);     // 427 => [1225]
            map.insert(438, &[841]);     // 438 => [841]
            map.insert(363, &[1353]);     // 363 => [1353]
            map.insert(235, &[1609]);     // 235 => [1609]
            map.insert(241, &[433]);     // 241 => [433]
            map.insert(369, &[689]);     // 369 => [689]
            map.insert(428, &[1201]);     // 428 => [1201]
            map.insert(433, &[817]);     // 433 => [817]
            map.insert(364, &[1329]);     // 364 => [1329]
            map.insert(236, &[1585]);     // 236 => [1585]
            map.insert(238, &[465]);     // 238 => [465]
            map.insert(366, &[721]);     // 366 => [721]
            map.insert(435, &[1233]);     // 435 => [1233]
            map.insert(430, &[849]);     // 430 => [849]
            map.insert(371, &[1361]);     // 371 => [1361]
            map.insert(243, &[1617]);     // 243 => [1617]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode11_2 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode11_2 {
    fn name(&self) -> String {
        "[11, 2] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        11
    }

    fn dimension(&self) -> usize {
        2
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
        let mut error = BinVector::with_capacity(11);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 11 / 64 + if 11 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(11) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(2);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[11 / 64] & !((1 << 11) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode11_2.generator_matrix();
        assert_eq!(code.ncols(), 11);
        assert_eq!(code.nrows(), 2);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode11_2;
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
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_2;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, true, true, true, false, false, true, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
