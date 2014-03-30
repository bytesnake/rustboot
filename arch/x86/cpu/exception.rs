use core::mem::transmute;

use platform::io;
use cpu::Context;
use cpu::idt;
use kernel::heap;
use kernel::memory::Allocator;

#[repr(u8)]
pub enum Fault {
    DivideError = 0,
    NMI = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundExceeded = 5,
    InvalidOpcode = 6,
    NoMathCoprocessor = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverun = 9,
    InvalidTss = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtection = 13,
    PageFault = 14,
    FloatingPointError = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdFpException = 19,
}

static Exceptions: &'static [&'static str] = &[
    "Divide-by-zero Error",
    "Debug",
    "Non-maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "x87 Floating-Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
];

#[no_split_stack]
#[inline(never)]
unsafe fn blue_screen(stack: &Context) {
    io::puts("Exception ");
    io::puts(Exceptions[stack.int_no]);
    asm!("hlt");
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn exception_handler() -> extern "C" unsafe fn() {
    asm!("jmp skip_exception_handler
      exception_handler_asm:"
        ::: "volatile" : "eax", "volatile", "intel")

    // Points to the data on the stack
    let stack_ptr = Context::save();

    if stack_ptr.int_no as u8 == PageFault as u8 {
        let mut value: u32;
        asm!("mov %cr2, $0" : "=r"(value));
        io::putx((value >> 8) as uint);
        io::putc(' ' as u8);
        io::putx(stack_ptr.call_stack.eip as uint);
    }

    if stack_ptr.int_no as u8 == transmute(Breakpoint) {
        asm!("debug:" :::: "volatile")
    }
    else {
        blue_screen(stack_ptr);
    }

    Context::restore();

    asm!("skip_exception_handler:"
        :::: "volatile", "intel")

    extern { fn exception_handler_asm(); }
    exception_handler_asm
}
