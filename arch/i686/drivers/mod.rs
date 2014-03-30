use core::option::{Option, None};
use core::mem::transmute;

use cpu::interrupt::{interrupt_handler, Isr, Int};
use kernel;
pub mod pic;
pub mod vga;
pub mod keyboard;

pub static mut keydown: Option<fn(u8)> = None;

pub fn init() {
    vga::clear_screen(vga::LightRed);
    vga::cursor_at(0);

    unsafe {
        kernel::int_table.map(|mut t| {
            t.enable_maskable(keyboard::IRQ, keyboard::isr_addr());
            t.set_intr_gate(0x80, Isr::new(transmute(0x80u8), false));
        });
    }
}
