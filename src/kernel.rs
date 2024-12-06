#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(int_roundings)]


use core::{panic::PanicInfo, ptr::{read_volatile, write_volatile}};
use paging::{Mapper, VirtualAddress, AddressType::{VirtualAddress as VA_t, PhysicalAddress as PA_t}};
use vga::Vga;

use crate::{interrupts::{check_apic, get_msr_val}, ata::read_chs, core::arch::asm};


mod regs;
mod interrupts;
mod paging;
mod ata;
mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut display = Vga::new();
    display.print_string("panic :(");
    loop {}
}

pub fn temp_panic(_msg: &'static str) -> !{
    let mut display = Vga::new();
    display.print_string("panic temp! :(");
    loop {};
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let mut display = Vga::new();
    display.clrscr();
    unsafe{
        read_chs(0x9200 as *const u64, 0x0000000000A0340C);
        write_volatile(0x9200 as *mut u64, 0x69420);
        display.print_string("BASIC ASS OS TESTING V34\n64-bit mode\nFirst stage loader: [0x7e00-0x9200]\nID Mapping: DONE\nVGA Address:");
        display.print_hex(display.vga_address as u64);
    }

    loop {}
}
