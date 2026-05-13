use core::{
    arch::asm,
    fmt::{self, Write},
};

use crate::{
    config::CFG,
    crc::{RND_3EEE, RND_3EFE},
    data::*,
    video::get_flags,
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::dos::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

pub fn _print(args: fmt::Arguments) {
    DosWriter {}.write_fmt(args).unwrap();
}

struct DosWriter;

impl Write for DosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            unsafe {
                asm!(
                    "push ds",
                    "push es",
                    "int 0x21",
                    "pop es",
                    "pop ds",
                    in("ax") 0x200u16,
                    in("dl") c,
                )
            }
        }

        Ok(())
    }
}

pub unsafe fn break_in_debug() {
    asm!("int 3");
}

pub unsafe fn inb<const PORT: u16>() -> u8 {
    let mut v: u8;

    if PORT > u8::MAX.into() {
        asm!("in al, dx", out("al") v, in("dx") PORT);
    } else {
        asm!("in al, {0}", const PORT, out("al") v);
    }

    v
}

pub unsafe fn inw<const PORT: u16>() -> u16 {
    let mut v: u16;

    if PORT > u8::MAX.into() {
        asm!("in al, dx", "mov ah, al", "in al, dx", in("dx") PORT, out("ax") v);
    } else {
        asm!("in al, {0}", "mov ah, al", "in al, {0}", const PORT, out("ax") v);
    }

    v.swap_bytes()
}

pub unsafe fn outb<const PORT: u16>(data: u8) {
    if PORT > u8::MAX.into() {
        asm!("out dx, al", in("dx") PORT, in("al") data)
    } else {
        asm!("out {0}, al", const PORT, in("al") data)
    }
}

pub unsafe fn outw<const PORT: u16>(data: u16) {
    if PORT > u8::MAX.into() {
        asm!("out dx, al", "mov al, ah", "out dx, al", in("dx") PORT, in("ax") data);
    } else {
        asm!("out {0}, al", "mov al, ah", "out {0}, al", const PORT, in("ax") data);
    }
}

pub unsafe fn cli() {
    asm!("cli")
}

pub unsafe fn sti() {
    asm!("sti")
}

#[allow(clippy::empty_loop)]
pub unsafe fn exit(rt: u8) -> ! {
    asm!("mov ah, 0x4C", "int 0x21", in("al") rt);
    loop {}
}

pub unsafe fn get_data_seg() -> u16 {
    let mut ds: u16;
    asm!("mov {:x}, ds", out(reg) ds);
    ds
}

pub unsafe fn set_data_seg(ds: u16) {
    asm!("mov ds, {:x}", in(reg) ds);
}

pub unsafe fn getd<const SEGMENT: u16, const OFFSET: u16>() -> u32 {
    let v: u32;
    asm!(
        "push es",
        "mov es, {segment:x}",
        "mov {value}, es:[{offset}]",
        "pop es",
        segment = in(reg) SEGMENT,
        value = out(reg) v,
        offset = const OFFSET,
    );
    v
}

pub unsafe fn setd<const SEGMENT: u16, const OFFSET: u16>(v: u32) {
    asm!(
        "push es",
        "mov es, {segment:x}",
        "mov es:[{offset}], {value}",
        "pop es",
        segment = in(reg) SEGMENT,
        offset = const OFFSET,
        value = in(reg) v,
    );
}

pub unsafe fn read_word(segment: u16, offset: u16) -> u16 {
    let val;
    asm!(
        "push es",
        "mov es, {segment:x}",
        "mov di, {offset:x}",
        "mov {value:x}, es:[di]",
        "pop es",
        segment = in(reg) segment,
        offset = in(reg) offset,
        value = out(reg) val,
        out("di") _,
    );
    val
}

pub unsafe fn write_byte(segment: u16, offset: u16, value: u8) {
    asm!(
        "push es",
        "mov es, {segment:x}",
        "mov di, {offset:x}",
        "mov es:[di], {value:x}",
        "pop es",
        segment = in(reg) segment,
        value = in(reg) value as u16,
        offset = in(reg) offset,
        out("di") _,
    );
}

pub struct MouseState {
    pub buttons: u16, // bit 0 = left, bit 1 = right
    pub x: u16,       // column
    pub y: u16,       // row
}

/// Get mouse position and button state
unsafe fn mouse_get_position() -> MouseState {
    let mut m = MouseState {
        buttons: 0,
        x: 0,
        y: 0,
    };

    asm!(
        "push ds",
        "push es",
        "int 0x33",
        "pop es",
        "pop ds",
        "mov {btn:x}, bx",
        in("ax") 3,
        out("cx") m.x,
        out("dx") m.y,
        btn = out(reg) m.buttons,
    );

    m
}

/// DA94
pub unsafe fn sub_da94() {
    CFG.arr_172c = [0; 128];
    CFG.arr_172c[1] = 0xFF;
    CFG.arr_172c[25] = 0xFF;

    for i in &CFG.p1_kbd {
        CFG.arr_172c[*i as usize] = 0xFF;
    }

    for i in &CFG.p2_kbd {
        CFG.arr_172c[*i as usize] = 0xFF;
    }
}

