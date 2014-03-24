use core::mem::{size_of, transmute};

use cpu::DtReg;
use util::ptr::mut_offset;
use kernel::heap;

pub static SIZE_32: u16 = 1 << 14;
pub static PAGES:   u16 = 1 << 15;
pub static ACCESSED:   u16 = 1 << 0;
pub static EXTEND:     u16 = 1 << 1;
pub static CONFORM:    u16 = 1 << 2;
pub static CODE:       u16 = 1 << 3;
pub static STORAGE:    u16 = 1 << 4;
pub static PRESENT:    u8  = 1 << 7;
pub static CODE_READ:  u16 = CODE | EXTEND;
pub static DATA_WRITE: u16 = EXTEND;

pub type GdtReg = DtReg<GdtEntry>;

#[packed]
pub struct GdtEntry {
    priv limit_lo: u16,
    priv base_lo: u16,
    priv base_hl: u8,
    priv access: u8,
    priv limit_hi_flags: u8,
    priv base_hh: u8
}

impl GdtEntry {
    pub fn flat(access: u16, dpl: u8) -> GdtEntry {
        GdtEntry {
            limit_lo: 0xFFFF,
            base_lo:  0,
            base_hl: 0,
            base_hh:  0,
            access: access as u8,
            limit_hi_flags: 0x0F | ((access >> 8) & 0xF0) as u8
        }
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
            *reg = DtReg::new(table_ptr, 256);

            Gdt { reg: transmute(reg_ptr), table: table_ptr }
        }
    }

    pub fn enable(&self, n: uint, mut entry: GdtEntry) {
        unsafe {
            entry.access |= PRESENT;
            *mut_offset(self.table, n as int) = entry;
            // (*self.table)[n].access |= PRESENT;
        }
    }

    pub unsafe fn disable(&self, n: uint) {
        (*mut_offset(self.table, n as int)).access &= !PRESENT;
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

impl super::DtReg<GdtEntry> {
    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lgdt [$0]" :: "r"(self) :: "intel");
        }
    }
}
