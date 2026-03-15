pub static mut CFG: Config = Config::new();

pub struct Config {
    pub race_type: Race,            // 16E2
    pub is_2pl_mode: bool,          // 16E4
    pub p1_gears: Transmission,     // 16E6
    pub p2_gears: Transmission,     // 16E8
    pub p1_accel: Acceleration,     // 16EA
    pub p2_accel: Acceleration,     // 16EC
    pub course_type: Course,        // 16EE
    pub unk_16f0: u16,              // 16F0
    pub word_16f2: u16,             // 16F2
    pub snd_setting: SoundSettings, // 16F4
    pub word_16f6: SoundSettings,   // 16F6
    pub word_16f8: u16,             // 16F8
    pub word_16fa: u16,             // 16FA
    pub word_16fc: u16,             // 16FC
    pub byte_16fe: u8,              // 16FE
    pub word_16ff: u16,             // 16FF
    pub p1_kbd: [u8; 6],            // 1701: left, right, gear up, gear down, accel, brake
    pub p1_joy: JoyState,           // 1707
    pub byte_1715: u8,              // 1715
    pub word_1716: u16,             // 1716
    pub p2_kbd: [u8; 6],            // 1718: left, right, gear up, gear down, accel, brake
    pub p2_joy: JoyState,           // 171E
    pub arr_172c: [u8; 128],        // 172C
    pub p1_name: TextField,         // 17AC
    pub p2_name: TextField,         // 17B8
    pub cfg_keys: [TextField; 64],  // 17C4: 64 keys (12 chars each), e.g.: pwrwvwhnm-30
    pub game_keys: [TextField; 10], // 1AC4: 9 keys (12 chars each), e.g.: xKXCJGFJH-33
    pub game_code: TextField,       // 1B3C
    pub arr_1b48: [Score; 60],      // 1B48
    pub byte_1f08: u8,              // 1F08
}

