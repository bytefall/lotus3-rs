use core::{arch::asm, mem::transmute};

use crate::{
    data::*,
    dos::{cli, getd, inb, inw, outb, sti},
    timer::{
        PIT_CH2_LATCH, PIT_CH2_MODE0, PIT_CHANNEL_0, PIT_CHANNEL_2, PIT_MODE, prepare_task_context,
        restore_dos_ceh, restore_kbd_isr, restore_timer, set_dos_ceh, set_kbd_isr, set_timer,
        sub_d6f9,
    },
};

/// Convert value to %
///
/// - 0% = 0
/// - 4% = 1
/// - 8% = 2
/// - 12% = 3
/// - 16% = 4
/// - 20% = 5
/// - 24% = 6
/// - 28% = 7
/// - 32% = 8
/// - 36% = 9
/// - 40% = 10
/// - 44% = 11
/// - 48% = 12
/// - 50% = 13 (0xD)
/// - 52% = 14
/// - 56% = 15
/// - 60% = 16
/// - 64% = 17
/// - 68% = 18
/// - 72% = 19
/// - 76% = 20
/// - 80% = 21
/// - 84% = 22
/// - 88% = 23
/// - 92% = 24
/// - 96% = 25
/// - 100% = 26 (0x1A)
pub fn sub_d097(ax: u16) -> u16 {
    match ax {
        13 => 50,
        x if x > 13 => (x - 1) * 4,
        x => x * 4,
    }
}

/// D0A7: Fade in
pub unsafe fn fade_in() {
    if BYTE_3D6A == 0 {
        set_vga_pal(&PALETTE);
        return;
    }

    WORD_3D6C = 20;
    fade_by_color(&PALETTE);
    sub_d161();
    BYTE_3D6A = 0;
}

/// D0C6
pub unsafe fn fade_in_pal(pal: &[u8]) {
    WORD_3D6C = 20;
    fade_by_color(pal);
    sub_d161();
    BYTE_3D6A = 0;
}

/// D0D8: Fade out
pub unsafe fn fade_out() {
    if BYTE_3D6A != 0 {
        set_vga_pal(&PALETTE2);
        return;
    }

    WORD_3D6C = 20;
    fade_by_color(&PALETTE2);
    sub_d161();
    BYTE_3D6A = 0xFF;
}

/// D0F7
pub unsafe fn fade_in2() {
    if BYTE_3D6A == 0 {
        set_vga_pal(&PALETTE);
        return;
    }

    WORD_3D6C = 20;
    fade_by_color(&PALETTE);
    BYTE_3D6A = 0;
}

/// D113
pub unsafe fn fade_out2() {
    if BYTE_3D6A != 0 {
        set_vga_pal(&PALETTE2);
        return;
    }

    WORD_3D6C = 20;
    fade_by_color(&PALETTE2);
    BYTE_3D6A = 0xFF;
}

/// D12C: Fade in/out some palette colors
pub unsafe fn fade_by_color(pal: &[u8]) {
    sub_d161();

    let mut di = 0;

    for (bx, ah) in pal.iter().enumerate() {
        VGA_FADE_PAL[di] = (i16::from_le_bytes([0, ah.wrapping_sub(VGA_PAL[bx])]) as i32)
            .saturating_div(WORD_3D6C.into()) as u16;
        di += 1;

        VGA_FADE_PAL[di] = u16::from_le_bytes([0, VGA_PAL[bx]]);
        di += 1;
    }

    set_fade_vga_pal();

    FADE_STEP = WORD_3D6C;
    VGA_PAL_READY = true;
}

/// D161
pub unsafe fn sub_d161() {
    loop {
        sub_d6f9();

        if FADE_STEP == 0 {
            break;
        }
    }
}

/// D16C
pub unsafe fn sub_d16c() {
    while VGA_PAL_READY {
        asm!("nop");
    }
}

