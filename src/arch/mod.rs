pub mod x86_64;

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    x86_64::init();
}
