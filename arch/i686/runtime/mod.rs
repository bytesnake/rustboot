// use core::mem::transmute;
use core::cmp::expect;
use core::ptr::offset;
use core::c_types::c_int;

use util::int::range;
use util::ptr::mut_offset;
use util::int::range;
use kernel;

mod stack;

// TODO: use SSE

#[inline]
fn stosb(s: *mut u8, c: u8, n: uint) {
    unsafe {
        asm!("rep stosb" :: "{al}"(c), "{edi}"(s), "{ecx}"(n))
    }
}

#[inline]
fn stosd(s: *mut u8, c: u32, n: uint) {
    unsafe {
        asm!("rep stosl" :: "A"(c), "{edi}"(s), "{ecx}"(n))
    }
}

#[inline]
fn stosd8(s: *mut u8, c: u8, n: uint) {
    unsafe {
        let mut dword: u32 = c as u32;
        dword |= (dword << 24) | (dword << 16) | (dword << 8);
        asm!("rep stosl" :: "A"(dword), "{edi}"(s), "{ecx}"(n))
    }
}

#[inline]
fn stosd16(s: *mut u8, c: u16, n: uint) {
    unsafe {
        let mut dword: u32 = c as u32;
        dword |= dword << 16;
        asm!("rep stosl" :: "A"(dword), "{edi}"(s), "{ecx}"(n))
    }
}

#[inline]
fn memset_nonzero(mut s: *mut u8, c: u8, mut n: uint) {
    if unlikely!(n == 0) {
        return
    }
    if unlikely!(n == 1) {
        unsafe { *s = c; }
        return
    }

    while n > 0 {
        match n % 4 {
            0 => {
                stosd8(s, c, n / 4);
                n = 0;
            }
            /*2 => unsafe {
                let mut word: u16 = c as u16;
                word = (word << 8) | word;
                asm!("rep stosw" :: "A"(word), "{edi}"(s), "{ecx}"(n / 2))
                n = 0;
            },*/
            q => {
                stosb(s, c, q);
                s = unsafe { mut_offset(s, q as int) };
                n -= q;
            }
        }
    }
}

pub fn wmemset(mut dest: *mut u8, c: u16, n: uint) {
    if unlikely!(n == 0) {
        return;
    }

    if (n % 2) == 1 {
        unsafe {
            *(dest as *mut u16) = c;
            dest = mut_offset(dest, 2);
        }
    }

    stosd16(dest, c, n >> 1);
}

fn dmemset(s: *mut u8, c: u32, n: uint) {
    if unlikely!(n == 0) {
        return;
    }

    stosd(s, c, n);
}
/*
fn dqamemset(s: *mut u8, c: u32, n: uint) {
    if unlikely!(n == 0) {
        return;
    }

    stosd(s, c, n);
}*/

pub fn sse2_dmem_movdqa_add(mut dest: *mut u8, c: u32, inc: u32, n: uint) {
    unsafe {
        // kernel::rt::breakpoint();
        asm!("movd xmm0, $1 // xmm0 = start page
              pshufd xmm0, xmm0, 0

              movd xmm1, $0 // xmm1 = increment
              paddd xmm0, xmm1
              pshufd xmm1, xmm1, 0b11110000
              paddd xmm0, xmm1
              pshufd xmm1, xmm1, 0b11000000
              paddd xmm0, xmm1
              pshufd xmm0, xmm0, 0b00011011
              pshufd xmm1, xmm1, 0
              pslld xmm1, 2"
            :: "r"(inc), "r"(c)
            :: "intel")
        range(0, n, |_| {
            asm!("movdqa [$0], xmm0
                  paddd xmm0, xmm1" :: "r"(dest) :: "intel")
            dest = mut_offset(dest, 16);
        });
    }
}

#[no_mangle]
pub fn memset(s: *mut u8, c: c_int, n: int) {
    memset_nonzero(s, (c & 0xFF) as u8, n as uint);
}

#[no_mangle]
pub fn memcpy(dest: *mut u8, src: *u8, mut n: uint) {
    if unlikely!(n == 0) {
        return;
    }
    unsafe {
        if n < 12 {
            asm!("rep movsb" :: "{edi}"(dest), "{esi}"(src), "{ecx}"(n))
            return;
        }

        let offset = (4 - (dest as uint % 4)) % 4;
        n -= offset;

        let mut pd: *mut u8;
        let mut ps: *u8;
        asm!("rep movsb" : "={edi}"(pd), "={esi}"(ps) : "{edi}"(dest), "{esi}"(src), "{ecx}"(offset))
        asm!("rep movsl" : "={edi}"(pd), "={esi}"(ps) : "{edi}"(pd), "{esi}"(ps), "{ecx}"(n >> 2))
        asm!("rep movsb" :: "{edi}"(pd), "{esi}"(ps), "{ecx}"(n % 4))
    }
}

#[no_mangle]
pub fn memmove(dest: *mut u8, src: *u8, n: uint) {
    unsafe {
        if src < dest as *u8 {
            asm!("std")
            memcpy(mut_offset(dest, n as int), offset(src, n as int), n);
            asm!("cld")
        }
        else {
            asm!("cld")
            memcpy(dest, src, n);
        }
    }
}
