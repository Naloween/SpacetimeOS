pub fn init_pit(frequency_hz: u32) {
    let divisor = 1193180 / frequency_hz;
    unsafe {
        // Command port
        x86_64::instructions::port::Port::new(0x43).write(0x36u8);
        // Channel 0 data port (low byte, then high byte)
        x86_64::instructions::port::Port::new(0x40).write((divisor & 0xFF) as u8);
        x86_64::instructions::port::Port::new(0x40).write((divisor >> 8) as u8);
    }
}
