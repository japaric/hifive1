#![feature(asm)]
#![deny(warnings)]
#![no_std]

use bare_metal::Nr;

pub mod gpio;
pub mod msip;
pub mod plic;

pub enum Interrupt {
    WATCHDOG,
    RTC,
    UART0,
    UART1,
    QSPI0,
    QSPI1,
    QSPI2,
    GPIO0,
    GPIO1,
    GPIO2,
    GPIO3,
    GPIO4,
    GPIO5,
    GPIO6,
    GPIO7,
    GPIO8,
    GPIO9,
    GPIO10,
    GPIO11,
    GPIO12,
    GPIO13,
    GPIO14,
    GPIO15,
    GPIO16,
    GPIO17,
    GPIO18,
    GPIO19,
    GPIO20,
    GPIO21,
    GPIO22,
    GPIO23,
    GPIO24,
    GPIO25,
    GPIO26,
    GPIO27,
    GPIO28,
    GPIO29,
    GPIO30,
    GPIO31,
    PWM0CMP0,
    PWM0CMP1,
    PWM0CMP2,
    PWM0CMP3,
    PWM1CMP0,
    PWM1CMP1,
    PWM1CMP2,
    PWM1CMP3,
    PWM2CMP0,
    PWM2CMP1,
    PWM2CMP2,
    PWM2CMP3,
}

unsafe impl Nr for Interrupt {
    fn nr(&self) -> u8 {
        use Interrupt::*;

        match *self {
            WATCHDOG => 1,
            RTC => 2,
            UART0 => 3,
            UART1 => 4,
            QSPI0 => 5,
            QSPI1 => 6,
            QSPI2 => 7,
            GPIO0 => 8,
            GPIO1 => 9,
            GPIO2 => 10,
            GPIO3 => 11,
            GPIO4 => 12,
            GPIO5 => 13,
            GPIO6 => 14,
            GPIO7 => 15,
            GPIO8 => 16,
            GPIO9 => 17,
            GPIO10 => 18,
            GPIO11 => 19,
            GPIO12 => 20,
            GPIO13 => 21,
            GPIO14 => 22,
            GPIO15 => 23,
            GPIO16 => 24,
            GPIO17 => 25,
            GPIO18 => 26,
            GPIO19 => 27,
            GPIO20 => 28,
            GPIO21 => 29,
            GPIO22 => 30,
            GPIO23 => 31,
            GPIO24 => 32,
            GPIO25 => 33,
            GPIO26 => 34,
            GPIO27 => 35,
            GPIO28 => 36,
            GPIO29 => 37,
            GPIO30 => 38,
            GPIO31 => 39,
            PWM0CMP0 => 40,
            PWM0CMP1 => 41,
            PWM0CMP2 => 42,
            PWM0CMP3 => 43,
            PWM1CMP0 => 44,
            PWM1CMP1 => 45,
            PWM1CMP2 => 46,
            PWM1CMP3 => 47,
            PWM2CMP0 => 48,
            PWM2CMP1 => 49,
            PWM2CMP2 => 50,
            PWM2CMP3 => 51,
        }
    }
}

extern "C" {
    fn WATCHDOG();
    fn RTC();

    fn UART0();
    fn UART1();

    fn QSPI0();
    fn QSPI1();
    fn QSPI2();

    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn GPIO5();
    fn GPIO6();
    fn GPIO7();
    fn GPIO8();
    fn GPIO9();
    fn GPIO10();
    fn GPIO11();
    fn GPIO12();
    fn GPIO13();
    fn GPIO14();
    fn GPIO15();
    fn GPIO16();
    fn GPIO17();
    fn GPIO18();
    fn GPIO19();
    fn GPIO20();
    fn GPIO21();
    fn GPIO22();
    fn GPIO23();
    fn GPIO24();
    fn GPIO25();
    fn GPIO26();
    fn GPIO27();
    fn GPIO28();
    fn GPIO29();
    fn GPIO30();
    fn GPIO31();

    fn PWM0CMP0();
    fn PWM0CMP1();
    fn PWM0CMP2();
    fn PWM0CMP3();
    fn PWM1CMP0();
    fn PWM1CMP1();
    fn PWM1CMP2();
    fn PWM1CMP3();
    fn PWM2CMP0();
    fn PWM2CMP1();
    fn PWM2CMP2();
    fn PWM2CMP3();
}

#[no_mangle]
static VECTOR_TABLE: [unsafe extern "C" fn(); 51] = [
    WATCHDOG, RTC, UART0, UART1, QSPI0, QSPI1, QSPI2, GPIO0, GPIO1, GPIO2, GPIO3, GPIO4, GPIO5,
    GPIO6, GPIO7, GPIO8, GPIO9, GPIO10, GPIO11, GPIO12, GPIO13, GPIO14, GPIO15, GPIO16, GPIO17,
    GPIO18, GPIO19, GPIO20, GPIO21, GPIO22, GPIO23, GPIO24, GPIO25, GPIO26, GPIO27, GPIO28, GPIO29,
    GPIO30, GPIO31, PWM0CMP0, PWM0CMP1, PWM0CMP2, PWM0CMP3, PWM1CMP0, PWM1CMP1, PWM1CMP2, PWM1CMP3,
    PWM2CMP0, PWM2CMP1, PWM2CMP2, PWM2CMP3,
];

// Memory mapped CSRs
#[no_mangle]
static _mtime: usize = 0x0200bff8;

#[no_mangle]
static _mtimecmp: usize = 0x02004000;
