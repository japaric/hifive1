pub fn read() -> Mcause {
    unsafe {
        let r: u32;
        asm!("csrr $0, mcause" : "=r"(r) : : : "volatile");
        Mcause { value: r }
    }
}

pub struct Mcause {
    value: u32,
}

impl Mcause {
    pub fn get_exception_code(&self) -> u8 {
        self.value as u8
    }

    pub fn get_interrupt(&self) -> bool {
        self.value & (1 << 31) != 0
    }
}

impl ufmt::uDebug for Mcause {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite,
    {
        f.debug_struct("Mcause")?
            .field("exception_code", &self.get_exception_code())?
            .field("interrupt", &self.get_interrupt())?
            .finish()
    }
}
