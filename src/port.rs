// this file will define our port configuration for use within our program. 
// this will allow gpio functions and expose digital outputs

// we will use the volatile_register crate to help us access the peripheral registers
use volatile_register::{RW, WO};

// first we need to define our port configuration registers. we use the repr(C)
// directive to tell the compiler that these data structures need to be stored 
// in the same style as the C compiler. in this case, the struct is public so we
// can use the struct in our program, however the values are private to prevent a
// user from accessing the registers directly. they must instead be accessed through the
// available functions
#[repr(C)]
pub struct Port {
    pcr: [RW<u32>; 32],
    gpclr: WO<u32>,
    gphlr: WO<u32>,
    _pad1: [u32; 2],
    isfr: RW<u32>,
    _pad2: [u32; 3],
    dfer: RW<u32>,
    dfcr: RW<u32>,
    dfwr: RW<u32>,
}

/*
// now we can add our functions to implement port control
*/
//
impl Port {
    // we will write a function to instantiate each port, this can be called so that
    // the user does not need to know the memory locations in order to use the functions
    pub fn new(port_id: char) -> *mut Port {

        let new_port: *mut Port;
        // we use a match statement to check which port needs to be created. if the port is 
        // not properly selected, we default to port A
        match port_id {
            'A' => new_port = 0x4004_9000 as *mut Port,
            'B' => new_port = 0x4004_A000 as *mut Port, 
            'C' => new_port = 0x4004_B000 as *mut Port,
            'D' => new_port = 0x4004_C000 as *mut Port,
            'E' => new_port = 0x4004_D000 as *mut Port,
            _ => new_port = 0x4004_9000 as *mut Port,
        }
        
        // now return the reference
        return new_port
    }   

    // this function will enable GPIO functions on the input pin number 
    pub fn enable_gpio(&self, pin: usize) -> () {
        // first we need to read the value at the register
        let mut pcr_val: u32 = self.pcr[pin].read();
        // now update our pcr value to enable gpio function, we need to set the
        // three least significant bits to 0b100. to do this we need to OR the 1 bit,
        // and then AND the 0 bits. we can use hex values to keep this short and readable.
        pcr_val |= 0x4;
        pcr_val &= 0xFFFFFFFC;
        // now we can write this new value back to the register
        unsafe {self.pcr[pin].write(pcr_val);};
    }
}