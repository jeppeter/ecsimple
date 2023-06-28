use num_bigint::{BigInt,Sign};

use crate::*;
use crate::logger::*;


type BValue = u64;

const BVALUE_SIZE :usize = std::mem::size_of::<BValue>();
#[allow(dead_code)]
const BVALUE_BITS :usize = BVALUE_SIZE * 8;

pub struct BinBn  {
	/*little endian*/
	data :Vec<BValue>,
}

#[allow(dead_code)]
impl BinBn {

	fn _check_self(&self) {
		if self.data.len() == 0 {
			panic!("self len 0");
		}
	}
	fn _check_other(&self,other :&BinBn) {
		self._check_self();
		other._check_self();
	}

	pub fn new_from_le(varr :&[u8]) -> BinBn {
		let mut rdata :Vec<BValue> = Vec::new();
		let mut passlen :usize = 0;
		let mut curval :BValue;
		let leftlen :usize;
		while (passlen + BVALUE_SIZE) <= varr.len() {
			curval = 0;
			for i in 0..BVALUE_SIZE {
				curval |= (varr[passlen + i] as BValue) << (i * 8);
			}
			rdata.push(curval);
			passlen += BVALUE_SIZE;
		}

		if passlen != varr.len() {
			curval = 0;
			leftlen = varr.len() - passlen;
			for i in 0..leftlen {
				curval |= (varr[passlen+i] as BValue) << (i * 8);
			}
			rdata.push(curval);
		}
		if rdata.len() == 0 {
			rdata.push(0);
		}

		ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len() * BVALUE_SIZE, "to bytes");
		BinBn {
			data : rdata,
		}
	}

	pub fn new_from_be(varr :&[u8]) -> BinBn {
		let mut rdata :Vec<BValue> = Vec::new();
		let mut passlen :usize = 0;
		let leftlen :usize;
		let mut curval :BValue;
		if (varr.len() % BVALUE_SIZE) != 0 {
			leftlen = varr.len() % BVALUE_SIZE;
			curval = 0;
			for i in 0..leftlen {
				curval |= (varr[i] as BValue) << ((leftlen - i - 1) * 8);
			}
			rdata.insert(0,curval);
			passlen += leftlen;
		}
		ecsimple_debug_buffer_trace!(varr.as_ptr(),varr.len(), "varr ");

		while passlen < varr.len() {
			curval = 0;
			for i in 0..BVALUE_SIZE {
				curval |= (varr[passlen + i] as BValue) << ((BVALUE_SIZE - i - 1) * 8 );
			}
			rdata.insert(0,curval);
			passlen += BVALUE_SIZE;
		}

		if rdata.len() == 0 {
			rdata.push(0);
		}

		ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len() * BVALUE_SIZE, "to bytes");
		BinBn {
			data : rdata,
		}
	}

	pub fn to_bigint(&self) -> BigInt {
		let mut rdata :Vec<u8> = Vec::new();
		for i in 0..self.data.len() {
			for j in 0..BVALUE_SIZE {
				let val :u8 = (self.data[i] >> (j * 8)) as u8;
				rdata.push(val);
			}
		}
		ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len(), "to bytes");
		BigInt::from_bytes_le(Sign::Plus,&rdata)
	}

	pub fn add_op(&self, other :&BinBn) -> BinBn {
		let mut retv :Vec<BValue> = Vec::new();
		let mut maxlen :usize = self.data.len();
		let mut aval :BValue;
		let mut bval :BValue;
		let mut rv :BinBn;
		let r8 :Vec<u8> = vec![0];
		self._check_other(other);
		if maxlen < other.data.len() {
			maxlen = other.data.len();
		}

		for i in 0..maxlen {
			if i < self.data.len() {
				aval = self.data[i];
			} else {
				aval = 0;
			}
			if i < other.data.len() {
				bval = other.data[i];
			} else {
				bval = 0;
			}

			aval = aval ^ bval;
			retv.push(aval);
		}

		rv = BinBn::new_from_be(&r8);
		rv.data= retv;
		rv
	}


	pub fn mul_op(&self, other :&BinBn) -> BinBn {
		let mut maxlen :usize = 0;
		let alen :usize = self.data.len();
		let olen :usize = other.data.len();
		let r8 :Vec<u8> = vec![0];
		let rv :BinBn = BinBn::new_from_be(&r8);
		self._check_other(other);

		rv
	}

}


impl core::fmt::Debug for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::Display::fmt(&bnum, f)
	}
}

impl core::fmt::Display for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::Display::fmt(&bnum,f)
	}
}

impl core::fmt::Binary for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::Binary::fmt(&bnum,f)		
	}
}

impl core::fmt::Octal for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::Octal::fmt(&bnum,f)		
	}
}

impl core::fmt::LowerHex for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::LowerHex::fmt(&bnum,f)		
	}
}

impl core::fmt::UpperHex for BinBn {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let bnum :BigInt = self.to_bigint();
		core::fmt::UpperHex::fmt(&bnum,f)		
	}
}
