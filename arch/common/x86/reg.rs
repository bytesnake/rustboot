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

macro_rules! impl_read_write (
    ($name:ident : $T:ident, $reg:expr) => (
        impl $name {
            #[inline] #[allow(dead_code)]
            pub fn read() -> $T {
                unsafe {
                    let flags;
                    asm!(concat!("mov $0, ", $reg) : "=r"(flags) ::: "intel");
                    $T(flags)
                }
            }

            #[inline] #[allow(dead_code)]
            pub fn write(f: $T) {
                match f {
                    $T(val) => unsafe {
                        asm!(concat!("mov ", $reg, ", $0") :: "r"(val) :: "volatile", "intel");
                    }
                }
            }
        }
    );
)

define_reg!(CR0, CR0Flags: uint {
    CR0_PG = 1 << 31
})

impl_read_write!(CR0: CR0Flags, "cr0")

pub struct CR2;
impl_read_write!(CR2: Page, "cr2")

pub struct CR3;
impl_read_write!(CR3: Page, "cr3")

// http://www.jaist.ac.jp/iscenter-new/mpc/altix/altixdata/opt/intel/vtune/doc/users_guide/mergedProjects/analyzer_ec/mergedProjects/reference_olh/mergedProjects/instructions/instruct32_hh/vc178.htm

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
