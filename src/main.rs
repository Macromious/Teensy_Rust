// these directives tell the rust compiler not to use the standard library and not to 
// expect a main function entry point. this allows us to remove overhead we can't use
// from the standard library, and to define our own program entry.
#![no_main]
#![no_std]

// the core panicinfo datatype needs to be included for panic handling
use core::panic::PanicInfo;

// we also need to include all of our chip access files
mod port;
mod gpio;

use crate::port::Port;
use crate::gpio::GPIO;

// we define our reset function that will run whenever the chip is reset. the no_mangle 
// directive tells the compiler to ensure the symbol defined for the function does not change.
// In this case, our symbol will be called reset, and we can then use this to point our
// reset vector to this function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    // in our reset function, we will simply run our main function. this
    // must be declared unsafe, as the compiler cannot complete borrow checking on external
    // functions
    main();

    // if main exits, we want to loop to prevent further execution
    loop {}
}

// wwe must also define the reset vector in order to tell the compiler where our entry point will
// be
#[unsafe(link_section = ".reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

// we also need to define our flash config. This is done by referencing the 16 byte values in the
// cortex datasheet. The layout is as follows
// First 8 bytes is backdoor access Key. this is not used so should have all bytes defaulted 
// to 0xFF. The next 4 bytes handle flash protection. for our purposes we do not want to allow
// any flash protection so these should be filled as 0xFF. The final 4 bytes, refer to FlexNVM
// flash data protection, FlexNVM EEPROM data protection, flash options, and flash security
// respectively. We do not need any data protection so the FlexNVM registers can be set to 0xFF,
// we want all the flash options enabled, such as nmi interrupts and the boot mode. finally, we
// want the MCU to be secure and the ability to mass erase the flash, so our final byte should
// also be set to 0xFF
#[unsafe(link_section = ".flashconfig")]
#[unsafe(no_mangle)]
pub static FLASH_CONFIG: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

// The panic_handler directive tells the compiler the function to run when a panic occurs.
// In this case, as we have no way to alert the user currently, we can simple make the program
// enter an endless loop to prevent further execution of the program
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

// with all the setup complete, we can define our main function and begin writing our program
pub extern "C" fn main() -> () {
    // lets start by attempting to define port C to access the onboard LED
    let port_c: *mut Port = Port::new('C');
    // next we can define our GPIO
    let gpio_c: *mut GPIO = GPIO::new('C');
    // now we can set our pin to gpio. for the onboard LED, this is pin 5
    unsafe {
        (*port_c).enable_gpio(5);
    }

    // with GPIO enabled on our pin, we can now try to turn on the LED 
    unsafe {
        // first set the gpio direction
        (*gpio_c).set_dir(5, 1);
        // then set the output to high
        (*gpio_c).set_output(5, 1);
    }
}