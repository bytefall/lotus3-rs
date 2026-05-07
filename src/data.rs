use core::ptr;

use crate::{
    config::{Acceleration, Race, Transmission},
    hud::VGA_HEIGHT,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct FarPointer {
    pub offset: u16,
    pub segment: u16,
}

impl FarPointer {
    pub const fn new() -> Self {
        Self {
            offset: 0,
            segment: 0,
        }
    }
}

/// Order in the menu:
/// Rally, Futuristic, Night, Marsh, Mountains, Snow,
/// Roadworks, Storm, Desert, Fog, MotorWay, Wind, Forest.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Scenario {
    Forest = 0,
    Night = 1,
    Fog = 2,
    Snow = 3,
    Desert = 4,
    MotorWay = 5,
    Marsh = 6,
    Storm = 7,
    Roadworks = 8,
    Rally = 9,
    Wind = 10,
    Futuristic = 11,
    Mountains = 12,
}

pub static mut WORD_156B: u16 = 0; // 156B
pub static mut WORD_156D: u16 = 0; // 156D

// TOOD: ...
pub static mut WORD_1F20: u16 = 0; // 1F20
// rb 1                                    ; 1F22

// TOOD: ...
pub static mut WORD_1F28: u16 = 0; // 1F28

/// Array of 9 elements:
///
/// 0) Type (straight, circular) + track_num
/// 1) Curves
/// 2) Sharpness
/// 3) Length
/// 4) Hills
/// 5) Steepness
/// 6) Scenery
/// 7) Scatter
/// 8) Obstacles
pub static mut ARR_1F3E: [u16; 9] = [0; 9];
pub static mut WORD_1F50: u16 = 0; // 1F50: difficulty (0..=99)

pub const ARR_2674: [u8; 2] = [0, 0];

pub static mut IS_CIRCULAR_TRACK: bool = false; // 23F4
pub static mut WORD_23F6: u16 = 1; // 23F6
pub static mut WORD_23F8: u16 = 0; // 23F8
pub static mut WORD_23FA: u16 = 0; // 23FA

pub static mut NUM_OF_TRACKS: u16 = 0; // 23F0
pub static mut WORD_23F2: u16 = 0;
pub static mut WORD_23FC: usize = 0; // offset for TMP_FILE_BUF
pub static mut WORD_23FE: u16 = 0;

pub static mut ARR_2400: [u16; 17] = [0; 17]; // 2400

pub static mut WORD_249E: u16 = 0; // 249E

