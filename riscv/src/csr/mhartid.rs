pub fn read() -> u32 {
    unsafe {
        let r: u32;
        asm!("csrr $0, mhartid" : "=r"(r) : : : "volatile");
        r
    }
}
