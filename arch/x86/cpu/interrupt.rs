use core::mem::transmute;
use core::ptr::offset;

use common::x86::reg;
use cpu::Context;
use cpu::exception::Fault;
use cpu::idt::{IdtEntry, IdtReg, INTR_GATE, PRESENT};
use platform::drivers::pic;
use util::ptr::mut_offset;
use kernel::heap;
use kernel::syscall;

// TODO
// enum Trap {
//     Syscall = 0x80
// }


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
            *(reg as *mut IdtReg) = reg::DtReg::new(table, 256);
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
        pic::mask(self.mask);
    }

    #[allow(visible_private_types)]
    pub unsafe fn set_isr(&mut self, val: Fault, code: bool, handler: extern unsafe fn()) {
        *mut_offset(self.table, val as int) = Isr::new(Fault(val), code).idt_entry(handler);
    }

    pub fn set_intr_gate(&mut self, val: uint, isr: &mut Isr) {
        unsafe {
            *mut_offset(self.table, val as int) = isr.idt_entry(interrupt_handler());
        }
    }

    pub fn load(&self) {
        self.reg.load();
        pic::remap();
        pic::mask(self.mask);
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

        // unsafe { *this = default_isr };
        // // unsafe {
        // //     match transmute::<Isr, IsrRepr>(default_isr) {
        // //         IsrRepr(v, _) => *transmute::<&Isr, &mut u32>(this) = v
        // //     }
        // // };
        // if code { this.push_dummy = 0x90 }
        // this.value = val;
        // this
    }

    pub unsafe fn idt_entry(&mut self, handler: extern "C" unsafe fn()) -> IdtEntry {
        self.rel = handler as i32 - offset(transmute::<&Isr, *Isr>(self), 1) as i32;
        IdtEntry::new(transmute(self), 1 << 3, INTR_GATE | PRESENT)
    }
}

// static mut handlers: [fn(regs: &Context), ..256] = [
    // dummy_isr_handler, ..256
// ];

#[no_split_stack]
#[inline(never)]
pub unsafe fn interrupt_handler() -> extern "C" unsafe fn() {
    // Points to the data on stack
    asm!("jmp $$.skip
      interrupt_handler_asm:"
        :::: "volatile", "intel");

    let stack_ptr = Context::save();

    let mut args = syscall::args(stack_ptr.eax, stack_ptr.ebx, stack_ptr.ecx, stack_ptr.edx);
    stack_ptr.eax = syscall::handler(&mut args);

    Context::restore();

    asm!("$$.skip:"
        :::: "volatile", "intel");

    extern { fn interrupt_handler_asm(); }
    interrupt_handler_asm
}
