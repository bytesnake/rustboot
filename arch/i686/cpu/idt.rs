use core;

use common::x86::reg;

define_flags!(IdtFlags: u8 {
    INTR_GATE = 0b1110,
    TRAP_GATE = 0b1111,
    PRESENT = 1 << 7
})

pub type IdtReg = reg::DtReg<IdtEntry>;

#[packed]
pub struct IdtEntry {
    priv addr_lo: u16,
    priv sel: u16,
    priv zero: u8,
    priv flags: IdtFlags,
    priv addr_hi: u16
}

impl IdtEntry {
    pub fn new(func: extern unsafe fn(), sel: u16, flags: IdtFlags) -> IdtEntry {
        let addr = func as uint;
        let (addr_hi, addr_lo) = (
            (addr & 0xFFFF0000) >> 16,
            (addr & 0x____FFFF)
        );
        IdtEntry {
            addr_lo: addr_lo as u16,
            addr_hi: addr_hi as u16,
            sel: sel,
            zero: 0,
            flags: flags
        }
    }
}

impl reg::DtReg<IdtEntry> {
    #[inline]
    pub fn load(&self) {
        unsafe {
            asm!("lidt [$0]" :: "A"(self) :: "intel");
        }
    }
}
