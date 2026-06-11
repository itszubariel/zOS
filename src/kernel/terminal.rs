use crate::drivers::vga;

pub const MAX_BUFFER_ROWS: usize = 100;
pub const WIDTH: usize = 78; // 80 - 2 for borders
pub const HEIGHT: usize = 23; // 25 - 2 for borders

#[derive(PartialEq)]
pub enum Theme {
    Red,
    Blue,
    Pride,
}

pub struct Terminal {
    pub buffer: [u16; MAX_BUFFER_ROWS * WIDTH],
    pub write_pos: usize,
    pub scroll_offset: usize,
    pub theme: Theme,
}

impl Terminal {
    pub const fn new() -> Self {
        Terminal {
            buffer: [0x0f20; MAX_BUFFER_ROWS * WIDTH], // Space with white on black
            write_pos: 0,
            scroll_offset: 0,
            theme: Theme::Blue, // Default
        }
    }

    pub fn write_str(&mut self, text: &[u8]) {
        self.write_str_with_color(text, 0x0f);
    }

    pub fn write_str_with_color(&mut self, text: &[u8], color: u8) {
        for &byte in text {
            if byte == b'\n' {
                let current_row = self.write_pos / WIDTH;
                self.write_pos = (current_row + 1) * WIDTH;
            } else {
                if self.write_pos < MAX_BUFFER_ROWS * WIDTH {
                    self.buffer[self.write_pos] = ((color as u16) << 8) | (byte as u16);
                    self.write_pos += 1;
                }
            }

            if self.write_pos >= MAX_BUFFER_ROWS * WIDTH {
                self.scroll_buffer_up();
                self.write_pos = (MAX_BUFFER_ROWS - 1) * WIDTH;
            }
        }
        self.auto_scroll();
    }

    fn scroll_buffer_up(&mut self) {
        for i in 0..(MAX_BUFFER_ROWS - 1) * WIDTH {
            self.buffer[i] = self.buffer[i + WIDTH];
        }
        for i in (MAX_BUFFER_ROWS - 1) * WIDTH..MAX_BUFFER_ROWS * WIDTH {
            self.buffer[i] = 0x0f20;
        }
    }

    fn auto_scroll(&mut self) {
        let current_row = self.write_pos / WIDTH;
        if current_row >= self.scroll_offset + HEIGHT {
            self.scroll_offset = current_row - HEIGHT + 1;
        }

        if self.scroll_offset + HEIGHT > MAX_BUFFER_ROWS {
            self.scroll_offset = MAX_BUFFER_ROWS - HEIGHT;
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let current_row = self.write_pos / WIDTH;
        let max_scroll = if current_row >= HEIGHT { current_row - HEIGHT + 1 } else { 0 };

        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
        }
    }

    pub fn page_up(&mut self) {
        for _ in 0..HEIGHT {
            self.scroll_up();
        }
    }

    pub fn page_down(&mut self) {
        for _ in 0..HEIGHT {
            self.scroll_down();
        }
    }

    pub fn clear(&mut self) {
        self.buffer = [0x0f20; MAX_BUFFER_ROWS * WIDTH];
        self.write_pos = 0;
        self.scroll_offset = 0;
    }

    pub fn render(&self) {
        // Redraw border
        match self.theme {
            Theme::Red => vga::draw_box(0, 0, 80, 25, " zOS Terminal ", 0x0c),
            Theme::Blue => vga::draw_box(0, 0, 80, 25, " zOS Terminal ", 0x0b),
            Theme::Pride => vga::draw_pride_box(0, 0, 80, 25, " zOS Terminal "),
        }

        let offset_x = 1;
        let offset_y = 1;
        for row in 0..HEIGHT {
            let buffer_row = self.scroll_offset + row;
            if buffer_row < MAX_BUFFER_ROWS {
                for col in 0..WIDTH {
                    let entry = self.buffer[buffer_row * WIDTH + col];
                    let char = entry as u8;
                    let color = (entry >> 8) as u8;
                    vga::write_char_at(char, offset_y + row, offset_x + col, color);
                }
            }
        }
    }

    pub fn get_cursor_row(&self) -> usize {
        let current_row = self.write_pos / WIDTH;
        if current_row >= self.scroll_offset {
            1 + (current_row - self.scroll_offset) // +1 for border
        } else {
            1
        }
    }

    pub fn get_cursor_col(&self) -> usize {
        1 + (self.write_pos % WIDTH)
    }
}
