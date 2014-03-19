use core::mem::size_of;
use core::option::{Option, None, Some};
use core;

use kernel::heap;
use kernel;

mod gdt;
mod idt;
mod tss;
pub mod interrupt;
pub mod io;
mod exception;
pub mod mmu;

// TODO: make push_dummy push ds?
// exception info and processor state saved on stack
struct Context {
    // Registers saved by the ISR (in reverse order)
    edi: u32, esi: u32, ebp: u32, esp: u32, ebx: u32, edx: u32, ecx: u32, eax: u32,
    ds: u32, es: u32, fs: u32, gs: u32,
    int_no: u32,   // added by ISRs
    err_code: u32, // added by some exceptions
    call_stack: IsrCallStack
}

// the cpu adds these when calling the ISR
struct IsrCallStack {
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32
}

impl Context {
    unsafe fn save() -> &mut Context {
        let this: &mut Context;
        asm!("push gs
              push fs
              .byte 0x06 // push es
              .byte 0x1e // push ds
              pusha"
            : "={esp}"(this) ::: "volatile", "intel");
        this
    }

    unsafe fn restore() {
        asm!("popa
              .byte 0x1f // pop ds
              .byte 0x07 // pop es
              pop fs
              pop gs
              add esp, 8
              iretd"
            :::: "volatile", "intel");
    }
}

struct LocalSegment {
    ts: tss::TssEntry,
}

pub static mut desc_table: Option<gdt::Gdt> = None;

pub fn init() {
    use cpu::gdt::{Gdt, GdtEntry, SIZE_32, STORAGE, CODE_READ, DATA_WRITE, DPL3};

    let local_data = unsafe {
        heap::zero_alloc::<LocalSegment>(1)
    };
    let tls = unsafe {
        let seg = heap::zero_alloc::<u32>(32);
        *seg = local_data as u32;
        // *(mut_offset(seg, 12)) = 0; // TODO: record stack bottom later
        seg
    };

    let t = Gdt::new();
    t.enable(1, GdtEntry::flat(STORAGE | CODE_READ, SIZE_32));
    t.enable(2, GdtEntry::flat(STORAGE | DATA_WRITE, SIZE_32));
    t.enable(3, GdtEntry::flat(STORAGE | CODE_READ | DPL3, SIZE_32));
    t.enable(4, GdtEntry::flat(STORAGE | DATA_WRITE | DPL3, SIZE_32));
    t.enable(5, GdtEntry::new(tls as u32, 32 * 4, STORAGE | DPL3, SIZE_32));
    unsafe {
        t.enable(6, (*local_data).ts.gdt_entry());
    }
    t.load(1 << 3, 2 << 3, 5 << 3);

    unsafe {
        desc_table = Some(t);

        kernel::int_table.map(|mut t| {
            use cpu::exception::{Breakpoint, exception_handler};
            t.set_isr(Breakpoint, false, exception_handler());
        });

        mmu::init();
    }
}

pub fn info() {
}
