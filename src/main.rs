#![allow(deprecated)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rsos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;
mod serial;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rsos::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rsos::init();

    #[cfg(test)]
    test_main();

    println!("Welcome to RSOS");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}

