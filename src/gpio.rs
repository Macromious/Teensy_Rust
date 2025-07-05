// this file will outline our interactions with configured gpio pins

use core::ptr;

// again we define an enum to access each port individually
pub enum _PortName {
    C
}

// first we need to define our register block, this is done as we did 
// in the port struct declaration
#[repr(C)]
pub struct GPIO {
    pdor: u32,
    psor: u32,
    pcor: u32,
    ptor: u32,
    pdir: u32,
    pddr: u32,
}

/*
// Now we add our implementations to instantiate GPIO use, and control 
// the input/output of the gpio pins
*/
impl GPIO {
    // define the new function to instantiate a gpio variable using hardware
    // register addresses. if this fails, the return value defaults to port A
    pub fn new(port_id: _PortName) -> &'static mut GPIO {
        match port_id {
            _PortName::C => unsafe {&mut *(0x400F_F080 as *mut GPIO)},
        }
    }

    // next we can implement a function to set the pin to either input or output
    // this needs to set the register bit to 1 for output, and 0 for input
    pub fn set_dir(&mut self, _pin_id: usize, _dir: usize) -> () {
        // first we read the register
        unsafe {
            let value = ptr::read_volatile(&((*self).pddr));
            let mask = 1 << 5;
            ptr::write_volatile(&mut (*self).pddr, value | mask);
        };
    }

    // last thing to implement for now is setting the output value high or low
    pub fn set_output(&mut self, _pin_id: usize, _out_val: bool) -> () {
        // this will be very similar to the set_dir function
        // first we read the register
        unsafe {
            let value = ptr::read_volatile(&((*self).pdor));
            let mask = 1 << 5;
            ptr::write_volatile(&mut (*self).pdor, value | mask);
        };
    }
}