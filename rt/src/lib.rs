#![no_std]
#![feature(asm)]

#[cfg(panic)]
pub use nds_panic::panic;

use core::ptr::write_volatile;
use nds_registers::arm9::*;

pub use nds_registers as registers;

pub mod bios;
pub mod interrupt;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main();
    }

    main();
    // shutdown sequence
    write_volatile(POWCNT1, 0);
    loop {}
}
