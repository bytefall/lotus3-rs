use heapless::Vec as StackVec;

use crate::{
    archive::load_resource_series,
    chars::chr_load_and_prepare_few,
    crc::sub_abf0,
    data::WORD_2E78,
    hud::{print_string, update_screen},
    menu::{
        BYTE_12A4, WORD_12A1, WORD_129D, WORD_129F, WORD_1299, handle_input, res_unpack_simple,
        res_unpack_with_pal,
    },
    sprite::{Point, Size, draw_sprite},
    timer::sub_d421,
    video::{fade_in, fade_out},
};

/// D83
const PROT_D83: [(u8, u8); 48] = [
    (0, 0),
    (0, 4),
    (0, 8),
    (0, 10),
    (0, 13),
    (0, 17),
    (0, 19),
    (0, 21),
    (0, 25),
    (0, 28),
    (0, 30),
    (0, 35),
    (1, 2),
    (1, 5),
    (1, 11),
    (1, 12),
    (1, 16),
    (1, 20),
    (1, 24),
    (1, 29),
    (1, 31),
    (1, 34),
    (2, 0),
    (2, 2),
    (2, 4),
    (2, 6),
    (2, 10),
    (2, 12),
    (2, 14),
    (2, 17),
    (2, 18),
    (2, 21),
    (3, 22),
    (3, 1),
    (3, 3),
    (3, 5),
    (3, 7),
    (3, 9),
    (3, 11),
    (3, 13),
    (3, 15),
    (3, 16),
    (3, 21),
    (4, 11),
    (4, 2),
    (4, 4),
    (4, 7),
    (4, 9),
];

/// DE3
const PROT_DE3: [u8; 12] = [
    b'D', b'C', b'B', b'A', b'6', b'?', b'>', b'=', b'H', b'G', b'F', b'E',
];

const HELMET_SIZE: Size = Size::wh(48, 40);
const HELMET_TOP: Point = Point::xy(141, 13);
const HELMET_BOTTOM: Point = Point::xy(141, 73);

/// 2F1C:
pub unsafe fn protection_screen() {
    const PROT_FONTS: [u8; 1] = [3];
    const PROT_RES_IDS: [u8; 2] = [0x21, 0x22];

    fade_out();

    WORD_2E78 = 0;

    // loc_2F3D
    let prot_code = sub_abf0().carrying_mul(48, 0).1;
    let prot_d5c = sub_abf0().carrying_mul(12, 0).1;
    let prot_d5e = sub_abf0().carrying_mul(12, 0).1;

    // loc_2FF5
    let [bgr, helmets] = load_resource_series(b'I', &PROT_RES_IDS);
    draw_sprite(&res_unpack_with_pal(&bgr), Size::full(), Point::start());

    let helmets = res_unpack_simple(&helmets);
    draw_sprite(
        &helmets[HELMET_SIZE.size() * prot_d5c as usize..],
        HELMET_SIZE,
        HELMET_TOP,
    );
    draw_sprite(
        &helmets[HELMET_SIZE.size() * (prot_d5e as usize + 12)..],
        HELMET_SIZE,
        HELMET_BOTTOM,
    );

    let mut prot_text = *b"ENTER CODE FOR WINDOW ??";
    prot_text[prot_text.len() - 2] = ((prot_code + 1) / 10 + 0x30) as u8;
    prot_text[prot_text.len() - 1] = ((prot_code + 1) % 10 + 0x30) as u8;
    let [font, ..] = chr_load_and_prepare_few(&PROT_FONTS);
    print_string(&prot_text, 140, &font);

    update_screen();
    fade_in();

    WORD_129D = 3;
    WORD_129F = 0;
    WORD_12A1 = 2;
    BYTE_12A4 = 3;

    // two lines below added by codex
    WORD_1299 = WORD_129F;
    while sub_d421() != 0 {}
    // ends here

    let mut input = [b' '; 3];
    handle_input(&mut input, Point::xy(150, 165), &font);

    fade_out();
}

unsafe fn make_secret(prot_code: u16, prot_d5c: u16, prot_d5e: u16) -> StackVec<u8, 3> {
    let (one, two) = PROT_D83[prot_code as usize];

    let mut prot_d7b = StackVec::<u8, 3>::new();
    let mut dx = prot_d5c.wrapping_sub(prot_d5e) as i8;

    if dx < 0 {
        dx += 12;
    }

    let mut cx = (3 - one / 2) as u16;

    let mut dx = (cx * ((dx as u16) & 0xFF)).to_le_bytes();
    dx[0] = dx[0].wrapping_add(two);
    let mut dx = u16::from_le_bytes(dx);

    let ax = cx * 12;

    if ax <= dx {
        dx = dx.wrapping_sub(ax);
    }

    if one >= 1 {
        // loc_2FA9
        if one != 1 {
            // loc_2FB2
            if one >= 3 {
                // loc_2FBB
                if one != 3 {
                    // loc_2FCC
                    let mut dd = dx.to_le_bytes();
                    dd[0] = PROT_DE3[dx as usize];

                    dx = u16::from_le_bytes(dd);
                } else {
                    let dl = (dx as u8).wrapping_add(83);

                    prot_d7b
                        .push(if dl < 89 { dl } else { dl.wrapping_sub(24) })
                        .unwrap();
                    prot_d7b.resize(3, b' ').unwrap();

                    return prot_d7b;
                }
            } else {
                dx += 37;
            }
        } else {
            dx = dx.wrapping_neg() + 106;
        }
    } else {
        dx += 1;
    }

    // loc_2FD2
    cx &= 0xFF00;
    let mut dx = dx.to_le_bytes();

    if dx[0] >= 100 {
        dx[0] = dx[0].wrapping_sub(100);
        prot_d7b.push(49).unwrap();
        cx = cx.wrapping_sub(1);
    }

    // loc_2FE1
    let ax = (u16::from_le_bytes(dx) / 10).to_le_bytes();
    let cx = cx.to_le_bytes();

    if ax[0] != cx[0] {
        prot_d7b.push(ax[0] + b'0').unwrap();
    }

    // loc_2FF0
    prot_d7b.push(ax[1] + b'0').unwrap();
    prot_d7b.resize(3, b' ').unwrap();
    prot_d7b
}

// ;
// ; 3095:
// ;
// unk_3095:
//     and     bl, [bx]
//     nop
//     add     [si + 8], dh
//     cmp     word [prot_d7e], 2020h
//     je      ret_30C1
//     mov     ax, [prot_d7e]
//     cmp     ax, [prot_d7b]
//     jne     loc_30B5
//     mov     al, [prot_d80]
//     cmp     al, [prot_d7d]
//     je      ret_30C1

// loc_30B5:
//     dec     byte [prot_d82]
//     je      loc_30BE
//     jmp     loc_2F3D

// loc_30BE:
//     jmp     loc_DC70

// ret_30C1:
//     ret
