pub fn clear_msi() {
    unsafe { (0x0200_0000 as *mut u32).write_volatile(0) }
}

pub fn set_msi() {
    unsafe { (0x0200_0000 as *mut u32).write_volatile(1) }
}
