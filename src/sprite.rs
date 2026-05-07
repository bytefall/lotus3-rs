use crate::hud::{VGA_DBL_BUF, VGA_WIDTH};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn start() -> Self {
        Self { x: 0, y: 0 }
    }

    pub const fn xy(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn index(&self) -> usize {
        self.y * VGA_WIDTH + self.x
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self::xy(x, y)
    }
}

pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub const fn full() -> Self {
        Self {
            width: 320,
            height: 200,
        }
    }

    pub const fn wh(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub const fn size(&self) -> usize {
        self.width * self.height
    }
}

/// C047: Draw a sprite
pub unsafe fn draw_sprite(data: impl AsRef<[u8]>, size: Size, pos: Point) {
    for (dst, src) in VGA_DBL_BUF[pos.index()..]
        .chunks_exact_mut(VGA_WIDTH)
        .take(size.height)
        .zip(data.as_ref().chunks_exact(size.width))
    {
        dst[..src.len()].copy_from_slice(src);
    }
}

/// C076: Draw a single character
pub unsafe fn draw_char(chr: u8, font: impl AsRef<[u8]>, size: Size, skip: usize) {
    let font = &font.as_ref()[chr as usize * size.width * size.height..];

    for (dst, src) in VGA_DBL_BUF[skip..]
        .chunks_exact_mut(VGA_WIDTH)
        .take(size.height)
        .zip(font.chunks_exact(size.width))
    {
        for (d, s) in dst[..src.len()].iter_mut().zip(src) {
            *d |= *s;
        }
    }
}
