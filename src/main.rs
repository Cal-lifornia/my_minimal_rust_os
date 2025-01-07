#![no_std]
#![no_main]
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Cast the integer into a raw pointer of the VGA buffer address
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Write the string byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // Write the cyan colour byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
