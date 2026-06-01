use crate::vga;
use crate::commands;

static mut INPUT_BUFFER: [u8; 80] = [0; 80];
static mut BUFFER_LEN: usize = 0;
static mut SHIFT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;
pub static mut CURRENT_ROW: usize = 2;
const PROMPT: &[u8] = b"[root@zos]$ ";

pub fn render_line() {
    unsafe {
        let mut col = 0;

        // Render prompt: [root@zos]$
        vga::print_at(b"[", CURRENT_ROW, col, 0x07); col += 1;
        vga::print_at(b"root", CURRENT_ROW, col, 0x0a); col += 4;
        vga::print_at(b"@", CURRENT_ROW, col, 0x07); col += 1;
        vga::print_at(b"zos", CURRENT_ROW, col, 0x09); col += 3;
        vga::print_at(b"]$ ", CURRENT_ROW, col, 0x07); col += 3;

        let prompt_len = col; // 12

        // Print input buffer
        vga::print_at(&INPUT_BUFFER[..BUFFER_LEN], CURRENT_ROW, prompt_len, 0x0f);

        // Print Cursor
        vga::print_at(b"_", CURRENT_ROW, prompt_len + BUFFER_LEN, 0x0f);

        // Clear remaining
        for i in (prompt_len + BUFFER_LEN + 1)..80 {
            vga::print_at(b" ", CURRENT_ROW, i, 0x0f);
        }
    }
}

pub fn handle_scancode(scancode: u8) {
    if scancode == 0 {
        return;
    }

    // Shift
    if scancode == 0x2a || scancode == 0x36 {
        unsafe {
            SHIFT_PRESSED = true;
        }
        return;
    }
    if scancode == 0xaa || scancode == 0xb6 {
        unsafe {
            SHIFT_PRESSED = false;
        }
        return;
    }

    // Caps Lock
    if scancode == 0x3a {
        unsafe {
            CAPS_LOCK = !CAPS_LOCK;
        }
        return;
    }

    if (scancode & 0x80) != 0 {
        return;
    }

    // Backspace
    if scancode == 0x0e {
        unsafe {
            if BUFFER_LEN > 0 {
                BUFFER_LEN -= 1;
            }
        }
        render_line();
        return;
    }

    // Enter
    if scancode == 0x1c {
        unsafe {
            commands::run(&raw const INPUT_BUFFER as *const u8, BUFFER_LEN, &raw mut CURRENT_ROW);
            BUFFER_LEN = 0;
            if CURRENT_ROW > 23 {
                CURRENT_ROW = 2;
                vga::clear_screen();
            }
        }
        render_line();
        return;
    }

    let is_shifted = unsafe { SHIFT_PRESSED };
    let is_caps = unsafe { CAPS_LOCK };

    let character = match scancode {
        0x1e => if is_shifted ^ is_caps { b'A' } else { b'a' },
        0x30 => if is_shifted ^ is_caps { b'B' } else { b'b' },
        0x2e => if is_shifted ^ is_caps { b'C' } else { b'c' },
        0x20 => if is_shifted ^ is_caps { b'D' } else { b'd' },
        0x12 => if is_shifted ^ is_caps { b'E' } else { b'e' },
        0x21 => if is_shifted ^ is_caps { b'F' } else { b'f' },
        0x22 => if is_shifted ^ is_caps { b'G' } else { b'g' },
        0x23 => if is_shifted ^ is_caps { b'H' } else { b'h' },
        0x17 => if is_shifted ^ is_caps { b'I' } else { b'i' },
        0x24 => if is_shifted ^ is_caps { b'J' } else { b'j' },
        0x25 => if is_shifted ^ is_caps { b'K' } else { b'k' },
        0x26 => if is_shifted ^ is_caps { b'L' } else { b'l' },
        0x32 => if is_shifted ^ is_caps { b'M' } else { b'm' },
        0x31 => if is_shifted ^ is_caps { b'N' } else { b'n' },
        0x18 => if is_shifted ^ is_caps { b'O' } else { b'o' },
        0x19 => if is_shifted ^ is_caps { b'P' } else { b'p' },
        0x10 => if is_shifted ^ is_caps { b'Q' } else { b'q' },
        0x13 => if is_shifted ^ is_caps { b'R' } else { b'r' },
        0x1f => if is_shifted ^ is_caps { b'S' } else { b's' },
        0x14 => if is_shifted ^ is_caps { b'T' } else { b't' },
        0x16 => if is_shifted ^ is_caps { b'U' } else { b'u' },
        0x2f => if is_shifted ^ is_caps { b'V' } else { b'v' },
        0x11 => if is_shifted ^ is_caps { b'W' } else { b'w' },
        0x2d => if is_shifted ^ is_caps { b'X' } else { b'x' },
        0x15 => if is_shifted ^ is_caps { b'Y' } else { b'y' },
        0x2c => if is_shifted ^ is_caps { b'Z' } else { b'z' },
        0x02 => if is_shifted { b'!' } else { b'1' },
        0x03 => if is_shifted { b'@' } else { b'2' },
        0x04 => if is_shifted { b'#' } else { b'3' },
        0x05 => if is_shifted { b'$' } else { b'4' },
        0x06 => if is_shifted { b'%' } else { b'5' },
        0x07 => if is_shifted { b'^' } else { b'6' },
        0x08 => if is_shifted { b'&' } else { b'7' },
        0x09 => if is_shifted { b'*' } else { b'8' },
        0x0a => if is_shifted { b'(' } else { b'9' },
        0x0b => if is_shifted { b')' } else { b'0' },
        0x0c => if is_shifted { b'_' } else { b'-' },
        0x0d => if is_shifted { b'+' } else { b'=' },
        0x1a => if is_shifted { b'{' } else { b'[' },
        0x1b => if is_shifted { b'}' } else { b']' },
        0x2b => if is_shifted { b'|' } else { b'\\' },
        0x27 => if is_shifted { b':' } else { b';' },
        0x28 => if is_shifted { b'"' } else { b'\'' },
        0x33 => if is_shifted { b'<' } else { b',' },
        0x34 => if is_shifted { b'>' } else { b'.' },
        0x35 => if is_shifted { b'?' } else { b'/' },
        0x39 => b' ',
        _ => return,
    };

    unsafe {
        if BUFFER_LEN < (80 - 12 - 1) { // prompt len 12
            INPUT_BUFFER[BUFFER_LEN] = character;
            BUFFER_LEN += 1;
        }
    }

    render_line();
}
