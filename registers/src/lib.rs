#![no_std]

const fn io(reg: usize) -> usize {
    reg + 0x4000000
}

pub mod arm9;
