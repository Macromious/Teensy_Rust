// this file holds all the system integration control, this won't be used much now for anything 
// other than enabling the ports.
use core::ptr;

pub enum PortName {
    A,
    //B,
    C,
    //D,
    E
}

#[derive(Copy, Clone)]
pub struct SafeSIM {
    sim: *mut SIM,
}

// first we need to set up the struct to access the registers (this is a long one)
#[repr(C)]
struct SIM {
    sopt1: u32,
    sopt1cfg: u32,
    usbphyctl: u32,
    _pad1: [u32; 1022],
    sopt2: u32,
    _pad2: u32,
    sopt4: u32,
    sopt5: u32,
    _pad3: u32,
    sopt7: u32,
    sopt8: u32,
    sopt9: u32,
    sdid: u32,
    scgc: [u32; 7],
    clkdiv1: u32,
    clkdiv2: u32,
    fcfg1: u32,
    fcfg2: u32,
    uidh: u32,
    uidmh: u32,
    uidml: u32,
    uidl: u32,
    clkdiv3: u32,
    clkdiv4: u32,
}

/*
// Now we set up the implementation of functions to control the registers
*/
impl SIM {
    // first is a function to initialize the register block
    const fn new() -> &'static mut SIM {
        return unsafe {&mut *(0x4004_7000 as *mut SIM)};
    }

    // next we have the function to control clock gating for the ports. this will be used to enable
    // the ports
    fn cg5_control(&mut self, port: PortName) -> () {
        // shift_val will represent how many pits to shift across to enable the correct port
        let shift_val: usize;
        // check which port is requested and set the relevant bit to 1 to enable the clock. if the
        // requested port is incorrect, default to port A
        match port {
            PortName::A => shift_val = 9,
            //PortName::B => shift_val = 10,
            PortName::C => shift_val = 11,
            //PortName::D => shift_val = 12,
            PortName::E => shift_val = 13,
        }

        // now we can read and write this to update the value in the register
        unsafe {
            let mut reg_val: u32 = ptr::read_volatile(&(*self).scgc[4]);
            reg_val |= 1 << shift_val;
            ptr::write_volatile(&mut (*self).scgc[4],reg_val);
        }; 
    }

    // the next function we need is a way to enable/disable access to the RTC
    fn rtc_access(&mut self, status: bool) {
        unsafe {
            let mut reg_val = ptr::read_volatile(&(*self).scgc[5]);
            // use an if statement to check if we are enabling or disabling access to the rtc
            if status {
                // if we want access, we need to write 1 to the rtc
                reg_val |= 0b1 << 29;
            } else {
                // otherwise, never allow access
                reg_val &= !(0b1 << 29);
            }
            // finish by writing the new value back to the register
            ptr::write_volatile(&mut (*self).scgc[5], reg_val);
        }
    }

    // this function can enable the 1Hz clock output 
    fn enable_1hz_clkout(&mut self, enable: bool) {
        unsafe {
            // do a read of the register
            let mut reg_val: u32 = ptr::read_volatile(&(*self).sopt2);
            // check if it is enable or disable
            if enable {
                reg_val &= !(0b1 << 4);
            } else {
                reg_val |= 0b1 << 4;
            }

            // write the result back to the register
            ptr::write_volatile(&mut (*self).sopt2, reg_val);
        }
    }
}

impl SafeSIM {
    pub const fn take() -> SafeSIM {
        SafeSIM { sim: SIM::new() }
    }

    pub fn cg5_control(self, port: PortName) {
        unsafe {
            (*self.sim).cg5_control(port);
        }
    }

    pub fn rtc_access(self, status:bool) {
        unsafe {
            (*self.sim).rtc_access(status);
        }
    }

    pub fn enable_1hz_clkout(self, enable: bool) {
        unsafe {
            (*self.sim).enable_1hz_clkout(enable);
        }
    }
}