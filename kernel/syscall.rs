use core::slice;
use core::mem::transmute;

use platform = arch::x86;
use cpu::Context;

pub struct args(u32, u32, u32, u32);

macro_rules! syscall(
    (fn $name:ident() -> $ret:ty $func:expr) => (
        pub fn $name(regs: &mut Context) {
            let ret: $ret = { $func };
            regs.eax = ret as u32;
        }
    );
    (fn $name:ident($a0:ident: $t0:ty) -> $ret:ty $func:expr) => (
        pub fn $name(regs: &mut Context) {
            let $a0 = regs.ebx as $t0;
            let ret: $ret = { $func };
            regs.eax = ret as u32;
        }
    );
    // 1 arg
    (fn $name:ident($a0:ident: $t0:ty) $func:expr) => (
        fn $name(regs: &mut Context) {
            let $a0 = regs.ebx as $t0;
            $func
        }
    );
    // 3 args
    (fn $name:ident($a0:ident: $t0:ty, $a1:ident: $t1:ty, $a2:ident: $t2:ty) -> $ret:ty $func:expr) => (
        fn $name(regs: &mut Context) {
            let $a0 = regs.ebx as $t0;
            let $a1 = regs.ecx as $t1;
            let $a2 = regs.edx as $t2;
            let ret: $ret = { $func };
            regs.eax = ret as u32;
        }
    );
    (fn $name:ident($a0:ident: $t0:ty, $a1:ident: $t1:ty, $a2:ident: $t2:ty) $func:expr) => (
        fn $name(regs: &mut Context) {
            let $a0 = regs.ebx as $t0;
            let $a1 = regs.ecx as $t1;
            let $a2 = regs.edx as $t2;
            $func
        }
    );
)

pub fn handler(ctx: &mut Context) {
    // let args(f, _, _, _) = *a;
    match ctx.eax {
        1 => {
            exit(ctx)
        }
        3 => {
            read(ctx)
        }
        4 => {
            write(ctx)
        }
        5 => {
            open(ctx)
        }
        6 => {
            close(ctx)
        }
        // 12 => {
        //     chdir(ctx)
        // }
        23 => {
            setuid(ctx)
        }
        243 => {
            set_thread_area(ctx)
        }
        // 25 => {
        //     getuid(ctx)
        // }
        _ => {
            platform::io::puti(ctx.eax as int);
            platform::io::putc(' ' as u8);
        }
    }
}

pub fn init() {
}

syscall!(fn exit(error_code: int) -> uint {
    platform::io::puti(error_code as int);
    0
})

syscall!(fn read(fd: uint, buf: *u8, count: uint) -> uint {
    0
})

syscall!(fn write(fd: uint, buf: *u8, count: uint) -> uint {
    unsafe {
        platform::io::puts(transmute(slice::Slice { data: buf, len: count }));
    }
    count
})

syscall!(fn open(filename: *u8, flags: int, mode: uint) -> uint {
    0
})

syscall!(fn close(fd: uint) -> uint {
    0
})

syscall!(fn chdir(filename: *u8) -> uint {
    0
})

syscall!(fn setuid(uid: uint) -> uint {
    0
})

// syscall!(fn getuid() -> uint {
//     0
// })

syscall!(fn set_thread_area(entry: *platform::cpu::IdtEntry) -> uint {
    0
})
