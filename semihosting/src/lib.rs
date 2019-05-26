#![feature(asm)]
#![no_std]

#[macro_use]
mod macros;
pub mod hio;
pub mod nr;

pub unsafe fn syscall<T>(nr: usize, arg: &T) -> usize {
    syscall1(nr, arg as *const T as usize)
}

// https://github.com/riscv/riscv-openocd/pull/266/files#diff-6476a9d02543f742bc66fc75ed583159R93
pub fn syscall1(mut nr: usize, arg: usize) -> usize {
    // NOTE(arguments) x10 = a0; x11 = a1
    // NOTE(`.option norvc`) `ebreak` must be left uncompressed
    unsafe {
        asm!("
.option norvc
slli zero,zero,0x1f
ebreak
srai zero,zero,0x7
": "+{x10}"(nr) : "{x11}"(arg) : : "volatile");
    }
    nr
}
