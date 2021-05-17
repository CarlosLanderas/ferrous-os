#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxid_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use oxid_os::{println};
use x86_64::{VirtAddr, structures::paging::Translate};

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    
    println!("Hello World{}", "!");
    
    oxid_os::init();

    println!("Physical memory offset: 0x{:x}", boot_info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { oxid_os::memory::init(phys_mem_offset)};
    let addresses = [        
        0xb8000,        
        0x201008,        
        0x0100_0020_1a10,        
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);

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
