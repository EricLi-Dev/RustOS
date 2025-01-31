#![no_std] // disables the std. lib.
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo; // imports the PanicInfo struct.

static HELLO: &[u8] = b"Hello World!";

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // raw mutable pointer to the mem. addr.

    // &byte: dereference the &u8 byte to get the actual u8 value
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // write the string byte and corresponding color byte
            // to the correct offset i away
            // multiply by 2 to correctly address the byte pair
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
