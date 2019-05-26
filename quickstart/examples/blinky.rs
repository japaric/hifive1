#![deny(warnings)]
#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use hifive1::gpio::{self, BLUE, GREEN, RED};
use panic_ebreak as _;
use riscv::csr;
use rt as _;

// about one second -- mtime is clocked by the RTC (32_768 Hz)
const PERIOD: u64 = 32_768;

static mut INSTANT: u64 = 0;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    gpio::set_input_en(0);
    gpio::set_output_en(0);
    gpio::set_out_xor(RED);
    gpio::set_port(BLUE | GREEN);
    gpio::set_output_en(BLUE | GREEN | RED);

    let instant = csr::mtime::read() + PERIOD;
    INSTANT = instant;
    csr::mtimecmp::write(instant);

    // enable only timer interrupt
    csr::mie::write(csr::mie::MTIE);

    // unmask interrupts
    csr::mstatus::set_mie();

    loop {}
}

#[no_mangle]
unsafe extern "C" fn mti() {
    static X: AtomicBool = AtomicBool::new(false);

    // blink RED led
    let x = X.load(Ordering::Relaxed);
    X.store(!x, Ordering::Relaxed);
    gpio::set_out_xor(if x { RED } else { 0 });

    // prepare a new interrupt
    let instant = INSTANT + PERIOD;
    INSTANT = instant;
    csr::mtimecmp::write(instant);
}
