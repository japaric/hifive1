//! A software interrupt can preempt itself
//!
//! Expected output:
//!
//! ```
//! [msi] before
//! [msi] before
//! [msi] after
//! [msi] after
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;

#[no_mangle]
unsafe extern "C" fn main() {
    // only enable software interrupt
    csr::mie::write(csr::mie::MSIE);

    // trigger software interrupt
    hifive1::msip::set_msi();

    // unmask interrupts
    csr::mstatus::set_mie();

    loop {}
}

#[no_mangle]
unsafe extern "C" fn msi() {
    static ONCE: AtomicBool = AtomicBool::new(false);

    hifive1::msip::clear_msi();

    if let Ok(w) = hio::hstdout() {
        w.write(b"[msi] before\n").ok();

        if ONCE
            .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            // trigger software interrupt
            hifive1::msip::set_msi();

            // unmask interrupts
            csr::mstatus::set_mie();
        }

        w.write(b"[msi] after\n").ok();
    }
}
