pub struct args(u32, u32, u32, u32);

macro_rules! syscall(
    // 1 arg
    (fn $name:ident($a0:ident: $t0:ty) $func:expr) => (
        fn $name(regs: &mut idt::Registers) {
            let $a0 = regs.ebx as $t0;
            $func
        }
    );
    // 3 args
    (fn $name:ident($a0:ident: $t0:ty, $a1:ident: $t1:ty, $a2:ident: $t2:ty) -> $ret:ty $func:expr) => (
        fn $name(regs: &mut idt::Registers) {
            let $a0 = regs.ebx as $t0;
            let $a1 = regs.ecx as $t1;
            let $a2 = regs.edx as $t2;
            regs.eax = { $func } as $ret;
        }
    );
    (fn $name:ident($a0:ident: $t0:ty, $a1:ident: $t1:ty, $a2:ident: $t2:ty) $func:expr) => (
        fn $name(regs: &mut idt::Registers) {
            let $a0 = regs.ebx as $t0;
            let $a1 = regs.ecx as $t1;
            let $a2 = regs.edx as $t2;
            $func
        }
    );
)

pub fn handler(a: &mut args) -> u32 {
    0
}

pub fn init() {
}
