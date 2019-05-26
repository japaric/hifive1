# `hifive1`

> [Prototype] Real Time For the Masses on the HiFive1

**IMPORTANT** Do **not** use any of the crates in this repository. If you want
to use Rust on RISC-V check out the crates under the [riscv-rust] and
[rust-embedded] organizations.

[riscv-rust]: https://github.com/riscv-rust/
[rust-embedded]: https://github.com/rust-embedded/

## Examples

To run the examples you'll need to install:

- [`riscv-openocd`](https://github.com/riscv/riscv-openocd), an OpenOCD with
  RISC-V support that ships with the `board/sifive-hifive1.cfg` file.

- [`riscv-unknown-elf-gdb`], a GDB with RISCV support. If you want to use a GDB
  with a different name / prefix then you'll need to change the name in
  `.cargo/config`.

Then, on a console run OpenOCD

``` console
$ cd rtfm

$ riscv-openocd
Open On-Chip Debugger 0.10.0+dev-00614-g998fed1fe (2019-05-24-16:26)
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
adapter speed: 10000 kHz
Info : auto-selecting first available session transport "jtag". To override use 'transport select <transport>'.
Info : ftdi: if you experience problems at higher adapter clocks, try the command "ftdi_tdo_sample_edge falling"
Info : clock speed 10000 kHz
Info : JTAG tap: riscv.cpu tap/device found: 0x10e31913 (mfg: 0x489 (SiFive, Inc.), part: 0x0e31, ver: 0x1)
Info : [0] Found 2 triggers
halted at 0x200004e6 due to debug interrupt
Info : Examined RISCV core; XLEN=32, misa=0x40001105
Info : Listening on port 3333 for gdb connections
Info : Found flash device 'issi is25lp128d' (ID 0x0018609d)
cleared protection for sectors 64 through 255 on flash bank 0
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
```

And on another console:

``` console
$ cargo run --example lock --release
(..)
     Running `riscv-unknown-elf-gdb -x openocd.gdb (..)
```

You'll see on the first console:

``` console
$ riscv-openocd
(..)
A
B - SHARED = 1
C
D - SHARED = 2
E
```

There are non-RTFM in the `quickstart` directory.

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
