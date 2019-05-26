// NOTE memory mapped CSR; address is device specific and must be provided by the linker script
pub fn read() -> u32 {
    extern "C" {
        static _mtime: usize;
    }

    unsafe { (_mtime as *const u32).read_volatile() }
}

pub fn write(x: u32) {
    extern "C" {
        static _mtime: usize;
    }

    unsafe { (_mtime as *mut u32).write_volatile(x) }
}
