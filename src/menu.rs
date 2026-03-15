use alloc::vec::Vec;
use core::{mem, ptr};

use crate::{
    archive::{arc_unpack_res, load_resource_series, res_load},
    bitmap::draw_char_small,
    chars::{BmpVec, chr_load_and_prepare_few},
    config::CFG,
    data::*,
    game::sub_575c,
    hud::update_screen,
    sound::{sub_20ed, sub_209c},
    sprite::{Point, Size, draw_sprite},
    timer::{sub_d6f9, sub_d421, sub_d915},
    video::{fade_in, fade_out},
};

const KBD_ENTER: u8 = 0xD;
const KBD_LEFT: u8 = 0xCB;
const KBD_RIGHT: u8 = 0xCD;
const KBD_UP: u8 = 0xC8;
const KBD_DOWN: u8 = 0xD0;

static mut DWORD_FDC: usize = 0; // FDC
static mut WORD_FE0: u16 = 0; // FE0
const ARR176_FE2: [u8; 88] = [
    1, 1, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 1, 2, 2, 1, 0, 0, 0, 0, 1, 2, 2, 2, 1, 0, 0, 0,
    1, 2, 2, 2, 2, 1, 0, 0, 1, 2, 2, 2, 2, 2, 1, 0, 1, 1, 1, 2, 1, 1, 1, 0, 0, 0, 1, 2, 1, 0, 0, 0,
    0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0,
];
static mut ARR176_FE3: [u8; 88] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static mut WORD_1092: usize = 0; // 1092
static mut WORD_1094: u16 = 0; // 1094
static mut WORD_1096: u16 = 0; // 1096
static mut WORD_1098: u16 = 0; // 1098
static mut WORD_109A: u16 = 0; // 109A
const WORD_109C: [u8; 46] = [
    0x7C, 0x3D, 0, 0, 0xD8, 0x35, 0, 0, 0x8C, 0x3D, 2, 0, 0x7C, 0x3D, 0, 0, 0xD8, 0x35, 2, 0, 0x8C,
    0x3D, 1, 0, 0x7C, 0x3D, 0, 0, 0xD8, 0x35, 1, 0, 0x8C, 0x3D, 0, 0, 0x9C, 0x3D, 0, 0, 0xD8, 0x35,
    0, 0, 0xB3, 0x3D,
];

pub static mut BYTE_1F58: u8 = 0; // 1F58

pub static mut Y_POS: u16 = 0; // 1295
pub static mut X_POS: u16 = 0; // 1297
pub static mut WORD_1299: u16 = 0; // 1299
pub static mut STR_PTR: *const u8 = ptr::null(); // 129B
pub static mut WORD_129D: u16 = 0; // 129D
pub static mut WORD_129F: u16 = 0; // 129F
pub static mut WORD_12A1: u16 = 0; // 12A1
pub static mut BYTE_12A3: u8 = 0; // 12A3: bitmap index (3 in "C03")
pub static mut BYTE_12A4: u8 = 0; // 12A4
// db ? ; 12A5

static mut CURR_MENU_COL: u16 = 0; // F9C: X = 2nd column
static mut CURR_MENU_ROW: u16 = 0; // F9E: Y = 1st column

