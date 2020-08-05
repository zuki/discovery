#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart, _itm) = aux11::init();

    // Send a single character
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        while usart.sr.read().txe().bit_is_clear() {}

        usart.dr.write(|w| w.dr().bits(u16::from(*byte)));
    }

    loop {}
}
