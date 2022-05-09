#![feature(used_with_arg)]
#![feature(core_ffi_c)]
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use crate::memory::physical::PhysicalMemoryManager;
use core::ffi::c_char;
use numtoa::NumToA;
use stivale_boot::v2::{
    StivaleHeader, StivaleMemoryMapEntryType, StivaleStruct, StivaleTerminalHeaderTag,
};

mod memory;
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
    term_write("LylyOS v0.1.0\n".as_ptr() as *const c_char, 14);
    term_write("Hello, world!\n".as_ptr() as *const c_char, 14);

    let mmap = stivale_struct
        .memory_map()
        .expect("Failed to get memory map");
    let mut memory_len = 0;
    let mut used_len = 0;
    let mut base = None;
    for entry in mmap.as_slice() {
        memory_len += entry.length;
        if entry.entry_type != StivaleMemoryMapEntryType::Usable {
            used_len += entry.length;
        }
    }

    let pmm_size = memory_len / 4096 / 8;
    for entry in mmap.as_slice() {
        if base.is_none()
            && entry.entry_type == StivaleMemoryMapEntryType::Usable
            && entry.length >= pmm_size
        {
            base = Some(entry.base);
        }
    }
    let pmm = unsafe {
        PhysicalMemoryManager::new(
            unsafe { core::mem::transmute(base.expect("Failed to find usable memory")) },
            pmm_size as usize * 8,
            used_len as usize / 4096,
        )
    };

    let used = pmm.used();
    let mut buffer = [0; 20];
    let used_str = used.numtoa_str(10, &mut buffer);
    term_write("Used: ".as_ptr() as *const c_char, 6);
    term_write(used_str.as_ptr() as *const c_char, used_str.len());
    term_write("/".as_ptr() as *const c_char, 1);
    let total_str = pmm.size().numtoa_str(10, &mut buffer);
    term_write(total_str.as_ptr() as *const c_char, total_str.len());
    term_write(" pages (".as_ptr() as *const c_char, 8);
    let percentage = used * 10000 / pmm.size();
    let percentage_str = (percentage / 100).numtoa_str(10, &mut buffer);
    term_write(
        percentage_str.as_ptr() as *const c_char,
        percentage_str.len(),
    );
    term_write(".".as_ptr() as *const c_char, 1);
    let percentage_str = (percentage % 100).numtoa_str(10, &mut buffer);
    term_write(
        percentage_str.as_ptr() as *const c_char,
        percentage_str.len(),
    );
    term_write("%)\n".as_ptr() as *const c_char, 3);

    loop {}
}