/// D174: Copy words from `VGA_FADE_PAL` (and update every 2nd byte) to `VGA_PAL`
pub unsafe fn set_fade_vga_pal() {
    for (dst, src) in VGA_PAL.iter_mut().zip(VGA_FADE_PAL.chunks_exact_mut(2)) {
        src[1] += src[0];
        *dst = src[1].to_le_bytes()[1];
    }
}

/// D18C: Copy palette
pub unsafe fn set_vga_pal(pal: &[u8; 768]) {
    sub_d161();

    BYTE_3D6A = if pal.as_ptr() == PALETTE2.as_ptr() {
        0xFF
    } else {
        0
    };
    VGA_PAL_READY = false;
    VGA_PAL = *pal;
    VGA_PAL_READY = true;
}

/// D1B6: Perform fade in/out step by updating palette and flushing it
pub unsafe fn fade_step() {
    if !VGA_PAL_READY {
        FADE_STEP = 0;
        return;
    }

    FADE_STEP = FADE_STEP.saturating_sub(1);
    flush_vga_pal();

    if FADE_STEP > 0 {
        set_fade_vga_pal();
    } else {
        VGA_PAL_READY = false;
    }
}

/// D1D7: Set VGA DAC colors
pub unsafe fn flush_vga_pal() {
    asm!(
        "pushf",
        "push ecx",
        "push edx",
        "push esi",
        "push {src:e}",
        "push {len:e}",
        "pop ecx",
        "pop esi",
        "cld",
        "mov dx, 0x3C8",
        "xor al, al",
        "out dx, al",
        "inc dx",
        ".byte 0x67, 0xF3, 0x6E",
        "pop esi",
        "pop edx",
        "pop ecx",
        "popf",
        src = in(reg) &raw const VGA_PAL,
        len = in(reg) VGA_PAL.len() as u32,
        lateout("ax") _,
    );
}

static mut WORD_D379: u16 = 0;

/// D1F6: Sets vga 320x200 mode.
pub unsafe fn set_video_mode_and_timer() {
    WORD_3E16 = inw::<PIT_CHANNEL_0>().swap_bytes(); // NB: order is big endian
    BYTE_3D72 = get_default_disk();
    DOS_VID_MODE = get_video_mode().0;
    set_video_mode(0x13);

    BYTE_3E2C = reset_mouse();

    cli();
    save_interrupt_vectors();

    outb::<PIT_MODE>(PIT_CH2_MODE0);
    outb::<PIT_CHANNEL_2>(0);
    outb::<PIT_CHANNEL_2>(0);
    outb::<0x61>(inb::<0x61>() | 1);

    while inb::<0x3DA>() & 0b1000 != 0 {}

    while inb::<0x3DA>() & 0b1000 == 0 {}
    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let bx = inw::<PIT_CHANNEL_2>();

    while inb::<0x3DA>() & 0b1000 != 0 {}
    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let cx = inw::<PIT_CHANNEL_2>();

    while inb::<0x3DA>() & 0b1000 == 0 {}
    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let dx = inw::<PIT_CHANNEL_2>();

    WORD_3D7A = bx - dx;
    BYTE_3D74 = if (bx - cx) * 2 < WORD_3D7A { 0xFF } else { 0 };

    // PC/XT PPI port B bits:
    // 0: Tmr 2 gate --- OR 03H=spkr ON
    // 1: Tmr 2 data -+  AND 0fcH=spkr OFF
    // 3: 1=read high switches
    // 4: 0=enable RAM parity checking
    // 5: 0=enable I/O channel check
    // 6: 0=hold keyboard clock low
    // 7: 0=enable kbrd
    outb::<0x61>(inb::<0x61>() & 0b1111_1110);

    let max = (0..u16::MAX)
        .map(|_| {
            outb::<PIT_MODE>(0);
            inw::<PIT_CHANNEL_0>()
        })
        .max()
        .unwrap_or_default();

    fn mln_div(max: u16) -> u16 {
        let max = (max as u32).max(1);

        (0x1_0000 / max + if ((0x1_0000 % max) << 1) > max { 1 } else { 0 }) as u16
    }

    let cx = mln_div(max);
    WORD_3D78 = if cx == 1 { 0 } else { mln_div(cx) };

    outb::<PIT_MODE>(PIT_CH2_MODE0);
    outb::<PIT_CHANNEL_2>(0);
    outb::<PIT_CHANNEL_2>(0);
    outb::<0x61>(inb::<0x61>() | 1);

    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let bx = inw::<PIT_CHANNEL_2>();

    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let dx = inw::<PIT_CHANNEL_2>();

    loop_self(5000);

    outb::<PIT_MODE>(PIT_CH2_LATCH);
    let cx = inw::<PIT_CHANNEL_2>();

    // see https://www.vogons.org/viewtopic.php?p=308588#p308588
    WORD_3D76 = (5962500 / 1.max(dx - cx - bx - dx) as u32) as u16;

    outb::<0x61>(inb::<0x61>() & 0b1111_1110);
    sti();
    cli();

    const IOPL_BITS: u16 = 0b11 << 12; // IOPL bits (12 and 13)

    set_flags(get_flags() | IOPL_BITS);
    let flags = get_flags();

    // clear IOPL bits if they are set
    if (flags & IOPL_BITS) != 0 {
        set_flags(flags & !IOPL_BITS);

        WORD_D379 = 2;
    }

    sti();
}

