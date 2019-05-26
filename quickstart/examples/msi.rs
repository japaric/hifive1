//! Software interrupt

#![deny(warnings)]
#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;
use ufmt::uwriteln;
use ufmt_utils::{consts, Ignore, LineBuffered};

#[no_mangle]
unsafe extern "C" fn main() {
    if let Ok(mut w) = hio::hstdout()
        .map(Ignore::new)
        .map(LineBuffered::<_, consts::U100>::new)
    {
        uwriteln!(&mut w, "{:?}", csr::mstatus::read()).ok();

        // only enable software interrupt
        csr::mie::write(csr::mie::MSIE);

        // trigger software interrupt
        hifive1::msip::set_msi();

        // unmask interrupts
        csr::mstatus::set_mie();

        let mstatus = csr::mstatus::read();
        assert!(mstatus.get_mie());
        uwriteln!(&mut w, "{:?}", mstatus).ok();
    }

    loop {}
}

#[no_mangle]
unsafe extern "C" fn msi() {
    // clear the software interrupt
    hifive1::msip::clear_msi();

    if let Ok(mut w) = hio::hstdout()
        .map(Ignore::new)
        .map(LineBuffered::<_, consts::U100>::new)
    {
        uwriteln!(&mut w, "[msi] {:?}", csr::mstatus::read()).ok();
    }
}
