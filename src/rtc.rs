// the rtc file is where we can define the controls for the RTC
use core::{arch::asm, ptr};

#[derive(Copy, Clone)]
// define the safe wrapper for the rtc
pub struct SafeRTC {
    rtc: *mut RTC,
}

// define the rtc struct
#[repr(C)]
struct RTC {
    tsr: u32,
    tpr: u32,
    tar: u32,
    tcr: u32,
    cr: u32,
    sr: u32,
    lr: u32,
    ier: u32,
    ttsr: u32,
    mer: u32,
    mclr: u32,
    mchr: u32,
    _reserved: [u32; 500],
    war: u32,
    rar: u32,
}

// here we will implement the initialisation and control of the RTC registers
impl RTC {
    // pass back the memory address and structure of the RTC
    const fn new() -> &'static mut RTC {
        return unsafe {&mut *(0x4003_D000 as *mut RTC)}
    }

    // a function to unlock the status register is needed. this function is not public as we do not
    // currently want to allow the register to be unlocked at any time
    fn unlock_sr(&mut self) {
        unsafe {
            // first we read the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).lr);
            // now update byte 6 to 1
            reg_val |= 0b1 << 5;
            // finally write bac kto the register
            ptr::write_volatile(&mut (*self).lr, reg_val);
        }
    }

    // we also need to be able to lock it again after
    fn _lock_sr(&mut self) {
        unsafe {
            // first we read the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).lr);
            // now update byte 6 to 0
            reg_val &= !(0b1 << 5);
            // finally write bac kto the register
            ptr::write_volatile(&mut (*self).lr, reg_val);
        }
    }

    // we need a function to disable the supervisor protection
    fn _disable_supervisor(&mut self) {
        unsafe {
            // first we read the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).cr);
            // now update byte 6 to 0
            reg_val &= 0b1 << 2;
            // finally write bac kto the register
            ptr::write_volatile(&mut (*self).cr, reg_val);
        }
    }

    // next we need a function to enable the oscillator
    fn enable_osc(&mut self) {
        // start by writing the counter to zero
        self.clear_counter();

        unsafe {
            // first we need to read the current value
            let mut reg_val: u32 = ptr::read_volatile(&(*self).cr);
            // next we update the value of OCSE
            reg_val |= 0b1 << 8;
            // finally write the value back to the register
            ptr::write_volatile(&mut (*self).cr, reg_val);

            // create a delay to allow the oscillator to stabilise
            for _n in 1..(10e6 as u32) {
                asm!("nop");
            }
        }
    }

    fn clear_counter(&mut self) {
        unsafe {
            ptr::write_volatile(&mut (*self).tsr, 0x1);

            // create a small delay for the change to take place
            for _n in 1..(10e3 as u32) {
                asm!("nop");
            }
        }
    }

    // this function enables the counter for the rtc. this basically enables the 'seconds hand' of 
    // the rtc
    fn enable_counter(&mut self, enable: bool) {

        unsafe {
            // read the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).sr);
            if enable {
                // next update the register data
                reg_val |= 0b1 << 4;
            } else {
                // next update the register data
                reg_val &= !(0b1 << 4);
            }
            // finally write the new value back to the register
            ptr::write_volatile(&mut (*self).sr, reg_val);
        }
    }

    fn read_reg(&mut self) -> u32 {
        unsafe {
            // just read the reguster and return the value
            return ptr::read_volatile(&(*self).ier);
        }
    }

    // in this function we use all the previous functions to make execution simple when enabling 
    // the rtc. this also makes writing the wrapper functions simpler
    fn enable_rtc(&mut self) {
        // first we need to enable the oscillator. Hopefully this will alow it time to start up 
        // before we enable the counter
        //self.enable_osc();
        // next we need to enable the counter.
        //self.unlock_sr();
        self.enable_counter(true);
        //self.lock_sr();
    }

    // the final function for now is a function to enable the interrupt
    fn enable_seconds_interrupt(&mut self) {
        unsafe {
            // first we read the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).ier);
            // next we update the TSIE register bit
            reg_val |= 0b1 << 4;
            // now write the result back to the register
            ptr::write_volatile(&mut (*self).ier, reg_val);
        }
    }

    fn check_valid(&mut self) -> bool {
        unsafe {
            // read the status register
            let reg_val: u32 = ptr::read_volatile( &(*self).sr);
            // extract the first bit and return the value
            return (reg_val & 0b1) != 0;
        }
    }

    fn check_time(&mut self) -> u32 {
        unsafe {
            // read the register
            return ptr::read_volatile(&(*self).tsr)
        }
    }
}

impl SafeRTC {
    // we first need our initialisation function
    pub const fn take() -> SafeRTC {
        SafeRTC { rtc: RTC::new() }
    }

    pub fn _disable_supervisor(self) {
        unsafe {
            (*self.rtc)._disable_supervisor();
        }
    }

    pub fn enable_osc(self) {
        unsafe {
            (*self.rtc).enable_osc();
        }
    }

    pub fn enable_counter(self, enable: bool) {
        unsafe {
            (*self.rtc).enable_counter(enable);
        }
    }

    // next is a function to enable the rtc
    pub fn enable_rtc(self) {
        unsafe {
            (*self.rtc).enable_rtc();
        }
    }

    pub fn read_reg(self) -> u32 {
        unsafe {
            return (*self.rtc).read_reg();
        }
    }

    pub fn enable_seconds_interrupt(self) {
        unsafe {
            (*self.rtc).enable_seconds_interrupt();
        }
    }

    pub fn check_valid(self) -> bool {
        unsafe {
            return (*self.rtc).check_valid();
        }
    }

    pub fn check_time (self) -> u32 {
        unsafe {
            return (*self.rtc).check_time();
        }
    }
}