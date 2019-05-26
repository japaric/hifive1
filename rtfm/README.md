# Real Time For the Masses (RTFM)

Single core RTFM on the HiFive1.

This port implements the base features (`spawn` / `#[task]` / `lock`) plus the
`schedule` and `#[interrupt]` extensions.

Some notable differences between this port and the ARM Cortex-M port.

## Background

The HiFive1 has three main non-device-specific interrupts, which are common to
all RISC-V cores: the software interrupt (MSI), the timer interrupt (MTI) and
the external interrupt (MEI).

The external interrupt (MEI) multiplexes the ~50 device specific interrupts the
HiFive1 has. These ~50 interrupts will be dispatched by the same external
interrupt handler and can be prioritized by the PLIC (Platform-Level
Interrupt Controller); the HiFive1 supports 7 priority levels and it has a
`threshold` register that behaves like the BASEPRI register on Cortex-M cores.

The software (MSI) and timer (MTI) interrupt sit outside the "PLIC" so they
don't have priorities like the device specific interrupts have and are not
affected by the `threshold` register. However, the three main interrupts can be
individually masked and these masks can be used to implement software-level
prioritization between them.

## Differences

The HiFive1 doesn't have a register to "pend" a device specific interrupt (cf.
NVIC_ISPR). There's a read-only register that returns the pending state of an
interrupt but doesn't let you change its state to pending. For this reason I
didn't implement software tasks on top of device-specific interrupts like in the
Cortex-M port; in this port all software tasks are dispatched by the software
interrupt. This imposes a limitation to the user: all software tasks must run at
the same priority.

For simplicity, I also enforce that software tasks are given lower priority than
hardware tasks. This prioritization is implemented by having all hardware tasks
run with the software interrupt (MSI) masked. I don't think this is necessary,
though; it should be possible to assign all software tasks an arbitrary priority
such that some hardware tasks have lower priority than them and some other
higher priority; however, this would make the implementation trickier to
implement.

The Cortex-M port uses two timers to implement the `schedule` API: a monotonic
timer (configurable) and another timer to produce timeouts (always the
`SysTick`, the system timer). The HiFive1 port uses a single 64-bit timer which
must be available on all RISC-V implementations and it's interfaced through two
registers (`mtime` and `mtimecmp`). On the HiFive1 chip this timer is clocked at
32,768 Hz so the resulting `schedule` API is also coarse grained.
