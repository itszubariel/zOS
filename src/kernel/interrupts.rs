use core::arch::asm;
use crate::drivers::keyboard;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

#[repr(C, packed)]
pub struct IDTPtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IDTEntry; 256] = [
    IDTEntry {
        offset_low: 0,
        selector: 0,
        ist: 0,
        type_attr: 0,
        offset_mid: 0,
        offset_high: 0,
        zero: 0,
    };
    256
];

// The CPU pushes this on the stack automatically
#[allow(dead_code)]
#[repr(C)]
pub struct InterruptStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

unsafe extern "C" {
    fn keyboard_interrupt_handler();
    fn timer_interrupt_handler();
}

pub fn init_idt() {
    let handler_addr = keyboard_interrupt_handler as *const () as u64;
    let timer_handler_addr = timer_interrupt_handler as *const () as u64;

    unsafe {
        IDT[0x20] = IDTEntry {
            offset_low: timer_handler_addr as u16,
            selector: 0x08,
            ist: 0,
            type_attr: 0x8e,
            offset_mid: (timer_handler_addr >> 16) as u16,
            offset_high: (timer_handler_addr >> 32) as u32,
            zero: 0,
        };
        IDT[0x21] = IDTEntry {
            offset_low: handler_addr as u16,
            selector: 0x08, // Kernel code segment
            ist: 0,
            type_attr: 0x8e, // Interrupt gate, present, ring 0
            offset_mid: (handler_addr >> 16) as u16,
            offset_high: (handler_addr >> 32) as u32,
            zero: 0,
        };

        let idt_ptr = IDTPtr {
            limit: (256 * 16 - 1) as u16,
            base: &raw const IDT as u64,
        };

        asm!("lidt [{}]", in(reg) &idt_ptr);
    }
}

pub fn init_pic() {
    unsafe {
        // ICW1: Start initialization
        outb(0x20, 0x11);
        outb(0xa0, 0x11);
        // ICW2: Set master offset to 0x20, slave to 0x28
        outb(0x21, 0x20);
        outb(0xa1, 0x28);
        // ICW3: Cascade
        outb(0x21, 0x04);
        outb(0xa1, 0x02);
        // ICW4: 8086 mode
        outb(0x21, 0x01);
        outb(0xa1, 0x01);
        // Unmask IRQ0 (timer) and IRQ1 (keyboard)
        outb(0x21, 0xfc);
        outb(0xa1, 0xff);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn keyboard_handler_inner() {
    unsafe {
        let scancode = inb(0x60);
        keyboard::handle_scancode(scancode);
        // EOI to master PIC
        outb(0x20, 0x20);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn timer_handler_inner() {
    crate::drivers::pit::tick();
    unsafe {
        outb(0x20, 0x20);
    }
}

unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") result);
    }
    result
}

pub unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}
