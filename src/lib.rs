#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(async_fn_in_trait)]
#![feature(global_asm)]

mod kernel;
pub mod std;

extern crate alloc;


pub fn init() {
	kernel::gdt::init();
	kernel::interrupts::init_idt();
	unsafe { kernel::interrupts::PICS.lock().initialize() };
	x86_64::instructions::interrupts::enable();
}