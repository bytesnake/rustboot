use core::container::Container;
use core::mem::transmute;

use kernel;

// use platform::runtime::memset;

pub type EfiHandle = *();
pub struct EfiGuid(u32, u16, u16, [u8, ..8]);

struct EfiTableHeader {
    signature  : u64,
    revision   : u32,
    headerSize : u32,
    crc32      : u32,
    reserved   : u32
}

pub struct EfiSystemTable {
    priv hdr : EfiTableHeader,
    priv firmwareVendor : *u16,
    priv firmwareRevision : u32,
    priv consoleInHandle : EfiHandle,
    priv conIn : *EfiSimpleTextInputProtocol,
    priv consoleOutHandle : EfiHandle,
    priv conOut : *EfiSimpleTextOutputProtocol,
    priv consoleErrorHandle : EfiHandle,
    priv stdErr : *EfiSimpleTextOutputProtocol,
    priv runtimeServices : *EfiRuntimeServices,
    priv bootServices : *EfiBootServices,
    priv numberOfTableEntries : uint,
    priv configurationTable : *EfiConfigurationTable
}

pub static mut SYSTEM_TABLE : *EfiSystemTable = 0 as *EfiSystemTable;

struct EfiSimpleTextInputProtocol {
    reset: EfiInputReset,
    readKeyStroke: EfiInputReadKey,
    waitForKey: *()
}

type EfiInputReset = extern "win64" fn(*EfiSimpleTextInputProtocol,
                                       bool);

type EfiInputReadKey = extern "win64" fn(*EfiSimpleTextInputProtocol,
                                         *mut EfiInputKey);

struct EfiInputKey(u16, u16);

struct EfiSimpleTextOutputProtocol {
    reset : EfiTextReset,
    outputString : EfiTextString,
    // ... and more stuff that we're ignoring.
}

type EfiTextReset = *();

type EfiTextString = extern "win64" fn(*EfiSimpleTextOutputProtocol,
                                       *u16);

struct EfiRuntimeServices;

struct EfiBootServices;

struct EfiConfigurationTable {
    vendorGuid : EfiGuid,
    vendorTable : *()
}

pub struct SystemTable(*EfiSystemTable);


impl SystemTable {
    #[no_split_stack]
    pub fn console(&self) -> Console {
        unsafe {
            let &SystemTable(tbl) = self;
            Console {
                input:  (*tbl).conIn,
                output: (*tbl).conOut,
            }
        }
    }
}

fn unpack<T>(slice: &[T]) -> (*T, uint) {
    unsafe {
        transmute(slice)
    }
}

pub trait SimpleTextOutput {
    unsafe fn write_raw(&self, str: *u16);

    #[no_split_stack]
    fn write(&self, str: &str) {
        let mut buf = [0u16, ..4096];

        let mut i = 0;
        while i < buf.len() && i < str.len() {
            // TODO: make sure the characters are all ascii
            buf[i] = str[i] as u16;
            i += 1;
        }
        buf[buf.len() - 1] = 0;

        unsafe {
            let (p, _) = unpack(buf);
            self.write_raw(p);
        }
    }
}

pub trait SimpleTextInput {
}

pub struct Console {
    priv input  : *EfiSimpleTextInputProtocol,
    priv output : *EfiSimpleTextOutputProtocol,
}

impl SimpleTextOutput for Console {
    #[no_split_stack]
    unsafe fn write_raw(&self, str: *u16) {
        ((*(*self).output).outputString)(self.output, str);
    }
}

impl SimpleTextInput for Console {
}

#[no_mangle]
#[no_split_stack]
pub fn efi_main(sys_table : SystemTable) {
    sys_table.console().write("Hello!");
    abort();
}

#[no_mangle]
#[no_split_stack]
pub extern "win64" fn efi_start(_ImageHandle : EfiHandle,
                                sys_table : *EfiSystemTable) -> int {
    unsafe { SYSTEM_TABLE = sys_table; }
    efi_main(SystemTable(sys_table));
    // kernel::main(); //SystemTable(sys_table)
    0
}

#[no_mangle]
#[no_split_stack]
pub fn __morestack() {
    // Horrible things will probably happen if this is ever called.
}

#[no_mangle]
#[no_split_stack]
pub fn abort() -> ! {
    loop {}
}

#[no_mangle]
#[no_split_stack]
pub fn breakpoint() {
}
