#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Error de panico: {}", _info);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Imprimeme... {}", "!");
    loop{}
}