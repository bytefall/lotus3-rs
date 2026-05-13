use alloc::vec::Vec;
use core::mem::transmute;

use crate::{
    archive::{arc_unpack_res, load_resource_series},
    bitmap::draw_char_small,
    chars::{BmpVec, chr_load_and_prepare_few},
    config::{CFG, Controls, Player, SoundSettings},
    data::*,
    dos::sub_da94,
    game::{handle_esc_key, sub_575c},
    hud::{VGA_DBL_BUF, VGA_HEIGHT, VGA_WIDTH, print_string, update_screen},
    sound::{sub_20ed, sub_209c},
    sprite::{Point, Size, draw_sprite},
    state::{sub_d001, sub_d03e},
    timer::{sub_d6f9, sub_d421, sub_d915},
    video::{fade_in, fade_in2, fade_out, fade_out2, sub_d097},
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
static mut WORD_122F: u16 = 0; // 122F: Define menu selected row
static mut WORD_1231: u16 = 0; // 1231: RECS menu selected column
static mut WORD_1233: u16 = 0; // 1233: RECS menu selected row
static mut WORD_1273: u16 = 0; // 1273: RECS value editor previous input

pub static mut BYTE_1F58: u8 = 0; // 1F58

pub static mut WORD_1299: u16 = 0; // 1299
pub static mut WORD_129D: u16 = 0; // 129D
pub static mut WORD_129F: u16 = 0; // 129F
pub static mut WORD_12A1: u16 = 0; // 12A1
pub static mut BYTE_12A4: u8 = 0; // 12A4
// db ? ; 12A5

static mut CURR_MENU_COL: u16 = 0; // F9C: X = 2nd column
static mut CURR_MENU_ROW: u16 = 0; // F9E: Y = 1st column

// 359A: Init before going to main menu.
pub unsafe fn enter_main_menu() {
    sub_d915();
    fade_out();

    WORD_63BA = WORD_5166;

    // TODO:

    main_menu();
}

/// 35B5: Main menu.
pub unsafe fn main_menu() {
    const MAIN_MENU_FNT_IDS: [u8; 3] = [3, 4, 5];
    const MAIN_MENU_RES_IDS: [u8; 10] =
        [0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1F, 0x20];

    WORD_2E78 = 0;

    let [font3, font4, font5] = chr_load_and_prepare_few(&MAIN_MENU_FNT_IDS);
    let [i14, i15, i16, i17, i18, i19, i1a, i1b, i1f, i20] =
        load_resource_series(b'I', &MAIN_MENU_RES_IDS);

    CURR_MENU_COL = 1;
    CURR_MENU_ROW = 0;

    'main: loop {
        sub_d915();
        fade_out();

        draw_sprite(&res_unpack_with_pal(&i14), Size::full(), Point::start());

        let i15 = res_unpack_simple(&i15);
        draw_main_menu_items(&i15, &font3, &font4);
        fade_in();

        // loc_3600
        loop {
            WORD_3E18 = 0;

            // loc_3606
            loop {
                draw_main_menu_items(&i15, &font3, &font4);

                if handle_esc_key() {
                    break 'main;
                }

                sub_d6f9();
                let al = sub_575c().0 as u8;

                if BYTE_1F58 != 0 {
                    let cx = CURR_MENU_COL;
                    CURR_MENU_COL = WORD_3E30 / 214;

                    let dx = CURR_MENU_ROW;
                    CURR_MENU_ROW = WORD_3E32 / 40;

                    if (cx, dx) != (CURR_MENU_COL, CURR_MENU_ROW) {
                        break;
                    }
                }

                if al & 1 != 0 {
                    CURR_MENU_COL = CURR_MENU_COL.saturating_sub(1);
                    break;
                }

                if al & 2 != 0 {
                    CURR_MENU_COL = 2.min(CURR_MENU_COL + 1);
                    break;
                }

                if al & 8 != 0 {
                    CURR_MENU_ROW = CURR_MENU_ROW.saturating_sub(1);
                    break;
                }

                if al & 4 != 0 {
                    CURR_MENU_ROW = 4.min(CURR_MENU_ROW + 1);
                    break;
                }

                if al & 0x30 != 0 {
                    match (CURR_MENU_ROW, CURR_MENU_COL) {
                        (0, 0) => sub_39e2(&font4),
                        (0, 1) => {
                            // start_game();
                            continue 'main;
                        }
                        (0, 2) => loc_3a13(&font4),
                        (1, 0) => CFG.p1_gears = CFG.p1_gears.next(),
                        (1, 1) => CFG.race_type = CFG.race_type.next(),
                        (1, 2) => CFG.p2_gears = CFG.p2_gears.next(),
                        (2, 0) => CFG.p1_accel = CFG.p1_accel.next(),
                        (2, 1) => CFG.course_type = CFG.course_type.next(),
                        (2, 2) => CFG.p2_accel = CFG.p2_accel.next(),
                        (3, 0) => {
                            settings_menu(&i19, &i1a, &i1b, &i1f, &font3);
                            continue 'main;
                        }
                        (3, 1) => CFG.is_2pl_mode = !CFG.is_2pl_mode,
                        (3, 2) => {
                            sound_setup_menu(&i20);
                            continue 'main;
                        }
                        (4, 0) => {
                            recs_menu(&i17, &i18, &font3, &font4, &font5);
                            continue 'main;
                        }
                        (4, 1) => loc_3a44(&font3),
                        (4, 2) => {
                            define_menu(&i16, &font4);
                            continue 'main;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

/// 36D2: Draw main menu items (P1 and P2 name, gears, etc.) and a red frame.
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

    draw_menu_item(i15, Point::xy(6, 52), CFG.p1_gears as u8);
    draw_menu_item(i15, Point::xy(6, 91), CFG.p1_accel as u8 + 2);
    draw_menu_item(i15, Point::xy(214, 52), CFG.p2_gears as u8);
    draw_menu_item(i15, Point::xy(214, 91), CFG.p2_accel as u8 + 2);
    draw_menu_item(i15, Point::xy(110, 52), CFG.race_type as u8 + 6);
    draw_menu_item(i15, Point::xy(110, 91), CFG.course_type as u8 + 10);
    draw_menu_item(i15, Point::xy(110, 130), CFG.is_2pl_mode as u8 + 8);

    sub_389e();
    draw_menu_frame(
        CURR_MENU_COL * 104 + 8,
        CURR_MENU_ROW * 39 + 7,
        Size::wh(95, 30),
    );
    sub_37ca();
}

/// 37CA:
unsafe fn sub_37ca() {
    if CFG.word_16ff != Controls::Mouse {
        update_screen();
        return;
    }

    let x = (WORD_3E30 / 2).min((VGA_WIDTH - 8) as u16) as usize;
    let y = WORD_3E32.min(VGA_HEIGHT as u16) as usize;

    DWORD_FDC = y * VGA_WIDTH + x;
    WORD_FE0 = (VGA_HEIGHT as u16).saturating_sub(WORD_3E32);

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

    for row in 0..WORD_FE0.min(11) as usize {
        let start = DWORD_FDC + row * VGA_WIDTH;

        if start + 8 > VGA_DBL_BUF.len() {
            break;
        }

        for px in &mut VGA_DBL_BUF[start..start + 8] {
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

    for row in 0..WORD_FE0.min(11) as usize {
        let start = DWORD_FDC + row * VGA_WIDTH;

        if start + 8 > VGA_DBL_BUF.len() {
            break;
        }

        for px in &mut VGA_DBL_BUF[start..start + 8] {
            let (dd, al) = it.next().unwrap();

            if al != 0 {
                *px = *dd;
            }
        }
    }
}

/// 3864: Draw 104x26 sprite
unsafe fn draw_menu_item(data: &[u8], pos: Point, ix: u8) {
    const MENU_ITEM_SIZE: Size = Size::wh(104, 26);

    draw_sprite(
        &data[ix as usize * MENU_ITEM_SIZE.size()..],
        MENU_ITEM_SIZE,
        pos,
    );
}

/// 3881: Clear screen area with an empty string
unsafe fn print_empty_string(len: u16, mut pos: Point, font: &BmpVec) {
    for _ in 0..len {
        draw_char_small(0, pos, font);
        pos.x += 7;
    }
}

/// 3891: Print a text with narrow spacing between letters
unsafe fn print_string_narrow(text: &[u8], mut pos: Point, font: &BmpVec) {
    for chr in text {
        draw_char_small(*chr, pos, font);
        pos.x += 7;
    }
}

/// 389E:
unsafe fn sub_389e() {
    let cx = (WORD_1094 as u32 * 168) as u16;

    if cx == 0 {
        return;
    }

    for b in VGA_DBL_BUF[WORD_1092..].iter_mut().take(cx as usize * 2) {
        *b &= 0x1F;
    }
}

/// 38BD: Draw a red frame around current menu item
unsafe fn draw_menu_frame(col: u16, row: u16, size: Size) {
    const FRAME_BORDER: u8 = 4;

    const TRANSPARENT: u8 = 0;
    const RED: u8 = 0x20;
    const MAROON: u8 = 0x40;
    const BLACK: u8 = 0x60;

    WORD_1094 = (size.height + 12) as u16;
    WORD_1092 = VGA_WIDTH * (row - 6) as usize + (col - 6) as usize;

    let mut di = WORD_1092;

    // draw top left corner
    VGA_DBL_BUF[di + 2] |= BLACK;
    VGA_DBL_BUF[di + 3] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH] |= TRANSPARENT;
    VGA_DBL_BUF[di + VGA_WIDTH + 1] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH + 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH + 3] |= RED;

    VGA_DBL_BUF[di + VGA_WIDTH * 2] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 1] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 2] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 3] |= MAROON;

    VGA_DBL_BUF[di + VGA_WIDTH * 3] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 1] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 3] |= BLACK;

    di += 4;

    // draw top line
    for _ in 0..size.width + 4 {
        VGA_DBL_BUF[di] |= BLACK;
        VGA_DBL_BUF[di + VGA_WIDTH] |= RED;
        VGA_DBL_BUF[di + VGA_WIDTH * 2] |= BLACK;
        di += 1;
    }

    // draw top right corner
    VGA_DBL_BUF[di] |= BLACK;
    VGA_DBL_BUF[di + 1] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH + 1] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH + 2] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH + 3] |= TRANSPARENT;

    VGA_DBL_BUF[di + VGA_WIDTH * 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 1] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 3] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH * 3] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 1] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 2] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 3] |= BLACK;

    di += VGA_WIDTH * 4;

    // draw right vertical line
    for _ in 0..size.height + 4 {
        VGA_DBL_BUF[di + 1] |= BLACK;
        VGA_DBL_BUF[di + 2] |= RED;
        VGA_DBL_BUF[di + 3] |= BLACK;
        di += VGA_WIDTH;
    }

    // draw right bottom corner
    VGA_DBL_BUF[di] |= BLACK;
    VGA_DBL_BUF[di + 1] |= MAROON;
    VGA_DBL_BUF[di + 2] |= RED;
    VGA_DBL_BUF[di + 3] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH + 1] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH + 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH + 3] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH * 2] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 1] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 2] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 3] |= TRANSPARENT;

    VGA_DBL_BUF[di + VGA_WIDTH * 3] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 1] |= BLACK;

    // draw bottom horizontal line
    for _ in 0..size.width + 4 {
        di -= 1;
        VGA_DBL_BUF[di + VGA_WIDTH] |= BLACK;
        VGA_DBL_BUF[di + VGA_WIDTH * 2] |= RED;
        VGA_DBL_BUF[di + VGA_WIDTH * 3] |= BLACK;
    }

    di -= 4;

    // draw left bottom corner
    VGA_DBL_BUF[di] |= BLACK;
    VGA_DBL_BUF[di + 1] |= RED;
    VGA_DBL_BUF[di + 2] |= MAROON;
    VGA_DBL_BUF[di + 3] |= BLACK;

    VGA_DBL_BUF[di + VGA_WIDTH] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH + 1] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH + 2] |= RED;
    VGA_DBL_BUF[di + VGA_WIDTH + 3] |= MAROON;

    VGA_DBL_BUF[di + VGA_WIDTH * 2] |= TRANSPARENT;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 1] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 2] |= MAROON;
    VGA_DBL_BUF[di + VGA_WIDTH * 2 + 3] |= RED;

    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 2] |= BLACK;
    VGA_DBL_BUF[di + VGA_WIDTH * 3 + 3] |= BLACK;

    // draw left vertical line
    for _ in 0..size.height + 4 {
        di -= VGA_WIDTH;
        VGA_DBL_BUF[di] |= BLACK;
        VGA_DBL_BUF[di + 1] |= RED;
        VGA_DBL_BUF[di + 2] |= BLACK;
    }
}

