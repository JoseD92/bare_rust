#![no_std]
#![no_main]

#[panic_handler]
fn panic(__info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = 0xb8000 as *mut u8;
    let mut offset = 0;
    let mut i = 0;
    let mut j = 0;

    loop {
        for b in b"Hello world\n".iter() {
            unsafe {
                *vga = *b;
                vga = vga.wrapping_offset(1);
                *vga = 0x0f;
                vga = vga.wrapping_offset(1);
            }
        }
        vga = 0xb8000 as *mut u8;
        j = j + 1;
        if j == 75_000 {
            unsafe {
                *(vga.wrapping_offset(80*2*24 + i)) = 0 as u8;
                *(vga.wrapping_offset(80*2*24 + i + 1)) = 0x00;
                i = (i + 2) % (80*2);
                *(vga.wrapping_offset(80*2*24 + i)) = b'j';
                *(vga.wrapping_offset(80*2*24 + i + 1)) = 0x0f;
            }
            j = 0;
        }
    }
}