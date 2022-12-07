// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_fields::{
    FftParameters,
    FieldParameters,
    Fp384,
    Fp384Parameters,
    PoseidonDefaultParameters,
    PoseidonDefaultParametersEntry,
};
use snarkvm_utilities::biginteger::BigInteger384 as BigInteger;

pub type Fq = Fp384<FqParameters>;

pub struct FqParameters;

impl Fp384Parameters for FqParameters {}

impl FftParameters for FqParameters {
    type BigInteger = BigInteger;

    #[rustfmt::skip]
    const POWERS_OF_G: &'static [BigInteger] = &[
        BigInteger([
            17996873173959050988,
            10210379113943688432,
            16060314773336049477,
            12535769066843889595,
            7418605006406333375,
            22896867409027865,
        ]),
        BigInteger([
            5706777076162712822,
            3725681104238544220,
            16669217276640153949,
            17770581463204014376,
            11037690435855536924,
            80662696902059040,
        ]),
        BigInteger([
            9528136227366883686,
            15940151514509259071,
            6063110915911597874,
            14179142717672186233,
            11009738250486323604,
            47384157053466917,
        ]),
        BigInteger([
            10764188345449430697,
            7959010195604568991,
            10000525708797247270,
            4207891961224487566,
            9689648742923441244,
            48064277202018476,
        ]),
        BigInteger([
            12718552889982942512,
            18133511038630511291,
            14899912338113452343,
            15516296821141110345,
            15732557677504262050,
            78246883815774905,
        ]),
        BigInteger([
            6415234469133141773,
            17367339562015369897,
            11931395511947217086,
            5045941310177530580,
            13518143794790514339,
            103268657015304051,
        ]),
        BigInteger([
            14352682668999022623,
            7026894588891143467,
            7735505258231294796,
            17809089958441899662,
            628744051468303615,
            69005403723232687,
        ]),
        BigInteger([
            12548739151421501126,
            576036931610806170,
            2634872496679636265,
            12852640323373141654,
            154120939403914057,
            2543498364588928,
        ]),
        BigInteger([
            15209782064606934234,
            16260045614326083872,
            12475743211121229186,
            15226631780184314417,
            219303318093357991,
            67606664056234000,
        ]),
        BigInteger([
            3144011546166367568,
            5335531015802071517,
            17477147297521039448,
            15102339517158929594,
            17670914995559379802,
            66909698399507088,
        ]),
        BigInteger([
            3550783113700019973,
            16197040289510991633,
            18379891914226906859,
            12870362203812364864,
            7063077475028665074,
            5255766724106909,
        ]),
        BigInteger([
            15209039471587474684,
            10182386448638629601,
            2305369556811619699,
            5801105543234944954,
            14200696912666277920,
            118236130334317493,
        ]),
        BigInteger([
            2421945037639076031,
            9694215637625905909,
            7647615592616096976,
            15975344468382392081,
            14765275808657377129,
            61901179513140877,
        ]),
        BigInteger([
            15117481760361014838,
            5852162982714242562,
            12313841861598791729,
            11379604026846265327,
            1926514399373095840,
            58241922009765606,
        ]),
        BigInteger([
            17100837002903032891,
            16572698491878503845,
            7973706629080422690,
            3500897055639134319,
            1349379894634726138,
            22198304587490505,
        ]),
        BigInteger([
            13364154593739988305,
            17995796247536525512,
            6704893605488387216,
            2608690301491699834,
            17027610785716903953,
            18173159571226763,
        ]),
        BigInteger([
            11003039552925084804,
            10268821551105233413,
            12998845499775110052,
            635974232848055267,
            13053824521535540386,
            87343056544625574,
        ]),
        BigInteger([
            3707233261748073133,
            5287122061844540749,
            620146429869512639,
            1853188153552017963,
            9720229733035950481,
            41893275312623014,
        ]),
        BigInteger([
            793307488817134267,
            12620279237096559202,
            13523331246137599808,
            13721048246113684414,
            1798545244885345317,
            78928607107889172,
        ]),
        BigInteger([
            677224556530027880,
            9535881874043149441,
            11623947349677173530,
            13523496288103312411,
            15506613293958370102,
            45535725822663448,
        ]),
        BigInteger([
            3416834455341960795,
            16671020695514226682,
            10710465894442751248,
            718478200974601650,
            3563429719891691537,
            43433448924881166,
        ]),
        BigInteger([
            3615755344780392091,
            13225252129344058256,
            5330515027140079601,
            5315557301843237139,
            2333762400548443030,
            79213881900666201,
        ]),
        BigInteger([
            10019775360171954594,
            3292164760774962587,
            16370615967770433931,
            6827336786375020347,
            16238058989036166340,
            7556890630566277,
        ]),
        BigInteger([
            579643355688191102,
            15099485853982845065,
            14618491928976252937,
            17007384500210323837,
            12448719448429960498,
            51082080232610834,
        ]),
        BigInteger([
            5596339551731332537,
            15837533577554839307,
            15635967226359126990,
            4659533789167041643,
            13106949951045664984,
            80128178967663400,
        ]),
        BigInteger([
            14019264838572891659,
            6400996839992409755,
            6565105146359285638,
            6383939638433678925,
            6582988261855044443,
            110656476413704584,
        ]),
        BigInteger([
            287259043728827890,
            3865714827310624388,
            8022848478755595898,
            17810540512561044995,
            8094826246670247906,
            25916055667842693,
        ]),
        BigInteger([
            14150984043257839253,
            463038923537477831,
            6644519428194523278,
            7774969284163988706,
            7467638196757700306,
            40971474705339168,
        ]),
        BigInteger([
            14896249825832492977,
            6247778456993711545,
            14984023490203848738,
            12967220433925762461,
            6601233886321608688,
            98238061543052725,
        ]),
        BigInteger([
            14848717959437733567,
            3309457137038122964,
            13763365140627586135,
            16062021882067413309,
            7229231860487346305,
            30272462085483167,
        ]),
        BigInteger([
            4199920014185254035,
            1145045319049427557,
            12379873888035362389,
            14394345415537323406,
            6268509663813744538,
            103247570299432997,
        ]),
        BigInteger([
            5169328062946206750,
            16215534107881034782,
            18021962400448657987,
            9315840525735915025,
            6572395748765661667,
            77106805151608303,
        ]),
        BigInteger([
            16119001857909989344,
            9510304643196864149,
            11437652554290579262,
            5732570145580506,
            13573142844696243134,
            72031869019681982,
        ]),
        BigInteger([
            221080794120486324,
            9996683015888068749,
            7833014216051980072,
            10632703659365106810,
            6310168204945177572,
            48089586690329769,
        ]),
        BigInteger([
            9022619561486767301,
            17553445384523444137,
            10400100925644134566,
            1683532096887319944,
            13394864453231539130,
            95166237974040943,
        ]),
        BigInteger([
            8306272497313957463,
            8791831552799712898,
            3202647621904043022,
            7742712682926636042,
            7086646998158300902,
            76112631761704005,
        ]),
        BigInteger([
            17906849380840419925,
            11559670361897073469,
            8424191052863510908,
            17933094168489888653,
            14981006829295325992,
            57489660399770324,
        ]),
        BigInteger([
            15912301345294400536,
            3182902485023995235,
            18197782134250135297,
            11822741890895079691,
            8969499950166193558,
            82329970405499020,
        ]),
        BigInteger([
            2152173747998460775,
            9945061689732138904,
            6023122736819035367,
            771867933453144720,
            1406759935355035250,
            45989374955517438,
        ]),
        BigInteger([
            11012533712938735735,
            7384680292390220632,
            13649643846145887898,
            13264988082256794054,
            2302540502721730529,
            73107699216164418,
        ]),
        BigInteger([
            8520381167695498408,
            17868933688113535245,
            3911694768295522418,
            11728219780371092327,
            1059449801918088203,
            20228402438690260,
        ]),
        BigInteger([
            140111123280745772,
            11605739007888229499,
            11905751673233073332,
            17957499317206297169,
            1249120155008339798,
            113834668821765116,
        ]),
        BigInteger([
            10349903476102058307,
            9855656041869648539,
            10363369640431657398,
            2788550324891179872,
            14627511342860821184,
            115940846405920347,
        ]),
        BigInteger([
            17367426984546042096,
            10571643292679523776,
            13258728718453997571,
            9295289955334036063,
            14151721428720228289,
            87405613584593738,
        ]),
        BigInteger([
            1626338308030653342,
            9057540302009369202,
            7065330193000183965,
            15717352684046726349,
            393450225809197705,
            19756049059797280,
        ]),
    ];
    #[rustfmt::skip]
    const TWO_ADICITY: u32 = 46u32;
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        2022196864061697551u64,
        17419102863309525423u64,
        8564289679875062096u64,
        17152078065055548215u64,
        17966377291017729567u64,
        68610905582439508u64,
    ]);
}

