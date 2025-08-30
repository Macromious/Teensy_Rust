// for now we will just use this to disable the watchdog
use core::ptr;

#[derive(Copy, Clone)]
pub struct SafeWatchdog {
    watchdog: *mut Watchdog,
}
#[repr(C)]
struct Watchdog {
    stctrlh: u16,
    stctrll: u16,
    tovalh: u16,
    tovall: u16,
    winh: u16,
    winl: u16,
    refresh: u16,
    unlock: u16,
    tmrouth: u16,
    tmroutl: u16,
    rstcnt: u16,
    presc: u16,
}

impl Watchdog {
    // we create the initialisation function first
    const fn new() -> &'static mut Watchdog {
        return unsafe {&mut *(0x4005_2000 as *mut Watchdog)};
    }

    // we need a function to execute the watchdog unlock sequence so we can update the settings
    fn unlock_watchdog(&mut self) {
        unsafe {
            // first write 0xC520 to the unlock register
            ptr::write_volatile(&mut (*self).unlock, 0xC520);
            // then write 0xD928 to the register
            ptr::write_volatile(&mut (*self).unlock, 0xD928);
        }
    } 

    // next we can enable or disable the watchdog
    fn enable_watchdog(&mut self, enable: bool) {
        unsafe {
            // first we need to read the current register value
            let mut reg_val: u16 = ptr::read_volatile(&(*self).stctrlh);

            // next we check if we are enabling or disabling the watchdog and updating accordingly
            if enable {
                reg_val |= 0b1;
            } else {
                reg_val &= !0b1;
            }
            // unlock the watchdog right before updating the register
            self.unlock_watchdog();
            // then we can write the value back to the register
            ptr::write_volatile(&mut (*self).stctrlh, reg_val);
        }
    }
}

// copy the functions to the wrapper
impl SafeWatchdog {
    pub const fn take() -> SafeWatchdog {
        SafeWatchdog { watchdog: Watchdog::new() }
    }

    pub fn enable_watchdog(self, enable: bool) {
        unsafe {
            (*self.watchdog).enable_watchdog(enable);
        }
    }
}