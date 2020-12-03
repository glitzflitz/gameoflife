#![no_std]
#![no_main]

mod game;
mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    game::start();
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    print!("{}", panic_info);
    loop {}
}
