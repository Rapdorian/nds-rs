pub use bg::prelude::*;
pub use engine::*;
pub use engine_a::*;
pub use engine_b::DISPCNT as DISPCNT_B;
pub use engine_b::MASTER_BRIGHT as MASTER_BRIGHT_B;
pub use ipc::*;
pub use irq::*;
pub use math::*;
pub use mem::*;
pub use sys::*;
pub use vram::*;

pub mod sys {
    use crate::io;

    pub const POWCNT1: *mut u16 = io(0x304) as *mut u16;

    pub const POWER_LCD: u16 = (1 << 0);
    pub const POWER_2D_A: u16 = ( 1 << 1);
    pub const POWER_MATRIX: u16 = ( 1 << 2);
    pub const POWER_3D_CORE: u16 = ( 1 << 3);
    pub const POWER_2D_B: u16 = (1 << 9);
    pub const POWER_SWAP_LCDS: u16 = (1 << 15);
    pub const POWER_ALL_2D: u16 = POWER_LCD | POWER_2D_A | POWER_2D_B;
    pub const POWER_ALL: u16 = POWER_ALL_2D | POWER_3D_CORE | POWER_MATRIX;
}

pub mod bg {

    pub mod prelude {
        use crate::io;

        pub const BGCTRL: *mut [u16; 4] = io(0x08) as *mut [u16; 4];
        pub const BG0CNT: *mut u16 = io(0x08) as *mut u16;
        pub const BG1CNT: *mut u16 = io(0x0A) as *mut u16;
        pub const BG2CNT: *mut u16 = io(0x0C) as *mut u16;
        pub const BG3CNT: *mut u16 = io(0x0E) as *mut u16;

        pub const BGCTRL_SUB: *mut [u16; 4] = io(0x1008) as *mut [u16; 4];
        pub const BG0CNT_SUB: *mut u16 = io(0x1008) as *mut u16;
        pub const BG1CNT_SUB: *mut u16 = io(0x100A) as *mut u16;
        pub const BG2CNT_SUB: *mut u16 = io(0x100C) as *mut u16;
        pub const BG3CNT_SUB: *mut u16 = io(0x100E) as *mut u16;

        pub const BG_32_32: u16 = 0 << 14;
        pub const BG_64_32: u16 = 1 << 14;
        pub const BG_32_64: u16 = 2 << 14;
        pub const BG_64_64: u16 = 3 << 14;

        /* TODO: Many more modes */

        pub const BG_COLOR_256: u16 = 0x80;
        pub const BG_COLOR_16: u16 = 0x00;

        pub const BG_PALETTE: *mut u16 = 0x0500_0000 as *mut u16;
        pub const BG_PALETTE_SUB: *mut u16 = 0x0500_0400 as *mut u16;

        /* Scrolling registers */
        pub const BG0HOFS: *mut u16 = io(0x10) as *mut u16;
        pub const BG0VOFS: *mut u16 = io(0x12) as *mut u16;
        pub const BG1HOFS: *mut u16 = io(0x14) as *mut u16;
        pub const BG1VOFS: *mut u16 = io(0x16) as *mut u16;
        pub const BG2HOFS: *mut u16 = io(0x18) as *mut u16;
        pub const BG2VOFS: *mut u16 = io(0x1A) as *mut u16;

        pub const BG0HOFS_SUB: *mut u16 = io(0x1010) as *mut u16;
        pub const BG0VOFS_SUB: *mut u16 = io(0x1012) as *mut u16;
        pub const BG1HOFS_SUB: *mut u16 = io(0x1014) as *mut u16;
        pub const BG1VOFS_SUB: *mut u16 = io(0x1016) as *mut u16;
        pub const BG2HOFS_SUB: *mut u16 = io(0x1018) as *mut u16;
        pub const BG2VOFS_SUB: *mut u16 = io(0x101A) as *mut u16;

    }

    pub const fn map_base(base: u16) -> u16 {
        base << 8
    }

    pub const fn tile_base(base: u16) -> u16 {
        base << 2
    }

    pub const fn bmp_base(base: u16) -> u16 {
        map_base(base)
    }

    pub const fn map_ram_sub(base: usize) -> *mut u16 {
        ((base*0x800) + 0x0620_0000) as *mut u16
    }

    pub const fn tile_ram_sub(base: usize) -> *mut u16 {
        ((base * 0x4000) + 0x0620_0000) as *mut u16
    }

    pub const fn map_ram(base: usize) -> *mut u16 {
        ((base * 0x800) + 0x0600_0000) as *mut u16
    }

    pub const fn tile_ram(base: usize) -> *mut u16 {
        ((base * 0x4000) + 0x0600_0000) as *mut u16
    }
}

pub mod engine {
    pub const MODE_0_2D: u32 = 0x1_0000;
    pub const MODE_1_2D: u32 = 0x1_0001;
    pub const MODE_2_2D: u32 = 0x1_0002;
    pub const MODE_3_2D: u32 = 0x1_0003;
    pub const MODE_4_2D: u32 = 0x1_0004;
    pub const MODE_5_2D: u32 = 0x1_0005;
    pub const MODE_6_2D: u32 = 0x1_0006;

    pub const MODE_FB0: u32 = 0x2_0000;
    pub const MODE_FB1: u32 = 0x6_0000;
    pub const MODE_FB2: u32 = 0xA_0000;
    pub const MODE_FB3: u32 = 0xE_0000;

    pub const DISPLAY_BG0_ACTIVE: u32 = 1 << 8;
    pub const DISPLAY_BG1_ACTIVE: u32 = 1 << 9;
    pub const DISPLAY_BG2_ACTIVE: u32 = 1 << 10;
    pub const DISPLAY_BG3_ACTIVE: u32 = 1 << 11;
    pub const DISPLAY_SPR_ACTIVE: u32 = 1 << 12;
    pub const DISPLAY_WIN0_ON: u32 = 1 << 13;
    pub const DISPLAY_WIN1_ON: u32 = 1 << 14;
    pub const DISPLAY_SPR_WIN_ON: u32 = 1 << 15;
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

    pub const fn vram_offset(n: u8) -> u8 {
        n << 3
    }

    pub const VRAM_A: *mut [[u16; 256]; 192] = vram(0) as *mut [[u16; 256]; 192];
    pub const VRAM_B: *mut [[u16; 256]; 192] = vram(0x20_000) as *mut [[u16; 256]; 192];
    pub const VRAM_C: *mut [[u16; 256]; 192] = vram(0x40_000) as *mut [[u16; 256]; 192];
    pub const VRAM_D: *mut [[u16; 256]; 192] = vram(0x60_000) as *mut [[u16; 256]; 192];
    pub const VRAM_E: *mut [[u16; 256]; 192] = vram(0x80_000) as *mut [[u16; 256]; 192];
    pub const VRAM_F: *mut [[u16; 256]; 192] = vram(0x90_000) as *mut [[u16; 256]; 192];

    pub const VRAM_ENABLE: u8 = 1 << 7;

    pub const VRAM_A_LCD: u8 = 0;
    pub const VRAM_A_MAIN_BG: u8 = 1;
    pub const VRAM_A_MAIN_SPRITE: u8 = 2;
    pub const VRAM_A_TEXTURE: u8 = 3;

    pub const VRAM_C_SUB_BG: u8 = 4;

    /* TODO: Lots more modes */
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
