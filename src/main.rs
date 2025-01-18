#![no_std] // disables the std. lib.
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo; // imports the PanicInfo struct.

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}
