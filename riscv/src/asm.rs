pub fn ebreak() {
    unsafe { asm!("ebreak" : : : : "volatile") }
}

pub fn nop() {
    unsafe { asm!("nop" : : : : "volatile") }
}

pub fn wfi() {
    unsafe { asm!("wfi" : : : : "volatile") }
}

// User-mode only? These generate an illegal instruction exception
pub fn rdcycle() -> u32 {
    unsafe {
        let r: u32;
        asm!("rdcycle $0" : "=r"(r) : : : "volatile");
        r
    }
}

pub fn rdcycleh() -> u32 {
    unsafe {
        let r: u32;
        asm!("rdcycleh $0" : "=r"(r) : : : "volatile");
        r
    }
}

pub fn rdtime() -> u32 {
    unsafe {
        let r: u32;
        asm!("rdtime $0" : "=r"(r) : : : "volatile");
        r
    }
}

pub fn rdtimeh() -> u32 {
    unsafe {
        let r: u32;
        asm!("rdtimeh $0" : "=r"(r) : : : "volatile");
        r
    }
}
