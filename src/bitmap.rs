use crate::{
    chars::{BmpItem, BmpVec},
    data::{SCREEN_WIDTH, WORD_63BC},
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
    let repeat = (bmp.repeat + 7) >> 3;
    let height = bmp.height as usize;
    let mut it = bmp.data.iter();

    for y in pos.y..pos.y + height {
        let mut offset = y * SCREEN_WIDTH + pos.x;

        for _ in 0..repeat {
            for o in get_codes(*it.next().unwrap() as usize).iter().flatten() {
                match *o {
                    Code::Skip(num) => offset += num as usize,
                    Code::Draw(num) => {
                        for _ in 0..num {
                            WORD_63BC[offset] = *it.next().unwrap();

                            offset += 1;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Code {
    Skip(u8),
    Draw(u8),
}

fn get_codes(mut y: usize) -> [Option<Code>; 8] {
    let mut codes = [None; 8];
    let mut x = 0;

    for _ in 0..codes.len() {
        let cf = y & 0x80 != 0;
        y <<= 1;

        if cf {
            // loc_DB7C
            match codes[x] {
                Some(Code::Draw(v)) => codes[x] = Some(Code::Draw(v + 1)),
                Some(Code::Skip(_)) => {
                    x += 1;
                    codes[x] = Some(Code::Draw(1));
                }
                None => codes[x] = Some(Code::Draw(1)),
            }
        } else {
            // loc_DB65
            match codes[x] {
                Some(Code::Skip(v)) => codes[x] = Some(Code::Skip(v + 1)),
                Some(Code::Draw(_)) => {
                    x += 1;
                    codes[x] = Some(Code::Skip(1));
                }
                None => codes[x] = Some(Code::Skip(1)),
            }
        }
    }

    codes
}
