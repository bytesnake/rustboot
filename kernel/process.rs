use core::clone::Clone;
use core::mem::transmute;

use kernel::mm::virtual::{Flags, PageDirectory};
use kernel::mm::physical;

use util::rt::breakpoint;

use platform::cpu::mmu::{switch_directory, directory};

pub struct Process {
    eip: u32,
    esp: u32,
    paging: physical::Phys<PageDirectory>
}

impl Process {
    pub fn new() -> Process {
        Process {
            eip: 0,
            esp: 0,
            // paging: unsafe { physical::zero_alloc_frames(1) as *mut PageDirectory }
            paging: unsafe { (*directory).clone() }
        }
    }

    pub fn mmap(&self, page_ptr: *mut u8, size: uint, flags: Flags) {
        unsafe {
            (*self.paging.as_ptr()).map(page_ptr, size, flags);
        }
    }

    #[cfg(target_arch = "x86")]
    pub fn enter(&self) {
        unsafe {
            breakpoint();
            // TODO need to store physical address
            switch_directory(self.paging);
            asm!("xor %eax, %eax
                  xor %edx, %edx
                  jmp *$0" :: "m"(self.eip), "{esp}"(self.esp) :: "volatile")
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn enter(&self) {}
}
