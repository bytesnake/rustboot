target remote localhost:1234
symbol-file boot/initram.elf
add-symbol-file boot/kernel.elf 0x00010000 -s .boot 0x00007c00

set disassembly-flavor intel

b debug
c
