use num_bigint::{BigInt};
use num_traits::{one,zero};
use crate::*;
use crate::consts::*;
use crate::utils::*;
use std::error::Error;

ecsimple_error_class!{MontError}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct MontNum {
	RR :BigInt,
	N :BigInt,
	R :BigInt,
	INVR :BigInt,
	BL :i64,
}

impl std::fmt::Display for MontNum {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"N 0x{:X} R 0x{:X} INVR 0x{:X} RR 0x{:X} BL 0x{:X}", self.N,self.R,self.INVR,self.RR,self.BL)
	}
}

impl MontNum {
	pub fn new(modv :&BigInt) -> Result<MontNum,Box<dyn Error>> {
		let ov :BigInt = one();
		let tv :BigInt = ov.clone() + ov.clone();
		let zv :BigInt = zero();
		if (modv.clone() % &tv) == zv {
			ecsimple_new_error!{MontError,"0x{:X} not valid",modv}
		}
		let mut bl :i64 = get_max_bits(modv);
		bl = ((bl + MONT_BIT_SIZE - 1) / MONT_BIT_SIZE) * MONT_BIT_SIZE;
		let r :BigInt =  &ov << bl ;
		let rr :BigInt = &ov << (bl * 2);
		let invr :BigInt = r.modpow(&(modv.clone() - &tv), &modv);
		Ok(MontNum {
			R : r,
			RR : rr,
			INVR : invr,
			BL : bl,
			N : modv.clone(),
		})
	}

	pub fn mont_to(&self,bn :&BigInt) -> BigInt {
		let retv :BigInt = (bn * &self.RR * &self.INVR) % &self.N;
		return retv;
	}

	pub fn mont_from(&self,bn :&BigInt) -> BigInt {
		let retv :BigInt = (bn * &self.INVR) % &self.N;
		return retv;
	}
}
