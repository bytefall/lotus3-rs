use crate::{
    chars::BmpVec,
    hud::{VGA_DBL_BUF, VGA_WIDTH},
    sprite::Point,
};

/// A1D6: Draw small character
pub unsafe fn draw_char_small(chr: u8, pos: Point, font: &BmpVec) {
    let chr = chr & 0b111_1111;

    match chr {
        0 => bmp_draw(b'&', pos, font),
        b'%' => bmp_draw(b'\'', pos, font),
        b'-' => bmp_draw(b'$', pos, font),
        b'.' => bmp_draw(b'%', pos, font),
        b'a'..=b'z' => bmp_draw(chr - b'W', pos, font),
        b'A'..=b'Z' => bmp_draw(chr - b'7', pos, font),
        b'0'..=b'9' => bmp_draw(chr - b'0', pos, font),
        _ => (),
    }
}

/// A224: Draw a bitmap
pub unsafe fn bmp_draw(chr: u8, pos: Point, bmp: &BmpVec) {
    let bmp = &bmp[chr as usize];

    let width = (bmp.repeat + 7) >> 3;
    let height = bmp.height as usize;
    let mut it = bmp.data.iter();

    for y in pos.y..pos.y + height {
        let mut offset = y * VGA_WIDTH + pos.x;

        for _ in 0..width {
            let mut mask = *it.next().unwrap();

            for _ in 0..8 {
                if (mask & 0x80) != 0 {
                    VGA_DBL_BUF[offset] = *it.next().unwrap();
                }

                mask <<= 1;
                offset += 1;
            }
        }
    }
}
