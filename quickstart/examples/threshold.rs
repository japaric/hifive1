//! Priority threshold
//!
//! NOTE: DIG2 must be connected to DIG3, and DIG4 must be connected to DIG5
//!
//! ```
//! [main] before
//! [DIG2] before
//! [DIG2] after
//! [DIG4]
//! [main] after
//! ```

#![deny(warnings)]
#![no_main]
#![no_std]

use hifive1::{
    gpio::{self, DIG2, DIG3, DIG4, DIG5},
    plic,
};
use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;

#[no_mangle]
unsafe extern "C" fn main() {
    if let Ok(w) = hio::hstdout() {
        // try to reset registers because software resets don't
        gpio::set_input_en(0);
        gpio::set_output_en(0);
        gpio::set_rise_ie(0);
        gpio::set_out_xor(0);
        gpio::set_pue(0);

        // discard all pending interrupts
        while plic::claim() != 0 {}

        // DIG3 = output low
        gpio::set_port(0);
        gpio::set_output_en(DIG3 | DIG5);

        // DIG2 = floating input
        gpio::set_input_en(DIG2 | DIG4);
        gpio::set_rise_ip(!0); // clear any pending interrupt
        gpio::set_rise_ie(DIG2 | DIG4);

        // enable DIG1 interrupt and set priority to 1
        plic::set_priority(plic::DIG2, 1);
        plic::set_priority(plic::DIG4, 2);
        plic::enable(plic::DIG2);
        plic::enable(plic::DIG4);

        // set priority threshold to 0
        plic::set_threshold(0);

        // enable only external interrupts
        csr::mie::write(csr::mie::MEIE);

        // unmask interrupts
        csr::mstatus::set_mie();

        w.write(b"[main] before\n").ok();

        // trigger DIG4 interrupt
        gpio::set_port(DIG5);

        w.write(b"[main] after\n").ok();
    }

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn GPIO18() {
    plic::set_threshold(1);
    csr::mstatus::set_mie();

    // clear interrupt flag
    gpio::set_rise_ip(DIG2);

    if let Ok(w) = hio::hstdout() {
        w.write(b"[DIG2]\n").ok();
    }

    csr::mstatus::clear_mie();
    plic::set_threshold(0);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn GPIO20() {
    let t = plic::get_threshold();
    plic::set_threshold(2);
    csr::mstatus::set_mie();

    // clear interrupt flag
    gpio::set_rise_ip(DIG4);

    if let Ok(w) = hio::hstdout() {
        w.write(b"[DIG4] before\n").ok();

        // trigger DIG2 interrupt
        // this is lower priority so it will be dispatched after this handler returns
        gpio::set_port(DIG3 | DIG5);

        w.write(b"[DIG4] after\n").ok();
    }

    csr::mstatus::clear_mie();
    plic::set_threshold(t);
}
