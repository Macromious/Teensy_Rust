// this file will define our port configuration for use within our program. 
// this will allow gpio functions and expose digital outputs

// we will use the volatile_register crate to help us access the peripheral registers
use core::ptr;

// first we define an enum where we can place each port in it's own type
pub enum _PortName {
    C
}

// first we need to define our port configuration registers. we use the repr(C)
// directive to tell the compiler that these data structures need to be stored 
// in the same style as the C compiler. in this case, the struct is public so we
// can use the struct in our program, however the values are private to prevent a
// user from accessing the registers directly. they must instead be accessed through the
// available functions
#[repr(C)]
pub struct Port {
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
    pub fn new(port_id: _PortName) -> &'static mut Port {
        // we use a match statement to check which port needs to be created. if the port is 
        // not properly selected, we default to port A
        match port_id {
            _PortName::C => unsafe {&mut *(0x4004_B000 as *mut Port)},
        }
    }   

    // this function will enable GPIO functions on the input pin number 
    pub fn enable_gpio(&mut self, pin: usize) -> () {
        // first we need to read the value at the register
        unsafe {
            let mut reg_val: u32 = ptr::read_volatile(&self.pcr[pin]);
            // now update our pcr value to enable gpio function, we need to set the
            // three least significant bits to 0b100. to do this we need to OR the 1 bit,
            // and then AND the 0 bits. we can use hex values to keep this short and readable.
            reg_val |= 1 << 8;
            //pcr_val &= 0xFFFFFFFC << 8;
            // now we can write this new value back to the register
            ptr::write_volatile(&mut (*self).pcr[pin], reg_val);
        };
    }
}