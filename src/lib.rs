#![no_std]
#![no_main]

extern crate alloc;

pub mod drivers;
pub mod kernel;
pub mod ui;

use core::panic::PanicInfo;
use core::arch::asm;
use crate::kernel::terminal::Terminal;

pub static mut TERMINAL: Terminal = Terminal::new();

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    drivers::vga::clear_screen();

    // Initialize heap
    unsafe {
        kernel::memory::ALLOCATOR.init(
            &raw const kernel::memory::HEAP_MEM as usize,
            (&raw const kernel::memory::HEAP_MEM as usize) + kernel::memory::HEAP_SIZE
        );
    }

    drivers::vga::enable_cursor();
    drivers::vga::update_cursor(1, 1);

    // Draw full-screen border
    drivers::vga::draw_box(0, 0, 80, 25, " zOS Terminal ", 0x0b);

    // Initial banner
    unsafe {
        let term = (&raw mut TERMINAL).as_mut().unwrap();
        term.write_str_with_color(b"Welcome to zOS v0.1.1-alpha\n", 0x0e); // Yellow for contrast
        term.write_str(b"Type 'help' for commands.\n");
        term.render();
    }

    kernel::interrupts::init_pic();
    kernel::interrupts::init_idt();
    drivers::pit::init_pit(100);

    drivers::keyboard::render_line();

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
