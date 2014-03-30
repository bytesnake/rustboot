use core::option::{Some, None};
use core::{str, slice};
use core::iter::Iterator;

use super::drivers::vga;
use super::drivers::keyboard;
use util::int;

pub static mut pos: int = 0;

pub unsafe fn seek(offset: int) {
    pos += offset;
    vga::cursor_at(pos as uint);
}

pub unsafe fn write_char(c: char) {
    if c == '\x08' {
        if pos > 0 {
            if pos % 80 == 0 {
                while (*vga::SCREEN)[pos-1].char == 0 {
                    pos -= 1;
                }
            }
            else if pos > 0 {
                if pos > 0 { pos -= 1; }
                (*vga::SCREEN)[pos].char = 0;
            }
        }
    }
    else if c == '\n' {
        seek(80 - pos % 80);
    }
    else {
        (*vga::SCREEN)[pos].char = c as u8;
        pos += 1;
    }

    vga::cursor_at(pos as uint);
}

pub fn putc(c: u8) {
    unsafe {
        write_char(c as char);
    }
}

pub fn puts(s: &str) {
    for c in slice::iter(str::as_bytes(s)) {
        putc(*c);
    }
}

pub fn puti(num: int) {
    int::to_str_bytes(num, 10, |n| {
        putc(n);
    });
}