/// 35B5:
pub unsafe fn main_menu() {
    WORD_2E78 = 0;

    let [font3, font4, font5] = chr_load_and_prepare_few(&[3, 4, 5]);
    let [i14, i15] = load_resource_series(b'I', &[0x14, 0x15]);
    // &[0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1F, 0x20]

    CURR_MENU_COL = 1;
    CURR_MENU_ROW = 0;

    // main_menu_loop
    sub_d915();
    fade_out();

    draw_sprite(&res_unpack_with_pal(&i14), Size::full(), Point::start());

    let i15 = res_unpack_simple(&i15);
    draw_main_menu_items(&i15, &font3, &font4);
    fade_in();

    // loc_3600
    'loc_3600: loop {
        WORD_3E18 = 0;

        // loc_3606
        'loc_3606: loop {
            draw_main_menu_items(&i15, &font3, &font4);
            // handle_esc_key(loc_DC70);
            sub_d6f9();
            let (ax, bl) = sub_575c();

            if BYTE_1F58 != 0 {
                let cx = CURR_MENU_COL;
                CURR_MENU_COL = WORD_3E30 / 214;

                let dx = CURR_MENU_ROW;
                CURR_MENU_ROW = WORD_3E32 / 40;

                if (cx, dx) != (CURR_MENU_COL, CURR_MENU_ROW) {
                    continue 'loc_3600;
                }
            }

            // loc_364A
            if ax & 1 != 0 {
                CURR_MENU_COL = CURR_MENU_COL.saturating_sub(1);
                continue 'loc_3600;
            }

            // loc_365B
            if ax & 2 != 0 {
                CURR_MENU_COL = 2.min(CURR_MENU_COL + 1);
                continue 'loc_3600;
            }

            // loc_366C
            if ax & 8 != 0 {
                CURR_MENU_ROW = CURR_MENU_ROW.saturating_sub(1);
                continue 'loc_3600;
            }

            // loc_367D
            if ax & 4 != 0 {
                CURR_MENU_ROW = 4.min(CURR_MENU_ROW + 1);
                continue 'loc_3600;
            }

            // loc_3692
            // if ax & 48 != 0 {
            //     match CURR_MENU_ROW * 3 + CURR_MENU_COL {
            //         0 => sub_39e2(), // p1 name
            //         1 => start_game(),
            //         2 => loc_3a13(), // p2 name
            //         3 => CFG_DATA.p1_gears = CFG_DATA.p1_gears.next(),
            //         4 => CFG_DATA.race_type = CFG_DATA.race_type.next(),
            //         5 => CFG_DATA.p2_gears = CFG_DATA.p2_gears.next(),
            //         6 => CFG_DATA.p1_accel = CFG_DATA.p1_accel.next(),
            //         7 => CFG_DATA.course_type = CFG_DATA.course_type.next(),
            //         8 => CFG_DATA.p2_accel = CFG_DATA.p2_accel.next(),
            //         9 => sub_3b4f(), // settings
            //         10 => CFG_DATA.is_2pl_mode = !CFG_DATA.is_2pl_mode,
            //         11 => sound_setup_menu(),
            //         12 => sub_4135(), // RECS
            //         13 => loc_3A44(), // game code
            //         14 => define_menu(),
            //         _ => (),
            //     }
            //     continue 'loc_3600;
            // }

            continue 'loc_3606; // loc_3606
        }
    }
}

/// 36D2: Draw main menu items (P1 and P2 name, gears, etc.) and a red frame.
#[inline(never)]
unsafe fn draw_main_menu_items(i15: &[u8], font3: &BmpVec, font4: &BmpVec) {
    P1_GEARS = CFG.p1_gears;
    P2_GEARS = CFG.p2_gears;
    P1_ACCEL = CFG.p1_accel;
    P2_ACCEL = CFG.p2_accel;
    IS_2PL_MODE = CFG.is_2pl_mode;
    RACE_TYPE = CFG.race_type;

    unsafe fn print_text(text: &[u8], pos: Point, font: &BmpVec) {
        print_empty_string(12, pos, font);
        print_string_narrow(text, pos, font);
    }

    print_text(&CFG.p1_name, Point::xy(13, 21), font4);
    print_text(&CFG.p2_name, Point::xy(221, 21), font4);
    print_text(&CFG.game_code, Point::xy(117, 177), font3);

    draw_menu_item(&i15, Point::xy(6, 52), CFG.p1_gears as u8);
    draw_menu_item(&i15, Point::xy(6, 91), CFG.p1_accel as u8 + 2);
    draw_menu_item(&i15, Point::xy(214, 52), CFG.p2_gears as u8);
    draw_menu_item(&i15, Point::xy(214, 91), CFG.p2_accel as u8 + 2);
    // draw_menu_item(&i15, Point::xy(110, 52), CFG_DATA.race_type as u8 + 6);
    // draw_menu_item(&i15, Point::xy(110, 91), CFG_DATA.course_type as u8 + 10);
    // draw_menu_item(&i15, Point::xy(110, 130), CFG_DATA.is_2pl_mode as u8 + 8);

    sub_389e();
    draw_menu_frame(
        CURR_MENU_COL * 104 + 8,
        CURR_MENU_ROW * 39 + 7,
        Size::wh(95, 30),
    );
    sub_37ca();
}

/// 37CA:
#[inline(never)]
unsafe fn sub_37ca() {
    if CFG.word_16ff != 2 {
        update_screen();
        return;
    }

    DWORD_FDC = ((WORD_3E32 * 21) / 16 + WORD_3E30 / 2) as usize;
    WORD_FE0 = 200 - WORD_3E32;

    sub_3802();
    update_screen();
    sub_3834();
}

