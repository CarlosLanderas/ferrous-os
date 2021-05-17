#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxid_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use oxid_os::{memory::active_level4_table, println};
use x86_64::VirtAddr;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello World{}", "!");
    
    oxid_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level4_table(phys_mem_offset)};

    for(i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    oxid_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    oxid_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) ->! {
    oxid_os::test_panic_handler(info)
}
