//! Blinky using the `schedule` API

#![no_main]
#![no_std]

use core::time::Duration;

use hifive1::gpio;
use panic_halt as _;

#[rtfm::app]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        gpio::set_input_en(0);
        gpio::set_output_en(0);
        gpio::set_out_xor(gpio::RED);
        gpio::set_port(gpio::BLUE | gpio::GREEN);
        gpio::set_output_en(gpio::BLUE | gpio::GREEN | gpio::RED);

        c.spawn.foo().ok();
    }

    #[task(schedule = [foo])]
    fn foo(c: foo::Context) {
        static mut STATE: bool = false;

        gpio::set_out_xor(if *STATE { gpio::RED } else { 0 });
        *STATE = !*STATE;

        c.schedule.foo(c.scheduled + Duration::from_secs(1)).ok();
    }
};
