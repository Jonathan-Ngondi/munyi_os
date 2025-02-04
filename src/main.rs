#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;
mod vga_buffer;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Munyi os! \n A small OS to help with Operating System learning");


    loop{}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println! ("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
