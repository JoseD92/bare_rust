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

#[repr(packed)]
struct VgaByte {
    char: u8,
    color: u8
}

macro_rules! to_vga{
    ($byte:expr,$c1:expr,$c2:expr)=>{
        {
            let mut byte = VgaByte{char: $byte, color: 0x00};
            byte.color = byte.color | (($c1 as u8) << 4);
            byte.color = byte.color | $c2 as u8;
            byte
        }
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = 0xb8000 as *mut VgaByte;
    let mut i:u32 = 0;
    let mut j:u32 = 0;
    let colors = [VgaColor::Blue,VgaColor::Brown,VgaColor::Cyan,VgaColor::DarkGray,VgaColor::Gray,VgaColor::Green,VgaColor::LightBlue,VgaColor::LightCyan,VgaColor::LightGreen,VgaColor::LightPurple,VgaColor::LightRed,VgaColor::Purple,VgaColor::Red,VgaColor::White,VgaColor::Yellow];

    loop {        
        for b in b"Hello world\n".iter() {
            unsafe {
                *vga = to_vga!(*b,VgaColor::Black,VgaColor::White);
                vga = vga.wrapping_offset(1);
            }
        }
        vga = 0xb8000 as *mut VgaByte;
        j = j + 1;
        if j == 75_000 {
            unsafe {
                *(vga.wrapping_offset(80*24 + i as isize)) = to_vga!(
                    (0 as u8),VgaColor::Black,VgaColor::Black);
                i = (i + 1) % 80;
                *(vga.wrapping_offset(80*24 + i as isize)) = to_vga!(
                    b'j',VgaColor::Black,colors[(i % 15) as usize]);
            }
            j = 0;
        }
    }
}