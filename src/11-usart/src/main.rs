#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            while self.usart1.sr.read().txe().bit_is_clear() {}

            self.usart1.dr.write(|w| w.dr().bits(u16::from(byte)));
        }
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart, _dwt, _itm) = aux11::init();

    let mut serial = SerialPort { usart1: usart };

    uprintln!(serial, "The anser is {}", 40 + 2);

    loop {}
}
