## ARM platform
### Files
```
├── boot
│   ├── aeabi_runtime.s Implementation of support routines
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
└── boot
    ├── loader.o
    ├── aeabi_runtime.o
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

In ARM mode, an undefined opcode is used as a breakpoint to break execution[[7]].

When the exception handler has completed execution, the processor restores the state so that the program can resume. The following instructions are used to leave an exception handler[[8]]:

| Exception | Return instruction |
|-----------|--------------------|
| UNDEF     | `movs pc, lr`      |
| IRQ, FIQ  | `subs pc, lr, #4`  |

### Memory management unit: `cpu/mmu.rs`



[1]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0056d/Caccfahd.html
[2]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0203j/Cihdidh2.html
[3]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0273a/Cihiicbh.html
[4]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0225d/I1042232.html
[5]: http://balau82.wordpress.com/2012/04/15/arm926-interrupts-in-qemu/ "ARM926 interrupts in QEMU"
[7]: http://stackoverflow.com/questions/11345371/how-do-i-set-a-software-breakpoint-on-an-arm-processor "How do I set a software breakpoint on an ARM processor? - Stack Overflow"
[8]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0222b/I3108.html "2.9.1. Exception entry and exit summary"
[6]: https://github.com/torvalds/linux/blob/master/arch/arm/mm/proc-arm926.S#L368 "linux / arch / arm / mm / proc-arm926.S"
http://wiki.osdev.org/ARM_Overview
[6]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0198e/ch03s06s01.html
https://gitorious.org/0xlab-kernel/u-boot/source/ef22b50370f7b6d8deba9e9e64d2cb13c542b647:cpu/arm926ejs/start.S

http://man7.org/linux/man-pages/man2/syscall.2.html#notes "Architecture calling conventions"
http://stackoverflow.com/questions/19285992/in-linux-how-to-do-system-calls-through-gnu-arm-assembly
http://peterdn.com/post/e28098Hello-World!e28099-in-ARM-assembly.aspx
http://www.ethernut.de/en/documents/arm-exceptions.html
http://openocd.sourceforge.net/doc/html/Architecture-and-Core-Commands.html
http://asm.sourceforge.net/syscall.html
http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0203j/Cihdidh2.html "Types of exception"
http://homepages.wmich.edu/~grantner/ece6050/ARM7100vA_3.pdf
