// this file will outline our interactions with configured gpio pins

use volatile_register::{RO, RW, WO};

// first we need to define our register block, this is done as we did 
// in the port struct declaration
#[repr(C)]
pub struct GPIO {
    pdor: RW<u32>,
    psor: WO<u32>,
    pcor: WO<u32>,
    ptor: WO<u32>,
    pdir: RO<u32>,
    pddr: RW<u32>,
}

/*
// Now we add our implementations to instantiate GPIO use, and control 
// the input/output of the gpio pins
*/
impl GPIO {
    // define the new function to instantiate a gpio variable using hardware
    // register addresses. if this fails, the return value defaults to port A
    pub fn new(port_id: char) -> *mut GPIO {
        match port_id {
            'A' => return 0x400F_F000 as *mut GPIO,
            'B' => return 0x400F_F040 as *mut GPIO,
            'C' => return 0x400F_F080 as *mut GPIO,
            'D' => return 0x400F_F0C0 as *mut GPIO,
            'E' => return 0x400F_F100 as *mut GPIO,
            _ => return 0x400F_F000 as *mut GPIO,
        }
    }

    // next we can implement a function to set the pin to either input or output
    // this needs to set the register bit to 1 for output, and 0 for input
    pub fn set_dir(&self, pin_id: usize, dir: usize) -> () {
        // first we read the register
        let mut reg_val: u32 = self.pddr.read();
        // use an if staement to complete this
        if dir == 0 {
            // if we are setting input, we can use AND and a bitshift
            // to the correct position
            reg_val &= !0b1 << pin_id;
        } else {
            // if not input, then output is expected, so we can OR the dir value
            // for the correct pin
            reg_val |= 0b1 << pin_id;
        }
        
        // finally, we can write our new register value back to the register
        unsafe {self.pddr.write(reg_val)};
    }

    // last thing to implement for now is setting the output value high or low
    pub fn set_output(&self, pin_id: usize, out_val: usize) -> () {
        // this will be very similar to the set_dir function
        // first we read the register
        let mut reg_val: u32 = self.pdor.read();
        // use an if staement to complete this
        if out_val == 0 {
            // if we are setting input, we can use AND and a bitshift
            // to the correct position
            reg_val &= !0b1 << pin_id;
        } else {
            // if not input, then output is expected, so we can OR the dir value
            // for the correct pin
            reg_val |= 0b1 << pin_id;
        }
        
        // finally, we can write our new register value back to the register
        unsafe {self.pdor.write(reg_val)};
    }
}