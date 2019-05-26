pub fn read() -> usize {
    unsafe {
        let r: usize;
        asm!("csrr $0, mtvec" : "=r"(r) : : : "volatile");
        r
    }
}

pub fn write(mtvec: usize, vectored: bool) {
    unsafe {
        asm!("csrw mtvec, $0" : : "r"(mtvec & !0b11 + if vectored { 1 } else { 0 }) : : "volatile")
    }
}
