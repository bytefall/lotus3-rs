use core::{
    arch::{asm, naked_asm},
    mem::{swap, transmute},
};

use crate::{
    config::{CFG, JoyState},
    data::*,
    dos::{cli, inb, inw, outb, read_word, setd, sti, sub_dad1},
    game::loc_5283,
    video::{fade_step, get_flags, loop_self},
};

/// PS/2 data port
const PS2_DATA: u16 = 0x60;

/// PS/2 status bits
const PS2_STATUS: u16 = 0x61;

/// Primary PIC Command and Status Register
const PIC_CSR: u16 = 0x20;

/// Primary PIC Interrupt Mask Register and Data Register
const PIC_IMR: u16 = 0x21;

/// End of Interrupt (EOI)
const PIC_EOI: u8 = 0x20;

/// Channel 0 data port (read/write)
pub const PIT_CHANNEL_0: u16 = 0x40;

/// Channel 2 data port (read/write)
pub const PIT_CHANNEL_2: u16 = 0x42;

/// Mode/Command register (write only, a read is ignored)
pub const PIT_MODE: u16 = 0x43;

/// 00  = Channel 0
/// 11  = Access mode: lobyte/hibyte
/// 011 = Mode 3: square wave generator
/// 0   = 16-bit binary
#[allow(clippy::unusual_byte_groupings)]
const PIT_CH0_MODE3: u8 = 0b11_011_0;

/// 10  = Channel 2
/// 11  = Access mode: lobyte/hibyte
/// 000 = Mode 0: interrupt on terminal count
/// 0   = 16-bit binary
#[allow(clippy::unusual_byte_groupings)]
pub const PIT_CH2_MODE0: u8 = 0b10_11_000_0;

/// 10  = Channel 2
/// 00  = Latch count value command
/// 000 = Mode 0: interrupt on terminal count
/// 0   = 16-bit binary
#[allow(clippy::unusual_byte_groupings)]
pub const PIT_CH2_LATCH: u8 = 0b10_00_000_0;

/// D3B6: Set INT 24h (DOS critical error handler) vector to point to sub_d3da
pub unsafe fn set_dos_ceh() {
    asm!(
        "push es",
        "mov es, {seg:x}",
        "mov es:[0x90], {off:x}",
        "mov es:[0x92], cs",
        "pop es",
        seg = in(reg) 0_u16,
        off = in(reg) dos_ceh_isr as *const () as u16,
    );
}

/// D3C7: Restore INT 24h (DOS critical error handler) vector using stored WORD_3D7E / WORD_3D80
pub unsafe fn restore_dos_ceh() {
    setd::<0, 0x90>(transmute::<FarPointer, u32>(ORIG_DOS_CEH_ISR));
}

/// D3DA: ISR for DOS critical error handler
///
/// http://www.techhelpmanual.com/564-int_24h__critical_error_handler.htm
#[unsafe(naked)]
unsafe extern "C" fn dos_ceh_isr() {
    naked_asm!("xor ax, ax", "iret");
}

/// D3DD: Set INT 9h (keyboard hardware interrupt) vector
pub unsafe fn set_kbd_isr() {
    clear_kbd_buf();

    cli();
    asm!(
        "push es",
        "mov es, {seg:x}",
        "mov es:[0x24], {off:x}",
        "mov es:[0x26], cs",
        "pop es",
        seg = in(reg) 0_u16,
        off = in(reg) kbd_isr as *const () as u16,
    );
    sti();
}

/// D3F3:
unsafe fn clear_kbd_buf() {
    BYTE_3D88 = [0; 128];
}

/// D400: Restore the old INT 9h handler
pub unsafe fn restore_kbd_isr() {
    // restore old INT 9h handler
    cli();
    setd::<0, 0x24>(transmute::<FarPointer, u32>(ORIG_KBD_ISR));
    sti();

    // drain the BIOS keyboard buffer (using INT 16h) so that no keystrokes remain
    while kbd_get_key().is_some() {}
}

