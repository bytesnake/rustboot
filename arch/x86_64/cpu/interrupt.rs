use core::mem::{size_of, transmute};
use core::ptr::offset;

use cpu::DtReg;
use cpu::exception::Fault;
use cpu::idt::{IdtEntry, IdtReg, INTR_GATE, PRESENT};
// use drivers::pic;
use util::ptr::mut_offset;
use kernel::heap;

pub enum Int {
    Fault(Fault)
}

pub struct Table {
    priv reg: &'static IdtReg,
    priv table: *mut IdtEntry,
    priv mask: u16,
}

impl Table {
    pub fn new() -> Table {
        unsafe {
            let table = heap::zero_alloc::<IdtEntry>(256);
            let reg = heap::alloc::<IdtReg>(1);
            *(reg as *mut IdtReg) = DtReg::new(table, 256);
            Table {
                reg: transmute(reg),
                table: table,
                mask: 0xffff
            }
        }
    }

    pub unsafe fn enable_maskable(&mut self, irq: uint, isr: extern "C" unsafe fn()) {
        *mut_offset(self.table, irq as int) = IdtEntry::new(
            isr,                // interrupt service routine
            1 << 3,             // segment selector
            INTR_GATE | PRESENT // flags
        );

        self.mask &= !(1u16 << (irq & 0b1111));
        // pic::mask(self.mask);
    }

    pub unsafe fn set_isr(&mut self, val: Fault, code: bool, handler: extern "C" unsafe fn()) {
        *mut_offset(self.table, val as int) = Isr::new(Fault(val), code).idt_entry(handler);
    }

    pub fn load(&self) {
        self.reg.load();
        // pic::remap();
        // pic::mask(self.mask);
        enable();
    }
}

fn enable() {
    unsafe {
        asm!("sti" :::: "volatile", "intel");
    }
}

#[packed]
pub struct Isr {
    priv push_dummy: u8, // push eax  // (only for exceptions without error codes)
    priv push: u8,       // push byte <imm>  // save int. number
    priv value: Int,
    priv jmp: u8,        // jmp rel  // jump to the common handler
    priv rel: i32
}

impl Isr {
    pub fn new(val: Int, code: bool) -> &mut Isr {
        let this: &mut Isr = unsafe { transmute(heap::alloc::<Isr>(1)) };
        *this = Isr {
            push_dummy: if code { 0x90 } else { 0x50 },   // [9]
            push: 0x6a, value: val,
            jmp: 0xe9, rel: -5
        };
        this
    }

    pub unsafe fn idt_entry(&mut self, handler: extern "C" unsafe fn()) -> IdtEntry {
        self.rel = handler as i32 - offset(transmute::<&Isr, *Isr>(self), 1) as i32;
        IdtEntry::new(transmute(self), 1 << 3, INTR_GATE | PRESENT)
    }
}
