#![no_std]
#![no_main]

use core::panic::PanicInfo;


// This function will be called when the system is panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point of the system
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
