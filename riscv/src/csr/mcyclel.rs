pub fn read() -> u32 {
    unsafe {
        let r: u32;
        asm!("csrr $0, mcycle" : "=r"(r) : : : "volatile");
        r
    }
}
