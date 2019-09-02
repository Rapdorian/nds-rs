use crate::interrupt::Interrupt;

/// wait for vblank
///
/// Continues to wait in Halt status until a new V-Blank interrupt occurs.
///
/// Equivalent to `wait(Interrupt::VBlank)`
pub fn wait_vblank() {
    unsafe {
        asm!("SWI 0x05");
    }
}
/// wait for an interrupt
///
/// Continues to wait in Halt state until one (or more) of the 
/// specified interrupt(s) occur. The function forcefully sets 
/// IME=1. When using multiple interrupts at the same time, this function
/// has less overhead than repeatedly calling the Halt function.
pub fn wait(int: Interrupt) {
    unsafe {
        asm!("ldr r0, =1
              swi 0x04
              " 
              : 
              : "{r1}"(int as u32) 
              : "r0"
        );
    }
}

/// Halts the CPU until an interrupt request occurs.
///
/// The CPU is switched into low-power mode, all
/// other circuits are kept operating.
///
/// Halt mode is terminated when any enabled interrupts are requested.
pub fn halt() {
    unsafe {
        asm!("SWI 0x06");
    }
}

/// Switches to a very low power mode.
///
/// The CPU, System Clock, Sound, Video, SIO-Shift Clock, DMAs,
/// and Timers are stopped.
///
/// Stop state is terminated by the following interrupts
/// Joypad, Game Pak, or General-Purpose-SIO.
pub fn stop() {
    unsafe {
        asm!("SWI 0x03");
    }
}

/// bios division routine
///
/// This is a software division routine,
/// the NDS9 additionally supports hardware division.
pub fn div(num: i32, denom: i32) -> (i32, i32, u32) {
    let quotient;
    let remainder;
    let abs_quotient;
    unsafe{
        asm!("
             swi 0x09"
             : "={r0}"(quotient) "={r1}"(remainder) "={r3}"(abs_quotient)
             :"{r0}"(num) "{r1}"(denom)
        );
    }
    return (quotient, remainder, abs_quotient);
}

/// bios sqrt routine
///
/// This is a software sqrt routine,
/// the NDS9 additionally supports a hardware sqrt.  
pub fn sqrt(num: u32) -> u16 { let v;
    unsafe {
        asm!("swi 0x0D"
             : "={r0}"(v)
             : "{r0}"(num)
        );
    }
    return v;
}

/// bios atan routine
///
/// unlike sqrt and division the NDS doesn't have
/// a hardware atan
pub fn atan(_num: u16) -> u16{
    unimplemented!();
}

/// bios atan2 routine
///
/// unlike sqrt and division the NDS doesn't have
/// a hardware atan
pub fn atan2(_x: u16, _y: u16) -> u16 {
    unimplemented!();
}

pub fn reset(){
    unsafe {
        asm!("SWI 0x00");
    }
}


/// Resets the I/O registers and RAM specified in ResetFlags. 
///
/// | Bit |   Expl.                                                      |
/// |-----|--------------------------------------------------------------|
/// | 0   | Clear 256K on-board WRAM  ;-don't use when returning to WRAM |
/// | 1   | Clear 32K in-chip WRAM    ;-excluding last 200h bytes        |
/// | 2   | Clear Palette                                                |
/// | 3   | Clear VRAM                                                   |
/// | 4   | Clear OAM              ;-zerofilled! does NOT disable OBJs!  |
/// | 5   | Reset SIO registers    ;-switches to general purpose mode!   |
/// | 6   | Reset Sound registers                                        |
/// | 7   | Reset all other registers (except SIO, Sound)                |
pub fn hw_reset(flags: u8){
    unsafe {
        asm!("SWI 0x01"
             :
             :"{r0}"(flags)
        );
    }
}

pub fn reboot(){
    unsafe {
        asm!("SWI 0x26");
    }
}