/// D421: Retrieve next pending key from WORD_3E08, normalize character, uppercase conversion, return in AX
pub unsafe fn sub_d421() -> u8 {
    let al = match WORD_3E08.to_le_bytes() {
        [0, 0] => return 0,
        [0, ah] => ah | 0b1000_0000,
        [al, _] => al,
    };

    WORD_3E08 = 0;

    if al.is_ascii_lowercase() {
        al - b' ' // to uppercase
    } else {
        al
    }
}

/// https://wiki.osdev.org/PS/2_Keyboard
const ESC: u8 = 0x01;
const CTRL: u8 = 0x1D;
const KEY_A: u8 = 0x1E;
const KEY_SQ: u8 = 0x28;
const SHIFT: u8 = 0x2A;
const ASTERISK: u8 = 0x37;
const ALT: u8 = 0x38;
const CAPS: u8 = 0x3A;

/// D445: Keyboard interrupt handler (ISR) for IRQ1 (INT 09h)
/// https://github.com/rust-lang/rust/issues/40180
#[unsafe(naked)]
unsafe extern "C" fn kbd_isr() {
    naked_asm!(
        "cld",
        "push ds",
        "push es",
        "pushal",
        "push cs",
        "pop ds",
        "push ds",
        "pop es",
        "call {body}",
        "popal",
        "pop es",
        "pop ds",
        "iret",
        body = sym kbd_isr_body,
    );
}

unsafe extern "C" fn kbd_isr_body() {
    // asm!("pusha", "push ds", "push es");
    // mov ds, [cs:word_5162]

    let scan_code = inb::<PS2_DATA>();
    let bl = scan_code & 0b111_1111;

    if !matches!(bl, 0x60 | SHIFT) {
        BYTE_3D86 = scan_code < 0x80; // store press/release flag
        BYTE_3D87 = bl; // store last scan code

        if (BYTE_3D86 as u8) != BYTE_3D88[bl as usize] {
            BYTE_3D88[bl as usize] = BYTE_3D86 as u8;
        }
    }

    if matches!(bl, ASTERISK | CTRL | 0x7F) {
        // loc_D49D
        outb::<PIC_CSR>(PIC_EOI);

        let v = inb::<PS2_STATUS>();
        outb::<PS2_STATUS>(v | 0b1000_0000);
        outb::<PS2_STATUS>(v);
    } else {
        asm!(
            "pushf",
            "lcall [{}]",
            "push cs",
            "pop ds",
            "push ds",
            "pop es",
            sym ORIG_KBD_ISR,
        );
        cli();

        while let Some(key) = kbd_get_key() {
            WORD_3E08 = key;
        }
    }

    // iret_D499
    // asm!("pop es", "pop ds", "popa", "iret");
}

/// D4AF: Program the PIT and set INT 08h handler to `sub_D508`
pub unsafe fn set_timer() {
    WORD_3E12 = WORD_3D7A / 4 - 90;
    BYTE_3E0E = 0;

    cli();
    outb::<PIT_MODE>(PIT_CH0_MODE3);
    outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[0]);
    outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[1]);
    asm!(
        "push es",
        "mov es, {seg:x}",
        "mov es:[0x20], {off:x}",
        "mov es:[0x22], cs",
        "pop es",
        seg = in(reg) 0_u16,
        off = in(reg) timer_isr as *const () as u16,
    );
    sti();
}

/// D4E3: Restore the original PIT divisor and INT 08h handler
pub unsafe fn restore_timer() {
    cli();
    outb::<PIT_MODE>(PIT_CH0_MODE3);
    outb::<PIT_CHANNEL_0>(WORD_3D78.to_le_bytes()[0]);
    outb::<PIT_CHANNEL_0>(WORD_3D78.to_le_bytes()[1]);
    setd::<0, 0x20>(transmute::<FarPointer, u32>(ORIG_PIT_ISR));
    sti();
}

/// D508: PIT ISR
#[unsafe(naked)]
unsafe extern "C" fn timer_isr() {
    naked_asm!(
        "cld",
        "push ds",
        "push es",
        "pushal",
        "push cs",
        "pop ds",
        "push ds",
        "pop es",
        "call {body}",
        "popal",
        "pop es",
        "pop ds",
        "iret",
        body = sym timer_isr_body,
    );
}

