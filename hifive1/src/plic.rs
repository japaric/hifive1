use bare_metal::Nr;

use crate::Interrupt;

// Aliases
pub const DIG2: Interrupt = Interrupt::GPIO18;
pub const DIG3: Interrupt = Interrupt::GPIO19;
pub const DIG4: Interrupt = Interrupt::GPIO20;
pub const DIG5: Interrupt = Interrupt::GPIO21;
pub const DIG6: Interrupt = Interrupt::GPIO22;
pub const DIG7: Interrupt = Interrupt::GPIO23;
pub const DIG8: Interrupt = Interrupt::GPIO0;
pub const DIG9: Interrupt = Interrupt::GPIO1;
pub const DIG10: Interrupt = Interrupt::GPIO2;
pub const DIG11: Interrupt = Interrupt::GPIO3;
pub const DIG12: Interrupt = Interrupt::GPIO4;
pub const DIG13: Interrupt = Interrupt::GPIO5;
pub const DIG15: Interrupt = Interrupt::GPIO9;
pub const DIG16: Interrupt = Interrupt::GPIO10;
pub const DIG17: Interrupt = Interrupt::GPIO11;
pub const DIG18: Interrupt = Interrupt::GPIO12;
pub const DIG19: Interrupt = Interrupt::GPIO13;

const BASE_ADDRESS: usize = 0x0c000000;

const PENDING: usize = 0x1000;
const ENABLE: usize = 0x2000;
const THRESHOLD: usize = 0x20_0000;
const CLAIM: usize = 0x20_0004;

pub unsafe fn set_priority(interrupt: impl Nr, prio: u8) {
    let nr = interrupt.nr();
    assert!(nr >= 1 && nr <= 51);
    assert!(prio < 8);

    asm!("sw $0, 0($1)"
         :
         : "r"(prio) "r"(BASE_ADDRESS + (4 * usize::from(nr)))
         : "memory"
         : "volatile");
}

// NOTE RMW operation
pub unsafe fn enable(interrupt: impl Nr) {
    let nr = interrupt.nr();

    if nr < 32 {
        set_enable1(get_enable1() | (1 << nr))
    } else {
        set_enable2(get_enable2() | (1 << (nr - 32)))
    }
}

pub unsafe fn set_enable1(val: u32) {
    asm!("sw $0, 0($1)" : : "r"(val) "r"(BASE_ADDRESS + ENABLE) : "memory" : "volatile");
}

pub unsafe fn set_enable2(val: u32) {
    asm!("sw $0, 0($1)" : : "r"(val) "r"(BASE_ADDRESS + ENABLE + 4) : "memory" : "volatile");
}

pub fn get_enable1() -> u32 {
    unsafe {
        let r: u32;
        asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + ENABLE) : : "volatile");
        r
    }
}

pub fn get_enable2() -> u32 {
    unsafe {
        let r: u32;
        asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + ENABLE + 4) : : "volatile");
        r
    }
}

pub fn get_pending1() -> u32 {
    unsafe {
        let r: u32;
        asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + PENDING) : : "volatile");
        r
    }
}

pub fn get_pending2() -> u32 {
    unsafe {
        let r: u32;
        asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + PENDING + 4) : : "volatile");
        r
    }
}

pub fn get_threshold() -> u8 {
    unsafe {
        let r: u8;
        asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + THRESHOLD) : : "volatile");
        r
    }
}

pub unsafe fn set_threshold(val: u8) {
    asm!("sw $0, 0($1)" : : "r"(val) "r"(BASE_ADDRESS + THRESHOLD) : "memory" : "volatile");
}

pub fn claim() -> u32 {
    let r: u32;
    unsafe { asm!("lw $0, 0($1)" : "=r"(r) : "r"(BASE_ADDRESS + CLAIM) : : "volatile") }
    r
}

pub fn complete(val: u32) {
    unsafe {
        asm!("sw $0, 0($1)" : : "r"(val) "r"(BASE_ADDRESS + CLAIM) : : "volatile");
    }
}
