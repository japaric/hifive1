#![deny(warnings)]
#![no_main]
#![no_std]

use panic_ebreak as _;
use rt as _;

#[no_mangle]
extern "C" fn main() -> ! {
    panic!();
}
