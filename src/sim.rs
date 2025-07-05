// this file holds all the system integration control, this won't be used 
// much now for anything other than enabling the ports.

use core::ptr;

pub enum PortName {
    C
}

// first we need to set up the struct to access the registers (this is a 
// long one)
#[repr(C)]
pub struct SIM {
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
    pub fn new() -> &'static mut SIM {
        return unsafe {&mut *(0x4004_7000 as *mut SIM)};
    }

    // next we have the function to control clock gating for the ports. this 
    // will be used to enable the ports
    pub fn cg5_control(&mut self, port: PortName) -> () {
        // shift_val will represent how many pits to shift across to enable 
        // the correct port
        let shift_val: usize;
        // check which port is requested and set the relevant bit to 1 to 
        // enable the clock. if the requested port is incorrect, default to
        // port A
        match port {
            PortName::C => shift_val = 11,
        }

        // now we can read and write this to update the value in the register
        unsafe {
            let mut reg_val: u32 = ptr::read_volatile(&(*self).scgc[4]);
            reg_val |= 1 << shift_val;
            ptr::write_volatile(&mut (*self).scgc[4],reg_val);
        }; 
    }
}