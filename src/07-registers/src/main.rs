#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    iprintln!(&mut itm.stim[0], "itm test");

    unsafe {
        // A magic address!
        const GPIOD_BSRR: u32 = 0x4002_0C18;
        const GPIOD_ODR:  u32 = 0x4002_0C14;

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOD_ODR as *const u16)
        );

        // Turn on the "North" LED (orange)
        ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 13);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOD_ODR as *const u16)
        );

        // Turn on the "East" LED (red)
        ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << 14);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOD_ODR as *const u16)
        );

        // Turn off the "North" LED
        ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (13 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOD_ODR as *const u16)
        );

        // Turn off the "East" LED
        ptr::write_volatile(GPIOD_BSRR as *mut u32, 1 << (14 + 16));
    }

    loop {}
}
