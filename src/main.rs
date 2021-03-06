#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}

static PRINT_MESSAGE: &[u8] = b"That Rusty OS";

#[no_mangle]
pub extern "C" fn _start()-> !{
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in PRINT_MESSAGE.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}