unsafe extern "C" fn timer_isr_body() {
    // asm!("pusha", "push ds", "push es");
    // mov ds, [cs:word_5162]

    // let word_6352 = WORD_6352;
    // let word_6354 = WORD_6354;
    // let word_6356 = WORD_6356;
    // let word_6358 = WORD_6358;
    // let word_635a = WORD_635A;
    // let word_635c = WORD_635C;
    // let word_635e = WORD_635E;
    // let word_6360 = WORD_6360;
    // let word_6362 = WORD_6362;
    // let word_6364 = WORD_6364;
    // let word_6366 = WORD_6366;
    // let word_6368 = WORD_6368;
    // let word_636a = WORD_636A;
    // let word_636c = WORD_636C;
    // let word_636e = WORD_636E;
    // let word_6370 = WORD_6370;
    // let word_6372 = WORD_6372;
    // let word_6374 = WORD_6374;
    // let word_6376 = WORD_6376;
    // let word_6378 = WORD_6378;
    // let word_637a = WORD_637A;
    // let word_637c = WORD_637C;
    // let word_637e = WORD_637E;
    // let word_6380 = WORD_6380;
    // let word_6382 = WORD_6382;
    // let word_6384 = WORD_6384;
    // let word_6386 = WORD_6386;
    // let word_6388 = WORD_6388;
    // let word_638a = WORD_638A;
    // let word_638c = WORD_638C;
    // let word_638e = WORD_638E;
    // let word_6390 = WORD_6390;

    // mov ds, [cs:word_5162]
    // asm!("cld");
    BYTE_3E10 = inb::<PIC_IMR>();
    outb::<PIC_IMR>(BYTE_3E10 | 0b111_1111); // disable all, keep IRQ 7 as before (bit 0 = enable, 1 = disable)

    BYTE_3E0E += 1;

    if BYTE_3E0E >= 4 {
        BYTE_3E0E = 0;

        outb::<PIT_MODE>(PIT_CH0_MODE3);

        while (inb::<0x3DA>() ^ BYTE_3D74) & 0b1000 != 0 {}

        outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[0]);
        outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[1]);
    }

    // loc_D5CD
    if let Some(v) = WORD_3E14.checked_sub(WORD_3D7A / 4) {
        WORD_3E14 = v;

        outb::<PIC_CSR>(PIC_EOI);
    } else {
        WORD_3E14 += WORD_3D78;

        asm!(
            "pushf",
            "lcall [{}]",
            "push cs",
            "pop ds",
            "push ds",
            "pop es",
            sym ORIG_PIT_ISR,
        );
    }

    // loc_D5EB
    sti();

    if BYTE_3E0E == 0 {
        fade_step();
    }

    BYTE_3E0F -= 1;

    if BYTE_3E0F >= 0 {
        BYTE_3E0F += 6;

        if BYTE_3E0F < 0 {
            BYTE_3E0F = 0;
        }

        // TODO:
        /*
        if matches!(WORD_3E1C, 0 | 0x8000) {
            // mov     ax, ss
            // cmp     ax, [cs:word_5164]
            // jne     loc_D63E
            // mov     bp, sp
            // cmp     word [bp + 56h], code_seg
            // jnz     loc_D63E
            sub_d9eb();
            // mov     bx, [word_3E1C]
            // mov     [bx + word_3E1E], sp
            // mov     sp, [word_3E20]
            WORD_3E1C = 2;
        }*/
    }

    // loc_D63E
    WORD_3E16 += 1;
    WORD_3E18 += 1;

    cli();
    outb::<PIC_IMR>(BYTE_3E10);

    // TODO:
    /*WORD_3E1A += 1;

    if WORD_3E1A <= 1 {
        let tmp = WORD_3E1C;
        WORD_3E1C |= 0x8000;

        loop {
            sti();
            sub_211e();
            cli();

            WORD_3E1A -= 1;

            if WORD_3E1A == 0 {
                break;
            }
        }

        WORD_3E1C = tmp;
    }*/

    // mov ds, [cs:word_5162]
    // WORD_6390 = word_6390;
    // WORD_638E = word_638e;
    // WORD_638C = word_638c;
    // WORD_638A = word_638a;
    // WORD_6388 = word_6388;
    // WORD_6386 = word_6386;
    // WORD_6384 = word_6384;
    // WORD_6382 = word_6382;
    // WORD_6380 = word_6380;
    // WORD_637E = word_637e;
    // WORD_637C = word_637c;
    // WORD_637A = word_637a;
    // WORD_6378 = word_6378;
    // WORD_6376 = word_6376;
    // WORD_6374 = word_6374;
    // WORD_6372 = word_6372;
    // WORD_6370 = word_6370;
    // WORD_636E = word_636e;
    // WORD_636C = word_636c;
    // WORD_636A = word_636a;
    // WORD_6368 = word_6368;
    // WORD_6366 = word_6366;
    // WORD_6364 = word_6364;
    // WORD_6362 = word_6362;
    // WORD_6360 = word_6360;
    // WORD_635E = word_635e;
    // WORD_635C = word_635c;
    // WORD_635A = word_635a;
    // WORD_6358 = word_6358;
    // WORD_6356 = word_6356;
    // WORD_6354 = word_6354;
    // WORD_6352 = word_6352;

    // asm!("pop es", "pop ds", "popa", "iret");
}

