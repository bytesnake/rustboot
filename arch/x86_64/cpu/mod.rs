use core::mem::size_of;
use core;

mod gdt;
mod idt;
pub mod interrupt;
mod exception;
pub mod mmu;
pub mod io;

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

pub fn init() {
    Efer::write(Efer | EFER_LME);
}
