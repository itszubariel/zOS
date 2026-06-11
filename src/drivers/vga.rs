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

pub fn write_char_at(char: u8, row: usize, col: usize, color: u8) {
    unsafe {
        let offset = (row * VGA_WIDTH + col) * 2;
        if offset < VGA_WIDTH * VGA_HEIGHT * 2 {
            *VGA_BUFFER.add(offset) = char;
            *VGA_BUFFER.add(offset + 1) = color;
        }
    }
}

pub fn enable_cursor() {
    unsafe {
        crate::kernel::interrupts::outb(0x3d4, 0x0a);
        crate::kernel::interrupts::outb(0x3d5, 0x0e); // Cursor start: line 14 (thinner)
        crate::kernel::interrupts::outb(0x3d4, 0x0b);
        crate::kernel::interrupts::outb(0x3d5, 0x0f); // Cursor end: line 15
    }
}

pub fn update_cursor(row: usize, col: usize) {
    let pos = (row * VGA_WIDTH + col) as u16;
    unsafe {
        crate::kernel::interrupts::outb(0x3d4, 0x0f);
        crate::kernel::interrupts::outb(0x3d5, (pos & 0xff) as u8);
        crate::kernel::interrupts::outb(0x3d4, 0x0e);
        crate::kernel::interrupts::outb(0x3d5, ((pos >> 8) & 0xff) as u8);
    }
}

pub fn draw_box(x: usize, y: usize, width: usize, height: usize, title: &str, color: u8) {
    // Top-left corner
    write_char_at(0xda, y, x, color);
    // Top-right corner
    write_char_at(0xbf, y, x + width - 1, color);
    // Bottom-left corner
    write_char_at(0xc0, y + height - 1, x, color);
    // Bottom-right corner
    write_char_at(0xd9, y + height - 1, x + width - 1, color);

    // Top and Bottom lines
    for i in 1..width - 1 {
        write_char_at(0xc4, y, x + i, color);
        write_char_at(0xc4, y + height - 1, x + i, color);
    }
    // Side lines
    for i in 1..height - 1 {
        write_char_at(0xb3, y + i, x, color);
        write_char_at(0xb3, y + i, x + width - 1, color);
    }

    // Title
    let title_bytes = title.as_bytes();
    for (i, &byte) in title_bytes.iter().enumerate() {
        if i < width - 2 {
            write_char_at(byte, y, x + 1 + i, color | 0x08); // Title highlight
        }
    }
}
