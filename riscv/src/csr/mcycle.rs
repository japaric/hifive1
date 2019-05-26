pub fn read() -> u64 {
    let l: u32;
    let h1: u32;
    let _h2: u32;
    unsafe {
        asm!("
1:
  csrr $0, mcycleh
  csrr $1, mcycle
  csrr $2, mcycleh
  bne $0, $2, 1b
" : "=r"(h1) "=r"(l) "=r"(_h2) : : : "volatile");
    }

    (u64::from(h1) << 32) | u64::from(l)
}