pub const ARR_24D4: [u8; 208] = [
    0xC8, 0x94, 0, 0, 0x80, 0x99, 0x80, 0x98, 0, 0, 0x10, 0x95, 0x20, 0x97, 0, 0, 0xF2, 0xC0, 0xF1,
    0xF5, 0xF8, 0xC1, 0xF8, 0xC2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xF0, 0xA3, 0xF8, 0xA4, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0xF8, 0xAD, 0, 0, 0xF0, 0xAF, 0xF0, 0xB0, 0xF1, 0xB1, 0xF2, 0xF2, 0xF0, 0xB3,
    0, 0, 0xF0, 0xB5, 0xF0, 0xB6, 0xF0, 0xB7, 0xF0, 0xB8, 0xF0, 0xB9, 0xF1, 0xBA, 0xF2, 0xF3, 0, 0,
    0xF0, 0x9B, 0xF0, 0xA2, 0xF0, 0xA2, 0xF3, 0xA2, 0xF0, 0x9B, 0xF0, 0xA2, 0xF0, 0xA2, 0xF3, 0xA2,
    0xF8, 0xC9, 0, 0, 0, 0, 0xF0, 0xCC, 0, 0, 0, 0, 0, 0, 0, 0, 0xF8, 0xD0, 0xF0, 0x97, 0xF0, 0xE8,
    0xF0, 0xEF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xF0, 0xD9, 0xF8, 0xD7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0xF8, 0xE7, 0, 0, 0x20, 0xE9, 0x20, 0xEA, 0x20, 0xED, 0, 0, 0, 0, 0, 0, 0xF8, 0xB2, 0xF8, 0xA6,
    0xC0, 0x86, 0xF0, 0xBB, 0xF0, 0x81, 0, 0, 0, 0, 0, 0, 0xF0, 0xDD, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0xF1, 0x86, 0xF1, 0x86, 0xF1, 0x86, 0xF1, 0x86, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub static mut WORD_2422: [u16; 16] = [0; 16]; // 2422: max 16 tracks
pub static mut WORD_2442: u16 = 0; // 2442
pub static mut WORD_2444: u16 = 0; // 2444
pub static mut WORD_2446: u16 = 0; // 2446
pub static mut WORD_2448: u16 = 0; // 2448
pub static mut WORD_244A: u16 = 0; // 244A
pub static mut WORD_244C: u16 = 0; // 244C
pub static mut WORD_244E: u16 = 0; // 244E
pub static mut WORD_2450: u16 = 0; // 2450
pub static mut WORD_2452: i16 = 0; // 2452
pub static mut WORD_2454: u16 = 0; // 2454
pub static mut WORD_2456: u16 = 0; // 2456
pub static mut WORD_2458: i16 = 0; // 2458
pub static mut WORD_245A: i16 = 0; // 245A
pub static mut WORD_245C: i16 = 0; // 245C
pub static mut WORD_245E: i16 = 0; // 245E
pub static mut WORD_2460: i16 = 0; // 2460
pub static mut WORD_2462: i16 = 0; // 2462
pub static mut WORD_2464: i16 = 0; // 2464
pub static mut WORD_2466: i16 = 0; // 2466
pub static mut WORD_2468: i16 = 0; // 2468
pub static mut WORD_246A: i16 = 0; // 246A
pub static mut WORD_246C: u16 = 0; // 246C
pub static mut WORD_246E: u16 = 0; // 246E
pub static mut WORD_2470: i16 = 0; // 2470
pub static mut WORD_2472: i16 = 0; // 2472
pub static mut WORD_2474: i16 = 0; // 2474
pub static mut WORD_2476: i16 = 0; // 2476
pub static mut WORD_2478: i16 = 0; // 2478
pub static mut WORD_247A: i16 = 0; // 247A
pub static mut WORD_247C: i16 = 0; // 247C
pub static mut WORD_247E: i16 = 0; // 247E
pub static mut WORD_2480: i16 = 0; // 2480
pub static mut WORD_2482: i16 = 0; // 2482
pub static mut WORD_2484: i16 = 0; // 2484
pub static mut WORD_2486: i16 = 0; // 2486
pub static mut WORD_2488: u16 = 0; // 2488
pub static mut WORD_248A: u16 = 0; // 248A
pub static mut WORD_248C: u16 = 0; // 248C

pub static mut WORD_2E78: u16 = 0; // 2E78

pub const ARR48_3126: [u8; 48] = [
    0, 0, 0, 0x15, 0x15, 0x15, 0x2A, 0x2A, 0x2A, 0x38, 0x38, 0x38, 0x20, 0, 0, 0x30, 0, 0, 0, 0x20,
    0, 0, 0x15, 9, 9, 0x2A, 0x17, 0x31, 0x2A, 9, 0x18, 0x15, 9, 0x21, 0x18, 9, 0x2C, 0x24, 9, 0,
    0x13, 0x2A, 0, 0xD, 0x22, 0, 0, 0,
]; // 3126
pub static mut ARR4_3156: [u8; 4] = *b"???\0"; // 3156
pub static mut RES_KEY: [u8; 8] = [b' '; 8]; // 315A
// dw ?                                    ; 3162
pub static mut ARC_HEADER: [u8; 2816] = [0; 2816]; // 3164
pub const ARC_FILE_NAME: &str = "LOTUS.DAT\0"; // 3C64
pub static mut ARC_FILE_HANDLE: u16 = 0; // 3C6E

pub static mut CAR_NUM: u16 = 0; // 3EDA: 0 - Esprit S4, 1 - Elan SE, 2 - M200
pub static mut IS_2PL_MODE: bool = false; // 3EDC
pub static mut RACE_TYPE: Race = Race::TimeLimit; // 3EDE

pub static mut BYTE_3D6A: u16 = 0; // 3D6A
pub static mut WORD_3D6C: u16 = 20; // 3D6C: fade speed (greater = slower)
pub static mut VGA_PAL_READY: bool = false; // 3D6E
pub static mut FADE_STEP: u16 = 0; // 3D70
pub static mut BYTE_3D72: u8 = 0; // 3D72: default_disk
pub static mut DOS_VID_MODE: u8 = 3; // 3D73
pub static mut BYTE_3D74: u8 = 0; // 3D74
pub static mut WORD_3D76: u16 = 0; // 3D76
pub static mut WORD_3D78: u16 = 0; // 3D78
pub static mut WORD_3D7A: u16 = 0; // 3D7A: calibration

/// 3D7C: Interrupt Service Routines are set
pub static mut ISRS_SET: bool = false;

pub static mut ORIG_DOS_CEH_ISR: FarPointer = FarPointer::new(); // 3D7E, 3D80
pub static mut ORIG_KBD_ISR: FarPointer = FarPointer::new(); // 3D82, 3D84
pub static mut BYTE_3D86: bool = false; // 3D86: true = key pressed, false = released
pub static mut BYTE_3D87: u8 = 0; // 3D87

pub static mut BYTE_3D88: [u8; 128] = [0; 128]; // 3D88
// byte_3D88:          db ?                                    ; 3D88
// byte_3D89:          rb 24                                   ; 3D89
// byte_3DA1:          rb 3                                    ; 3DA1
// byte_3DA4:          rb 44                                   ; 3DA4
// byte_3DD0:          dw ?                                    ; 3DD0
// byte_3DD2:          db ?                                    ; 3DD2
// byte_3DD3:          dw ?                                    ; 3DD3
// byte_3DD5:          db ?                                    ; 3DD5
// byte_3DD6:          dw ?                                    ; 3DD6
// byte_3DD8:          rb 48                                   ; 3DD8

pub static mut WORD_3E08: u16 = 0; // 3E08
pub static mut ORIG_PIT_ISR: FarPointer = FarPointer::new(); // 3E0A, 3E0C
pub static mut BYTE_3E0E: u8 = 0; // 3E0E
pub static mut BYTE_3E0F: i8 = 1; // 3E0F
pub static mut BYTE_3E10: u8 = 0; // 3E10
pub static mut WORD_3E12: u16 = 0; // 3E12
pub static mut WORD_3E14: u16 = 0; // 3E14
pub static mut WORD_3E16: u16 = 0; // 3E16: timer_value
pub static mut WORD_3E18: u16 = 0; // 3E18
pub static mut WORD_3E1A: u16 = 0; // 3E1A
pub static mut WORD_3E1C: u16 = 0x8000; // 3E1C
pub static mut WORD_3E1E: u16 = 0; // 3E1E
pub static mut WORD_3E20: u16 = 0; // 3E20
pub static mut WORD_3E22: u16 = 0; // 3E22
pub static mut BYTE_3E24: u8 = 0xF; // 3E24
pub static mut BYTE_3E25: u8 = 0; // 3E25
pub static mut BYTE_3E2C: u8 = 0; // 3E2C: mouse_status
pub static mut BYTE_3E2D: u8 = 0; // 3E2D
pub static mut WORD_3E2E: u16 = 0; // 3E2E

pub static mut WORD_3E30: u16 = 0x140; // 3E30: mouse x
pub static mut WORD_3E32: u16 = 0x64; // 3E32: mouse y
pub const CREATE_ERROR: &str = "ERROR: Can't create $\0"; // 3E34
pub const OPEN_ERROR: &str = "ERROR: Can't open $\0"; // 3E4A
pub const READ_ERROR: &str = "ERROR: Read error in $\0\0"; // 3E5E
pub const WRITE_ERROR: &str = "ERROR: Write error in $\0"; // 3E76
pub const CLOSE_ERROR: &str = "ERROR: Can''t close $\0\0"; // 3E8E
pub static mut ERR_STR_PTR: *const u8 = ptr::null(); // 3EA4
pub static mut MEMORY_ERROR: &str = "HELP! Memory full.\0"; // 3EA6
pub static mut WORD_3EB9: &str = VERSION_STR; // 3EB9
pub static mut BYTE_3EBB: u8 = 0; // 3EBB
pub const VERSION_STR: &str = "LOTUS III  Version 14/07/93\0"; // 3EBC

pub static mut WORD_3EE0: u16 = 0; // 3EE0
pub static mut WORD_3EE2: u16 = 0; // 3EE2
pub static mut WORD_3EE4: Race = Race::TimeLimit; // 3EE4
pub static mut WORD_3EE6: u16 = 0; // 3EE6
pub static mut WORD_3EE8: u16 = 0; // 3EE8
pub static mut WORD_3EEA: u16 = 0; // 3EEA
pub static mut WORD_3EEC: u16 = 0; // 3EEC
// rnd_3EEE:           rw 8                                    ; 3EEE
// rnd_3EFE:           rw 8                                    ; 3EFE
// arr_3F0E:           rw 16                                   ; 3F0E
pub static mut RACE_CODE: [u8; 12] = [0; 12]; // 3F2E
pub static mut WORD_3F3A: u16 = 0; // 3F3A, is a struct (gameplay data)
pub static mut WORD_3F3C: u16 = 0; // 3F3C
pub static mut WORD_3F3E: u16 = 0; // 3F3E
// u16 = 0;                                    // 3F40
pub static mut WORD_3F42: u16 = 0; // 3F42
pub static mut WORD_3F44: u16 = 0; // 3F44
pub static mut WORD_3F46: u8 = 0; // 3F46
// u8 = 0;                                    // 3F47
pub static mut WORD_3F48: u16 = 0; // 3F48: p1_speed
// u16 = 0;                                    // 3F4A
// rb 8                                    // 3F4C
pub static mut WORD_3F54: u16 = 0; // 3F54
// u16 = 0;                                    // 3F56
// u16 = 0;                                    // 3F58
pub static mut WORD_3F5A: u16 = 0; // 3F5A
pub static mut WORD_3F5C: [u16; 6] = [0; 6]; // 3F5C
pub static mut WORD_3F68: u16 = 0; // 3F68
pub static mut WORD_3F6A: u16 = 0; // 3F6A
pub static mut BYTE_3F6C: u8 = 0; // 3F6C
pub static mut BYTE_3F6D: u8 = 0; // 3F6D
pub static mut WORD_3F6E: u16 = 0; // 3F6E
pub static mut BYTE_3F70: u16 = 0; // 3F70
pub static mut P1_ACCEL: Acceleration = Acceleration::Button; // 3F72
pub static mut P1_GEARS: Transmission = Transmission::Automatic; // 3F74
// rb 10                                   // 3F76
pub static mut WORD_3F80: u16 = 0; // 3F80
pub static mut WORD_3F82: [u16; 25] = [0; 25]; // 3F82
pub static mut WORD_3FB4: u16 = 0; // 3FB4
pub static mut WORD_3FB6: u16 = 0; // 3FB6
pub static mut WORD_3FB8: u16 = 0; // 3FB8
pub static mut WORD_3FBA: u16 = 0; // 3FBA
pub static mut WORD_3FBC: u16 = 0; // 3FBC
pub static mut WORD_3FBE: u16 = 0; // 3FBE
pub static mut WORD_3FC0: u16 = 0; // 3FC0
pub static mut WORD_3FC2: u16 = 0; // 3FC2
// rb 6                                    // 3FC4
pub static mut WORD_3FCA: u16 = 0; // 3FCA
// rb 6                                    // 3FCC
pub static mut WORD_3FD2: u16 = 0; // 3FD2
// rb 14                                   // 3FD4
pub static mut WORD_3FE2: u16 = 0; // 3FE2
// rb 12                                   // 3FE4
pub static mut WORD_3FF0: u16 = 0; // 3FF0
// rb 26                                   // 3FF2
pub static mut WORD_400C: u16 = 0; // 400C
// rb 4                                    // 400E
pub static mut WORD_4012: u16 = 0; // 4012
// u16 = 0;                                    // 4014
// u16 = 0;                                    // 4016
pub static mut WORD_4018: u16 = 0; // 4018
pub static mut WORD_401A: u16 = 0; // 401A
pub static mut WORD_401C: u16 = 0; // 401C
pub static mut WORD_401E: u16 = 0; // 401E
// u16 = 0;                                    // 4020
// u16 = 0;                                    // 4022
// u16 = 0;                                    // 4024
pub static mut WORD_4026: u16 = 0; // 4026
pub static mut WORD_4028: u16 = 0; // 4028
pub static mut WORD_402A: u16 = 0; // 402A
// rb 8                                    // 402C
pub static mut WORD_4034: u16 = 0; // 4034
pub static mut WORD_4036: u16 = 0; // 4036
pub static mut WORD_4038: u16 = 0; // 4038
// u16 = 0;                                    // 403A
pub static mut WORD_403C: u16 = 0; // 403C
pub static mut WORD_403E: u16 = 0; // 403E
pub static mut WORD_4040: u16 = 0; // 4040
pub static mut WORD_4042: u16 = 0; // 4042
// u16 = 0;                                    // 4044
// u16 = 0;                                    // 4046
// u16 = 0;                                    // 4048
// u16 = 0;                                    // 404A
// u16 = 0;                                    // 404C
pub static mut WORD_404E: u16 = 0; // 404E
// u16 = 0;                                    // 4050
// u16 = 0;                                    // 4052
pub static mut WORD_4054: u16 = 0; // 4054
pub static mut WORD_4056: u16 = 0; // 4056
// rb 10                                   // 4058
pub static mut WORD_4062: u16 = 0; // 4062
pub static mut WORD_4064: u16 = 0; // 4064
pub static mut BYTE_4066: u8 = 0; // 4066
// u8 = 0;                                    // 4067
pub static mut BYTE_4068: u8 = 0; // 4068
// u8 = 0;                                    // 4069
pub static mut BYTE_406A: u8 = 0; // 406A
// u8 = 0;                                    // 406B
pub static mut P2_ACCEL: Acceleration = Acceleration::Button; // 406C
pub static mut P2_GEARS: Transmission = Transmission::Automatic; // 406E
// rb 10                                   // 4070
pub static mut WORD_407A: u16 = 0; // 407A
pub static mut WORD_407C: [u16; 25] = [0; 25]; // 407C
pub static mut WORD_40AE: u16 = 0; // 40AE
pub static mut WORD_40B0: u16 = 0; // 40B0
pub static mut WORD_40B2: u16 = 0; // 40B2
pub static mut WORD_40B4: u16 = 0; // 40B4
pub static mut WORD_40B6: u16 = 0; // 40B6
pub static mut WORD_40B8: u16 = 0; // 40B8
pub static mut WORD_40BA: u16 = 0; // 40BA
pub static mut WORD_40BC: u16 = 0; // 40BC
// rb 6                                    // 40BE
pub static mut WORD_40C4: u16 = 0; // 40C4
// rb 6                                    // 40C6
pub static mut WORD_40CC: u16 = 0; // 40CC
// rb 14                                   // 40CE
pub static mut WORD_40DC: u16 = 0; // 40DC
// rb 12                                   // 40DE
pub static mut WORD_40EA: u16 = 0; // 40EA
// rb 26                                   // 40EC
pub static mut WORD_4106: u16 = 0; // 4106
// u16 = 0;                                    // 4108
// u16 = 0;                                    // 410A
pub static mut WORD_410C: u16 = 0; // 410C
// u16 = 0;                                    // 410E
// u16 = 0;                                    // 4110
pub static mut WORD_4112: u16 = 0; // 4112
pub static mut WORD_4114: u16 = 0; // 4114
pub static mut WORD_4116: u16 = 0; // 4116
pub static mut WORD_4118: u16 = 0; // 4118
// rb 6                                    // 411A
pub static mut WORD_4120: u16 = 0; // 4120
pub static mut WORD_4122: u16 = 0; // 4122
pub static mut WORD_4124: u16 = 0; // 4124
// rb 8                                    // 4126
pub static mut ARR_412E: [u8; 80] = [0; 80]; // 412E
pub static mut ARR_417E: [u8; 96] = [0; 96]; // 417E
pub static mut WORD_41DE: u16 = 0; // 41DE
pub static mut WORD_41E0: u16 = 0; // 41E0
// rb 140                                  // 41E2

pub static mut ARR_426E: [u8; 4] = [0; 4]; // 426E
pub static mut ARR_4272: [u8; 4] = [0; 4]; // 4272
pub static mut ARR_4276: [u8; 4] = [0; 4]; // 4276
pub static mut ARR_427A: [u8; 4] = [0; 4]; // 427A
pub static mut ARR_427E: [u8; 120] = [0; 120]; // 427E
pub static mut WORD_42F6: u16 = 0; // 42F6
// u16 = 0;                                    // 42F8
pub static mut WORD_42FA: u16 = 0; // 42FA
// u16 = 0;                                    // 42FC
pub static mut WORD_42FE: u16 = 0; // 42FE
// u16 = 0;                                    // 4300
pub static mut WORD_4302: u16 = 0; // 4302
// u16 = 0;                                    // 4304
pub static mut WORD_4306: u16 = 0; // 4306
pub static mut WORD_4308: u16 = 0; // 4308
// rb 156                                  // 430A
// TODO: word_43A6

pub static mut WORD_632E: u16 = 0; // 632E, si
pub static mut WORD_6330: u16 = 0; // 6330, es
pub static mut _UNUSED: u16 = 0; // 6332
pub static mut WORD_6334: u16 = 0; // 6334
pub static mut WORD_6336: u16 = 0; // 6336
pub static mut WORD_6338: u16 = 0; // 6338
pub static mut WORD_633A: u16 = VGA_HEIGHT as u16; // 633A
pub static mut TRACK_NUM: Scenario = Scenario::Forest; // 633C
pub static mut WORD_633E: u16 = 0; // 633E
pub static mut WORD_6340: u16 = 0; // 6340
pub static mut WORD_6342: u16 = 0; // 6342
pub static mut WORD_6344: u16 = 0; // 6344
pub static mut WORD_6346: u16 = 0; // 6346
pub static mut WORD_6348: u16 = 0; // 6348
pub static mut WORD_634A: u16 = 0; // 634A
pub static mut WORD_634C: u16 = 0; // 634C
pub static mut WORD_634E: i16 = 0; // 634E
pub static mut WORD_6350: u16 = 0; // 6350
pub static mut WORD_6352: u16 = 0; // 6352
pub static mut WORD_6354: u16 = 0; // 6354
pub static mut WORD_6356: u16 = 0; // 6356
pub static mut WORD_6358: u16 = 0; // 6358
pub static mut WORD_635A: u16 = 0; // 635A
pub static mut WORD_635C: u16 = 0; // 635C
pub static mut WORD_635E: u16 = 0; // 635E
pub static mut WORD_6360: u16 = 0; // 6360
pub static mut WORD_6362: (u8, u8) = (0, 0); // 6362: TODO: make it (i8, u8)
pub static mut WORD_6364: u16 = 0; // 6364
pub static mut WORD_6366: i16 = 0; // 6366
pub static mut WORD_6368: u16 = 0; // 6368
pub static mut WORD_636A: i16 = 0; // 636A
pub static mut WORD_636C: u16 = 0; // 636C
pub static mut WORD_636E: u16 = 0; // 636E
pub static mut WORD_6370: u16 = 0; // 6370
pub static mut WORD_6372: u16 = 0; // 6372
pub static mut WORD_6374: u16 = 0; // 6374
pub static mut WORD_6376: usize = 0; // 6376
pub static mut WORD_6378: u16 = 0; // 6378
pub static mut WORD_637A: u16 = 0; // 637A
pub static mut WORD_637C: u16 = 0; // 637C
pub static mut WORD_637E: u16 = 0; // 637E
pub static mut WORD_6380: u16 = 0; // 6380
pub static mut WORD_6382: usize = 0; // 6382
pub static mut WORD_6384: u16 = 0; // 6384
pub static mut WORD_6386: u16 = 0; // 6386
pub static mut WORD_6388: u16 = 0; // 6388
pub static mut WORD_638A: u16 = 0; // 638A
pub static mut WORD_638C: u16 = 0; // 638C
pub static mut WORD_638E: u16 = 0; // 638E
pub static mut WORD_6390: u16 = 0; // 6390
pub static mut WORD_6392: u16 = 0; // 6392
pub static mut WORD_6394: u16 = 0; // 6394
pub static mut WORD_6396: u16 = 0; // 6396
pub static mut WORD_6398: u16 = 0; // 6398
pub static mut WORD_639A: u16 = 0; // 639A

pub static mut IS_TR_FOREST: bool = false; // 639C
pub static mut IS_TR_NIGHT: bool = false; // 639E
pub static mut IS_TR_FOG: bool = false; // 63A0
pub static mut IS_TR_SNOW: bool = false; // 63A2
pub static mut IS_TR_DESERT: bool = false; // 63A4
pub static mut IS_TR_MOTORWAY: bool = false; // 63A6
pub static mut IS_TR_MARSH: bool = false; // 63A8
pub static mut IS_TR_STORM: bool = false; // 63AA
pub static mut IS_TR_ROADWORKS: bool = false; // 63AC
pub static mut IS_TR_RALLY: bool = false; // 63AE
pub static mut IS_TR_WIND: bool = false; // 63B0
pub static mut IS_TR_FUTURISTIC: bool = false; // 63B2
pub static mut IS_TR_MOUNTAINS: bool = false; // 63B4

pub static mut PLAYING_DEMO: u16 = 0; // 63B6
pub static mut IS_PAUSED: u16 = 0; // 63B8
pub static mut WORD_63BA: u16 = 0; // 63BA
pub static mut WORD_63BE: u16 = 0; // dw arc_header_seg ; 63BE
pub static mut DAT_FILE_HANDLE: u16 = 0; // 63C0
pub static mut ARR_63C2: [u16; 512] = [0; 512]; // 63C2
pub static mut ARR_67C2: [u16; 128] = [0; 128]; // 67C2
pub static mut ARR_68C2: [u16; 128] = [0; 128]; // 68C2
pub static mut ARR_69C2: [u16; 128] = [0; 128]; // 69C2
pub static mut ARR_6AC2: [u16; 128] = [0; 128]; // 6AC2
pub static mut ARR_6BC2: [u16; 257] = [0; 257]; // 6BC2
pub static mut ARR_6DC4: [u16; 201] = [0; 201]; // 6DC4
pub static mut VGA_PAL: [u8; 256 * 3] = [0; 256 * 3]; // 6F56: VGA palette which is flushed to the DAC
pub static mut PALETTE2: [u8; 256 * 3] = [0; 256 * 3]; // 7256
pub static mut PALETTE: [u8; 256 * 3] = [0; 256 * 3]; // 7556

pub static mut TMP_FILE_BUF: [u8; 16384] = [0; 1024 * 16]; // 8156 + 8256 + 8358 + 8A00

pub static mut VGA_FADE_PAL: [u16; 256 * 6] = [0; 256 * 6]; // C560

// vars.inc
pub static mut WORD_2AB6: u16 = 0;

pub static mut WORD_5160: u16 = 0; // 5160: ES
pub static mut WORD_5162: u16 = 0; // 5162
pub static mut WORD_5164: u16 = 0; // 5164: SS
pub static mut WORD_5166: u16 = 0; // 5166
pub static mut WORD_5168: u16 = 0; // 5168
pub static mut WORD_516A: u16 = 0; // 516A
