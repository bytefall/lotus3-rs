use crate::{
    config::CFG,
    data::*,
    menu::BYTE_1F58,
    prepare::sub_c011,
    timer::{resume_task_context, sub_d92b, sub_d915},
};

/// 5283:
pub unsafe fn loc_5283() {
    sub_d915();

    PLAYING_DEMO = 0;
    // mov     word [word_1F26], loc_5222
    // call    prepare_task_context(&loc_5371)
    // mov     ax, [demo_counter]
    // call    sub_4EE6
    // mov     ax, 0AF0h
    // call    sleep
    // jmp     loc_51E9
}

/// 5371:
pub unsafe fn loc_5371() {
    loop {
        if sub_c011() {
            // WORD_1F26(); // jmp to exit fn 0x51C4
            break;
        }

        resume_task_context();
    }
}

/// 537F
pub unsafe fn sub_537f() {
    const ARR_1F2A: [u16; 5] = [7, 10, 15, 1, 9];

    let ax = ARR_1F2A[CFG.course_type as usize];

    WORD_23F6 = ax;
    WORD_3EE4 = RACE_TYPE;
    WORD_3F68 = 0;
    WORD_3F6A = 0;
    WORD_4062 = 0;
    WORD_4064 = 0;
    WORD_156B = 0;
    WORD_156D = 0;
    WORD_1F28 = 0;
}

const ARR_1F34: [u8; 10] = [0, 7, 17, 74, 64, 32, 39, 49, 74, 64]; // 1F34

// Global state (simulating memory locations)
static mut BYTE_3DA4: u8 = 0;
static mut BYTE_3DD0: u8 = 0;
static mut BYTE_3DD8: u8 = 0;
static mut BYTE_3DD5: u8 = 0;
static mut BYTE_3DD3: u8 = 0;
static mut WORD_16FF: u16 = 0;
static mut BYTE_16FE: u8 = 0;
static mut BYTE_3F70: u8 = 0;
static mut BYTE_3F6C: u8 = 0;
static mut WORD_3F6E: u16 = 0;

pub unsafe fn sub_575c() -> (u16, u8) {
    fn shr_rcl(ah: u8, al: u8) -> u8 {
        (al << 1) | ((ah >> 1) & 1)
    }

    sub_d92b();

    let mut al: u8 = 0;
    al = shr_rcl(BYTE_3DA4, al);
    al = shr_rcl(BYTE_3DD0, al);
    al = shr_rcl(BYTE_3DD8, al);
    al = shr_rcl(BYTE_3DD5, al);
    al = shr_rcl(BYTE_3DD3, al);

    BYTE_3F70 = match WORD_16FF {
        1 => {
            BYTE_1F58 = 0;
            al ^ BYTE_16FE
        }
        2 => {
            BYTE_1F58 = BYTE_3E2D;
            al | BYTE_3E2D & 0b1_0000
        }
        _ => {
            BYTE_1F58 = 0;
            al
        }
    };

    WORD_3F6E = u16::from_le_bytes([
        (BYTE_3F6C ^ BYTE_3F70) & BYTE_3F70,
        WORD_3F6E.to_le_bytes()[1],
    ]);
    BYTE_3F6C = BYTE_3F70;

    (WORD_3F6E, BYTE_3F70)
}
