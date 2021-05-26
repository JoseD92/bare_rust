#![no_std]
#![no_main]

use modular_bitfield::prelude::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[derive(Debug, Copy, Clone, BitfieldSpecifier)]
#[bits = 4]
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

#[bitfield]
struct VgaByte {
    char: u8,
    foreground: VgaColor,
    background: VgaColor,
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

    let mut vga2 = (0xb8000 + last_line) as *mut VgaByte;
    let mut vga3 = (0xb8000 + last_line) as *mut VgaByte;
    let mut j:u32 = 0;
    let colors = [Blue,Brown,Cyan,DarkGray,Gray,Green,LightBlue,LightCyan,LightGreen,LightPurple,LightRed,Purple,Red,White,Yellow];
    let mut colors_ptr = colors.as_ptr();
    let vga_array: *mut [VgaByte; 80*24] = 0xb8000 as *mut [VgaByte; 80*24];
    let vga_array: &mut [VgaByte; 80*24] = unsafe { &mut *vga_array };

    loop {
        for (i, b) in b"Hello world\n".iter().enumerate() {
            vga_array[i] = VgaByte::new().with_char(*b).with_background(Yellow).with_foreground(White);
        }
        for (i, b) in b"second line".iter().enumerate() {
            vga_array[i+80] = VgaByte::new().with_char(*b).with_background(LightPurple).with_foreground(White);
        }
        j = j + 1;
        if j == 75_000 {
            to_vga!(vga2,0 as u8,Black,Black);
            vga2 = if vga2 != (0xb8000 + out_line - core::mem::size_of::<VgaByte>() as u32) as *mut VgaByte
                {vga2.wrapping_offset(1)}
                else {(0xb8000 + last_line) as *mut VgaByte};
            vga3 = vga2;
            colors_ptr = if colors_ptr != colors.as_ptr().wrapping_offset(14)
                {colors_ptr.wrapping_offset(1)}
                else {colors.as_ptr()};
            for b in b"Jose".iter() {
                to_vga!(vga3,*b,Black,*colors_ptr);
                vga3 = if vga3 != (0xb8000 + out_line - core::mem::size_of::<VgaByte>() as u32) as *mut VgaByte
                    {vga3.wrapping_offset(1)}
                    else {(0xb8000 + last_line) as *mut VgaByte};
            }
            j = 0;
        }
    }
}