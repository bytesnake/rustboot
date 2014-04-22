use core::mem::{transmute, size_of};
use core::option::{Option, Some, None};
use platform::{io, cpu};

#[packed]
struct MPFloatingPointer
{
	signature: u32,
	config_ptr: u32,
	config_length: u8,
	version: u8,
	checksum: u8,
	features_1: u8,
	features_2: u8,
	fill: [u8, ..3]
}

impl MPFloatingPointer
{
	#[inline(always)]
	unsafe fn ConstructSearchPtr() -> (*mut u32, *mut u32)
	{
		// TODO P2V
		let bda = 0x400 as *mut u8;
		
		(/*((*(bda+0x0F) as u32) << 8 | */((*(bda+0x0E) as u32) << 4) as *mut u32,
		 ((*(bda+0x14) << 8 | (*(bda+0x13) << 4)) as u32 * 1024 - 1024) as *mut u32)
	}

	unsafe fn Search(ptr: *mut u32, length: u32) -> Option<*mut MPFloatingPointer>
	{
		// TODO P2V
		let mut pos = ptr as *mut u8;
		let end = ptr + length as int;

		while (end as int) > (pos as int) {
			if (*(pos as *mut MPFloatingPointer)).signature == 0x5F504D5F/*"_MP_" in hex*/ {
				return Some(pos as *mut MPFloatingPointer);
			}

			pos = pos + 1;
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

		match MPFloatingPointer::Search(0xE0000 as *mut u32, 0x10000) {
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

struct MPProc
{
	etype: u8,
	apicid: u8,
	version: u8,
	flags: u8,
	signature: [u8, ..4],
	feature: u32,
	reserved: [u8, ..8]
}

struct IOApic
{
	etype: u8,
	apicno: u8,
	version: u8,
	flags: u8,
	addr: u32
}

impl MPConf
{
	unsafe fn Find() -> Option<*mut MPConf>
	{
		let conf = match MPFloatingPointer::Find() {
			Some(mp_t) => mp_t,
			None => { return None }
		};

		let mpconf = ((*conf).config_ptr) as *mut MPConf;

		if((*mpconf).signature != 0x504D4350) {
			return None;
		}
		if((*mpconf).version != 1 && (*mpconf).version != 4) {
			return None;
		}
		// TODO SUM
		
		Some(mpconf)
	}
}

pub unsafe fn init() {
	let conf = match MPConf::Find() {
		Some(mp_t) => mp_t,
		None => { return; } 
	};

	let mut p: *mut u8 = ((conf+1) as *mut u8);
	let e = (conf as *mut u8) + (*conf).length as int;

	let mut mpproc: *mut MPProc;
	let mut ioapic: *mut IOApic;

	let mut nproc: u16 = 0;

	while (p as int) < (e as int) {
		match *p {
			0x00 => {
				mpproc = p as *mut MPProc;
				nproc += 1;
				p = p + size_of::<MPProc>() as int;
			}
			0x02 => {
				ioapic = p as *mut IOApic;
				io::puts("*");
				p = p + size_of::<IOApic>() as int;
			}
			0x04 => {
				p = p + 8;
			}
			_ => {break;}
		}
	}

	io::puts("Found ");
	io::puti(nproc as int);
	io::puts(" CPUs\n");
}	
	
