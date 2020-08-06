//! Initialization code

// USART2を使用
// tx: PA2
// rx: PA3

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::{ITM, DWT}};
pub use cortex_m_rt::entry;
pub use stm32f4xx_hal::{
    prelude,
    serial::{config::Config, Serial},
    stm32::usart1,
    dwt::Dwt,
};

use stm32f4xx_hal::{
    prelude::*,
    stm32::{self, USART2},
    dwt::{DwtExt},
};

pub fn init() -> (&'static mut usart1::RegisterBlock, Dwt, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    let dwt = cp.DWT.constrain(cp.DCB, clocks);

    let gpioa = dp.GPIOA.split();

    let tx = gpioa.pa2.into_alternate_af7();
    let rx = gpioa.pa3.into_alternate_af7();

    Serial::usart2(
        dp.USART2,
        (tx, rx),
        Config::default().baudrate(115_200.bps()),
        clocks,
    ).unwrap();

    unsafe {
        (
            &mut *(USART2::ptr() as *mut _),
            dwt,
            cp.ITM,
        )
    }
}
