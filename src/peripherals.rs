// we need to include the modules for each peripheral here
use crate::port;
use crate::sim;
use crate::watchdog;
use crate::gpio;

// this file wil create a struct to use as a singleton for the entire abstractionn of hardware, 
// we can then use this to 'take control' of each peripheral in order to  track ownership of 
// each peripheral in the system. we can also potentially use this to remove dereferencing from
// the main.rs file

// thisa is the struct that will store all ourr peripherals, this will be expanded as new 
// peripherals are added
pub struct Peripherals {
    sim: &'static mut sim::SIM,
    port: &'static mut port::Port,
    watchdog: &'static mut watchdog::Watchdog,
    gpio: &'static mut gpio::GPIO,
}

// here we only need to implement the take and release functions of the peripheral struct to allow
// a single value to take control of each peripheral at a time
impl peripherals {
    // the take function passes the address relating to the hardware to the variable defined
    pub fn take() {

    }
}