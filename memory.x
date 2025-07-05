/*
This first section simply tells the compiler where each section of memory is 
addressed, as well as it's size. for our initial layout, we allowing access
to the entire flash memory and ram
*/
MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, Length = 1M
    RAM (rwx) : ORIGIN = 0x1FFF0000, LENGTH = 256K
}

/*
This section is how we describe the location of our program and it's data. 
*/
SECTIONS {
    /*
    The .text section is where all the program code should be placed. for this 
    embedded implementation, code should be stored in flash, as well as some other
    data that is required for the function of the processor
    */
    .text : {
        /*
        Cortex-M processors have a vector table listed at the start of flash. The
        first address, 0x0000_0000, is the stack pointer and the second address,
        0x0000_0004 is the program pointer. the stack pointer is defined as the address
        at the end of ur ram, as it is a descending stack. the reset vector is defined
        in main.rs
        */
        . = 0;
        LONG(ORIGIN(RAM) + LENGTH(RAM))
        KEEP(*(.reset_vector))
        /*
        The next section we need to describe is the flash config. The cortex processor
        has a 16 byte flash configuration stored at 0x0000_0400. This describes any 
        restrictions to the flash module
        */
        . = 0x400;
        KEEP(*(.flashconfig*))
        /*
        Once the vector table and flash config are defined, we can write our program
        code. This needs to start at a x4 alilgned address, as the processor accesses
        data in 4 byte sections.
        */
        . = ALIGN(4);
        *(.text*)
    /*
    The > FLASH = 0xFF descriptors tell the compiler to place all the above data into 
    the flash section of memory, and to fill any blank space with the value 0xFF. this 
    value is chosen as Flash is erased to 0xFF and then writing will flip the required 
    bits as needed to reduce the number of writes to the flash.
    */
    } > FLASH = 0xFF

    /*
    The rodata section describes all the data that is read only and will not be changed
    */
    .rodata : {
        *(.rodata*)
    /*
    all rodata should be stored in flash for permanence. this can be read to ram as needed
    */
    } > FLASH

    /*
    The /DISCARD/ section simply tells the compiler not to include this data in the final 
    binary
    */ 
    /DISCARD/ : {
        *(.arm*)
    }
}