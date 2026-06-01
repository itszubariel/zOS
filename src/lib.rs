#![no_std]
#![no_main]

mod vga;
mod interrupts;
mod keyboard;
mod commands;

use core::panic::PanicInfo;
use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    vga::clear_screen();
    vga::disable_cursor();

    // Banner
    vga::print_at(b"Welcome to ", 0, 0, 0x0f);
    vga::print_at(b"ZOS", 0, 11, 0x09);
    vga::print_at(b" A hobbyist x86_64 kernel.", 0, 14, 0x0f);
    vga::print_at(b"Version: 0.1.0-alpha", 1, 0, 0x07);
    vga::print_at(b"Type 'help' for commands.", 2, 0, 0x07);

    interrupts::init_pic();
    interrupts::init_idt();

    // Reset prompt row after banner
    unsafe {
        crate::keyboard::CURRENT_ROW = 4;
    }

    keyboard::render_line();

    unsafe {
        asm!("sti");
    }

    loop {
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}