/// D6F9: Essentially it’s a watchdog or re-synchronization loop for the system timer tick, ensuring the PIT counter doesn’t stall or go out of bounds.
/// Intel 8253/8254 PIT (Programmable Interval Timer) ports (43h = PIT control, 40h = channel 0).
pub unsafe fn sub_d6f9() {
    unsafe fn read_pit_counter() -> u16 {
        cli();
        outb::<PIT_MODE>(0);
        let val = inw::<PIT_CHANNEL_0>();
        sti();
        val
    }

    loop {
        let one = read_pit_counter();
        loop_self(WORD_3D76);
        let two = read_pit_counter();

        if one == two || one >= WORD_3E12 {
            cli();
            outb::<PIT_MODE>(PIT_CH0_MODE3);
            outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[0]);
            outb::<PIT_CHANNEL_0>(WORD_3E12.to_le_bytes()[1]);
            sti();
        } else {
            break;
        }
    }
}

/// D740:
pub unsafe fn prepare_task_context() {
    return;
    /*
    asm!(
        "mov ds, [cs:{w5162}]",
        "or word ptr [{w3e1c}], 0x8000",
        "mov [{w3e20}], sp",

        // load new stack pointer
        "mov sp, 0x1400",

        // enable interrupts (so handlers can run briefly while switching)
        "sti",

        // save flags, CS, AX and general regs / segments and push many saved words
        "pushf",
        "push cs",
        "push ax",
        "pusha",
        "push ds",
        "push es",

        // reload DS from CS:WORD_5162 (again)
        "mov ds, [cs:{w5162}]",

        // push the saved words block (word_6352 .. word_6390)
        "push word ptr [{w6352}]",
        "push word ptr [{w6354}]",
        "push word ptr [{w6356}]",
        "push word ptr [{w6358}]",
        "push word ptr [{w635a}]",
        "push word ptr [{w635c}]",
        "push word ptr [{w635e}]",
        "push word ptr [{w6360}]",
        "push word ptr [{w6362}]",
        "push word ptr [{w6364}]",
        "push word ptr [{w6366}]",
        "push word ptr [{w6368}]",
        "push word ptr [{w636a}]",
        "push word ptr [{w636c}]",
        "push word ptr [{w636e}]",
        "push word ptr [{w6370}]",
        "push word ptr [{w6372}]",
        "push word ptr [{w6374}]",
        "push word ptr [{w6376}]",
        "push word ptr [{w6378}]",
        "push word ptr [{w637a}]",
        "push word ptr [{w637c}]",
        "push word ptr [{w637e}]",
        "push word ptr [{w6380}]",
        "push word ptr [{w6382}]",
        "push word ptr [{w6384}]",
        "push word ptr [{w6386}]",
        "push word ptr [{w6388}]",
        "push word ptr [{w638a}]",
        "push word ptr [{w638c}]",
        "push word ptr [{w638e}]",
        "push word ptr [{w6390}]",

        // swap SP with saved SP location: xchg sp, [word_3E20]
        "xchg sp, [{w3e20}]",

        // clear the high bit in [word_3E1C] -> AND with 0x7FFF
        "and word ptr [{w3e1c}], 0x7FFF",
        w5162 = sym WORD_5162,
        w3e1c = sym WORD_3E1C,
        w3e20 = sym WORD_3E20,
        w6352 = sym WORD_6352,
        w6354 = sym WORD_6354,
        w6356 = sym WORD_6356,
        w6358 = sym WORD_6358,
        w635a = sym WORD_635A,
        w635c = sym WORD_635C,
        w635e = sym WORD_635E,
        w6360 = sym WORD_6360,
        w6362 = sym WORD_6362,
        w6364 = sym WORD_6364,
        w6366 = sym WORD_6366,
        w6368 = sym WORD_6368,
        w636a = sym WORD_636A,
        w636c = sym WORD_636C,
        w636e = sym WORD_636E,
        w6370 = sym WORD_6370,
        w6372 = sym WORD_6372,
        w6374 = sym WORD_6374,
        w6376 = sym WORD_6376,
        w6378 = sym WORD_6378,
        w637a = sym WORD_637A,
        w637c = sym WORD_637C,
        w637e = sym WORD_637E,
        w6380 = sym WORD_6380,
        w6382 = sym WORD_6382,
        w6384 = sym WORD_6384,
        w6386 = sym WORD_6386,
        w6388 = sym WORD_6388,
        w638a = sym WORD_638A,
        w638c = sym WORD_638C,
        w638e = sym WORD_638E,
        w6390 = sym WORD_6390,
    );*/
}

