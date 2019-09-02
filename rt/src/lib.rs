#![no_std]
#![feature(asm)]

#[cfg(panic)]
pub use nds_panic::panic;

use core::ptr::{read_volatile, write_volatile};
use nds_registers::arm9::*;

pub mod bios;
pub mod interrupt;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main();
    }

    main();
    // shutdown sequence
    unsafe {
        write_volatile(POWCNT1, 0);
    }
    loop {}
}
