use core::mem::{size_of, transmute};
use core;

use common::x86::reg;
use util::ptr::mut_offset;
use kernel::heap;

define_flags!(GdtAccess: u8 {
    ACCESSED = 1 << 0,
    EXTEND   = 1 << 1,
    CONFORM  = 1 << 2,
    CODE     = 1 << 3,
    STORAGE  = 1 << 4, // not TSS
    DPL0 = 0 << 5,
    DPL1 = 1 << 5,
    DPL2 = 2 << 5,
    DPL3 = 3 << 5,
    PRESENT  = 1 << 7,

    CODE_READ = (1 << 3) | (1 << 1),
    TSS       = 0b1001
})

define_flags!(GdtFlags: u8 {
    SIZE_32  = 1 << 6,
    PAGES    = 1 << 7
})

pub static DATA_WRITE: GdtAccess = EXTEND;

pub type GdtReg = reg::DtReg<GdtEntry>;

#[packed]
pub struct GdtEntry {
    priv limit_lo: u16,
    priv base_lo: u16,
    priv base_hl: u8,
    priv access: u8, // TODO: use GdtAccess here
    priv limit_hi_flags: u8,
    priv base_hh: u8
}

impl GdtEntry {
    pub fn new(base: u32, limit: u32, access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        let (base_hh, base_hl, base_lo) = (
            (base  & 0xFF000000) >> 24,
            (base  & 0x__FF0000) >> 16,
            (base  & 0x____FFFF)
        );
        let (limit_hi, limit_lo) = (
            (limit & 0x___F0000) >> 16,
            (limit & 0x____FFFF)
        );
        GdtEntry {
            limit_lo: limit_lo as u16,
            base_lo:  base_lo as u16,
            base_hl: base_hl as u8,
            base_hh:  base_hh as u8,
            access: access.get(),
            limit_hi_flags: flags.get() | limit_hi as u8
        }
    }

    pub fn seg<T>(data: *mut T, access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        // assert!(size_of::<T>() < (1u << 20));
        GdtEntry::new(data as u32, size_of::<T>() as u32, access, flags)
    }

    pub fn flat(access: GdtAccess, flags: GdtFlags) -> GdtEntry {
        GdtEntry::new(0, 0xFFFFF, access, flags | PAGES)
    }
}

pub struct Gdt {
    priv reg: *GdtReg,
    priv table: *mut GdtEntry
}

impl Gdt {
    pub fn new() -> Gdt {
        unsafe {
            let table_ptr = heap::zero_alloc::<GdtEntry>(256);
            let reg_ptr: *mut GdtReg = heap::alloc(1);

            let reg: &mut GdtReg = transmute(reg_ptr);
            *reg = reg::DtReg::new(table_ptr, 256);

            Gdt { reg: transmute(reg_ptr), table: table_ptr }
        }
    }

    pub fn enable(&self, n: uint, mut entry: GdtEntry) {
        unsafe {
            entry.access |= PRESENT.get();
            *mut_offset(self.table, n as int) = entry;
        }
    }

    pub unsafe fn disable(&self, n: uint) {
        (*mut_offset(self.table, n as int)).access &= !PRESENT.get();
    }

    pub fn load(&self, code: u16, data: u16, local: u16) {
        unsafe {
            (*self.reg).load();
            asm!("mov ds, $0
                  mov ss, $0
                  mov fs, $1
                  mov gs, $1"
                :: "r"(data), "r"(local)
                :: "volatile", "intel");
            asm!("jmp $0, $$.flush; .flush:" :: "Ir"(code) :: "volatile")
        }
    }
}

impl reg::DtReg<GdtEntry> {
    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lgdt [$0]" :: "r"(self) :: "intel");
        }
    }
}