impl Config {
    const fn new() -> Self {
        Self {
            race_type: Race::TimeLimit,
            is_2pl_mode: false,
            p1_gears: Transmission::Automatic,
            p2_gears: Transmission::Automatic,
            p1_accel: Acceleration::Button,
            p2_accel: Acceleration::Button,
            course_type: Course::T1,
            unk_16f0: 0,
            word_16f2: 0,
            snd_setting: SoundSettings::PcSpeaker,
            word_16f6: SoundSettings::PcSpeaker,
            word_16f8: 0,
            word_16fa: 0,
            word_16fc: 0,
            byte_16fe: 0,
            word_16ff: 0,
            p1_kbd: [0x4B, 0x4D, 0x48, 0x50, 0x1C, 0x52],
            p1_joy: JoyState::new(),
            byte_1715: 0,
            word_1716: 0,
            p2_kbd: [0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F],
            p2_joy: JoyState::new(),
            arr_172c: [0; 128],
            p1_name: *b"PLAYER 1    ",
            p2_name: *b"PLAYER 2    ",
            cfg_keys: [
                *b"pwrwvwhnm-30",
                *b"xmqiyskas-80",
                *b"uvqsnpbcm-70",
                *b"cwvbqpcav-50",
                *b"sfxuxxxxp-60",
                *b"hsywyskcg-50",
                *b"ivvemmkoz-50",
                *b"annsmqlpn-60",
                *b"vzvdophcy-50",
                *b"rtlmyjkhb-60",
                *b"errurv   -67",
                *b"nsssxxxxs-60",
                *b"wsvuqpcsj-70",
                *b"oundefacg-99",
                *b"gxwdypacv-68",
                *b"bz zf bat-90",
                *b"lwnjwkacn-90",
                *b"iyvvnveqr-35",
                *b"kazznikai-45",
                *b"fgqljgdaf-65",
                *b"mffsrpydu-60",
                *b"plqtzqdpe-80",
                *b"zkzgkjkkk-50",
                *b"tggjggttt-63",
                *b"afzybqcjt-70",
                *b"jboukjhka-99",
                *b"dasicotet-80",
                *b"xdnuseece-85",
                *b"qdscjvebt-75",
                *b"skgyxxxxk-57",
                *b"ykgjwvnak-92",
                *b"wjmegmeqh-60",
                *b"crripwbxx-28",
                *b"qpwmvqkcq-34",
                *b"xgpgpzhhs-42",
                *b"fgwlsyckm-51",
                *b"prrumpumv-68",
                *b"nancxxxxz-39",
                *b"ipwonwobp-65",
                *b"rlqydvaka-48",
                *b"hdmoqfaka-51",
                *b"wxqbqmdxd-88",
                *b"udonajhal-47",
                *b"nkwcxxxxk-33",
                *b"aonglqktc-63",
                *b"zxjghbkhf-70",
                *b"dpgtqkbhq-42",
                *b"ipmijobhq-62",
                *b"muyurwfha-86",
                *b"pprggqfvl-52",
                *b"jpiqkuhce-65",
                *b"eiibggafe-48",
                *b"cigiuqclt-92",
                *b"knhuphhke-64",
                *b"vvoshgsig-86",
                *b"rghsvbret-89",
                *b"ydoeractj-86",
                *b"gxqfsumpp-45",
                *b"tvqlsyufu-89",
                *b"wmqhymtvj-85",
                *b"okuobjiac-86",
                *b"fimjjibck-68",
                *b"sigtxxxxh-35",
                *b"wnqkmphvn-80",
            ],
            game_keys: [
                *b"x        -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
                *b"         -00",
            ],
            game_code: *b"x        -00",
            arr_1b48: [
                Score {
                    name: *b"mf ltd      ",
                    score: 5_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"shaun       ",
                    score: 4_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"andrew      ",
                    score: 3_000_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"peter       ",
                    score: 2_500_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"chris       ",
                    score: 2_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"andy        ",
                    score: 1_500_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"doug        ",
                    score: 1_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"dave        ",
                    score: 500_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"luga        ",
                    score: 200_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"michelangelo",
                    score: 100_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"sout        ",
                    score: 50,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"ric         ",
                    score: 40,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"craig       ",
                    score: 30,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"nige        ",
                    score: 25,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"rachell     ",
                    score: 20,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"esther      ",
                    score: 15,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"rich        ",
                    score: 10,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"stuart      ",
                    score: 5,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"tim         ",
                    score: 2,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"andy        ",
                    score: 1,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"a morris    ",
                    score: 5_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"janet       ",
                    score: 4_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"lorna       ",
                    score: 3_000_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"vicky       ",
                    score: 2_500_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"carly       ",
                    score: 2_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"gary        ",
                    score: 1_500_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"sharon x 2  ",
                    score: 1_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"fiona       ",
                    score: 500_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"sam the dog ",
                    score: 200_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"all vegans  ",
                    score: 100_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"brian hobbs ",
                    score: 50,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"stacey green",
                    score: 40,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"ross morris ",
                    score: 30,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"chris clark ",
                    score: 25,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"andy turner ",
                    score: 20,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"iwan        ",
                    score: 15,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"eric        ",
                    score: 10,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"dafydd      ",
                    score: 5,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"darren      ",
                    score: 2,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"tom         ",
                    score: 1,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"peter       ",
                    score: 5_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"ernie       ",
                    score: 4_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"gaz hawley  ",
                    score: 3_000_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"frank       ",
                    score: 2_500_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"annette     ",
                    score: 2_000_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"ritchie     ",
                    score: 1_500_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"sue         ",
                    score: 1_000_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"scotty      ",
                    score: 500_000,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"colin       ",
                    score: 200_000,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"billy west  ",
                    score: 100_000,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"sue         ",
                    score: 50,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"mel         ",
                    score: 40,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"ali         ",
                    score: 30,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"anne        ",
                    score: 25,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"mandy       ",
                    score: 20,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"janet       ",
                    score: 15,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"christine   ",
                    score: 10,
                    model: ScoreModel::Esprit,
                },
                Score {
                    name: *b"becca       ",
                    score: 5,
                    model: ScoreModel::Elan,
                },
                Score {
                    name: *b"tim         ",
                    score: 2,
                    model: ScoreModel::M200,
                },
                Score {
                    name: *b"maggie      ",
                    score: 1,
                    model: ScoreModel::Esprit,
                },
            ],
            byte_1f08: 0,
        }
    }

