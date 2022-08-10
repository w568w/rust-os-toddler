#![feature(once_cell)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod pixel_buffer;
mod interrupts;

use core::borrow::Borrow;
use core::fmt::Write;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use crate::pixel_buffer::global_impl::MaybeUninitializedLogger;
use crate::pixel_buffer::logger::Logger;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(frame_buffer) = boot_info.framebuffer.as_mut() {
        let info = frame_buffer.info();
        MaybeUninitializedLogger::init(frame_buffer.buffer_mut(), info);
        start();
    }
    loop {}
}

// This entry point is invoked when all features are initialized.
fn start() {
    println!("Hello World");
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();
}