use core::mem::{size_of, transmute};
use core;

use cpu::mmu::Page;

define_flags!(Eflags: uint {
    CF,
    IF = 1 << 9
})

impl Eflags {
    fn read() -> Eflags {
        unsafe {
            let flags;
            asm!("pushf; pop $0;" : "=r"(flags) ::: "volatile")
            Eflags(flags)
        }
    }
}

define_flags!(CR0Flags: uint {
    CR0_PG = 1 << 31
})

pub struct CR0;

// http://www.jaist.ac.jp/iscenter-new/mpc/altix/altixdata/opt/intel/vtune/doc/users_guide/mergedProjects/analyzer_ec/mergedProjects/reference_olh/mergedProjects/instructions/instruct32_hh/vc178.htm
impl CR0 {
    #[inline]
    pub fn read() -> CR0Flags {
        unsafe {
            let flags;
            asm!("mov $0, cr0" : "=r"(flags) ::: "intel");
            CR0Flags(flags)
        }
    }

    #[inline]
    pub fn write(f: CR0Flags) {
        match f {
            CR0Flags(val) => unsafe {
                asm!("mov cr0, $0" :: "r"(val) :: "volatile", "intel");
            }
        }
    }
}

impl core::ops::BitOr<CR0Flags, CR0Flags> for CR0 {
    #[inline(always)]
    fn bitor(&self, other: &CR0Flags) -> CR0Flags {
        match (CR0::read(), other) {
            (CR0Flags(flags1), &CR0Flags(flags2)) => CR0Flags(flags1 | flags2)
        }
    }
}

pub struct CR3;

// http://www.jaist.ac.jp/iscenter-new/mpc/altix/altixdata/opt/intel/vtune/doc/users_guide/mergedProjects/analyzer_ec/mergedProjects/reference_olh/mergedProjects/instructions/instruct32_hh/vc178.htm
impl CR3 {
    #[inline]
    pub fn read() -> Page {
        unsafe {
            let ptr: uint;
            asm!("mov $0, cr3" : "=r"(ptr) ::: "intel");
            transmute(ptr)
        }
    }

    #[inline]
    pub fn write(ptr: Page) {
        unsafe {
            let ptr: uint = transmute(ptr);
            asm!("mov cr3, $0" :: "r"(ptr) :: "volatile", "intel");
        }
    }
}

// Any of descriptor table (IDT, GDT) registers
#[packed]
pub struct DtReg<T> {
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
