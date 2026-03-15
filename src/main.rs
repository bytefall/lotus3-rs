#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn, static_mut_refs)]
#![allow(dead_code)]
#![feature(alloc_error_handler, abi_x86_interrupt)]

use core::{arch::asm, panic::PanicInfo};

extern crate alloc;

use crate::{
    archive::open_and_read_data_file,
    config::CFG,
    data::{SCREEN_HEIGHT, SCREEN_WIDTH, VERSION_STR, WORD_63BC, WORD_5160, WORD_5162, WORD_5164},
    dos::{exit, get_data_seg, printf, set_data_seg, sub_db28},
    intro::show_intro,
    mem::GLOBAL_ALLOCATOR,
    menu::main_menu,
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
mod timer;
mod video;

#[unsafe(link_section = ".startup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> i32 {
    asm!("mov ax, cs", "mov ds, ax", "mov es, ax");

    WORD_5160 = 0;
    WORD_5164 = 0;
    WORD_5162 = get_data_seg();

    GLOBAL_ALLOCATOR.init();
    WORD_63BC.resize(SCREEN_WIDTH * SCREEN_HEIGHT, 0); // +VGA_DBL_BUF_START

    set_video_mode_and_timer();
    open_and_read_data_file();
    sub_d37b();
    sub_db28();
    // protection_screen();
    // mov     word [word_1F26], loc_51C4 // exit fn
    // prepare_task_context(&loc_5371);
    // play_music(3);
    // sub_20ed();
    show_intro();
    // main_menu();

    sub_d396(); // temporary here
    printf(VERSION_STR);
    exit(0)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        sub_d396();
        println!("{}", info);
        exit(1)
    }
}