/// 39E2: Edit P1 name.
pub unsafe fn sub_39e2(font4: &BmpVec) {
    WORD_129D = 12;
    WORD_129F = 0;
    WORD_12A1 = 11;
    BYTE_12A4 = 3;
    handle_input(&mut CFG.p1_name, Point::xy(13, 21), font4);
}

/// 3A13: Edit P2 name.
unsafe fn loc_3a13(font4: &BmpVec) {
    WORD_129D = 12;
    WORD_129F = 0;
    WORD_12A1 = 11;
    BYTE_12A4 = 3;
    handle_input(&mut CFG.p2_name, Point::xy(221, 21), font4);
}

/// 3A44: Edit game code.
unsafe fn loc_3a44(font3: &BmpVec) {
    loop {
        sub_3a66(&mut CFG.game_code, Point::xy(117, 177), font3);

        if CFG.game_code[0] != b' ' {
            break;
        }
    }
}

/// 3A66: Edit game code or game keys.
unsafe fn sub_3a66(input: &mut [u8], pos: Point, font: &BmpVec) -> u8 {
    WORD_129D = 12;

    loop {
        WORD_129F = 0; // start pos
        WORD_12A1 = 8; // end pos
        BYTE_12A4 = 2;
        let chr = handle_input(input, pos, font);

        if !matches!(chr, KBD_LEFT | KBD_RIGHT) {
            return chr;
        }

        if WORD_1299 == 0 {
            continue;
        }

        WORD_129F = 10;
        WORD_12A1 = 11;
        BYTE_12A4 = 5;
        let chr = handle_input(input, pos, font);

        if !matches!(chr, KBD_LEFT | KBD_RIGHT) {
            return chr;
        }
    }
}

