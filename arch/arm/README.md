## ARM platform
### Files
```
├── aeabi_ldivmod.s Implementation of (u)ldivmod
├── boot
│   ├── loader.s    Bootloader
│   └── linker.ld   Linker script
├── core.bc
├── cpu
│   ├── interrupt.rs   Vector table
│   └── mod.rs
├── drivers
│   └── mod.rs  UART io
├── io
│   └── mod.rs  UART read/write
├── Makefile
└── README.md   this document
```

### Produced files
```
├── aeabi_ldivmod.o
└── boot
    ├── loader.o
    ├── main.s
    ├── main.o
    ├── libcore-2e829c2f-0.0.rlib
    ├── core.bc
    ├── core.s
    ├── core.o
    ├── floppy.elf
    └── floppy.img
```

### Interrupts: `cpu/interrupt.rs`

Exception handlers can be dynamically installed[[1]] into the vector table[[2]].
Interrupts must be unmasked with the `VIC_INT_ENABLE`[[3]] interrupt controller register[[4]].

Enabling interrupts[[5]]

[1]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0056d/Caccfahd.html
[2]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0203j/Cihdidh2.html
[3]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0273a/Cihiicbh.html
[4]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/I1042232.html
[5]: http://balau82.wordpress.com/2012/04/15/arm926-interrupts-in-qemu/ "ARM926 interrupts in QEMU"
