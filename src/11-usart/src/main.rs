#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart, _itm) = aux11::init();

    // Send a single character
    usart.dr.write(|w| w.dr().bits(u16::from(b'X')));

    loop {}
}
