use crate::{config::CFG, data::*};

const ARR_3D4E: [u8; 27] = [
    0x18, 0x12, 0x17, 0x10, 0x16, 0x11, 0x0C, 0x19, 0x0E, 0x13, 0x1A, 0x0F, 0x06, 0x0D, 0x08, 0x0B,
    0x03, 0x05, 0x01, 0x09, 0x15, 0x14, 0x04, 0x02, 0x00, 0x07, 0x0A,
];

/// D001: Convert RECS binary data back to ASCII game code.
pub unsafe fn sub_d001() {
    let mut arr = [0; 9];

    for (dst, src) in arr.iter_mut().zip(ARR_1F3E) {
        *dst = src as u8;
    }

    sub_d085(&mut arr);

    for (dst, al) in CFG.game_code.iter_mut().take(9).zip(arr) {
        *dst = match al {
            0..=12 => b'a' + al,
            13 => b' ',
            _ => b'a' + al - 1,
        };
    }

    CFG.game_code[9] = b'-';
    CFG.game_code[10] = b'0' + (WORD_1F50 / 10).min(9) as u8;
    CFG.game_code[11] = b'0' + (WORD_1F50 % 10) as u8;
}

/// D03E: Convert ASCII game code to RECS binary data.
pub unsafe fn sub_d03e() {
    let mut arr = [0; 9];

    for (dst, chr) in arr.iter_mut().zip(CFG.game_code.iter().take(9).copied()) {
        *dst = if chr == b' ' {
            13
        } else {
            let mut chr = chr;

            if chr < b'a' {
                chr = chr.wrapping_add(b' ');
            }

            if chr >= b'n' {
                chr = chr.wrapping_add(1);
            }

            chr.wrapping_sub(b'a')
        };
    }

    sub_d085(&mut arr);

    for (dst, src) in ARR_1F3E.iter_mut().zip(arr) {
        *dst = src as u16;
    }

    let tens = CFG.game_code[10].wrapping_sub(b'0') as u16;
    let ones = CFG.game_code[11].wrapping_sub(b'0') as u16;
    WORD_1F50 = tens * 10 + ones;
}

/// D085: Apply the game-code substitution table.
fn sub_d085(arr: &mut [u8; 9]) {
    for b in arr {
        *b = ARR_3D4E[*b as usize];
    }
}
