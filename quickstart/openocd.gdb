target extended-remote :3333

# set print asm-demangle on

monitor arm semihosting enable

load

stepi
