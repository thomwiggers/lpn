use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 14]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode23_14;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 3833857 ],
                &[ 1933314 ],
                &[ 4816900 ],
                &[ 7815176 ],
                &[ 7438352 ],
                &[ 2441248 ],
                &[ 3539008 ],
                &[ 6209664 ],
                &[ 737536 ],
                &[ 7324160 ],
                &[ 5489664 ],
                &[ 5785600 ],
                &[ 6230016 ],
                &[ 6955008 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 122369 ],
                &[ 1742338 ],
                &[ 2405892 ],
                &[ 6025224 ],
                &[ 7973392 ],
                &[ 7109152 ],
                &[ 7330880 ],
                &[ 4641408 ],
                &[ 7411456 ],
                
            ], 23));
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
            map.insert(439, &[512]);     // 439 => [512]
            map.insert(327, &[1024]);     // 327 => [1024]
            map.insert(113, &[2048]);     // 113 => [2048]
            map.insert(495, &[4096]);     // 495 => [4096]
            map.insert(60, &[8192]);     // 60 => [8192]
            map.insert(233, &[16384]);     // 233 => [16384]
            map.insert(223, &[32768]);     // 223 => [32768]
            map.insert(345, &[65536]);     // 345 => [65536]
            map.insert(202, &[131072]);     // 202 => [131072]
            map.insert(228, &[262144]);     // 228 => [262144]
            map.insert(122, &[524288]);     // 122 => [524288]
            map.insert(282, &[1048576]);     // 282 => [1048576]
            map.insert(372, &[2097152]);     // 372 => [2097152]
            map.insert(504, &[4194304]);     // 504 => [4194304]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(65, &[65]);     // 65 => [65]
            map.insert(129, &[129]);     // 129 => [129]
            map.insert(257, &[257]);     // 257 => [257]
            map.insert(438, &[513]);     // 438 => [513]
            map.insert(326, &[1025]);     // 326 => [1025]
            map.insert(112, &[2049]);     // 112 => [2049]
            map.insert(494, &[4097]);     // 494 => [4097]
            map.insert(61, &[8193]);     // 61 => [8193]
            map.insert(232, &[16385]);     // 232 => [16385]
            map.insert(222, &[32769]);     // 222 => [32769]
            map.insert(344, &[65537]);     // 344 => [65537]
            map.insert(203, &[131073]);     // 203 => [131073]
            map.insert(229, &[262145]);     // 229 => [262145]
            map.insert(123, &[524289]);     // 123 => [524289]
            map.insert(283, &[1048577]);     // 283 => [1048577]
            map.insert(373, &[2097153]);     // 373 => [2097153]
            map.insert(505, &[4194305]);     // 505 => [4194305]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(66, &[66]);     // 66 => [66]
            map.insert(130, &[130]);     // 130 => [130]
            map.insert(258, &[258]);     // 258 => [258]
            map.insert(437, &[514]);     // 437 => [514]
            map.insert(325, &[1026]);     // 325 => [1026]
            map.insert(115, &[2050]);     // 115 => [2050]
            map.insert(493, &[4098]);     // 493 => [4098]
            map.insert(62, &[8194]);     // 62 => [8194]
            map.insert(235, &[16386]);     // 235 => [16386]
            map.insert(221, &[32770]);     // 221 => [32770]
            map.insert(347, &[65538]);     // 347 => [65538]
            map.insert(200, &[131074]);     // 200 => [131074]
            map.insert(230, &[262146]);     // 230 => [262146]
            map.insert(120, &[524290]);     // 120 => [524290]
            map.insert(280, &[1048578]);     // 280 => [1048578]
            map.insert(374, &[2097154]);     // 374 => [2097154]
            map.insert(506, &[4194306]);     // 506 => [4194306]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(68, &[68]);     // 68 => [68]
            map.insert(132, &[132]);     // 132 => [132]
            map.insert(260, &[260]);     // 260 => [260]
            map.insert(435, &[516]);     // 435 => [516]
            map.insert(323, &[1028]);     // 323 => [1028]
            map.insert(117, &[2052]);     // 117 => [2052]
            map.insert(491, &[4100]);     // 491 => [4100]
            map.insert(56, &[8196]);     // 56 => [8196]
            map.insert(237, &[16388]);     // 237 => [16388]
            map.insert(219, &[32772]);     // 219 => [32772]
            map.insert(349, &[65540]);     // 349 => [65540]
            map.insert(206, &[131076]);     // 206 => [131076]
            map.insert(224, &[262148]);     // 224 => [262148]
            map.insert(126, &[524292]);     // 126 => [524292]
            map.insert(286, &[1048580]);     // 286 => [1048580]
            map.insert(368, &[2097156]);     // 368 => [2097156]
            map.insert(508, &[4194308]);     // 508 => [4194308]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(72, &[72]);     // 72 => [72]
            map.insert(136, &[136]);     // 136 => [136]
            map.insert(264, &[264]);     // 264 => [264]
            map.insert(447, &[520]);     // 447 => [520]
            map.insert(335, &[1032]);     // 335 => [1032]
            map.insert(121, &[2056]);     // 121 => [2056]
            map.insert(487, &[4104]);     // 487 => [4104]
            map.insert(52, &[8200]);     // 52 => [8200]
            map.insert(225, &[16392]);     // 225 => [16392]
            map.insert(215, &[32776]);     // 215 => [32776]
            map.insert(337, &[65544]);     // 337 => [65544]
            map.insert(194, &[131080]);     // 194 => [131080]
            map.insert(236, &[262152]);     // 236 => [262152]
            map.insert(114, &[524296]);     // 114 => [524296]
            map.insert(274, &[1048584]);     // 274 => [1048584]
            map.insert(380, &[2097160]);     // 380 => [2097160]
            map.insert(496, &[4194312]);     // 496 => [4194312]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(80, &[80]);     // 80 => [80]
            map.insert(144, &[144]);     // 144 => [144]
            map.insert(272, &[272]);     // 272 => [272]
            map.insert(423, &[528]);     // 423 => [528]
            map.insert(343, &[1040]);     // 343 => [1040]
            map.insert(97, &[2064]);     // 97 => [2064]
            map.insert(511, &[4112]);     // 511 => [4112]
            map.insert(44, &[8208]);     // 44 => [8208]
            map.insert(249, &[16400]);     // 249 => [16400]
            map.insert(207, &[32784]);     // 207 => [32784]
            map.insert(329, &[65552]);     // 329 => [65552]
            map.insert(218, &[131088]);     // 218 => [131088]
            map.insert(244, &[262160]);     // 244 => [262160]
            map.insert(106, &[524304]);     // 106 => [524304]
            map.insert(266, &[1048592]);     // 266 => [1048592]
            map.insert(356, &[2097168]);     // 356 => [2097168]
            map.insert(488, &[4194320]);     // 488 => [4194320]
            map.insert(96, &[96]);     // 96 => [96]
            map.insert(160, &[160]);     // 160 => [160]
            map.insert(288, &[288]);     // 288 => [288]
            map.insert(407, &[544]);     // 407 => [544]
            map.insert(359, &[1056]);     // 359 => [1056]
            map.insert(81, &[2080]);     // 81 => [2080]
            map.insert(463, &[4128]);     // 463 => [4128]
            map.insert(28, &[8224]);     // 28 => [8224]
            map.insert(201, &[16416]);     // 201 => [16416]
            map.insert(255, &[32800]);     // 255 => [32800]
            map.insert(377, &[65568]);     // 377 => [65568]
            map.insert(234, &[131104]);     // 234 => [131104]
            map.insert(196, &[262176]);     // 196 => [262176]
            map.insert(90, &[524320]);     // 90 => [524320]
            map.insert(314, &[1048608]);     // 314 => [1048608]
            map.insert(340, &[2097184]);     // 340 => [2097184]
            map.insert(472, &[4194336]);     // 472 => [4194336]
            map.insert(192, &[192]);     // 192 => [192]
            map.insert(320, &[320]);     // 320 => [320]
            map.insert(503, &[576]);     // 503 => [576]
            map.insert(263, &[1088]);     // 263 => [1088]
            map.insert(49, &[2112]);     // 49 => [2112]
            map.insert(431, &[4160]);     // 431 => [4160]
            map.insert(124, &[8256]);     // 124 => [8256]
            map.insert(169, &[16448]);     // 169 => [16448]
            map.insert(159, &[32832]);     // 159 => [32832]
            map.insert(281, &[65600]);     // 281 => [65600]
            map.insert(138, &[131136]);     // 138 => [131136]
            map.insert(164, &[262208]);     // 164 => [262208]
            map.insert(58, &[524352]);     // 58 => [524352]
            map.insert(346, &[1048640]);     // 346 => [1048640]
            map.insert(308, &[2097216]);     // 308 => [2097216]
            map.insert(440, &[4194368]);     // 440 => [4194368]
            map.insert(384, &[384]);     // 384 => [384]
            map.insert(311, &[640]);     // 311 => [640]
            map.insert(455, &[1152]);     // 455 => [1152]
            map.insert(241, &[2176]);     // 241 => [2176]
            map.insert(367, &[4224]);     // 367 => [4224]
            map.insert(188, &[8320]);     // 188 => [8320]
            map.insert(105, &[16512]);     // 105 => [16512]
            map.insert(95, &[32896]);     // 95 => [32896]
            map.insert(473, &[65664]);     // 473 => [65664]
            map.insert(74, &[131200]);     // 74 => [131200]
            map.insert(100, &[262272]);     // 100 => [262272]
            map.insert(250, &[524416]);     // 250 => [524416]
            map.insert(410, &[1048704]);     // 410 => [1048704]
            map.insert(500, &[2097280]);     // 500 => [2097280]
            map.insert(376, &[4194432]);     // 376 => [4194432]
            map.insert(183, &[768]);     // 183 => [768]
            map.insert(71, &[1280]);     // 71 => [1280]
            map.insert(369, &[2304]);     // 369 => [2304]
            map.insert(239, &[4352]);     // 239 => [4352]
            map.insert(316, &[8448]);     // 316 => [8448]
            map.insert(489, &[16640]);     // 489 => [16640]
            map.insert(479, &[33024]);     // 479 => [33024]
            map.insert(89, &[65792]);     // 89 => [65792]
            map.insert(458, &[131328]);     // 458 => [131328]
            map.insert(484, &[262400]);     // 484 => [262400]
            map.insert(378, &[524544]);     // 378 => [524544]
            map.insert(26, &[1048832]);     // 26 => [1048832]
            map.insert(116, &[2097408]);     // 116 => [2097408]
            map.insert(248, &[4194560]);     // 248 => [4194560]
            map.insert(240, &[1536]);     // 240 => [1536]
            map.insert(454, &[2560]);     // 454 => [2560]
            map.insert(88, &[4608]);     // 88 => [4608]
            map.insert(395, &[8704]);     // 395 => [8704]
            map.insert(350, &[16896]);     // 350 => [16896]
            map.insert(360, &[33280]);     // 360 => [33280]
            map.insert(238, &[66048]);     // 238 => [66048]
            map.insert(381, &[131584]);     // 381 => [131584]
            map.insert(339, &[262656]);     // 339 => [262656]
            map.insert(461, &[524800]);     // 461 => [524800]
            map.insert(173, &[1049088]);     // 173 => [1049088]
            map.insert(195, &[2097664]);     // 195 => [2097664]
            map.insert(79, &[4194816]);     // 79 => [4194816]
            map.insert(310, &[3072]);     // 310 => [3072]
            map.insert(168, &[5120]);     // 168 => [5120]
            map.insert(379, &[9216]);     // 379 => [9216]
            map.insert(430, &[17408]);     // 430 => [17408]
            map.insert(408, &[33792]);     // 408 => [33792]
            map.insert(30, &[66560]);     // 30 => [66560]
            map.insert(397, &[132096]);     // 397 => [132096]
            map.insert(419, &[263168]);     // 419 => [263168]
            map.insert(317, &[525312]);     // 317 => [525312]
            map.insert(93, &[1049600]);     // 93 => [1049600]
            map.insert(51, &[2098176]);     // 51 => [2098176]
            map.insert(191, &[4195328]);     // 191 => [4195328]
            map.insert(414, &[6144]);     // 414 => [6144]
            map.insert(77, &[10240]);     // 77 => [10240]
            map.insert(152, &[18432]);     // 152 => [18432]
            map.insert(174, &[34816]);     // 174 => [34816]
            map.insert(296, &[67584]);     // 296 => [67584]
            map.insert(187, &[133120]);     // 187 => [133120]
            map.insert(149, &[264192]);     // 149 => [264192]
            map.insert(11, &[526336]);     // 11 => [526336]
            map.insert(363, &[1050624]);     // 363 => [1050624]
            map.insert(261, &[2099200]);     // 261 => [2099200]
            map.insert(393, &[4196352]);     // 393 => [4196352]
            map.insert(467, &[12288]);     // 467 => [12288]
            map.insert(262, &[20480]);     // 262 => [20480]
            map.insert(304, &[36864]);     // 304 => [36864]
            map.insert(182, &[69632]);     // 182 => [69632]
            map.insert(293, &[135168]);     // 293 => [135168]
            map.insert(267, &[266240]);     // 267 => [266240]
            map.insert(405, &[528384]);     // 405 => [528384]
            map.insert(245, &[1052672]);     // 245 => [1052672]
            map.insert(155, &[2101248]);     // 155 => [2101248]
            map.insert(23, &[4198400]);     // 23 => [4198400]
            map.insert(213, &[24576]);     // 213 => [24576]
            map.insert(227, &[40960]);     // 227 => [40960]
            map.insert(357, &[73728]);     // 357 => [73728]
            map.insert(246, &[139264]);     // 246 => [139264]
            map.insert(216, &[270336]);     // 216 => [270336]
            map.insert(70, &[532480]);     // 70 => [532480]
            map.insert(294, &[1056768]);     // 294 => [1056768]
            map.insert(328, &[2105344]);     // 328 => [2105344]
            map.insert(452, &[4202496]);     // 452 => [4202496]
            map.insert(54, &[49152]);     // 54 => [49152]
            map.insert(432, &[81920]);     // 432 => [81920]
            map.insert(35, &[147456]);     // 35 => [147456]
            map.insert(13, &[278528]);     // 13 => [278528]
            map.insert(147, &[540672]);     // 147 => [540672]
            map.insert(499, &[1064960]);     // 499 => [1064960]
            map.insert(413, &[2113536]);     // 413 => [2113536]
            map.insert(273, &[4210688]);     // 273 => [4210688]
            map.insert(390, &[98304]);     // 390 => [98304]
            map.insert(21, &[163840]);     // 21 => [163840]
            map.insert(59, &[294912]);     // 59 => [294912]
            map.insert(165, &[557056]);     // 165 => [557056]
            map.insert(453, &[1081344]);     // 453 => [1081344]
            map.insert(427, &[2129920]);     // 427 => [2129920]
            map.insert(295, &[4227072]);     // 295 => [4227072]
            map.insert(403, &[196608]);     // 403 => [196608]
            map.insert(445, &[327680]);     // 445 => [327680]
            map.insert(291, &[589824]);     // 291 => [589824]
            map.insert(67, &[1114112]);     // 67 => [1114112]
            map.insert(45, &[2162688]);     // 45 => [2162688]
            map.insert(161, &[4259840]);     // 161 => [4259840]
            map.insert(46, &[393216]);     // 46 => [393216]
            map.insert(176, &[655360]);     // 176 => [655360]
            map.insert(464, &[1179648]);     // 464 => [1179648]
            map.insert(446, &[2228224]);     // 446 => [2228224]
            map.insert(306, &[4325376]);     // 306 => [4325376]
            map.insert(158, &[786432]);     // 158 => [786432]
            map.insert(510, &[1310720]);     // 510 => [1310720]
            map.insert(400, &[2359296]);     // 400 => [2359296]
            map.insert(284, &[4456448]);     // 284 => [4456448]
            map.insert(352, &[1572864]);     // 352 => [1572864]
            map.insert(270, &[2621440]);     // 270 => [2621440]
            map.insert(386, &[4718592]);     // 386 => [4718592]
            map.insert(110, &[3145728]);     // 110 => [3145728]
            map.insert(226, &[5242880]);     // 226 => [5242880]
            map.insert(140, &[6291456]);     // 140 => [6291456]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(131, &[131]);     // 131 => [131]
            map.insert(259, &[259]);     // 259 => [259]
            map.insert(436, &[515]);     // 436 => [515]
            map.insert(324, &[1027]);     // 324 => [1027]
            map.insert(492, &[4099]);     // 492 => [4099]
            map.insert(63, &[8195]);     // 63 => [8195]
            map.insert(220, &[32771]);     // 220 => [32771]
            map.insert(231, &[262147]);     // 231 => [262147]
            map.insert(375, &[2097155]);     // 375 => [2097155]
            map.insert(507, &[4194307]);     // 507 => [4194307]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(69, &[69]);     // 69 => [69]
            map.insert(133, &[133]);     // 133 => [133]
            map.insert(434, &[517]);     // 434 => [517]
            map.insert(322, &[1029]);     // 322 => [1029]
            map.insert(490, &[4101]);     // 490 => [4101]
            map.insert(57, &[8197]);     // 57 => [8197]
            map.insert(348, &[65541]);     // 348 => [65541]
            map.insert(127, &[524293]);     // 127 => [524293]
            map.insert(287, &[1048581]);     // 287 => [1048581]
            map.insert(509, &[4194309]);     // 509 => [4194309]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(73, &[73]);     // 73 => [73]
            map.insert(137, &[137]);     // 137 => [137]
            map.insert(265, &[265]);     // 265 => [265]
            map.insert(334, &[1033]);     // 334 => [1033]
            map.insert(486, &[4105]);     // 486 => [4105]
            map.insert(53, &[8201]);     // 53 => [8201]
            map.insert(214, &[32777]);     // 214 => [32777]
            map.insert(336, &[65545]);     // 336 => [65545]
            map.insert(275, &[1048585]);     // 275 => [1048585]
            map.insert(497, &[4194313]);     // 497 => [4194313]
            map.insert(145, &[145]);     // 145 => [145]
            map.insert(422, &[529]);     // 422 => [529]
            map.insert(342, &[1041]);     // 342 => [1041]
            map.insert(107, &[524305]);     // 107 => [524305]
            map.insert(289, &[289]);     // 289 => [289]
            map.insert(406, &[545]);     // 406 => [545]
            map.insert(358, &[1057]);     // 358 => [1057]
            map.insert(462, &[4129]);     // 462 => [4129]
            map.insert(29, &[8225]);     // 29 => [8225]
            map.insert(254, &[32801]);     // 254 => [32801]
            map.insert(197, &[262177]);     // 197 => [262177]
            map.insert(91, &[524321]);     // 91 => [524321]
            map.insert(315, &[1048609]);     // 315 => [1048609]
            map.insert(341, &[2097185]);     // 341 => [2097185]
            map.insert(193, &[193]);     // 193 => [193]
            map.insert(321, &[321]);     // 321 => [321]
            map.insert(502, &[577]);     // 502 => [577]
            map.insert(125, &[8257]);     // 125 => [8257]
            map.insert(139, &[131137]);     // 139 => [131137]
            map.insert(309, &[2097217]);     // 309 => [2097217]
            map.insert(441, &[4194369]);     // 441 => [4194369]
            map.insert(385, &[385]);     // 385 => [385]
            map.insert(366, &[4225]);     // 366 => [4225]
            map.insert(189, &[8321]);     // 189 => [8321]
            map.insert(104, &[16513]);     // 104 => [16513]
            map.insert(94, &[32897]);     // 94 => [32897]
            map.insert(75, &[131201]);     // 75 => [131201]
            map.insert(101, &[262273]);     // 101 => [262273]
            map.insert(251, &[524417]);     // 251 => [524417]
            map.insert(411, &[1048705]);     // 411 => [1048705]
            map.insert(501, &[2097281]);     // 501 => [2097281]
            map.insert(478, &[33025]);     // 478 => [33025]
            map.insert(459, &[131329]);     // 459 => [131329]
            map.insert(485, &[262401]);     // 485 => [262401]
            map.insert(27, &[1048833]);     // 27 => [1048833]
            map.insert(394, &[8705]);     // 394 => [8705]
            map.insert(351, &[16897]);     // 351 => [16897]
            map.insert(361, &[33281]);     // 361 => [33281]
            map.insert(338, &[262657]);     // 338 => [262657]
            map.insert(460, &[524801]);     // 460 => [524801]
            map.insert(172, &[1049089]);     // 172 => [1049089]
            map.insert(78, &[4194817]);     // 78 => [4194817]
            map.insert(409, &[33793]);     // 409 => [33793]
            map.insert(31, &[66561]);     // 31 => [66561]
            map.insert(396, &[132097]);     // 396 => [132097]
            map.insert(418, &[263169]);     // 418 => [263169]
            map.insert(92, &[1049601]);     // 92 => [1049601]
            map.insert(50, &[2098177]);     // 50 => [2098177]
            map.insert(190, &[4195329]);     // 190 => [4195329]
            map.insert(415, &[6145]);     // 415 => [6145]
            map.insert(76, &[10241]);     // 76 => [10241]
            map.insert(153, &[18433]);     // 153 => [18433]
            map.insert(175, &[34817]);     // 175 => [34817]
            map.insert(297, &[67585]);     // 297 => [67585]
            map.insert(186, &[133121]);     // 186 => [133121]
            map.insert(148, &[264193]);     // 148 => [264193]
            map.insert(362, &[1050625]);     // 362 => [1050625]
            map.insert(392, &[4196353]);     // 392 => [4196353]
            map.insert(466, &[12289]);     // 466 => [12289]
            map.insert(305, &[36865]);     // 305 => [36865]
            map.insert(292, &[135169]);     // 292 => [135169]
            map.insert(404, &[528385]);     // 404 => [528385]
            map.insert(154, &[2101249]);     // 154 => [2101249]
            map.insert(22, &[4198401]);     // 22 => [4198401]
            map.insert(212, &[24577]);     // 212 => [24577]
            map.insert(247, &[139265]);     // 247 => [139265]
            map.insert(217, &[270337]);     // 217 => [270337]
            map.insert(55, &[49153]);     // 55 => [49153]
            map.insert(433, &[81921]);     // 433 => [81921]
            map.insert(146, &[540673]);     // 146 => [540673]
            map.insert(498, &[1064961]);     // 498 => [1064961]
            map.insert(412, &[2113537]);     // 412 => [2113537]
            map.insert(391, &[98305]);     // 391 => [98305]
            map.insert(426, &[2129921]);     // 426 => [2129921]
            map.insert(402, &[196609]);     // 402 => [196609]
            map.insert(444, &[327681]);     // 444 => [327681]
            map.insert(290, &[589825]);     // 290 => [589825]
            map.insert(47, &[393217]);     // 47 => [393217]
            map.insert(177, &[655361]);     // 177 => [655361]
            map.insert(465, &[1179649]);     // 465 => [1179649]
            map.insert(307, &[4325377]);     // 307 => [4325377]
            map.insert(401, &[2359297]);     // 401 => [2359297]
            map.insert(285, &[4456449]);     // 285 => [4456449]
            map.insert(353, &[1572865]);     // 353 => [1572865]
            map.insert(271, &[2621441]);     // 271 => [2621441]
            map.insert(387, &[4718593]);     // 387 => [4718593]
            map.insert(111, &[3145729]);     // 111 => [3145729]
            map.insert(141, &[6291457]);     // 141 => [6291457]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(134, &[134]);     // 134 => [134]
            map.insert(119, &[2054]);     // 119 => [2054]
            map.insert(204, &[131078]);     // 204 => [131078]
            map.insert(370, &[2097158]);     // 370 => [2097158]
            map.insert(42, &[42]);     // 42 => [42]
            map.insert(333, &[1034]);     // 333 => [1034]
            map.insert(382, &[2097162]);     // 382 => [2097162]
            map.insert(82, &[82]);     // 82 => [82]
            map.insert(421, &[530]);     // 421 => [530]
            map.insert(99, &[2066]);     // 99 => [2066]
            map.insert(205, &[32786]);     // 205 => [32786]
            map.insert(331, &[65554]);     // 331 => [65554]
            map.insert(98, &[98]);     // 98 => [98]
            map.insert(162, &[162]);     // 162 => [162]
            map.insert(83, &[2082]);     // 83 => [2082]
            map.insert(253, &[32802]);     // 253 => [32802]
            map.insert(198, &[262178]);     // 198 => [262178]
            map.insert(312, &[1048610]);     // 312 => [1048610]
            map.insert(474, &[4194338]);     // 474 => [4194338]
            map.insert(429, &[4162]);     // 429 => [4162]
            map.insert(171, &[16450]);     // 171 => [16450]
            map.insert(157, &[32834]);     // 157 => [32834]
            map.insert(166, &[262210]);     // 166 => [262210]
            map.insert(442, &[4194370]);     // 442 => [4194370]
            map.insert(243, &[2178]);     // 243 => [2178]
            map.insert(365, &[4226]);     // 365 => [4226]
            map.insert(475, &[65666]);     // 475 => [65666]
            map.insert(102, &[262274]);     // 102 => [262274]
            map.insert(181, &[770]);     // 181 => [770]
            map.insert(371, &[2306]);     // 371 => [2306]
            map.insert(318, &[8450]);     // 318 => [8450]
            map.insert(477, &[33026]);     // 477 => [33026]
            map.insert(456, &[131330]);     // 456 => [131330]
            map.insert(118, &[2097410]);     // 118 => [2097410]
            map.insert(242, &[1538]);     // 242 => [1538]
            map.insert(383, &[131586]);     // 383 => [131586]
            map.insert(170, &[5122]);     // 170 => [5122]
            map.insert(428, &[17410]);     // 428 => [17410]
            map.insert(399, &[132098]);     // 399 => [132098]
            map.insert(417, &[263170]);     // 417 => [263170]
            map.insert(319, &[525314]);     // 319 => [525314]
            map.insert(298, &[67586]);     // 298 => [67586]
            map.insert(185, &[133122]);     // 185 => [133122]
            map.insert(151, &[264194]);     // 151 => [264194]
            map.insert(180, &[69634]);     // 180 => [69634]
            map.insert(330, &[2105346]);     // 330 => [2105346]
            map.insert(15, &[278530]);     // 15 => [278530]
            map.insert(388, &[98306]);     // 388 => [98306]
            map.insert(167, &[557058]);     // 167 => [557058]
            map.insert(425, &[2129922]);     // 425 => [2129922]
            map.insert(163, &[4259842]);     // 163 => [4259842]
            map.insert(178, &[655362]);     // 178 => [655362]
            map.insert(156, &[786434]);     // 156 => [786434]
            map.insert(354, &[1572866]);     // 354 => [1572866]
            map.insert(268, &[2621442]);     // 268 => [2621442]
            map.insert(108, &[3145730]);     // 108 => [3145730]
            map.insert(142, &[6291458]);     // 142 => [6291458]
            map.insert(443, &[524]);     // 443 => [524]
            map.insert(483, &[4108]);     // 483 => [4108]
            map.insert(211, &[32780]);     // 211 => [32780]
            map.insert(278, &[1048588]);     // 278 => [1048588]
            map.insert(84, &[84]);     // 84 => [84]
            map.insert(276, &[276]);     // 276 => [276]
            map.insert(355, &[1060]);     // 355 => [1060]
            map.insert(85, &[2084]);     // 85 => [2084]
            map.insert(476, &[4194340]);     // 476 => [4194340]
            map.insert(451, &[1156]);     // 451 => [1156]
            map.insert(184, &[8324]);     // 184 => [8324]
            map.insert(109, &[16516]);     // 109 => [16516]
            map.insert(179, &[772]);     // 179 => [772]
            map.insert(480, &[262404]);     // 480 => [262404]
            map.insert(252, &[4194564]);     // 252 => [4194564]
            map.insert(450, &[2564]);     // 450 => [2564]
            map.insert(364, &[33284]);     // 364 => [33284]
            map.insert(457, &[524804]);     // 457 => [524804]
            map.insert(199, &[2097668]);     // 199 => [2097668]
            map.insert(313, &[525316]);     // 313 => [525316]
            map.insert(300, &[67588]);     // 300 => [67588]
            map.insert(471, &[12292]);     // 471 => [12292]
            map.insert(209, &[24580]);     // 209 => [24580]
            map.insert(332, &[2105348]);     // 332 => [2105348]
            map.insert(448, &[4202500]);     // 448 => [4202500]
            map.insert(39, &[147460]);     // 39 => [147460]
            map.insert(277, &[4210692]);     // 277 => [4210692]
            map.insert(449, &[1081348]);     // 449 => [1081348]
            map.insert(468, &[1179652]);     // 468 => [1179652]
            map.insert(210, &[131096]);     // 210 => [131096]
            map.insert(87, &[32904]);     // 87 => [32904]
            map.insert(481, &[16648]);     // 481 => [16648]
            map.insert(389, &[132104]);     // 389 => [132104]
            map.insert(269, &[2099208]);     // 269 => [2099208]
            map.insert(301, &[135176]);     // 301 => [135176]
            map.insert(208, &[270344]);     // 208 => [270344]
            map.insert(302, &[1056776]);     // 302 => [1056776]
            map.insert(43, &[147464]);     // 43 => [147464]
            map.insert(398, &[98312]);     // 398 => [98312]
            map.insert(303, &[4227080]);     // 303 => [4227080]
            map.insert(299, &[589832]);     // 299 => [589832]
            map.insert(150, &[786440]);     // 150 => [786440]
            map.insert(279, &[1104]);     // 279 => [1104]
            map.insert(143, &[32848]);     // 143 => [32848]
            map.insert(424, &[4194384]);     // 424 => [4194384]
            map.insert(470, &[2576]);     // 470 => [2576]
            map.insert(86, &[532496]);     // 86 => [532496]
            map.insert(416, &[81936]);     // 416 => [81936]
            map.insert(469, &[1081360]);     // 469 => [1081360]
            map.insert(103, &[1312]);     // 103 => [1312]
            map.insert(420, &[262464]);     // 420 => [262464]
            map.insert(482, &[5243136]);     // 482 => [5243136]
            map.insert(135, &[37376]);     // 135 => [37376]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode23_14 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode23_14 {
    fn name(&self) -> String {
        "[23, 14] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        14
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
        let mut error = BinVector::with_capacity(23);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 23 / 64 + if 23 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(23) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(14);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[23 / 64] & !((1 << 23) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode23_14.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 14);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode23_14;
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
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, true, true, true, false, true, true, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, true, false, true, false, true, true, false, false, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, true, false, false, true, false, true, true, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, false, false, true, true, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, false, true, true, false, false, true, false, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, true, true, false, true, true, false, false, false, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, false, true, false, false, true, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, false, true, true, true, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, false, false, false, false, false, false, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, true, false, true, true, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, true, false, false, true, false, false, true, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, false, false, true, false, false, true, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, true, false, true, true, true, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, true, true, true, true, false, false, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, true, false, true, true, false, false, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, false, true, true, false, false, false, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, true, false, true, false, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true, true, false, true, false, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, false, true, true, false, true, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, false, false, true, false, false, true, true, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, false, false, true, false, false, true, true, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, true, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
