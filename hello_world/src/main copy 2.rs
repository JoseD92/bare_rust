#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn init_heap() {
    let heap_start = 0x4444_4444_0000;
    let heap_end = 2_000_000;
    let heap_size = 10 * 1024 * 1024;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

#[repr(C)]
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
#[derive(Debug, Copy, Clone)]
struct VgaByte {
    char: u8,
    backgroundColor: VgaColor,
    letterColor: VgaColor,
}

fn to_vga(string: &[u8], c1: VgaColor, c2: VgaColor) -> Vec<VgaByte> {
    string.into_iter().map(|&x| VgaByte{
        char: x,
        backgroundColor: c1,
        letterColor: c2
    }).collect()
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_heap();
    let mut vga = 0xb8000 as *mut VgaByte;
    let mut i = 0;
    let mut j = 0;
    let helloVga = to_vga (b"Hello world\n", VgaColor::Black, VgaColor::White);

    loop {
        for b in helloVga.iter() {
            unsafe {
                *vga = *b;
                vga = vga.wrapping_offset(1);
            }
        }
        vga = 0xb8000 as *mut VgaByte;
        j = j + 1;
        if j == 75_000 {
            unsafe {
                *(vga.wrapping_offset(80*24 + i)) = VgaByte{
                    char: (0 as u8),
                    backgroundColor: VgaColor::Black,
                    letterColor: VgaColor::Black};
                i = (i + 1) % 80;
                *(vga.wrapping_offset(80*24 + i)) = VgaByte{
                    char: b'j',
                    backgroundColor: VgaColor::Black,
                    letterColor: VgaColor::Black};
            }
            j = 0;
        }
    }
}