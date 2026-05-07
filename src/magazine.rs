use crate::{
    archive::{load_resource_series, sub_cbc3},
    data::{BYTE_3E0E, PALETTE, PALETTE2, WORD_3E18},
    dos::{cli, sti},
    hud::{VGA_WIDTH, copy_to_vga, update_screen},
    menu::{res_unpack_simple, res_unpack_with_pal},
    prepare::sleep,
    sprite::{Point, Size, draw_sprite},
    video::{fade_in, fade_in_pal, fade_out, set_vga_pal, sub_d16c},
};

/// 2DED: Show automotive magazine with a short video
pub unsafe fn show_magazine() {
    const VIDEO_IDS: [u8; 41] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    ];
    const ANIM_STEP: u16 = 24;

    fade_out();

    draw_sprite(
        res_unpack_with_pal(sub_cbc3(b'V', 0x32)),
        Size::full(),
        Point::start(),
    );
    update_screen();

    let vids = load_resource_series(b'V', &VIDEO_IDS);
    draw_video_sprite(&vids[0], -1);

    if WORD_3E18 < 3 {
        draw_video_sprite(&vids[0], 0);
        fade_in();

        WORD_3E18 = ANIM_STEP;
        let mut i = 0;

        while i < VIDEO_IDS.len() - 1 {
            i = if WORD_3E18 < (VIDEO_IDS.len() as u16) * ANIM_STEP {
                ((WORD_3E18 / ANIM_STEP) as u8) as usize
            } else {
                VIDEO_IDS.len() - 1
            };

            draw_video_sprite(&vids[i], 0);
        }

        PALETTE[16 * 3..].fill(0x3F);
        fade_in_pal(&PALETTE);
    }

    // loc_2E7E
    draw_video_sprite(sub_cbc3(b'V', 0x33), 1);
    fade_in_pal(&PALETTE);

    WORD_3E18 = 0;
    sleep(1400);
    fade_out();
}

/// 2EB2:
unsafe fn draw_video_sprite(res: impl AsRef<[u8]>, cx: i8) {
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

    let mut src = r.as_ptr().add(PREFIX_PAL_LEN);
    let mut dst = VIDEO_POS.y * VGA_WIDTH + VIDEO_POS.x;

    for _ in 0..VIDEO_SIZE.height {
        copy_to_vga(src as usize, dst as u32, VIDEO_SIZE.width as u32);

        src = src.add(VIDEO_SIZE.width);
        dst += VGA_WIDTH;
    }
}
