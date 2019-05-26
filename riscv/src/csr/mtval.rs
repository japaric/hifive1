pub fn read() -> usize {
    unsafe {
        let r: usize;
        asm!("csrr $0, mtval" : "=r"(r) : : : "volatile");
        r
    }
}
