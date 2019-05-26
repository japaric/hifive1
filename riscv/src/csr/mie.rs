pub fn read() -> Mie {
    unsafe {
        let r: u32;
        asm!("csrr $0, mie" : "=r"(r) : : : "volatile");
        Mie { value: r as u16 }
    }
}

pub const MSIE: u16 = 1 << 3;
pub const MTIE: u16 = 1 << 7;
pub const MEIE: u16 = 1 << 11;

const MASK: u16 = (1 << 12) - 1;

// NOTE(unsafe/memory) end of critical section
pub unsafe fn write(mie: u16) {
    asm!("csrw mie, $0" : : "r"(mie & MASK) : "memory" : "volatile" )
}

pub fn clear(mask: u16) {
    // NOTE(memory) start of critical section
    unsafe { asm!("csrc mie, $0" : : "r"(mask & MASK) : "memory" : "volatile") }
}

pub fn clear_msie() {
    // NOTE(memory) start of critical section
    unsafe { asm!("csrci mie, 8" : : : "memory" : "volatile") }
}

// NOTE(unsafe/memory) end of critical section
pub unsafe fn set_msie() {
    asm!("csrsi mie, 8" : : : "memory" : "volatile")
}

pub fn clear_mtie() {
    // NOTE(memory) start of critical section
    unsafe { asm!("csrc mie, $0" : : "r"(MTIE) : "memory" : "volatile") }
}

// NOTE(unsafe/memory) end of critical section
pub unsafe fn set(mask: u16) {
    asm!("csrs mie, $0" : : "r"(mask & MASK) : "memory" : "volatile")
}

// NOTE(unsafe/memory) end of critical section
pub unsafe fn set_mtie() {
    asm!("csrs mie, $0" : : "r"(MTIE) : "memory" : "volatile")
}

pub fn clear_meie() {
    // NOTE(memory) start of critical section
    unsafe { asm!("csrc mie, $0" : : "r"(MEIE) : "memory" : "volatile") }
}

// NOTE(unsafe/memory) end of critical section
pub unsafe fn set_meie() {
    asm!("csrs mie, $0" : : "r"(MEIE) : "memory" : "volatile")
}

#[derive(Clone, Copy)]
pub struct Mie {
    value: u16,
}

impl Mie {
    pub fn get_msie(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    pub fn get_mtie(&self) -> bool {
        self.value & (1 << 7) != 0
    }

    pub fn get_meie(&self) -> bool {
        self.value & (1 << 11) != 0
    }

    pub fn bits(self) -> u16 {
        self.value
    }
}

impl ufmt::uDebug for Mie {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite,
    {
        f.debug_struct("Mie")?
            .field("msie", &if self.get_msie() { 1 } else { 0 })?
            .field("mtie", &if self.get_mtie() { 1 } else { 0 })?
            .field("meie", &if self.get_meie() { 1 } else { 0 })?
            .finish()
    }
}
