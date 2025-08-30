// this file will define our port configuration for use within our program. 
// this will allow gpio functions and expose digital outputs

// we will use the volatile_register crate to help us access the peripheral registers
use core::ptr;

// first we define an enum where we can place each port in it's own type
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Copy, Clone)]
// create a new struct for safe abstraction of our ports
pub struct SafePort {
    _port_a: *mut Port,
    _port_b: *mut Port,
    _port_c: *mut Port,
    _port_d: *mut Port,
    _port_e: *mut Port,
}

// first we need to define our port configuration registers. we use the repr(C)
// directive to tell the compiler that these data structures need to be stored 
// in the same style as the C compiler. in this case, the struct is public so we
// can use the struct in our program, however the values are private to prevent a
// user from accessing the registers directly. they must instead be accessed through the
// available functions
#[repr(C)]
struct Port {
    pcr: [u32; 32],
    gpclr: u32,
    gphlr: u32,
    _pad1: [u8; 24],
    isfr: u32,
    _pad2: [u32; 7],
    dfer: u32,
    dfcr: u32,
    dfwr: u32,
}

/*
// now we can add our functions to implement port control
*/
//
impl Port {
    // we will write a function to instantiate each port, this can be called so that
    // the user does not need to know the memory locations in order to use the functions
    const fn new(port_id: PortName) -> *mut Port {
        // we use a match statement to check which port needs to be created. if the port is 
        // not properly selected, we default to port A
        match port_id {
            PortName::A => unsafe {&mut *(0x4004_9000 as *mut Port)},
            PortName::B => unsafe {&mut *(0x4004_A000 as *mut Port)},
            PortName::C => unsafe {&mut *(0x4004_B000 as *mut Port)},
            PortName::D => unsafe {&mut *(0x4004_C000 as *mut Port)},
            PortName::E => unsafe {&mut *(0x4004_D000 as *mut Port)},
        }
    }   

    // this function will enable GPIO functions on the input pin number 
    fn set_mode(&mut self, mode: u32, pin: usize) -> () {
        // first we need to read the value at the register
        unsafe {
            let mut reg_val: u32 = ptr::read_volatile(&self.pcr[pin]);
            // now update our pcr value to enable gpio function, we need to set the
            // three least significant bits to 0b100. to do this we need to OR the 1 bit,
            // and then AND the 0 bits. we can use hex values to keep this short and readable.
            reg_val = ((reg_val & !(!mode << 8)) | (mode << 8)) & !(0b1 << 6) & !(0b1 << 2);
            //pcr_val &= 0xFFFFFFFC << 8;
            // now we can write this new value back to the register
            ptr::write_volatile(&mut (*self).pcr[pin], reg_val);
        };
    }

    fn set_interrupt(&mut self, mode: u32, pin: usize) {
        unsafe {
            //first we read the register
            let mut reg_val = ptr::read_volatile(&(*self).pcr[pin]);
            //then we need to do modify the register
            reg_val = (reg_val & !(mode << 16)) | mode << 16;
            // then write the result bac kto the register
            ptr::write_volatile(&mut (*self).pcr[pin], reg_val);
        }
    }
}

impl SafePort {
    // this will call new for each port to create the SafePort struct
    pub const fn take() -> SafePort {
        // create the SafePort instance, using the port::new function call for each port
        SafePort {
            _port_a: Port::new(PortName::A),
            _port_b: Port::new(PortName::B),
            _port_c: Port::new(PortName::C),
            _port_d: Port::new(PortName::D),
            _port_e: Port::new(PortName::E)
        }
    }

    // this will be our function to enable a full port to gpio. this will be updated later to not 
    // require a port selection, only a pin selection
    pub fn set_mode(self, port_id: PortName, mode: u32, pin: usize) {
        // we can just use a match to get call the enable_gpio function for the correct port and 
        // pin
        unsafe {
            match port_id {
                PortName::A => (*self._port_a).set_mode(mode, pin),
                PortName::B => (*self._port_b).set_mode(mode, pin),
                PortName::C => (*self._port_c).set_mode(mode, pin),
                PortName::D => (*self._port_d).set_mode(mode, pin),
                PortName::E => (*self._port_e).set_mode(mode, pin),
            }
        }
    }

    pub fn set_interrupt(self, port_id: PortName, mode: u32, pin: usize) {
        unsafe {
            match port_id {
                PortName::A => (*self._port_a).set_interrupt(mode, pin),
                PortName::B => (*self._port_b).set_interrupt(mode, pin),
                PortName::C => (*self._port_c).set_interrupt(mode, pin),
                PortName::D => (*self._port_d).set_interrupt(mode, pin),
                PortName::E => (*self._port_e).set_interrupt(mode, pin),
            }
        }
    }
}