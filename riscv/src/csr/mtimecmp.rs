// NOTE memory mapped CSR; address is device specific and must be provided by the linker script
// NOTE in assembly because this is time critical
pub fn write(cmp: u64) {
    extern "C" {
        static _mtimecmp: usize;
    }

    let low = cmp as u32;
    let high = (cmp >> 32) as u32;
    unsafe {
        asm!("
sw  $0, 0($3)
sw  $1, 4($3)
sw  $2, 0($3)
" : : "r"(-1) "r"(high) "r"(low) "r"(_mtimecmp) : : "volatile")
    }
}

pub fn write_max() {
    extern "C" {
        static _mtimecmp: usize;
    }

    unsafe {
        asm!("
sw  $0, 0($1)
sw  $0, 4($1)
" : : "r"(-1) "r"(_mtimecmp) : : "volatile")
    }
}
