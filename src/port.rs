use core::arch::asm;

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("outb dx, al", in("dx") port, in("al") value);
    }
}

pub fn inb(port: u16) -> u8 {
    let value;
    unsafe { asm!("inb al, dx", in("dx") port, out("al") value) }
    value
}
