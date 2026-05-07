use crate::{
    archive::{load_resource_series, load_resources, res_load, sub_cbc3},
    data::{PALETTE, PALETTE2, WORD_1F20, WORD_3D6C, WORD_3E18},
    hud::{VGA_DBL_BUF, VGA_WIDTH, sub_92a4, update_screen},
    magazine::show_magazine,
    menu::{res_unpack_simple, res_unpack_with_pal},
    prepare::sleep,
    sprite::{Point, Size, draw_char, draw_sprite},
    timer::sub_d6f9,
    video::{fade_by_color, fade_in, fade_out, set_vga_pal, sub_d161},
};

pub unsafe fn show_intro() {
    show_gremlin();
    show_magnetic_fields();
    show_credits();
    show_lotus_logo();
    show_magazine();
}

/// 30DF: Show Gremlin logo
pub unsafe fn show_gremlin() {
    const FLASH_SIZE: Size = Size::wh(16, 8);
    const FLASH_STEP: u16 = 32;
    const FLASH_TOP: Point = Point::xy(112, 85);
    const FLASH_BOTTOM: Point = Point::xy(144, 110);

    fade_out();
    draw_sprite(
        res_unpack_with_pal(sub_cbc3(b'Q', 0)),
        Size::full(),
        Point::start(),
    );
    update_screen();
    fade_in();

    WORD_3E18 = 0;
    sleep(280);
    WORD_3E18 = 0;

    unsafe fn draw_flash(flashes: impl AsRef<[u8]>, pos: Point, skip: usize) {
        loop {
            sub_d6f9();

            let mut ax = WORD_3E18 / FLASH_STEP;

            if ax >= 7 {
                break;
            }

            if ax >= 4 {
                ax = 6 - ax;
            }

            draw_sprite(
                &flashes.as_ref()[((skip + ax as usize) * FLASH_SIZE.width * FLASH_SIZE.height)..],
                FLASH_SIZE,
                pos,
            );
            update_screen();
        }
    }

    let flashes = res_load(b'Q', 1);

    draw_flash(&flashes, FLASH_TOP, 0);

    WORD_3E18 = 0;
    sleep(140);
    WORD_3E18 = 0;

    draw_flash(&flashes, FLASH_BOTTOM, 4);

    WORD_3E18 = 0;
}

/// 31A6: Show Magnetic Fields logo
pub unsafe fn show_magnetic_fields() {
    const MAGN_FIELDS_IDS: [u8; 22] = [
        0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    ];
    const MAGN_FIELDS_STEP: u16 = 8;

    let res = load_resource_series(b'Q', &MAGN_FIELDS_IDS);
    sleep(280);
    fade_out();

    res_unpack_with_pal(res.last().unwrap()); // at this point only PALETTE is needed
    sub_92a4();
    update_screen();
    set_vga_pal(&PALETTE);

    WORD_3E18 = 0;
    let mut cx = 0;

    loop {
        sub_d6f9();

        let ax = WORD_3E18 / MAGN_FIELDS_STEP;

        if ax == cx {
            continue;
        }

        draw_sprite(
            res_unpack_simple(&res[ax.min(20) as usize]),
            Size::full(),
            Point::start(),
        );
        update_screen();

        cx = ax;

        if cx >= 20 {
            break;
        }
    }

    draw_sprite(
        res_unpack_simple(res.last().unwrap()),
        Size::full(),
        Point::start(),
    );
    update_screen();

    WORD_3E18 = 0;
    sleep(1120);
    fade_out();
}

