#![deny(unsafe_code)]
#![no_main]
#![no_std]

// _embedded_hal_blocking_delay_DelayMsはdelay_ms()に必要
use aux5::{entry, _embedded_hal_blocking_delay_DelayMs, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let ms = 50_u16;

    // infinite loop; just so we don't leave this stack frame
    loop {
        for curr in 0..4 {
            let next = (curr + 1) % 4;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    /*
        leds[LedColor::Orange].on();
        delay.delay_ms(ms);
        leds[LedColor::Red].on();
        delay.delay_ms(ms);
        leds[LedColor::Blue].on();
        delay.delay_ms(ms);
        leds[LedColor::Orange].off();
        leds[LedColor::Green].on();
        delay.delay_ms(ms);
        leds[LedColor::Red].off();
        delay.delay_ms(ms);
        leds[LedColor::Blue].off();
        delay.delay_ms(ms);
        leds[LedColor::Green].off();
    */
    }
}
