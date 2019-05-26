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
        gpio::set_output_en(gpio::DIG3 | gpio::DIG5);

        // DIG2, DIG4 = floating input
        gpio::set_input_en(gpio::DIG2 | gpio::DIG4);
        gpio::set_rise_ip(!0); // clear any pending interrupt
        gpio::set_rise_ie(gpio::DIG2 | gpio::DIG4);
    }

    #[idle(resources = [SHARED])]
    fn idle(mut c: idle::Context) -> ! {
        if let Ok(w) = hio::hstdout() {
            w.write(b"A\n").ok();

            c.resources.SHARED.lock(|shared| {
                *shared += 1;

                // trigger DIG2 interrupt
                gpio::set_port(gpio::DIG3);

                let mut w = LineBuffered::<_, consts::U100>::new(Ignore::new(&w));
                uwriteln!(&mut w, "B - SHARED = {}", shared).ok();

                // trigger DIG4 interrupt
                gpio::set_port(gpio::DIG5);
            });

            w.write(b"E\n").ok();
        }

        loop {}
    }

    #[interrupt(binds = GPIO18, resources = [SHARED], priority = 1)]
    fn dig2(c: dig2::Context) {
        // clear interrupt flag
        gpio::set_rise_ip(gpio::DIG2);

        *c.resources.SHARED += 1;

        if let Ok(mut w) = hio::hstdout()
            .map(Ignore::new)
            .map(LineBuffered::<_, consts::U100>::new)
        {
            uwriteln!(&mut w, "D - SHARED = {}", c.resources.SHARED).ok();
        }
    }

    #[interrupt(binds = GPIO20, priority = 2)]
    fn dig4(_: dig4::Context) {
        // clear interrupt flag
        gpio::set_rise_ip(gpio::DIG4);

        if let Ok(w) = hio::hstdout() {
            w.write(b"C\n").ok();
        }
    }
};