/// 3AAC: Sound setup menu
pub unsafe fn sound_setup_menu(i20: &[u8]) {
    fade_out();

    draw_sprite(&res_unpack_with_pal(i20), Size::full(), Point::start());
    sub_3b30();
    fade_in();

    // loc_3ACE
    loop {
        sub_3b30();
        sub_d6f9();

        if handle_esc_key() {
            break;
        }

        let al = sub_575c().0 as u8;

        if BYTE_1F58 != 0 {
            let prev = CFG.snd_setting;
            CFG.snd_setting = transmute::<u16, SoundSettings>((WORD_3E32 / 40) & 0xFF);

            if CFG.snd_setting != prev {
                continue;
            }
        }

        if al & 8 != 0 {
            CFG.snd_setting =
                transmute::<u16, SoundSettings>((CFG.snd_setting as u16).saturating_sub(1));
            continue;
        }

        if al & 4 != 0 {
            CFG.snd_setting =
                transmute::<u16, SoundSettings>((CFG.snd_setting as u16).saturating_add(1).min(4));
            continue;
        }

        if al & 0x30 != 0 {
            sub_209c();
            CFG.word_16f6 = CFG.snd_setting;
            sub_20ed();
            break;
        }
    }
}

/// 3B30:
unsafe fn sub_3b30() {
    sub_389e();
    draw_menu_frame(112, CFG.snd_setting as u16 * 39 + 8, Size::wh(95, 30));
    sub_37ca();
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SettingsMenuAction {
    Continue,
    Restart,
    Exit,
}

const KEY_PROMPTS: [[(&str, u8); 1]; 6] = [
    [("PRESS KEY FOR LEFT", 96)],
    [("PRESS KEY FOR RIGHT", 96)],
    [("PRESS KEY FOR CHANGE UP A GEAR", 96)],
    [("PRESS KEY FOR CHANGE DOWN A GEAR", 96)],
    [("PRESS KEY FOR ACCELERATE", 96)],
    [("PRESS KEY FOR BRAKE", 96)],
];

const DUPLICATE_KEY_PROMPT: [(&str, u8); 2] = [
    ("THAT KEY IS ALREADY USED", 90),
    ("PRESS SPACE THEN SELECT ANOTHER", 102),
];

const JOYSTICK_PROMPTS: [[(&str, u8); 2]; 3] = [
    [("PUSH JOYSTICK TO TOP LEFT", 90), ("THEN PRESS FIRE", 102)],
    [
        ("PUSH JOYSTICK TO BOTTOM RIGHT", 90),
        ("THEN PRESS FIRE", 102),
    ],
    [("PUSH JOYSTICK TO CENTRE", 90), ("THEN PRESS FIRE", 102)],
];

/// 3B4F: Settings menu.
pub unsafe fn settings_menu(i19: &[u8], i1a: &[u8], i1b: &[u8], i1f: &[u8], font3: &BmpVec) {
    WORD_1096 = 1;
    WORD_1098 = 3;

    'loc_3b5e: loop {
        fade_out();

        draw_sprite(&res_unpack_with_pal(i19), Size::full(), Point::start());

        let items = res_unpack_simple(i1a);
        sub_3c87(&items);
        fade_in();

        // loc_3B83
        loop {
            sub_3c87(&items);
            sub_d6f9();

            if handle_esc_key() {
                return;
            }

            let al = sub_575c().0 as u8;

            if BYTE_1F58 != 0 {
                let cx = (WORD_3E30 / 214).min(2);
                let mut dx = if WORD_3E32 < 22 {
                    0
                } else {
                    ((WORD_3E32 - 22) / 39).min(3)
                };

                if dx >= 3 {
                    dx = 3;
                }

                if dx == 3 || cx != 1 {
                    let prev = (WORD_1096, WORD_1098);
                    WORD_1096 = cx;
                    WORD_1098 = dx;

                    if prev != (WORD_1096, WORD_1098) {
                        continue;
                    }
                }
            }

            if al & 0x30 != 0 {
                match sub_3c63(i1b, i1f, font3) {
                    SettingsMenuAction::Continue => continue,
                    SettingsMenuAction::Restart => continue 'loc_3b5e,
                    SettingsMenuAction::Exit => return,
                }
            }

            if al & 1 != 0 {
                if WORD_1096 != 0 {
                    if WORD_1098 == 3 {
                        WORD_1096 -= 1;
                    } else {
                        WORD_1096 = 0;
                    }
                }
                continue;
            }

            if al & 2 != 0 {
                if WORD_1096 < 2 {
                    if WORD_1098 == 3 {
                        WORD_1096 += 1;
                    } else {
                        WORD_1096 = 2;
                    }
                }
                continue;
            }

            if al & 8 != 0 {
                if WORD_1096 != 1 && WORD_1098 != 0 {
                    WORD_1098 -= 1;
                }
                continue;
            }

            if al & 4 != 0 {
                if WORD_1098 < 3 {
                    WORD_1098 += 1;
                }
                continue;
            }
        }
    }
}

