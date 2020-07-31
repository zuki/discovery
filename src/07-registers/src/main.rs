#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let gpiod = aux7::init().1;

    // Turn on the "North" LED (orange)
    gpiod.bsrr.write(|w| w.bs13().set_bit());

    // Turn on the "East" LED (red)
    gpiod.bsrr.write(|w| w.bs14().set_bit());

    // Turn off the "North" LED
    gpiod.bsrr.write(|w| w.br13().set_bit());

    // Turn off the "East" LED
    gpiod.bsrr.write(|w| w.br14().set_bit());

    loop {}
}
