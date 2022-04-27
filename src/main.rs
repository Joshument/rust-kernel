#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod vga_buffer;
mod tests;
mod qemu;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("{} {} {} {}", "woo", 7 * 3, 5 / 3, core::f32::consts::PI);

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

