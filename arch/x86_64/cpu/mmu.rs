use core::mem::{transmute, size_of};
use core::clone::{Clone, DeepClone};
use core;

use platform::runtime;
use kernel::mm::physical;
use kernel::mm::physical::Phys;
use util::int::range;
use util::rt;
use kernel;

pub type Frame = [u8, ..PAGE_SIZE];

// 32380 => 32460 bytes!
define_flags!(Flags: u32 {
    PRESENT  = 1 << 0,
    RW       = 1 << 1,
    USER     = 1 << 2,
    ACCESSED = 1 << 5,
    HUGE     = 1 << 7
})

#[packed]
struct Page(u32);

static PAGE_SIZE: uint = 0x1000;
static PAGE_SIZE_LOG2: uint = 12;
static ENTRIES:   uint = 1024;

static DIRECTORY_VADDR: u32 = 0xFFFFF000;
static TEMP1: u32 = 0xFF7FF000;

static directory_temp_tables: *mut Directory = 0xFF800000 as *mut Directory;
static directory_temp: *mut PageDirectory = 0xFFBFF000 as *mut PageDirectory;

static directory_tables: *mut Directory = 0xFFC00000 as *mut Directory;
pub static directory: *mut PageDirectory = DIRECTORY_VADDR as *mut PageDirectory;

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
    (*dir.as_ptr()).set_addr(transmute(0), table, PRESENT | RW);

    // Map the directory as its own last table.
    // When accessing its virtual address(...)
    (*dir.as_ptr()).set_addr(directory, dir, PRESENT | RW);

    kernel::int_table.map(|mut t| {
        use super::exception::{PageFault, exception_handler};
        t.set_isr(PageFault, true, exception_handler());
    });

    (*dir.as_ptr()).switch();
}

// TODO: directory.map
pub unsafe fn map(mut page_ptr: *mut u8, len: uint, flags: Flags) {
    use util::ptr::mut_offset;
    let end = mut_offset(page_ptr, len as int);
    while page_ptr < end {
        (*directory).map_frame(page_ptr, flags);
        page_ptr = mut_offset(page_ptr, PAGE_SIZE as int);
    }
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
        Page((i * PAGE_SIZE) as u32) | flags
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
    fn set_addr<T>(&mut self, vaddr: *mut T, phys: Phys<T>, flags: Flags) {
        // FIXME error: internal compiler error: missing default for a not explicitely provided type param
        self.set(vaddr as u32, Page::new(phys, flags));
        flush_tlb(vaddr);
    }

    fn set(&mut self, addr: u32, page: Page) {
        use platform::io::putx;
        // update entry, based on the underlying type (page, table)
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr as uint / size) % ENTRIES;
        self.entries[index] = page;
    }

    fn get(&self, addr: u32) -> Page {
        let size = size_of::<U>() / size_of::<Page>() * PAGE_SIZE;
        let index = (addr as uint / size) % ENTRIES;
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
    fn fetch_table(&mut self, addr: u32, flags: Flags) -> *mut PageTable {
        match self.get(addr) {
            table @ Page(_) if table.present() => {
                table.physical().as_ptr()
            }
            _ => unsafe { // allocate table
                let table: Phys<PageTable> = physical::zero_alloc_frames(1);
                (*directory).set_addr(addr as *mut PageTable, table, flags); // page fault
                // flush_tlb(table);
                table.as_ptr()
            }
        }
    }

    pub unsafe fn set_page<T>(&mut self, vptr: *mut T, phys: Phys<T>, flags: Flags) -> *mut T {
        let table = self.fetch_table(vptr as u32, flags);
        (*table).set_addr(vptr, phys, flags);
        vptr
    }

    pub unsafe fn map_frame(&mut self, vptr: *mut u8, flags: Flags) {
        self.set_page(vptr, physical::alloc_frames(1), flags | PRESENT);
    }

    // fn map_self

    unsafe fn switch(&mut self) {
        use super::{CR3, CR0, CR0_PG};
        CR3::write(self);
        CR0::write(CR0 | CR0_PG);
    }
}

impl Clone for Table<Table<Page>> {
    #[inline(always)]
    fn clone(&self) -> Table<Table<Page>> {
        unsafe {
            // new directory
            let dir_phys: Phys<PageDirectory> = physical::zero_alloc_frames(1);
            let dir_temp = (*directory).set_page(transmute(TEMP1), dir_phys, PRESENT | RW | USER);
            flush_tlb(dir_temp);
            rt::breakpoint();
            (*dir_temp).set(directory as u32, Page::new(dir_phys, PRESENT | RW));
            (*dir_temp).set(0, self.get(0));

            let mut i = (ENTRIES * PAGE_SIZE) as u32;
            while i < 0xC0000000 {
                (*dir_temp).set(i, self.get(i));

                i += PAGE_SIZE as u32;
            }

            *dir_phys.as_ptr()
        }
    }
}

impl DeepClone for Table<Table<Page>> {
    #[inline(always)]
    fn deep_clone(&self) -> Table<Table<Page>> {
        *self
    }
}