/// 3802:
unsafe fn sub_3802() {
    if WORD_FE0 == 0 {
        return;
    }

    let mut it = ARR176_FE3.iter_mut().zip(ARR176_FE2);

    for ch in WORD_63BC[DWORD_FDC..]
        .chunks_exact_mut(328)
        .take(WORD_FE0.max(11) as usize)
    {
        for px in &mut ch[..8] {
            let (dd, al) = it.next().unwrap();
            let al = al.wrapping_sub(1);

            if (al as i8) >= 0 {
                *dd = *px;
                *px = al.wrapping_neg();
            }
        }
    }
}

/// 3834:
unsafe fn sub_3834() {
    if WORD_FE0 == 0 {
        return;
    }

    let mut it = ARR176_FE3.iter().zip(ARR176_FE2);

    for ch in WORD_63BC[DWORD_FDC..]
        .chunks_exact_mut(328)
        .take(WORD_FE0.max(11) as usize)
    {
        for px in &mut ch[..8] {
            let (dd, al) = it.next().unwrap();

            if al != 0 {
                *px = *dd;
            }
        }
    }
}

/// 3864: Draw 104x26 sprite
// ;
// ; in:
// ; (ax, bx) - x, y
// ; cx - unpacked texture index
// ; word_63BA - unpacked data
#[inline(never)]
unsafe fn draw_menu_item(data: &[u8], pos: Point, ix: u8) {
    const MENU_ITEM_SIZE: Size = Size::wh(104, 26);

    draw_sprite(
        &data[ix as usize * MENU_ITEM_SIZE.size()..],
        MENU_ITEM_SIZE,
        pos,
    );
}

/// 3881: Clear screen area with an empty string
// ;
// ; in:
// ; (ax, bx) - x, y
// ; ch - unpacked texture index
// ; dx - length
// ;
#[inline(never)]
unsafe fn print_empty_string(len: u16, mut pos: Point, font: &BmpVec) {
    for _ in 0..len {
        draw_char_small(0, pos, font);
        pos.x += 7;
    }
}

/// 3891: Print a text with narrow spacing between letters
// ;
// ; in:
// ; (ax, bx) - x, y
// ; ch - unpacked texture index
// ; dx - length
// ; ds:si - string
// ;
#[inline(never)]
unsafe fn print_string_narrow(text: &[u8], mut pos: Point, font: &BmpVec) {
    for chr in text {
        draw_char_small(*chr, pos, font);
        pos.x += 7;
    }
}

/// 389E:
#[inline(never)]
unsafe fn sub_389e() {
    let cx = (WORD_1094 as u32 * 168) as u16;

    if cx == 0 {
        return;
    }

    for b in WORD_63BC[WORD_1092..].iter_mut().take(cx as usize * 2) {
        *b &= 0x1F;
    }
}

