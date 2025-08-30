// these directives tell the rust compiler not to use the standard library and not to expect a main
// function entry point. this allows us to remove overhead we can't use from the standard library, 
// and to define our own program entry.
#![no_main]
#![no_std]
// this line means we won't be notified every time there is an unused function present in our code. 
// this can be removed once we have a consistent setup
#![allow(dead_code)]

// the core panicinfo datatype needs to be included for panic handling
use core::panic::PanicInfo;

// we also need to include all of our chip access files
mod port;
mod gpio;
mod sim;
mod rtc;
mod nvic;
mod watchdog;

// in order to define the vector table, we need to be able to pass multiplee types into a single 
// struct. thst can be done using unions, as the union can define a variable as multiple types.
// this results in reading the value at the union as whichever type is requested. they share the
// same data
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

// here we define our stack and unused exception pointers
unsafe extern "C" {
    fn _stack_top();
}

// we define our reset function that will run whenever the chip is reset. the no_mangle directive
// tells the compiler to ensure the symbol defined for the function does not change. In this case,
// our symbol will be called reset, and we can then use this to point our reset vector to this 
// function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() {
    // in this function, we will just call our main loop. this can be updated later to also do more
    // things when a reset occurs
    main();

    // if main exits, we just want to enter a loop to prevent execution from stopping completely
    loop{};
}

// we must also define the reset vector in order to tell the compiler where our entry point will
// be
#[unsafe(link_section = ".reset_vectors")]
#[unsafe(no_mangle)]
pub static _START_VECTORS: [Vector; 2] = [
    Vector { handler: _stack_top }, 
    Vector { handler: Reset },
];

// next we need to define the exception table to include the device interrupts. the union
// defined earlier can be used to space values correctly without causing type errors
#[unsafe(link_section = ".exception_vectors")]
#[unsafe(no_mangle)]
pub static _EXCEPTIONS: [Vector; 114] = [
    Vector { reserved: 0 },     // NMI
    Vector { reserved: 0 },     // Hard Fault
    Vector { reserved: 0 },     // MemManage Fault
    Vector { reserved: 0 },     // Bus Fault
    Vector { reserved: 0 },     // Usage Fault
    Vector { reserved: 0 },     // reserved
    Vector { reserved: 0 },     // reserved
    Vector { reserved: 0 },     // reserved
    Vector { reserved: 0 },     // reserved
    Vector { reserved: 0 },     // SVCall
    Vector { reserved: 0 },     // Debug
    Vector { reserved: 0 },     // reserved
    Vector { reserved: 0 },     // PSRVReq
    Vector { reserved: 0 },     // SysTick
    Vector { reserved: 0 },     // IRQ0
    Vector { reserved: 0 },     // IRQ1
    Vector { reserved: 0 },     // IRQ2
    Vector { reserved: 0 },     // IRQ3
    Vector { reserved: 0 },     // IRQ4
    Vector { reserved: 0 },     // IRQ5
    Vector { reserved: 0 },     // IRQ6
    Vector { reserved: 0 },     // IRQ7
    Vector { reserved: 0 },     // IRQ8
    Vector { reserved: 0 },     // IRQ9
    Vector { reserved: 0 },     // IRQ10
    Vector { reserved: 0 },     // IRQ11
    Vector { reserved: 0 },     // IRQ12
    Vector { reserved: 0 },     // IRQ13
    Vector { reserved: 0 },     // IRQ14
    Vector { reserved: 0 },     // IRQ15
    Vector { reserved: 0 },     // IRQ16
    Vector { reserved: 0 },     // IRQ17
    Vector { reserved: 0 },     // IRQ18
    Vector { reserved: 0 },     // IRQ19
    Vector { reserved: 0 },     // IRQ20
    Vector { reserved: 0 },     // IRQ21
    Vector { reserved: 0 },     // IRQ22
    Vector { reserved: 0 },     // IRQ23
    Vector { reserved: 0 },     // IRQ24
    Vector { reserved: 0 },     // IRQ25
    Vector { reserved: 0 },     // IRQ26
    Vector { reserved: 0 },     // IRQ27
    Vector { reserved: 0 },     // IRQ28
    Vector { reserved: 0 },     // IRQ29
    Vector { reserved: 0 },     // IRQ30
    Vector { reserved: 0 },     // IRQ31
    Vector { reserved: 0 },     // IRQ32
    Vector { reserved: 0 },     // IRQ33
    Vector { reserved: 0 },     // IRQ34
    Vector { reserved: 0 },     // IRQ35
    Vector { reserved: 0 },     // IRQ36
    Vector { reserved: 0 },     // IRQ37
    Vector { reserved: 0 },     // IRQ38
    Vector { reserved: 0 },     // IRQ39
    Vector { reserved: 0 },     // IRQ40
    Vector { reserved: 0 },     // IRQ41
    Vector { reserved: 0 },     // IRQ42
    Vector { reserved: 0 },     // IRQ43
    Vector { reserved: 0 },     // IRQ44
    Vector { reserved: 0 },     // IRQ45
    Vector { reserved: 0 },     // IRQ46
    Vector { handler: rtc_seconds_handler },    // IRQ47
    Vector { reserved: 0 },     // IRQ48
    Vector { reserved: 0 },     // IRQ49
    Vector { reserved: 0 },     // IRQ50
    Vector { reserved: 0 },     // IRQ51
    Vector { reserved: 0 },     // IRQ52
    Vector { reserved: 0 },     // IRQ53
    Vector { reserved: 0 },     // IRQ54
    Vector { reserved: 0 },     // IRQ55
    Vector { reserved: 0 },     // IRQ56
    Vector { reserved: 0 },     // IRQ57
    Vector { reserved: 0 },     // IRQ58
    Vector { reserved: 0 },     // IRQ59
    Vector { reserved: 0 },     // IRQ60
    Vector { reserved: 0 },     // IRQ61
    Vector { reserved: 0 },     // IRQ62
    Vector { reserved: 0 },     // IRQ63
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },

];

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
pub static _FLASH_CONFIG: [u8; 16] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF];

