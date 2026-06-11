use crate::drivers::vga;
use crate::kernel::commands;

static mut INPUT_BUFFER: [u8; 80] = [0; 80];
static mut BUFFER_LEN: usize = 0;
static mut SHIFT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;
pub static mut CURRENT_ROW: usize = 0;

pub fn render_line() {
    unsafe {
        let term = (&raw mut crate::TERMINAL).as_mut().unwrap();
        CURRENT_ROW = term.get_cursor_row();

        let current_row_in_history = term.write_pos / crate::kernel::terminal::WIDTH;
        if
            current_row_in_history < term.scroll_offset ||
            current_row_in_history >= term.scroll_offset + crate::kernel::terminal::HEIGHT
        {
            return;
        }

        let mut col = 1; // Start at 1 for border

        // Render prompt: [root@zos]$
        vga::print_at(b"[", CURRENT_ROW, col, 0x07);
        col += 1;
        vga::print_at(b"root", CURRENT_ROW, col, 0x0a);
        col += 4;
        vga::print_at(b"@", CURRENT_ROW, col, 0x07);
        col += 1;
        vga::print_at(b"zos", CURRENT_ROW, col, 0x09);
        col += 3;
        vga::print_at(b"]$ ", CURRENT_ROW, col, 0x07);
        col += 3;

        let prompt_len = col;

        // Print input buffer
        vga::print_at(&INPUT_BUFFER[..BUFFER_LEN], CURRENT_ROW, prompt_len, 0x0f);

        // Print Cursor (Software Cursor)
        vga::print_at(b"_", CURRENT_ROW, prompt_len + BUFFER_LEN, 0x0f);

        // Update hardware cursor position
        vga::update_cursor(CURRENT_ROW, prompt_len + BUFFER_LEN);

        // Clear remaining
        for i in prompt_len + BUFFER_LEN + 1..crate::kernel::terminal::WIDTH + 1 {
            vga::write_char_at(b' ', CURRENT_ROW, i, 0x0f);
        }
    }
}

pub fn handle_scancode(scancode: u8) {
    match scancode {
        0x0e => {
            // Backspace
            unsafe {
                if BUFFER_LEN > 0 {
                    BUFFER_LEN -= 1;
                    INPUT_BUFFER[BUFFER_LEN] = 0;
                }
            }
        }
        0x1c => {
            // Enter
            unsafe {
                let term = (&raw mut crate::TERMINAL).as_mut().unwrap();

                // Print command to terminal history with full color support
                term.write_str_with_color(b"[", 0x07);
                term.write_str_with_color(b"root", 0x0a);
                term.write_str_with_color(b"@", 0x07);
                term.write_str_with_color(b"zos", 0x09);
                term.write_str_with_color(b"]$ ", 0x07);

                term.write_str(&INPUT_BUFFER[..BUFFER_LEN]);
                term.write_str(b"\n");

                commands::run(&raw const INPUT_BUFFER as *const u8, BUFFER_LEN, term);

                // After command, we might have multiple lines of output.
                // Update CURRENT_ROW to the next available row in terminal.
                CURRENT_ROW = term.get_cursor_row();

                BUFFER_LEN = 0;
                INPUT_BUFFER = [0; 80];

                term.render();
            }
        }
        0x49 => {
            // Page Up
            unsafe {
                let term = (&raw mut crate::TERMINAL).as_mut().unwrap();
                term.page_up();
                term.render();
            }
        }
        0x51 => {
            // Page Down
            unsafe {
                let term = (&raw mut crate::TERMINAL).as_mut().unwrap();
                term.page_down();
                term.render();
            }
        }
        0x2a | 0x36 => unsafe {
            SHIFT_PRESSED = true;
        }
        0xaa | 0xb6 => unsafe {
            SHIFT_PRESSED = false;
        }
        0x3a => unsafe {
            CAPS_LOCK = !CAPS_LOCK;
        }
        _ => {
            let character = scancode_to_char(scancode);
            if character != 0 {
                unsafe {
                    if BUFFER_LEN < 80 - 12 - 1 {
                        INPUT_BUFFER[BUFFER_LEN] = character;
                        BUFFER_LEN += 1;
                    }
                }
            }
        }
    }
    render_line();
}

fn scancode_to_char(scancode: u8) -> u8 {
    let map = if (unsafe { SHIFT_PRESSED ^ CAPS_LOCK }) {
        b")!@#$%^&*("
    } else {
        b"0123456789"
    };

    match scancode {
        0x02..=0x0b => map[(scancode - 0x02) as usize],
        0x1e => b'a',
        0x30 => b'b',
        0x2e => b'c',
        0x20 => b'd',
        0x12 => b'e',
        0x21 => b'f',
        0x22 => b'g',
        0x23 => b'h',
        0x17 => b'i',
        0x24 => b'j',
        0x25 => b'k',
        0x26 => b'l',
        0x32 => b'm',
        0x31 => b'n',
        0x18 => b'o',
        0x19 => b'p',
        0x10 => b'q',
        0x13 => b'r',
        0x1f => b's',
        0x14 => b't',
        0x16 => b'u',
        0x2f => b'v',
        0x11 => b'w',
        0x2d => b'x',
        0x15 => b'y',
        0x2c => b'z',
        0x39 => b' ',
        _ => 0,
    }
}
