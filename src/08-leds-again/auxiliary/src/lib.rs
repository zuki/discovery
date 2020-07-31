//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::bkpt;
pub use cortex_m_rt::entry;
pub use stm32f407g_disc::gpioi;
pub use stm32f407g_disc::hal::stm32::rcc;

use stm32f407g_disc::hal::stm32::{self, GPIOD, RCC};

pub fn init() -> (&'static gpioi::RegisterBlock, &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32::Peripherals::take().unwrap());

    unsafe { (&*GPIOD::ptr(), &*RCC::ptr()) }
}
