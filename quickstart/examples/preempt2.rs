//! The timer interrupt can preempt the software interrupt
//!
//! Expected output
//!
//! ``` text
//! [main] before
//! [msi] before
//! [mti]
//! [msi] after
//! [main] after
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    if let Ok(w) = hio::hstdout() {
        w.write(b"[main] before\n").ok();

        // enable only software interrupt
        csr::mie::write(csr::mie::MSIE);

        // trigger software interrupt
        hifive1::msip::set_msi();

        // unmask interrupts
        csr::mstatus::set_mie();

        w.write(b"[main] after\n").ok();
    }

    loop {}
}

#[no_mangle]
unsafe extern "C" fn msi() {
    hifive1::msip::clear_msi();

    if let Ok(w) = hio::hstdout() {
        w.write(b"[msi] before\n").ok();

        // immediately timeout
        csr::mtimecmp::write(csr::mtime::read());

        // enable timer interrupt
        csr::mie::set_mtie();

        // unmask interrupts
        csr::mstatus::set_mie();

        w.write(b"[msi] after\n").ok();
    }
}

#[no_mangle]
unsafe extern "C" fn mti() {
    if let Ok(w) = hio::hstdout() {
        w.write(b"[mti]\n").ok();

        // disable timer interrupt
        csr::mie::clear_mtie();
    }
}
