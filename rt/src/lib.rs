#![no_std]
#![feature(asm)]

#[cfg(panic)]
pub use nds_panic::panic;

use core::ptr::read_volatile;
use nds_registers::arm9::*;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main();
    }

    main();
    // shutdown sequence
    loop {}
}

pub fn wait_vblank() {
    unsafe{
        asm!("mov r11, r11");
        asm!("SWI 0x05");
    }
    //unsafe { while read_volatile(VCOUNT) < 192 {} }
}

pub fn wait_vstart() {
    //unsafe { while read_volatile(VCOUNT) >= 192 {} }
}
