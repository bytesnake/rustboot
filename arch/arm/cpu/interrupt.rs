use core::mem::{volatile_store, transmute};
use core::ptr::offset;
use core;

use kernel;
use platform::io;

struct PICRegisters {
    irq_status    : IntSource, // IRQ status register
    fiq_status    : IntSource, // FIQ status register
    raw_intr      : IntSource, // Raw interrupt status register
    int_select    : IntSource, // Interrupt select register
    int_enable    : IntSource, // Interrupt enable register
    int_en_clear  : IntSource, // Interrupt enable clear register
    soft_int      : IntSource, // Software interrupt register
    soft_int_clear: IntSource, // Software interrupt clear register
    protection    : u32,       // Protection enable register
    // ...
}

define_flags!(IntSource: u32 {
    UART0 = 1 << 12,
    UART1 = 1 << 13,
    UART2 = 1 << 14
})

static PIC: *mut PICRegisters = 0x10140000 as *mut PICRegisters;

static VT: *u32 = 0 as *u32;

#[repr(u8)]
pub enum Int {
    Reset = 0,
    Undef,
    SWI, // software interrupt
    PrefetchAbort,
    DataAbort,
    IRQ = 6,
    FIQ
}

fn set_word(vector: u8, instruction: u32) {
    unsafe {
        volatile_store(offset(VT, vector as int) as *mut u32, instruction);
    }
}

fn branch(rel: u32) -> u32 {
    // b isr ; branch instruction [1]
    0xea000000 | (((rel - 8) >> 2) & 0xffffff)
}

pub struct Table;

impl Table {
    pub fn new() -> Table {
        Table
    }

    #[allow(visible_private_types)]
    pub fn enable(&self, which: Int, isr: unsafe fn()) {
        // Installing exception handlers into the vectors directly [1]
        let vector: u8 = unsafe { transmute(which) };
        set_word(vector, branch(isr as u32 - (vector as u32 * 4)));
    }

    pub fn load(&self) {
        let mut i = 0;
        while i < 10 {
            // make every handler loop indefinitely
            set_word(i, branch(0));
            i += 1;
        }

        self.enable(Reset, unsafe { transmute(start) });
        // breakpoints use an UND opcode to trigger UNDEF. [7]
        self.enable(Undef, debug);
        self.enable(SWI, handler);

        unsafe {
            // Enable IRQs [5]
            asm!("mov r2, sp
              mrs r0, cpsr      // get Program Status Register
              bic r1, r0, #0x1F // go in IRQ mode
              orr r1, r1, #0x12
              msr cpsr, r1
              mov sp, 0x19000   // set IRQ stack
              bic r0, r0, #0x80 // Enable IRQs
              msr cpsr, r0      // go back in Supervisor mode
              mov sp, r2"
            ::: "r0", "r1", "r2", "cpsr");

            // enable UART0 IRQ [4]
            (*PIC).int_enable = UART0;
            // enable RXIM interrupt
            *io::UART0_IMSC = 1 << 4;
        }
    }
}

extern {
    fn start();
}

#[no_mangle]
pub unsafe fn debug() {
    asm!("movs pc, lr")
}

unsafe fn handler() {
    kernel::syscall::handler(&mut kernel::syscall::args(0, 0, 0, 0));
}

/*
#[lang="fail_"]
#[fixed_stack_segment]
pub fn fail(expr: *u8, file: *u8, line: uint) -> ! {
    unsafe { zero::abort(); }
}

#[lang="fail_bounds_check"]
#[fixed_stack_segment]
pub fn fail_bounds_check(file: *u8, line: uint, index: uint, len: uint) {
    unsafe { zero::abort(); }
}
*/
