use core::mem::volatile_store;

pub static UART0: *mut u32 = 0x101f1000 as *mut u32;
pub static UART0_IMSC: *mut u32 = (0x101f1000 + 0x038) as *mut u32;

pub unsafe fn write_word(c: u32) {
    volatile_store(UART0, c);
}

pub unsafe fn write_char(c: char) {
    if c == '\r' {
        *UART0 = '\x1b' as u32;
        volatile_store(UART0, '\x1b' as u32);
    }
    volatile_store(UART0, c as u32);
}

pub fn putc(c: u32) {
	unsafe {
		write_word(c);
	}
}
