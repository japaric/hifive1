pub fn read() -> Mstatus {
    unsafe {
        let r: u32;
        asm!("csrr $0, mstatus" : "=r"(r) : : : "volatile");
        Mstatus { value: r as u16 }
    }
}

pub fn clear_mie() {
    // NOTE("memory") needs to be a compiler fence because it's used to create critical sections
    unsafe { asm!("csrci mstatus, 8" : : : "memory" : "volatile") }
}

pub unsafe fn set_mie() {
    // NOTE("memory") see above
    asm!("csrsi mstatus, 8" : : : "memory" : "volatile")
}

pub struct Mstatus {
    value: u16,
}

impl Mstatus {
    pub fn get_mie(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    pub fn get_mpie(&self) -> bool {
        self.value & (1 << 7) != 0
    }

    pub fn get_mpp(&self) -> u8 {
        ((self.value >> 11) & 0b11) as u8
    }
}

impl ufmt::uDebug for Mstatus {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite,
    {
        f.debug_struct("Mstatus")?
            .field("mie", &if self.get_mie() { 1 } else { 0 })?
            .field("mpie", &if self.get_mpie() { 1 } else { 0 })?
            .field("mpp", &self.get_mpp())?
            .finish()
    }
}
