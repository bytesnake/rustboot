use core::mem::size_of;
use core;

mod gdt;
mod idt;
pub mod interrupt;
mod exception;
pub mod mmu;
pub mod io;

define_flags!(Eflags: u64 {
    CF,
    IF = 1 << 9
})

impl Eflags {
    #[inline]
    fn read() -> Eflags {
        unsafe {
            let flags;
            asm!("pushf; pop $0;" : "=r"(flags) ::: "volatile")
            Eflags(flags)
        }
    }
}

define_flags!(CR0Flags: u64 {
    CR0_PG = 1 << 31
})

// Extended Feature Enable Register
define_flags!(EferFlags: u64 {
    EFER_SCE = 1 << 0,
    EFER_LME = 1 << 8, // long mode enable
    EFER_LMA = 1 << 10
})

struct Efer;

impl Efer {
    #[inline]
    fn read() -> EferFlags {
        unsafe {
            let flags;
            asm!("rdmsr" : "=A"(flags) : "{ecx}"(0xC0000080) :: "intel");
            EferFlags(flags)
        }
    }

    #[inline]
    fn write(f: EferFlags) {
        match f {
            EferFlags(val) => unsafe {
                asm!("wrmsr" :: "{ecx}"(0xC0000080), "A"(val) :: "volatile", "intel");
            }
        }
    }
}

impl core::ops::BitOr<EferFlags, EferFlags> for Efer {
    #[inline(always)]
    fn bitor(&self, other: &EferFlags) -> EferFlags {
        match (Efer::read(), other) {
            (EferFlags(flags1), &EferFlags(flags2)) => EferFlags(flags1 | flags2)
        }
    }
}


#[packed]
struct DtReg<T> {
    size: u16,
    addr: *mut T,
}

impl<T> DtReg<T> {
    pub fn new(descriptor_table: *mut T, capacity: uint) -> DtReg<T> {
        DtReg {
            size: (capacity * size_of::<T>() - 1) as u16,
            addr: descriptor_table,
        }
    }
}

pub fn init() {
    Efer::write(Efer | EFER_LME);
}
