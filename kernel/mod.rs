use core::option::{Option, Some, None};

use platform::{cpu, io, drivers};
use cpu::interrupt;
pub use cpu::interrupt::Table;
use util::rt::breakpoint;

pub mod util;
pub mod mm;
pub mod heap;
pub mod syscall;
mod process;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod elf;
mod sh;

pub static mut int_table: Option<Table> = None;
// #[lang="fail_"]
// pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
//     loop {}
// }
#[lang="start"]
#[no_mangle]
pub fn main() {
    heap::init();
    mm::physical::init();

    let table = interrupt::Table::new();
    table.load();
    unsafe {
        int_table = Some(table);
        drivers::keydown = Some(io::putc);
    }
    cpu::init();

    drivers::init();
    // io::puti(-2147483648);
    // unsafe {asm!("swi 0")}
    elf::exec(&_binary_initram_elf_start);
    extern { static _binary_initram_elf_start: u8; }
}
