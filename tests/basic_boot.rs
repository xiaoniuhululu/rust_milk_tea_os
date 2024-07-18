#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(rust_milk_tea_os::test_runner)]

use core::panic::PanicInfo;
use rust_milk_tea_os::println;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     unimplemented!();
// }

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_milk_tea_os::test_panic_handler(info);
    // loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output111");
}