/// D37B
pub unsafe fn sub_d37b() {
    set_dos_ceh();
    set_kbd_isr();
    // mov ax, sub_D926
    prepare_task_context();
    set_timer();
    set_vga_pal(&PALETTE2);

    ISRS_SET = true;
}

/// D396
pub unsafe fn sub_d396() {
    if ISRS_SET {
        // fade_out();
        restore_timer();
        restore_kbd_isr();
        restore_dos_ceh();

        ISRS_SET = false;
    }

    set_video_mode(DOS_VID_MODE);
}

unsafe fn get_default_disk() -> u8 {
    let disk: u8;
    asm!(
        "push ds",
        "push es",
        "mov ah, 0x19",
        "int 0x21",
        "pop es",
        "pop ds",
        out("al") disk,
    );
    disk
}

unsafe fn get_video_mode() -> (u8, u8, u8) {
    let mode: u8;
    let columns: u8;
    let page: u8;
    asm!(
        "push ds",
        "push es",
        "mov ah, 0xF",
        "int 0x10",
        "pop es",
        "pop ds",
        "mov {}, al",
        "mov {}, ah",
        "mov {}, bh",
        out(reg_byte) mode,
        out(reg_byte) columns,
        out(reg_byte) page,
    );
    (mode, columns, page)
}

unsafe fn set_video_mode(mode: u8) {
    asm!(
        "push ds",
        "push es",
        "int 0x10",
        "pop es",
        "pop ds",
        in("ax") mode as u16,
    );
}

unsafe fn reset_mouse() -> u8 {
    let mut status: u16 = 0;
    asm!(
        "push ds",
        "push es",
        "int 0x33",
        "pop es",
        "pop ds",
        inout("ax") status,
    );
    status as u8
}

/// https://osdev.wiki/wiki/Interrupt_Vector_Table
unsafe fn save_interrupt_vectors() {
    ORIG_PIT_ISR = transmute::<u32, FarPointer>(getd::<0, 0x20>());
    ORIG_KBD_ISR = transmute::<u32, FarPointer>(getd::<0, 0x24>());
    ORIG_DOS_CEH_ISR = transmute::<u32, FarPointer>(getd::<0, 0x90>());
}

pub unsafe fn get_flags() -> u16 {
    let mut flags: u16;
    asm!(
        "pushf",
        "pop {:x}",
        out(reg) flags,
    );
    flags
}

unsafe fn set_flags(flags: u16) {
    asm!(
        "push {:x}",
        "popf",
        in(reg) flags,
    );
}

pub unsafe fn loop_self(times: u16) {
    asm!(
        "2:",
        "loop 2b",
        in("cx") times,
    );
}
