#![no_std]
#![feature(panic_info_message, lang_items, asm)]

use core::fmt::Write;
use core::ptr::write_volatile;
use font8x8::{UnicodeFonts, BASIC_FONTS};
use nds_registers::arm9::*;

fn wait_vblank() {
    unsafe{
        asm!("SWI 0x05");
    }
}

fn draw_glyph(c: char, x: usize, y: usize) {
    let x = x * 8;
    let y = y * 8;
    if let Some(g) = BASIC_FONTS.get(c) {
        // draw glyph
        let mut line = 0;
        for row in &g {
            for bit in 0..8 {
                if *row & 1 << bit != 0 {
                    unsafe {
                        write_volatile((&mut ((*VRAM_A)[y + line][x + bit])) as *mut u16, 0x7FFF);
                    }
                }
            }
            line += 1;
        }
    }
}

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        static mut X: usize = 0;
        static mut Y: usize = 0;
        for c in s.chars(){
            if c == '\n' || (unsafe{X} > 20 && c.is_whitespace() || unsafe{X} >= 32){
                unsafe {
                    X = 0;
                    Y += 1;
                }
            }else {
                unsafe{
                    draw_glyph(c, X, Y);
                    X += 1;
                }
            }
        }
        Ok(())
    }
}

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(panic: &PanicInfo<'_>) -> ! {
    wait_vblank();
    unsafe {
        write_volatile(POWCNT1, 0x3);
        write_volatile(DISPCNT, 0x20000);
        write_volatile(VRAMCNT_A, 0x80);
    }

    writeln!(Console, "{}", panic).unwrap();
    wait_vblank();
    loop {}
}
