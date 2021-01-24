use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[15, 6]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode15_6;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 19649 ],
                &[ 22914 ],
                &[ 29444 ],
                &[ 5064 ],
                &[ 10128 ],
                &[ 31456 ],
                
            ], 15));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 29185 ],
                &[ 25602 ],
                &[ 18436 ],
                &[ 8712 ],
                &[ 30224 ],
                &[ 7712 ],
                &[ 20032 ],
                &[ 23680 ],
                &[ 30976 ],
                
            ], 15));
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
            map.insert(433, &[4096]);     // 433 => [4096]
            map.insert(283, &[8192]);     // 283 => [8192]
            map.insert(471, &[16384]);     // 471 => [16384]
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
            map.insert(432, &[4097]);     // 432 => [4097]
            map.insert(282, &[8193]);     // 282 => [8193]
            map.insert(470, &[16385]);     // 470 => [16385]
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
            map.insert(435, &[4098]);     // 435 => [4098]
            map.insert(281, &[8194]);     // 281 => [8194]
            map.insert(469, &[16386]);     // 469 => [16386]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(260, &[260]);     // 260 => [260]
            map.insert(125, &[516]);     // 125 => [516]
            map.insert(246, &[1028]);     // 246 => [1028]
            map.insert(480, &[2052]);     // 480 => [2052]
            map.insert(437, &[4100]);     // 437 => [4100]
            map.insert(287, &[8196]);     // 287 => [8196]
            map.insert(467, &[16388]);     // 467 => [16388]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(264, &[264]);     // 264 => [264]
            map.insert(113, &[520]);     // 113 => [520]
            map.insert(250, &[1032]);     // 250 => [1032]
            map.insert(492, &[2056]);     // 492 => [2056]
            map.insert(441, &[4104]);     // 441 => [4104]
            map.insert(275, &[8200]);     // 275 => [8200]
            map.insert(479, &[16392]);     // 479 => [16392]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(272, &[272]);     // 272 => [272]
            map.insert(105, &[528]);     // 105 => [528]
            map.insert(226, &[1040]);     // 226 => [1040]
            map.insert(500, &[2064]);     // 500 => [2064]
            map.insert(417, &[4112]);     // 417 => [4112]
            map.insert(267, &[8208]);     // 267 => [8208]
            map.insert(455, &[16400]);     // 455 => [16400]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(288, &[288]);     // 288 => [288]
            map.insert(89, &[544]);     // 89 => [544]
            map.insert(210, &[1056]);     // 210 => [1056]
            map.insert(452, &[2080]);     // 452 => [2080]
            map.insert(401, &[4128]);     // 401 => [4128]
            map.insert(315, &[8224]);     // 315 => [8224]
            map.insert(503, &[16416]);     // 503 => [16416]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(320, &[320]);     // 320 => [320]
            map.insert(57, &[576]);     // 57 => [576]
            map.insert(178, &[1088]);     // 178 => [1088]
            map.insert(420, &[2112]);     // 420 => [2112]
            map.insert(497, &[4160]);     // 497 => [4160]
            map.insert(347, &[8256]);     // 347 => [8256]
            map.insert(407, &[16448]);     // 407 => [16448]
            map.insert(384, &[384]);     // 384 => [384]
            map.insert(249, &[640]);     // 249 => [640]
            map.insert(114, &[1152]);     // 114 => [1152]
            map.insert(356, &[2176]);     // 356 => [2176]
            map.insert(305, &[4224]);     // 305 => [4224]
            map.insert(411, &[8320]);     // 411 => [8320]
            map.insert(343, &[16512]);     // 343 => [16512]
            map.insert(377, &[768]);     // 377 => [768]
            map.insert(498, &[1280]);     // 498 => [1280]
            map.insert(228, &[2304]);     // 228 => [2304]
            map.insert(177, &[4352]);     // 177 => [4352]
            map.insert(27, &[8448]);     // 27 => [8448]
            map.insert(215, &[16640]);     // 215 => [16640]
            map.insert(139, &[1536]);     // 139 => [1536]
            map.insert(413, &[2560]);     // 413 => [2560]
            map.insert(456, &[4608]);     // 456 => [4608]
            map.insert(354, &[8704]);     // 354 => [8704]
            map.insert(430, &[16896]);     // 430 => [16896]
            map.insert(278, &[3072]);     // 278 => [3072]
            map.insert(323, &[5120]);     // 323 => [5120]
            map.insert(489, &[9216]);     // 489 => [9216]
            map.insert(293, &[17408]);     // 293 => [17408]
            map.insert(85, &[6144]);     // 85 => [6144]
            map.insert(255, &[10240]);     // 255 => [10240]
            map.insert(51, &[18432]);     // 51 => [18432]
            map.insert(170, &[12288]);     // 170 => [12288]
            map.insert(102, &[20480]);     // 102 => [20480]
            map.insert(204, &[24576]);     // 204 => [24576]
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
            map.insert(434, &[4099]);     // 434 => [4099]
            map.insert(280, &[8195]);     // 280 => [8195]
            map.insert(468, &[16387]);     // 468 => [16387]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(261, &[261]);     // 261 => [261]
            map.insert(124, &[517]);     // 124 => [517]
            map.insert(247, &[1029]);     // 247 => [1029]
            map.insert(481, &[2053]);     // 481 => [2053]
            map.insert(436, &[4101]);     // 436 => [4101]
            map.insert(286, &[8197]);     // 286 => [8197]
            map.insert(466, &[16389]);     // 466 => [16389]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(265, &[265]);     // 265 => [265]
            map.insert(112, &[521]);     // 112 => [521]
            map.insert(251, &[1033]);     // 251 => [1033]
            map.insert(493, &[2057]);     // 493 => [2057]
            map.insert(440, &[4105]);     // 440 => [4105]
            map.insert(274, &[8201]);     // 274 => [8201]
            map.insert(478, &[16393]);     // 478 => [16393]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(81, &[81]);     // 81 => [81]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(273, &[273]);     // 273 => [273]
            map.insert(104, &[529]);     // 104 => [529]
            map.insert(227, &[1041]);     // 227 => [1041]
            map.insert(501, &[2065]);     // 501 => [2065]
            map.insert(416, &[4113]);     // 416 => [4113]
            map.insert(266, &[8209]);     // 266 => [8209]
            map.insert(454, &[16401]);     // 454 => [16401]
            map.insert(97, &[97]);     // 97 => [97]
            map.insert(161, &[161]);     // 161 => [161]
            map.insert(289, &[289]);     // 289 => [289]
            map.insert(88, &[545]);     // 88 => [545]
            map.insert(211, &[1057]);     // 211 => [1057]
            map.insert(453, &[2081]);     // 453 => [2081]
            map.insert(400, &[4129]);     // 400 => [4129]
            map.insert(314, &[8225]);     // 314 => [8225]
            map.insert(502, &[16417]);     // 502 => [16417]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(321, &[321]);     // 321 => [321]
            map.insert(56, &[577]);     // 56 => [577]
            map.insert(179, &[1089]);     // 179 => [1089]
            map.insert(421, &[2113]);     // 421 => [2113]
            map.insert(496, &[4161]);     // 496 => [4161]
            map.insert(346, &[8257]);     // 346 => [8257]
            map.insert(406, &[16449]);     // 406 => [16449]
            map.insert(385, &[385]);     // 385 => [385]
            map.insert(248, &[641]);     // 248 => [641]
            map.insert(115, &[1153]);     // 115 => [1153]
            map.insert(357, &[2177]);     // 357 => [2177]
            map.insert(304, &[4225]);     // 304 => [4225]
            map.insert(410, &[8321]);     // 410 => [8321]
            map.insert(342, &[16513]);     // 342 => [16513]
            map.insert(376, &[769]);     // 376 => [769]
            map.insert(499, &[1281]);     // 499 => [1281]
            map.insert(229, &[2305]);     // 229 => [2305]
            map.insert(176, &[4353]);     // 176 => [4353]
            map.insert(26, &[8449]);     // 26 => [8449]
            map.insert(214, &[16641]);     // 214 => [16641]
            map.insert(138, &[1537]);     // 138 => [1537]
            map.insert(412, &[2561]);     // 412 => [2561]
            map.insert(457, &[4609]);     // 457 => [4609]
            map.insert(355, &[8705]);     // 355 => [8705]
            map.insert(431, &[16897]);     // 431 => [16897]
            map.insert(279, &[3073]);     // 279 => [3073]
            map.insert(322, &[5121]);     // 322 => [5121]
            map.insert(488, &[9217]);     // 488 => [9217]
            map.insert(292, &[17409]);     // 292 => [17409]
            map.insert(84, &[6145]);     // 84 => [6145]
            map.insert(254, &[10241]);     // 254 => [10241]
            map.insert(50, &[18433]);     // 50 => [18433]
            map.insert(171, &[12289]);     // 171 => [12289]
            map.insert(103, &[20481]);     // 103 => [20481]
            map.insert(205, &[24577]);     // 205 => [24577]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(70, &[70]);     // 70 => [70]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(262, &[262]);     // 262 => [262]
            map.insert(127, &[518]);     // 127 => [518]
            map.insert(244, &[1030]);     // 244 => [1030]
            map.insert(482, &[2054]);     // 482 => [2054]
            map.insert(439, &[4102]);     // 439 => [4102]
            map.insert(285, &[8198]);     // 285 => [8198]
            map.insert(465, &[16390]);     // 465 => [16390]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(74, &[74]);     // 74 => [74]
            map.insert(494, &[2058]);     // 494 => [2058]
            map.insert(443, &[4106]);     // 443 => [4106]
            map.insert(477, &[16394]);     // 477 => [16394]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(146, &[146]);     // 146 => [146]
            map.insert(107, &[530]);     // 107 => [530]
            map.insert(224, &[1042]);     // 224 => [1042]
            map.insert(419, &[4114]);     // 419 => [4114]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(290, &[290]);     // 290 => [290]
            map.insert(91, &[546]);     // 91 => [546]
            map.insert(208, &[1058]);     // 208 => [1058]
            map.insert(403, &[4130]);     // 403 => [4130]
            map.insert(313, &[8226]);     // 313 => [8226]
            map.insert(194, &[194]);     // 194 => [194]
            map.insert(59, &[578]);     // 59 => [578]
            map.insert(422, &[2114]);     // 422 => [2114]
            map.insert(345, &[8258]);     // 345 => [8258]
            map.insert(405, &[16450]);     // 405 => [16450]
            map.insert(386, &[386]);     // 386 => [386]
            map.insert(358, &[2178]);     // 358 => [2178]
            map.insert(307, &[4226]);     // 307 => [4226]
            map.insert(409, &[8322]);     // 409 => [8322]
            map.insert(341, &[16514]);     // 341 => [16514]
            map.insert(379, &[770]);     // 379 => [770]
            map.insert(230, &[2306]);     // 230 => [2306]
            map.insert(213, &[16642]);     // 213 => [16642]
            map.insert(415, &[2562]);     // 415 => [2562]
            map.insert(458, &[4610]);     // 458 => [4610]
            map.insert(352, &[8706]);     // 352 => [8706]
            map.insert(428, &[16898]);     // 428 => [16898]
            map.insert(276, &[3074]);     // 276 => [3074]
            map.insert(491, &[9218]);     // 491 => [9218]
            map.insert(295, &[17410]);     // 295 => [17410]
            map.insert(87, &[6146]);     // 87 => [6146]
            map.insert(253, &[10242]);     // 253 => [10242]
            map.insert(168, &[12290]);     // 168 => [12290]
            map.insert(100, &[20482]);     // 100 => [20482]
            map.insert(206, &[24578]);     // 206 => [24578]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(44, &[44]);     // 44 => [44]
            map.insert(76, &[76]);     // 76 => [76]
            map.insert(140, &[140]);     // 140 => [140]
            map.insert(268, &[268]);     // 268 => [268]
            map.insert(117, &[524]);     // 117 => [524]
            map.insert(445, &[4108]);     // 445 => [4108]
            map.insert(475, &[16396]);     // 475 => [16396]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(148, &[148]);     // 148 => [148]
            map.insert(109, &[532]);     // 109 => [532]
            map.insert(271, &[8212]);     // 271 => [8212]
            map.insert(451, &[16404]);     // 451 => [16404]
            map.insert(164, &[164]);     // 164 => [164]
            map.insert(93, &[548]);     // 93 => [548]
            map.insert(448, &[2084]);     // 448 => [2084]
            map.insert(319, &[8228]);     // 319 => [8228]
            map.insert(196, &[196]);     // 196 => [196]
            map.insert(324, &[324]);     // 324 => [324]
            map.insert(61, &[580]);     // 61 => [580]
            map.insert(182, &[1092]);     // 182 => [1092]
            map.insert(351, &[8260]);     // 351 => [8260]
            map.insert(388, &[388]);     // 388 => [388]
            map.insert(118, &[1156]);     // 118 => [1156]
            map.insert(309, &[4228]);     // 309 => [4228]
            map.insert(339, &[16516]);     // 339 => [16516]
            map.insert(381, &[772]);     // 381 => [772]
            map.insert(181, &[4356]);     // 181 => [4356]
            map.insert(31, &[8452]);     // 31 => [8452]
            map.insert(143, &[1540]);     // 143 => [1540]
            map.insert(460, &[4612]);     // 460 => [4612]
            map.insert(426, &[16900]);     // 426 => [16900]
            map.insert(327, &[5124]);     // 327 => [5124]
            map.insert(55, &[18436]);     // 55 => [18436]
            map.insert(174, &[12292]);     // 174 => [12292]
            map.insert(200, &[24580]);     // 200 => [24580]
            map.insert(152, &[152]);     // 152 => [152]
            map.insert(234, &[1048]);     // 234 => [1048]
            map.insert(508, &[2072]);     // 508 => [2072]
            map.insert(425, &[4120]);     // 425 => [4120]
            map.insert(463, &[16408]);     // 463 => [16408]
            map.insert(296, &[296]);     // 296 => [296]
            map.insert(218, &[1064]);     // 218 => [1064]
            map.insert(511, &[16424]);     // 511 => [16424]
            map.insert(328, &[328]);     // 328 => [328]
            map.insert(186, &[1096]);     // 186 => [1096]
            map.insert(505, &[4168]);     // 505 => [4168]
            map.insert(392, &[392]);     // 392 => [392]
            map.insert(364, &[2184]);     // 364 => [2184]
            map.insert(369, &[776]);     // 369 => [776]
            map.insert(506, &[1288]);     // 506 => [1288]
            map.insert(236, &[2312]);     // 236 => [2312]
            map.insert(185, &[4360]);     // 185 => [4360]
            map.insert(223, &[16648]);     // 223 => [16648]
            map.insert(362, &[8712]);     // 362 => [8712]
            map.insert(331, &[5128]);     // 331 => [5128]
            map.insert(301, &[17416]);     // 301 => [17416]
            map.insert(110, &[20488]);     // 110 => [20488]
            map.insert(299, &[8240]);     // 299 => [8240]
            map.insert(336, &[336]);     // 336 => [336]
            map.insert(391, &[16464]);     // 391 => [16464]
            map.insert(233, &[656]);     // 233 => [656]
            map.insert(372, &[2192]);     // 372 => [2192]
            map.insert(395, &[8336]);     // 395 => [8336]
            map.insert(361, &[784]);     // 361 => [784]
            map.insert(199, &[16656]);     // 199 => [16656]
            map.insert(155, &[1552]);     // 155 => [1552]
            map.insert(397, &[2576]);     // 397 => [2576]
            map.insert(472, &[4624]);     // 472 => [4624]
            map.insert(370, &[8720]);     // 370 => [8720]
            map.insert(446, &[16912]);     // 446 => [16912]
            map.insert(239, &[10256]);     // 239 => [10256]
            map.insert(220, &[24592]);     // 220 => [24592]
            map.insert(217, &[672]);     // 217 => [672]
            map.insert(375, &[16544]);     // 375 => [16544]
            map.insert(398, &[16928]);     // 398 => [16928]
            map.insert(310, &[3104]);     // 310 => [3104]
            map.insert(151, &[16704]);     // 151 => [16704]
            map.insert(203, &[1600]);     // 203 => [1600]
            map.insert(191, &[10304]);     // 191 => [10304]
            map.insert(302, &[17024]);     // 302 => [17024]
            map.insert(157, &[2816]);     // 157 => [2816]
            map.insert(367, &[3584]);     // 367 => [3584]
            map.insert(348, &[17920]);     // 348 => [17920]
            map.insert(167, &[7168]);     // 167 => [7168]
            map.insert(62, &[25600]);     // 62 => [25600]
            map.insert(334, &[14336]);     // 334 => [14336]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(39, &[39]);     // 39 => [39]
            map.insert(71, &[71]);     // 71 => [71]
            map.insert(135, &[135]);     // 135 => [135]
            map.insert(263, &[263]);     // 263 => [263]
            map.insert(126, &[519]);     // 126 => [519]
            map.insert(245, &[1031]);     // 245 => [1031]
            map.insert(483, &[2055]);     // 483 => [2055]
            map.insert(438, &[4103]);     // 438 => [4103]
            map.insert(284, &[8199]);     // 284 => [8199]
            map.insert(464, &[16391]);     // 464 => [16391]
            map.insert(43, &[43]);     // 43 => [43]
            map.insert(75, &[75]);     // 75 => [75]
            map.insert(495, &[2059]);     // 495 => [2059]
            map.insert(442, &[4107]);     // 442 => [4107]
            map.insert(476, &[16395]);     // 476 => [16395]
            map.insert(83, &[83]);     // 83 => [83]
            map.insert(147, &[147]);     // 147 => [147]
            map.insert(106, &[531]);     // 106 => [531]
            map.insert(225, &[1043]);     // 225 => [1043]
            map.insert(418, &[4115]);     // 418 => [4115]
            map.insert(99, &[99]);     // 99 => [99]
            map.insert(163, &[163]);     // 163 => [163]
            map.insert(291, &[291]);     // 291 => [291]
            map.insert(90, &[547]);     // 90 => [547]
            map.insert(209, &[1059]);     // 209 => [1059]
            map.insert(402, &[4131]);     // 402 => [4131]
            map.insert(312, &[8227]);     // 312 => [8227]
            map.insert(195, &[195]);     // 195 => [195]
            map.insert(58, &[579]);     // 58 => [579]
            map.insert(423, &[2115]);     // 423 => [2115]
            map.insert(344, &[8259]);     // 344 => [8259]
            map.insert(404, &[16451]);     // 404 => [16451]
            map.insert(387, &[387]);     // 387 => [387]
            map.insert(359, &[2179]);     // 359 => [2179]
            map.insert(306, &[4227]);     // 306 => [4227]
            map.insert(408, &[8323]);     // 408 => [8323]
            map.insert(340, &[16515]);     // 340 => [16515]
            map.insert(378, &[771]);     // 378 => [771]
            map.insert(231, &[2307]);     // 231 => [2307]
            map.insert(212, &[16643]);     // 212 => [16643]
            map.insert(414, &[2563]);     // 414 => [2563]
            map.insert(459, &[4611]);     // 459 => [4611]
            map.insert(353, &[8707]);     // 353 => [8707]
            map.insert(429, &[16899]);     // 429 => [16899]
            map.insert(277, &[3075]);     // 277 => [3075]
            map.insert(490, &[9219]);     // 490 => [9219]
            map.insert(294, &[17411]);     // 294 => [17411]
            map.insert(86, &[6147]);     // 86 => [6147]
            map.insert(252, &[10243]);     // 252 => [10243]
            map.insert(169, &[12291]);     // 169 => [12291]
            map.insert(101, &[20483]);     // 101 => [20483]
            map.insert(207, &[24579]);     // 207 => [24579]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(45, &[45]);     // 45 => [45]
            map.insert(77, &[77]);     // 77 => [77]
            map.insert(141, &[141]);     // 141 => [141]
            map.insert(269, &[269]);     // 269 => [269]
            map.insert(116, &[525]);     // 116 => [525]
            map.insert(444, &[4109]);     // 444 => [4109]
            map.insert(474, &[16397]);     // 474 => [16397]
            map.insert(53, &[53]);     // 53 => [53]
            map.insert(149, &[149]);     // 149 => [149]
            map.insert(108, &[533]);     // 108 => [533]
            map.insert(270, &[8213]);     // 270 => [8213]
            map.insert(450, &[16405]);     // 450 => [16405]
            map.insert(165, &[165]);     // 165 => [165]
            map.insert(92, &[549]);     // 92 => [549]
            map.insert(449, &[2085]);     // 449 => [2085]
            map.insert(318, &[8229]);     // 318 => [8229]
            map.insert(197, &[197]);     // 197 => [197]
            map.insert(325, &[325]);     // 325 => [325]
            map.insert(60, &[581]);     // 60 => [581]
            map.insert(183, &[1093]);     // 183 => [1093]
            map.insert(350, &[8261]);     // 350 => [8261]
            map.insert(389, &[389]);     // 389 => [389]
            map.insert(119, &[1157]);     // 119 => [1157]
            map.insert(308, &[4229]);     // 308 => [4229]
            map.insert(338, &[16517]);     // 338 => [16517]
            map.insert(380, &[773]);     // 380 => [773]
            map.insert(180, &[4357]);     // 180 => [4357]
            map.insert(30, &[8453]);     // 30 => [8453]
            map.insert(142, &[1541]);     // 142 => [1541]
            map.insert(461, &[4613]);     // 461 => [4613]
            map.insert(427, &[16901]);     // 427 => [16901]
            map.insert(326, &[5125]);     // 326 => [5125]
            map.insert(54, &[18437]);     // 54 => [18437]
            map.insert(175, &[12293]);     // 175 => [12293]
            map.insert(201, &[24581]);     // 201 => [24581]
            map.insert(153, &[153]);     // 153 => [153]
            map.insert(235, &[1049]);     // 235 => [1049]
            map.insert(509, &[2073]);     // 509 => [2073]
            map.insert(424, &[4121]);     // 424 => [4121]
            map.insert(462, &[16409]);     // 462 => [16409]
            map.insert(297, &[297]);     // 297 => [297]
            map.insert(219, &[1065]);     // 219 => [1065]
            map.insert(510, &[16425]);     // 510 => [16425]
            map.insert(329, &[329]);     // 329 => [329]
            map.insert(187, &[1097]);     // 187 => [1097]
            map.insert(504, &[4169]);     // 504 => [4169]
            map.insert(393, &[393]);     // 393 => [393]
            map.insert(365, &[2185]);     // 365 => [2185]
            map.insert(368, &[777]);     // 368 => [777]
            map.insert(507, &[1289]);     // 507 => [1289]
            map.insert(237, &[2313]);     // 237 => [2313]
            map.insert(184, &[4361]);     // 184 => [4361]
            map.insert(222, &[16649]);     // 222 => [16649]
            map.insert(363, &[8713]);     // 363 => [8713]
            map.insert(330, &[5129]);     // 330 => [5129]
            map.insert(300, &[17417]);     // 300 => [17417]
            map.insert(111, &[20489]);     // 111 => [20489]
            map.insert(298, &[8241]);     // 298 => [8241]
            map.insert(337, &[337]);     // 337 => [337]
            map.insert(390, &[16465]);     // 390 => [16465]
            map.insert(232, &[657]);     // 232 => [657]
            map.insert(373, &[2193]);     // 373 => [2193]
            map.insert(394, &[8337]);     // 394 => [8337]
            map.insert(360, &[785]);     // 360 => [785]
            map.insert(198, &[16657]);     // 198 => [16657]
            map.insert(154, &[1553]);     // 154 => [1553]
            map.insert(396, &[2577]);     // 396 => [2577]
            map.insert(473, &[4625]);     // 473 => [4625]
            map.insert(371, &[8721]);     // 371 => [8721]
            map.insert(447, &[16913]);     // 447 => [16913]
            map.insert(238, &[10257]);     // 238 => [10257]
            map.insert(221, &[24593]);     // 221 => [24593]
            map.insert(216, &[673]);     // 216 => [673]
            map.insert(374, &[16545]);     // 374 => [16545]
            map.insert(399, &[16929]);     // 399 => [16929]
            map.insert(311, &[3105]);     // 311 => [3105]
            map.insert(150, &[16705]);     // 150 => [16705]
            map.insert(202, &[1601]);     // 202 => [1601]
            map.insert(190, &[10305]);     // 190 => [10305]
            map.insert(303, &[17025]);     // 303 => [17025]
            map.insert(156, &[2817]);     // 156 => [2817]
            map.insert(366, &[3585]);     // 366 => [3585]
            map.insert(349, &[17921]);     // 349 => [17921]
            map.insert(166, &[7169]);     // 166 => [7169]
            map.insert(63, &[25601]);     // 63 => [25601]
            map.insert(335, &[14337]);     // 335 => [14337]
            map.insert(46, &[46]);     // 46 => [46]
            map.insert(78, &[78]);     // 78 => [78]
            map.insert(95, &[550]);     // 95 => [550]
            map.insert(317, &[8230]);     // 317 => [8230]
            map.insert(383, &[774]);     // 383 => [774]
            map.insert(172, &[12294]);     // 172 => [12294]
            map.insert(189, &[10306]);     // 189 => [10306]
            map.insert(159, &[2818]);     // 159 => [2818]
            map.insert(332, &[14338]);     // 332 => [14338]
            map.insert(47, &[47]);     // 47 => [47]
            map.insert(79, &[79]);     // 79 => [79]
            map.insert(94, &[551]);     // 94 => [551]
            map.insert(316, &[8231]);     // 316 => [8231]
            map.insert(382, &[775]);     // 382 => [775]
            map.insert(173, &[12295]);     // 173 => [12295]
            map.insert(188, &[10307]);     // 188 => [10307]
            map.insert(158, &[2819]);     // 158 => [2819]
            map.insert(333, &[14339]);     // 333 => [14339]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode15_6 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode15_6 {
    fn name(&self) -> String {
        "[15, 6] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        15
    }

    fn dimension(&self) -> usize {
        6
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
        let mut error = BinVector::with_capacity(15);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 15 / 64 + if 15 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(15) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(6);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[15 / 64] & !((1 << 15) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode15_6.generator_matrix();
        assert_eq!(code.ncols(), 15);
        assert_eq!(code.nrows(), 6);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode15_6;
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
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_6;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, true, false, false, true, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
