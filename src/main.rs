#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn, static_mut_refs)]
#![allow(dead_code)]
#![feature(alloc_error_handler)]
#![feature(naked_functions_rustic_abi)]

use core::{
    arch::{asm, naked_asm},
    panic::PanicInfo,
};

extern crate alloc;

use crate::{
    archive::open_and_read_data_file,
    data::*,
    dos::{exit, get_data_seg, sub_db28},
    game::loc_5371,
    hud::{VGA_DBL_BUF, VGA_SIZE},
    intro::show_intro,
    mem::GLOBAL_ALLOCATOR,
    menu::enter_main_menu,
    protection::protection_screen,
    timer::prepare_task_context,
    video::{set_video_mode_and_timer, sub_d37b, sub_d396},
};

mod archive;
mod bitmap;
mod chars;
mod config;
mod crc;
mod data;
mod dos;
mod game;
mod hud;
mod intro;
mod magazine;
mod mem;
mod menu;
mod prepare;
mod protection;
mod sound;
mod sprite;
mod state;
mod timer;
mod video;

/// # Safety
///
/// This the main entry point.
#[unsafe(link_section = ".startup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    asm!(
        "cli",
        "mov bx, es",
        "mov ax, cs",
        "add ax, {data_seg_delta}",
        "mov ds, ax",
        "mov ss, ax",
        "mov esp, {data_stack_top}",
        "mov es, ax",
        "mov [{w5160}], bx",
        "mov [{w5164}], ax",
        "sti",
        w5160 = sym WORD_5160,
        w5164 = sym WORD_5164,
        data_seg_delta = const DATA_SEG_DELTA,
        data_stack_top = const DATA_STACK_TOP,
        lateout("ax") _,
        lateout("bx") _,
    );

    WORD_5162 = get_data_seg();

    GLOBAL_ALLOCATOR.init();
    VGA_DBL_BUF.resize(VGA_SIZE, 0);

    set_video_mode_and_timer();
    open_and_read_data_file();
    sub_d37b();
    sub_db28();
    protection_screen();

    WORD_1F26 = loc_51c4 as *const () as usize;
    prepare_task_context(loc_5371);
    show_intro();

    // TODO:

    loc_51c4();
}

#[unsafe(naked)]
unsafe extern "C" fn loc_51c4() -> ! {
    naked_asm!(
        "cli",
        "mov ax, cs",
        "add ax, {data_seg_delta}",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        "mov esp, {data_stack_top}",
        "sti",
        "jmp {body}",
        data_seg_delta = const DATA_SEG_DELTA,
        data_stack_top = const DATA_STACK_TOP,
        body = sym loc_51c4_body,
    );
}

unsafe fn loc_51c4_body() -> ! {
    loop {
        PLAYING_DEMO = 0;
        BYTE_3D86 = false;
        BYTE_3D87 = 0;
        BYTE_3D88 = [0; 128];
        WORD_3E08 = 0;

        enter_main_menu();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        sub_d396();
        println!("{}", info);
        exit(1)
    }
}
