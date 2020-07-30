//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

use stm32f407g_disc as board;
use board::hal::prelude::*;
use board::hal::stm32;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;

pub fn init() -> ITM {
    let bp = stm32::Peripherals::take().unwrap();
    let p = cortex_m::Peripherals::take().unwrap();

    let rcc = bp.RCC.constrain();
    rcc.cfgr.sysclk(168.mhz()).freeze();

    p.ITM
}
