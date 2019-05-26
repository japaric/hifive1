MEMORY
{
  QSPI : ORIGIN = 0x20000000, LENGTH = 512M
  DTIM : ORIGIN = 0x80000000, LENGTH = 16K
}

_stack_top = ORIGIN(DTIM) + LENGTH(DTIM);

ENTRY(_start);

PROVIDE(exception = default_handler);
PROVIDE(msi = default_handler);
PROVIDE(mti = default_handler);

/* device-specific interrupts */
PROVIDE(WATCHDOG = default_handler);
PROVIDE(RTC = default_handler);

PROVIDE(UART0 = default_handler);
PROVIDE(UART1 = default_handler);

PROVIDE(QSPI0 = default_handler);
PROVIDE(QSPI1 = default_handler);
PROVIDE(QSPI2 = default_handler);

PROVIDE(GPIO0 = default_handler);
PROVIDE(GPIO1 = default_handler);
PROVIDE(GPIO2 = default_handler);
PROVIDE(GPIO3 = default_handler);
PROVIDE(GPIO4 = default_handler);
PROVIDE(GPIO5 = default_handler);
PROVIDE(GPIO6 = default_handler);
PROVIDE(GPIO7 = default_handler);
PROVIDE(GPIO8 = default_handler);
PROVIDE(GPIO9 = default_handler);
PROVIDE(GPIO10 = default_handler);
PROVIDE(GPIO11 = default_handler);
PROVIDE(GPIO12 = default_handler);
PROVIDE(GPIO13 = default_handler);
PROVIDE(GPIO14 = default_handler);
PROVIDE(GPIO15 = default_handler);
PROVIDE(GPIO16 = default_handler);
PROVIDE(GPIO17 = default_handler);
PROVIDE(GPIO18 = default_handler);
PROVIDE(GPIO19 = default_handler);
PROVIDE(GPIO20 = default_handler);
PROVIDE(GPIO21 = default_handler);
PROVIDE(GPIO22 = default_handler);
PROVIDE(GPIO23 = default_handler);
PROVIDE(GPIO24 = default_handler);
PROVIDE(GPIO25 = default_handler);
PROVIDE(GPIO26 = default_handler);
PROVIDE(GPIO27 = default_handler);
PROVIDE(GPIO28 = default_handler);
PROVIDE(GPIO29 = default_handler);
PROVIDE(GPIO30 = default_handler);
PROVIDE(GPIO31 = default_handler);

PROVIDE(PWM0CMP0 = default_handler);
PROVIDE(PWM0CMP1 = default_handler);
PROVIDE(PWM0CMP2 = default_handler);
PROVIDE(PWM0CMP3 = default_handler);
PROVIDE(PWM1CMP0 = default_handler);
PROVIDE(PWM1CMP1 = default_handler);
PROVIDE(PWM1CMP2 = default_handler);
PROVIDE(PWM1CMP3 = default_handler);
PROVIDE(PWM2CMP0 = default_handler);
PROVIDE(PWM2CMP1 = default_handler);
PROVIDE(PWM2CMP2 = default_handler);
PROVIDE(PWM2CMP3 = default_handler);

SECTIONS
{
  .text : ALIGN(4)
  {
    *(.text._start);
    *(.text .text.*);
  } > QSPI

  .rodata :
  {
    *(.rodata .rodata.*);
  } > QSPI

  .bss : ALIGN(4)
  {
    *(.bss .bss.*);

    . = ALIGN(4);
  } > DTIM

  _sbss = ADDR(.bss);
  _ebss = ADDR(.bss) + SIZEOF(.bss);

  .data : ALIGN(4)
  {
    *(.data .data.*);

    . = ALIGN(4);
  } > DTIM AT > QSPI

  /* VMA of .data */
  _sdata = ADDR(.data);
  _edata = ADDR(.data) + SIZEOF(.data);

  /* LMA of .data */
  _sidata = LOADADDR(.data);

}
