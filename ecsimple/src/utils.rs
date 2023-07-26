use num_bigint::{BigInt,Sign};
use num_traits::{zero,one};


pub (crate) fn get_max_bits(bn :&BigInt) -> i64 {
	let mut retv : i64 = -1;
	let mut idx : i64 = 0 ;
	let zv :BigInt = zero();
	let mut cv : BigInt = one();

	while bn >= &cv {
		if (&cv & bn) != zv {
			/*for expand*/
			retv = idx + 1;
		}
		idx += 1;
		cv <<= 1;
	}
	return retv;
}

pub (crate) fn format_bigint_as_order(bn :&BigInt, order :&BigInt) -> BigInt {
	let obits :i64 = get_max_bits(order);
	let mut bs :Vec<u8>;
	(_,bs) = bn.to_bytes_be();
	if (8 * bs.len()) > (obits as usize) {
		bs = bs[0..(((obits as usize) +7) >> 3)].to_vec();
	}

	let mut retv :BigInt = BigInt::from_bytes_be(Sign::Plus,&bs);
	if bs.len() * 8 > (obits as usize) {
		retv = retv >> (8 - obits & 0x7);
	}
	return retv;
}

pub (crate) fn get_bit_set(bn :&BigInt,i :i32) -> i32 {
	let mut retv :i32 = 0;
	let ov :BigInt = one();
	let zv :BigInt = zero();
	if (bn & (ov << i)) != zv {
		retv = 1;
	}
	return retv;
}

pub (crate) fn nmod(a :&BigInt,m :&BigInt) -> BigInt {
	let mut retv :BigInt;
	let zv :BigInt = zero();
	retv = a % m;
	if retv < zv {
		retv += m;
	}
	return retv;
}
