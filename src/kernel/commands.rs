use crate::kernel::terminal::Terminal;
use crate::kernel::interrupts;
use crate::drivers::pit;

pub fn run(buffer: *const u8, len: usize, terminal: &mut Terminal) {
    if len == 0 {
        return;
    }

    let cmd = unsafe { core::slice::from_raw_parts(buffer, len) };

    if cmd == b"help" {
        terminal.write_str_with_color(b"Available commands:\n", 0x0e); // Yellow
        terminal.write_str(
            b"- help: Show this menu\n- clear: Clear screen\n- echo <msg>: Echo msg\n- version: Show version\n- uptime: Show uptime\n- theme <red/blue>: Change theme\n- reboot: Reboot system\n- sysinfo: Show system info\n"
        );
    } else if cmd == b"clear" {
        terminal.clear();
    } else if cmd == b"kyll" {
        for _ in 0..5 {
            terminal.write_str(b"kyll\n");
        }
    } else if cmd == b"riv" {
        for _ in 0..5 {
            terminal.write_str(b"riv\n");
        }
    } else if cmd.starts_with(b"theme ") {
        let theme = &cmd[6..];
        if theme == b"red" {
            terminal.border_color = 0x0c; // Light Red
            terminal.write_str_with_color(b"Theme changed to red.\n", 0x0c);
        } else if theme == b"blue" {
            terminal.border_color = 0x0b; // Light Cyan (Original Blue)
            terminal.write_str_with_color(b"Theme changed to blue.\n", 0x0b);
        } else {
            terminal.write_str(b"Unknown theme. Use 'red' or 'blue'.\n");
        }
        terminal.render();
    } else if cmd == b"reboot" {
        terminal.write_str_with_color(b"Rebooting...\n", 0x0c); // Red
        unsafe {
            interrupts::outb(0x64, 0xfe);
        }
    } else if cmd == b"sysinfo" {
        terminal.write_str_with_color(b"System: ", 0x0a); // Green
        terminal.write_str(b"zOS (hobbyist)\n");
        terminal.write_str_with_color(b"Arch: ", 0x0a);
        terminal.write_str(b"x86_64\n");
    } else if cmd.starts_with(b"echo ") {
        terminal.write_str(&cmd[5..]);
        terminal.write_str(b"\n");
    } else if cmd == b"version" {
        terminal.write_str_with_color(b"zOS v0.1.1 (alpha)\n", 0x0b); // Light Cyan
    } else if cmd == b"uptime" {
        let uptime = pit::get_uptime_ticks() / 100;
        let mut buf = [0u8; 20];
        let mut i = 0;
        let mut n = uptime;
        if n == 0 {
            buf[i] = b'0';
            i += 1;
        } else {
            while n > 0 {
                buf[i] = ((n % 10) as u8) + b'0';
                n /= 10;
                i += 1;
            }
        }
        buf[0..i].reverse();
        terminal.write_str_with_color(b"Uptime: ", 0x0a);
        terminal.write_str(&buf[0..i]);
        terminal.write_str(b" s\n");
    } else {
        terminal.write_str_with_color(b"Command not found: ", 0x0c);
        terminal.write_str(cmd);
        terminal.write_str(b"\n");
    }
}
