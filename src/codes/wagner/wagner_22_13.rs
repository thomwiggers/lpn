use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[22, 13]`` Wagner code
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct WagnerCode22_13;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 253953 ],
                &[ 843778 ],
                &[ 2793476 ],
                &[ 1916936 ],
                &[ 1105936 ],
                &[ 352288 ],
                &[ 2252864 ],
                &[ 2400384 ],
                &[ 3227904 ],
                &[ 2695680 ],
                &[ 1737728 ],
                &[ 1550336 ],
                &[ 3108864 ],
                
            ], 22));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 3663361 ],
                &[ 1938434 ],
                &[ 1008196 ],
                &[ 1761352 ],
                &[ 2508368 ],
                &[ 46688 ],
                &[ 3752576 ],
                &[ 1454848 ],
                &[ 3840000 ],
                
            ], 22));
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
            map.insert(7, &[65664]);     // 7 => [65664]
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
            map.insert(22, &[262401]);     // 22 => [262401]
            map.insert(23, &[262400]);     // 23 => [262400]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[81920]);     // 26 => [81920]
            map.insert(27, &[81921]);     // 27 => [81921]
            map.insert(28, &[96]);     // 28 => [96]
            map.insert(29, &[16512]);     // 29 => [16512]
            map.insert(30, &[2621441]);     // 30 => [2621441]
            map.insert(31, &[2621440]);     // 31 => [2621440]
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
            map.insert(42, &[34817]);     // 42 => [34817]
            map.insert(43, &[34816]);     // 43 => [34816]
            map.insert(44, &[80]);     // 44 => [80]
            map.insert(45, &[8448]);     // 45 => [8448]
            map.insert(46, &[82]);     // 46 => [82]
            map.insert(47, &[8450]);     // 47 => [8450]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(49, &[1026]);     // 49 => [1026]
            map.insert(50, &[1025]);     // 50 => [1025]
            map.insert(51, &[1024]);     // 51 => [1024]
            map.insert(52, &[72]);     // 52 => [72]
            map.insert(53, &[73]);     // 53 => [73]
            map.insert(54, &[1029]);     // 54 => [1029]
            map.insert(55, &[1028]);     // 55 => [1028]
            map.insert(56, &[68]);     // 56 => [68]
            map.insert(57, &[69]);     // 57 => [69]
            map.insert(58, &[270336]);     // 58 => [270336]
            map.insert(59, &[1032]);     // 59 => [1032]
            map.insert(60, &[64]);     // 60 => [64]
            map.insert(61, &[65]);     // 61 => [65]
            map.insert(62, &[66]);     // 62 => [66]
            map.insert(63, &[135168]);     // 63 => [135168]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(67, &[65540]);     // 67 => [65540]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(69, &[65538]);     // 69 => [65538]
            map.insert(70, &[65537]);     // 70 => [65537]
            map.insert(71, &[65536]);     // 71 => [65536]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(75, &[65548]);     // 75 => [65548]
            map.insert(76, &[16401]);     // 76 => [16401]
            map.insert(77, &[16400]);     // 77 => [16400]
            map.insert(78, &[526336]);     // 78 => [526336]
            map.insert(79, &[65544]);     // 79 => [65544]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(81, &[2099200]);     // 81 => [2099200]
            map.insert(82, &[146]);     // 82 => [146]
            map.insert(83, &[2099202]);     // 83 => [2099202]
            map.insert(84, &[16393]);     // 84 => [16393]
            map.insert(85, &[16392]);     // 85 => [16392]
            map.insert(86, &[1179648]);     // 86 => [1179648]
            map.insert(87, &[65552]);     // 87 => [65552]
            map.insert(88, &[8704]);     // 88 => [8704]
            map.insert(89, &[16388]);     // 89 => [16388]
            map.insert(90, &[8706]);     // 90 => [8706]
            map.insert(91, &[16390]);     // 91 => [16390]
            map.insert(92, &[16385]);     // 92 => [16385]
            map.insert(93, &[16384]);     // 93 => [16384]
            map.insert(94, &[16387]);     // 94 => [16387]
            map.insert(95, &[16386]);     // 95 => [16386]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(97, &[16448]);     // 97 => [16448]
            map.insert(98, &[262656]);     // 98 => [262656]
            map.insert(99, &[262657]);     // 99 => [262657]
            map.insert(100, &[557057]);     // 100 => [557057]
            map.insert(101, &[557056]);     // 101 => [557056]
            map.insert(102, &[65569]);     // 102 => [65569]
            map.insert(103, &[65568]);     // 103 => [65568]
            map.insert(104, &[1052673]);     // 104 => [1052673]
            map.insert(105, &[1052672]);     // 105 => [1052672]
            map.insert(106, &[17412]);     // 106 => [17412]
            map.insert(107, &[1052674]);     // 107 => [1052674]
            map.insert(108, &[17410]);     // 108 => [17410]
            map.insert(109, &[1052676]);     // 109 => [1052676]
            map.insert(110, &[17408]);     // 110 => [17408]
            map.insert(111, &[17409]);     // 111 => [17409]
            map.insert(112, &[66564]);     // 112 => [66564]
            map.insert(113, &[1154]);     // 113 => [1154]
            map.insert(114, &[1153]);     // 114 => [1153]
            map.insert(115, &[1152]);     // 115 => [1152]
            map.insert(116, &[66560]);     // 116 => [66560]
            map.insert(117, &[768]);     // 117 => [768]
            map.insert(118, &[66562]);     // 118 => [66562]
            map.insert(119, &[770]);     // 119 => [770]
            map.insert(120, &[2129922]);     // 120 => [2129922]
            map.insert(121, &[65602]);     // 121 => [65602]
            map.insert(122, &[2129920]);     // 122 => [2129920]
            map.insert(123, &[65600]);     // 123 => [65600]
            map.insert(124, &[192]);     // 124 => [192]
            map.insert(125, &[16416]);     // 125 => [16416]
            map.insert(126, &[194]);     // 126 => [194]
            map.insert(127, &[16418]);     // 127 => [16418]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(133, &[1572864]);     // 133 => [1572864]
            map.insert(134, &[262161]);     // 134 => [262161]
            map.insert(135, &[262160]);     // 135 => [262160]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(137, &[36864]);     // 137 => [36864]
            map.insert(138, &[266]);     // 138 => [266]
            map.insert(139, &[36866]);     // 139 => [36866]
            map.insert(140, &[8225]);     // 140 => [8225]
            map.insert(141, &[8224]);     // 141 => [8224]
            map.insert(142, &[9232]);     // 142 => [9232]
            map.insert(143, &[8226]);     // 143 => [8226]
            map.insert(144, &[272]);     // 144 => [272]
            map.insert(145, &[8256]);     // 145 => [8256]
            map.insert(146, &[262149]);     // 146 => [262149]
            map.insert(147, &[262148]);     // 147 => [262148]
            map.insert(148, &[262147]);     // 148 => [262147]
            map.insert(149, &[262146]);     // 149 => [262146]
            map.insert(150, &[262145]);     // 150 => [262145]
            map.insert(151, &[262144]);     // 151 => [262144]
            map.insert(152, &[3145730]);     // 152 => [3145730]
            map.insert(153, &[133124]);     // 153 => [133124]
            map.insert(154, &[3145728]);     // 154 => [3145728]
            map.insert(155, &[3145729]);     // 155 => [3145729]
            map.insert(156, &[133121]);     // 156 => [133121]
            map.insert(157, &[133120]);     // 157 => [133120]
            map.insert(158, &[9216]);     // 158 => [9216]
            map.insert(159, &[262152]);     // 159 => [262152]
            map.insert(160, &[288]);     // 160 => [288]
            map.insert(161, &[289]);     // 161 => [289]
            map.insert(162, &[6144]);     // 162 => [6144]
            map.insert(163, &[6145]);     // 163 => [6145]
            map.insert(164, &[263168]);     // 164 => [263168]
            map.insert(165, &[8200]);     // 165 => [8200]
            map.insert(166, &[263170]);     // 166 => [263170]
            map.insert(167, &[8202]);     // 167 => [8202]
            map.insert(168, &[16896]);     // 168 => [16896]
            map.insert(169, &[8196]);     // 169 => [8196]
            map.insert(170, &[262209]);     // 170 => [262209]
            map.insert(171, &[262208]);     // 171 => [262208]
            map.insert(172, &[8193]);     // 172 => [8193]
            map.insert(173, &[8192]);     // 173 => [8192]
            map.insert(174, &[8195]);     // 174 => [8195]
            map.insert(175, &[8194]);     // 175 => [8194]
            map.insert(176, &[66050]);     // 176 => [66050]
            map.insert(177, &[1282]);     // 177 => [1282]
            map.insert(178, &[66048]);     // 178 => [66048]
            map.insert(179, &[1280]);     // 179 => [1280]
            map.insert(180, &[641]);     // 180 => [641]
            map.insert(181, &[640]);     // 181 => [640]
            map.insert(182, &[163840]);     // 182 => [163840]
            map.insert(183, &[262176]);     // 183 => [262176]
            map.insert(184, &[324]);     // 184 => [324]
            map.insert(185, &[8212]);     // 185 => [8212]
            map.insert(186, &[66056]);     // 186 => [66056]
            map.insert(187, &[1288]);     // 187 => [1288]
            map.insert(188, &[320]);     // 188 => [320]
            map.insert(189, &[8208]);     // 189 => [8208]
            map.insert(190, &[322]);     // 190 => [322]
            map.insert(191, &[8210]);     // 191 => [8210]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(193, &[385]);     // 193 => [385]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(195, &[65796]);     // 195 => [65796]
            map.insert(196, &[1538]);     // 196 => [1538]
            map.insert(197, &[65794]);     // 197 => [65794]
            map.insert(198, &[1536]);     // 198 => [1536]
            map.insert(199, &[65792]);     // 199 => [65792]
            map.insert(200, &[577]);     // 200 => [577]
            map.insert(201, &[576]);     // 201 => [576]
            map.insert(202, &[278528]);     // 202 => [278528]
            map.insert(203, &[1050624]);     // 203 => [1050624]
            map.insert(204, &[2228224]);     // 204 => [2228224]
            map.insert(205, &[2228225]);     // 205 => [2228225]
            map.insert(206, &[2228226]);     // 206 => [2228226]
            map.insert(207, &[1050628]);     // 207 => [1050628]
            map.insert(208, &[327680]);     // 208 => [327680]
            map.insert(209, &[327681]);     // 209 => [327681]
            map.insert(210, &[655361]);     // 210 => [655361]
            map.insert(211, &[655360]);     // 211 => [655360]
            map.insert(212, &[545]);     // 212 => [545]
            map.insert(213, &[544]);     // 213 => [544]
            map.insert(214, &[262273]);     // 214 => [262273]
            map.insert(215, &[262272]);     // 215 => [262272]
            map.insert(216, &[327688]);     // 216 => [327688]
            map.insert(217, &[16644]);     // 217 => [16644]
            map.insert(218, &[278544]);     // 218 => [278544]
            map.insert(219, &[655368]);     // 219 => [655368]
            map.insert(220, &[16641]);     // 220 => [16641]
            map.insert(221, &[16640]);     // 221 => [16640]
            map.insert(222, &[9344]);     // 222 => [9344]
            map.insert(223, &[16642]);     // 223 => [16642]
            map.insert(224, &[1081344]);     // 224 => [1081344]
            map.insert(225, &[1081345]);     // 225 => [1081345]
            map.insert(226, &[1081346]);     // 226 => [1081346]
            map.insert(227, &[2101264]);     // 227 => [2101264]
            map.insert(228, &[529]);     // 228 => [529]
            map.insert(229, &[528]);     // 229 => [528]
            map.insert(230, &[1568]);     // 230 => [1568]
            map.insert(231, &[530]);     // 231 => [530]
            map.insert(232, &[73730]);     // 232 => [73730]
            map.insert(233, &[8324]);     // 233 => [8324]
            map.insert(234, &[73728]);     // 234 => [73728]
            map.insert(235, &[73729]);     // 235 => [73729]
            map.insert(236, &[528384]);     // 236 => [528384]
            map.insert(237, &[8320]);     // 237 => [8320]
            map.insert(238, &[528386]);     // 238 => [528386]
            map.insert(239, &[8322]);     // 239 => [8322]
            map.insert(240, &[24576]);     // 240 => [24576]
            map.insert(241, &[516]);     // 241 => [516]
            map.insert(242, &[2101249]);     // 242 => [2101249]
            map.insert(243, &[2101248]);     // 243 => [2101248]
            map.insert(244, &[513]);     // 244 => [513]
            map.insert(245, &[512]);     // 245 => [512]
            map.insert(246, &[515]);     // 246 => [515]
            map.insert(247, &[514]);     // 247 => [514]
            map.insert(248, &[24584]);     // 248 => [24584]
            map.insert(249, &[524]);     // 249 => [524]
            map.insert(250, &[73744]);     // 250 => [73744]
            map.insert(251, &[2101256]);     // 251 => [2101256]
            map.insert(252, &[521]);     // 252 => [521]
            map.insert(253, &[520]);     // 253 => [520]
            map.insert(254, &[1314816]);     // 254 => [1314816]
            map.insert(255, &[522]);     // 255 => [522]
            map.insert(256, &[2048]);     // 256 => [2048]
            map.insert(257, &[2049]);     // 257 => [2049]
            map.insert(258, &[2050]);     // 258 => [2050]
            map.insert(259, &[2051]);     // 259 => [2051]
            map.insert(260, &[2052]);     // 260 => [2052]
            map.insert(261, &[2053]);     // 261 => [2053]
            map.insert(262, &[2054]);     // 262 => [2054]
            map.insert(263, &[12296]);     // 263 => [12296]
            map.insert(264, &[2056]);     // 264 => [2056]
            map.insert(265, &[589824]);     // 265 => [589824]
            map.insert(266, &[393216]);     // 266 => [393216]
            map.insert(267, &[32800]);     // 267 => [32800]
            map.insert(268, &[2113536]);     // 268 => [2113536]
            map.insert(269, &[2113537]);     // 269 => [2113537]
            map.insert(270, &[524416]);     // 270 => [524416]
            map.insert(271, &[12288]);     // 271 => [12288]
            map.insert(272, &[2064]);     // 272 => [2064]
            map.insert(273, &[2097280]);     // 273 => [2097280]
            map.insert(274, &[540673]);     // 274 => [540673]
            map.insert(275, &[540672]);     // 275 => [540672]
            map.insert(276, &[2162690]);     // 276 => [2162690]
            map.insert(277, &[32834]);     // 277 => [32834]
            map.insert(278, &[2162688]);     // 278 => [2162688]
            map.insert(279, &[32832]);     // 279 => [32832]
            map.insert(280, &[33792]);     // 280 => [33792]
            map.insert(281, &[33793]);     // 281 => [33793]
            map.insert(282, &[33794]);     // 282 => [33794]
            map.insert(283, &[540680]);     // 283 => [540680]
            map.insert(284, &[131329]);     // 284 => [131329]
            map.insert(285, &[131328]);     // 285 => [131328]
            map.insert(286, &[2162696]);     // 286 => [2162696]
            map.insert(287, &[131330]);     // 287 => [131330]
            map.insert(288, &[2080]);     // 288 => [2080]
            map.insert(289, &[2081]);     // 289 => [2081]
            map.insert(290, &[4352]);     // 290 => [4352]
            map.insert(291, &[32776]);     // 291 => [32776]
            map.insert(292, &[2084]);     // 292 => [2084]
            map.insert(293, &[266256]);     // 293 => [266256]
            map.insert(294, &[4356]);     // 294 => [4356]
            map.insert(295, &[32780]);     // 295 => [32780]
            map.insert(296, &[32771]);     // 296 => [32771]
            map.insert(297, &[32770]);     // 297 => [32770]
            map.insert(298, &[32769]);     // 298 => [32769]
            map.insert(299, &[32768]);     // 299 => [32768]
            map.insert(300, &[2128]);     // 300 => [2128]
            map.insert(301, &[32774]);     // 301 => [32774]
            map.insert(302, &[32773]);     // 302 => [32773]
            map.insert(303, &[32772]);     // 303 => [32772]
            map.insert(304, &[139264]);     // 304 => [139264]
            map.insert(305, &[139265]);     // 305 => [139265]
            map.insert(306, &[3073]);     // 306 => [3073]
            map.insert(307, &[3072]);     // 307 => [3072]
            map.insert(308, &[266241]);     // 308 => [266241]
            map.insert(309, &[266240]);     // 309 => [266240]
            map.insert(310, &[1049096]);     // 310 => [1049096]
            map.insert(311, &[266242]);     // 311 => [266242]
            map.insert(312, &[2116]);     // 312 => [2116]
            map.insert(313, &[32786]);     // 313 => [32786]
            map.insert(314, &[32785]);     // 314 => [32785]
            map.insert(315, &[32784]);     // 315 => [32784]
            map.insert(316, &[2112]);     // 316 => [2112]
            map.insert(317, &[2113]);     // 317 => [2113]
            map.insert(318, &[1049088]);     // 318 => [1049088]
            map.insert(319, &[1049089]);     // 319 => [1049089]
            map.insert(320, &[2176]);     // 320 => [2176]
            map.insert(321, &[2097168]);     // 321 => [2097168]
            map.insert(322, &[2178]);     // 322 => [2178]
            map.insert(323, &[2097170]);     // 323 => [2097170]
            map.insert(324, &[524298]);     // 324 => [524298]
            map.insert(325, &[67586]);     // 325 => [67586]
            map.insert(326, &[524296]);     // 326 => [524296]
            map.insert(327, &[67584]);     // 327 => [67584]
            map.insert(328, &[524294]);     // 328 => [524294]
            map.insert(329, &[1048834]);     // 329 => [1048834]
            map.insert(330, &[524292]);     // 330 => [524292]
            map.insert(331, &[1048832]);     // 331 => [1048832]
            map.insert(332, &[524290]);     // 332 => [524290]
            map.insert(333, &[524291]);     // 333 => [524291]
            map.insert(334, &[524288]);     // 334 => [524288]
            map.insert(335, &[524289]);     // 335 => [524289]
            map.insert(336, &[2097153]);     // 336 => [2097153]
            map.insert(337, &[2097152]);     // 337 => [2097152]
            map.insert(338, &[2097155]);     // 338 => [2097155]
            map.insert(339, &[2097154]);     // 339 => [2097154]
            map.insert(340, &[2097157]);     // 340 => [2097157]
            map.insert(341, &[2097156]);     // 341 => [2097156]
            map.insert(342, &[4609]);     // 342 => [4609]
            map.insert(343, &[4608]);     // 343 => [4608]
            map.insert(344, &[2097161]);     // 344 => [2097161]
            map.insert(345, &[2097160]);     // 345 => [2097160]
            map.insert(346, &[524308]);     // 346 => [524308]
            map.insert(347, &[2097162]);     // 347 => [2097162]
            map.insert(348, &[1310720]);     // 348 => [1310720]
            map.insert(349, &[18432]);     // 349 => [18432]
            map.insert(350, &[524304]);     // 350 => [524304]
            map.insert(351, &[524305]);     // 351 => [524305]
            map.insert(352, &[2098178]);     // 352 => [2098178]
            map.insert(353, &[2097200]);     // 353 => [2097200]
            map.insert(354, &[2098176]);     // 354 => [2098176]
            map.insert(355, &[2098177]);     // 355 => [2098177]
            map.insert(356, &[1056770]);     // 356 => [1056770]
            map.insert(357, &[2097224]);     // 357 => [2097224]
            map.insert(358, &[1056768]);     // 358 => [1056768]
            map.insert(359, &[1056769]);     // 359 => [1056769]
            map.insert(360, &[131584]);     // 360 => [131584]
            map.insert(361, &[131585]);     // 361 => [131585]
            map.insert(362, &[32897]);     // 362 => [32897]
            map.insert(363, &[32896]);     // 363 => [32896]
            map.insert(364, &[98304]);     // 364 => [98304]
            map.insert(365, &[2097216]);     // 365 => [2097216]
            map.insert(366, &[524320]);     // 366 => [524320]
            map.insert(367, &[524321]);     // 367 => [524321]
            map.insert(368, &[2097185]);     // 368 => [2097185]
            map.insert(369, &[2097184]);     // 369 => [2097184]
            map.insert(370, &[524352]);     // 370 => [524352]
            map.insert(371, &[524353]);     // 371 => [524353]
            map.insert(372, &[49154]);     // 372 => [49154]
            map.insert(373, &[2097188]);     // 373 => [2097188]
            map.insert(374, &[49152]);     // 374 => [49152]
            map.insert(375, &[49153]);     // 375 => [49153]
            map.insert(376, &[131600]);     // 376 => [131600]
            map.insert(377, &[525316]);     // 377 => [525316]
            map.insert(378, &[524360]);     // 378 => [524360]
            map.insert(379, &[32912]);     // 379 => [32912]
            map.insert(380, &[525313]);     // 380 => [525313]
            map.insert(381, &[525312]);     // 381 => [525312]
            map.insert(382, &[49160]);     // 382 => [49160]
            map.insert(383, &[525314]);     // 383 => [525314]
            map.insert(384, &[2304]);     // 384 => [2304]
            map.insert(385, &[2305]);     // 385 => [2305]
            map.insert(386, &[4128]);     // 386 => [4128]
            map.insert(387, &[4129]);     // 387 => [4129]
            map.insert(388, &[40962]);     // 388 => [40962]
            map.insert(389, &[131096]);     // 389 => [131096]
            map.insert(390, &[40960]);     // 390 => [40960]
            map.insert(391, &[40961]);     // 391 => [40961]
            map.insert(392, &[1114116]);     // 392 => [1114116]
            map.insert(393, &[1048706]);     // 393 => [1048706]
            map.insert(394, &[1048705]);     // 394 => [1048705]
            map.insert(395, &[1048704]);     // 395 => [1048704]
            map.insert(396, &[1114112]);     // 396 => [1114112]
            map.insert(397, &[131088]);     // 397 => [131088]
            map.insert(398, &[1114114]);     // 398 => [1114114]
            map.insert(399, &[131090]);     // 399 => [131090]
            map.insert(400, &[5121]);     // 400 => [5121]
            map.insert(401, &[5120]);     // 401 => [5120]
            map.insert(402, &[1064964]);     // 402 => [1064964]
            map.insert(403, &[5122]);     // 403 => [5122]
            map.insert(404, &[131081]);     // 404 => [131081]
            map.insert(405, &[131080]);     // 405 => [131080]
            map.insert(406, &[1064960]);     // 406 => [1064960]
            map.insert(407, &[264192]);     // 407 => [264192]
            map.insert(408, &[131077]);     // 408 => [131077]
            map.insert(409, &[131076]);     // 409 => [131076]
            map.insert(410, &[4164]);     // 410 => [4164]
            map.insert(411, &[131078]);     // 411 => [131078]
            map.insert(412, &[131073]);     // 412 => [131073]
            map.insert(413, &[131072]);     // 413 => [131072]
            map.insert(414, &[4160]);     // 414 => [4160]
            map.insert(415, &[131074]);     // 415 => [131074]
            map.insert(416, &[4098]);     // 416 => [4098]
            map.insert(417, &[131136]);     // 417 => [131136]
            map.insert(418, &[4096]);     // 418 => [4096]
            map.insert(419, &[4097]);     // 419 => [4097]
            map.insert(420, &[2097664]);     // 420 => [2097664]
            map.insert(421, &[2097665]);     // 421 => [2097665]
            map.insert(422, &[4100]);     // 422 => [4100]
            map.insert(423, &[4101]);     // 423 => [4101]
            map.insert(424, &[4106]);     // 424 => [4106]
            map.insert(425, &[33026]);     // 425 => [33026]
            map.insert(426, &[4104]);     // 426 => [4104]
            map.insert(427, &[33024]);     // 427 => [33024]
            map.insert(428, &[10241]);     // 428 => [10241]
            map.insert(429, &[10240]);     // 429 => [10240]
            map.insert(430, &[132096]);     // 430 => [132096]
            map.insert(431, &[132097]);     // 431 => [132097]
            map.insert(432, &[4114]);     // 432 => [4114]
            map.insert(433, &[131152]);     // 433 => [131152]
            map.insert(434, &[4112]);     // 434 => [4112]
            map.insert(435, &[4113]);     // 435 => [4113]
            map.insert(436, &[294920]);     // 436 => [294920]
            map.insert(437, &[131112]);     // 437 => [131112]
            map.insert(438, &[4116]);     // 438 => [4116]
            map.insert(439, &[264224]);     // 439 => [264224]
            map.insert(440, &[294916]);     // 440 => [294916]
            map.insert(441, &[524802]);     // 441 => [524802]
            map.insert(442, &[524801]);     // 442 => [524801]
            map.insert(443, &[524800]);     // 443 => [524800]
            map.insert(444, &[294912]);     // 444 => [294912]
            map.insert(445, &[131104]);     // 445 => [131104]
            map.insert(446, &[294914]);     // 446 => [294914]
            map.insert(447, &[131106]);     // 447 => [131106]
            map.insert(448, &[147456]);     // 448 => [147456]
            map.insert(449, &[147457]);     // 449 => [147457]
            map.insert(450, &[1048585]);     // 450 => [1048585]
            map.insert(451, &[1048584]);     // 451 => [1048584]
            map.insert(452, &[2359298]);     // 452 => [2359298]
            map.insert(453, &[69664]);     // 453 => [69664]
            map.insert(454, &[2359296]);     // 454 => [2359296]
            map.insert(455, &[2359297]);     // 455 => [2359297]
            map.insert(456, &[1048579]);     // 456 => [1048579]
            map.insert(457, &[1048578]);     // 457 => [1048578]
            map.insert(458, &[1048577]);     // 458 => [1048577]
            map.insert(459, &[1048576]);     // 459 => [1048576]
            map.insert(460, &[524546]);     // 460 => [524546]
            map.insert(461, &[1048582]);     // 461 => [1048582]
            map.insert(462, &[524544]);     // 462 => [524544]
            map.insert(463, &[1048580]);     // 463 => [1048580]
            map.insert(464, &[2097409]);     // 464 => [2097409]
            map.insert(465, &[2097408]);     // 465 => [2097408]
            map.insert(466, &[196616]);     // 466 => [196616]
            map.insert(467, &[2097410]);     // 467 => [2097410]
            map.insert(468, &[3670016]);     // 468 => [3670016]
            map.insert(469, &[2097412]);     // 469 => [2097412]
            map.insert(470, &[33288]);     // 470 => [33288]
            map.insert(471, &[1048672]);     // 471 => [1048672]
            map.insert(472, &[786433]);     // 472 => [786433]
            map.insert(473, &[786432]);     // 473 => [786432]
            map.insert(474, &[196608]);     // 474 => [196608]
            map.insert(475, &[1048592]);     // 475 => [1048592]
            map.insert(476, &[131201]);     // 476 => [131201]
            map.insert(477, &[131200]);     // 477 => [131200]
            map.insert(478, &[33280]);     // 478 => [33280]
            map.insert(479, &[33281]);     // 479 => [33281]
            map.insert(480, &[4226]);     // 480 => [4226]
            map.insert(481, &[532482]);     // 481 => [532482]
            map.insert(482, &[4224]);     // 482 => [4224]
            map.insert(483, &[532480]);     // 483 => [532480]
            map.insert(484, &[69633]);     // 484 => [69633]
            map.insert(485, &[69632]);     // 485 => [69632]
            map.insert(486, &[4228]);     // 486 => [4228]
            map.insert(487, &[69634]);     // 487 => [69634]
            map.insert(488, &[1049616]);     // 488 => [1049616]
            map.insert(489, &[1048610]);     // 489 => [1048610]
            map.insert(490, &[1048609]);     // 490 => [1048609]
            map.insert(491, &[1048608]);     // 491 => [1048608]
            map.insert(492, &[2105360]);     // 492 => [2105360]
            map.insert(493, &[69640]);     // 493 => [69640]
            map.insert(494, &[524576]);     // 494 => [524576]
            map.insert(495, &[1048612]);     // 495 => [1048612]
            map.insert(496, &[1049608]);     // 496 => [1049608]
            map.insert(497, &[2564]);     // 497 => [2564]
            map.insert(498, &[4240]);     // 498 => [4240]
            map.insert(499, &[1048644]);     // 499 => [1048644]
            map.insert(500, &[2561]);     // 500 => [2561]
            map.insert(501, &[2560]);     // 501 => [2560]
            map.insert(502, &[1048641]);     // 502 => [1048641]
            map.insert(503, &[1048640]);     // 503 => [1048640]
            map.insert(504, &[1049600]);     // 504 => [1049600]
            map.insert(505, &[1049601]);     // 505 => [1049601]
            map.insert(506, &[1049602]);     // 506 => [1049602]
            map.insert(507, &[20484]);     // 507 => [20484]
            map.insert(508, &[2105344]);     // 508 => [2105344]
            map.insert(509, &[2105345]);     // 509 => [2105345]
            map.insert(510, &[20481]);     // 510 => [20481]
            map.insert(511, &[20480]);     // 511 => [20480]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl WagnerCode22_13 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for WagnerCode22_13 {
    fn name(&self) -> String {
        "[22, 13] Wagner code".to_owned()
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
        let code = WagnerCode22_13.generator_matrix();
        assert_eq!(code.ncols(), 22);
        assert_eq!(code.nrows(), 13);
    }

    #[test]
    fn test_decode_sample() {
        let code = WagnerCode22_13;
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
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, true, true, true, false, true, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, true, true, true, false, true, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, false, false, true, false, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, false, true, true, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, true, true, false, true, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, true, false, false, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, false, true, false, false, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, false, false, true, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, true, false, true, true, true, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, true, false, false, false, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, false, true, false, false, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, true, false, true, true, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, true, false, true, true, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, false, false, true, true, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, false, false, true, true, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, false, false, true, true, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, false, false, true, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, false, false, false, false, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, true, false, false, false, false, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, false, false, false, false, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, true, false, false, false, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, true, false, false, false, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, false, false, true, false, true, false, false, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, false, false, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, false, true, true, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, false, false, true, true, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, false, false, true, true, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, false, false, true, true, true, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, true, false, false, false, false, true, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, false, false, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, true, true, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, false, true, false, true, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, true, false, false, true, false, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, true, true, false, true, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, false, true, false, false, false, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, true, false, false, false, false, false, false, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, false, true, true, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, false, true, true, false, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, true, false, false, true, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, true, true, true, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, false, true, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, true, true, true, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, true, false, true, true, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, false, false, true, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, true, true, false, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, false, false, false, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, false, false, false, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, false, true, false, true, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, false, true, false, false, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, true, false, true, false, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, false, false, false, true, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, false, false, false, true, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, true, false, true, false, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, true, true, false, false, true, true, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, false, false, false, true, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, false, true, false, true, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, false, true, true, true, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, false, false, false, false, false, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, false, false, false, false, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, false, false, false, false, true, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, false, true, false, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, false, true, false, true, true, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, false, false, true, false, true, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, false, true, false, true, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, true, true, true, true, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, true, false, true, true, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, true, false, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, false, true, true, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false, false, false, false, true, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, false, false, false, false, true, false, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, false, true, true, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, false, true, true, false, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, false, false, true, true, false, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, false, true, false, true, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, false, true, false, true, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, false, true, false, true, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, false, true, true, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, true, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, true, false, true, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, true, true, false, true, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, false, false, true, true, true, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, true, false, true, true, true, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, false, false, false, false, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, true, true, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, true, false, false, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, false, false, true, true, true, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, false, false, true, true, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, true, true, false, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, false, true, false, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, true, false, false, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, true, true, false, false, false, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, false, true, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, true, false, false, true, false, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false, true, true, true, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, false, true, true, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, true, false, false, true, true, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, false, true, true, true, true, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, false, false, false, true, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, true, true, true, true, true, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, true, false, true, true, true, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, true, false, false, false, true, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, true, true, false, false, true, false, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, false, false, true, true, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, true, true, false, true, false, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false, true, true, true, false, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, true, true, false, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, false, false, false, false, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, false, false, false, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, false, true, false, true, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, false, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, true, false, true, true, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, true, true, false, true, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, false, true, true, false, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, true, false, true, true, false, true, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, false, true, false, false, false, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, true, false, true, true, false, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, false, false, false, false, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, true, true, true, true, false, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, true, true, true, false, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, true, false, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, true, false, true, false, false, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, true, false, true, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, false, false, false, false, true, true, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, false, false, false, true, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, true, true, true, true, true, false, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, true, true, true, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, true, true, false, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, false, true, true, false, false, true, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, false, true, false, false, false, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, false, true, true, false, false, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, false, false, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, false, false, true, true, true, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, false, false, true, true, true, true, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, false, true, false, false, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, true, true, true, false, false, true, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, true, false, false, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, true, false, true, true, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, true, false, true, true, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, true, false, false, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, true, true, false, false, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, true, true, false, true, true, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, false, true, true, true, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, true, true, true, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, false, true, true, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, false, true, false, false, true, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, true, false, false, true, false, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, true, false, false, true, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, true, false, false, true, true, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, true, false, false, true, true, true, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, false, true, false, true, false, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, false, true, false, false, false, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, true, false, true, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, true, true, false, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, true, false, false, false, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false, false, true, true, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, false, false, true, true, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, true, false, true, true, true, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, true, false, true, false, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, false, true, false, false, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, true, true, true, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, true, false, true, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, false, true, true, false, true, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, false, true, true, true, true, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, false, true, true, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, false, true, true, false, true, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, true, false, false, false, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, false, true, false, false, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, false, true, false, true, true, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, false, false, false, true, false, true, true, false, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true, true, false, false, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true, true, false, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, false, true, true, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, true, true, false, false, true, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, false, false, true, false, false, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, false, true, true, false, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, false, true, true, false, false, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, true, true, false, true, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false, false, false, true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, false, false, true, false, false, false, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, false, false, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, true, false, true, true, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, false, true, true, true, false, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, false, false, true, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, true, true, true, false, false, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, true, false, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, true, false, true, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, false, false, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, false, false, false, false, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, true, true, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, false, false, true, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, false, true, true, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, true, false, true, true, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, true, false, true, true, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, false, false, true, true, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, false, false, true, true, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, true, false, true, false, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, false, true, true, true, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true, false, true, true, true, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, true, false, true, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, false, true, true, false, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, true, true, true, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, true, false, true, false, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, true, false, true, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, false, false, true, true, false, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, false, true, false, false, true, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, true, true, false, false, true, false, false, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, true, true, true, true, false, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, true, true, true, false, false, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, false, false, true, false, false, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, false, true, false, false, true, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, true, true, false, false, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, true, true, true, true, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, true, false, false, true, false, false, true, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, true, false, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, true, true, true, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, true, true, true, false, true, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, true, false, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, true, true, true, false, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, false, true, true, true, false, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, true, false, false, false, false, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, false, true, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, false, true, false, true, true, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, false, true, true, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, false, false, true, false, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, true, false, true, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false, false, false, false, false, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, false, true, false, false, false, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, true, false, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, false, true, true, false, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, false, true, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, false, true, true, false, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, false, true, true, false, false, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, false, false, true, false, false, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, false, true, false, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, false, true, false, false, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, false, false, true, false, true, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, false, false, false, false, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, false, false, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, true, true, true, false, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, true, true, false, true, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, true, true, false, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode22_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, false, true, false, false, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
