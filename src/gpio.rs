// this file will outline our interactions with configured gpio pins

use core::ptr;

// again we define an enum to access each port individually
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Copy, Clone)]
pub struct SafeGPIO {
    port_a: *mut GPIO,
    port_b: *mut GPIO,
    port_c: *mut GPIO,
    port_d: *mut GPIO,
    port_e: *mut GPIO,
}

// first we need to define our register block, this is done as we did 
// in the port struct declaration
#[repr(C)]
struct GPIO {
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
    pub const fn new(port_id: PortName) -> &'static mut GPIO {
        match port_id {
            PortName::A => unsafe {&mut *(0x400F_F000 as *mut GPIO)},
            PortName::B => unsafe {&mut *(0x400F_F040 as *mut GPIO)},
            PortName::C => unsafe {&mut *(0x400F_F080 as *mut GPIO)},
            PortName::D => unsafe {&mut *(0x400F_F0C0 as *mut GPIO)},
            PortName::E => unsafe {&mut *(0x400F_F100 as *mut GPIO)},
        }
    }

    // next we can implement a function to set the pin to either input or output
    // this needs to set the register bit to 1 for output, and 0 for input
    pub fn set_dir(&mut self, pin_id: usize, _dir: bool) -> () {
        // first we read the register
        unsafe {
            let value = ptr::read_volatile(&((*self).pddr));
            let mask = 1 << pin_id;
            ptr::write_volatile(&mut (*self).pddr, value | mask);
        };
    }

    // last thing to implement for now is setting the output value high or low
    pub fn set_output(&mut self, pin_id: usize, out_val: bool) -> () {
        // this will be very similar to the set_dir function
        // first we read the register
        unsafe {
            // first read the register
            let mut reg_val: u32 = ptr::read_volatile(&((*self).pdor));
            // next check the output val to set
            if out_val {
                // if setting to high, update value to 1
                reg_val |= 0b1 << pin_id;
            } else {
                // otherwwise write low
                reg_val &= !(0b1 << pin_id);
            }
            // finally write the value back to the register
            ptr::write_volatile(&mut (*self).pdor, reg_val);
        };
    }

    // thjis function can toggle the output of the gpio pin to the inverse of the
    // current output value
    pub fn toggle_output(&mut self, pin_id: usize) -> () {
        // use a straight write to toggle only the desired pins, as ptor only 
        // toggles pins which the bit is set to 1
        let val: u32 = 1 << pin_id;
        unsafe {
            ptr::write_volatile(&mut (*self).ptor, val);
        }
    }
}

impl SafeGPIO {
    // creates our safe wrapper for our GPIO
    pub const fn take() -> SafeGPIO {
        // create and return the SafeGPIO struct
        SafeGPIO {
            port_a: GPIO::new(PortName::A),
            port_b: GPIO::new(PortName::B),
            port_c: GPIO::new(PortName::C),
            port_d: GPIO::new(PortName::D),
            port_e: GPIO::new(PortName::E)
        }
    }

    // use this as the wrapper to set whether the gpio is input or output
    pub fn set_dir(self, port_id: PortName, pin: usize, dir: bool) {
        unsafe {
            // the match function is used to set the direction on the correct pin
            match port_id {
                PortName::A => (*self.port_a).set_dir(pin, dir),
                PortName::B => (*self.port_b).set_dir(pin, dir),
                PortName::C => (*self.port_c).set_dir(pin, dir),
                PortName::D => (*self.port_d).set_dir(pin, dir),
                PortName::E => (*self.port_e).set_dir(pin, dir),
            }
        }
    }

    pub fn set_output(self, port_id: PortName, pin: usize, out_val: bool) {
        unsafe {
            // the match function is used to set the direction on the correct pin
            match port_id {
                PortName::A => (*self.port_a).set_output(pin, out_val),
                PortName::B => (*self.port_b).set_output(pin, out_val),
                PortName::C => (*self.port_c).set_output(pin, out_val),
                PortName::D => (*self.port_d).set_output(pin, out_val),
                PortName::E => (*self.port_e).set_output(pin, out_val),
            }
        }
    }

    pub fn toggle_output(self, port_id: PortName, pin: usize) {
        unsafe {
            // the match function is used to set the direction on the correct pin
            match port_id {
                PortName::A => (*self.port_a).toggle_output(pin),
                PortName::B => (*self.port_b).toggle_output(pin),
                PortName::C => (*self.port_c).toggle_output(pin),
                PortName::D => (*self.port_d).toggle_output(pin),
                PortName::E => (*self.port_e).toggle_output(pin),
            }
        }
    }
}