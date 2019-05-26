#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use hifive1::gpio;
use panic_halt as _;
use semihosting::hio;
use ufmt::uwriteln;
use ufmt_utils::{consts, Ignore, LineBuffered};

#[rtfm::app]
const APP: () = {
    static mut SHARED: u32 = 0;

    #[init]
    fn init(_: init::Context) {
        // try to reset registers because software resets don't
        gpio::set_input_en(0);
        gpio::set_output_en(0);
        gpio::set_rise_ie(0);
        gpio::set_out_xor(0);
        gpio::set_pue(0);

        // DIG3, DIG5 = output low
        gpio::set_port(0);
        gpio::set_output_en(gpio::DIG3);

        // DIG2, DIG4 = floating input
        gpio::set_input_en(gpio::DIG2);
        gpio::set_rise_ip(!0); // clear any pending interrupt
        gpio::set_rise_ie(gpio::DIG2);
    }

    #[idle(resources = [SHARED], spawn = [foo])]
    fn idle(c: idle::Context) -> ! {
        if let Ok(w) = hio::hstdout() {
            let mut resources = c.resources;
            let spawn = c.spawn;

            w.write(b"A\n").ok();

            resources.SHARED.lock(|shared| {
                *shared += 1;

                spawn.foo().ok();

                let mut w = LineBuffered::<_, consts::U100>::new(Ignore::new(&w));
                uwriteln!(&mut w, "B - SHARED = {}", shared).ok();

                // trigger DIG2 interrupt
                gpio::set_port(gpio::DIG3);
            });

            w.write(b"E\n").ok();
        }

        loop {}
    }

    #[task(resources = [SHARED], priority = 1)]
    fn foo(c: foo::Context) {
        *c.resources.SHARED += 1;

        if let Ok(mut w) = hio::hstdout()
            .map(Ignore::new)
            .map(LineBuffered::<_, consts::U100>::new)
        {
            uwriteln!(&mut w, "D - SHARED = {}", c.resources.SHARED).ok();
        }
    }

    #[interrupt(binds = GPIO18, priority = 2)]
    fn dig2(_: dig2::Context) {
        // clear interrupt flag
        gpio::set_rise_ip(gpio::DIG2);

        if let Ok(w) = hio::hstdout() {
            w.write(b"C\n").ok();
        }
    }
};