    /// 16E2..=1F08, len = 2087
    fn save(&self) {
        // self.race_type as u16
        // self.is_2pl_mode as u16
        // self.p1_gears as u16
        // self.p2_gears as u16
        // self.p1_accel as u16
        // self.p2_accel as u16
        // self.course_type as u16
        // self.unk_16f0
        // self.word_16f2
        // self.snd_setting
        // self.word_16f6
        // self.word_16f8
        // self.word_16fa
        // self.word_16fc
        // self.byte_16fe
        // self.word_16ff
        // self.arr_1701
        // self.byte_1707
        // self.byte_1708
        // self.word_1709
        // self.word_170b
        // self.unk_170d
        // self.byte_1715
        // self.word_1716
        // self.arr_1718
        // self.byte_171e
        // self.byte_171f
        // self.word_1720
        // self.word_1722
        // self.unk_1724
        // self.arr_172c
        // self.p1_name
        // self.p2_name
        // self.cfg_keys
        // self.game_keys
        // self.game_code
        // self.arr_1b48
        // self.byte_1f08
    }
}

pub type TextField = [u8; 12];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Transmission {
    Manual = 0,
    Automatic = 1,
}

impl Transmission {
    pub fn next(&self) -> Self {
        match self {
            Self::Manual => Self::Automatic,
            Self::Automatic => Self::Manual,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Acceleration {
    Button = 0,
    Joystick = 1,
}

impl Acceleration {
    pub fn next(&self) -> Self {
        match self {
            Self::Button => Self::Joystick,
            Self::Joystick => Self::Button,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Race {
    TimeLimit = 0,
    Competition = 1,
}

impl Race {
    pub fn next(&self) -> Self {
        match self {
            Self::TimeLimit => Self::Competition,
            Self::Competition => Self::TimeLimit,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Course {
    T1 = 0,
    T2 = 1,
    T3 = 2,
    Circular = 3,
    Unknown = 4,
}

impl Course {
    pub fn next(&self) -> Self {
        match self {
            Self::T1 => Self::T2,
            Self::T2 => Self::T3,
            Self::T3 => Self::Circular,
            Self::Circular => Self::Unknown,
            Self::Unknown => Self::T1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Esprit = 0,
    Elan = 1,
    M200 = 2,
}

impl Default for Model {
    fn default() -> Self {
        Self::Esprit
    }
}

impl Model {
    pub fn next(&self) -> Self {
        match self {
            Self::Esprit => Self::Elan,
            Self::Elan => Self::M200,
            Self::M200 => Self::Esprit,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Esprit => Self::M200,
            Self::Elan => Self::Esprit,
            Self::M200 => Self::Elan,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SoundSettings {
    Adlib = 1,
    SoundBlaster = 2,
    PcSpeaker = 3,
    Off = 4,
}

pub struct Score {
    pub name: TextField,
    pub score: u32,
    pub model: ScoreModel,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScoreModel {
    Esprit = 0,
    Elan = 0x40,
    M200 = 0x80,
}

#[derive(Default)]
pub struct JoyState {
    pub byte0: u8,  // 1707/171E
    pub byte1: u8,  // 1708/171F
    pub word2: u16, // 1709/1720
    pub word4: u16, // 170B/1722
    pub word6: u16,
    pub word8: u16,
    pub word10: u16,
    pub word12: u16,
}

impl JoyState {
    const fn new() -> Self {
        Self {
            byte0: 0,
            byte1: 0,
            word2: 0,
            word4: 0,
            word6: 0,
            word8: 0,
            word10: 0,
            word12: 0,
        }
    }
}
