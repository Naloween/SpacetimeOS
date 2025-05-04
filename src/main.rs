#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(spacetime_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spacetime_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    println,
    task::{
        Task,
        executor::{Executor, Spawner},
        keyboard,
    },
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

    let mut executor = Executor::new();
    let mut task_spawner = Spawner::new(&executor);
    task_spawner.spawn(Task::new(example_task(task_spawner.clone())));
    task_spawner.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

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

async fn example_task(mut spawner: Spawner) {
    println!("Spawning a new task inside a task...");
    spawner.spawn(Task::new(example_task2()));
}

async fn example_task2() {
    println!("Example task print");
}
