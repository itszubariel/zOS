use crate::kernel::interrupts::outb;
use core::sync::atomic::{ AtomicU64, Ordering };

// Frequency is 1.193182 MHz
const PIT_FREQUENCY: u64 = 1193182;
static TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init_pit(frequency: u64) {
    let divisor = PIT_FREQUENCY / frequency;
    unsafe {
        // Send command to PIT channel 0: binary, rate generator, LSB then MSB
        outb(0x43, 0x36);
        outb(0x40, (divisor & 0xff) as u8);
        outb(0x40, ((divisor >> 8) & 0xff) as u8);
    }
}

pub fn tick() {
    TICKS.fetch_add(1, Ordering::SeqCst);
}

pub fn get_uptime_ticks() -> u64 {
    TICKS.load(Ordering::SeqCst)
}
