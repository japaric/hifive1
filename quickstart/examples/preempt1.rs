//! The software interrupt can preempt the timer interrupt
//!
//! Expected output:
//!
//! ```
//! [main] before
//! [mti] before
//! [msi]
//! [mti] after
//! [main] after
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use hifive1::msip;
use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    if let Ok(w) = hio::hstdout() {
        csr::mtimecmp::write(csr::mtime::read() + 1024);

        // enable timer and software interrupts
        csr::mie::write(csr::mie::MTIE | csr::mie::MSIE);

        w.write(b"[main] before\n").ok();

        // enable interrupts
        csr::mstatus::set_mie();

        w.write(b"[main] after\n").ok();
    }

    loop {}
}

// lower priority
#[no_mangle]
unsafe extern "C" fn mti() {
    if let Ok(w) = hio::hstdout() {
        w.write(b"[mti] before\n").ok();

        csr::mie::clear_mtie();

        // trigger software interrupt
        msip::set_msi();

        // enable interrupts
        csr::mstatus::set_mie();

        w.write(b"[mti] after\n").ok();
    }
}

// higher priority
#[no_mangle]
extern "C" fn msi() {
    msip::clear_msi();

    if let Ok(w) = hio::hstdout() {
        w.write(b"[msi]\n").ok();
    }
}
