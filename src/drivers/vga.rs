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

pub fn draw_pride_box(x: usize, y: usize, width: usize, height: usize, title: &str) {
    // 7-color rainbow palette: Red, Orange, Yellow, Green, Cyan, Blue, Magenta
    let colors = [0x04, 0x06, 0x0e, 0x02, 0x03, 0x01, 0x05];
    let mut color_idx = 0;

    let mut next_color = || {
        let color = colors[color_idx];
        color_idx = (color_idx + 1) % colors.len();
        color
    };

    // Calculate border characters
    // Top border
    for i in 0..width {
        let char = if i == 0 { 0xda } else if i == width - 1 { 0xbf } else { 0xc4 };
        write_char_at(char, y, x + i, next_color());
    }

    // Right border
    for i in 1..height - 1 {
        write_char_at(0xb3, y + i, x + width - 1, next_color());
    }

    // Bottom border
    for i in (0..width).rev() {
        let char = if i == 0 { 0xc0 } else if i == width - 1 { 0xd9 } else { 0xc4 };
        write_char_at(char, y + height - 1, x + i, next_color());
    }

    // Left border
    for i in (1..height - 1).rev() {
        write_char_at(0xb3, y + i, x, next_color());
    }

    // Title
    let title_bytes = title.as_bytes();
    for (i, &byte) in title_bytes.iter().enumerate() {
        if i < width - 2 {
            write_char_at(byte, y, x + 1 + i, 0x0f); // White title
        }
    }
}
