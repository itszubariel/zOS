use crate::vga;

pub fn run(buffer: *const u8, len: usize, row: *mut usize) {
    if len == 0 {
        return;
    }

    // Convert raw pointer to slice for safe reading
    let cmd = unsafe { core::slice::from_raw_parts(buffer, len) };

    if cmd == b"help" {
        unsafe {
            vga::print_at(b"Available commands:", *row + 1, 0, 0x0f);
            vga::print_at(b"- help: Show this menu", *row + 2, 0, 0x07);
            vga::print_at(b"- clear: Clear screen", *row + 3, 0, 0x07);
            vga::print_at(b"- echo <msg>: Echo msg", *row + 4, 0, 0x07);
            vga::print_at(b"- version: Show version", *row + 5, 0, 0x07);
            *row += 6;
        }
    } else if cmd == b"clear" {
        vga::clear_screen();
        unsafe {
            *row = 2;
        } // Reset prompt row
    } else if cmd.starts_with(b"echo ") {
        unsafe {
            vga::print_at(&cmd[5..], *row + 1, 0, 0x0f);
            *row += 2;
        }
    } else if cmd == b"version" {
        unsafe {
            vga::print_at(b"zOS v0.1.0 (alpha)", *row + 1, 0, 0x0f);
            *row += 2;
        }
    } else {
        unsafe {
            vga::print_at(b"Command not found", *row + 1, 0, 0x04);
            *row += 2;
        }
    }
}
