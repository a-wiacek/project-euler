#[macro_use] extern crate cached;
#[macro_use] extern crate itertools;
mod solutions {
    pub mod euler001;
    pub mod euler002;
    pub mod euler003;
    pub mod euler004;
    pub mod euler005;
    pub mod euler006;
    pub mod euler007;
    pub mod euler008;
    pub mod euler009;
    pub mod euler010;
    pub mod euler011;
    pub mod euler012;
    pub mod euler013;
    pub mod euler014;
    pub mod euler015;
    pub mod euler016;
    pub mod euler017;
    pub mod euler018;
    pub mod euler019;
    pub mod euler020;
    pub mod euler021;
    pub mod euler022;
    pub mod euler023;
    pub mod euler024;
    pub mod euler025;
    pub mod euler026;
    pub mod euler027;
    pub mod euler028;
    pub mod euler029;
    pub mod euler030;
    pub mod euler031;
    pub mod euler032;
    pub mod euler033;
    pub mod euler034;
    pub mod euler035;
    pub mod euler036;
    pub mod euler037;
    pub mod euler038;
    pub mod euler039;
    pub mod euler040;
    pub mod euler041;
    pub mod euler042;
    pub mod euler043;
    pub mod euler044;
    pub mod euler045;
    pub mod euler046;
    pub mod euler047;
    pub mod euler048;
    pub mod euler049;
    pub mod euler050;
    pub mod euler051;
    pub mod euler052;
    pub mod euler053;
    pub mod euler054;
    pub mod euler055;
    pub mod euler056;
    pub mod euler057;
    pub mod euler058;
    pub mod euler059;
    pub mod euler060;
    pub mod euler061;
    pub mod euler062;
    pub mod euler063;
    pub mod euler064;
    pub mod euler065;
    pub mod euler066;
    pub mod euler067;
    pub mod euler068;
    pub mod euler069;
    pub mod euler070;
    pub mod euler071;
    pub mod euler072;
    pub mod euler073;
    pub mod euler074;
    pub mod euler075;
    pub mod euler076;
    pub mod euler077;
    pub mod euler078;
    pub mod euler079;
    pub mod euler080;
    pub mod euler081;
    pub mod euler082;
    pub mod euler083;
    pub mod euler084;
    pub mod euler085;
    pub mod euler086;
    pub mod euler087;
    pub mod euler088;
    pub mod euler089;
    pub mod euler090;
    pub mod euler091;
    pub mod euler092;
    pub mod euler093;
    pub mod euler094;
    pub mod euler095;
    pub mod euler096;
    pub mod euler097;
    pub mod euler098;
    pub mod euler099;
    pub mod euler100;
    pub mod euler101;
    pub mod euler102;
    pub mod euler103;
    pub mod euler104;
    pub mod euler105;
    pub mod euler106;
    pub mod euler107;
    pub mod euler108;
    pub mod euler109;
    pub mod euler110;
    pub mod euler111;
    pub mod euler112;
    pub mod euler113;
    pub mod euler114;
    pub mod euler115;
    pub mod euler116;
    pub mod euler117;
    pub mod euler118;
    pub mod euler119;
    pub mod euler120;
    pub mod euler121;
    pub mod euler122;
    pub mod euler123;
    pub mod euler124;
    pub mod euler125;
    pub mod euler126;
    pub mod euler127;
    pub mod euler129;
    pub mod euler130;
    pub mod euler131;
    pub mod euler132;
    pub mod euler134;
    pub mod euler135;
    pub mod euler136;
    pub mod euler137;
    pub mod euler138;
    pub mod euler139;
    pub mod euler140;
    pub mod euler142;
    pub mod euler145;
    pub mod euler146;
    pub mod euler149;
    pub mod euler155;
    pub mod euler156;
    pub mod euler157;
    pub mod euler159;
    pub mod euler160;
    pub mod euler161;
    pub mod euler162;
    pub mod euler163;
    pub mod euler164;
    pub mod euler165;
    pub mod euler166;
    pub mod euler168;
    pub mod euler169;
    pub mod euler171;
    pub mod euler172;
    pub mod euler173;
    pub mod euler174;
    pub mod euler178;
    pub mod euler179;
    pub mod euler180;
    pub mod euler181;
    pub mod euler182;
    pub mod euler183;
    pub mod euler186;
    pub mod euler187;
    pub mod euler188;
    pub mod euler189;
    pub mod euler190;
    pub mod euler191;
    pub mod euler192;
    pub mod euler193;
    pub mod euler196;
    pub mod euler197;
    pub mod euler201;
    pub mod euler202;
    pub mod euler203;
    pub mod euler204;
    pub mod euler205;
    pub mod euler206;
    pub mod euler207;
    pub mod euler211;
    pub mod euler214;
    pub mod euler215;
    pub mod euler216;
    pub mod euler217;
    pub mod euler218;
    pub mod euler221;
    pub mod euler225;
    pub mod euler231;
    pub mod euler233;
    pub mod euler235;
    pub mod euler237;
    pub mod euler243;
    pub mod euler244;
    pub mod euler245;
    pub mod euler246;
    pub mod euler247;
    pub mod euler260;
    pub mod euler265;
    pub mod euler267;
    pub mod euler271;
    pub mod euler272;
    pub mod euler274;
    pub mod euler277;
    pub mod euler278;
    pub mod euler282;
    pub mod euler285;
    pub mod euler286;
    pub mod euler287;
    pub mod euler291;
    pub mod euler301;
    pub mod euler303;
    pub mod euler304;
    pub mod euler306;
    pub mod euler307;
    pub mod euler312;
    pub mod euler313;
    pub mod euler315;
    pub mod euler320;
    pub mod euler321;
    pub mod euler323;
    pub mod euler324;
    pub mod euler335;
    pub mod euler336;
    pub mod euler342;
    pub mod euler343;
    pub mod euler345;
    pub mod euler346;
    pub mod euler347;
    pub mod euler348;
    pub mod euler349;
    pub mod euler353;
    pub mod euler357;
    pub mod euler358;
    pub mod euler365;
    pub mod euler377;
    pub mod euler381;
    pub mod euler393;
    pub mod euler475;
    pub mod euler493;
    pub mod euler504;
    pub mod euler510;
    pub mod euler512;
    pub mod euler531;
    pub mod euler540;
    pub mod euler571;
    pub mod euler587;
    pub mod euler601;
    pub mod euler610;
    pub mod euler613;
    pub mod euler618;
    pub mod euler622;
    pub mod euler623;
    pub mod euler643;
    pub mod euler650;
    pub mod euler662;
    pub mod euler668;
    pub mod euler684;
    pub mod euler686;
}
mod utils {
    pub mod input;
    pub mod numeric {
        pub mod digits;
        pub mod fibonacci;
        pub mod polygonal {
            pub mod triangular;
            pub mod square;
            pub mod pentagonal;
            pub mod hexagonal;
            pub mod heptagonal;
            pub mod octogonal;
        }
    }
    pub mod number_theory;
}
use std::time::Instant;

