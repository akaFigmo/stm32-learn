// 1. define the vector table for micro


// 2. define the reset handler
#[unsafe(no_mangle)]
fn reset_handler() {
    // 1. copy .data from FLASH to RAM
    // 2. zero out the .bss section in the RAM
    // 3. call main()
    crate::main();
}

// 3. define teh exception handlers
