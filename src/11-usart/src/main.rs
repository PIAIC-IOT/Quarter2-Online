#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        aux11::bkpt();
    }
}



// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, mut itm) = aux11::init();

//     let instant = mono_timer.now();
//     // Send a string
//     for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
//         // wait until it's safe to write to TDR
//         while usart1.isr.read().txe().bit_is_clear() {} // <- NEW!

//         usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//     }
//     let elapsed = instant.elapsed(); // in ticks

//     iprintln!(
//         &mut itm.stim[0],
//         "`for` loop took {} ticks ({} us)",
//         elapsed,
//         elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
//     );

//     loop {}
// }






// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, mut itm) = aux11::init();

//     let instant = mono_timer.now();
//     // Send a string
//     for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
//         usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//     }
//     let elapsed = instant.elapsed(); // in ticks

//     iprintln!(
//         &mut itm.stim[0],
//         "`for` loop took {} ticks ({} us)",
//         elapsed,
//         elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
//     );

//     loop {}
// }



// ----------------------- second ----------------
// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, itm) = aux11::init();

//     // Send a string
//     for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
//         usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
//     }

//     loop {}
// }












// ----------------- First ---------------------------
// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux11::{entry, iprint, iprintln};

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, itm) = aux11::init();

//     // Send a single character
//     usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

//     loop {}
// }




