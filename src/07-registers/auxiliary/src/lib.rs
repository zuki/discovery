//! Initialization code

#![deny(warnings)]
#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
use stm32f407g_disc::{
    hal::{
        prelude::*,
        stm32::{self, GPIOD},
    },
    gpioi,
    led::Leds,
};

#[inline(never)]
pub fn init() -> (ITM, &'static gpioi::RegisterBlock) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let gpiod = dp.GPIOD.split();
    // ボード上のUser LEDを初期化
    Leds::new(gpiod);
    // クロックレジスタをconstrain
    let rcc = dp.RCC.constrain();
    rcc.cfgr.sysclk(168.mhz()).freeze();

    (cp.ITM, unsafe { &*GPIOD::ptr() })
}