/// 38BD: Draw a red frame around current menu item
// ;
// ; in:
// ; ax - col * 104 + 8
// ; bx - row * 39 + 7
// ; cx - frame width (95 for standard frame)
// ; dx - frame height (30 for standard frame)
#[inline(never)]
unsafe fn draw_menu_frame(col: u16, row: u16, size: Size) {
    const FRAME_BORDER: u8 = 4;

    const TRANSPARENT: u8 = 0;
    const RED: u8 = 0x20;
    const MAROON: u8 = 0x40;
    const BLACK: u8 = 0x60;

    WORD_1094 = (size.height + 12) as u16;
    WORD_1092 = SCREEN_WIDTH * (row - 6) as usize + (col - 6) as usize;

    let mut di = WORD_1092;

    // draw top left corner
    WORD_63BC[di + 2] |= BLACK;
    WORD_63BC[di + 3] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH] |= TRANSPARENT;
    WORD_63BC[di + SCREEN_WIDTH + 1] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH + 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH + 3] |= RED;

    WORD_63BC[di + SCREEN_WIDTH * 2] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 1] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 2] |= RED;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 3] |= MAROON;

    WORD_63BC[di + SCREEN_WIDTH * 3] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 1] |= RED;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 3] |= BLACK;

    di += 4;

    // draw top line
    for _ in 0..size.width + 4 {
        WORD_63BC[di] |= BLACK;
        WORD_63BC[di + SCREEN_WIDTH] |= RED;
        WORD_63BC[di + SCREEN_WIDTH * 2] |= BLACK;
        di += 1;
    }

    // draw top right corner
    WORD_63BC[di] |= BLACK;
    WORD_63BC[di + 1] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH] |= RED;
    WORD_63BC[di + SCREEN_WIDTH + 1] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH + 2] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH + 3] |= TRANSPARENT;

    WORD_63BC[di + SCREEN_WIDTH * 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 1] |= RED;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 3] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH * 3] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 1] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 2] |= RED;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 3] |= BLACK;

    di += SCREEN_WIDTH * 4;

    // draw right vertical line
    for _ in 0..size.height + 4 {
        WORD_63BC[di + 1] |= BLACK;
        WORD_63BC[di + 2] |= RED;
        WORD_63BC[di + 3] |= BLACK;
        di += SCREEN_WIDTH;
    }

    // draw right bottom corner
    WORD_63BC[di] |= BLACK;
    WORD_63BC[di + 1] |= MAROON;
    WORD_63BC[di + 2] |= RED;
    WORD_63BC[di + 3] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH + 1] |= RED;
    WORD_63BC[di + SCREEN_WIDTH + 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH + 3] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH * 2] |= RED;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 1] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 2] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 3] |= TRANSPARENT;

    WORD_63BC[di + SCREEN_WIDTH * 3] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 1] |= BLACK;

    // draw bottom horizontal line
    for _ in 0..size.width + 4 {
        di -= 1;
        WORD_63BC[di + SCREEN_WIDTH] |= BLACK;
        WORD_63BC[di + SCREEN_WIDTH * 2] |= RED;
        WORD_63BC[di + SCREEN_WIDTH * 3] |= BLACK;
    }

    di -= 4;

    // draw left bottom corner
    WORD_63BC[di] |= BLACK;
    WORD_63BC[di + 1] |= RED;
    WORD_63BC[di + 2] |= MAROON;
    WORD_63BC[di + 3] |= BLACK;

    WORD_63BC[di + SCREEN_WIDTH] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH + 1] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH + 2] |= RED;
    WORD_63BC[di + SCREEN_WIDTH + 3] |= MAROON;

    WORD_63BC[di + SCREEN_WIDTH * 2] |= TRANSPARENT;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 1] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 2] |= MAROON;
    WORD_63BC[di + SCREEN_WIDTH * 2 + 3] |= RED;

    WORD_63BC[di + SCREEN_WIDTH * 3 + 2] |= BLACK;
    WORD_63BC[di + SCREEN_WIDTH * 3 + 3] |= BLACK;

    // draw left vertical line
    for _ in 0..size.height + 4 {
        di -= SCREEN_WIDTH;
        WORD_63BC[di] |= BLACK;
        WORD_63BC[di + 1] |= RED;
        WORD_63BC[di + 2] |= BLACK;
    }
}

/// 39E2:
pub unsafe fn sub_39e2() {
    X_POS = 13;
    Y_POS = 21;

    BYTE_12A3 = 4;
    STR_PTR = core::mem::transmute(CFG.p1_name.as_ptr());
    WORD_129D = 12;
    WORD_129F = 0;
    WORD_12A1 = 11;
    BYTE_12A4 = 3;

    // handle_input();
}

/// 3A13:
unsafe fn loc_3a13() {
    X_POS = 221;
    Y_POS = 21;
    BYTE_12A3 = 4;
    STR_PTR = core::mem::transmute(CFG.p2_name.as_ptr());
    WORD_129D = 12;
    WORD_129F = 0;
    WORD_12A1 = 11;
    BYTE_12A4 = 3;

    // handle_input();
}

/// 3A44:
unsafe fn loc_3a44() {
    loop {
        X_POS = 117;
        Y_POS = 177;
        BYTE_12A3 = 3;
        // STR_PTR = &CFG_DATA.game_code;
        sub_3a66();

        if CFG.game_code[0] != b' ' {
            break;
        }
    }
}

/// 3A66:
unsafe fn sub_3a66() {
    WORD_129D = 12;

    // loc_3A6C
    loop {
        WORD_129F = 0;
        WORD_12A1 = 8;
        BYTE_12A4 = 2;
        // let al = handle_input();

        // if al != KBD_LEFT && al != KBD_RIGHT {
        //     return;
        // }

        if WORD_1299 == 0 {
            continue;
        }

        WORD_129F = 10;
        WORD_12A1 = 11;
        BYTE_12A4 = 5;
        // let al = handle_input();

        // if al != KBD_LEFT && al != KBD_RIGHT {
        //     return;
        // }
    }
}

