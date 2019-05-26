#![no_std]

use core::panic::PanicInfo;

use riscv::asm;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    asm::ebreak();

    loop {}
}
