pub mod gdt;
pub mod interrupts;
pub mod timer;

pub fn init() {
    gdt::init();
    timer::init_pit(100); // 100Hz = 10ms
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
