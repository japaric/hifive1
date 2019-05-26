pub const DIG0: u32 = 1 << 16;
pub const DIG1: u32 = 1 << 17;
pub const DIG2: u32 = 1 << 18;
pub const DIG3: u32 = 1 << 19;
pub const DIG4: u32 = 1 << 20;
pub const DIG5: u32 = 1 << 21;
pub const DIG6: u32 = 1 << 22;
pub const DIG7: u32 = 1 << 23;
pub const DIG8: u32 = 1 << 0;
pub const DIG9: u32 = 1 << 1;
pub const DIG10: u32 = 1 << 2;
pub const DIG11: u32 = 1 << 3;
pub const DIG12: u32 = 1 << 4;
pub const DIG13: u32 = 1 << 5;
pub const DIG15: u32 = 1 << 9;
pub const DIG16: u32 = 1 << 10;
pub const DIG17: u32 = 1 << 11;
pub const DIG18: u32 = 1 << 12;
pub const DIG19: u32 = 1 << 13;

pub const TX: u32 = DIG0;
pub const RX: u32 = DIG1;

pub const BLUE: u32 = DIG5;
pub const GREEN: u32 = DIG3;
pub const RED: u32 = DIG6;

const BASE_ADDRESS: usize = 0x10012000;

// const VALUE: usize = 0x00;
const INPUT_EN: usize = 0x04;
const OUTPUT_EN: usize = 0x08;
const PORT: usize = 0x0c;
const PUE: usize = 0x10;
// const DS: usize = 0x14;
const RISE_IE: usize = 0x18;
const RISE_IP: usize = 0x1c;
const FALL_IE: usize = 0x20;
const FALL_IP: usize = 0x24;
// const HIGH_IE: usize = 0x28;
const HIGH_IP: usize = 0x2c;
// const LOW_IE: usize = 0x30;
const LOW_IP: usize = 0x34;
// const IOF_EN: usize = 0x38;
// const IOF_SEL: usize = 0x3c;
const OUT_XOR: usize = 0x40;

pub fn set_input_en(val: u32) {
    unsafe { ((BASE_ADDRESS + INPUT_EN) as *mut u32).write_volatile(val) }
}

pub fn set_output_en(val: u32) {
    unsafe { ((BASE_ADDRESS + OUTPUT_EN) as *mut u32).write_volatile(val) }
}

pub fn set_port(val: u32) {
    unsafe { ((BASE_ADDRESS + PORT) as *mut u32).write_volatile(val) }
}

pub fn set_pue(val: u32) {
    unsafe { ((BASE_ADDRESS + PUE) as *mut u32).write_volatile(val) }
}

pub fn set_rise_ie(val: u32) {
    unsafe { ((BASE_ADDRESS + RISE_IE) as *mut u32).write_volatile(val) }
}

pub fn set_rise_ip(val: u32) {
    unsafe { ((BASE_ADDRESS + RISE_IP) as *mut u32).write_volatile(val) }
}

pub fn set_fall_ie(val: u32) {
    unsafe { ((BASE_ADDRESS + FALL_IE) as *mut u32).write_volatile(val) }
}

pub fn set_fall_ip(val: u32) {
    unsafe { ((BASE_ADDRESS + FALL_IP) as *mut u32).write_volatile(val) }
}

pub fn set_high_ip(val: u32) {
    unsafe { ((BASE_ADDRESS + HIGH_IP) as *mut u32).write_volatile(val) }
}

pub fn set_low_ip(val: u32) {
    unsafe { ((BASE_ADDRESS + LOW_IP) as *mut u32).write_volatile(val) }
}

pub fn set_out_xor(val: u32) {
    unsafe { ((BASE_ADDRESS + OUT_XOR) as *mut u32).write_volatile(val) }
}
