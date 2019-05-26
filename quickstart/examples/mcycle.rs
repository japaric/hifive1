#![deny(warnings)]
#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use core::sync::atomic::{self, AtomicU32, Ordering};

use hifive1 as _; // _mtime & _timecmp
use panic_ebreak as _;
use riscv::csr;
use rt as _;
use semihosting::hio;
use ufmt::uwriteln;
use ufmt_utils::{consts, Ignore, LineBuffered};

// about one second with the default clock configuration
const SHIFT: usize = 17;
const PERIOD: u64 = 1 << SHIFT;

static X: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let mcycle = csr::mcyclel::read();
    let mtime = csr::mtime::read();
    atomic::compiler_fence(Ordering::SeqCst);
    csr::mtimecmp::write(mtime + PERIOD);
    X.store(mcycle, Ordering::Relaxed);

    // enable timer interrupt
    csr::mie::set_mtie();

    // enable interrupts
    csr::mstatus::set_mie();

    loop {}
}

#[no_mangle]
unsafe extern "C" fn mti() {
    let y = csr::mcyclel::read();
    atomic::compiler_fence(Ordering::SeqCst);
    csr::mie::clear_mtie();
    let x = X.load(Ordering::Relaxed);

    if let Ok(mut w) = hio::hstdout()
        .map(Ignore::new)
        .map(LineBuffered::<_, consts::U100>::new)
    {
        // ratio between `mcycle` and `mtime` = 545
        uwriteln!(&mut w, "ratio={}", ((y - x) >> SHIFT) as u16).ok();
    }
}