impl FieldParameters for FqParameters {
    #[rustfmt::skip]
    const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    /// GENERATOR = -5
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xfc0b8000000002fa,
        0x97d39cf6e000018b,
        0x2072420fbfa05044,
        0xcbbcbd50d97c3802,
        0xbaf1ec35813f9eb,
        0x9974a2c0945ad2,
    ]);
    #[rustfmt::skip]
    const INV: u64 = 9586122913090633727u64;
    /// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x8508c00000000001,
        0x170b5d4430000000,
        0x1ef3622fba094800,
        0x1a22d9f300f5138f,
        0xc63b05c06ca1493b,
        0x1ae3a4617c510ea,
    ]);
    #[rustfmt::skip]
    const MODULUS_BITS: u32 = 377;
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x4284600000000000,
        0xb85aea218000000,
        0x8f79b117dd04a400,
        0x8d116cf9807a89c7,
        0x631d82e03650a49d,
        0xd71d230be28875,
    ]);
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        202099033278250856u64,
        5854854902718660529u64,
        11492539364873682930u64,
        8885205928937022213u64,
        5545221690922665192u64,
        39800542322357402u64,
    ]);
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0xb786686c9400cd22,
        0x329fcaab00431b1,
        0x22a5f11162d6b46d,
        0xbfdf7d03827dc3ac,
        0x837e92f041790bf9,
        0x6dfccb1e914b88,
    ]);
    #[rustfmt::skip]
    const REPR_SHAVE_BITS: u32 = 7;
    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    /// T = (MODULUS - 1) // 2^S =
    /// 3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0x7510c00000021423,
        0x88bee82520005c2d,
        0x67cc03d44e3c7bcd,
        0x1701b28524ec688b,
        0xe9185f1443ab18ec,
        0x6b8,
    ]);
    /// (T - 1) // 2 =
    /// 1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xba88600000010a11,
        0xc45f741290002e16,
        0xb3e601ea271e3de6,
        0xb80d94292763445,
        0x748c2f8a21d58c76,
        0x35c,
    ]);
}

impl PoseidonDefaultParameters for FqParameters {
    const PARAMS_OPT_FOR_CONSTRAINTS: [PoseidonDefaultParametersEntry; 7] = [
        PoseidonDefaultParametersEntry::new(2, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(3, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(4, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(5, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(6, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(7, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(8, 5, 8, 57, 0),
    ];
}
