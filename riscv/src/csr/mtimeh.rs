// NOTE memory mapped CSR; address is device specific and must be provided by the linker script
pub fn read() -> u32 {
    extern "C" {
        static _mtime: usize;
    }

    unsafe { (_mtime as *const u32).add(1).read_volatile() }
}
