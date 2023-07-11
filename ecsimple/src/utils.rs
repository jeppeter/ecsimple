use num_bigint::{BigInt};
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

pub (crate) fn get_bit_set(bn :&BigInt,i :i32) -> i32 {
	let mut retv :i32 = 0;
	let ov :BigInt = one();
	let zv :BigInt = zero();
	if (bn & (ov << i)) != zv {
		retv = 1;
	}
	return retv;
}
