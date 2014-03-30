use core::mem::{transmute, size_of};
use core::clone::{Clone, DeepClone};
use core;

use kernel::mm::physical;
use kernel::mm::physical::Phys;
use util::int::range;
use util::rt;
use kernel;

pub type Frame = [u8, ..PAGE_SIZE];

// 32380 => 32460 bytes!
define_flags!(Flags: uint {
    PRESENT  = 1 << 0,
    RW       = 1 << 1,
    USER     = 1 << 2,
    ACCESSED = 1 << 5,
    HUGE     = 1 << 7
})

#[packed]
pub struct Page(uint);

static PAGE_SIZE: uint = 0x1000;
static PAGE_SIZE_LOG2: uint = 12;
static ENTRIES:   uint = 1024;

static DIR_VADDR: uint = 0xFFFFF000;
static TEMP1: uint = 0xFF7FF000;

static directory_temp_tables: *mut Directory = 0xFF800000_u as *mut Directory;
static directory_temp: *mut PageDirectory = 0xFFBFF000_u as *mut PageDirectory;

static directory_tables: *mut Directory = 0xFFC00000_u as *mut Directory;
pub static directory: *mut PageDirectory = DIR_VADDR as *mut PageDirectory;

// U: underlying element type
#[packed]
struct Table<U> {
    entries: [Page, ..ENTRIES]
}

#[packed]
struct Directory<U = PageTable> {
    entries: [U, ..ENTRIES]
}

pub type PageTable = Table<Page>;
pub type PageDirectory = Table<Table<Page>>;

pub unsafe fn init() {
    let dir: Phys<PageDirectory> = physical::zero_alloc_frames(1);
    let table: Phys<PageTable>   = physical::alloc_frames(1);

    (*table.as_ptr()).identity_map(0, PRESENT | RW);
    (*dir.as_ptr()).set_addr(0 as *mut u8, table, PRESENT | RW);

    // Map the directory as its own last table.
    // When accessing its virtual address(...)
    (*dir.as_ptr()).set_addr(directory, dir, PRESENT | RW);

    kernel::int_table.map(|mut t| {
        use super::exception::{PageFault, exception_handler};
        t.set_isr(PageFault, true, exception_handler());
    });

    switch_directory(dir);
    enable_paging();
}

pub fn switch_directory(dir: Phys<PageDirectory>) {
    use common::x86::reg::CR3;
    CR3::write(Page::new(dir, Flags::zero()));
}

fn enable_paging() {
    use common::x86::reg::{CR0, CR0_PG};
    CR0::write(CR0 | CR0_PG);
}

pub unsafe fn map(page_ptr: *mut u8, len: uint, flags: Flags) {
    (*directory).map(page_ptr, len, flags);
}

#[inline]
fn flush_tlb<T>(addr: T) {
    unsafe {
        asm!("invlpg [$0]" :: "r"(addr) : "memory" : "volatile", "intel")
    }
}

impl Page {
    fn new<T>(addr: Phys<T>, flags: Flags) -> Page {
        Page(addr.offset()) | flags
    }

    fn at_frame(i: uint, flags: Flags) -> Page {
        Page(i * PAGE_SIZE) | flags
    }

    fn physical<P>(&self) -> Phys<P> {
        match *self {
            Page(p) => Phys::at(p & 0xFFFFF000)
        }
    }

    fn present(self) -> bool {
        self & PRESENT
    }
}

impl core::ops::BitOr<Flags, Page> for Page {
    #[inline(always)]
    fn bitor(&self, other: &Flags) -> Page {
        match (self, other) {
            (&Page(p), &Flags(f)) => Page(p | f)
        }
    }
}

impl core::ops::BitAnd<Flags, bool> for Page {
    #[inline(always)]
    fn bitand(&self, other: &Flags) -> bool {
        match (self, other) {
            (&Page(p), &Flags(f)) => p & f != 0
        }
    }
}

impl<U> Table<U> {
    fn set_addr<S, T>(&mut self, vaddr: *mut S, phys: Phys<T>, flags: Flags) {
        // FIXME error: internal compiler error: missing default for a not explicitely provided type param
        self.set(vaddr as uint, Page::new(phys, flags));
        flush_tlb(vaddr);
    }

