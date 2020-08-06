#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart, dwt, mut itm) = aux11::init();

    let mut lap_times = [0u32; 2];
    let mut sw = dwt.stopwatch(&mut lap_times);

    // Send a single character
    for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
        while usart.sr.read().txe().bit_is_clear() {}

        usart.dr.write(|w| w.dr().bits(u16::from(*byte)));
    }

    sw.lap();
    if let Some(lap_time) = sw.lap_time(1) {
        iprintln!(
            &mut itm.stim[0],
            "for loop took {} ticks ({} us)",
            lap_time.as_ticks(),
            lap_time.as_micros()
        );
    }

    loop {}
}
