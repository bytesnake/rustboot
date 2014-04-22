use core::fail::assert;

pub fn range(lo: uint, hi: uint, it: |uint|) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}

macro_rules! int_module (($T:ty, $bits:expr) => (

#[inline]
pub fn to_str_bytes(mut num: $T, radix: uint, f: |u8|) {
	let isNeg = num < 0;

	if isNeg {
		num = -num;
	}

	let mut buf = [0u8, ..65];
	let mut cur: u32 = 0;
	let mut digit;

	while(num != 0) {
		digit = num % radix as $T;
		buf[cur] = match digit as u8 {
			i @ 0..9 => '0' as u8 + i,
			i        => 'A' as u8 + (i-10),
		};

		cur += 1;
		num /= radix as $T;
	}

	if isNeg {
		f('-' as u8);
	}

	while cur > 0 {
		cur -= 1;
		f(buf[cur]);
	}
}

))

#[cfg(target_word_size = "32")] int_module!(int, 32)
#[cfg(target_word_size = "64")] int_module!(int, 64)