    fn set(&mut self, addr: uint, page: Page) { // TODO addr: Phys<T>
        // update entry, based on the underlying type (page, table)
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr / size) % ENTRIES;
        self.entries[index] = page;
    }

    fn get(&self, addr: uint) -> Page {
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr / size) % ENTRIES;
        self.entries[index]
    }
}

impl Table<Page> {
    fn identity_map(&mut self, start: uint, flags: Flags) {
        range(0, ENTRIES, |i| {
            self.entries[i] = Page::at_frame(start + i, flags);
        });
    }
}

// Can't impl on typedefs. Rust #9767
impl Table<Table<Page>> {
    fn fetch_table<T>(&mut self, vptr: *mut T, flags: Flags) -> *mut PageTable {
        match self.get(vptr as uint) {
            table @ Page(_) if table.present() => {
                table.physical().as_ptr()
            }
            _ => unsafe { // allocate table
                let table: Phys<PageTable> = physical::zero_alloc_frames(1);
                self.set_addr(vptr, table, flags); // page fault
                // flush_tlb(table);
                table.as_ptr()
            }
        }
    }

    pub unsafe fn set_page<T>(&mut self, vptr: *mut T, phys: Phys<T>, flags: Flags) -> *mut T {
        let table = self.fetch_table(vptr, flags);
        (*table).set_addr(vptr, phys, flags);
        vptr
    }

    pub unsafe fn map_frame(&mut self, vptr: *mut u8, flags: Flags) {
        self.set_page(vptr, physical::alloc_frames(1), flags | PRESENT);
    }

    pub fn map(&mut self, mut page_ptr: *mut u8, len: uint, flags: Flags) {
        use util::ptr::mut_offset;
        // TODO: optimize with uints?
        unsafe {
            let end = mut_offset(page_ptr, len as int);
            while page_ptr < end {
                let frame = physical::alloc_frames(1);
                self.set_page(page_ptr, frame, flags | PRESENT);
                (*directory).set_page(page_ptr, frame, flags | PRESENT);
                page_ptr = mut_offset(page_ptr, PAGE_SIZE as int);
            }
        }
    }

    // fn map_self

    pub fn clone(&self) -> *mut Table<Table<Page>> {
        unsafe {
            // new directory
            let dir_phys: Phys<PageDirectory> = physical::zero_alloc_frames(1);
            let dir_temp = (*directory).set_page(transmute(TEMP1), dir_phys, PRESENT | RW | USER);

            rt::breakpoint();
            (*dir_temp).set(directory as uint, Page::new(dir_phys, PRESENT | RW));
            (*dir_temp).set(0, self.get(0));

            let mut i = (ENTRIES * PAGE_SIZE) as uint;
            while i < 0xC0000000 {
                (*dir_temp).set(i, self.get(i));

                i += PAGE_SIZE as uint;
            }

            dir_phys.as_ptr()
        }
    }
}

// impl Clone for Table<Table<Page>> {
//     #[inline(always)]
//     fn clone(&self) -> Table<Table<Page>> {
//         unsafe {
//             // new directory
//             let dir_phys: Phys<PageDirectory> = physical::zero_alloc_frames(1);
//             let dir_temp = (*directory).set_page(transmute(TEMP1), dir_phys, PRESENT | RW | USER);

//             rt::breakpoint();
//             (*dir_temp).set(directory as uint, Page::new(dir_phys, PRESENT | RW));
//             (*dir_temp).set(0, self.get(0));

//             let mut i = (ENTRIES * PAGE_SIZE) as uint;
//             while i < 0xC0000000 {
//                 (*dir_temp).set(i, self.get(i));

//                 i += PAGE_SIZE as uint;
//             }

//             *dir_phys.as_ptr()
//         }
//     }
// }

// impl DeepClone for Table<Table<Page>> {
//     #[inline(always)]
//     fn deep_clone(&self) -> Table<Table<Page>> {
//         *self
//     }
// }
