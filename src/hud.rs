use core::{arch::asm, ptr, slice};
use heapless::Vec as StackVec;

use crate::{
    bitmap::draw_char_small,
    chars::BmpVec,
    data::{SCREEN_WIDTH, WORD_63BC, WORD_633A},
    sprite::Point,
};

// pub const VGA_RAW_PTR: *mut u8 = (0xA0000usize - 320 * 20 - 32) as *mut u8;
pub const VGA_RAW_PTR: *mut u8 = 0xA0000usize as *mut u8;
pub const VGA_WIDTH: usize = 320;
pub const VGA_HEIGHT: usize = 200;

/// 8EDE: Print text
pub unsafe fn print_string<const MAX_LEN: usize>(text: &[u8; MAX_LEN], y: usize, font: &BmpVec) {
    let text: StackVec<u8, MAX_LEN> = text
        .split(|x| x == &b' ')
        .flat_map(|x| x.iter().copied().chain([b' '])) // TODO: add ' ' only in-between
        .take(MAX_LEN)
        .collect();

    let mut pos = Point::xy((VGA_WIDTH - text.len() * 8) / 2, y);

    for chr in &text {
        draw_char_small(*chr, pos, font);
        pos.x += 8;
    }
}

/// 929A: P1: Update the screen
pub unsafe fn sub_929a() {
    loc_92ce(0)
}

/// 929F: P2: Update the screen
pub unsafe fn sub_929f() {
    loc_92ce(99 * SCREEN_WIDTH + 16)
}

/// 92A4: Clear double buffer (fill 67200 bytes with 0)
pub unsafe fn sub_92a4() {
    ptr::write_bytes(WORD_63BC.as_mut_ptr(), 0, WORD_63BC.len()); // +VGA_DBL_BUF_START
}

/// 92C4: update_screen -> vga_flush
pub unsafe fn update_screen() {
    vga_draw(200, 0);
}

/// 92CE
pub unsafe fn loc_92ce(skip: usize) {
    vga_draw(WORD_633A as usize, skip);
}

/// 92D2
pub unsafe fn vga_draw(height: usize, skip: usize) {
    ptr::copy(
        WORD_63BC.as_ptr(), // +VGA_DBL_BUF_START
        VGA_RAW_PTR.add(skip),
        VGA_WIDTH * height,
    );
}

/// 9332
pub unsafe fn clear_screen() {
    ptr::write_bytes(VGA_RAW_PTR, 0, VGA_WIDTH * VGA_HEIGHT);
}
