#![feature(used_with_arg)]
#![feature(core_ffi_c)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use core::ffi::c_char;
use stivale_boot::v2::{StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag};

mod port;

static STACK: [u8; 4096] = [0; 4096];

#[used(linker)]
#[link_section = ".stivale2hdr"]
static HEADER: StivaleHeader = StivaleHeader::new()
    .entry_point(stivale2_main)
    .stack(STACK.as_ptr())
    .tags(&CONSOLE_TAG as *const StivaleTerminalHeaderTag as *const ());
static CONSOLE_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();

extern "C" fn stivale2_main(stivale_struct: &StivaleStruct) -> ! {
    let terminal = stivale_struct
        .terminal()
        .expect("Failed to get command line");
    let term_write: fn(string: *const c_char, length: usize) =
        unsafe { core::mem::transmute(terminal.term_write_addr) };
    (term_write)("LylyOS v0.1.0\n".as_ptr() as *const c_char, 14);
    (term_write)("Hello, world!\n".as_ptr() as *const c_char, 14);
    loop {}
}