/// 323D: Show credits
pub unsafe fn show_credits() {
    const CREDITS_IDS: [u8; 5] = [0x19, 0x1B, 0x1C, 0x1D, 0x1E];
    const CREDITS_TXT: [(&[(&str, Point)], u16); 5] = [
        (
            &[
                ("A GAME", Point::xy(118, 43)),
                ("BY", Point::xy(146, 67)),
                ("ANDREW MORRIS", Point::xy(69, 91)),
                ("AND", Point::xy(139, 115)),
                ("SHAUN SOUTHERN", Point::xy(62, 139)),
            ],
            1120,
        ),
        (
            &[
                ("LEVEL DESIGN", Point::xy(76, 67)),
                ("BY", Point::xy(146, 91)),
                ("PETER LIGGETT", Point::xy(69, 115)),
            ],
            840,
        ),
        (
            &[
                ("MUSIC", Point::xy(125, 67)),
                ("BY", Point::xy(146, 91)),
                ("PATRICK PHELAN", Point::xy(62, 115)),
            ],
            840,
        ),
        (
            &[
                ("PC CONVERSION", Point::xy(69, 43)),
                ("BY", Point::xy(146, 67)),
                ("JON MEDHURST FOR", Point::xy(48, 91)),
                ("CYGNUS SOFTWARE", Point::xy(55, 115)),
                ("ENGINEERING LTD.", Point::xy(52, 139)),
            ],
            1400,
        ),
        (
            &[
                ("COPYRIGHT 1993", Point::xy(62, 43)),
                ("MAGNETIC FIELDS", Point::xy(55, 67)),
                ("(SOFTWARE DESIGN) LTD.", Point::xy(10, 91)),
                ("GREMLIN GRAPHICS", Point::xy(48, 115)),
                ("SOFTWARE LTD.", Point::xy(73, 139)),
            ],
            1960,
        ),
    ];

    let [bgr, car, frame1, frame2, frame3] = load_resources(b'Q', &CREDITS_IDS);
    fade_out();

    PALETTE.copy_from_slice(&bgr[bgr.len() - PALETTE.len()..]);

    sub_33e6();
    draw_sprite(&bgr, Size::full(), Point::start());
    update_screen();
    fade_in();

    WORD_3E18 = 0;
    let font = res_load(b'Q', 0x1A);
    sleep(280);

    for (text, delay) in CREDITS_TXT.iter().take(1) {
        draw_credits(&font, text, *delay);
    }

    draw_car_approaching(bgr, car);
    sub_32e2(frame1);
    sub_32e2(frame2);
    sub_32e2(frame3);

    WORD_3E18 = 0;
    sleep(560);

    PALETTE[60 * 3..60 * 3 + 3].copy_from_slice(&[0, 0, 0]);

    WORD_3D6C = 140;
    fade_by_color(&PALETTE);

    sub_d161();
    WORD_3E18 = 0;
}

/// 32E2:
unsafe fn sub_32e2(data: impl AsRef<[u8]>) {
    WORD_3E18 = 0;
    sleep(32);

    draw_sprite(data, Size::full(), Point::start());
    update_screen();
}

/// 3300: Draw an animation with a car that is approaching
unsafe fn draw_car_approaching(bgr: impl AsRef<[u8]>, q1b: impl AsRef<[u8]>) {
    const ANIM_STEP: u16 = 32;

    WORD_3E18 = 0;
    let mut prev_step = 0;

    while prev_step < 36 {
        sub_d6f9();

        let step = WORD_3E18 / ANIM_STEP;

        if step == prev_step {
            continue;
        }

        prev_step = step.min(36);

        draw_sprite(&bgr, Size::full(), Point::start());
        draw_a_car(&q1b, prev_step.into());
        update_screen();
    }
}

/// 333E: Draw a car (when car is approaching in the intro)
fn draw_a_car(data: impl AsRef<[u8]>, step: usize) {
    const WIDTH: usize = VGA_WIDTH; // 336?

    let cx = 256 + (36 - step) * 512;
    let di = 160 - ((((WIDTH * 170) + 224) / cx) >> 1) + (WIDTH * 64)
        - (((((WIDTH * 118) + 288) / cx) * (WIDTH * 32 + 170)) >> 16) * WIDTH;

    for (y, row) in (di / WIDTH..).zip((0..WIDTH * 118).step_by(cx)) {
        let offset = (row >> 8) * 224;

        for (x, i) in (di % WIDTH..).zip((offset..offset + 224).step_by(cx >> 8)) {
            let px = data.as_ref()[i];

            if px != 0xFF {
                unsafe {
                    VGA_DBL_BUF[Point::xy(x, y).index()] = px;
                }
            }
        }
    }
}

