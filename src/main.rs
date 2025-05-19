#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(spacetime_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, string::ToString};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use spacetime_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    module::AccessLevel,
    println,
    spacetime_core::SpacetimeCore,
    task::keyboard::print_keypresses,
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    spacetime_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut spacetime_engine = SpacetimeCore::new();

    let module_id = spacetime_engine.insert_module("System".to_string(), AccessLevel::Admin);
    let keyboard_reducer_id = spacetime_engine
        .insert_reducer(
            module_id,
            "keyboard_reducer".to_string(),
            Box::new(|_ctx| Box::pin(print_keypresses())),
        )
        .unwrap();
    spacetime_engine.call_reducer(module_id, keyboard_reducer_id);

    spacetime_engine.run();

    println!("It did not crash!");
    spacetime_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    spacetime_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    spacetime_os::test_panic_handler(info)
}
