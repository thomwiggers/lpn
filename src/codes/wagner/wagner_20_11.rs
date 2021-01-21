use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[20, 11]`` Wagner code
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct WagnerCode20_11;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 260097 ],
                &[ 849922 ],
                &[ 997380 ],
                &[ 1034248 ],
                &[ 1044496 ],
                &[ 88096 ],
                &[ 301120 ],
                &[ 661632 ],
                &[ 110848 ],
                &[ 807424 ],
                &[ 629760 ],
                
            ], 20));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 958481 ],
                &[ 745490 ],
                &[ 232468 ],
                &[ 634904 ],
                &[ 259104 ],
                &[ 694336 ],
                &[ 433280 ],
                &[ 421120 ],
                &[ 1033728 ],
                
            ], 20));
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
            map.insert(7, &[24]);     // 7 => [24]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(11, &[20]);     // 11 => [20]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(13, &[18]);     // 13 => [18]
            map.insert(14, &[17]);     // 14 => [17]
            map.insert(15, &[16]);     // 15 => [16]
            map.insert(16, &[32]);     // 16 => [32]
            map.insert(17, &[33]);     // 17 => [33]
            map.insert(18, &[34]);     // 18 => [34]
            map.insert(19, &[35]);     // 19 => [35]
            map.insert(20, &[36]);     // 20 => [36]
            map.insert(21, &[263168]);     // 21 => [263168]
            map.insert(22, &[38]);     // 22 => [38]
            map.insert(23, &[263170]);     // 23 => [263170]
            map.insert(24, &[40]);     // 24 => [40]
            map.insert(25, &[41]);     // 25 => [41]
            map.insert(26, &[8449]);     // 26 => [8449]
            map.insert(27, &[8448]);     // 27 => [8448]
            map.insert(28, &[66050]);     // 28 => [66050]
            map.insert(29, &[50]);     // 29 => [50]
            map.insert(30, &[66048]);     // 30 => [66048]
            map.insert(31, &[48]);     // 31 => [48]
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(35, &[132096]);     // 35 => [132096]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(39, &[132100]);     // 39 => [132100]
            map.insert(40, &[72]);     // 40 => [72]
            map.insert(41, &[73]);     // 41 => [73]
            map.insert(42, &[524801]);     // 42 => [524801]
            map.insert(43, &[524800]);     // 43 => [524800]
            map.insert(44, &[76]);     // 44 => [76]
            map.insert(45, &[82]);     // 45 => [82]
            map.insert(46, &[81]);     // 46 => [81]
            map.insert(47, &[80]);     // 47 => [80]
            map.insert(48, &[96]);     // 48 => [96]
            map.insert(49, &[97]);     // 49 => [97]
            map.insert(50, &[98]);     // 50 => [98]
            map.insert(51, &[132128]);     // 51 => [132128]
            map.insert(52, &[589825]);     // 52 => [589825]
            map.insert(53, &[589824]);     // 53 => [589824]
            map.insert(54, &[393216]);     // 54 => [393216]
            map.insert(55, &[393217]);     // 55 => [393217]
            map.insert(56, &[4224]);     // 56 => [4224]
            map.insert(57, &[4225]);     // 57 => [4225]
            map.insert(58, &[4226]);     // 58 => [4226]
            map.insert(59, &[524832]);     // 59 => [524832]
            map.insert(60, &[4228]);     // 60 => [4228]
            map.insert(61, &[589832]);     // 61 => [589832]
            map.insert(62, &[393224]);     // 62 => [393224]
            map.insert(63, &[112]);     // 63 => [112]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(70, &[17408]);     // 70 => [17408]
            map.insert(71, &[17409]);     // 71 => [17409]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(75, &[148]);     // 75 => [148]
            map.insert(76, &[140]);     // 76 => [140]
            map.insert(77, &[146]);     // 77 => [146]
            map.insert(78, &[145]);     // 78 => [145]
            map.insert(79, &[144]);     // 79 => [144]
            map.insert(80, &[160]);     // 80 => [160]
            map.insert(81, &[161]);     // 81 => [161]
            map.insert(82, &[278529]);     // 82 => [278529]
            map.insert(83, &[278528]);     // 83 => [278528]
            map.insert(84, &[557058]);     // 84 => [557058]
            map.insert(85, &[263296]);     // 85 => [263296]
            map.insert(86, &[557056]);     // 86 => [557056]
            map.insert(87, &[557057]);     // 87 => [557057]
            map.insert(88, &[4160]);     // 88 => [4160]
            map.insert(89, &[4161]);     // 89 => [4161]
            map.insert(90, &[4162]);     // 90 => [4162]
            map.insert(91, &[278536]);     // 91 => [278536]
            map.insert(92, &[4164]);     // 92 => [4164]
            map.insert(93, &[33344]);     // 93 => [33344]
            map.insert(94, &[557064]);     // 94 => [557064]
            map.insert(95, &[176]);     // 95 => [176]
            map.insert(96, &[192]);     // 96 => [192]
            map.insert(97, &[193]);     // 97 => [193]
            map.insert(98, &[98305]);     // 98 => [98305]
            map.insert(99, &[98304]);     // 99 => [98304]
            map.insert(100, &[2304]);     // 100 => [2304]
            map.insert(101, &[147456]);     // 101 => [147456]
            map.insert(102, &[2306]);     // 102 => [2306]
            map.insert(103, &[147458]);     // 103 => [147458]
            map.insert(104, &[4128]);     // 104 => [4128]
            map.insert(105, &[4129]);     // 105 => [4129]
            map.insert(106, &[4130]);     // 106 => [4130]
            map.insert(107, &[98312]);     // 107 => [98312]
            map.insert(108, &[4132]);     // 108 => [4132]
            map.insert(109, &[147464]);     // 109 => [147464]
            map.insert(110, &[165888]);     // 110 => [165888]
            map.insert(111, &[208]);     // 111 => [208]
            map.insert(112, &[4104]);     // 112 => [4104]
            map.insert(113, &[4105]);     // 113 => [4105]
            map.insert(114, &[4106]);     // 114 => [4106]
            map.insert(115, &[4116]);     // 115 => [4116]
            map.insert(116, &[4108]);     // 116 => [4108]
            map.insert(117, &[4114]);     // 117 => [4114]
            map.insert(118, &[4113]);     // 118 => [4113]
            map.insert(119, &[4112]);     // 119 => [4112]
            map.insert(120, &[4096]);     // 120 => [4096]
            map.insert(121, &[4097]);     // 121 => [4097]
            map.insert(122, &[4098]);     // 122 => [4098]
            map.insert(123, &[4099]);     // 123 => [4099]
            map.insert(124, &[4100]);     // 124 => [4100]
            map.insert(125, &[33280]);     // 125 => [33280]
            map.insert(126, &[10241]);     // 126 => [10241]
            map.insert(127, &[10240]);     // 127 => [10240]
            map.insert(128, &[256]);     // 128 => [256]
            map.insert(129, &[257]);     // 129 => [257]
            map.insert(130, &[258]);     // 130 => [258]
            map.insert(131, &[259]);     // 131 => [259]
            map.insert(132, &[260]);     // 132 => [260]
            map.insert(133, &[261]);     // 133 => [261]
            map.insert(134, &[262]);     // 134 => [262]
            map.insert(135, &[280]);     // 135 => [280]
            map.insert(136, &[264]);     // 136 => [264]
            map.insert(137, &[265]);     // 137 => [265]
            map.insert(138, &[163840]);     // 138 => [163840]
            map.insert(139, &[8224]);     // 139 => [8224]
            map.insert(140, &[81920]);     // 140 => [81920]
            map.insert(141, &[81921]);     // 141 => [81921]
            map.insert(142, &[273]);     // 142 => [273]
            map.insert(143, &[272]);     // 143 => [272]
            map.insert(144, &[288]);     // 144 => [288]
            map.insert(145, &[289]);     // 145 => [289]
            map.insert(146, &[16896]);     // 146 => [16896]
            map.insert(147, &[8200]);     // 147 => [8200]
            map.insert(148, &[8208]);     // 148 => [8208]
            map.insert(149, &[8209]);     // 149 => [8209]
            map.insert(150, &[8210]);     // 150 => [8210]
            map.insert(151, &[8204]);     // 151 => [8204]
            map.insert(152, &[8195]);     // 152 => [8195]
            map.insert(153, &[8194]);     // 153 => [8194]
            map.insert(154, &[8193]);     // 154 => [8193]
            map.insert(155, &[8192]);     // 155 => [8192]
            map.insert(156, &[6144]);     // 156 => [6144]
            map.insert(157, &[6145]);     // 157 => [6145]
            map.insert(158, &[8197]);     // 158 => [8197]
            map.insert(159, &[8196]);     // 159 => [8196]
            map.insert(160, &[320]);     // 160 => [320]
            map.insert(161, &[321]);     // 161 => [321]
            map.insert(162, &[322]);     // 162 => [322]
            map.insert(163, &[12416]);     // 163 => [12416]
            map.insert(164, &[2176]);     // 164 => [2176]
            map.insert(165, &[2177]);     // 165 => [2177]
            map.insert(166, &[2178]);     // 166 => [2178]
            map.insert(167, &[331776]);     // 167 => [331776]
            map.insert(168, &[33793]);     // 168 => [33793]
            map.insert(169, &[33792]);     // 169 => [33792]
            map.insert(170, &[163904]);     // 170 => [163904]
            map.insert(171, &[33794]);     // 171 => [33794]
            map.insert(172, &[2184]);     // 172 => [2184]
            map.insert(173, &[33796]);     // 173 => [33796]
            map.insert(174, &[598016]);     // 174 => [598016]
            map.insert(175, &[336]);     // 175 => [336]
            map.insert(176, &[352]);     // 176 => [352]
            map.insert(177, &[540680]);     // 177 => [540680]
            map.insert(178, &[16960]);     // 178 => [16960]
            map.insert(179, &[8264]);     // 179 => [8264]
            map.insert(180, &[294920]);     // 180 => [294920]
            map.insert(181, &[590080]);     // 181 => [590080]
            map.insert(182, &[540688]);     // 182 => [540688]
            map.insert(183, &[131712]);     // 183 => [131712]
            map.insert(184, &[540673]);     // 184 => [540673]
            map.insert(185, &[540672]);     // 185 => [540672]
            map.insert(186, &[8257]);     // 186 => [8257]
            map.insert(187, &[8256]);     // 187 => [8256]
            map.insert(188, &[294912]);     // 188 => [294912]
            map.insert(189, &[294913]);     // 189 => [294913]
            map.insert(190, &[294914]);     // 190 => [294914]
            map.insert(191, &[8260]);     // 191 => [8260]
            map.insert(192, &[384]);     // 192 => [384]
            map.insert(193, &[262656]);     // 193 => [262656]
            map.insert(194, &[386]);     // 194 => [386]
            map.insert(195, &[262658]);     // 195 => [262658]
            map.insert(196, &[2112]);     // 196 => [2112]
            map.insert(197, &[2113]);     // 197 => [2113]
            map.insert(198, &[2114]);     // 198 => [2114]
            map.insert(199, &[134144]);     // 199 => [134144]
            map.insert(200, &[66562]);     // 200 => [66562]
            map.insert(201, &[262664]);     // 201 => [262664]
            map.insert(202, &[66560]);     // 202 => [66560]
            map.insert(203, &[66561]);     // 203 => [66561]
            map.insert(204, &[2120]);     // 204 => [2120]
            map.insert(205, &[565248]);     // 205 => [565248]
            map.insert(206, &[66564]);     // 206 => [66564]
            map.insert(207, &[400]);     // 207 => [400]
            map.insert(208, &[1540]);     // 208 => [1540]
            map.insert(209, &[262688]);     // 209 => [262688]
            map.insert(210, &[17024]);     // 210 => [17024]
            map.insert(211, &[8328]);     // 211 => [8328]
            map.insert(212, &[1536]);     // 212 => [1536]
            map.insert(213, &[1537]);     // 213 => [1537]
            map.insert(214, &[1538]);     // 214 => [1538]
            map.insert(215, &[327688]);     // 215 => [327688]
            map.insert(216, &[655364]);     // 216 => [655364]
            map.insert(217, &[8322]);     // 217 => [8322]
            map.insert(218, &[8321]);     // 218 => [8321]
            map.insert(219, &[8320]);     // 219 => [8320]
            map.insert(220, &[655360]);     // 220 => [655360]
            map.insert(221, &[655361]);     // 221 => [655361]
            map.insert(222, &[327681]);     // 222 => [327681]
            map.insert(223, &[327680]);     // 223 => [327680]
            map.insert(224, &[2052]);     // 224 => [2052]
            map.insert(225, &[2053]);     // 225 => [2053]
            map.insert(226, &[12289]);     // 226 => [12289]
            map.insert(227, &[12288]);     // 227 => [12288]
            map.insert(228, &[2048]);     // 228 => [2048]
            map.insert(229, &[2049]);     // 229 => [2049]
            map.insert(230, &[2050]);     // 230 => [2050]
            map.insert(231, &[2051]);     // 231 => [2051]
            map.insert(232, &[196609]);     // 232 => [196609]
            map.insert(233, &[196608]);     // 233 => [196608]
            map.insert(234, &[786432]);     // 234 => [786432]
            map.insert(235, &[2064]);     // 235 => [2064]
            map.insert(236, &[2056]);     // 236 => [2056]
            map.insert(237, &[2057]);     // 237 => [2057]
            map.insert(238, &[49153]);     // 238 => [49153]
            map.insert(239, &[49152]);     // 239 => [49152]
            map.insert(240, &[2084]);     // 240 => [2084]
            map.insert(241, &[265216]);     // 241 => [265216]
            map.insert(242, &[167936]);     // 242 => [167936]
            map.insert(243, &[131588]);     // 243 => [131588]
            map.insert(244, &[2080]);     // 244 => [2080]
            map.insert(245, &[2081]);     // 245 => [2081]
            map.insert(246, &[131585]);     // 246 => [131585]
            map.insert(247, &[131584]);     // 247 => [131584]
            map.insert(248, &[4352]);     // 248 => [4352]
            map.insert(249, &[4353]);     // 249 => [4353]
            map.insert(250, &[4354]);     // 250 => [4354]
            map.insert(251, &[525316]);     // 251 => [525316]
            map.insert(252, &[4356]);     // 252 => [4356]
            map.insert(253, &[525314]);     // 253 => [525314]
            map.insert(254, &[525313]);     // 254 => [525313]
            map.insert(255, &[525312]);     // 255 => [525312]
            map.insert(256, &[512]);     // 256 => [512]
            map.insert(257, &[513]);     // 257 => [513]
            map.insert(258, &[514]);     // 258 => [514]
            map.insert(259, &[515]);     // 259 => [515]
            map.insert(260, &[516]);     // 260 => [516]
            map.insert(261, &[36864]);     // 261 => [36864]
            map.insert(262, &[518]);     // 262 => [518]
            map.insert(263, &[36866]);     // 263 => [36866]
            map.insert(264, &[520]);     // 264 => [520]
            map.insert(265, &[24576]);     // 265 => [24576]
            map.insert(266, &[524353]);     // 266 => [524353]
            map.insert(267, &[524352]);     // 267 => [524352]
            map.insert(268, &[65570]);     // 268 => [65570]
            map.insert(269, &[530]);     // 269 => [530]
            map.insert(270, &[65568]);     // 270 => [65568]
            map.insert(271, &[528]);     // 271 => [528]
            map.insert(272, &[544]);     // 272 => [544]
            map.insert(273, &[65552]);     // 273 => [65552]
            map.insert(274, &[16640]);     // 274 => [16640]
            map.insert(275, &[133120]);     // 275 => [133120]
            map.insert(276, &[65546]);     // 276 => [65546]
            map.insert(277, &[65556]);     // 277 => [65556]
            map.insert(278, &[65544]);     // 278 => [65544]
            map.insert(279, &[65545]);     // 279 => [65545]
            map.insert(280, &[65542]);     // 280 => [65542]
            map.insert(281, &[65560]);     // 281 => [65560]
            map.insert(282, &[65540]);     // 282 => [65540]
            map.insert(283, &[65541]);     // 283 => [65541]
            map.insert(284, &[65538]);     // 284 => [65538]
            map.insert(285, &[65539]);     // 285 => [65539]
            map.insert(286, &[65536]);     // 286 => [65536]
            map.insert(287, &[65537]);     // 287 => [65537]
            map.insert(288, &[576]);     // 288 => [576]
            map.insert(289, &[577]);     // 289 => [577]
            map.insert(290, &[524297]);     // 290 => [524297]
            map.insert(291, &[524296]);     // 291 => [524296]
            map.insert(292, &[524304]);     // 292 => [524304]
            map.insert(293, &[264192]);     // 293 => [264192]
            map.insert(294, &[524306]);     // 294 => [524306]
            map.insert(295, &[264194]);     // 295 => [264194]
            map.insert(296, &[524291]);     // 296 => [524291]
            map.insert(297, &[524290]);     // 297 => [524290]
            map.insert(298, &[524289]);     // 298 => [524289]
            map.insert(299, &[524288]);     // 299 => [524288]
            map.insert(300, &[524312]);     // 300 => [524312]
            map.insert(301, &[524294]);     // 301 => [524294]
            map.insert(302, &[524293]);     // 302 => [524293]
            map.insert(303, &[524292]);     // 303 => [524292]
            map.insert(304, &[3072]);     // 304 => [3072]
            map.insert(305, &[3073]);     // 305 => [3073]
            map.insert(306, &[3074]);     // 306 => [3074]
            map.insert(307, &[524328]);     // 307 => [524328]
            map.insert(308, &[3076]);     // 308 => [3076]
            map.insert(309, &[32904]);     // 309 => [32904]
            map.insert(310, &[65608]);     // 310 => [65608]
            map.insert(311, &[131456]);     // 311 => [131456]
            map.insert(312, &[3080]);     // 312 => [3080]
            map.insert(313, &[524322]);     // 313 => [524322]
            map.insert(314, &[524321]);     // 314 => [524321]
            map.insert(315, &[524320]);     // 315 => [524320]
            map.insert(316, &[32897]);     // 316 => [32897]
            map.insert(317, &[32896]);     // 317 => [32896]
            map.insert(318, &[65600]);     // 318 => [65600]
            map.insert(319, &[65601]);     // 319 => [65601]
            map.insert(320, &[640]);     // 320 => [640]
            map.insert(321, &[262400]);     // 321 => [262400]
            map.insert(322, &[642]);     // 322 => [642]
            map.insert(323, &[262402]);     // 323 => [262402]
            map.insert(324, &[644]);     // 324 => [644]
            map.insert(325, &[262404]);     // 325 => [262404]
            map.insert(326, &[69696]);     // 326 => [69696]
            map.insert(327, &[9224]);     // 327 => [9224]
            map.insert(328, &[648]);     // 328 => [648]
            map.insert(329, &[262408]);     // 329 => [262408]
            map.insert(330, &[270368]);     // 330 => [270368]
            map.insert(331, &[9220]);     // 331 => [9220]
            map.insert(332, &[139328]);     // 332 => [139328]
            map.insert(333, &[9218]);     // 333 => [9218]
            map.insert(334, &[9217]);     // 334 => [9217]
            map.insert(335, &[9216]);     // 335 => [9216]
            map.insert(336, &[1284]);     // 336 => [1284]
            map.insert(337, &[528386]);     // 337 => [528386]
            map.insert(338, &[528385]);     // 338 => [528385]
            map.insert(339, &[528384]);     // 339 => [528384]
            map.insert(340, &[1280]);     // 340 => [1280]
            map.insert(341, &[1281]);     // 341 => [1281]
            map.insert(342, &[1282]);     // 342 => [1282]
            map.insert(343, &[528388]);     // 343 => [528388]
            map.insert(344, &[270338]);     // 344 => [270338]
            map.insert(345, &[32836]);     // 345 => [32836]
            map.insert(346, &[270336]);     // 346 => [270336]
            map.insert(347, &[270337]);     // 347 => [270337]
            map.insert(348, &[32833]);     // 348 => [32833]
            map.insert(349, &[32832]);     // 349 => [32832]
            map.insert(350, &[65664]);     // 350 => [65664]
            map.insert(351, &[65665]);     // 351 => [65665]
            map.insert(352, &[704]);     // 352 => [704]
            map.insert(353, &[262464]);     // 353 => [262464]
            map.insert(354, &[69636]);     // 354 => [69636]
            map.insert(355, &[524424]);     // 355 => [524424]
            map.insert(356, &[69634]);     // 356 => [69634]
            map.insert(357, &[32808]);     // 357 => [32808]
            map.insert(358, &[69632]);     // 358 => [69632]
            map.insert(359, &[69633]);     // 359 => [69633]
            map.insert(360, &[139268]);     // 360 => [139268]
            map.insert(361, &[524418]);     // 361 => [524418]
            map.insert(362, &[524417]);     // 362 => [524417]
            map.insert(363, &[524416]);     // 363 => [524416]
            map.insert(364, &[139264]);     // 364 => [139264]
            map.insert(365, &[32800]);     // 365 => [32800]
            map.insert(366, &[139266]);     // 366 => [139266]
            map.insert(367, &[32802]);     // 367 => [32802]
            map.insert(368, &[32786]);     // 368 => [32786]
            map.insert(369, &[32780]);     // 369 => [32780]
            map.insert(370, &[32784]);     // 370 => [32784]
            map.insert(371, &[32785]);     // 371 => [32785]
            map.insert(372, &[32777]);     // 372 => [32777]
            map.insert(373, &[32776]);     // 373 => [32776]
            map.insert(374, &[18432]);     // 374 => [18432]
            map.insert(375, &[131328]);     // 375 => [131328]
            map.insert(376, &[4608]);     // 376 => [4608]
            map.insert(377, &[32772]);     // 377 => [32772]
            map.insert(378, &[4610]);     // 378 => [4610]
            map.insert(379, &[32774]);     // 379 => [32774]
            map.insert(380, &[32769]);     // 380 => [32769]
            map.insert(381, &[32768]);     // 381 => [32768]
            map.insert(382, &[32771]);     // 382 => [32771]
            map.insert(383, &[32770]);     // 383 => [32770]
            map.insert(384, &[768]);     // 384 => [768]
            map.insert(385, &[262272]);     // 385 => [262272]
            map.insert(386, &[16416]);     // 386 => [16416]
            map.insert(387, &[16417]);     // 387 => [16417]
            map.insert(388, &[73729]);     // 388 => [73729]
            map.insert(389, &[73728]);     // 389 => [73728]
            map.insert(390, &[16420]);     // 390 => [16420]
            map.insert(391, &[73730]);     // 391 => [73730]
            map.insert(392, &[776]);     // 392 => [776]
            map.insert(393, &[262280]);     // 393 => [262280]
            map.insert(394, &[16424]);     // 394 => [16424]
            map.insert(395, &[135172]);     // 395 => [135172]
            map.insert(396, &[5184]);     // 396 => [5184]
            map.insert(397, &[135170]);     // 397 => [135170]
            map.insert(398, &[135169]);     // 398 => [135169]
            map.insert(399, &[135168]);     // 399 => [135168]
            map.insert(400, &[16386]);     // 400 => [16386]
            map.insert(401, &[16387]);     // 401 => [16387]
            map.insert(402, &[16384]);     // 402 => [16384]
            map.insert(403, &[16385]);     // 403 => [16385]
            map.insert(404, &[1152]);     // 404 => [1152]
            map.insert(405, &[1153]);     // 405 => [1153]
            map.insert(406, &[16388]);     // 406 => [16388]
            map.insert(407, &[16389]);     // 407 => [16389]
            map.insert(408, &[34817]);     // 408 => [34817]
            map.insert(409, &[34816]);     // 409 => [34816]
            map.insert(410, &[16392]);     // 410 => [16392]
            map.insert(411, &[8704]);     // 411 => [8704]
            map.insert(412, &[16401]);     // 412 => [16401]
            map.insert(413, &[16400]);     // 413 => [16400]
            map.insert(414, &[65792]);     // 414 => [65792]
            map.insert(415, &[65793]);     // 415 => [65793]
            map.insert(416, &[532512]);     // 416 => [532512]
            map.insert(417, &[262336]);     // 417 => [262336]
            map.insert(418, &[16480]);     // 418 => [16480]
            map.insert(419, &[524552]);     // 419 => [524552]
            map.insert(420, &[5128]);     // 420 => [5128]
            map.insert(421, &[73792]);     // 421 => [73792]
            map.insert(422, &[41088]);     // 422 => [41088]
            map.insert(423, &[131232]);     // 423 => [131232]
            map.insert(424, &[5124]);     // 424 => [5124]
            map.insert(425, &[524546]);     // 425 => [524546]
            map.insert(426, &[524545]);     // 426 => [524545]
            map.insert(427, &[524544]);     // 427 => [524544]
            map.insert(428, &[5120]);     // 428 => [5120]
            map.insert(429, &[5121]);     // 429 => [5121]
            map.insert(430, &[5122]);     // 430 => [5122]
            map.insert(431, &[524548]);     // 431 => [524548]
            map.insert(432, &[532480]);     // 432 => [532480]
            map.insert(433, &[532481]);     // 433 => [532481]
            map.insert(434, &[16448]);     // 434 => [16448]
            map.insert(435, &[16449]);     // 435 => [16449]
            map.insert(436, &[532484]);     // 436 => [532484]
            map.insert(437, &[131202]);     // 437 => [131202]
            map.insert(438, &[131201]);     // 438 => [131201]
            map.insert(439, &[131200]);     // 439 => [131200]
            map.insert(440, &[266241]);     // 440 => [266241]
            map.insert(441, &[266240]);     // 441 => [266240]
            map.insert(442, &[16456]);     // 442 => [16456]
            map.insert(443, &[266242]);     // 443 => [266242]
            map.insert(444, &[5152]);     // 444 => [5152]
            map.insert(445, &[266244]);     // 445 => [266244]
            map.insert(446, &[65856]);     // 446 => [65856]
            map.insert(447, &[131208]);     // 447 => [131208]
            map.insert(448, &[262145]);     // 448 => [262145]
            map.insert(449, &[262144]);     // 449 => [262144]
            map.insert(450, &[262147]);     // 450 => [262147]
            map.insert(451, &[262146]);     // 451 => [262146]
            map.insert(452, &[1056]);     // 452 => [1056]
            map.insert(453, &[262148]);     // 453 => [262148]
            map.insert(454, &[1058]);     // 454 => [1058]
            map.insert(455, &[262150]);     // 455 => [262150]
            map.insert(456, &[262153]);     // 456 => [262153]
            map.insert(457, &[262152]);     // 457 => [262152]
            map.insert(458, &[262164]);     // 458 => [262164]
            map.insert(459, &[262154]);     // 459 => [262154]
            map.insert(460, &[262162]);     // 460 => [262162]
            map.insert(461, &[526338]);     // 461 => [526338]
            map.insert(462, &[262160]);     // 462 => [262160]
            map.insert(463, &[526336]);     // 463 => [526336]
            map.insert(464, &[1028]);     // 464 => [1028]
            map.insert(465, &[262176]);     // 465 => [262176]
            map.insert(466, &[16512]);     // 466 => [16512]
            map.insert(467, &[16513]);     // 467 => [16513]
            map.insert(468, &[1024]);     // 468 => [1024]
            map.insert(469, &[1025]);     // 469 => [1025]
            map.insert(470, &[1026]);     // 470 => [1026]
            map.insert(471, &[131136]);     // 471 => [131136]
            map.insert(472, &[1036]);     // 472 => [1036]
            map.insert(473, &[1042]);     // 473 => [1042]
            map.insert(474, &[1041]);     // 474 => [1041]
            map.insert(475, &[1040]);     // 475 => [1040]
            map.insert(476, &[1032]);     // 476 => [1032]
            map.insert(477, &[1033]);     // 477 => [1033]
            map.insert(478, &[1034]);     // 478 => [1034]
            map.insert(479, &[1044]);     // 479 => [1044]
            map.insert(480, &[262209]);     // 480 => [262209]
            map.insert(481, &[262208]);     // 481 => [262208]
            map.insert(482, &[40964]);     // 482 => [40964]
            map.insert(483, &[262210]);     // 483 => [262210]
            map.insert(484, &[2560]);     // 484 => [2560]
            map.insert(485, &[2561]);     // 485 => [2561]
            map.insert(486, &[40960]);     // 486 => [40960]
            map.insert(487, &[131104]);     // 487 => [131104]
            map.insert(488, &[20482]);     // 488 => [20482]
            map.insert(489, &[262216]);     // 489 => [262216]
            map.insert(490, &[20480]);     // 490 => [20480]
            map.insert(491, &[20481]);     // 491 => [20481]
            map.insert(492, &[2568]);     // 492 => [2568]
            map.insert(493, &[33056]);     // 493 => [33056]
            map.insert(494, &[20484]);     // 494 => [20484]
            map.insert(495, &[131112]);     // 495 => [131112]
            map.insert(496, &[1092]);     // 496 => [1092]
            map.insert(497, &[131078]);     // 497 => [131078]
            map.insert(498, &[131077]);     // 498 => [131077]
            map.insert(499, &[131076]);     // 499 => [131076]
            map.insert(500, &[1088]);     // 500 => [1088]
            map.insert(501, &[131074]);     // 501 => [131074]
            map.insert(502, &[131073]);     // 502 => [131073]
            map.insert(503, &[131072]);     // 503 => [131072]
            map.insert(504, &[131088]);     // 504 => [131088]
            map.insert(505, &[131089]);     // 505 => [131089]
            map.insert(506, &[67584]);     // 506 => [67584]
            map.insert(507, &[67585]);     // 507 => [67585]
            map.insert(508, &[33025]);     // 508 => [33025]
            map.insert(509, &[33024]);     // 509 => [33024]
            map.insert(510, &[131081]);     // 510 => [131081]
            map.insert(511, &[131080]);     // 511 => [131080]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl WagnerCode20_11 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for WagnerCode20_11 {
    fn name(&self) -> String {
        "[20, 11] Wagner code".to_owned()
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
        let code = WagnerCode20_11.generator_matrix();
        assert_eq!(code.ncols(), 20);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn test_decode_sample() {
        let code = WagnerCode20_11;
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
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, false, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, true, true, false, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, true, true, false, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, false, false, true, false, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, false, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, false, true, false, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, true, true, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, false, true, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, false, false, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, false, false, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, false, false, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, false, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, true, false, false, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, false, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, false, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, false, true, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, false, true, true, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, false, true, false, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, true, false, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, true, true, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, false, true, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, true, false, false, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, true, false, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, false, true, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, true, false, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, true, true, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, true, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, true, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, false, true, true, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, false, true, false, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, true, false, true, false, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, false, true, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, true, true, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, true, false, false, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, false, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, true, true, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, false, false, true, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, true, true, false, true, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, false, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, true, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, false, false, true, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, false, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, false, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, true, true, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, false, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, false, false, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, false, true, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, false, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, false, true, true, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, false, true, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, true, true, false, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, true, false, false, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, true, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, true, false, false, false, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, true, false, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, false, false, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, false, false, false, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, false, true, false, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, false, true, true, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, false, true, false, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, false, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, false, true, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, false, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, true, true, true, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, true, false, false, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, true, true, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, true, true, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, true, true, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, true, true, true, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, true, true, true, false, false, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, true, false, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, true, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, true, true, true, true, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, true, true, true, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, false, false, true, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, false, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, true, false, true, true, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, false, true, true, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, true, false, false, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, true, false, false, true, true, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, true, false, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, true, false, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, true, true, false, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, false, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, true, false, true, false, true, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, false, false, false, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, false, true, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, false, false, true, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, false, true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, false, true, false, true, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, false, false, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, true, false, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, true, true, false, true, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, false, true, false, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, false, true, false, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, true, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, false, false, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, false, false, true, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, true, true, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, false, true, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, false, true, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, false, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true, true, false, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, true, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, true, false, true, false, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, true, true, false, true, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, false, true, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, true, false, true, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, false, false, false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, false, false, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, false, false, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, false, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, true, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, true, false, false, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, false, true, true, false, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, true, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, false, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, true, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, false, true, false, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, false, true, false, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, false, false, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, false, true, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, false, false, false, false, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, false, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, false, true, true, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, false, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, false, false, true, false, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, false, false, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, true, false, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, true, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, true, true, false, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, true, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, false, false, false, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, false, true, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, false, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, true, false, false, true, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, true, false, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, false, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, false, false, false, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, false, false, false, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, true, false, false, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, false, false, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, true, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, false, true, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, false, true, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, false, false, true, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, true, true, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, true, false, false, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, true, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, true, true, true, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, true, true, true, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, true, true, false, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, true, true, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, false, true, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, false, true, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, false, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, true, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, true, true, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, false, true, true, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, true, false, false, true, true, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, true, true, false, true, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, true, false, true, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true, false, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, false, true, true, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, true, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, true, true, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, false, true, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, false, true, true, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, false, false, false, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, true, false, false, true, false, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, false, false, false, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, false, false, false, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, true, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, true, false, true, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, true, true, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, true, false, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, false, false, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, false, false, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, true, true, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, true, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, true, false, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false, false, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, false, false, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, true, true, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, true, true, false, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, true, true, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, true, true, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, true, true, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, true, false, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, false, false, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, true, false, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, true, true, true, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, true, true, true, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, true, false, false, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, true, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, true, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, true, false, true, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, true, false, false, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false, true, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, true, false, true, false, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, false, false, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, false, false, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, false, true, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, true, false, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, false, false, true, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, false, false, false, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, false, true, false, true, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, false, true, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, true, false, true, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, true, true, true, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, true, true, false, true, false, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, true, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, false, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, true, false, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, false, false, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, true, true, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, false, true, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, true, false, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, false, true, true, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, true, true, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, false, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, true, false, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, false, true, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, false, true, false, true, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, true, true, false, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, false, false, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, false, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, false, true, true, true, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, true, false, false, true, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, true, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, false, true, true, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, false, false, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, true, false, false, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, true, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, true, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, true, true, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, true, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, true, true, true, true, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, true, true, true, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, false, true, false, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, false, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, false, true, true, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, true, true, false, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = WagnerCode20_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, false, false, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, false, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