// The panic_handler directive tells the compiler the function to run when a panic occurs.
// In this case, as we have no way to alert the user currently, we can simple make the program
// enter an endless loop to prevent further execution of the program
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop{}
}

#[unsafe(no_mangle)]
// define a function for handling the RTC Second Interrupt. this interrupt triggers once per second
pub extern "C" fn rtc_seconds_handler() {
    // inside this function, we need to define what to do on an NMI. for this case, we would like 
    // to toggle the output of pin 13. to toggle the pin, use the ptor register
    unsafe {
        // start by clearing the interrupt
        //NVIC.clear_interrupt(47);
        // then toggle the output
        GPIO.toggle_output(gpio::PortName::C, 5);
    }
    return;
}

/*
// THIS SECTION IS DEFINING GLOBAL VARIABLES AND SHOULD BE CHANGED WHEN A BETTER SOLUTION IS FOUND
*/
static mut WATCHDOG: watchdog::SafeWatchdog = watchdog::SafeWatchdog::take();
static mut NVIC: nvic::SafeNVIC = nvic::SafeNVIC::take();
static mut GPIO: gpio::SafeGPIO = gpio::SafeGPIO::take();
static mut SIM: sim::SafeSIM = sim::SafeSIM::take();
static mut RTC: rtc::SafeRTC = rtc::SafeRTC::take();
static mut PORT: port::SafePort = port::SafePort::take();

// with all the setup complete, we can define our main function and begin writing our program
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() {

    // we need to disable the watchdog before anything else can happen
    WATCHDOG.enable_watchdog(false);

    // first we enable RTC access using the SIM
    SIM.rtc_access(true);

    // we need to disable the seconds counter before enabling the oscillator
    RTC.enable_counter(false);
    // next we enable the oscillator
    RTC.enable_osc();

    // next we enable the clock gating for port C
    SIM.cg5_control(sim::PortName::C);

    // now we can set our pin to gpio. for the onboard LED, this is pin 13, which 
    // corresponds to portc_5
    PORT.set_mode(port::PortName::C, 1, 5);

    // set the gpio direction
    GPIO.set_dir(gpio::PortName::C, 5, true);

    // next we need to enable the rtc
    RTC.enable_rtc();
    // then we enable the seconds interrupt
    RTC.enable_seconds_interrupt();
    // also enable the interrupt using the nvic
    NVIC.enable_interrupt(47, true);

    // initially make sure the led is off
    GPIO.set_output(gpio::PortName::C, 5, false);

    loop {}
}