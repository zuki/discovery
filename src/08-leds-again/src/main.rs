#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpiod, rcc) = aux8::init();

    // TODO initialize GPIOD
    rcc.ahb1enr.modify(|_, w| {
        w.gpioden().set_bit()
    });

    // Set PD12-PD15 as digital output
    gpiod.moder.modify(|_, w| {
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

    // Turn on all the LEDs in the compass
    gpiod.odr.write(|w| {
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit();
        w.odr12().set_bit()
    });

    aux8::bkpt();

    loop {}
}