/// DAD1
pub unsafe fn sub_dad1() -> u8 {
    if BYTE_3E2C == 0 {
        return 0;
    }

    let mut m = mouse_get_position();
    m.buttons &= 3;

    BYTE_3E2D = if m.buttons == 0 { 0 } else { 0xFF };
    WORD_3E2E = m.buttons;
    WORD_3E30 = m.x;
    WORD_3E32 = m.y;

    let two_bits = ((BYTE_3D88[80] & 1) << 1) | (BYTE_3D88[72] & 1);

    let mut al = if CFG.word_16f2 == 0 {
        ((m.buttons as u8) << 2) | two_bits
    } else {
        two_bits << 2 | m.buttons as u8
    };

    al = (al << 1) | if m.x < 0x200 { 0 } else { 1 };
    al = (al << 1) | if m.x < 0x80 { 0 } else { 1 };

    al
}

/// DB28
pub unsafe fn sub_db28() {
    sub_da94();

    let [lo, hi] = WORD_3E16.to_le_bytes();

    for i in &mut RND_3EEE.iter_mut().chain(RND_3EFE.iter_mut()) {
        *i = u32::from_le_bytes([lo, hi, lo, hi]);
    }

    // Fill ss:0C00h to ss:0D00h with some data
    // for bp in 0xC00..0xD00 {
    //     let mut ah = 0;
    //     let mut al = bp as u8;

    //     for _ in 0..8 {
    //         // let (v, cf) = al.overflowing_shl(1);
    //         // al = v;
    //         // ah += cf.into();
    //         al = al.rotate_left(1);
    //         ah = (ah << 1) | (al >> 7);
    //     }

    //     // BYTE_C00[bp] = ah; // TODO:
    // }
}

/// DBD9: Open a file.
pub unsafe fn file_open(file_name: &str) -> u16 {
    ERR_STR_PTR = file_name.as_ptr();

    let file_handle: u16;
    asm!(
        "push ds",
        "push es",
        "int 0x21",
        "pop es",
        "pop ds",
        inout("ax") 0x3D00_u16 => file_handle,
        in("dx") file_name.as_ptr(),
    );

    if get_flags() & 1 != 0 {
        loc_dc60(OPEN_ERROR);
    }

    file_handle
}

/// DBF1: Reads a file.
pub unsafe fn file_read(file_handle: u16, mut buf: impl AsMut<[u8]>) -> u16 {
    let buf = buf.as_mut();

    let (ds, dx) = if buf.as_ptr() as usize > 0xFFFF {
        let mem_pos = (get_data_seg() as u32) * 16 + buf.as_ptr() as u32;

        ((mem_pos / 16) as u16, (mem_pos % 16) as u16)
    } else {
        (get_data_seg(), buf.as_ptr() as u16)
    };

    let len: u16;
    asm!(
        "push ds",
        "push es",
        "mov ds, {call_ds:x}",
        "int 0x21",
        "pop es",
        "pop ds",
        inout("ax") 0x3F00_u16 => len,
        in("bx") file_handle,
        in("cx") buf.len(),
        in("dx") dx,
        call_ds = in(reg) ds,
    );

    if get_flags() & 1 != 0 {
        // close file
        asm!(
            "push ds",
            "push es",
            "int 0x21",
            "pop es",
            "pop ds",
            in("ax") 0x3E00_u16,
            in("bx") file_handle,
        );

        loc_dc60(READ_ERROR);
    }

    len
}

/// DC30: printf
pub unsafe fn printf(s: &str) {
    const ERR_STR_PLACEHOLDER: u8 = b'$';

    asm!(
        "mov     si, ax",
        "2:",
        "lodsb",
        "cmp     al, 0",
        "jz      4f",
        "cmp     al, {}",
        "jz      3f",
        "push    si",
        "push    ds",
        "push    es",
        "mov     dl, al",
        "mov     ah, 2",
        "int     0x21",
        "pop     es",
        "pop     ds",
        "pop     si",
        "jmp     2b",
        "3:",
        "push    si",
        "mov     si, {}",
        "call    2b",
        "pop     si",
        "jmp     2b",
        "4:",
        const ERR_STR_PLACEHOLDER,
        sym ERR_STR_PTR,
        in("ax") s.as_ptr(),
    )
}

// /// DC4F
// pub unsafe fn sub_dc4f() {
//     if WORD_63BA < WORD_63BE {
//         return;
//     }

//     loc_dc60(MEMORY_ERROR);
// }

/// DC60
pub unsafe fn loc_dc60(msg: &'static str) -> ! {
    //     mov     ds, [cs:word_5162]
    WORD_3EB9 = msg;
    BYTE_3EBB = 1;

    loc_dc70();
}

/// DC70
pub unsafe fn loc_dc70() -> ! {
    //     mov     ds, [cs:word_5162]
    //     cld
    //     call    sub_D396
    //     call    sub_209C
    //     call    sub_D3B6
    //     call    sub_5080
    //     call    sub_D3C7
    printf(WORD_3EB9);
    exit(BYTE_3EBB);
}
