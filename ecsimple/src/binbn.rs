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

		//ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len() * BVALUE_SIZE, "to bytes");
		BinBn {
			data : rdata,
		}
	}

	pub fn new_from_bigint(bn :&BigInt) -> BinBn {
		let varr :Vec<u8>;
		(_, varr) = bn.to_bytes_be();
		return BinBn::new_from_be(&varr);
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
		//ecsimple_debug_buffer_trace!(varr.as_ptr(),varr.len(), "varr ");

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

		//ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len() * BVALUE_SIZE, "to bytes");
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
		//ecsimple_debug_buffer_trace!(rdata.as_ptr(), rdata.len(), "to bytes");
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

	fn _mul_1x1(&self,x0 :BValue,y0 :BValue) -> Vec<BValue> {
		let  (mut h,mut l,mut s) :(BValue,BValue,BValue);
		let mut tab :Vec<BValue> = Vec::new();
		let top3 :BValue = (x0 >> 61) as BValue;
		let (a1,a2,a4,a8):(BValue,BValue,BValue,BValue);
		let mut retv :Vec<BValue> = Vec::new();
		a1 = x0 & (0x1FFFFFFFFFFFFFFF as BValue);
		a2 = a1 << 1;
		a4 = a2 << 1;
		a8 = a4 << 1;
		for _ in 0..16 {
			tab.push(0);
		}

		tab[0] = 0;
		tab[1] = a1;
		tab[2] = a2;
		tab[3] = a1 ^ a2;
		tab[4] = a4;
		tab[5] = a4 ^ a1;
		tab[6] = a4 ^ a2;
		tab[7] = a4 ^ a2 ^ a1;
		tab[8] = a8;
		tab[9] = a8 ^ a1;
		tab[10] = a8 ^ a2;
		tab[11] = a8 ^ a2 ^ a1;
		tab[12] = a8 ^ a4;
		tab[13] = a8 ^ a4 ^ a1;
		tab[14] = a8 ^ a4 ^ a2;
		tab[15] = a8 ^ a4 ^ a2 ^ a1;

		/*for i in 0..tab.len() {
			ecsimple_log_trace!("tab[{}]=[0x{:x}]",i,tab[i]);
		}*/

		ecsimple_log_trace!("a 0x{:x} b 0x{:x}",x0,y0);

		s = tab[(y0 & 0xF) as usize];
		l = s;
		//ecsimple_log_trace!("l [0x{:x}]",l);

		s = tab[((y0 >> 4) & 0xF) as usize];
		l ^= (s << 4) as BValue;
		h = (s >> 60) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",4,l,h);

		s = tab[((y0 >> 8) & 0xF) as usize];
		l ^= (s << 8) as BValue;
		h ^= (s >> 56) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",8,l,h);

		s = tab[((y0 >> 12) & 0xF) as usize];
		l ^= (s << 12) as BValue;
		h ^= (s >> 52) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",12,l,h);

		s = tab[((y0 >> 16) & 0xF) as usize];
		l ^= (s << 16) as BValue;
		h ^= (s >> 48) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",16,l,h);

		s = tab[((y0 >> 20) & 0xF) as usize];
		l ^= (s << 20) as BValue;
		h ^= (s >> 44) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",20,l,h);

		s = tab[((y0 >> 24) & 0xF) as usize];
		l ^= (s << 24) as BValue;
		h ^= (s >> 40) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",24,l,h);

		s = tab[((y0 >> 28) & 0xF) as usize];
		l ^= (s << 28) as BValue;
		h ^= (s >> 36) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",28,l,h);

		s = tab[((y0 >> 32) & 0xF) as usize];
		l ^= (s << 32) as BValue;
		h ^= (s >> 32) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",32,l,h);

		s = tab[((y0 >> 36) & 0xF) as usize];
		l ^= (s << 36) as BValue;
		h ^= (s >> 28) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",36,l,h);

		s = tab[((y0 >> 40) & 0xF) as usize];
		l ^= (s << 40) as BValue;
		h ^= (s >> 24) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",40,l,h);

		s = tab[((y0 >> 44) & 0xF) as usize];
		l ^= (s << 44) as BValue;
		h ^= (s >> 20) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",44,l,h);

		s = tab[((y0 >> 48) & 0xF) as usize];
		l ^= (s << 48) as BValue;
		h ^= (s >> 16) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",48,l,h);

		s = tab[((y0 >> 52) & 0xF) as usize];
		l ^= (s << 52) as BValue;
		h ^= (s >> 12) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",52,l,h);

		s = tab[((y0 >> 56) & 0xF) as usize];
		l ^= (s << 56) as BValue;
		h ^= (s >> 8) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",56,l,h);

		s = tab[((y0 >> 60) & 0xF) as usize];
		l ^= (s << 60) as BValue;
		h ^= (s >> 4) as BValue;
		//ecsimple_log_trace!("[{}]l [0x{:x}] h [0x{:x}]",60,l,h);

		if (top3 & 0x1) != 0 {
			l ^= (y0 << 61) as BValue;
			h ^= (y0 >> 3) as BValue;
		}

		if (top3 & 0x2) != 0 {
			l ^= (y0 << 62) as BValue;
			h ^= (y0 >> 2) as BValue;
		}

		if (top3 & 0x4) != 0 {
			l ^= (y0 << 63) as BValue;
			h ^= (y0 >> 1) as BValue;
		}

		retv.push(l);
		retv.push(h);
		ecsimple_log_trace!("h 0x{:x} l 0x{:x}", h, l);
		retv
	}

	fn _mul_2x2(&self,x0 :BValue,x1 :BValue, y0 :BValue,y1 :BValue) -> Vec<BValue> {
		let mut retv :Vec<BValue> = Vec::new();
		let mut resv :Vec<BValue>;
		ecsimple_log_trace!("x0 0x{:x} x1 0x{:x} y0 0x{:x} y1 0x{:x}",x0,x1,y0,y1);
		for _ in 0..4 {
			retv.push(0);
		}
		resv = self._mul_1x1(x1,y1);
		retv[2] = resv[0];
		retv[3] = resv[1];
		ecsimple_log_trace!("retv[3] 0x{:x} retv[2] 0x{:x}",retv[3],retv[2]);

		resv = self._mul_1x1(x0,y0);
		retv[0] = resv[0];
		retv[1] = resv[1];
		ecsimple_log_trace!("retv[1] 0x{:x} retv[0] 0x{:x}",retv[1],retv[0]);
		resv = self._mul_1x1(x0 ^ x1 , y0 ^ y1);
		ecsimple_log_trace!("m1 0x{:x} m0 0x{:x}",resv[1],resv[0]);

		retv[2] ^= resv[1] ^ retv[1] ^ retv[3];
		retv[1] = retv[3] ^ retv[2] ^ retv[0] ^ resv[1] ^ resv[0];


		ecsimple_log_trace!("retv 0x{:x} 0x{:x} 0x{:x} 0x{:x}",retv[3],retv[2],retv[1],retv[0]);
		retv
	}


	pub fn mul_op(&self, other :&BinBn) -> BinBn {
		let maxlen :usize;
		let alen :usize = self.data.len();
		let olen :usize = other.data.len();
		let r8 :Vec<u8> = vec![0];
		let mut rv :BinBn = BinBn::new_from_be(&r8);
		let mut retv :Vec<BValue> = Vec::new();
		let (mut y0,mut y1) :(BValue,BValue);
		let (mut x0,mut x1) :(BValue,BValue);
		let (mut i, mut j) : (usize,usize);
		self._check_other(other);

		maxlen = alen + olen + 4;
		for _ in 0..maxlen {
			retv.push(0);
		}

		i = 0;
		while i < alen {
			y0 = self.data[i];
			y1 = 0;
			if (i + 1) < alen {
				y1 = self.data[i+1];
			}
			j = 0;
			while j < olen {
				x0 = other.data[j];
				x1 = 0;
				if (j + 1) < olen {
					x1 = other.data[j+1];
				}

				let resv = self._mul_2x2(x0,x1,y0,y1);
				ecsimple_log_trace!("resv 0x{:x} 0x{:x} 0x{:x} 0x{:x}",resv[0],resv[1],resv[2],resv[3]);

				for k in 0..resv.len() {
					ecsimple_log_trace!("[{i}+{j}+{k}] 0x{:x} ^ [{k}] 0x{:x} => 0x{:x}",retv[i+j+k], resv[k],retv[i+j+k] ^ resv[k]);
					retv[i+j+k] ^= resv[k];
					
				}
				j += 2;
			}
			i += 2;
		}
		rv.data = retv;
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
