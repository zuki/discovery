#![deny(unsafe_code)]
#![no_main]
#![no_std]

// use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart, _dwt, _itm) = aux11::init();

    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            while usart.sr.read().rxne().bit_is_clear() {}
            let byte = usart.dr.read().dr().bits() as u8;
            if buffer.push(byte).is_err() {
                for byte in b"error: buffer full\n\r" {
                    while usart.sr.read().txe().bit_is_clear() {}
                    usart.dr.write(|w| w.dr().bits(u16::from(*byte)));
                }
                break;
            }
            // byte == \r
            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\r', b'\n']) {
                    while usart.sr.read().txe().bit_is_clear() {}
                    usart.dr.write(|w| w.dr().bits(u16::from(*byte)));
                }
                break;
            }
        }
    }
}
