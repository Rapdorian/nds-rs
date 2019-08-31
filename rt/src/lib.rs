#![no_std]

#[cfg(panic)]
pub use nds_panic::panic;

#[no_mangle]
#[link_section = ".text.startup"]
pub unsafe extern "C" fn _start() -> ! {
    extern "Rust" {
        fn main();
    }

    main();
    // shutdown sequence
    loop{}
}
