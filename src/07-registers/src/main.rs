#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const GPIOD_BSRR: u32 = 0x4002_0C18;

        // Turn on the "North" LED (orange)
        *(GPIOD_BSRR as *mut u32) = 1 << 13;

        // Turn on the "East" LED (red)
        *(GPIOD_BSRR as *mut u32) = 1 << 14;

        // Turn off the "North" LED
        *(GPIOD_BSRR as *mut u32) = 1 << (13 + 16);

        // Turn off the "East" LED
        *(GPIOD_BSRR as *mut u32) = 1 << (14 + 16);
    }

    loop {}
}