/// D7E9:
pub unsafe fn resume_task_context() {
    return;
    /*
    asm!(
        // pop word [cs:word_516A]
        // This pops a word from stack and stores it to the memory location WORD_516A in CS.
        "pop word ptr cs:[{w516a}]",

        // pushf ; push flags to stack
        "pushf",
        // push cs ; push CS
        "push cs",
        // push word [cs:word_516A] ; push the value we just stored
        "push word ptr cs:[{w516a}]",
        // pusha ; save general registers
        "pusha",
        "push ds",
        "push es",

        // DS = [cs:WORD_5162]
        "mov ds, [cs:{w5162}]",

        // push large saved block (word_6352 .. word_6390)
        "push word ptr [{w6352}]",
        "push word ptr [{w6354}]",
        "push word ptr [{w6356}]",
        "push word ptr [{w6358}]",
        "push word ptr [{w635a}]",
        "push word ptr [{w635c}]",
        "push word ptr [{w635e}]",
        "push word ptr [{w6360}]",
        "push word ptr [{w6362}]",
        "push word ptr [{w6364}]",
        "push word ptr [{w6366}]",
        "push word ptr [{w6368}]",
        "push word ptr [{w636a}]",
        "push word ptr [{w636c}]",
        "push word ptr [{w636e}]",
        "push word ptr [{w6370}]",
        "push word ptr [{w6372}]",
        "push word ptr [{w6374}]",
        "push word ptr [{w6376}]",
        "push word ptr [{w6378}]",
        "push word ptr [{w637a}]",
        "push word ptr [{w637c}]",
        "push word ptr [{w637e}]",
        "push word ptr [{w6380}]",
        "push word ptr [{w6382}]",
        "push word ptr [{w6384}]",
        "push word ptr [{w6386}]",
        "push word ptr [{w6388}]",
        "push word ptr [{w638a}]",
        "push word ptr [{w638c}]",
        "push word ptr [{w638e}]",
        "push word ptr [{w6390}]",

        // cli
        "cli",

        // mov [word_3E20], sp
        "mov [{w3e20}], sp",

        // mov word [word_3E1C], 0
        "mov word ptr [{w3e1c}], 0",

        // mov sp, [word_3E1E]
        "mov sp, [{w3e1e}]",

        // mov ds, [cs:word_5162]
        "mov ds, [cs:{w5162}]",

        // pop the saved block in reverse order (restore into the memory words)
        "pop word ptr [{w6390}]",
        "pop word ptr [{w638e}]",
        "pop word ptr [{w638c}]",
        "pop word ptr [{w638a}]",
        "pop word ptr [{w6388}]",
        "pop word ptr [{w6386}]",
        "pop word ptr [{w6384}]",
        "pop word ptr [{w6382}]",
        "pop word ptr [{w6380}]",
        "pop word ptr [{w637e}]",
        "pop word ptr [{w637c}]",
        "pop word ptr [{w637a}]",
        "pop word ptr [{w6378}]",
        "pop word ptr [{w6376}]",
        "pop word ptr [{w6374}]",
        "pop word ptr [{w6372}]",
        "pop word ptr [{w6370}]",
        "pop word ptr [{w636e}]",
        "pop word ptr [{w636c}]",
        "pop word ptr [{w636a}]",
        "pop word ptr [{w6368}]",
        "pop word ptr [{w6366}]",
        "pop word ptr [{w6364}]",
        "pop word ptr [{w6362}]",
        "pop word ptr [{w6360}]",
        "pop word ptr [{w635e}]",
        "pop word ptr [{w635c}]",
        "pop word ptr [{w635a}]",
        "pop word ptr [{w6358}]",
        "pop word ptr [{w6356}]",
        "pop word ptr [{w6354}]",
        "pop word ptr [{w6352}]",

        // pop es
        "pop es",
        // pop ds
        "pop ds",
        // popa
        "popa",
        // iret
        "iret",
        w5162 = sym WORD_5162,
        w516a = sym WORD_516A,
        w6352 = sym WORD_6352,
        w6354 = sym WORD_6354,
        w6356 = sym WORD_6356,
        w6358 = sym WORD_6358,
        w635a = sym WORD_635A,
        w635c = sym WORD_635C,
        w635e = sym WORD_635E,
        w6360 = sym WORD_6360,
        w6362 = sym WORD_6362,
        w6364 = sym WORD_6364,
        w6366 = sym WORD_6366,
        w6368 = sym WORD_6368,
        w636a = sym WORD_636A,
        w636c = sym WORD_636C,
        w636e = sym WORD_636E,
        w6370 = sym WORD_6370,
        w6372 = sym WORD_6372,
        w6374 = sym WORD_6374,
        w6376 = sym WORD_6376,
        w6378 = sym WORD_6378,
        w637a = sym WORD_637A,
        w637c = sym WORD_637C,
        w637e = sym WORD_637E,
        w6380 = sym WORD_6380,
        w6382 = sym WORD_6382,
        w6384 = sym WORD_6384,
        w6386 = sym WORD_6386,
        w6388 = sym WORD_6388,
        w638a = sym WORD_638A,
        w638c = sym WORD_638C,
        w638e = sym WORD_638E,
        w6390 = sym WORD_6390,
        w3e20 = sym WORD_3E20,
        w3e1c = sym WORD_3E1C,
        w3e1e = sym WORD_3E1E,
        options(noreturn, nostack)
    );*/
}

