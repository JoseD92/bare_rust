#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum VgaColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPurple,
    Yellow,
    White
}

#[inline(always)]
fn compress_colors(c1:VgaColor,c2:VgaColor) -> u8 {
    ((c1 as u8) << 4) | c2 as u8
}

#[repr(packed)]
struct VgaByte {
    char: u8,
    color: u8
}

macro_rules! to_vga{
    ($byte:expr,$c1:expr,$c2:expr)=>{
        {
            VgaByte{char: $byte, color: compress_colors($c1,$c2)}
        }
    };
    ($ptr:expr,$byte:expr,$c1:expr,$c2:expr)=>{
        {
            unsafe {
                let u8_ptr = $ptr as *mut u8;
                *u8_ptr = $byte;
                *(u8_ptr.wrapping_offset(1)) = compress_colors($c1,$c2);
            }
        }
    };
}

const last_line:u32 = 80 * (core::mem::size_of::<VgaByte>() as u32) * 24;
const out_line:u32 = 80 * (core::mem::size_of::<VgaByte>() as u32) * 25;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use VgaColor::*;

    let mut vga;
    let mut vga2 = (0xb8000 + last_line) as *mut VgaByte;
    let mut j:u32 = 0;
    let colors = [Blue,Brown,Cyan,DarkGray,Gray,Green,LightBlue,LightCyan,LightGreen,LightPurple,LightRed,Purple,Red,White,Yellow];
    let mut colors_ptr = colors.as_ptr();

    loop {
        vga = 0xb8000 as *mut VgaByte;
        for b in b"Hello world\n".iter() {
            to_vga!(vga,*b,Black,White);
            vga = vga.wrapping_offset(1);
        }
        j = j + 1;
        if j == 75_000 {
            to_vga!(vga2,0 as u8,Black,Black);
            vga2 = if vga2 != (0xb8000 + out_line) as *mut VgaByte {vga2.wrapping_offset(1)}
                else {(0xb8000 + last_line) as *mut VgaByte};
            colors_ptr = if colors_ptr != colors.as_ptr().wrapping_offset(15)
                {colors_ptr.wrapping_offset(1)}
                else {colors.as_ptr()};
            to_vga!(vga2,b'j',Black,*colors_ptr);
            j = 0;
        }
    }
}