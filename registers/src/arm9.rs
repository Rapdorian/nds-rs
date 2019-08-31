pub use sys::*;
pub use ipc::*;
pub use irq::*;
pub use vram::*;
pub use mem::*;
pub use math::*;
pub use engine_a::*;
pub use engine_b::DISPCNT as DISPCNT_B;
pub use engine_b::MASTER_BRIGHT as MASTER_BRIGHT_B;

pub mod sys {
    use crate::io;

    pub const POWCNT1: *mut u16 = io(0x304) as *mut u16;
}

pub mod engine_a {
    use crate::io;

    pub const DISPCNT: *mut u32 = io(0x0) as *mut u32;
    pub const DISPSTAT: *mut u16 = io(0x4) as *mut u16;
    pub const VCOUNT: *mut u16 = io(0x6) as *mut u16;
    /* GBA 0x50 long */
    pub const DISP3DCNT: *mut u16 = io(0x60) as *mut u16;
    pub const DISPCAPCNT: *mut u32 = io(0x64) as *mut u32;
    pub const DISP_MMEM_FIFO: *mut u32 = io(0x68) as *mut u32;
    pub const MASTER_BRIGHT: *mut u16 = io(0x6C) as *mut u16;
}

pub mod engine_b {
    use crate::io;

    pub const DISPCNT: *mut u32 = io(0x1000) as *mut u32;
    pub const MASTER_BRIGHT: *mut u16 = io(0x106C) as *mut u16;
}

pub mod ipc {
    use crate::io;

    pub const IPCSYNC: *mut u16 = io(0x180) as *mut u16;
    pub const IPCFIFOCNT: *mut u16 = io(0x184) as *mut u16;
    pub const IPCFIFOSEND: *mut u32 = io(0x188) as *mut u32;
    pub const IPCFIFORECT: *mut u32 = io(0x100000) as *mut u32;
}

pub mod irq {
    use crate::io;

    pub const IME: *mut u16 = io(0x208) as *mut u16;
    pub const IE: *mut u32 = io(0x210) as *mut u32;
    pub const IF: *mut u32 = io(0x214) as *mut u32;
}

pub mod vram {
    use crate::io;

    pub const VRAMCNT_A: *mut u8 = io(0x240) as *mut u8;
    pub const VRAMCNT_B: *mut u8 = io(0x241) as *mut u8;
    pub const VRAMCNT_C: *mut u8 = io(0x242) as *mut u8;
    pub const VRAMCNT_D: *mut u8 = io(0x243) as *mut u8;
    pub const VRAMCNT_E: *mut u8 = io(0x244) as *mut u8;
    pub const VRAMCNT_F: *mut u8 = io(0x245) as *mut u8;
    pub const VRAMCNT_G: *mut u8 = io(0x246) as *mut u8;
    pub const VRAMCNT_H: *mut u8 = io(0x248) as *mut u8;
    pub const VRAMCNT_I: *mut u8 = io(0x249) as *mut u8;

    const fn vram(ptr: usize) -> usize {
        0x6800000 + ptr
    }

    pub const VRAM_A: *mut [[u16;256]; 192] = vram(0) as *mut [[u16;256]; 192];
    pub const VRAM_B: *mut [[u16;256]; 192] = vram(0x20_000) as *mut [[u16;256]; 192];
    pub const VRAM_C: *mut [[u16;256]; 192] = vram(0x40_000) as *mut [[u16;256]; 192];
    pub const VRAM_D: *mut [[u16;256]; 192] = vram(0x60_000) as *mut [[u16;256]; 192];
    pub const VRAM_E: *mut [[u16;256]; 192] = vram(0x80_000) as *mut [[u16;256]; 192];
    pub const VRAM_F: *mut [[u16;256]; 192] = vram(0x90_000) as *mut [[u16;256]; 192];
}

pub mod mem {
    use crate::io;

    pub const EXMEMCNT: *mut u16 = io(0x204) as *mut u16;
    pub const WRAMCNT: *mut u8 = io(0x247) as *mut u8;
}

pub mod math {
    use crate::io;

    pub const DIVCNT: *mut u16 = io(0x280) as *mut u16;
    pub const DIV_NUMER: *mut i64 = io(0x290) as *mut i64;
    pub const DIV_DENOM: *mut i64 = io(0x298) as *mut i64;
    pub const DIV_RESULT: *mut i64 = io(0x2A0) as *mut i64;
    pub const DIVREM_RESULT: *mut i64 = io(0x2A8) as *mut i64;
    pub const SQRTCNT: *mut u16 = io(0x2B0) as *mut u16;
    pub const SQRT_RESULT: *mut u32 = io(0x2B4) as *mut u32;
    pub const SQRT_PARAM: *mut i64 = io(0x2B8) as *mut i64;
}