pub unsafe fn sub_d915() {
    WORD_3E1C = 0x8000;
    // pop     ax                         ; ax = return point from sub_D915
    // mov     sp, 1800h
    // push    ax

    // mov ax, sub_D926
    prepare_task_context();
}

pub unsafe fn sub_d926() {
    loop {
        resume_task_context();
    }
}

pub unsafe fn sub_d92b() {
    if PLAYING_DEMO == 0 {
        sub_d962();

        return;
    }

    let bx = if WORD_3E22 != 0 { WORD_3E22 } else { 916 };
    let ax = read_word(WORD_63BE, bx); // TODO: just read the word from WORD_63BE

    if ax == 0xFFFF {
        PLAYING_DEMO = 0;
        loc_5283();

        return;
    }

    WORD_3E22 = bx + 2;
    CFG.byte_16fe = ax.to_le_bytes()[0];
    CFG.byte_1715 = ax.to_le_bytes()[1];
}

pub unsafe fn sub_d962() {
    // player 1
    CFG.byte_16fe = sub_d989(match CFG.word_16ff {
        0 => sub_d99e(&CFG.p1_kbd),
        1 => sub_d9b4(&CFG.p1_joy),
        2 => sub_dad1(),
        _ => unreachable!(),
    });

    // player 2
    CFG.byte_1715 = sub_d989(match CFG.word_1716 {
        0 => sub_d99e(&CFG.p2_kbd),
        1 => sub_d9b4(&CFG.p2_joy),
        2 => sub_dad1(),
        _ => unreachable!(),
    });
}

