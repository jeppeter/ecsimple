use num_bigint::{BigInt,Sign};


type BValue = u64;

const BValueSize :usize = std::mem::size_of::<BValue>();

pub struct BinBn  {
	data :Vec<BValue>,
}

impl BinBn {
	pub fn new_from_le(varr :&[u8]) -> BinBn {
		let mut rdata :Vec<BValue> = Vec::new();
		let mut lens :usize = varr.len() / BValueSize;
		if (lens * BValueSize) < varr.len() {
			lens += 1;
		}
		for i in 0..lens {
			let mut curval : BValue = 0;
			for j in 0..BValueSize {
				curval |= (varr[(i * BValueSize) + j] as BValue )<< (j * 8);
			}
			rdata.insert(0,curval);
		}
		BinBn {
			data : rdata,
		}
	}

	pub fn new_from_be(varr :&[u8]) -> BinBn {
		let mut rdata :Vec<BValue> = Vec::new();
		let mut lens :usize = varr.len() / BValueSize;
		if (lens * BValueSize) < varr.len() {
			lens += 1;
		}
		for i in 0..lens {
			let mut curval : BValue = 0;
			for j in 0..BValueSize {
				curval |= (varr[(i * BValueSize) + j]  as BValue)<< ((BValueSize - j - 1) * 8);
			}
			rdata.push(curval);
		}
		BinBn {
			data : rdata,
		}
	}

	pub fn to_bigint(&self) -> BigInt {
		let mut rdata :Vec<u8> = Vec::new();
		for i in 0..self.data.len() {
			for j in 0..BValueSize {
				let val :u8 = (self.data[i] >> ((BValueSize - j - 1) * 8)) as u8;
				rdata.push(val);
			}
		}
		BigInt::from_bytes_be(Sign::Plus,&rdata)
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
