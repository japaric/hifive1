pub fn read() -> Mip {
    unsafe {
        let r: u32;
        asm!("csrr $0, mip" : "=r"(r) : : : "volatile");
        Mip { value: r as u16 }
    }
}

pub struct Mip {
    value: u16,
}

impl Mip {
    pub fn get_msip(&self) -> bool {
        self.value & (1 << 3) != 0
    }

    pub fn get_mtip(&self) -> bool {
        self.value & (1 << 7) != 0
    }

    pub fn get_meip(&self) -> bool {
        self.value & (1 << 11) != 0
    }
}

impl ufmt::uDebug for Mip {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite,
    {
        f.debug_struct("Mip")?
            .field("msip", &if self.get_msip() { 1 } else { 0 })?
            .field("mtip", &if self.get_mtip() { 1 } else { 0 })?
            .field("meip", &if self.get_meip() { 1 } else { 0 })?
            .finish()
    }
}
