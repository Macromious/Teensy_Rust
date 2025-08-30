// the NVIC registers are used to enable device interrupts.

use core::ptr;

// create our safe wrapper for the nvic registers that has the copy trait
#[derive(Copy, Clone)]
pub struct SafeNVIC {
    nvic: *mut NVIC,
}

// this represents our nvic registers. we will use this to configure interrupts
#[repr(C)]
struct NVIC {
    ictr: u32,
    _pad0: [u32; 62],
    iser: [u32; 8],
    _pad1: [u32; 24],
    icer: [u32; 8],
    _pad2: [u32; 24],
    ispr: [u32; 8],
    _pad3: [u32; 24],
    icpr: [u32; 8],
    _pad4: [u32; 24],
    iabr: [u32; 8],
    _pad5: [u32; 56],
    ipr: [u32; 60],
}

impl NVIC {
    // first we need to implement our initialisation function
    const fn new() -> &'static mut NVIC {
        unsafe { &mut *(0xE000_E004 as *mut NVIC) }
    }
    
    // fn read_reg(&self) -> u32 {
    //     return ptr::read_volatile(&(*self).)
    // }

    // next we can create our interrupt enable function. this will involve taking the IRQ position 
    // of the register. later this can be done using a table to map the IRQ to the interrupt we 
    // want
    fn enable_interrupt(&mut self, irq: usize, enable: bool) {
        // first we can calculate our nvic register number and bit position using the IQR
        let non_ipr: usize = irq / 32;
        let non_ipr_bit: usize = irq % 32;
        // these are here for safekeeping, used for setting priority
        // let ipr: u32 = irq / 4;
        // let ipr_bit: u32 = 8 * (irq % 4) + 4;
        
        unsafe {
            // now we can update the interrupt enable register to enable or disable the interrupt
            let mut reg_val: u32 = ptr::read_volatile(&(*self).iser[non_ipr]);
            // check if we are enabling or disabling the interrupt
            if enable {
                // to enable the interrupt, set the bit to 1
                reg_val |= 0b1 << non_ipr_bit;
            } else {
                // otherwise disable it by setting to 0
                reg_val &= !(0b1 << non_ipr_bit);
            }

            // finally, we can write the updated value to the register
            ptr::write_volatile(&mut (*self).iser[non_ipr], reg_val);
        }
    }

    // we also need a function to be able to clear the interrupt when triggered
    fn _clear_interrupt(&mut self, irq: usize) {
        // first we can calculate our nvic register number and bit position using the IQR
        let non_ipr: usize = irq / 32;
        let non_ipr_bit: usize = irq % 32;
        
        unsafe {
            // now we can update the interrupt clear register to enable or disable the interrupt
            let mut reg_val: u32 = ptr::read_volatile(&(*self).icpr[non_ipr]);
            // otherwise disable it by setting to 0
            reg_val &= 0b1 << non_ipr_bit;
            // finally, we can write the updated value to the register
            ptr::write_volatile(&mut (*self).icpr[non_ipr], reg_val);
        }
    }
}

// wrap the unsafe functions to the SafeNVIC struct so we can use them
impl SafeNVIC {
    pub const fn take() -> SafeNVIC {
        return SafeNVIC { nvic: NVIC::new() }
    }

     pub fn enable_interrupt(self, irq: usize, enable: bool) {
        unsafe {
            (*self.nvic).enable_interrupt(irq, enable);
        }
    }

    pub fn _clear_interrupt(self, irq: usize) {
        unsafe {
            (*self.nvic)._clear_interrupt(irq);
        }
    }
}