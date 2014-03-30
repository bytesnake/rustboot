#![no_std]

extern crate core;

use core::slice;
use core::mem::transmute;

extern {
	fn write(fd: int, buffer: *u8, count: uint) -> uint;
}

#[start]
pub fn main(_: int, _: **u8) -> int {
	unsafe {
		let sl: slice::Slice<u8> = transmute("Hello, world!");
		write(0, sl.data, sl.len);
	}
    42
}
