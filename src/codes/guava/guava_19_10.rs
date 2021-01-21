use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[19, 10]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode19_10;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 464897 ],
                &[ 152578 ],
                &[ 221188 ],
                &[ 388104 ],
                &[ 46096 ],
                &[ 457760 ],
                &[ 343104 ],
                &[ 361600 ],
                &[ 389376 ],
                &[ 434688 ],
                
            ], 19));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 145473 ],
                &[ 342082 ],
                &[ 70724 ],
                &[ 377928 ],
                &[ 440336 ],
                &[ 521312 ],
                &[ 111744 ],
                &[ 417024 ],
                &[ 254464 ],
                
            ], 19));
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
            map.insert(47, &[64]);     // 47 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(256, &[512]);     // 256 => [512]
            map.insert(236, &[1024]);     // 236 => [1024]
            map.insert(147, &[2048]);     // 147 => [2048]
            map.insert(247, &[4096]);     // 247 => [4096]
            map.insert(371, &[8192]);     // 371 => [8192]
            map.insert(424, &[16384]);     // 424 => [16384]
            map.insert(376, &[32768]);     // 376 => [32768]
            map.insert(366, &[65536]);     // 366 => [65536]
            map.insert(433, &[131072]);     // 433 => [131072]
            map.insert(186, &[262144]);     // 186 => [262144]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(46, &[65]);     // 46 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(257, &[513]);     // 257 => [513]
            map.insert(237, &[1025]);     // 237 => [1025]
            map.insert(146, &[2049]);     // 146 => [2049]
            map.insert(246, &[4097]);     // 246 => [4097]
            map.insert(370, &[8193]);     // 370 => [8193]
            map.insert(425, &[16385]);     // 425 => [16385]
            map.insert(377, &[32769]);     // 377 => [32769]
            map.insert(367, &[65537]);     // 367 => [65537]
            map.insert(432, &[131073]);     // 432 => [131073]
            map.insert(187, &[262145]);     // 187 => [262145]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(45, &[66]);     // 45 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(258, &[514]);     // 258 => [514]
            map.insert(238, &[1026]);     // 238 => [1026]
            map.insert(145, &[2050]);     // 145 => [2050]
            map.insert(245, &[4098]);     // 245 => [4098]
            map.insert(369, &[8194]);     // 369 => [8194]
            map.insert(426, &[16386]);     // 426 => [16386]
            map.insert(378, &[32770]);     // 378 => [32770]
            map.insert(364, &[65538]);     // 364 => [65538]
            map.insert(435, &[131074]);     // 435 => [131074]
            map.insert(184, &[262146]);     // 184 => [262146]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(43, &[68]);     // 43 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(260, &[516]);     // 260 => [516]
            map.insert(232, &[1028]);     // 232 => [1028]
            map.insert(151, &[2052]);     // 151 => [2052]
            map.insert(243, &[4100]);     // 243 => [4100]
            map.insert(375, &[8196]);     // 375 => [8196]
            map.insert(428, &[16388]);     // 428 => [16388]
            map.insert(380, &[32772]);     // 380 => [32772]
            map.insert(362, &[65540]);     // 362 => [65540]
            map.insert(437, &[131076]);     // 437 => [131076]
            map.insert(190, &[262148]);     // 190 => [262148]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(39, &[72]);     // 39 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(264, &[520]);     // 264 => [520]
            map.insert(228, &[1032]);     // 228 => [1032]
            map.insert(155, &[2056]);     // 155 => [2056]
            map.insert(255, &[4104]);     // 255 => [4104]
            map.insert(379, &[8200]);     // 379 => [8200]
            map.insert(416, &[16392]);     // 416 => [16392]
            map.insert(368, &[32776]);     // 368 => [32776]
            map.insert(358, &[65544]);     // 358 => [65544]
            map.insert(441, &[131080]);     // 441 => [131080]
            map.insert(178, &[262152]);     // 178 => [262152]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(63, &[80]);     // 63 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(272, &[528]);     // 272 => [528]
            map.insert(252, &[1040]);     // 252 => [1040]
            map.insert(131, &[2064]);     // 131 => [2064]
            map.insert(231, &[4112]);     // 231 => [4112]
            map.insert(355, &[8208]);     // 355 => [8208]
            map.insert(440, &[16400]);     // 440 => [16400]
            map.insert(360, &[32784]);     // 360 => [32784]
            map.insert(382, &[65552]);     // 382 => [65552]
            map.insert(417, &[131088]);     // 417 => [131088]
            map.insert(170, &[262160]);     // 170 => [262160]
            map.insert(15, &[96]);     // 15 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(288, &[544]);     // 288 => [544]
            map.insert(204, &[1056]);     // 204 => [1056]
            map.insert(179, &[2080]);     // 179 => [2080]
            map.insert(215, &[4128]);     // 215 => [4128]
            map.insert(339, &[8224]);     // 339 => [8224]
            map.insert(392, &[16416]);     // 392 => [16416]
            map.insert(344, &[32800]);     // 344 => [32800]
            map.insert(334, &[65568]);     // 334 => [65568]
            map.insert(401, &[131104]);     // 401 => [131104]
            map.insert(154, &[262176]);     // 154 => [262176]
            map.insert(111, &[192]);     // 111 => [192]
            map.insert(175, &[320]);     // 175 => [320]
            map.insert(303, &[576]);     // 303 => [576]
            map.insert(195, &[1088]);     // 195 => [1088]
            map.insert(188, &[2112]);     // 188 => [2112]
            map.insert(216, &[4160]);     // 216 => [4160]
            map.insert(348, &[8256]);     // 348 => [8256]
            map.insert(391, &[16448]);     // 391 => [16448]
            map.insert(343, &[32832]);     // 343 => [32832]
            map.insert(321, &[65600]);     // 321 => [65600]
            map.insert(414, &[131136]);     // 414 => [131136]
            map.insert(149, &[262208]);     // 149 => [262208]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(320, &[640]);     // 320 => [640]
            map.insert(172, &[1152]);     // 172 => [1152]
            map.insert(211, &[2176]);     // 211 => [2176]
            map.insert(183, &[4224]);     // 183 => [4224]
            map.insert(307, &[8320]);     // 307 => [8320]
            map.insert(488, &[16512]);     // 488 => [16512]
            map.insert(312, &[32896]);     // 312 => [32896]
            map.insert(302, &[65664]);     // 302 => [65664]
            map.insert(497, &[131200]);     // 497 => [131200]
            map.insert(250, &[262272]);     // 250 => [262272]
            map.insert(384, &[768]);     // 384 => [768]
            map.insert(108, &[1280]);     // 108 => [1280]
            map.insert(19, &[2304]);     // 19 => [2304]
            map.insert(119, &[4352]);     // 119 => [4352]
            map.insert(499, &[8448]);     // 499 => [8448]
            map.insert(296, &[16640]);     // 296 => [16640]
            map.insert(504, &[33024]);     // 504 => [33024]
            map.insert(494, &[65792]);     // 494 => [65792]
            map.insert(305, &[131328]);     // 305 => [131328]
            map.insert(58, &[262400]);     // 58 => [262400]
            map.insert(492, &[1536]);     // 492 => [1536]
            map.insert(403, &[2560]);     // 403 => [2560]
            map.insert(503, &[4608]);     // 503 => [4608]
            map.insert(115, &[8704]);     // 115 => [8704]
            map.insert(168, &[16896]);     // 168 => [16896]
            map.insert(120, &[33280]);     // 120 => [33280]
            map.insert(110, &[66048]);     // 110 => [66048]
            map.insert(177, &[131584]);     // 177 => [131584]
            map.insert(442, &[262656]);     // 442 => [262656]
            map.insert(127, &[3072]);     // 127 => [3072]
            map.insert(27, &[5120]);     // 27 => [5120]
            map.insert(415, &[9216]);     // 415 => [9216]
            map.insert(324, &[17408]);     // 324 => [17408]
            map.insert(404, &[33792]);     // 404 => [33792]
            map.insert(386, &[66560]);     // 386 => [66560]
            map.insert(349, &[132096]);     // 349 => [132096]
            map.insert(86, &[263168]);     // 86 => [263168]
            map.insert(100, &[6144]);     // 100 => [6144]
            map.insert(480, &[10240]);     // 480 => [10240]
            map.insert(315, &[18432]);     // 315 => [18432]
            map.insert(491, &[34816]);     // 491 => [34816]
            map.insert(509, &[67584]);     // 509 => [67584]
            map.insert(290, &[133120]);     // 290 => [133120]
            map.insert(41, &[264192]);     // 41 => [264192]
            map.insert(388, &[12288]);     // 388 => [12288]
            map.insert(351, &[20480]);     // 351 => [20480]
            map.insert(399, &[36864]);     // 399 => [36864]
            map.insert(409, &[69632]);     // 409 => [69632]
            map.insert(326, &[135168]);     // 326 => [135168]
            map.insert(77, &[266240]);     // 77 => [266240]
            map.insert(219, &[24576]);     // 219 => [24576]
            map.insert(11, &[40960]);     // 11 => [40960]
            map.insert(29, &[73728]);     // 29 => [73728]
            map.insert(194, &[139264]);     // 194 => [139264]
            map.insert(457, &[270336]);     // 457 => [270336]
            map.insert(208, &[49152]);     // 208 => [49152]
            map.insert(198, &[81920]);     // 198 => [81920]
            map.insert(25, &[147456]);     // 25 => [147456]
            map.insert(274, &[278528]);     // 274 => [278528]
            map.insert(22, &[98304]);     // 22 => [98304]
            map.insert(201, &[163840]);     // 201 => [163840]
            map.insert(450, &[294912]);     // 450 => [294912]
            map.insert(223, &[196608]);     // 223 => [196608]
            map.insert(468, &[327680]);     // 468 => [327680]
            map.insert(267, &[393216]);     // 267 => [393216]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(44, &[67]);     // 44 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(259, &[515]);     // 259 => [515]
            map.insert(239, &[1027]);     // 239 => [1027]
            map.insert(244, &[4099]);     // 244 => [4099]
            map.insert(427, &[16387]);     // 427 => [16387]
            map.insert(365, &[65539]);     // 365 => [65539]
            map.insert(434, &[131075]);     // 434 => [131075]
            map.insert(185, &[262147]);     // 185 => [262147]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(42, &[69]);     // 42 => [69]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(261, &[517]);     // 261 => [517]
            map.insert(233, &[1029]);     // 233 => [1029]
            map.insert(150, &[2053]);     // 150 => [2053]
            map.insert(242, &[4101]);     // 242 => [4101]
            map.insert(374, &[8197]);     // 374 => [8197]
            map.insert(429, &[16389]);     // 429 => [16389]
            map.insert(381, &[32773]);     // 381 => [32773]
            map.insert(363, &[65541]);     // 363 => [65541]
            map.insert(436, &[131077]);     // 436 => [131077]
            map.insert(191, &[262149]);     // 191 => [262149]
            map.insert(38, &[73]);     // 38 => [73]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(265, &[521]);     // 265 => [521]
            map.insert(229, &[1033]);     // 229 => [1033]
            map.insert(254, &[4105]);     // 254 => [4105]
            map.insert(359, &[65545]);     // 359 => [65545]
            map.insert(49, &[49]);     // 49 => [49]
            map.insert(62, &[81]);     // 62 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(273, &[529]);     // 273 => [529]
            map.insert(253, &[1041]);     // 253 => [1041]
            map.insert(230, &[4113]);     // 230 => [4113]
            map.insert(354, &[8209]);     // 354 => [8209]
            map.insert(361, &[32785]);     // 361 => [32785]
            map.insert(383, &[65553]);     // 383 => [65553]
            map.insert(171, &[262161]);     // 171 => [262161]
            map.insert(14, &[97]);     // 14 => [97]
            map.insert(97, &[161]);     // 97 => [161]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(289, &[545]);     // 289 => [545]
            map.insert(205, &[1057]);     // 205 => [1057]
            map.insert(214, &[4129]);     // 214 => [4129]
            map.insert(338, &[8225]);     // 338 => [8225]
            map.insert(393, &[16417]);     // 393 => [16417]
            map.insert(345, &[32801]);     // 345 => [32801]
            map.insert(335, &[65569]);     // 335 => [65569]
            map.insert(400, &[131105]);     // 400 => [131105]
            map.insert(174, &[321]);     // 174 => [321]
            map.insert(189, &[2113]);     // 189 => [2113]
            map.insert(217, &[4161]);     // 217 => [4161]
            map.insert(390, &[16449]);     // 390 => [16449]
            map.insert(342, &[32833]);     // 342 => [32833]
            map.insert(148, &[262209]);     // 148 => [262209]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(173, &[1153]);     // 173 => [1153]
            map.insert(210, &[2177]);     // 210 => [2177]
            map.insert(182, &[4225]);     // 182 => [4225]
            map.insert(306, &[8321]);     // 306 => [8321]
            map.insert(489, &[16513]);     // 489 => [16513]
            map.insert(313, &[32897]);     // 313 => [32897]
            map.insert(496, &[131201]);     // 496 => [131201]
            map.insert(251, &[262273]);     // 251 => [262273]
            map.insert(385, &[769]);     // 385 => [769]
            map.insert(109, &[1281]);     // 109 => [1281]
            map.insert(118, &[4353]);     // 118 => [4353]
            map.insert(498, &[8449]);     // 498 => [8449]
            map.insert(297, &[16641]);     // 297 => [16641]
            map.insert(505, &[33025]);     // 505 => [33025]
            map.insert(495, &[65793]);     // 495 => [65793]
            map.insert(304, &[131329]);     // 304 => [131329]
            map.insert(59, &[262401]);     // 59 => [262401]
            map.insert(493, &[1537]);     // 493 => [1537]
            map.insert(402, &[2561]);     // 402 => [2561]
            map.insert(502, &[4609]);     // 502 => [4609]
            map.insert(114, &[8705]);     // 114 => [8705]
            map.insert(169, &[16897]);     // 169 => [16897]
            map.insert(121, &[33281]);     // 121 => [33281]
            map.insert(176, &[131585]);     // 176 => [131585]
            map.insert(443, &[262657]);     // 443 => [262657]
            map.insert(126, &[3073]);     // 126 => [3073]
            map.insert(26, &[5121]);     // 26 => [5121]
            map.insert(325, &[17409]);     // 325 => [17409]
            map.insert(405, &[33793]);     // 405 => [33793]
            map.insert(387, &[66561]);     // 387 => [66561]
            map.insert(87, &[263169]);     // 87 => [263169]
            map.insert(101, &[6145]);     // 101 => [6145]
            map.insert(481, &[10241]);     // 481 => [10241]
            map.insert(314, &[18433]);     // 314 => [18433]
            map.insert(490, &[34817]);     // 490 => [34817]
            map.insert(508, &[67585]);     // 508 => [67585]
            map.insert(291, &[133121]);     // 291 => [133121]
            map.insert(389, &[12289]);     // 389 => [12289]
            map.insert(350, &[20481]);     // 350 => [20481]
            map.insert(398, &[36865]);     // 398 => [36865]
            map.insert(408, &[69633]);     // 408 => [69633]
            map.insert(327, &[135169]);     // 327 => [135169]
            map.insert(76, &[266241]);     // 76 => [266241]
            map.insert(218, &[24577]);     // 218 => [24577]
            map.insert(28, &[73729]);     // 28 => [73729]
            map.insert(456, &[270337]);     // 456 => [270337]
            map.insert(209, &[49153]);     // 209 => [49153]
            map.insert(199, &[81921]);     // 199 => [81921]
            map.insert(275, &[278529]);     // 275 => [278529]
            map.insert(23, &[98305]);     // 23 => [98305]
            map.insert(200, &[163841]);     // 200 => [163841]
            map.insert(451, &[294913]);     // 451 => [294913]
            map.insert(222, &[196609]);     // 222 => [196609]
            map.insert(469, &[327681]);     // 469 => [327681]
            map.insert(266, &[393217]);     // 266 => [393217]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(262, &[518]);     // 262 => [518]
            map.insert(234, &[1030]);     // 234 => [1030]
            map.insert(241, &[4102]);     // 241 => [4102]
            map.insert(373, &[8198]);     // 373 => [8198]
            map.insert(430, &[16390]);     // 430 => [16390]
            map.insert(439, &[131078]);     // 439 => [131078]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(153, &[2058]);     // 153 => [2058]
            map.insert(418, &[16394]);     // 418 => [16394]
            map.insert(356, &[65546]);     // 356 => [65546]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(61, &[82]);     // 61 => [82]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(353, &[8210]);     // 353 => [8210]
            map.insert(419, &[131090]);     // 419 => [131090]
            map.insert(98, &[162]);     // 98 => [162]
            map.insert(162, &[290]);     // 162 => [290]
            map.insert(206, &[1058]);     // 206 => [1058]
            map.insert(213, &[4130]);     // 213 => [4130]
            map.insert(337, &[8226]);     // 337 => [8226]
            map.insert(394, &[16418]);     // 394 => [16418]
            map.insert(346, &[32802]);     // 346 => [32802]
            map.insert(332, &[65570]);     // 332 => [65570]
            map.insert(152, &[262178]);     // 152 => [262178]
            map.insert(301, &[578]);     // 301 => [578]
            map.insert(341, &[32834]);     // 341 => [32834]
            map.insert(323, &[65602]);     // 323 => [65602]
            map.insert(412, &[131138]);     // 412 => [131138]
            map.insert(322, &[642]);     // 322 => [642]
            map.insert(181, &[4226]);     // 181 => [4226]
            map.insert(300, &[65666]);     // 300 => [65666]
            map.insert(248, &[262274]);     // 248 => [262274]
            map.insert(117, &[4354]);     // 117 => [4354]
            map.insert(298, &[16642]);     // 298 => [16642]
            map.insert(506, &[33026]);     // 506 => [33026]
            map.insert(56, &[262402]);     // 56 => [262402]
            map.insert(501, &[4610]);     // 501 => [4610]
            map.insert(113, &[8706]);     // 113 => [8706]
            map.insert(122, &[33282]);     // 122 => [33282]
            map.insert(125, &[3074]);     // 125 => [3074]
            map.insert(413, &[9218]);     // 413 => [9218]
            map.insert(406, &[33794]);     // 406 => [33794]
            map.insert(84, &[263170]);     // 84 => [263170]
            map.insert(102, &[6146]);     // 102 => [6146]
            map.insert(482, &[10242]);     // 482 => [10242]
            map.insert(511, &[67586]);     // 511 => [67586]
            map.insert(397, &[36866]);     // 397 => [36866]
            map.insert(411, &[69634]);     // 411 => [69634]
            map.insert(79, &[266242]);     // 79 => [266242]
            map.insert(31, &[73730]);     // 31 => [73730]
            map.insert(459, &[270338]);     // 459 => [270338]
            map.insert(196, &[81922]);     // 196 => [81922]
            map.insert(203, &[163842]);     // 203 => [163842]
            map.insert(448, &[294914]);     // 448 => [294914]
            map.insert(221, &[196610]);     // 221 => [196610]
            map.insert(470, &[327682]);     // 470 => [327682]
            map.insert(140, &[268]);     // 140 => [268]
            map.insert(268, &[524]);     // 268 => [524]
            map.insert(224, &[1036]);     // 224 => [1036]
            map.insert(159, &[2060]);     // 159 => [2060]
            map.insert(420, &[16396]);     // 420 => [16396]
            map.insert(372, &[32780]);     // 372 => [32780]
            map.insert(445, &[131084]);     // 445 => [131084]
            map.insert(52, &[52]);     // 52 => [52]
            map.insert(276, &[532]);     // 276 => [532]
            map.insert(135, &[2068]);     // 135 => [2068]
            map.insert(227, &[4116]);     // 227 => [4116]
            map.insert(444, &[16404]);     // 444 => [16404]
            map.insert(421, &[131092]);     // 421 => [131092]
            map.insert(164, &[292]);     // 164 => [292]
            map.insert(292, &[548]);     // 292 => [548]
            map.insert(396, &[16420]);     // 396 => [16420]
            map.insert(330, &[65572]);     // 330 => [65572]
            map.insert(158, &[262180]);     // 158 => [262180]
            map.insert(107, &[196]);     // 107 => [196]
            map.insert(299, &[580]);     // 299 => [580]
            map.insert(220, &[4164]);     // 220 => [4164]
            map.insert(410, &[131140]);     // 410 => [131140]
            map.insert(311, &[8324]);     // 311 => [8324]
            map.insert(316, &[32900]);     // 316 => [32900]
            map.insert(104, &[1284]);     // 104 => [1284]
            map.insert(309, &[131332]);     // 309 => [131332]
            map.insert(407, &[2564]);     // 407 => [2564]
            map.insert(124, &[33284]);     // 124 => [33284]
            map.insert(106, &[66052]);     // 106 => [66052]
            map.insert(446, &[262660]);     // 446 => [262660]
            map.insert(123, &[3076]);     // 123 => [3076]
            map.insert(484, &[10244]);     // 484 => [10244]
            map.insert(319, &[18436]);     // 319 => [18436]
            map.insert(294, &[133124]);     // 294 => [133124]
            map.insert(347, &[20484]);     // 347 => [20484]
            map.insert(395, &[36868]);     // 395 => [36868]
            map.insert(461, &[270340]);     // 461 => [270340]
            map.insert(212, &[49156]);     // 212 => [49156]
            map.insert(278, &[278532]);     // 278 => [278532]
            map.insert(454, &[294916]);     // 454 => [294916]
            map.insert(464, &[327684]);     // 464 => [327684]
            map.insert(271, &[393220]);     // 271 => [393220]
            map.insert(55, &[88]);     // 55 => [88]
            map.insert(88, &[152]);     // 88 => [152]
            map.insert(280, &[536]);     // 280 => [536]
            map.insert(139, &[2072]);     // 139 => [2072]
            map.insert(352, &[32792]);     // 352 => [32792]
            map.insert(336, &[32808]);     // 336 => [32808]
            map.insert(103, &[200]);     // 103 => [200]
            map.insert(167, &[328]);     // 167 => [328]
            map.insert(295, &[584]);     // 295 => [584]
            map.insert(180, &[2120]);     // 180 => [2120]
            map.insert(340, &[8264]);     // 340 => [8264]
            map.insert(329, &[65608]);     // 329 => [65608]
            map.insert(157, &[262216]);     // 157 => [262216]
            map.insert(328, &[648]);     // 328 => [648]
            map.insert(507, &[8456]);     // 507 => [8456]
            map.insert(486, &[65800]);     // 486 => [65800]
            map.insert(112, &[33288]);     // 112 => [33288]
            map.insert(94, &[263176]);     // 94 => [263176]
            map.insert(483, &[34824]);     // 483 => [34824]
            map.insert(202, &[139272]);     // 202 => [139272]
            map.insert(449, &[270344]);     // 449 => [270344]
            map.insert(282, &[278536]);     // 282 => [278536]
            map.insert(30, &[98312]);     // 30 => [98312]
            map.insert(458, &[294920]);     // 458 => [294920]
            map.insert(476, &[327688]);     // 476 => [327688]
            map.insert(163, &[2096]);     // 163 => [2096]
            map.insert(318, &[65680]);     // 318 => [65680]
            map.insert(510, &[65808]);     // 510 => [65808]
            map.insert(487, &[4624]);     // 487 => [4624]
            map.insert(99, &[8720]);     // 99 => [8720]
            map.insert(333, &[132112]);     // 333 => [132112]
            map.insert(116, &[6160]);     // 116 => [6160]
            map.insert(57, &[264208]);     // 57 => [264208]
            map.insert(93, &[266256]);     // 93 => [266256]
            map.insert(473, &[270352]);     // 473 => [270352]
            map.insert(466, &[294928]);     // 466 => [294928]
            map.insert(207, &[196624]);     // 207 => [196624]
            map.insert(452, &[327696]);     // 452 => [327696]
            map.insert(283, &[393232]);     // 283 => [393232]
            map.insert(143, &[352]);     // 143 => [352]
            map.insert(156, &[2144]);     // 156 => [2144]
            map.insert(423, &[16480]);     // 423 => [16480]
            map.insert(270, &[65696]);     // 270 => [65696]
            map.insert(465, &[131232]);     // 465 => [131232]
            map.insert(51, &[2336]);     // 51 => [2336]
            map.insert(467, &[8480]);     // 467 => [8480]
            map.insert(472, &[33056]);     // 472 => [33056]
            map.insert(462, &[65824]);     // 462 => [65824]
            map.insert(460, &[1568]);     // 460 => [1568]
            map.insert(471, &[4640]);     // 471 => [4640]
            map.insert(83, &[8736]);     // 83 => [8736]
            map.insert(78, &[66080]);     // 78 => [66080]
            map.insert(95, &[3104]);     // 95 => [3104]
            map.insert(447, &[9248]);     // 447 => [9248]
            map.insert(477, &[67616]);     // 477 => [67616]
            map.insert(431, &[36896]);     // 431 => [36896]
            map.insert(226, &[139296]);     // 226 => [139296]
            map.insert(240, &[49184]);     // 240 => [49184]
            map.insert(54, &[98336]);     // 54 => [98336]
            map.insert(500, &[327712]);     // 500 => [327712]
            map.insert(284, &[8384]);     // 284 => [8384]
            map.insert(455, &[16576]);     // 455 => [16576]
            map.insert(279, &[32960]);     // 279 => [32960]
            map.insert(478, &[131264]);     // 478 => [131264]
            map.insert(60, &[2368]);     // 60 => [2368]
            map.insert(263, &[16704]);     // 263 => [16704]
            map.insert(286, &[131392]);     // 286 => [131392]
            map.insert(92, &[8768]);     // 92 => [8768]
            map.insert(75, &[6208]);     // 75 => [6208]
            map.insert(463, &[10304]);     // 463 => [10304]
            map.insert(269, &[133184]);     // 269 => [133184]
            map.insert(438, &[69696]);     // 438 => [69696]
            map.insert(317, &[278592]);     // 317 => [278592]
            map.insert(91, &[5248]);     // 91 => [5248]
            map.insert(479, &[9344]);     // 479 => [9344]
            map.insert(285, &[132224]);     // 285 => [132224]
            map.insert(105, &[264320]);     // 105 => [264320]
            map.insert(287, &[20608]);     // 287 => [20608]
            map.insert(89, &[147584]);     // 89 => [147584]
            map.insert(331, &[393344]);     // 331 => [393344]
            map.insert(281, &[69888]);     // 281 => [69888]
            map.insert(235, &[35328]);     // 235 => [35328]
            map.insert(475, &[25088]);     // 475 => [25088]
            map.insert(197, &[265216]);     // 197 => [265216]
            map.insert(293, &[271360]);     // 293 => [271360]
            map.insert(142, &[75776]);     // 142 => [75776]
            map.insert(85, &[83968]);     // 85 => [83968]
            map.insert(90, &[165888]);     // 90 => [165888]
            map.insert(53, &[143360]);     // 53 => [143360]
            map.insert(485, &[282624]);     // 485 => [282624]
            map.insert(225, &[102400]);     // 225 => [102400]
            map.insert(357, &[106496]);     // 357 => [106496]
            map.insert(71, &[135]);     // 71 => [135]
            map.insert(249, &[262275]);     // 249 => [262275]
            map.insert(141, &[269]);     // 141 => [269]
            map.insert(277, &[533]);     // 277 => [533]
            map.insert(165, &[293]);     // 165 => [293]
            map.insert(310, &[8325]);     // 310 => [8325]
            map.insert(308, &[131333]);     // 308 => [131333]
            map.insert(166, &[329]);     // 166 => [329]
            map.insert(453, &[327697]);     // 453 => [327697]
            map.insert(422, &[16481]);     // 422 => [16481]
            map.insert(474, &[25089]);     // 474 => [25089]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode19_10 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode19_10 {
    fn name(&self) -> String {
        "[19, 10] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        19
    }

    fn dimension(&self) -> usize {
        10
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
        let mut error = BinVector::with_capacity(19);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 19 / 64 + if 19 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(19) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(10);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[19 / 64] & !((1 << 19) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode19_10.generator_matrix();
        assert_eq!(code.ncols(), 19);
        assert_eq!(code.nrows(), 10);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode19_10;
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
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, false, true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, true, false, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, true, false, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, true, true, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, true, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, false, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, false, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, true, true, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, false, true, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, false, true, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, false, true, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, false, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, false, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, false, true, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, false, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, true, true, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_10;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, false, true, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
