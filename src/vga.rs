pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub fn clear_screen() {
    unsafe {
        for i in 0..VGA_WIDTH * VGA_HEIGHT {
            *VGA_BUFFER.add(i * 2) = b' ';
            *VGA_BUFFER.add(i * 2 + 1) = 0x0f;
        }
    }
}

pub fn print_at(text: &[u8], row: usize, col: usize, color: u8) {
    unsafe {
        for (i, &byte) in text.iter().enumerate() {
            let offset = (row * VGA_WIDTH + col + i) * 2;
            if offset < VGA_WIDTH * VGA_HEIGHT * 2 {
                *VGA_BUFFER.add(offset) = byte;
                *VGA_BUFFER.add(offset + 1) = color;
            }
        }
    }
}

pub fn disable_cursor() {
    unsafe {
        crate::interrupts::outb(0x3d4, 0x0a);
        crate::interrupts::outb(0x3d5, 0x20); // Bit 5 set = disable
    }
}