fn main() {
    let mut number = String::new();
    println!("Which problem do you want to run?");
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let fun = match number.trim().parse() {
        Ok(1) => solutions::euler001::euler001,
        Ok(2) => solutions::euler002::euler002,
        Ok(3) => solutions::euler003::euler003,
        Ok(4) => solutions::euler004::euler004,
        Ok(5) => solutions::euler005::euler005,
        Ok(6) => solutions::euler006::euler006,
        Ok(7) => solutions::euler007::euler007,
        Ok(8) => solutions::euler008::euler008,
        Ok(9) => solutions::euler009::euler009,
        Ok(10) => solutions::euler010::euler010,
        Ok(11) => solutions::euler011::euler011,
        Ok(12) => solutions::euler012::euler012,
        Ok(13) => solutions::euler013::euler013,
        Ok(14) => solutions::euler014::euler014,
        Ok(15) => solutions::euler015::euler015,
        Ok(16) => solutions::euler016::euler016,
        Ok(17) => solutions::euler017::euler017,
        Ok(18) => solutions::euler018::euler018,
        Ok(19) => solutions::euler019::euler019,
        Ok(20) => solutions::euler020::euler020,
        Ok(21) => solutions::euler021::euler021,
        Ok(22) => solutions::euler022::euler022,
        Ok(23) => solutions::euler023::euler023,
        Ok(24) => solutions::euler024::euler024,
        Ok(25) => solutions::euler025::euler025,
        Ok(26) => solutions::euler026::euler026,
        Ok(27) => solutions::euler027::euler027,
        Ok(28) => solutions::euler028::euler028,
        Ok(29) => solutions::euler029::euler029,
        Ok(30) => solutions::euler030::euler030,
        Ok(31) => solutions::euler031::euler031,
        Ok(32) => solutions::euler032::euler032,
        Ok(33) => solutions::euler033::euler033,
        Ok(34) => solutions::euler034::euler034,
        Ok(35) => solutions::euler035::euler035,
        Ok(36) => solutions::euler036::euler036,
        Ok(37) => solutions::euler037::euler037,
        Ok(38) => solutions::euler038::euler038,
        Ok(39) => solutions::euler039::euler039,
        Ok(40) => solutions::euler040::euler040,
        Ok(41) => solutions::euler041::euler041,
        Ok(42) => solutions::euler042::euler042,
        Ok(43) => solutions::euler043::euler043,
        Ok(44) => solutions::euler044::euler044,
        Ok(45) => solutions::euler045::euler045,
        Ok(46) => solutions::euler046::euler046,
        Ok(47) => solutions::euler047::euler047,
        Ok(48) => solutions::euler048::euler048,
        Ok(49) => solutions::euler049::euler049,
        Ok(50) => solutions::euler050::euler050,
        Ok(51) => solutions::euler051::euler051,
        Ok(52) => solutions::euler052::euler052,
        Ok(53) => solutions::euler053::euler053,
        Ok(54) => solutions::euler054::euler054,
        Ok(55) => solutions::euler055::euler055,
        Ok(56) => solutions::euler056::euler056,
        Ok(57) => solutions::euler057::euler057,
        Ok(58) => solutions::euler058::euler058,
        Ok(59) => solutions::euler059::euler059,
        Ok(60) => solutions::euler060::euler060,
        Ok(61) => solutions::euler061::euler061,
        Ok(62) => solutions::euler062::euler062,
        Ok(63) => solutions::euler063::euler063,
        Ok(64) => solutions::euler064::euler064,
        Ok(65) => solutions::euler065::euler065,
        Ok(66) => solutions::euler066::euler066,
        Ok(67) => solutions::euler067::euler067,
        Ok(68) => solutions::euler068::euler068,
        Ok(69) => solutions::euler069::euler069,
        Ok(70) => solutions::euler070::euler070,
        Ok(71) => solutions::euler071::euler071,
        Ok(72) => solutions::euler072::euler072,
        Ok(73) => solutions::euler073::euler073,
        Ok(74) => solutions::euler074::euler074,
        Ok(75) => solutions::euler075::euler075,
        Ok(76) => solutions::euler076::euler076,
        Ok(77) => solutions::euler077::euler077,
        Ok(78) => solutions::euler078::euler078,
        Ok(79) => solutions::euler079::euler079,
        Ok(80) => solutions::euler080::euler080,
        Ok(81) => solutions::euler081::euler081,
        Ok(82) => solutions::euler082::euler082,
        Ok(83) => solutions::euler083::euler083,
        Ok(84) => solutions::euler084::euler084,
        Ok(85) => solutions::euler085::euler085,
        Ok(86) => solutions::euler086::euler086,
        Ok(87) => solutions::euler087::euler087,
        Ok(88) => solutions::euler088::euler088,
        Ok(89) => solutions::euler089::euler089,
        Ok(90) => solutions::euler090::euler090,
        Ok(91) => solutions::euler091::euler091,
        Ok(92) => solutions::euler092::euler092,
        Ok(93) => solutions::euler093::euler093,
        Ok(94) => solutions::euler094::euler094,
        Ok(95) => solutions::euler095::euler095,
        Ok(96) => solutions::euler096::euler096,
        Ok(97) => solutions::euler097::euler097,
        Ok(98) => solutions::euler098::euler098,
        Ok(99) => solutions::euler099::euler099,
        Ok(100) => solutions::euler100::euler100,
        Ok(101) => solutions::euler101::euler101,
        Ok(102) => solutions::euler102::euler102,
        Ok(103) => solutions::euler103::euler103,
        Ok(104) => solutions::euler104::euler104,
        Ok(105) => solutions::euler105::euler105,
        Ok(106) => solutions::euler106::euler106,
        Ok(107) => solutions::euler107::euler107,
        Ok(108) => solutions::euler108::euler108,
        Ok(109) => solutions::euler109::euler109,
        Ok(110) => solutions::euler110::euler110,
        Ok(111) => solutions::euler111::euler111,
        Ok(112) => solutions::euler112::euler112,
        Ok(113) => solutions::euler113::euler113,
        Ok(114) => solutions::euler114::euler114,
        Ok(115) => solutions::euler115::euler115,
        Ok(116) => solutions::euler116::euler116,
        Ok(117) => solutions::euler117::euler117,
        Ok(118) => solutions::euler118::euler118,
        Ok(119) => solutions::euler119::euler119,
        Ok(120) => solutions::euler120::euler120,
        Ok(121) => solutions::euler121::euler121,
        Ok(122) => solutions::euler122::euler122,
        Ok(123) => solutions::euler123::euler123,
        Ok(124) => solutions::euler124::euler124,
        Ok(125) => solutions::euler125::euler125,
        Ok(126) => solutions::euler126::euler126,
        Ok(127) => solutions::euler127::euler127,
        Ok(129) => solutions::euler129::euler129,
        Ok(130) => solutions::euler130::euler130,
        Ok(131) => solutions::euler131::euler131,
        Ok(132) => solutions::euler132::euler132,
        Ok(134) => solutions::euler134::euler134,
        Ok(135) => solutions::euler135::euler135,
        Ok(136) => solutions::euler136::euler136,
        Ok(137) => solutions::euler137::euler137,
        Ok(138) => solutions::euler138::euler138,
        Ok(139) => solutions::euler139::euler139,
        Ok(140) => solutions::euler140::euler140,
        Ok(142) => solutions::euler142::euler142,
        Ok(145) => solutions::euler145::euler145,
        Ok(146) => solutions::euler146::euler146,
        Ok(149) => solutions::euler149::euler149,
        Ok(155) => solutions::euler155::euler155,
        Ok(156) => solutions::euler156::euler156,
        Ok(157) => solutions::euler157::euler157,
        Ok(159) => solutions::euler159::euler159,
        Ok(160) => solutions::euler160::euler160,
        Ok(161) => solutions::euler161::euler161,
        Ok(162) => solutions::euler162::euler162,
        Ok(163) => solutions::euler163::euler163,
        Ok(164) => solutions::euler164::euler164,
        Ok(165) => solutions::euler165::euler165,
        Ok(166) => solutions::euler166::euler166,
        Ok(168) => solutions::euler168::euler168,
        Ok(169) => solutions::euler169::euler169,
        Ok(171) => solutions::euler171::euler171,
        Ok(172) => solutions::euler172::euler172,
        Ok(173) => solutions::euler173::euler173,
        Ok(174) => solutions::euler174::euler174,
        Ok(178) => solutions::euler178::euler178,
        Ok(179) => solutions::euler179::euler179,
        Ok(180) => solutions::euler180::euler180,
        Ok(181) => solutions::euler181::euler181,
        Ok(182) => solutions::euler182::euler182,
        Ok(183) => solutions::euler183::euler183,
        Ok(186) => solutions::euler186::euler186,
        Ok(187) => solutions::euler187::euler187,
        Ok(188) => solutions::euler188::euler188,
        Ok(189) => solutions::euler189::euler189,
        Ok(190) => solutions::euler190::euler190,
        Ok(191) => solutions::euler191::euler191,
        Ok(192) => solutions::euler192::euler192,
        Ok(193) => solutions::euler193::euler193,
        Ok(196) => solutions::euler196::euler196,
        Ok(197) => solutions::euler197::euler197,
        Ok(201) => solutions::euler201::euler201,
        Ok(202) => solutions::euler202::euler202,
        Ok(203) => solutions::euler203::euler203,
        Ok(204) => solutions::euler204::euler204,
        Ok(205) => solutions::euler205::euler205,
        Ok(206) => solutions::euler206::euler206,
        Ok(207) => solutions::euler207::euler207,
        Ok(211) => solutions::euler211::euler211,
        Ok(214) => solutions::euler214::euler214,
        Ok(215) => solutions::euler215::euler215,
        Ok(216) => solutions::euler216::euler216,
        Ok(217) => solutions::euler217::euler217,
        Ok(218) => solutions::euler218::euler218,
        Ok(221) => solutions::euler221::euler221,
        Ok(225) => solutions::euler225::euler225,
        Ok(231) => solutions::euler231::euler231,
        Ok(233) => solutions::euler233::euler233,
        Ok(235) => solutions::euler235::euler235,
        Ok(237) => solutions::euler237::euler237,
        Ok(243) => solutions::euler243::euler243,
        Ok(244) => solutions::euler244::euler244,
        Ok(245) => solutions::euler245::euler245,
        Ok(246) => solutions::euler246::euler246,
        Ok(247) => solutions::euler247::euler247,
        Ok(260) => solutions::euler260::euler260,
        Ok(265) => solutions::euler265::euler265,
        Ok(267) => solutions::euler267::euler267,
        Ok(271) => solutions::euler271::euler271,
        Ok(272) => solutions::euler272::euler272,
        Ok(274) => solutions::euler274::euler274,
        Ok(277) => solutions::euler277::euler277,
        Ok(278) => solutions::euler278::euler278,
        Ok(282) => solutions::euler282::euler282,
        Ok(285) => solutions::euler285::euler285,
        Ok(286) => solutions::euler286::euler286,
        Ok(287) => solutions::euler287::euler287,
        Ok(291) => solutions::euler291::euler291,
        Ok(301) => solutions::euler301::euler301,
        Ok(303) => solutions::euler303::euler303,
        Ok(304) => solutions::euler304::euler304,
        Ok(306) => solutions::euler306::euler306,
        Ok(307) => solutions::euler307::euler307,
        Ok(312) => solutions::euler312::euler312,
        Ok(313) => solutions::euler313::euler313,
        Ok(315) => solutions::euler315::euler315,
        Ok(320) => solutions::euler320::euler320,
        Ok(321) => solutions::euler321::euler321,
        Ok(323) => solutions::euler323::euler323,
        Ok(324) => solutions::euler324::euler324,
        Ok(335) => solutions::euler335::euler335,
        Ok(336) => solutions::euler336::euler336,
        Ok(342) => solutions::euler342::euler342,
        Ok(343) => solutions::euler343::euler343,
        Ok(345) => solutions::euler345::euler345,
        Ok(346) => solutions::euler346::euler346,
        Ok(347) => solutions::euler347::euler347,
        Ok(348) => solutions::euler348::euler348,
        Ok(349) => solutions::euler349::euler349,
        Ok(353) => solutions::euler353::euler353,
        Ok(357) => solutions::euler357::euler357,
        Ok(358) => solutions::euler358::euler358,
        Ok(365) => solutions::euler365::euler365,
        Ok(377) => solutions::euler377::euler377,
        Ok(381) => solutions::euler381::euler381,
        Ok(393) => solutions::euler393::euler393,
        Ok(475) => solutions::euler475::euler475,
        Ok(493) => solutions::euler493::euler493,
        Ok(504) => solutions::euler504::euler504,
        Ok(510) => solutions::euler510::euler510,
        Ok(512) => solutions::euler512::euler512,
        Ok(531) => solutions::euler531::euler531,
        Ok(540) => solutions::euler540::euler540,
        Ok(571) => solutions::euler571::euler571,
        Ok(587) => solutions::euler587::euler587,
        Ok(601) => solutions::euler601::euler601,
        Ok(610) => solutions::euler610::euler610,
        Ok(613) => solutions::euler613::euler613,
        Ok(618) => solutions::euler618::euler618,
        Ok(622) => solutions::euler622::euler622,
        Ok(623) => solutions::euler623::euler623,
        Ok(643) => solutions::euler643::euler643,
        Ok(650) => solutions::euler650::euler650,
        Ok(662) => solutions::euler662::euler662,
        Ok(668) => solutions::euler668::euler668,
        Ok(684) => solutions::euler684::euler684,
        Ok(686) => solutions::euler686::euler686,
        Ok(num) => panic!("Solution for problem {} does not exist yet!", num),
        Err(_) => panic!("Failed to read a number"),
    };
    let now = Instant::now();
    println!("Output: {}", fun());
    println!("Execution time: {}s", now.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn read_answer(line: usize) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("txt");
        path.push("answers.txt");
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .nth(line - 1)
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_euler001() {
        assert_eq!(crate::solutions::euler001::euler001(), read_answer(1));
    }

    #[test]
    fn test_euler002() {
        assert_eq!(crate::solutions::euler002::euler002(), read_answer(2));
    }

    #[test]
    fn test_euler003() {
        assert_eq!(crate::solutions::euler003::euler003(), read_answer(3));
    }

    #[test]
    fn test_euler004() {
        assert_eq!(crate::solutions::euler004::euler004(), read_answer(4));
    }

    #[test]
    fn test_euler005() {
        assert_eq!(crate::solutions::euler005::euler005(), read_answer(5));
    }

    #[test]
    fn test_euler006() {
        assert_eq!(crate::solutions::euler006::euler006(), read_answer(6));
    }

    #[test]
    fn test_euler007() {
        assert_eq!(crate::solutions::euler007::euler007(), read_answer(7));
    }

    #[test]
    fn test_euler008() {
        assert_eq!(crate::solutions::euler008::euler008(), read_answer(8));
    }

    #[test]
    fn test_euler009() {
        assert_eq!(crate::solutions::euler009::euler009(), read_answer(9));
    }

    #[test]
    fn test_euler010() {
        assert_eq!(crate::solutions::euler010::euler010(), read_answer(10));
    }

    #[test]
    fn test_euler011() {
        assert_eq!(crate::solutions::euler011::euler011(), read_answer(11));
    }

    #[test]
    fn test_euler012() {
        assert_eq!(crate::solutions::euler012::euler012(), read_answer(12));
    }

    #[test]
    fn test_euler013() {
        assert_eq!(crate::solutions::euler013::euler013(), read_answer(13));
    }

    #[test]
    fn test_euler014() {
        assert_eq!(crate::solutions::euler014::euler014(), read_answer(14));
    }

    #[test]
    fn test_euler015() {
        assert_eq!(crate::solutions::euler015::euler015(), read_answer(15));
    }

    #[test]
    fn test_euler016() {
        assert_eq!(crate::solutions::euler016::euler016(), read_answer(16));
    }

    #[test]
    fn test_euler017() {
        assert_eq!(crate::solutions::euler017::euler017(), read_answer(17));
    }

    #[test]
    fn test_euler018() {
        assert_eq!(crate::solutions::euler018::euler018(), read_answer(18));
    }

    #[test]
    fn test_euler019() {
        assert_eq!(crate::solutions::euler019::euler019(), read_answer(19));
    }

    #[test]
    fn test_euler020() {
        assert_eq!(crate::solutions::euler020::euler020(), read_answer(20));
    }

    #[test]
    fn test_euler021() {
        assert_eq!(crate::solutions::euler021::euler021(), read_answer(21));
    }

    #[test]
    fn test_euler022() {
        assert_eq!(crate::solutions::euler022::euler022(), read_answer(22));
    }

    #[test]
    fn test_euler023() {
        assert_eq!(crate::solutions::euler023::euler023(), read_answer(23));
    }

    #[test]
    fn test_euler024() {
        assert_eq!(crate::solutions::euler024::euler024(), read_answer(24));
    }

    #[test]
    fn test_euler025() {
        assert_eq!(crate::solutions::euler025::euler025(), read_answer(25));
    }

    #[test]
    fn test_euler026() {
        assert_eq!(crate::solutions::euler026::euler026(), read_answer(26));
    }

    #[test]
    fn test_euler027() {
        assert_eq!(crate::solutions::euler027::euler027(), read_answer(27));
    }

    #[test]
    fn test_euler028() {
        assert_eq!(crate::solutions::euler028::euler028(), read_answer(28));
    }

    #[test]
    fn test_euler029() {
        assert_eq!(crate::solutions::euler029::euler029(), read_answer(29));
    }

    #[test]
    fn test_euler030() {
        assert_eq!(crate::solutions::euler030::euler030(), read_answer(30));
    }

    #[test]
    fn test_euler031() {
        assert_eq!(crate::solutions::euler031::euler031(), read_answer(31));
    }

    #[test]
    fn test_euler032() {
        assert_eq!(crate::solutions::euler032::euler032(), read_answer(32));
    }

    #[test]
    fn test_euler033() {
        assert_eq!(crate::solutions::euler033::euler033(), read_answer(33));
    }

    #[test]
    fn test_euler034() {
        assert_eq!(crate::solutions::euler034::euler034(), read_answer(34));
    }

    #[test]
    fn test_euler035() {
        assert_eq!(crate::solutions::euler035::euler035(), read_answer(35));
    }

    #[test]
    fn test_euler036() {
        assert_eq!(crate::solutions::euler036::euler036(), read_answer(36));
    }

    #[test]
    fn test_euler037() {
        assert_eq!(crate::solutions::euler037::euler037(), read_answer(37));
    }

    #[test]
    fn test_euler038() {
        assert_eq!(crate::solutions::euler038::euler038(), read_answer(38));
    }

    #[test]
    fn test_euler039() {
        assert_eq!(crate::solutions::euler039::euler039(), read_answer(39));
    }

    #[test]
    fn test_euler040() {
        assert_eq!(crate::solutions::euler040::euler040(), read_answer(40));
    }

    #[test]
    fn test_euler041() {
        assert_eq!(crate::solutions::euler041::euler041(), read_answer(41));
    }

    #[test]
    fn test_euler042() {
        assert_eq!(crate::solutions::euler042::euler042(), read_answer(42));
    }

    #[test]
    fn test_euler043() {
        assert_eq!(crate::solutions::euler043::euler043(), read_answer(43));
    }

    #[test]
    fn test_euler044() {
        assert_eq!(crate::solutions::euler044::euler044(), read_answer(44));
    }

    #[test]
    fn test_euler045() {
        assert_eq!(crate::solutions::euler045::euler045(), read_answer(45));
    }

    #[test]
    fn test_euler046() {
        assert_eq!(crate::solutions::euler046::euler046(), read_answer(46));
    }

    #[test]
    fn test_euler047() {
        assert_eq!(crate::solutions::euler047::euler047(), read_answer(47));
    }

    #[test]
    fn test_euler048() {
        assert_eq!(crate::solutions::euler048::euler048(), read_answer(48));
    }

    #[test]
    fn test_euler049() {
        assert_eq!(crate::solutions::euler049::euler049(), read_answer(49));
    }

    #[test]
    fn test_euler050() {
        assert_eq!(crate::solutions::euler050::euler050(), read_answer(50));
    }

    #[test]
    fn test_euler051() {
        assert_eq!(crate::solutions::euler051::euler051(), read_answer(51));
    }

    #[test]
    fn test_euler052() {
        assert_eq!(crate::solutions::euler052::euler052(), read_answer(52));
    }

    #[test]
    fn test_euler053() {
        assert_eq!(crate::solutions::euler053::euler053(), read_answer(53));
    }

    #[test]
    fn test_euler054() {
        assert_eq!(crate::solutions::euler054::euler054(), read_answer(54));
    }

    #[test]
    fn test_euler055() {
        assert_eq!(crate::solutions::euler055::euler055(), read_answer(55));
    }

    #[test]
    fn test_euler056() {
        assert_eq!(crate::solutions::euler056::euler056(), read_answer(56));
    }

    #[test]
    fn test_euler057() {
        assert_eq!(crate::solutions::euler057::euler057(), read_answer(57));
    }

    #[test]
    fn test_euler058() {
        assert_eq!(crate::solutions::euler058::euler058(), read_answer(58));
    }

    #[test]
    fn test_euler059() {
        assert_eq!(crate::solutions::euler059::euler059(), read_answer(59));
    }

    #[test]
    fn test_euler060() {
        assert_eq!(crate::solutions::euler060::euler060(), read_answer(60));
    }

    #[test]
    fn test_euler061() {
        assert_eq!(crate::solutions::euler061::euler061(), read_answer(61));
    }

    #[test]
    fn test_euler062() {
        assert_eq!(crate::solutions::euler062::euler062(), read_answer(62));
    }

    #[test]
    fn test_euler063() {
        assert_eq!(crate::solutions::euler063::euler063(), read_answer(63));
    }

    #[test]
    fn test_euler064() {
        assert_eq!(crate::solutions::euler064::euler064(), read_answer(64));
    }

    #[test]
    fn test_euler065() {
        assert_eq!(crate::solutions::euler065::euler065(), read_answer(65));
    }

    #[test]
    fn test_euler066() {
        assert_eq!(crate::solutions::euler066::euler066(), read_answer(66));
    }

    #[test]
    fn test_euler067() {
        assert_eq!(crate::solutions::euler067::euler067(), read_answer(67));
    }

    #[test]
    fn test_euler068() {
        assert_eq!(crate::solutions::euler068::euler068(), read_answer(68));
    }

    #[test]
    fn test_euler069() {
        assert_eq!(crate::solutions::euler069::euler069(), read_answer(69));
    }

    #[test]
    fn test_euler070() {
        assert_eq!(crate::solutions::euler070::euler070(), read_answer(70));
    }

    #[test]
    fn test_euler071() {
        assert_eq!(crate::solutions::euler071::euler071(), read_answer(71));
    }

    #[test]
    fn test_euler072() {
        assert_eq!(crate::solutions::euler072::euler072(), read_answer(72));
    }

    #[test]
    fn test_euler073() {
        assert_eq!(crate::solutions::euler073::euler073(), read_answer(73));
    }

    #[test]
    fn test_euler074() {
        assert_eq!(crate::solutions::euler074::euler074(), read_answer(74));
    }

    #[test]
    fn test_euler075() {
        assert_eq!(crate::solutions::euler075::euler075(), read_answer(75));
    }

    #[test]
    fn test_euler076() {
        assert_eq!(crate::solutions::euler076::euler076(), read_answer(76));
    }

    #[test]
    fn test_euler077() {
        assert_eq!(crate::solutions::euler077::euler077(), read_answer(77));
    }

    #[test]
    fn test_euler078() {
        assert_eq!(crate::solutions::euler078::euler078(), read_answer(78));
    }

    #[test]
    fn test_euler079() {
        assert_eq!(crate::solutions::euler079::euler079(), read_answer(79));
    }

    #[test]
    fn test_euler080() {
        assert_eq!(crate::solutions::euler080::euler080(), read_answer(80));
    }

    #[test]
    fn test_euler081() {
        assert_eq!(crate::solutions::euler081::euler081(), read_answer(81));
    }

    #[test]
    fn test_euler082() {
        assert_eq!(crate::solutions::euler082::euler082(), read_answer(82));
    }

    #[test]
    fn test_euler083() {
        assert_eq!(crate::solutions::euler083::euler083(), read_answer(83));
    }

    #[test]
    fn test_euler084() {
        assert_eq!(crate::solutions::euler084::euler084(), read_answer(84));
    }

    #[test]
    fn test_euler085() {
        assert_eq!(crate::solutions::euler085::euler085(), read_answer(85));
    }

    #[test]
    fn test_euler086() {
        assert_eq!(crate::solutions::euler086::euler086(), read_answer(86));
    }

    #[test]
    fn test_euler087() {
        assert_eq!(crate::solutions::euler087::euler087(), read_answer(87));
    }

    #[test]
    fn test_euler088() {
        assert_eq!(crate::solutions::euler088::euler088(), read_answer(88));
    }

    #[test]
    fn test_euler089() {
        assert_eq!(crate::solutions::euler089::euler089(), read_answer(89));
    }

    #[test]
    fn test_euler090() {
        assert_eq!(crate::solutions::euler090::euler090(), read_answer(90));
    }

    #[test]
    fn test_euler091() {
        assert_eq!(crate::solutions::euler091::euler091(), read_answer(91));
    }

    #[test]
    fn test_euler092() {
        assert_eq!(crate::solutions::euler092::euler092(), read_answer(92));
    }

    #[test]
    fn test_euler093() {
        assert_eq!(crate::solutions::euler093::euler093(), read_answer(93));
    }

    #[test]
    fn test_euler094() {
        assert_eq!(crate::solutions::euler094::euler094(), read_answer(94));
    }

    #[test]
    fn test_euler095() {
        assert_eq!(crate::solutions::euler095::euler095(), read_answer(95));
    }

    #[test]
    fn test_euler096() {
        assert_eq!(crate::solutions::euler096::euler096(), read_answer(96));
    }

    #[test]
    fn test_euler097() {
        assert_eq!(crate::solutions::euler097::euler097(), read_answer(97));
    }

    #[test]
    fn test_euler098() {
        assert_eq!(crate::solutions::euler098::euler098(), read_answer(98));
    }

    #[test]
    fn test_euler099() {
        assert_eq!(crate::solutions::euler099::euler099(), read_answer(99));
    }

    #[test]
    fn test_euler100() {
        assert_eq!(crate::solutions::euler100::euler100(), read_answer(100));
    }

    #[test]
    fn test_euler101() {
        assert_eq!(crate::solutions::euler101::euler101(), read_answer(101));
    }

    #[test]
    fn test_euler102() {
        assert_eq!(crate::solutions::euler102::euler102(), read_answer(102));
    }

    #[test]
    fn test_euler103() {
        assert_eq!(crate::solutions::euler103::euler103(), read_answer(103));
    }

    #[test]
    fn test_euler104() {
        assert_eq!(crate::solutions::euler104::euler104(), read_answer(104));
    }

    #[test]
    fn test_euler105() {
        assert_eq!(crate::solutions::euler105::euler105(), read_answer(105));
    }

    #[test]
    fn test_euler106() {
        assert_eq!(crate::solutions::euler106::euler106(), read_answer(106));
    }

    #[test]
    fn test_euler107() {
        assert_eq!(crate::solutions::euler107::euler107(), read_answer(107));
    }

    #[test]
    fn test_euler108() {
        assert_eq!(crate::solutions::euler108::euler108(), read_answer(108));
    }

    #[test]
    fn test_euler109() {
        assert_eq!(crate::solutions::euler109::euler109(), read_answer(109));
    }

    #[test]
    fn test_euler110() {
        assert_eq!(crate::solutions::euler110::euler110(), read_answer(110));
    }

    #[test]
    fn test_euler111() {
        assert_eq!(crate::solutions::euler111::euler111(), read_answer(111));
    }

    #[test]
    fn test_euler112() {
        assert_eq!(crate::solutions::euler112::euler112(), read_answer(112));
    }

    #[test]
    fn test_euler113() {
        assert_eq!(crate::solutions::euler113::euler113(), read_answer(113));
    }

    #[test]
    fn test_euler114() {
        assert_eq!(crate::solutions::euler114::euler114(), read_answer(114));
    }

    #[test]
    fn test_euler115() {
        assert_eq!(crate::solutions::euler115::euler115(), read_answer(115));
    }

    #[test]
    fn test_euler116() {
        assert_eq!(crate::solutions::euler116::euler116(), read_answer(116));
    }

    #[test]
    fn test_euler117() {
        assert_eq!(crate::solutions::euler117::euler117(), read_answer(117));
    }

    #[test]
    fn test_euler118() {
        assert_eq!(crate::solutions::euler118::euler118(), read_answer(118));
    }

    #[test]
    fn test_euler119() {
        assert_eq!(crate::solutions::euler119::euler119(), read_answer(119));
    }

    #[test]
    fn test_euler120() {
        assert_eq!(crate::solutions::euler120::euler120(), read_answer(120));
    }

    #[test]
    fn test_euler121() {
        assert_eq!(crate::solutions::euler121::euler121(), read_answer(121));
    }

    #[test]
    fn test_euler122() {
        assert_eq!(crate::solutions::euler122::euler122(), read_answer(122));
    }

    #[test]
    fn test_euler123() {
        assert_eq!(crate::solutions::euler123::euler123(), read_answer(123));
    }

    #[test]
    fn test_euler124() {
        assert_eq!(crate::solutions::euler124::euler124(), read_answer(124));
    }

    #[test]
    fn test_euler125() {
        assert_eq!(crate::solutions::euler125::euler125(), read_answer(125));
    }

    #[test]
    fn test_euler126() {
        assert_eq!(crate::solutions::euler126::euler126(), read_answer(126));
    }

    #[test]
    fn test_euler127() {
        assert_eq!(crate::solutions::euler127::euler127(), read_answer(127));
    }

    #[test]
    fn test_euler129() {
        assert_eq!(crate::solutions::euler129::euler129(), read_answer(129));
    }

    #[test]
    fn test_euler130() {
        assert_eq!(crate::solutions::euler130::euler130(), read_answer(130));
    }

    #[test]
    fn test_euler131() {
        assert_eq!(crate::solutions::euler131::euler131(), read_answer(131));
    }

    #[test]
    fn test_euler132() {
        assert_eq!(crate::solutions::euler132::euler132(), read_answer(132));
    }

    #[test]
    fn test_euler134() {
        assert_eq!(crate::solutions::euler134::euler134(), read_answer(134));
    }

    #[test]
    fn test_euler135() {
        assert_eq!(crate::solutions::euler135::euler135(), read_answer(135));
    }

    #[test]
    fn test_euler136() {
        assert_eq!(crate::solutions::euler136::euler136(), read_answer(136));
    }

    #[test]
    fn test_euler137() {
        assert_eq!(crate::solutions::euler137::euler137(), read_answer(137));
    }

    #[test]
    fn test_euler138() {
        assert_eq!(crate::solutions::euler138::euler138(), read_answer(138));
    }

    #[test]
    fn test_euler139() {
        assert_eq!(crate::solutions::euler139::euler139(), read_answer(139));
    }

    #[test]
    fn test_euler140() {
        assert_eq!(crate::solutions::euler140::euler140(), read_answer(140));
    }

    #[test]
    fn test_euler142() {
        assert_eq!(crate::solutions::euler142::euler142(), read_answer(142));
    }

    #[test]
    fn test_euler145() {
        assert_eq!(crate::solutions::euler145::euler145(), read_answer(145));
    }

    #[test]
    fn test_euler146() {
        assert_eq!(crate::solutions::euler146::euler146(), read_answer(146));
    }

    #[test]
    fn test_euler149() {
        assert_eq!(crate::solutions::euler149::euler149(), read_answer(149));
    }

    #[test]
    fn test_euler155() {
        assert_eq!(crate::solutions::euler155::euler155(), read_answer(155));
    }

    #[test]
    fn test_euler156() {
        assert_eq!(crate::solutions::euler156::euler156(), read_answer(156));
    }

    #[test]
    fn test_euler157() {
        assert_eq!(crate::solutions::euler157::euler157(), read_answer(157));
    }

    #[test]
    fn test_euler159() {
        assert_eq!(crate::solutions::euler159::euler159(), read_answer(159));
    }

    #[test]
    fn test_euler160() {
        assert_eq!(crate::solutions::euler160::euler160(), read_answer(160));
    }

    #[test]
    fn test_euler161() {
        assert_eq!(crate::solutions::euler161::euler161(), read_answer(161));
    }

    #[test]
    fn test_euler162() {
        assert_eq!(crate::solutions::euler162::euler162(), read_answer(162));
    }

    #[test]
    fn test_euler163() {
        assert_eq!(crate::solutions::euler163::euler163(), read_answer(163));
    }

    #[test]
    fn test_euler164() {
        assert_eq!(crate::solutions::euler164::euler164(), read_answer(164));
    }

    #[test]
    fn test_euler165() {
        assert_eq!(crate::solutions::euler165::euler165(), read_answer(165));
    }

    #[test]
    fn test_euler166() {
        assert_eq!(crate::solutions::euler166::euler166(), read_answer(166));
    }

    #[test]
    fn test_euler168() {
        assert_eq!(crate::solutions::euler168::euler168(), read_answer(168));
    }

    #[test]
    fn test_euler169() {
        assert_eq!(crate::solutions::euler169::euler169(), read_answer(169));
    }

    #[test]
    fn test_euler171() {
        assert_eq!(crate::solutions::euler171::euler171(), read_answer(171));
    }

    #[test]
    fn test_euler172() {
        assert_eq!(crate::solutions::euler172::euler172(), read_answer(172));
    }

    #[test]
    fn test_euler173() {
        assert_eq!(crate::solutions::euler173::euler173(), read_answer(173));
    }

    #[test]
    fn test_euler174() {
        assert_eq!(crate::solutions::euler174::euler174(), read_answer(174));
    }

    #[test]
    fn test_euler178() {
        assert_eq!(crate::solutions::euler178::euler178(), read_answer(178));
    }

    #[test]
    fn test_euler179() {
        assert_eq!(crate::solutions::euler179::euler179(), read_answer(179));
    }

    #[test]
    fn test_euler180() {
        assert_eq!(crate::solutions::euler180::euler180(), read_answer(180));
    }

    #[test]
    fn test_euler181() {
        assert_eq!(crate::solutions::euler181::euler181(), read_answer(181));
    }

    #[test]
    fn test_euler182() {
        assert_eq!(crate::solutions::euler182::euler182(), read_answer(182));
    }

    #[test]
    fn test_euler183() {
        assert_eq!(crate::solutions::euler183::euler183(), read_answer(183));
    }

    #[test]
    fn test_euler186() {
        assert_eq!(crate::solutions::euler186::euler186(), read_answer(186));
    }

    #[test]
    fn test_euler187() {
        assert_eq!(crate::solutions::euler187::euler187(), read_answer(187));
    }

    #[test]
    fn test_euler188() {
        assert_eq!(crate::solutions::euler188::euler188(), read_answer(188));
    }

    #[test]
    fn test_euler189() {
        assert_eq!(crate::solutions::euler189::euler189(), read_answer(189));
    }

    #[test]
    fn test_euler190() {
        assert_eq!(crate::solutions::euler190::euler190(), read_answer(190));
    }

    #[test]
    fn test_euler191() {
        assert_eq!(crate::solutions::euler191::euler191(), read_answer(191));
    }

    #[test]
    fn test_euler192() {
        assert_eq!(crate::solutions::euler192::euler192(), read_answer(192));
    }

    #[test]
    fn test_euler193() {
        assert_eq!(crate::solutions::euler193::euler193(), read_answer(193));
    }

    #[test]
    fn test_euler196() {
        assert_eq!(crate::solutions::euler196::euler196(), read_answer(196));
    }

    #[test]
    fn test_euler197() {
        assert_eq!(crate::solutions::euler197::euler197(), read_answer(197));
    }

    #[test]
    fn test_euler201() {
        assert_eq!(crate::solutions::euler201::euler201(), read_answer(201));
    }

    #[test]
    fn test_euler202() {
        assert_eq!(crate::solutions::euler202::euler202(), read_answer(202));
    }

    #[test]
    fn test_euler203() {
        assert_eq!(crate::solutions::euler203::euler203(), read_answer(203));
    }

    #[test]
    fn test_euler204() {
        assert_eq!(crate::solutions::euler204::euler204(), read_answer(204));
    }

    #[test]
    fn test_euler205() {
        assert_eq!(crate::solutions::euler205::euler205(), read_answer(205));
    }

    #[test]
    fn test_euler206() {
        assert_eq!(crate::solutions::euler206::euler206(), read_answer(206));
    }

    #[test]
    fn test_euler207() {
        assert_eq!(crate::solutions::euler207::euler207(), read_answer(207));
    }

    #[test]
    fn test_euler211() {
        assert_eq!(crate::solutions::euler211::euler211(), read_answer(211));
    }

    #[test]
    fn test_euler214() {
        assert_eq!(crate::solutions::euler214::euler214(), read_answer(214));
    }

    #[test]
    fn test_euler215() {
        assert_eq!(crate::solutions::euler215::euler215(), read_answer(215));
    }

    #[test]
    fn test_euler216() {
        assert_eq!(crate::solutions::euler216::euler216(), read_answer(216));
    }

    #[test]
    fn test_euler217() {
        assert_eq!(crate::solutions::euler217::euler217(), read_answer(217));
    }

    #[test]
    fn test_euler218() {
        assert_eq!(crate::solutions::euler218::euler218(), read_answer(218));
    }

    #[test]
    fn test_euler221() {
        assert_eq!(crate::solutions::euler221::euler221(), read_answer(221));
    }

    #[test]
    fn test_euler225() {
        assert_eq!(crate::solutions::euler225::euler225(), read_answer(225));
    }

    #[test]
    fn test_euler231() {
        assert_eq!(crate::solutions::euler231::euler231(), read_answer(231));
    }

    #[test]
    fn test_euler233() {
        assert_eq!(crate::solutions::euler233::euler233(), read_answer(233));
    }

    #[test]
    fn test_euler235() {
        assert_eq!(crate::solutions::euler235::euler235(), read_answer(235));
    }

    #[test]
    fn test_euler237() {
        assert_eq!(crate::solutions::euler237::euler237(), read_answer(237));
    }

    #[test]
    fn test_euler243() {
        assert_eq!(crate::solutions::euler243::euler243(), read_answer(243));
    }

    #[test]
    fn test_euler244() {
        assert_eq!(crate::solutions::euler244::euler244(), read_answer(244));
    }

    #[test]
    fn test_euler245() {
        assert_eq!(crate::solutions::euler245::euler245(), read_answer(245));
    }

    #[test]
    fn test_euler246() {
        assert_eq!(crate::solutions::euler246::euler246(), read_answer(246));
    }

    #[test]
    fn test_euler247() {
        assert_eq!(crate::solutions::euler247::euler247(), read_answer(247));
    }

    #[test]
    fn test_euler260() {
        assert_eq!(crate::solutions::euler260::euler260(), read_answer(260));
    }

    #[test]
    fn test_euler265() {
        assert_eq!(crate::solutions::euler265::euler265(), read_answer(265));
    }

    #[test]
    fn test_euler267() {
        assert_eq!(crate::solutions::euler267::euler267(), read_answer(267));
    }

    #[test]
    fn test_euler271() {
        assert_eq!(crate::solutions::euler271::euler271(), read_answer(271));
    }

    #[test]
    fn test_euler272() {
        assert_eq!(crate::solutions::euler272::euler272(), read_answer(272));
    }

    #[test]
    fn test_euler274() {
        assert_eq!(crate::solutions::euler274::euler274(), read_answer(274));
    }

    #[test]
    fn test_euler277() {
        assert_eq!(crate::solutions::euler277::euler277(), read_answer(277));
    }

    #[test]
    fn test_euler278() {
        assert_eq!(crate::solutions::euler278::euler278(), read_answer(278));
    }

    #[test]
    fn test_euler282() {
        assert_eq!(crate::solutions::euler282::euler282(), read_answer(282));
    }

    #[test]
    fn test_euler285() {
        assert_eq!(crate::solutions::euler285::euler285(), read_answer(285));
    }

    #[test]
    fn test_euler286() {
        assert_eq!(crate::solutions::euler286::euler286(), read_answer(286));
    }

    #[test]
    fn test_euler287() {
        assert_eq!(crate::solutions::euler287::euler287(), read_answer(287));
    }

    #[test]
    fn test_euler291() {
        assert_eq!(crate::solutions::euler291::euler291(), read_answer(291));
    }

    #[test]
    fn test_euler301() {
        assert_eq!(crate::solutions::euler301::euler301(), read_answer(301));
    }

    #[test]
    fn test_euler303() {
        assert_eq!(crate::solutions::euler303::euler303(), read_answer(303));
    }

    #[test]
    fn test_euler304() {
        assert_eq!(crate::solutions::euler304::euler304(), read_answer(304));
    }

    #[test]
    fn test_euler306() {
        assert_eq!(crate::solutions::euler306::euler306(), read_answer(306));
    }

    #[test]
    fn test_euler307() {
        assert_eq!(crate::solutions::euler307::euler307(), read_answer(307));
    }

    #[test]
    fn test_euler312() {
        assert_eq!(crate::solutions::euler312::euler312(), read_answer(312));
    }

    #[test]
    fn test_euler313() {
        assert_eq!(crate::solutions::euler313::euler313(), read_answer(313));
    }

    #[test]
    fn test_euler315() {
        assert_eq!(crate::solutions::euler315::euler315(), read_answer(315));
    }

    #[test]
    fn test_euler320() {
        assert_eq!(crate::solutions::euler320::euler320(), read_answer(320));
    }

    #[test]
    fn test_euler321() {
        assert_eq!(crate::solutions::euler321::euler321(), read_answer(321));
    }

    #[test]
    fn test_euler323() {
        assert_eq!(crate::solutions::euler323::euler323(), read_answer(323));
    }

    #[test]
    fn test_euler324() {
        assert_eq!(crate::solutions::euler324::euler324(), read_answer(324));
    }

    #[test]
    fn test_euler335() {
        assert_eq!(crate::solutions::euler335::euler335(), read_answer(335));
    }

    #[test]
    fn test_euler336() {
        assert_eq!(crate::solutions::euler336::euler336(), read_answer(336));
    }

    #[test]
    fn test_euler342() {
        assert_eq!(crate::solutions::euler342::euler342(), read_answer(342));
    }

    #[test]
    fn test_euler343() {
        assert_eq!(crate::solutions::euler343::euler343(), read_answer(343));
    }

    #[test]
    fn test_euler345() {
        assert_eq!(crate::solutions::euler345::euler345(), read_answer(345));
    }

    #[test]
    fn test_euler346() {
        assert_eq!(crate::solutions::euler346::euler346(), read_answer(346));
    }

    #[test]
    fn test_euler347() {
        assert_eq!(crate::solutions::euler347::euler347(), read_answer(347));
    }

    #[test]
    fn test_euler348() {
        assert_eq!(crate::solutions::euler348::euler348(), read_answer(348));
    }

    #[test]
    fn test_euler349() {
        assert_eq!(crate::solutions::euler349::euler349(), read_answer(349));
    }

    #[test]
    fn test_euler353() {
        assert_eq!(crate::solutions::euler353::euler353(), read_answer(353));
    }

    #[test]
    fn test_euler357() {
        assert_eq!(crate::solutions::euler357::euler357(), read_answer(357));
    }

    #[test]
    fn test_euler358() {
        assert_eq!(crate::solutions::euler358::euler358(), read_answer(358));
    }

    #[test]
    fn test_euler365() {
        assert_eq!(crate::solutions::euler365::euler365(), read_answer(365));
    }

    #[test]
    fn test_euler377() {
        assert_eq!(crate::solutions::euler377::euler377(), read_answer(377));
    }

    #[test]
    fn test_euler381() {
        assert_eq!(crate::solutions::euler381::euler381(), read_answer(381));
    }

    #[test]
    fn test_euler393() {
        assert_eq!(crate::solutions::euler393::euler393(), read_answer(393));
    }

    #[test]
    fn test_euler475() {
        assert_eq!(crate::solutions::euler475::euler475(), read_answer(475));
    }

    #[test]
    fn test_euler493() {
        assert_eq!(crate::solutions::euler493::euler493(), read_answer(493));
    }

    #[test]
    fn test_euler504() {
        assert_eq!(crate::solutions::euler504::euler504(), read_answer(504));
    }

    #[test]
    fn test_euler510() {
        assert_eq!(crate::solutions::euler510::euler510(), read_answer(510));
    }

    #[test]
    fn test_euler512() {
        assert_eq!(crate::solutions::euler512::euler512(), read_answer(512));
    }

    #[test]
    fn test_euler531() {
        assert_eq!(crate::solutions::euler531::euler531(), read_answer(531));
    }

    #[test]
    fn test_euler540() {
        assert_eq!(crate::solutions::euler540::euler540(), read_answer(540));
    }

    #[test]
    fn test_euler571() {
        assert_eq!(crate::solutions::euler571::euler571(), read_answer(571));
    }

    #[test]
    fn test_euler587() {
        assert_eq!(crate::solutions::euler587::euler587(), read_answer(587));
    }

    #[test]
    fn test_euler601() {
        assert_eq!(crate::solutions::euler601::euler601(), read_answer(601));
    }

    #[test]
    fn test_euler610() {
        assert_eq!(crate::solutions::euler610::euler610(), read_answer(610));
    }

    #[test]
    fn test_euler613() {
        assert_eq!(crate::solutions::euler613::euler613(), read_answer(613));
    }

    #[test]
    fn test_euler618() {
        assert_eq!(crate::solutions::euler618::euler618(), read_answer(618));
    }

    #[test]
    fn test_euler622() {
        assert_eq!(crate::solutions::euler622::euler622(), read_answer(622));
    }

    #[test]
    fn test_euler623() {
        assert_eq!(crate::solutions::euler623::euler623(), read_answer(623));
    }

    #[test]
    fn test_euler643() {
        assert_eq!(crate::solutions::euler643::euler643(), read_answer(643));
    }

    #[test]
    fn test_euler650() {
        assert_eq!(crate::solutions::euler650::euler650(), read_answer(650));
    }

    #[test]
    fn test_euler662() {
        assert_eq!(crate::solutions::euler662::euler662(), read_answer(662));
    }

    #[test]
    fn test_euler668() {
        assert_eq!(crate::solutions::euler668::euler668(), read_answer(668));
    }

    #[test]
    fn test_euler684() {
        assert_eq!(crate::solutions::euler684::euler684(), read_answer(684));
    }

    #[test]
    fn test_euler686() {
        assert_eq!(crate::solutions::euler686::euler686(), read_answer(686));
    }
}
