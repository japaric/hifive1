pub fn read() -> usize {
    unsafe {
        let r: usize;
        asm!("csrr $0, mepc" : "=r"(r) : : : "volatile");
        r
    }
}
