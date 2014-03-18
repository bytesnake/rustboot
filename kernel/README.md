## kernel
### Files
```
kernel/
├── elf.rs  ELF
├── int.rs  Integer
├── memory
│   ├── allocator.rs    Buddy memory allocator
│   ├── mod.rs
│   └── virtual.rs
├── mod.rs      Kernel
├── ptr.rs      Pointer (mut_offset)
├── README.md   this document
└── rt.rs       Runtime
```

### Memory allocator: `memory/allocator.rs`

The [buddy memory allocation[1]][1] system is implemented with the use of a binary tree.

### ELF loader: `elf/mod.rs`

To load an ELF file,

1: http://en.wikipedia.org/wiki/Buddy_memory_allocation

http://stackoverflow.com/questions/6554825/how-do-i-load-and-execute-an-elf-binary-executable-manually "How do I load and execute an ELF binary executable manually? - Stack Overflow"
http://shell-storm.org/blog/Physical-page-frame-allocation-with-bitmap-algorithms/ ""
http://wiki.osdev.org/Page_Frame_Allocation#Physical_Memory_Allocators
http://articles.manugarg.com/aboutelfauxiliaryvectors.html
http://asm.sourceforge.net/articles/startup.html
