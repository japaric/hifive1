#![deny(warnings)]
#![no_main]
#![no_std]

use hifive1::gpio::{self, BLUE, GREEN, RED};
use panic_ebreak as _;
use rt as _;

#[no_mangle]
unsafe extern "C" fn main() {
    gpio::set_output_en(BLUE | GREEN | RED);

    // NOTE `1` turns off the LED
    gpio::set_out_xor(0);
    gpio::set_port(
        // BLUE |
        GREEN |
        // RED |
        0,
    );

    loop {}
}
