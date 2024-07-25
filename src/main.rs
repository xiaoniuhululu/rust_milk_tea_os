#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_milk_tea_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_milk_tea_os::println;
use bootloader::{BootInfo, entry_point};
// use x86_64::structures::paging::PageTable;
extern crate alloc;
// use alloc::boxed::Box;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    // use rust_milk_tea_os::memory::active_level_4_table;
    // use x86_64::VirtAddr;
    // use rust_milk_tea_os::memory::translate_addr;
    // use rust_milk_tea_os::memory;
    // use x86_64::{structures::paging::Translate, VirtAddr};
    // use x86_64::{VirtAddr, structures::paging::Page};
    use x86_64::{VirtAddr};
    // use rust_milk_tea_os::memory::BootInfoFrameAllocator;
    use rust_milk_tea_os::allocator;
    use rust_milk_tea_os::memory::{self, BootInfoFrameAllocator};


    println!("Hello World{}", "!");

    rust_milk_tea_os::init();

    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // // 映射未使用的页
    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // 通过新的映射将字符串 `New!`  写到屏幕上。
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        
    //     let phys = mapper.translate_addr(virt); // new: use the `mapper.translate_addr` method
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         // get the physical address from the entry and convert it
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         // print non-empty entries of the level 3 table
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_milk_tea_os::hlt_loop(); 
}

// #[no_mangle]
// // pub extern "C" fn _start() -> ! {
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Hello World{}", "!");

//     rust_milk_tea_os::init();

//     // x86_64::instructions::interrupts::int3();

//     // unsafe {
//     //     *(0xdeadbeef as *mut u8) = 42;
//     // };

//     // fn stack_overflow() {
//     //     stack_overflow(); // 每一次递归都会将返回地址入栈
//     // }
//     // // 触发 stack overflow
//     // stack_overflow();

//     // let ptr = 0xdeadbeaf as *mut u8;
//     // unsafe { *ptr = 42; }

//     // let ptr = 0x2031b2 as *mut u8;
//     // // read from a code page
//     // unsafe { let x = *ptr; }
//     // println!("read worked");
//     // // write to a code page
//     // unsafe { *ptr = 42; }
//     // println!("write worked");

//     use x86_64::registers::control::Cr3;
//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

//     #[cfg(test)]
//     test_main();

//     // loop {}

//     // loop {
//     //     use rust_milk_tea_os::print;
//     //     print!("-");        // new
//     //     for _ in 0..10000 {}
//     // }

//     println!("It did not crash!");
//     rust_milk_tea_os::hlt_loop(); 
// }

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {}
    rust_milk_tea_os::hlt_loop(); 
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_milk_tea_os::test_panic_handler(info)
}