/// 33C2: when commented out - then text is fully transparent
unsafe fn set_text_palette() {
    for row in PALETTE.chunks_exact_mut(12).take(32) {
        row[3..12].copy_from_slice(&[48, 48, 48, 32, 32, 32, 0, 0, 0]);
    }
}

/// 33E6: fade-in / fade-out font
unsafe fn sub_33e6() {
    for row in PALETTE.chunks_exact_mut(12).take(32) {
        let (r, g, b) = (row[0], row[1], row[2]);

        row[3..12].copy_from_slice(&[r, g, b, r, g, b, r, g, b]);
    }
}

/// 3409: Leave pixels that have a color index 0, 1 or 2
unsafe fn clear_text_texture() {
    for i in &mut VGA_DBL_BUF[..] {
        *i &= 0b1111_1100;
    }
}

/// 341F:
unsafe fn draw_text_large(font: impl AsRef<[u8]>, text: &[(&str, Point)]) {
    'main: for (s, pt) in text {
        let mut skip = pt.index();

        for c in s.as_bytes() {
            let c = match *c {
                0 => continue 'main,
                b' ' => {
                    skip += 14;
                    continue;
                }
                b'.' => 36,
                b'(' => 37,
                b')' => 38,
                _ => {
                    let x = c.saturating_sub(0x30);
                    if x >= 10 { x - 7 } else { x }
                }
            };

            draw_char(c, &font, Size::wh(16, 18), skip);
            skip += 14;
        }
    }
}

/// 346D: Draw credits
unsafe fn draw_credits(font: impl AsRef<[u8]>, text: &[(&str, Point)], delay: u16) {
    clear_text_texture();
    draw_text_large(font, text);
    update_screen();
    set_text_palette();
    fade_by_color(&PALETTE);
    sub_d161();

    WORD_3E18 = 0;
    sleep(delay);

    sub_33e6();
    fade_by_color(&PALETTE);
    sub_d161();

    WORD_3E18 = 0;
    sleep(280);
}

/// 34AC: Show LOTUS logo and fade in "The Ultimate Challenge"
pub unsafe fn show_lotus_logo() {
    if WORD_1F20 == 0 {
        default_logo();
        return;
    }

    WORD_1F20 = 0;
    let logo = res_load(b'Q', 0x18); // LOTUS The Ultimate Challenge
    let pal = &logo[logo.len() - PALETTE.len()..];

    sub_92a4();
    draw_sprite(&logo, Size::full(), Point::start());
    sleep(560);

    set_vga_pal(&PALETTE2);
    update_screen();

    sub_352c(pal, 16);
    set_vga_pal(&PALETTE);

    WORD_3E18 = 0;
    sleep(560);

    sub_352c(pal, 48);
    WORD_3D6C = 140;
    fade_by_color(&PALETTE);
    sub_d161();

    WORD_3E18 = 0;
    sleep(1400);
}

/// 352C: Copy CX colors (CX * 3 bytes) from `word_F77` and fill next CX * 10 colors with zeroes
unsafe fn sub_352c(pal: &[u8], num: usize) {
    PALETTE[..num * 3].copy_from_slice(&pal[..num * 3]);
    PALETTE[num * 3..num * 10].fill(0);
}

/// 3552:
unsafe fn default_logo() {
    fade_out();

    let bgr = res_load(b'Q', 0x18);
    PALETTE.copy_from_slice(&bgr[bgr.len() - PALETTE.len()..]);

    draw_sprite(&bgr, Size::full(), Point::start());
    update_screen();
    fade_in();

    WORD_3E18 = 0;
    sleep(1400);
}
