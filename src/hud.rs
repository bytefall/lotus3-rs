use alloc::vec::Vec;
use core::{arch::asm, ptr};

use crate::{bitmap::draw_char_small, chars::BmpVec, data::WORD_633A, sprite::Point};

pub const VGA_WIDTH: usize = 320;
pub const VGA_HEIGHT: usize = 200;
pub const VGA_SIZE: usize = VGA_WIDTH * VGA_HEIGHT;

pub static mut VGA_DBL_BUF: Vec<u8> = Vec::new(); // 63BC

/// 8EDE: Print text
pub unsafe fn print_string<const MAX_LEN: usize>(text: &[u8; MAX_LEN], y: usize, font: &BmpVec) {
    let mut pos = Point::xy((VGA_WIDTH - text.len() * 8) / 2, y);

    for chr in text {
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
    loc_92ce(99 * VGA_WIDTH + 16)
}

/// 92A4: Clear double buffer (fill 67200 bytes with 0)
pub unsafe fn sub_92a4() {
    ptr::write_bytes(VGA_DBL_BUF.as_mut_ptr(), 0, VGA_DBL_BUF.len());
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
    if skip >= VGA_SIZE {
        return;
    }

    let src = VGA_DBL_BUF.as_ptr() as usize;
    let dst = skip as u32;
    let len = (VGA_WIDTH * height).min(VGA_SIZE - skip) as u32;

    copy_to_vga(src, dst, len);
}

pub unsafe fn copy_to_vga(src: usize, dst: u32, len: u32) {
    asm!(
        "push es",
        "pushf",
        "push ecx",
        "push edi",
        "push esi",
        "push {src:e}",
        "push {dst:e}",
        "push {len:e}",
        "pop ecx",
        "pop edi",
        "pop esi",
        "cld",
        "mov ax, 0xA000",
        "mov es, ax",
        ".byte 0x67, 0xF3, 0xA4",
        "pop esi",
        "pop edi",
        "pop ecx",
        "popf",
        "pop es",
        src = in(reg) src,
        dst = in(reg) dst,
        len = in(reg) len,
        lateout("ax") _,
    );
}

/// 9332
pub unsafe fn clear_screen() {
    asm!(
        "push es",
        "pushf",
        "push cx",
        "push di",
        "cld",
        "mov ax, 0xA000",
        "mov es, ax",
        "xor ax, ax",
        "xor di, di",
        "mov cx, {words:x}",
        "rep stosw",
        "pop di",
        "pop cx",
        "popf",
        "pop es",
        words = in(reg) (VGA_SIZE / 2) as u16,
        lateout("ax") _,
    );
}