/// 3C63: Dispatch selected settings menu item.
unsafe fn sub_3c63(i1b: &[u8], i1f: &[u8], font3: &BmpVec) -> SettingsMenuAction {
    match (WORD_1098, WORD_1096) {
        (0, 0) => {
            sub_3d7c(Controls::Keyboard);
            SettingsMenuAction::Continue
        }
        (0, 2) => {
            sub_3d8c(Controls::Keyboard);
            SettingsMenuAction::Continue
        }
        (1, 0) => {
            sub_3d7c(Controls::Mouse);
            SettingsMenuAction::Continue
        }
        (1, 2) => {
            sub_3d8c(Controls::Mouse);
            SettingsMenuAction::Continue
        }
        (2, 0) => {
            sub_3d7c(Controls::Joystick);
            SettingsMenuAction::Continue
        }
        (2, 2) => {
            sub_3d8c(Controls::Joystick);
            SettingsMenuAction::Continue
        }
        (3, 0) => sub_3d9c(Player::One, i1b, i1f, font3),
        (3, 2) => sub_3d9c(Player::Two, i1b, i1f, font3),
        _ => SettingsMenuAction::Exit,
    }
}

/// 3C87: Draw settings menu items and current frame.
unsafe fn sub_3c87(items: &[u8]) {
    draw_middle_menu_item(
        items,
        Point::xy(8, 25),
        if CFG.word_16ff == Controls::Keyboard {
            0
        } else {
            1
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(8, 64),
        if CFG.word_16ff == Controls::Mouse {
            2
        } else {
            3
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(8, 103),
        if CFG.word_16ff == Controls::Joystick {
            4
        } else {
            5
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(8, 142),
        match CFG.word_16ff {
            Controls::Keyboard => 12,
            Controls::Mouse => 13,
            Controls::Joystick => 14,
        },
    );

    draw_middle_menu_item(
        items,
        Point::xy(216, 25),
        if CFG.word_1716 == Controls::Keyboard {
            6
        } else {
            7
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(216, 64),
        if CFG.word_1716 == Controls::Mouse {
            8
        } else {
            9
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(216, 103),
        if CFG.word_1716 == Controls::Joystick {
            10
        } else {
            11
        },
    );
    draw_middle_menu_item(
        items,
        Point::xy(216, 142),
        match CFG.word_1716 {
            Controls::Keyboard => 15,
            Controls::Mouse => 16,
            Controls::Joystick => 17,
        },
    );

    sub_389e();
    draw_menu_frame(WORD_1096 * 104 + 8, WORD_1098 * 39 + 27, Size::wh(95, 30));
    sub_37ca();
}

/// 3D5F: Draw 96x34 settings-menu sprite.
unsafe fn draw_middle_menu_item(data: &[u8], pos: Point, ix: u8) {
    const MENU_ITEM_SIZE: Size = Size::wh(96, 34);

    let start = ix as usize * MENU_ITEM_SIZE.size();
    let end = start + MENU_ITEM_SIZE.size();

    if end <= data.len() {
        draw_sprite(&data[start..end], MENU_ITEM_SIZE, pos);
    }
}

/// 3D7C: Select P1 control mode.
unsafe fn sub_3d7c(ctrl: Controls) {
    if ctrl != Controls::Mouse || CFG.word_1716 != ctrl {
        CFG.word_16ff = ctrl;
    }
}

/// 3D8C: Select P2 control mode.
unsafe fn sub_3d8c(ctrl: Controls) {
    if ctrl != Controls::Mouse || CFG.word_16ff != ctrl {
        CFG.word_1716 = ctrl;
    }
}

/// 3D9C / 3DB3: Configure selected player's active control mode.
unsafe fn sub_3d9c(player: Player, i1b: &[u8], i1f: &[u8], font3: &BmpVec) -> SettingsMenuAction {
    match match player {
        Player::One => CFG.word_16ff,
        Player::Two => CFG.word_1716,
    } {
        Controls::Keyboard => loc_3e63(player, i1b, font3),
        Controls::Joystick => loc_3f21(player, i1b, font3),
        Controls::Mouse => loc_3dca(i1f),
    }
}

/// 3DCA: Mouse options menu.
unsafe fn loc_3dca(i1f: &[u8]) -> SettingsMenuAction {
    fade_out();
    draw_sprite(&res_unpack_with_pal(i1f), Size::full(), Point::start());
    sub_3e43();
    fade_in();

    loop {
        sub_3e43();
        sub_d6f9();

        if handle_esc_key() {
            return SettingsMenuAction::Exit;
        }

        let al = sub_575c().0 as u8;

        if BYTE_1F58 != 0 {
            let prev = CFG.word_16f2;
            CFG.word_16f2 = if WORD_3E30 >= 0x140 { 1 } else { 0 };

            if prev != CFG.word_16f2 {
                continue;
            }
        }

        if al & 1 != 0 {
            if CFG.word_16f2 != 0 {
                CFG.word_16f2 -= 1;
            }
            continue;
        }

        if al & 2 != 0 {
            if CFG.word_16f2 < 1 {
                CFG.word_16f2 += 1;
            }
            continue;
        }

        if al & 0x30 != 0 {
            return SettingsMenuAction::Restart;
        }
    }
}

/// 3E43: Mouse options menu frame.
unsafe fn sub_3e43() {
    sub_389e();
    draw_menu_frame(
        if CFG.word_16f2 == 0 { 56 } else { 160 },
        65,
        Size::wh(95, 70),
    );
    sub_37ca();
}

/// 3E63: Define keyboard controls.
unsafe fn loc_3e63(player: Player, i1b: &[u8], font3: &BmpVec) -> SettingsMenuAction {
    fade_out();
    let bg = res_unpack_with_pal(i1b);
    draw_sprite(&bg, Size::full(), Point::start());
    update_screen();
    fade_in();

    for ix in 0..6 {
        set_player_key(player, ix, 0x7F);
    }
    sub_da94();

    for (ix, prompt) in KEY_PROMPTS.iter().enumerate() {
        loop {
            wait_key_release();
            sub_3ef4(&bg, font3, prompt);

            let scan = wait_key_press();
            let used = CFG.arr_172c[scan as usize] != 0;
            CFG.arr_172c[scan as usize] = BYTE_3D86 as u8;

            if used {
                sub_3ef4(&bg, font3, &DUPLICATE_KEY_PROMPT);
                wait_for_space_key();
                continue;
            }

            set_player_key(player, ix, scan);
            break;
        }
    }

    SettingsMenuAction::Restart
}

unsafe fn wait_key_release() {
    while BYTE_3D86 {
        sub_d6f9();
    }
}

unsafe fn wait_key_press() -> u8 {
    loop {
        sub_d6f9();

        if BYTE_3D86 && BYTE_3D87 < 128 {
            return BYTE_3D87;
        }
    }
}

unsafe fn wait_for_space_key() {
    const SPACE_SCAN_CODE: u8 = 0x39;

    loop {
        wait_key_release();

        if wait_key_press() == SPACE_SCAN_CODE {
            return;
        }
    }
}

unsafe fn set_player_key(player: Player, ix: usize, scan: u8) {
    match player {
        Player::One => CFG.p1_kbd[ix] = scan,
        Player::Two => CFG.p2_kbd[ix] = scan,
    }
}

/// 3EF4: Redraw control-definition background and prompt text.
unsafe fn sub_3ef4(bg: &[u8], font3: &BmpVec, lines: &[(&str, u8)]) {
    draw_sprite(bg, Size::full(), Point::start());
    sub_3f0f(lines, font3);
    update_screen();
}

/// 3F0F: Print prompt text lines.
unsafe fn sub_3f0f(lines: &[(&str, u8)], font3: &BmpVec) {
    for (text, y) in lines {
        print_string(text.as_bytes(), *y as usize, font3);
    }
}

/// 3F21: Joystick calibration.
unsafe fn loc_3f21(player: Player, i1b: &[u8], font3: &BmpVec) -> SettingsMenuAction {
    fade_out();
    let bg = res_unpack_with_pal(i1b);
    draw_sprite(&bg, Size::full(), Point::start());
    update_screen();
    fade_in();

    let saved_word_16fa = CFG.word_16fa;
    CFG.word_16fa = 0xFFFF;

    let top_left = match sub_3faf(player, &bg, font3, &JOYSTICK_PROMPTS[0], saved_word_16fa) {
        Ok(point) => point,
        Err(action) => return action,
    };
    let bottom_right = match sub_3faf(player, &bg, font3, &JOYSTICK_PROMPTS[1], saved_word_16fa) {
        Ok(point) => point,
        Err(action) => return action,
    };
    let centre = match sub_3faf(player, &bg, font3, &JOYSTICK_PROMPTS[2], saved_word_16fa) {
        Ok(point) => point,
        Err(action) => return action,
    };

    set_joystick_calibration(player, top_left, bottom_right, centre);

    let min_timeout = bottom_right.0.max(bottom_right.1);
    CFG.word_16fa = saved_word_16fa.max(min_timeout);

    SettingsMenuAction::Restart
}

unsafe fn set_joystick_calibration(
    player: Player,
    top_left: (u16, u16),
    bottom_right: (u16, u16),
    centre: (u16, u16),
) {
    fn midpoint(a: u16, b: u16) -> u16 {
        ((i32::from(a) - i32::from(b)) / 2 + i32::from(b)) as u16
    }

    match player {
        Player::One => {
            CFG.p1_joy.word6 = midpoint(top_left.0, centre.0);
            CFG.p1_joy.word10 = midpoint(top_left.1, centre.1);
            CFG.p1_joy.word8 = midpoint(bottom_right.0, centre.0);
            CFG.p1_joy.word12 = midpoint(bottom_right.1, centre.1);
            CFG.p1_joy.byte0 = 0xFF;
        }
        Player::Two => {
            CFG.p2_joy.word6 = midpoint(top_left.0, centre.0);
            CFG.p2_joy.word10 = midpoint(top_left.1, centre.1);
            CFG.p2_joy.word8 = midpoint(bottom_right.0, centre.0);
            CFG.p2_joy.word12 = midpoint(bottom_right.1, centre.1);
            CFG.p2_joy.byte0 = 0xFF;
        }
    }
}

/// 3FAF: Wait for a joystick position confirmed by fire.
unsafe fn sub_3faf(
    player: Player,
    bg: &[u8],
    font3: &BmpVec,
    lines: &[(&str, u8)],
    saved_word_16fa: u16,
) -> Result<(u16, u16), SettingsMenuAction> {
    sub_3ef4(bg, font3, lines);

    loop {
        sub_d6f9();
        sub_3ff6(player, saved_word_16fa)?;

        if joy_button(Player::One) == 0 && joy_button(Player::Two) == 0 {
            break;
        }
    }

    let other = match player {
        Player::One => Player::Two,
        Player::Two => Player::One,
    };

    loop {
        sub_d6f9();
        sub_3ff6(player, saved_word_16fa)?;

        if joy_button(player) != 0 {
            return Ok(joy_position(player));
        }

        if joy_button(other) != 0 {
            CFG.word_16fc = !CFG.word_16fc;
            return Ok(joy_position(other));
        }
    }
}

unsafe fn joy_button(player: Player) -> u8 {
    match player {
        Player::One => CFG.p1_joy.byte1,
        Player::Two => CFG.p2_joy.byte1,
    }
}

unsafe fn joy_position(player: Player) -> (u16, u16) {
    match player {
        Player::One => (CFG.p1_joy.word2, CFG.p1_joy.word4),
        Player::Two => (CFG.p2_joy.word2, CFG.p2_joy.word4),
    }
}

/// 3FF6: Abort joystick calibration on ESC.
unsafe fn sub_3ff6(player: Player, saved_word_16fa: u16) -> Result<(), SettingsMenuAction> {
    const ESC_SCAN_CODE: usize = 1;

    if BYTE_3D88[ESC_SCAN_CODE] & 1 == 0 {
        return Ok(());
    }

    BYTE_3D88[ESC_SCAN_CODE] = 0x80;
    CFG.word_16fa = saved_word_16fa;

    if player == Player::One {
        CFG.word_16ff = Controls::Keyboard;
    }

    Err(SettingsMenuAction::Restart)
}

/// 4231: Dispatch selected RECS menu item.
unsafe fn dispatch_recs_menu_item(
    font3: &BmpVec,
    font4: &BmpVec,
    font5: &BmpVec,
) -> RecsMenuAction {
    match (WORD_1233, WORD_1231) {
        (0, 0) => {
            sub_4339();
            RecsMenuAction::Continue
        }
        (0, 2) => {
            sub_4369(font3);
            RecsMenuAction::Continue
        }
        (1, 0) => loc_4255(0, font4, font5),
        (1, 1) => loc_4255(3, font4, font5),
        (1, 2) => loc_4255(5, font4, font5),
        (2, 0) => loc_4255(1, font4, font5),
        (2, 1) => loc_4255(4, font4, font5),
        (2, 2) => loc_4255(6, font4, font5),
        (3, 0) => loc_4255(2, font4, font5),
        (3, 1) => loc_4255(8, font4, font5),
        (3, 2) => loc_4255(7, font4, font5),
        (4, _) => {
            sub_434e();
            RecsMenuAction::Continue
        }
        _ => RecsMenuAction::Exit,
    }
}

/// 4255: Edit one RECS percentage/difficulty value.
unsafe fn loc_4255(ix: usize, font4: &BmpVec, font5: &BmpVec) -> RecsMenuAction {
    WORD_1273 = 0xFFFF;

    loop {
        sub_445d(ix, font4, font5);
        sub_37ca();

        let bl = loop {
            sub_d6f9();

            if handle_esc_key() {
                return RecsMenuAction::Exit;
            }

            let mut bl = sub_575c().1;

            if BYTE_1F58 != 0 {
                bl = WORD_3E2E as u8 | 0x10;
            }

            if bl & 0x30 == 0 {
                return RecsMenuAction::Continue;
            }

            let prev = WORD_1273;
            WORD_1273 = bl as u16;

            if prev != WORD_1273 {
                WORD_3E18 = 0;
                break bl;
            }

            if WORD_3E18 >= 0x80 && WORD_3E18 & 0x0F == 0 {
                break bl;
            }
        };

        let max = if ix == 8 { 99 } else { 26 };
        let mut value = recs_value(ix);

        if bl & 1 != 0 {
            if value == 0 {
                continue;
            }
            value -= 1;
        } else if bl & 2 != 0 {
            if value >= max {
                continue;
            }
            value += 1;
        } else if bl & 8 != 0 {
            value = max;
        } else if bl & 4 != 0 {
            value = 0;
        } else {
            continue;
        }

        set_recs_value(ix, value);
    }
}

/// 4019: Define menu.
unsafe fn define_menu(i16: &[u8], font4: &BmpVec) {
    fade_out();

    draw_sprite(&res_unpack_with_pal(i16), Size::full(), Point::start());

    WORD_122F = 0;
    draw_keys_and_frame(font4);
    sub_37ca();
    fade_in();

    'define: loop {
        draw_keys_and_frame(font4);

        loop {
            sub_37ca();
            sub_d6f9();

            if handle_esc_key() {
                break 'define;
            }

            let al = sub_575c().0 as u8;

            if BYTE_1F58 != 0 {
                let prev = WORD_122F;
                WORD_122F = if WORD_3E32 >= 0x2E { 1 } else { 0 };

                if prev != WORD_122F {
                    continue 'define;
                }
            }

            if al & 8 != 0 {
                if WORD_122F == 1 {
                    WORD_122F = 0;
                }
                continue 'define;
            }

            if al & 4 != 0 {
                if WORD_122F == 0 {
                    WORD_122F = 1;
                }
                continue 'define;
            }

            if al & 0x30 == 0 {
                continue;
            }

            if WORD_122F == 0 {
                break 'define;
            }

            let mut ix = 0;
            let mut y = 56;

            loop {
                let chr = sub_3a66(&mut CFG.game_keys[ix], Point::xy(117, y), font4);

                match chr {
                    KBD_ENTER => continue 'define,
                    KBD_UP if ix > 0 => {
                        ix -= 1;
                        y -= 15;
                    }
                    KBD_UP => (),
                    KBD_DOWN if ix < 8 => {
                        ix += 1;
                        y += 15;
                    }
                    KBD_DOWN => (),
                    _ => continue 'define,
                }
            }
        }
    }
}

/// 40FA: Draw 9 string keys and current Define-menu frame.
unsafe fn draw_keys_and_frame(font4: &BmpVec) {
    let mut y = 56;

    for key in CFG.game_keys.iter().take(9) {
        print_string_narrow(key, Point::xy(117, y), font4);
        y += 15;
    }

    sub_389e();

    if WORD_122F == 0 {
        draw_menu_frame(112, 7, Size::wh(95, 30));
    } else {
        draw_menu_frame(112, 46, Size::wh(95, 147));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RecsMenuAction {
    Continue,
    Exit,
}

/// 4135: RECS menu.
unsafe fn recs_menu(i17: &[u8], i18: &[u8], font3: &BmpVec, font4: &BmpVec, font5: &BmpVec) {
    fade_out();

    let bg = res_unpack_with_pal(i17);
    let items = res_unpack_simple(i18);

    sub_d03e();
    WORD_1231 = 1;
    WORD_1233 = 0;
    draw_recs_menu(&bg, &items, font3, font4, font5);
    sub_37ca();
    fade_in();

    'redraw: loop {
        sub_d001();
        draw_recs_menu(&bg, &items, font3, font4, font5);

        loop {
            sub_37ca();
            sub_d6f9();

            if handle_esc_key() {
                break 'redraw;
            }

            let al = sub_575c().0 as u8;

            if BYTE_1F58 != 0 {
                let prev = (WORD_1231, WORD_1233);
                WORD_1231 = WORD_3E30 / 214;
                WORD_1233 = WORD_3E32 / 40;

                if prev != (WORD_1231, WORD_1233) {
                    continue 'redraw;
                }
            }

            if al & 1 != 0 {
                WORD_1231 = WORD_1231.saturating_sub(1);
                continue 'redraw;
            }

            if al & 2 != 0 {
                if WORD_1231 < 2 {
                    WORD_1231 += 1;
                }
                continue 'redraw;
            }

            if al & 8 != 0 {
                WORD_1233 = WORD_1233.saturating_sub(1);
                continue 'redraw;
            }

            if al & 4 != 0 {
                if WORD_1233 < 4 {
                    WORD_1233 += 1;
                }
                continue 'redraw;
            }

            if al & 0x30 == 0 {
                continue;
            }

            match dispatch_recs_menu_item(font3, font4, font5) {
                RecsMenuAction::Continue => continue 'redraw,
                RecsMenuAction::Exit => break 'redraw,
            }
        }
    }
}

/// 4339: Toggle between A-B and circular route type.
unsafe fn sub_4339() {
    let mut al = ARR_1F3E[0] as u8;

    if al >= 13 {
        al -= 1;
    }

    al ^= 1;

    if al >= 13 {
        al += 1;
    }

    ARR_1F3E[0] = al as u16;
}

/// 434E: Select next scenario.
unsafe fn sub_434e() {
    let mut al = ARR_1F3E[0] as u8;

    if al >= 13 {
        al -= 1;
    }

    al += 2;

    if al >= 0x1A {
        al -= 0x1A;
    }

    if al >= 13 {
        al += 1;
    }

    ARR_1F3E[0] = al as u16;
}

/// 4369: Edit RECS game code.
unsafe fn sub_4369(font3: &BmpVec) {
    sub_3a66(&mut CFG.game_code, Point::xy(221, 21), font3);
    sub_d03e();
}

/// 438D: Draw RECS menu.
unsafe fn draw_recs_menu(bg: &[u8], items: &[u8], font3: &BmpVec, font4: &BmpVec, font5: &BmpVec) {
    draw_sprite(bg, Size::full(), Point::start());

    let mut route = ARR_1F3E[0];

    if route as u8 >= 13 {
        route -= 1;
    }

    let scenario = route >> 1;

    if route & 1 == 0 {
        draw_small_menu_item(items, Point::xy(16, 13), 13);
        draw_small_menu_item(items, Point::xy(40, 13), 14);
    } else {
        draw_small_menu_item(items, Point::xy(72, 13), 15);
    }

    print_empty_string(12, Point::xy(221, 21), font3);
    print_string_narrow(&CFG.game_code, Point::xy(221, 21), font3);

    draw_small_menu_item(
        items,
        Point::xy(scenario as usize * 24 + 4, 169),
        scenario as u8,
    );

    if WORD_1233 == 4 {
        draw_menu_frame(8, 163, Size::wh(303, 30));
    } else {
        draw_menu_frame(WORD_1231 * 104 + 8, WORD_1233 * 39 + 7, Size::wh(95, 30));
    }

    for ix in 0..=8 {
        sub_445d(ix, font4, font5);
    }
}

/// 443C: Draw 24x26 RECS-menu sprite.
unsafe fn draw_small_menu_item(data: &[u8], pos: Point, ix: u8) {
    const MENU_ITEM_SIZE: Size = Size::wh(24, 26);

    let start = ix as usize * MENU_ITEM_SIZE.size();
    let end = start + MENU_ITEM_SIZE.size();

    if end <= data.len() {
        draw_sprite(&data[start..end], MENU_ITEM_SIZE, pos);
    }
}

/// 445D: Draw RECS percentage bar or difficulty digits.
unsafe fn sub_445d(ix: usize, font4: &BmpVec, font5: &BmpVec) {
    const RECS_VALUE_POS: [Point; 8] = [
        Point::xy(18, 54),
        Point::xy(18, 93),
        Point::xy(18, 132),
        Point::xy(122, 54),
        Point::xy(122, 93),
        Point::xy(226, 54),
        Point::xy(226, 93),
        Point::xy(226, 132),
    ];

    if ix == 8 {
        loc_452b(font5);
        return;
    }

    let pos = RECS_VALUE_POS[ix];
    let value = recs_value(ix);
    let bp = match value {
        13 => 0x25,
        14.. => (value - 1) as usize * 3,
        _ => value as usize * 3,
    };
    let mut dx = 74usize.saturating_sub(bp);

    if bp == 0 {
        dx += 1;
    }

    sub_4515(pos, bp, dx);

    for y in pos.y + 1..pos.y + 21 {
        let mut x = pos.x;

        if bp != 0 {
            fill_recs_bar_run(Point::xy(x, y), bp, 26);
            x += bp;
            fill_recs_bar_run(Point::xy(x, y), 1, 0);
            x += 1;
        }

        fill_recs_bar_run(Point::xy(x, y), dx, 30);
    }

    sub_4515(Point::xy(pos.x, pos.y + 21), bp, dx);
    draw_recs_percent(sub_d097(value), Point::xy(pos.x + 26, pos.y + 5), font4);
}

/// 4515: Draw one RECS percentage bar border row.
unsafe fn sub_4515(pos: Point, bp: usize, dx: usize) {
    let mut x = pos.x;

    fill_recs_bar_run(Point::xy(x, pos.y), bp, 28);
    x += bp;
    fill_recs_bar_run(Point::xy(x, pos.y), 1, 0);
    x += 1;
    fill_recs_bar_run(Point::xy(x, pos.y), dx, 13);
}

/// 452B: Draw RECS difficulty digits.
unsafe fn loc_452b(font5: &BmpVec) {
    let tens = (WORD_1F50 / 10).min(9) as u8;
    let ones = (WORD_1F50 % 10) as u8;

    draw_char_small(b'0' + tens, Point::xy(144, 132), font5);
    draw_char_small(b'0' + ones, Point::xy(160, 132), font5);
}

unsafe fn draw_recs_percent(value: u16, pos: Point, font4: &BmpVec) {
    let tens = (value / 10) as u8;
    let ones = (value % 10) as u8;
    let mut x = pos.x;

    if tens >= 10 {
        draw_char_small(b'1', Point::xy(x - 7, pos.y), font4);
        draw_char_small(b'0', Point::xy(x, pos.y), font4);
    } else if tens != 0 {
        draw_char_small(b'0' + tens, Point::xy(x, pos.y), font4);
    }

    x += 7;
    draw_char_small(b'0' + ones, Point::xy(x, pos.y), font4);
    x += 7;
    draw_char_small(b'%', Point::xy(x, pos.y), font4);
}

unsafe fn fill_recs_bar_run(pos: Point, len: usize, color: u8) {
    let start = pos.index();

    if start >= VGA_DBL_BUF.len() {
        return;
    }

    let end = (start + len).min(VGA_DBL_BUF.len());

    for px in &mut VGA_DBL_BUF[start..end] {
        *px = color;
    }
}

unsafe fn recs_value(ix: usize) -> u16 {
    if ix == 8 { WORD_1F50 } else { ARR_1F3E[ix + 1] }
}

unsafe fn set_recs_value(ix: usize, value: u16) {
    if ix == 8 {
        WORD_1F50 = value;
    } else {
        ARR_1F3E[ix + 1] = value;
    }
}

/// 454C:
pub unsafe fn handle_input(input: &mut [u8], pos: Point, font: &BmpVec) -> u8 {
    sub_d421();

    WORD_1299 = WORD_129F;

    if (BYTE_12A4 & 4) == 0 {
        WORD_1299 = WORD_12A1 + 1;

        while WORD_1299 > WORD_129F && input[(WORD_1299 - 1) as usize] == b' ' {
            WORD_1299 -= 1;
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
                let mask = if c.is_ascii_digit() { 1 } else { 2 };

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

/// 467F: Pick a car.
///
/// Returns `true` after selecting a car, or `false` if ESC aborts back to the
/// previous menu flow.
pub unsafe fn pick_a_car() -> bool {
    const CAR_SEL_RES_IDS: [u8; 6] = [0x1D, 0x1E, 0x10, 0x11, 0x12, 0x13];

    fade_out();
    let resources = load_resource_series(b'I', &CAR_SEL_RES_IDS);
    CAR_NUM = Default::default();

    loop {
        fade_out();

        let ix = CAR_NUM as usize * 2;
        draw_sprite(
            &res_unpack_with_pal(&resources[ix + 1]),
            Size::full(),
            Point::start(),
        );

        let car = res_unpack_simple(&resources[ix]);
        sub_37ca();
        WORD_3E18 = 0;
        fade_in2();
        sub_4756(&car);

        loop {
            sub_4764(&car);
            sub_d6f9();

            if handle_esc_key() {
                return false;
            }

            let al = sub_575c().0 as u8;

            if BYTE_1F58 != 0 {
                if WORD_3E32 < 36 {
                    if WORD_3E30 < 80 {
                        CAR_NUM = CAR_NUM.prev();
                        fade_out2();
                        sub_4756(&car);
                        break;
                    }

                    if WORD_3E30 > 560 {
                        CAR_NUM = CAR_NUM.next();
                        fade_out2();
                        sub_4756(&car);
                        break;
                    }
                }

                if WORD_3E30 >= 80 && WORD_3E30 - 80 < 480 && WORD_3E32 >= 26 && WORD_3E32 - 26 < 60
                {
                    fade_out2();
                    sub_4756(&car);
                    return true;
                }

                continue;
            }

            if al & 2 != 0 {
                CAR_NUM = CAR_NUM.next();
                fade_out2();
                sub_4756(&car);
                break;
            }

            if al & 1 != 0 {
                CAR_NUM = CAR_NUM.prev();
                fade_out2();
                sub_4756(&car);
                break;
            }

            if al & 0x30 != 0 {
                fade_out2();
                sub_4756(&car);
                return true;
            }
        }
    }
}

/// 4756: Keep animating the selected car while a fade is in progress.
unsafe fn sub_4756(car: &[u8]) {
    loop {
        sub_d6f9();
        sub_4764(car);

        if FADE_STEP == 0 {
            return;
        }
    }
}

/// 4764: Draw the current car-selection animation frame.
unsafe fn sub_4764(car: &[u8]) {
    const CAR_FRAME_SIZE: Size = Size::wh(88, 24);

    let frame = ((WORD_3E18 / 24) & 0x0F) as usize;
    let start = frame * CAR_FRAME_SIZE.size();
    let end = start + CAR_FRAME_SIZE.size();

    if end <= car.len() {
        draw_sprite(&car[start..end], CAR_FRAME_SIZE, Point::xy(91, 97));
    }

    sub_37ca();
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
