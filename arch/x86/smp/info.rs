use core::mem::{transmute, size_of};
use core::option::{Option, Some, None};

struct MPFloatingPointer
{
	signature: u32,
	config_ptr: u32,
	config_length: u8,
	version: u8,
	checksum: u8,
	features_1: u8,
	features_2: u8,
	_fill: u16,
	__fill: u8
}

impl MPFloatingPointer
{
	#[inline(always)]
	unsafe fn ConstructSearchPtr() -> (*mut u32, *mut u32)
	{
		// TODO P2V
		let bda = 0x400 as *mut u8;

		((*(bda+0x0F) << 8 | *(bda+0x0E) << 4) as *mut u32,
		 ((*(bda+0x14) << 8 | (*(bda+0x13) << 4)) as u32 * 1024 - 1024) as *mut u32)
	}

	unsafe fn Search(ptr: *mut u32, length: u32) -> Option<*mut MPFloatingPointer>
	{
		// TODO P2V
		//
		let mut pos: *mut MPFloatingPointer = ptr as *mut MPFloatingPointer;
		let end = ptr + length as int;

		while end > (pos as *mut u32) {
			if (*pos).signature == 0x5F4D505F/*"_MP_" in hex*/ {
				return Some(pos);
			}
			pos = pos + size_of::<MPFloatingPointer>() as int;
		}
	
		return None;
	}

	unsafe fn Find() -> Option<*mut MPFloatingPointer>
	{
		let (p1, p2) = MPFloatingPointer::ConstructSearchPtr();
		match MPFloatingPointer::Search(p1, 1024) {
			Some(mp_t) => { return Some(mp_t) },
			None => {}
		}
		
		match MPFloatingPointer::Search(p2, 1024) {
			Some(mp_t) => { return Some(mp_t) },
			None => {}
		}

		match MPFloatingPointer::Search(0xF0000 as *mut u32, 0x10000) {
			Some(mp_t) => { return Some(mp_t) },
			None => None
		}	 		
	}
}

struct MPConf
{
	signature: u32,
	length: u16,
	version: u8,
	checksum: u8,
	product: [u8, ..20],
	oemtable: *mut u32,
	oemlength: u16,
	entry: u16,
	lapicaddr: *mut u32,
	xlength: u16,
	xchecksum: u8,
	reserved: u8
}

impl MPConf
{
	unsafe fn Find() -> Option<*mut MPConf>
	{
		let conf = match MPFloatingPointer::Find() {
			Some(mp_t) => mp_t,
			None => { return None }
		};

		let mpconf = (*conf).config_ptr as *mut MPConf;

		if((*mpconf).signature != 0x50434D50) {
			return None;
		}
		if((*mpconf).version != 1 && (*mpconf).version != 4) {
			return None;
		}
		// TODO SUM
		
		Some(mpconf)
	}
}

unsafe fn init() {
	let conf = match MPConf::Find() {
		Some(mp_t) => mp_t,
		None => { return; }
	};
}	
	
