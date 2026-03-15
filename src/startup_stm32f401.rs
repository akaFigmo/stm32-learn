// 1. define the vector table for micro


// 2. define the reset handler
fn reset_handler() {
    // 1. copy .data from FLASH to RAM
    // 2. zero out the .bss section in the RAM
    // 3. call main()
    
}

// 3. define teh exception handlers
