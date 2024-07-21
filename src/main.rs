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

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_milk_tea_os::test_panic_handler(info)
}