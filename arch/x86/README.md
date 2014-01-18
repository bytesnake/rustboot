## x86 platform
### Files
```
├── boot
│   ├── loader.asm  Bootloader
│   └── linker.ld   Linker script
├── cpu
│   ├── mod.rs
│   ├── interrupt.rs
│   ├── exception.rs
│   ├── gdt.rs    Global descriptor table
│   ├── idt.rs    Interrupt descriptor table
│   ├── io.rs     CPU IO: inb, outb
│   └── paging.rs
├── drivers
│   ├── keyboard.rs
│   ├── mod.rs  Initializes drivers
│   ├── pic.rs  Programmable interrupt controller
│   ├── pit.rs  Programmable interval timer 
│   └── vga.rs  Text display
├── io
│   └── mod.rs  Input/output interface
├── Makefile
├── memset.asm  Implementation of memset
└── README.md   this document
```

### Produced files
```
└── boot
    ├── libcore-2e829c2f-0.0.rlib
    ├── linker.map
    └── loader.o
```

### Bootloader: `boot/loader.asm`

Assembly code in this file is used to set up the image in memory.
The directive `use16` marks the beginning of 16-bit code[[1]]. Label `start` is defined to specify an entry point.

BIOS interrupt 0x13 provides disk services. When given parameter ah=2, it reads sectors from drive[[2]].

The `cli` instruction clears the IF flag, masking external interrupts[[3]].

Register `cr0` controls the operation of the processor[[4]]. General purpose register is used to modify its value since most instructions can't access control and segment registers.

After protected mode is enabled, a far jump to 32-bit code is necessary to start executing 32-bit code and simultaneously load 32-bit **segment selector** to CS[[5]][[6]].

| Physical memory ranges | Size     | Description |
| ---------------------- | -------- | ----------- |
| 0x0000 ... 0x7BFF      | 31 KiB   | Stack       |
| 0x7C00 ... 0x7DFF      | 0.5 KiB  | Bootloader  |
| 0x07E00 ... 0x0FFFF    | 32.5 KiB | _unused_    |
| 0x10000 ... 0x1FFFF    | 64 KiB   | Kernel      |

[1]: http://www.nasm.us/doc/nasmdoc6.html#section-6.1.1 "6.1.1 USE16 & USE32: Aliases for BITS"
[2]: http://en.wikipedia.org/wiki/INT_13H#INT_13h_AH.3D02h:_Read_Sectors_From_Drive "INT 13h AH=02h: Read Sectors From Drive"
[3]: http://faydoc.tripod.com/cpu/cli.htm "CLI - Clear Interrupt Flag"
[4]: http://en.wikipedia.org/wiki/Control_register#CR0
[5]: http://www.c-jump.com/CIS77/ASM/Memory/M77_0290_segment_registers_protected.htm "Segment Registers in Protected Mode"
[6]: http://stackoverflow.com/questions/9113310/segment-selector-in-ia-32 "Segment Selector in IA-32"
