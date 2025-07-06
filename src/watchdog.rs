// for now we will just use this to disable the watchdog

use volatile_register::RW;

#[repr(C, packed(2))]
pub struct Watchdog {
    stctrlh: RW<u16>,
    stctrll: RW<u16>,
    tovalh: RW<u16>,
    tovall: RW<u16>,
    winh: RW<u16>,
    winl: RW<u16>,
    refresh: RW<u16>,
    unlock: RW<u16>,
    tmrouth: RW<u16>,
    tmroutl: RW<u16>,
    rstcnt: RW<u16>,
    presc: RW<u16>,
}

impl Watchdog {
    pub fn new() -> &'static mut Watchdog {
        return unsafe {&mut *(0x4005_2000 as *mut Watchdog)};
    }
}