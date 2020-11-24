#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("1.3 * 2.4 = {}", 1.3 * 2.4);

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
