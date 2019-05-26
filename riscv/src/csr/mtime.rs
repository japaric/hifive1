// NOTE in assembly because this is time critical
// NOTE memory mapped CSR; address is device specific and must be provided by the linker script
pub fn read() -> u64 {
    extern "C" {
        static _mtime: usize;
    }

    let l: u32;
    let h1: u32;
    let _h2: u32;
    unsafe {
        asm!("
1:
  lw  $0, 4($3)
  lw  $1, 0($3)
  lw  $2, 4($3)
  bne $0, $2, 1b
" : "=&r"(h1) "=&r"(l) "=&r"(_h2) : "r"(_mtime) : : "volatile");
    }
    (u64::from(h1) << 32) | u64::from(l)
}
