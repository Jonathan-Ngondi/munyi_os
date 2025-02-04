#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Munyi os! \n A small OS to help with Operating System learning");

    #[cfg(test)]
    test_main();

    loop{}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1,1);
    println!("[ok]");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println! ("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
