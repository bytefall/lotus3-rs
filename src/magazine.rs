use core::{arch::asm, ptr};

use crate::{
    archive::{load_resource_series, sub_cbc3},
    data::{BYTE_3E0E, PALETTE, PALETTE2, WORD_3E18},
    dos::{cli, get_data_seg, set_data_seg, sti},
    hud::update_screen,
    menu::res_unpack_simple,
    prepare::sleep,
    sprite::{Point, Size, draw_sprite},
    video::{fade_in, fade_in_pal, fade_out, set_vga_pal, sub_d16c},
};

/// 2DED: Show automotive magazine with a short video
pub unsafe fn show_magazine() {
    // const VIDEO_IDS: [u8; 41] = [
    //     0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    //     0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
    //     0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    // ];
    const VIDEO_IDS: [u8; 15] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    ];
    const ANIM_STEP: u16 = 24;

    fade_out();

    let bgr = crate::archive::res_load(b'V', 0x32);
    PALETTE.copy_from_slice(&bgr[bgr.len() - PALETTE.len()..]);
    // let bgr = res_unpack_with_pal(&sub_cbc3(b'V', 0x32));

    draw_sprite(&bgr, Size::full(), Point::start());
    update_screen();

    let vids = load_resource_series(b'V', &VIDEO_IDS);
    draw_sprite_with_leading_pal(&vids[0], -1);

    if WORD_3E18 < 3 {
        draw_sprite_with_leading_pal(&vids[0], 0);
        fade_in();

        WORD_3E18 = ANIM_STEP;
        let mut i = 0;

        while i < VIDEO_IDS.len() - 1 {
            i = if WORD_3E18 < (VIDEO_IDS.len() as u16) * ANIM_STEP {
                ((WORD_3E18 / ANIM_STEP) as u8) as usize
            } else {
                VIDEO_IDS.len() - 1
            };

            draw_sprite_with_leading_pal(&vids[i], 0);
        }

        PALETTE[16 * 3..].fill(0x3F);
        fade_in_pal(&PALETTE);
    }

    drop(vids);
    drop(bgr);

    // loc_2E7E
    draw_sprite_with_leading_pal(&sub_cbc3(b'V', 0x33), 1);
    fade_in_pal(&PALETTE);

    WORD_3E18 = 0;
    sleep(1400);
    fade_out();
}

/// 2EB2:
unsafe fn draw_sprite_with_leading_pal(res: &[u8], cx: i8) {
    const PREFIX_PAL_LEN: usize = 720;
    const VIDEO_SIZE: Size = Size::wh(160, 112);
    const VIDEO_POS: Point = Point::xy(136, 38);

    let r = res_unpack_simple(res);
    PALETTE[16 * 3..].copy_from_slice(&r[..PREFIX_PAL_LEN]);

    if cx != 1 {
        set_vga_pal(if cx == -1 { &PALETTE2 } else { &PALETTE });
        sub_d16c();

        if cx == -1 {
            cli();
            WORD_3E18 = BYTE_3E0E.into();
            sti();
        }
    }

    // let ds = get_data_seg();
    // let ptr = ds as u32 * 16 + r.as_ptr().byte_add(PREFIX_PAL_LEN) as u32;
    // asm!(
    //     "mov     ds, {:x}",
    //     "mov     si, {:x}",
    //     "mov     es, {:x}",
    //     "mov     di, {}",
    //     "mov     dx, 112",
    //     "2:",
    //     "mov     cx, 80",
    //     "rep movsw",
    //     "add     di, 160",
    //     "dec     dx",
    //     "jne     2b",
    //     in(reg) ptr / 16,
    //     in(reg) ptr % 16,
    //     in(reg) 0xA000,
    //     const VIDEO_POS.y * 320 + VIDEO_POS.x,
    // );
    // set_data_seg(ds);

    // cli(); // TODO: temporary
    let mut src = r.as_ptr().add(PREFIX_PAL_LEN);
    let mut dst = crate::hud::VGA_RAW_PTR.add(VIDEO_POS.y * 320 + VIDEO_POS.x);

    for _ in 0..VIDEO_SIZE.height / 2 {
        ptr::copy(src, dst, VIDEO_SIZE.width);

        src = src.add(VIDEO_SIZE.width);
        dst = dst.add(320);
    }
    // sti(); // TODO: temporary
    // panic!("WORD_3E18 is {}", WORD_3E18);
}
