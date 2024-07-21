#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_milk_tea_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_milk_tea_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_milk_tea_os::init();

    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow(); // 每一次递归都会将返回地址入栈
    // }
    // // 触发 stack overflow
    // stack_overflow();

    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }

    // let ptr = 0x2031b2 as *mut u8;
    // // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");
    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    // loop {}

    // loop {
    //     use rust_milk_tea_os::print;
    //     print!("-");        // new
    //     for _ in 0..10000 {}
    // }

    println!("It did not crash!");
    rust_milk_tea_os::hlt_loop(); 
}

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