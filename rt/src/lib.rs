#![deny(warnings)]
#![feature(global_asm)]
#![feature(proc_macro_hygiene)]
#![no_std]

use riscv::{asm, csr};
#[cfg(debug_assertions)]
use semihosting::hio;
#[cfg(debug_assertions)]
use ufmt::uwriteln;
#[cfg(debug_assertions)]
use ufmt_utils::{consts, Ignore, LineBuffered};

// NOTE `csrci   mstatus, 8` = disables interrupts -- the debugger may not disable them on reset
// NOTE `x1` = return address = "link register"
// NOTE `x8` = frame pointer
// `abort` uses UNIMP to generate an illegal instruction
global_asm!(
    r#"
  .global _start
  .section .text._start
_start:
  csrci mstatus, 8
  lui   sp, %hi(_stack_top)
  li    x1, 0
  li    x8, 0
  j     start

  .global abort
  .section .text.abort
abort:
  unimp
"#
);

#[no_mangle]
unsafe fn start() -> ! {
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;
        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;
        static tvec: u32;
    }

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    let mtvec = &tvec as *const u32 as usize;
    debug_assert_eq!(mtvec % 4, 0);
    // NOTE(false) The hifive1 doesn't support vectored mode
    csr::mtvec::write(mtvec, false);

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

// Registers that we must preserve:
//
// - (1) ra (return address)
// - (8) a0, a1, a2, a3, a4, a5, a6, a7 (arguments)
// - (7) t0, t1, t2, t3, t4, t5, t6 (temporaries)
//
// the functions we call from here (callees) will preserve these registers
//
// - (11) s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11
//
// NOTE(.align 4) this function must be 4-byte aligned
// NOTE ABI requires that SP is 16-byte aligned on function entry
// NOTE callees are allowed to re-enable interrupts but they must disable them before returning
global_asm!(
    r#"
  .global tvec
  .section .text.tvec
  .align 4
tvec:
  addi sp, sp, -64

  sw   ra, 60(sp)
  sw   a0, 56(sp)
  sw   a1, 52(sp)
  sw   a2, 48(sp)
  sw   a3, 44(sp)
  sw   a4, 40(sp)
  sw   a5, 36(sp)
  sw   a6, 32(sp)
  sw   a7, 28(sp)
  sw   t0, 24(sp)
  sw   t1, 20(sp)
  sw   t2, 16(sp)
  sw   t3, 12(sp)
  sw   t4,  8(sp)
  sw   t5,  4(sp)
  sw   t6,  0(sp)

  csrr a0, mcause
  addi a1, zero, -1
  bge  a1, a0, 1f

  jal  exception
  j    4f

1:
  andi a0, a0, 15
  addi a1, zero, 3
  beq  a0, a1, 2f

  addi a1, zero, 11
  beq  a0, a1, 3f

  jal  mti
  j    4f

2:
  jal  msi
  j    4f

3:
  addi sp, sp, -16

  sw   s1, 12(sp)
  sw   s2,  8(sp)

  lui  s1, 49664
  addi s1, s1, 4
  lw   s2, 0(s1)

  lui  a0, %hi(VECTOR_TABLE)
  addi a0, a0, %lo(VECTOR_TABLE)
  slli a1, s2, 2
  add  a0, a0, a1
  lw   a0, -4(a0)
  jalr a0

  sw   s2, 0(s1)

  lw   s2,  8(sp)
  lw   s1, 12(sp)

  addi sp, sp, 16

4:
  lw   t6,  0(sp)
  lw   t5,  4(sp)
  lw   t4,  8(sp)
  lw   t3, 12(sp)
  lw   t2, 16(sp)
  lw   t1, 20(sp)
  lw   t0, 24(sp)
  lw   a7, 28(sp)
  lw   a6, 32(sp)
  lw   a5, 36(sp)
  lw   a4, 40(sp)
  lw   a3, 44(sp)
  lw   a2, 48(sp)
  lw   a1, 52(sp)
  lw   a0, 56(sp)
  lw   ra, 60(sp)

  addi sp, sp, 64

  mret
"#
);

#[cfg(debug_assertions)]
#[no_mangle]
fn exception(code: u8) {
    if let Ok(mut w) = hio::hstdout()
        .map(Ignore::new)
        .map(LineBuffered::<_, consts::U32>::new)
    {
        uwriteln!(
            &mut w,
            "exception {:?} @ {:?} (mtval={:?})",
            code,
            csr::mepc::read() as *const u8,
            csr::mtval::read() as *const u8,
        )
        .ok();
    }

    loop {}
}

#[no_mangle]
fn default_handler() {
    asm::ebreak();
}