/// 3AAC: Sound setup menu
pub unsafe fn sound_setup_menu() {
    // add sp, 2
    fade_out();

    // res_unpack_with_pal(&i20);
    let bgr = res_load(b'I', 0x20);
    PALETTE.copy_from_slice(&bgr[bgr.len() - PALETTE.len()..]);

    draw_sprite(&bgr, Size::full(), Point::start());
    sub_3b30();
    fade_in();

    // loc_3ACE
    loop {
        sub_3b30();
        sub_d6f9();

        // let al = handle_esc_key(main_menu_loop); // TODO:

        let al = sub_575c().0 as u8;

        if BYTE_1F58 != 0 {
            let prev = CFG.snd_setting;
            CFG.snd_setting = mem::transmute(((WORD_3E32 / 40) & 0xFF) as u8);

            if CFG.snd_setting != prev {
                continue;
            }
        }

        if (al & 8) != 0 {
            CFG.snd_setting = mem::transmute((CFG.snd_setting as u8).saturating_sub(1));
            continue;
        }

        if (al & 4) != 0 {
            CFG.snd_setting = mem::transmute(4.min(CFG.snd_setting as u8 + 1));
            continue;
        }

        if (al & 0x30) != 0 {
            sub_209c();
            CFG.word_16f6 = CFG.snd_setting;
            sub_20ed();
            break; // jmp main_menu_loop
        }
    }
}

/// 3B30:
unsafe fn sub_3b30() {
    sub_389e();
    draw_menu_frame(112, CFG.snd_setting as u16 * 39 + 8, Size::wh(95, 30));
    sub_37ca();
}

/// 454C:
pub unsafe fn handle_input(input: &mut [u8], pos: Point, font: &BmpVec) -> u8 {
    sub_d421();

    if (BYTE_12A4 & 4) == 0 {
        for c in input[WORD_12A1 as usize..].iter().rev() {
            if *c == b' ' {
                break;
            }

            WORD_1299 -= 1;

            if WORD_1299 <= WORD_129F {
                break;
            }
        }
    }

    let c = loop {
        draw_user_input(input, pos, font);
        sub_d6f9();

        match sub_d421() {
            // enter
            c @ (KBD_LEFT | KBD_RIGHT | KBD_UP | KBD_DOWN | KBD_ENTER) => {
                break c;
            }
            // backspace
            8 => {
                if (BYTE_12A4 & 4) != 0 {
                    continue;
                }

                if WORD_1299 <= WORD_129F {
                    continue;
                }

                WORD_1299 -= 1;
                input[WORD_1299 as usize] = b' ';
            }
            c @ (b' ' | b'A'..=b'Z' | b'0'..=b'9') => {
                let mask = if matches!(c, b'0'..=b'9') { 1 } else { 2 };

                if (BYTE_12A4 & mask) == 0 {
                    continue;
                }

                // loc_45ED
                if (BYTE_12A4 & 4) == 0 {
                    if WORD_1299 > WORD_12A1 {
                        continue;
                    }

                    input[WORD_1299 as usize] = c;
                    WORD_1299 += 1;
                } else {
                    input[WORD_1299 as usize] = c;
                    WORD_1299 += 1;

                    if WORD_1299 > WORD_12A1 {
                        WORD_1299 = WORD_129F;
                    }
                }
            }
            _ => continue,
        }
    };

    // exit_462E
    sub_465e(pos, font);
    print_string_narrow(input, pos, font);

    c
}

/// 4637: Draw user's secret code and cursor
unsafe fn draw_user_input(input: &[u8], pos: Point, font: &BmpVec) {
    sub_465e(pos, font);
    print_string_narrow(input, pos, font);

    let pos = Point::xy(pos.x + WORD_1299.min(WORD_129D - 1) as usize * 7, pos.y + 9);
    draw_char_small(0x2E, pos, font); // 0x2E = '.' cursor

    update_screen();
}

/// 465E:
unsafe fn sub_465e(pos: Point, font: &BmpVec) {
    print_empty_string(WORD_129D, pos, font);
    print_empty_string(WORD_129D, Point::xy(pos.x, pos.y + 3), font)
}

/// 478D: Unpack resource
pub unsafe fn res_unpack_simple(data: &[u8]) -> Vec<u8> {
    arc_unpack_res(data).unwrap()
}

/// 47A2: Unpack resource with palette
pub unsafe fn res_unpack_with_pal(data: &[u8]) -> Vec<u8> {
    let mut data = arc_unpack_res(data).unwrap();
    let pal_start = data.len() - PALETTE.len();

    PALETTE.copy_from_slice(&data[pal_start..]);

    data.truncate(pal_start);
    data
}
