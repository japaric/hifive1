#![no_main]
#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

use panic_halt as _;
use semihosting::hio;

#[rtfm::app]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        c.spawn.foo().ok();
    }

    #[task(spawn = [foo])]
    fn foo(c: foo::Context) {
        static ONCE: AtomicBool = AtomicBool::new(false);

        if let Ok(w) = hio::hstdout() {
            w.write(b"[foo]\n").ok();
        }

        if !ONCE.load(Ordering::Relaxed) {
            ONCE.store(true, Ordering::Relaxed);

            c.spawn.foo().ok();
        }
    }
};
