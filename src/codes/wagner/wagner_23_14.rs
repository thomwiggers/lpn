use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 14]`` Wagner code
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct WagnerCode23_14;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 507905 ],
                &[ 1687554 ],
                &[ 5586948 ],
                &[ 3833864 ],
                &[ 2211856 ],
                &[ 704544 ],
                &[ 4505664 ],
                &[ 4800640 ],
                &[ 6455552 ],
                &[ 5390848 ],
                &[ 3474432 ],
                &[ 3098624 ],
                &[ 6213632 ],
                &[ 8232960 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7325185 ],
                &[ 3871746 ],
                &[ 2015812 ],
                &[ 3530824 ],
                &[ 5015120 ],
                &[ 95840 ],
                &[ 7504512 ],
                &[ 2913024 ],
                &[ 7682048 ],
                
            ], 23));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(512, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(7, &[131200]);     // 7 => [131200]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(14, &[1089]);     // 14 => [1089]
            map.insert(15, &[1088]);     // 15 => [1088]
            map.insert(16, &[16]);     // 16 => [16]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(19, &[1056]);     // 19 => [1056]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(22, &[524545]);     // 22 => [524545]
            map.insert(23, &[524544]);     // 23 => [524544]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[163840]);     // 26 => [163840]
            map.insert(27, &[163841]);     // 27 => [163841]
            map.insert(28, &[96]);     // 28 => [96]
            map.insert(29, &[32896]);     // 29 => [32896]
            map.insert(30, &[5242881]);     // 30 => [5242881]
            map.insert(31, &[5242880]);     // 31 => [5242880]
            map.insert(32, &[32]);     // 32 => [32]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(35, &[1040]);     // 35 => [1040]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(39, &[1044]);     // 39 => [1044]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(42, &[67585]);     // 42 => [67585]
            map.insert(43, &[67584]);     // 43 => [67584]
            map.insert(44, &[80]);     // 44 => [80]
            map.insert(45, &[16640]);     // 45 => [16640]
            map.insert(46, &[82]);     // 46 => [82]
            map.insert(47, &[16642]);     // 47 => [16642]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(49, &[1026]);     // 49 => [1026]
            map.insert(50, &[1025]);     // 50 => [1025]
            map.insert(51, &[1024]);     // 51 => [1024]
            map.insert(52, &[72]);     // 52 => [72]
            map.insert(53, &[270336]);     // 53 => [270336]
            map.insert(54, &[1029]);     // 54 => [1029]
            map.insert(55, &[1028]);     // 55 => [1028]
            map.insert(56, &[68]);     // 56 => [68]
            map.insert(57, &[69]);     // 57 => [69]
            map.insert(58, &[540672]);     // 58 => [540672]
            map.insert(59, &[1032]);     // 59 => [1032]
            map.insert(60, &[64]);     // 60 => [64]
            map.insert(61, &[65]);     // 61 => [65]
            map.insert(62, &[66]);     // 62 => [66]
            map.insert(63, &[266240]);     // 63 => [266240]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(67, &[131076]);     // 67 => [131076]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(69, &[131074]);     // 69 => [131074]
            map.insert(70, &[131073]);     // 70 => [131073]
            map.insert(71, &[131072]);     // 71 => [131072]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(75, &[131084]);     // 75 => [131084]
            map.insert(76, &[32785]);     // 76 => [32785]
            map.insert(77, &[32784]);     // 77 => [32784]
            map.insert(78, &[1050624]);     // 78 => [1050624]
            map.insert(79, &[131080]);     // 79 => [131080]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(81, &[4196352]);     // 81 => [4196352]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(83, &[4196354]);     // 83 => [4196354]
            map.insert(84, &[32777]);     // 84 => [32777]
            map.insert(85, &[32776]);     // 85 => [32776]
            map.insert(86, &[2359296]);     // 86 => [2359296]
            map.insert(87, &[131088]);     // 87 => [131088]
            map.insert(88, &[16896]);     // 88 => [16896]
            map.insert(89, &[32772]);     // 89 => [32772]
            map.insert(90, &[16898]);     // 90 => [16898]
            map.insert(91, &[32774]);     // 91 => [32774]
            map.insert(92, &[32769]);     // 92 => [32769]
            map.insert(93, &[32768]);     // 93 => [32768]
            map.insert(94, &[32771]);     // 94 => [32771]
            map.insert(95, &[32770]);     // 95 => [32770]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(97, &[32832]);     // 97 => [32832]
            map.insert(98, &[524800]);     // 98 => [524800]
            map.insert(99, &[2105344]);     // 99 => [2105344]
            map.insert(100, &[1114113]);     // 100 => [1114113]
            map.insert(101, &[1114112]);     // 101 => [1114112]
            map.insert(102, &[131105]);     // 102 => [131105]
            map.insert(103, &[131104]);     // 103 => [131104]
            map.insert(104, &[2101249]);     // 104 => [2101249]
            map.insert(105, &[2101248]);     // 105 => [2101248]
            map.insert(106, &[33796]);     // 106 => [33796]
            map.insert(107, &[2101250]);     // 107 => [2101250]
            map.insert(108, &[33794]);     // 108 => [33794]
            map.insert(109, &[2101252]);     // 109 => [2101252]
            map.insert(110, &[33792]);     // 110 => [33792]
            map.insert(111, &[33793]);     // 111 => [33793]
            map.insert(112, &[132100]);     // 112 => [132100]
            map.insert(113, &[1154]);     // 113 => [1154]
            map.insert(114, &[1153]);     // 114 => [1153]
            map.insert(115, &[1152]);     // 115 => [1152]
            map.insert(116, &[132096]);     // 116 => [132096]
            map.insert(117, &[768]);     // 117 => [768]
            map.insert(118, &[132098]);     // 118 => [132098]
            map.insert(119, &[770]);     // 119 => [770]
            map.insert(120, &[4259842]);     // 120 => [4259842]
            map.insert(121, &[131138]);     // 121 => [131138]
            map.insert(122, &[4259840]);     // 122 => [4259840]
            map.insert(123, &[131136]);     // 123 => [131136]
            map.insert(124, &[192]);     // 124 => [192]
            map.insert(125, &[32800]);     // 125 => [32800]
            map.insert(126, &[194]);     // 126 => [194]
            map.insert(127, &[32802]);     // 127 => [32802]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(131, &[73728]);     // 131 => [73728]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(133, &[3145728]);     // 133 => [3145728]
            map.insert(134, &[524305]);     // 134 => [524305]
            map.insert(135, &[524304]);     // 135 => [524304]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(137, &[69632]);     // 137 => [69632]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(139, &[69634]);     // 139 => [69634]
            map.insert(140, &[16417]);     // 140 => [16417]
            map.insert(141, &[16416]);     // 141 => [16416]
            map.insert(142, &[17424]);     // 142 => [17424]
            map.insert(143, &[16418]);     // 143 => [16418]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(145, &[16448]);     // 145 => [16448]
            map.insert(146, &[524293]);     // 146 => [524293]
            map.insert(147, &[524292]);     // 147 => [524292]
            map.insert(148, &[524291]);     // 148 => [524291]
            map.insert(149, &[524290]);     // 149 => [524290]
            map.insert(150, &[524289]);     // 150 => [524289]
            map.insert(151, &[524288]);     // 151 => [524288]
            map.insert(152, &[6291458]);     // 152 => [6291458]
            map.insert(153, &[264196]);     // 153 => [264196]
            map.insert(154, &[6291456]);     // 154 => [6291456]
            map.insert(155, &[6291457]);     // 155 => [6291457]
            map.insert(156, &[264193]);     // 156 => [264193]
            map.insert(157, &[264192]);     // 157 => [264192]
            map.insert(158, &[17408]);     // 158 => [17408]
            map.insert(159, &[524296]);     // 159 => [524296]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(162, &[6144]);     // 162 => [6144]
            map.insert(163, &[6145]);     // 163 => [6145]
            map.insert(164, &[525312]);     // 164 => [525312]
            map.insert(165, &[16392]);     // 165 => [16392]
            map.insert(166, &[525314]);     // 166 => [525314]
            map.insert(167, &[16394]);     // 167 => [16394]
            map.insert(168, &[33280]);     // 168 => [33280]
            map.insert(169, &[16388]);     // 169 => [16388]
            map.insert(170, &[524353]);     // 170 => [524353]
            map.insert(171, &[524352]);     // 171 => [524352]
            map.insert(172, &[16385]);     // 172 => [16385]
            map.insert(173, &[16384]);     // 173 => [16384]
            map.insert(174, &[16387]);     // 174 => [16387]
            map.insert(175, &[16386]);     // 175 => [16386]
            map.insert(176, &[131586]);     // 176 => [131586]
            map.insert(177, &[1282]);     // 177 => [1282]
            map.insert(178, &[131584]);     // 178 => [131584]
            map.insert(179, &[1280]);     // 179 => [1280]
            map.insert(180, &[641]);     // 180 => [641]
            map.insert(181, &[640]);     // 181 => [640]
            map.insert(182, &[327680]);     // 182 => [327680]
            map.insert(183, &[524320]);     // 183 => [524320]
            map.insert(184, &[324]);     // 184 => [324]
            map.insert(185, &[16404]);     // 185 => [16404]
            map.insert(186, &[131592]);     // 186 => [131592]
            map.insert(187, &[1288]);     // 187 => [1288]
            map.insert(188, &[320]);     // 188 => [320]
            map.insert(189, &[16400]);     // 189 => [16400]
            map.insert(190, &[322]);     // 190 => [322]
            map.insert(191, &[16402]);     // 191 => [16402]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(195, &[131332]);     // 195 => [131332]
            map.insert(196, &[1538]);     // 196 => [1538]
            map.insert(197, &[131330]);     // 197 => [131330]
            map.insert(198, &[1536]);     // 198 => [1536]
            map.insert(199, &[131328]);     // 199 => [131328]
            map.insert(200, &[577]);     // 200 => [577]
            map.insert(201, &[576]);     // 201 => [576]
            map.insert(202, &[557056]);     // 202 => [557056]
            map.insert(203, &[2099200]);     // 203 => [2099200]
            map.insert(204, &[4456448]);     // 204 => [4456448]
            map.insert(205, &[4456449]);     // 205 => [4456449]
            map.insert(206, &[4456450]);     // 206 => [4456450]
            map.insert(207, &[2099204]);     // 207 => [2099204]
            map.insert(208, &[655360]);     // 208 => [655360]
            map.insert(209, &[655361]);     // 209 => [655361]
            map.insert(210, &[1310721]);     // 210 => [1310721]
            map.insert(211, &[1310720]);     // 211 => [1310720]
            map.insert(212, &[545]);     // 212 => [545]
            map.insert(213, &[544]);     // 213 => [544]
            map.insert(214, &[524417]);     // 214 => [524417]
            map.insert(215, &[524416]);     // 215 => [524416]
            map.insert(216, &[655368]);     // 216 => [655368]
            map.insert(217, &[33028]);     // 217 => [33028]
            map.insert(218, &[557072]);     // 218 => [557072]
            map.insert(219, &[1310728]);     // 219 => [1310728]
            map.insert(220, &[33025]);     // 220 => [33025]
            map.insert(221, &[33024]);     // 221 => [33024]
            map.insert(222, &[17536]);     // 222 => [17536]
            map.insert(223, &[33026]);     // 223 => [33026]
            map.insert(224, &[2162688]);     // 224 => [2162688]
            map.insert(225, &[2162689]);     // 225 => [2162689]
            map.insert(226, &[2162690]);     // 226 => [2162690]
            map.insert(227, &[4198416]);     // 227 => [4198416]
            map.insert(228, &[529]);     // 228 => [529]
            map.insert(229, &[528]);     // 229 => [528]
            map.insert(230, &[1056768]);     // 230 => [1056768]
            map.insert(231, &[1056769]);     // 231 => [1056769]
            map.insert(232, &[147458]);     // 232 => [147458]
            map.insert(233, &[16516]);     // 233 => [16516]
            map.insert(234, &[147456]);     // 234 => [147456]
            map.insert(235, &[147457]);     // 235 => [147457]
            map.insert(236, &[1052672]);     // 236 => [1052672]
            map.insert(237, &[16512]);     // 237 => [16512]
            map.insert(238, &[1052674]);     // 238 => [1052674]
            map.insert(239, &[16514]);     // 239 => [16514]
            map.insert(240, &[49152]);     // 240 => [49152]
            map.insert(241, &[516]);     // 241 => [516]
            map.insert(242, &[4198401]);     // 242 => [4198401]
            map.insert(243, &[4198400]);     // 243 => [4198400]
            map.insert(244, &[513]);     // 244 => [513]
            map.insert(245, &[512]);     // 245 => [512]
            map.insert(246, &[515]);     // 246 => [515]
            map.insert(247, &[514]);     // 247 => [514]
            map.insert(248, &[4202497]);     // 248 => [4202497]
            map.insert(249, &[4202496]);     // 249 => [4202496]
            map.insert(250, &[147472]);     // 250 => [147472]
            map.insert(251, &[4202498]);     // 251 => [4202498]
            map.insert(252, &[521]);     // 252 => [521]
            map.insert(253, &[520]);     // 253 => [520]
            map.insert(254, &[2625536]);     // 254 => [2625536]
            map.insert(255, &[522]);     // 255 => [522]
            map.insert(256, &[2048]);     // 256 => [2048]
            map.insert(257, &[2049]);     // 257 => [2049]
            map.insert(258, &[2050]);     // 258 => [2050]
            map.insert(259, &[2051]);     // 259 => [2051]
            map.insert(260, &[2052]);     // 260 => [2052]
            map.insert(261, &[24576]);     // 261 => [24576]
            map.insert(262, &[2054]);     // 262 => [2054]
            map.insert(263, &[24578]);     // 263 => [24578]
            map.insert(264, &[2056]);     // 264 => [2056]
            map.insert(265, &[1179648]);     // 265 => [1179648]
            map.insert(266, &[786432]);     // 266 => [786432]
            map.insert(267, &[65568]);     // 267 => [65568]
            map.insert(268, &[4227072]);     // 268 => [4227072]
            map.insert(269, &[4227073]);     // 269 => [4227073]
            map.insert(270, &[1048704]);     // 270 => [1048704]
            map.insert(271, &[20480]);     // 271 => [20480]
            map.insert(272, &[2064]);     // 272 => [2064]
            map.insert(273, &[4194432]);     // 273 => [4194432]
            map.insert(274, &[1081345]);     // 274 => [1081345]
            map.insert(275, &[1081344]);     // 275 => [1081344]
            map.insert(276, &[4325378]);     // 276 => [4325378]
            map.insert(277, &[65602]);     // 277 => [65602]
            map.insert(278, &[4325376]);     // 278 => [4325376]
            map.insert(279, &[65600]);     // 279 => [65600]
            map.insert(280, &[66560]);     // 280 => [66560]
            map.insert(281, &[66561]);     // 281 => [66561]
            map.insert(282, &[66562]);     // 282 => [66562]
            map.insert(283, &[1081352]);     // 283 => [1081352]
            map.insert(284, &[262401]);     // 284 => [262401]
            map.insert(285, &[262400]);     // 285 => [262400]
            map.insert(286, &[4325384]);     // 286 => [4325384]
            map.insert(287, &[262402]);     // 287 => [262402]
            map.insert(288, &[2080]);     // 288 => [2080]
            map.insert(289, &[2081]);     // 289 => [2081]
            map.insert(290, &[4352]);     // 290 => [4352]
            map.insert(291, &[65544]);     // 291 => [65544]
            map.insert(292, &[2084]);     // 292 => [2084]
            map.insert(293, &[528400]);     // 293 => [528400]
            map.insert(294, &[4356]);     // 294 => [4356]
            map.insert(295, &[65548]);     // 295 => [65548]
            map.insert(296, &[8448]);     // 296 => [8448]
            map.insert(297, &[65538]);     // 297 => [65538]
            map.insert(298, &[65537]);     // 298 => [65537]
            map.insert(299, &[65536]);     // 299 => [65536]
            map.insert(300, &[8452]);     // 300 => [8452]
            map.insert(301, &[65542]);     // 301 => [65542]
            map.insert(302, &[65541]);     // 302 => [65541]
            map.insert(303, &[65540]);     // 303 => [65540]
            map.insert(304, &[278528]);     // 304 => [278528]
            map.insert(305, &[278529]);     // 305 => [278529]
            map.insert(306, &[3073]);     // 306 => [3073]
            map.insert(307, &[3072]);     // 307 => [3072]
            map.insert(308, &[528385]);     // 308 => [528385]
            map.insert(309, &[528384]);     // 309 => [528384]
            map.insert(310, &[2097672]);     // 310 => [2097672]
            map.insert(311, &[528386]);     // 311 => [528386]
            map.insert(312, &[2116]);     // 312 => [2116]
            map.insert(313, &[65554]);     // 313 => [65554]
            map.insert(314, &[65553]);     // 314 => [65553]
            map.insert(315, &[65552]);     // 315 => [65552]
            map.insert(316, &[2112]);     // 316 => [2112]
            map.insert(317, &[2113]);     // 317 => [2113]
            map.insert(318, &[2097664]);     // 318 => [2097664]
            map.insert(319, &[532480]);     // 319 => [532480]
            map.insert(320, &[2176]);     // 320 => [2176]
            map.insert(321, &[4194320]);     // 321 => [4194320]
            map.insert(322, &[2178]);     // 322 => [2178]
            map.insert(323, &[4194322]);     // 323 => [4194322]
            map.insert(324, &[1048586]);     // 324 => [1048586]
            map.insert(325, &[133122]);     // 325 => [133122]
            map.insert(326, &[1048584]);     // 326 => [1048584]
            map.insert(327, &[133120]);     // 327 => [133120]
            map.insert(328, &[1048582]);     // 328 => [1048582]
            map.insert(329, &[2097410]);     // 329 => [2097410]
            map.insert(330, &[1048580]);     // 330 => [1048580]
            map.insert(331, &[2097408]);     // 331 => [2097408]
            map.insert(332, &[1048578]);     // 332 => [1048578]
            map.insert(333, &[1048579]);     // 333 => [1048579]
            map.insert(334, &[1048576]);     // 334 => [1048576]
            map.insert(335, &[1048577]);     // 335 => [1048577]
            map.insert(336, &[4194305]);     // 336 => [4194305]
            map.insert(337, &[4194304]);     // 337 => [4194304]
            map.insert(338, &[4194307]);     // 338 => [4194307]
            map.insert(339, &[4194306]);     // 339 => [4194306]
            map.insert(340, &[4194309]);     // 340 => [4194309]
            map.insert(341, &[4194308]);     // 341 => [4194308]
            map.insert(342, &[4609]);     // 342 => [4609]
            map.insert(343, &[4608]);     // 343 => [4608]
            map.insert(344, &[4194313]);     // 344 => [4194313]
            map.insert(345, &[4194312]);     // 345 => [4194312]
            map.insert(346, &[1048596]);     // 346 => [1048596]
            map.insert(347, &[4194314]);     // 347 => [4194314]
            map.insert(348, &[2621440]);     // 348 => [2621440]
            map.insert(349, &[8704]);     // 349 => [8704]
            map.insert(350, &[1048592]);     // 350 => [1048592]
            map.insert(351, &[1048593]);     // 351 => [1048593]
            map.insert(352, &[4195330]);     // 352 => [4195330]
            map.insert(353, &[4194352]);     // 353 => [4194352]
            map.insert(354, &[4195328]);     // 354 => [4195328]
            map.insert(355, &[4195329]);     // 355 => [4195329]
            map.insert(356, &[2113538]);     // 356 => [2113538]
            map.insert(357, &[4194376]);     // 357 => [4194376]
            map.insert(358, &[2113536]);     // 358 => [2113536]
            map.insert(359, &[2113537]);     // 359 => [2113537]
            map.insert(360, &[262656]);     // 360 => [262656]
            map.insert(361, &[262657]);     // 361 => [262657]
            map.insert(362, &[65665]);     // 362 => [65665]
            map.insert(363, &[65664]);     // 363 => [65664]
            map.insert(364, &[196608]);     // 364 => [196608]
            map.insert(365, &[4194368]);     // 365 => [4194368]
            map.insert(366, &[1048608]);     // 366 => [1048608]
            map.insert(367, &[1048609]);     // 367 => [1048609]
            map.insert(368, &[4194337]);     // 368 => [4194337]
            map.insert(369, &[4194336]);     // 369 => [4194336]
            map.insert(370, &[1048640]);     // 370 => [1048640]
            map.insert(371, &[1048641]);     // 371 => [1048641]
            map.insert(372, &[98306]);     // 372 => [98306]
            map.insert(373, &[4194340]);     // 373 => [4194340]
            map.insert(374, &[98304]);     // 374 => [98304]
            map.insert(375, &[98305]);     // 375 => [98305]
            map.insert(376, &[262672]);     // 376 => [262672]
            map.insert(377, &[1049604]);     // 377 => [1049604]
            map.insert(378, &[1048648]);     // 378 => [1048648]
            map.insert(379, &[65680]);     // 379 => [65680]
            map.insert(380, &[1049601]);     // 380 => [1049601]
            map.insert(381, &[1049600]);     // 381 => [1049600]
            map.insert(382, &[98312]);     // 382 => [98312]
            map.insert(383, &[1049602]);     // 383 => [1049602]
            map.insert(384, &[2304]);     // 384 => [2304]
            map.insert(385, &[2305]);     // 385 => [2305]
            map.insert(386, &[4128]);     // 386 => [4128]
            map.insert(387, &[4129]);     // 387 => [4129]
            map.insert(388, &[81922]);     // 388 => [81922]
            map.insert(389, &[262168]);     // 389 => [262168]
            map.insert(390, &[81920]);     // 390 => [81920]
            map.insert(391, &[81921]);     // 391 => [81921]
            map.insert(392, &[8224]);     // 392 => [8224]
            map.insert(393, &[8225]);     // 393 => [8225]
            map.insert(394, &[2097281]);     // 394 => [2097281]
            map.insert(395, &[2097280]);     // 395 => [2097280]
            map.insert(396, &[2228224]);     // 396 => [2228224]
            map.insert(397, &[262160]);     // 397 => [262160]
            map.insert(398, &[2228226]);     // 398 => [2228226]
            map.insert(399, &[262162]);     // 399 => [262162]
            map.insert(400, &[5121]);     // 400 => [5121]
            map.insert(401, &[5120]);     // 401 => [5120]
            map.insert(402, &[2129924]);     // 402 => [2129924]
            map.insert(403, &[5122]);     // 403 => [5122]
            map.insert(404, &[8256]);     // 404 => [8256]
            map.insert(405, &[262152]);     // 405 => [262152]
            map.insert(406, &[2129920]);     // 406 => [2129920]
            map.insert(407, &[526336]);     // 407 => [526336]
            map.insert(408, &[262149]);     // 408 => [262149]
            map.insert(409, &[262148]);     // 409 => [262148]
            map.insert(410, &[9217]);     // 410 => [9217]
            map.insert(411, &[9216]);     // 411 => [9216]
            map.insert(412, &[262145]);     // 412 => [262145]
            map.insert(413, &[262144]);     // 413 => [262144]
            map.insert(414, &[4160]);     // 414 => [4160]
            map.insert(415, &[262146]);     // 415 => [262146]
            map.insert(416, &[4098]);     // 416 => [4098]
            map.insert(417, &[262208]);     // 417 => [262208]
            map.insert(418, &[4096]);     // 418 => [4096]
            map.insert(419, &[4097]);     // 419 => [4097]
            map.insert(420, &[4194816]);     // 420 => [4194816]
            map.insert(421, &[4194817]);     // 421 => [4194817]
            map.insert(422, &[4100]);     // 422 => [4100]
            map.insert(423, &[4101]);     // 423 => [4101]
            map.insert(424, &[8192]);     // 424 => [8192]
            map.insert(425, &[8193]);     // 425 => [8193]
            map.insert(426, &[8194]);     // 426 => [8194]
            map.insert(427, &[65792]);     // 427 => [65792]
            map.insert(428, &[8196]);     // 428 => [8196]
            map.insert(429, &[18432]);     // 429 => [18432]
            map.insert(430, &[263168]);     // 430 => [263168]
            map.insert(431, &[263169]);     // 431 => [263169]
            map.insert(432, &[4114]);     // 432 => [4114]
            map.insert(433, &[262224]);     // 433 => [262224]
            map.insert(434, &[4112]);     // 434 => [4112]
            map.insert(435, &[4113]);     // 435 => [4113]
            map.insert(436, &[589832]);     // 436 => [589832]
            map.insert(437, &[262184]);     // 437 => [262184]
            map.insert(438, &[4116]);     // 438 => [4116]
            map.insert(439, &[526368]);     // 439 => [526368]
            map.insert(440, &[8208]);     // 440 => [8208]
            map.insert(441, &[8209]);     // 441 => [8209]
            map.insert(442, &[1049089]);     // 442 => [1049089]
            map.insert(443, &[1049088]);     // 443 => [1049088]
            map.insert(444, &[589824]);     // 444 => [589824]
            map.insert(445, &[262176]);     // 445 => [262176]
            map.insert(446, &[589826]);     // 446 => [589826]
            map.insert(447, &[262178]);     // 447 => [262178]
            map.insert(448, &[294912]);     // 448 => [294912]
            map.insert(449, &[294913]);     // 449 => [294913]
            map.insert(450, &[2097161]);     // 450 => [2097161]
            map.insert(451, &[2097160]);     // 451 => [2097160]
            map.insert(452, &[4718594]);     // 452 => [4718594]
            map.insert(453, &[135200]);     // 453 => [135200]
            map.insert(454, &[4718592]);     // 454 => [4718592]
            map.insert(455, &[4718593]);     // 455 => [4718593]
            map.insert(456, &[2097155]);     // 456 => [2097155]
            map.insert(457, &[2097154]);     // 457 => [2097154]
            map.insert(458, &[2097153]);     // 458 => [2097153]
            map.insert(459, &[2097152]);     // 459 => [2097152]
            map.insert(460, &[1048834]);     // 460 => [1048834]
            map.insert(461, &[2097158]);     // 461 => [2097158]
            map.insert(462, &[1048832]);     // 462 => [1048832]
            map.insert(463, &[2097156]);     // 463 => [2097156]
            map.insert(464, &[4194561]);     // 464 => [4194561]
            map.insert(465, &[4194560]);     // 465 => [4194560]
            map.insert(466, &[393224]);     // 466 => [393224]
            map.insert(467, &[4194562]);     // 467 => [4194562]
            map.insert(468, &[8384]);     // 468 => [8384]
            map.insert(469, &[4194564]);     // 469 => [4194564]
            map.insert(470, &[66056]);     // 470 => [66056]
            map.insert(471, &[2097248]);     // 471 => [2097248]
            map.insert(472, &[1572865]);     // 472 => [1572865]
            map.insert(473, &[1572864]);     // 473 => [1572864]
            map.insert(474, &[393216]);     // 474 => [393216]
            map.insert(475, &[2097168]);     // 475 => [2097168]
            map.insert(476, &[262273]);     // 476 => [262273]
            map.insert(477, &[262272]);     // 477 => [262272]
            map.insert(478, &[66048]);     // 478 => [66048]
            map.insert(479, &[66049]);     // 479 => [66049]
            map.insert(480, &[4226]);     // 480 => [4226]
            map.insert(481, &[1064962]);     // 481 => [1064962]
            map.insert(482, &[4224]);     // 482 => [4224]
            map.insert(483, &[1064960]);     // 483 => [1064960]
            map.insert(484, &[135169]);     // 484 => [135169]
            map.insert(485, &[135168]);     // 485 => [135168]
            map.insert(486, &[4228]);     // 486 => [4228]
            map.insert(487, &[135170]);     // 487 => [135170]
            map.insert(488, &[8320]);     // 488 => [8320]
            map.insert(489, &[8321]);     // 489 => [8321]
            map.insert(490, &[2097185]);     // 490 => [2097185]
            map.insert(491, &[2097184]);     // 491 => [2097184]
            map.insert(492, &[8324]);     // 492 => [8324]
            map.insert(493, &[139266]);     // 493 => [139266]
            map.insert(494, &[139265]);     // 494 => [139265]
            map.insert(495, &[139264]);     // 495 => [139264]
            map.insert(496, &[2098184]);     // 496 => [2098184]
            map.insert(497, &[2564]);     // 497 => [2564]
            map.insert(498, &[4240]);     // 498 => [4240]
            map.insert(499, &[2097220]);     // 499 => [2097220]
            map.insert(500, &[2561]);     // 500 => [2561]
            map.insert(501, &[2560]);     // 501 => [2560]
            map.insert(502, &[2097217]);     // 502 => [2097217]
            map.insert(503, &[2097216]);     // 503 => [2097216]
            map.insert(504, &[2098176]);     // 504 => [2098176]
            map.insert(505, &[2098177]);     // 505 => [2098177]
            map.insert(506, &[2098178]);     // 506 => [2098178]
            map.insert(507, &[36868]);     // 507 => [36868]
            map.insert(508, &[4210688]);     // 508 => [4210688]
            map.insert(509, &[4210689]);     // 509 => [4210689]
            map.insert(510, &[36865]);     // 510 => [36865]
            map.insert(511, &[36864]);     // 511 => [36864]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl WagnerCode23_14 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for WagnerCode23_14 {
    fn name(&self) -> String {
        "[23, 14] Wagner code".to_owned()
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
        let code = WagnerCode23_14.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 14);
    }

    #[test]
    fn test_decode_sample() {
        let code = WagnerCode23_14;
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
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, false, true, true, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, true, false, true, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, false, true, false, false, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, true, false, false, false, false, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, true, false, false, false, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, true, true, true, false, false, false, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, false, true, true, false, false, false, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, false, false, true, false, false, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, true, false, false, false, false, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, true, false, false, true, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, true, true, false, false, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, false, false, true, true, false, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, false, false, true, false, true, true, false, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, true, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, false, false, false, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, false, false, true, true, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, false, false, false, true, false, false, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true, true, true, false, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, false, true, true, true, false, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, true, false, false, true, true, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, false, true, true, false, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, false, true, true, false, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, true, true, true, false, false, false, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, true, true, false, false, false, false, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, true, false, false, true, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, false, false, true, true, true, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, false, false, false, true, true, false, false, false, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, true, false, true, false, false, false, false, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, false, true, false, false, false, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, false, false, false, false, true, true, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, false, false, false, false, true, true, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, true, false, true, true, true, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, true, false, true, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, true, true, false, false, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, false, true, true, true, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, false, true, true, true, false, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, true, false, false, false, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, false, true, true, true, false, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, false, true, true, false, false, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, false, true, false, false, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, false, false, true, false, false, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, false, true, false, false, false, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, true, true, false, false, false, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, false, true, true, true, true, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, false, true, false, true, true, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, false, false, true, true, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, true, false, false, false, true, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, false, false, true, true, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, false, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, true, true, false, true, false, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, true, true, false, true, false, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, false, false, true, false, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, false, false, false, true, false, true, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, false, false, false, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, true, false, true, true, true, true, true, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, false, false, true, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, true, true, true, false, false, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, true, false, true, false, true, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, true, true, false, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, true, true, true, false, false, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, true, true, true, false, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, true, false, true, false, true, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true, false, true, false, true, true, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, false, false, true, false, false, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, true, false, true, false, true, true, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, false, true, false, false, true, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, false, false, true, false, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, false, false, true, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, true, false, false, true, false, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, false, true, false, false, true, false, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, true, false, true, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, true, false, true, false, true, false, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, true, false, true, true, true, false, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, true, false, true, false, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, true, false, false, true, false, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, true, true, true, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, false, true, true, true, true, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, true, true, true, true, true, false, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, true, true, true, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, true, false, true, true, true, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, false, false, true, false, false, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, true, true, false, false, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, true, false, false, false, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, true, true, true, false, true, false, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, true, false, false, true, false, true, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, false, true, false, true, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, false, false, false, true, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, false, false, false, true, false, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, false, false, true, false, true, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, false, false, true, false, false, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, true, true, true, false, false, false, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, false, false, true, false, false, false, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, true, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, false, false, true, true, true, true, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, false, false, false, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, true, true, false, false, true, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, true, true, false, false, true, true, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, true, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, true, false, false, true, true, false, false, true, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, false, false, true, true, false, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, true, false, true, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, true, false, false, false, false, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, false, false, true, false, false, true, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, true, false, false, true, true, true, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true, false, true, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, false, true, false, true, true, false, true, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, false, false, true, true, true, true, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, false, true, true, true, true, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, false, true, true, false, true, false, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, false, true, true, true, true, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, true, true, false, true, true, false, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, true, true, false, true, true, false, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, false, false, false, false, true, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, false, false, false, true, true, true, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, true, false, false, false, false, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, false, false, true, false, true, true, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, false, false, false, true, false, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, false, true, false, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, true, true, true, true, true, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, true, false, true, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, true, false, false, false, false, false, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, true, false, false, true, false, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false, true, true, false, false, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, true, false, false, true, false, false, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, false, false, true, true, false, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, false, true, false, true, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, true, false, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false, false, true, true, true, true, false, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, false, true, false, true, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, false, false, true, true, false, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, false, true, false, true, false, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, true, true, true, true, false, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, true, true, false, true, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, false, true, true, false, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, true, false, true, true, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, true, false, true, true, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, true, false, true, true, true, false, true, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, true, false, false, false, true, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, true, false, false, false, true, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, false, true, true, true, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, false, false, true, true, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, false, false, true, false, true, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, true, false, true, false, true, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, false, false, false, true, true, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, true, false, false, false, true, true, true, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, true, false, false, true, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, false, true, false, false, true, true, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, true, false, false, true, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, false, false, false, false, false, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, false, true, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, true, false, false, false, false, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false, true, false, false, true, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, false, true, false, false, true, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, false, true, false, false, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, false, false, true, false, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, true, true, true, false, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, false, false, true, true, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, true, false, false, false, true, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, true, false, false, true, true, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, false, false, true, false, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, false, false, true, false, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, true, true, true, false, true, true, true, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, true, false, true, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, true, true, true, false, true, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, true, true, true, false, true, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, false, true, false, false, false, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, true, true, false, true, false, true, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, false, false, false, true, true, true, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, true, true, true, true, false, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, true, false, false, true, true, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, false, true, true, false, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false, true, false, true, true, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, true, false, false, true, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, false, true, true, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, false, true, true, true, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, false, true, false, false, true, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, true, false, true, false, true, false, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, true, false, true, true, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, false, false, true, true, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, true, false, true, true, true, false, true, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, true, false, false, true, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, true, false, false, true, true, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, false, true, false, true, true, true, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, true, false, true, false, true, true, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, true, false, false, true, true, true, true, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, true, true, true, true, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, true, false, true, false, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, false, true, true, false, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, false, false, false, true, true, false, false, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, false, true, false, false, false, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, false, false, true, true, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, false, true, false, true, false, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, false, true, false, true, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, false, false, false, true, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, true, false, false, true, false, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, false, true, false, false, true, true, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, true, false, false, true, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false, true, true, true, false, false, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, true, false, false, true, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, true, true, true, false, true, false, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, true, false, true, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, true, false, true, true, true, false, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, true, false, true, true, true, false, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, true, false, false, false, false, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, false, false, true, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, false, true, true, false, false, false, true, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, true, true, true, false, true, false, true, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, true, true, true, false, true, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, false, true, true, true, true, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, true, false, true, true, true, true, true, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode23_14;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, false, false, false, true, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, true, true, false, false, false, false, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
