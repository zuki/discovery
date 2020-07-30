//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
use panic_halt as _; // panic handler

use stm32f407g_disc as board;

use crate::board::{
    hal::stm32,
    hal::prelude::*,
};

pub use crate::board::{
    hal::{delay::Delay, prelude::_embedded_hal_blocking_delay_DelayMs},
    led::{LedColor, Leds},
};

use cortex_m::peripheral::Peripherals;
pub use cortex_m_rt::entry;

pub fn init() -> (Delay, Leds) {
    let cp = Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    let gpiod = p.GPIOD.split();
    // ボード上のUser LEDを初期化
    let leds = Leds::new(gpiod);
    // クロックレジスタをconstrain
    let rcc = p.RCC.constrain();
    // クロックを最大周波数168MHzに設定してフリーズする
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();
    // delayプロバイダを取得
    let delay = Delay::new(cp.SYST, clocks);

    (delay, leds)
}
