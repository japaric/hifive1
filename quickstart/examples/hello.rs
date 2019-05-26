#![deny(warnings)]
#![no_main]
#![no_std]

use panic_ebreak as _;
use rt as _;
use semihosting::hio;

#[no_mangle]
extern "C" fn main() -> ! {
    if let Ok(w) = hio::hstdout() {
        w.write(b"Hello, world!\n").ok();
    }

    loop {}
}