/// Swap bits 2 and 3 of the input byte
pub fn sub_d989(al: u8) -> u8 {
    let bit3 = al & 8;
    let bit2 = al & 4;

    al & 0b1111_0011 | ((bit3 >> 1) | (bit2 << 1))
}

pub unsafe fn sub_d99e(buf: &[u8]) -> u8 {
    let mut dl = 0;

    for c in buf {
        let al = BYTE_3D88[*c as usize];

        dl = ((al & 1) << 7) | (dl >> 1);
    }

    dl >> 2
}

/// D9B4
pub fn sub_d9b4(joy: &JoyState) -> u8 {
    if joy.byte0 == 0 {
        return 0;
    }

    let mut dl = 0;

    for (a, b) in [
        (joy.word2, joy.word6),
        (joy.word8, joy.word2),
        (joy.word4, joy.word10),
        (joy.word12, joy.word4),
    ] {
        let carry = a < b;
        dl = (if carry { 0x80 } else { 0 }) | (dl >> 1);
    }

    (joy.byte1 << 4) | (dl >> 4)
}

/// D9EB
pub unsafe fn sub_d9eb() {
    let mut bx: u16 = 0;
    let mut bp: u16 = 0;
    let mut si: u16 = 0;
    let mut di: u16 = 0;
    let mut al: u8 = 0;

    BYTE_3E25 = 0;

    outb::<0x201>(0);

    // TODO: change this to shift_right
    fn ror_adc(b: &mut u8, w: &mut u16) {
        let cf = (*b & 1) != 0;

        *b = b.rotate_right(1);
        *w = w.wrapping_add(cf.into());
    }

    for _ in 0..=CFG.word_16fa.max(0x1000) {
        ror_adc(&mut al, &mut bx);
        ror_adc(&mut al, &mut bp);
        ror_adc(&mut al, &mut si);
        ror_adc(&mut al, &mut di);

        al = inb::<0x201>();

        if al == BYTE_3E24 {
            break;
        }
    }

    if BYTE_3E25 != 0 {
        return;
    }

    if bx >= 0x1000 {
        BYTE_3E24 &= 0xFC;
    }

    if si >= 0x1000 {
        BYTE_3E24 &= 0xF3;
    }

    let [al, ah] = u16::from_le_bytes([al, BYTE_3E24])
        .wrapping_shl(2)
        .to_le_bytes();
    let mut ax = !u16::from_le_bytes([al.rotate_left(2), ah]);
    let mut cx = ax;
    ax &= 0x101;
    cx &= 0x202;
    cx >>= 1;
    ax <<= 1;
    ax |= cx;

    if CFG.word_16fc != 0 {
        ax = ax.swap_bytes();
        swap(&mut bx, &mut si);
        swap(&mut bp, &mut di);
    }

    CFG.p1_joy.byte1 = ax.to_le_bytes()[0];
    CFG.p2_joy.byte1 = ax.to_le_bytes()[1];

    CFG.p1_joy.word2 += bx;
    CFG.p1_joy.word2 >>= 1;
    CFG.p1_joy.word4 += bp;
    CFG.p1_joy.word4 >>= 1;

    CFG.p2_joy.word2 += si;
    CFG.p2_joy.word2 >>= 1;
    CFG.p2_joy.word4 += di;
    CFG.p2_joy.word4 >>= 1;
}

/// Read the PS/2 keyboard. Return ASCII character is in AL and the scan code is in AH.
unsafe fn kbd_get_key() -> Option<u16> {
    // BIOS: get keyboard buffer status
    asm!(
        "push ds",
        "push es",
        "int 0x16",
        "pop es",
        "pop ds",
        in("ax") 0x100u16,
    );

    // ZF set if no key in buffer
    if get_flags() & (1 << 6) != 0 {
        return None;
    }

    // BIOS: read key (wait if empty)
    let mut key: u16;
    asm!(
        "push ds",
        "push es",
        "int 0x16",
        "pop es",
        "pop ds",
        inout("ax") 0u16 => key,
    );

    Some(key)
}
