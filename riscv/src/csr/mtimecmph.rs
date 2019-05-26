// NOTE memory mapped CSR; address is device specific and must be provided by the linker script
pub fn write(cmp: u32) {
    extern "C" {
        static _mtimecmp: usize;
    }

    unsafe { (_mtimecmp as *mut u32).add(1).write_volatile(cmp) }
}
